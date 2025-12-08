#!/usr/bin/env node

/**
 * Автоматический выпуск релиза:
 * 1) Берёт версию из src-tauri/tauri.conf.json (предпочтительна для тега)
 *    при несовпадении с package.json выводит предупреждение.
 * 2) Проверяет, что рабочее дерево чистое.
 * 3) Создаёт аннотированный тег v<version> и пушит его.
 * 4) Если задан GITHUB_TOKEN / GH_TOKEN — создаёт Release через GitHub API.
 * Workflow на GitHub Actions стартует автоматически по пушу тега v*.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';
import https from 'https';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT = path.resolve(__dirname, '..');

const readJson = (relPath) => {
  const full = path.join(ROOT, relPath);
  return JSON.parse(fs.readFileSync(full, 'utf8'));
};

const run = (cmd) =>
  execSync(cmd, { cwd: ROOT, stdio: ['ignore', 'pipe', 'pipe'] }).toString().trim();

const pkgVersion = readJson('package.json').version;
const tauriVersion = readJson('src-tauri/tauri.conf.json').version;
const version = tauriVersion || pkgVersion;

if (!version) {
  console.error('Не удалось определить версию приложения.');
  process.exit(1);
}

if (pkgVersion !== tauriVersion) {
  console.warn(
    `Внимание: версия в package.json (${pkgVersion}) не совпадает с src-tauri/tauri.conf.json (${tauriVersion}). Использую ${version} для тега.`
  );
}

const tag = `v${version}`;

const status = run('git status --porcelain');
if (status) {
  console.error('Рабочее дерево не чистое. Сохраните/закоммитьте изменения перед релизом.');
  process.exit(1);
}

const existing = run(`git tag -l ${tag}`);
if (existing === tag) {
  console.error(`Тег ${tag} уже существует.`);
  process.exit(1);
}

console.log(`Создаю тег ${tag}...`);
run(`git tag -a ${tag} -m "Release ${tag}"`);
console.log(`Пушу тег ${tag}...`);
run(`git push origin ${tag}`);

const token = process.env.GITHUB_TOKEN || process.env.GH_TOKEN;

const parseRepo = (remoteUrl) => {
  const ssh = remoteUrl.match(/^git@github\.com:([^/]+)\/([^/]+?)(\.git)?$/);
  if (ssh) return { owner: ssh[1], repo: ssh[2] };
  const httpsUrl = remoteUrl.match(/^https:\/\/github\.com\/([^/]+)\/([^/]+?)(\.git)?$/);
  if (httpsUrl) return { owner: httpsUrl[1], repo: httpsUrl[2] };
  return null;
};

async function createRelease() {
  if (!token) {
    console.warn('GITHUB_TOKEN не задан — пропускаю создание Release. Тег уже пушнут, workflow запустится.');
    return;
  }

  const remoteUrl = run('git config --get remote.origin.url');
  const repoInfo = parseRepo(remoteUrl);
  if (!repoInfo) {
    console.warn(`Не удалось распарсить remote.origin.url: ${remoteUrl}. Пропускаю создание Release.`);
    return;
  }

  const body = JSON.stringify({
    tag_name: tag,
    name: `Mergist ${tag}`,
    body: `Автоматический релиз ${tag}`,
    draft: false,
    prerelease: false
  });

  const options = {
    method: 'POST',
    hostname: 'api.github.com',
    path: `/repos/${repoInfo.owner}/${repoInfo.repo}/releases`,
    headers: {
      'User-Agent': 'release-script',
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
      'Content-Length': Buffer.byteLength(body),
      Accept: 'application/vnd.github+json'
    }
  };

  await new Promise((resolve) => {
    const req = https.request(options, (res) => {
      const chunks = [];
      res.on('data', (d) => chunks.push(d));
      res.on('end', () => {
        const text = Buffer.concat(chunks).toString('utf8');
        if (res.statusCode === 201) {
          console.log(`Создан Release для ${tag}.`);
        } else if (res.statusCode === 422 && text.includes('already_exists')) {
          console.log(`Release для ${tag} уже существует, пропускаю.`);
        } else {
          console.warn(`Не удалось создать Release (${res.statusCode}): ${text}`);
        }
        resolve();
      });
    });
    req.on('error', (err) => {
      console.warn(`Ошибка при создании Release: ${err.message}`);
      resolve();
    });
    req.write(body);
    req.end();
  });
}

createRelease()
  .then(() => {
    console.log(`Готово. Тег ${tag} пушнут. Workflow GitHub Actions (Release) запустится автоматически.`);
  })
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });

