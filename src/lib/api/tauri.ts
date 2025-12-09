import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { save } from '@tauri-apps/plugin-dialog';
import { copyFile, remove } from '@tauri-apps/plugin-fs';
import { relaunch } from '@tauri-apps/plugin-process';
import { check as checkUpdate, type DownloadEvent } from '@tauri-apps/plugin-updater';
import type {
  ConversionResult,
  ConvertOptions,
  IssuesExportOptions,
  IssuesExportResult,
} from '$lib/types';
import { encode } from 'gpt-tokenizer';

export function listenConversionProgress(
  channel: 'conversion-progress' | 'issues-progress',
  cb: (current: number, total: number) => void
): Promise<UnlistenFn> {
  return listen(channel, (event) => {
    const payload = event.payload as { current: number; total: number };
    cb(payload.current ?? 0, payload.total ?? 0);
  });
}

export async function convertRepo(
  repo: string,
  options: ConvertOptions
): Promise<ConversionResult> {
  return invoke<ConversionResult>('convert_repo_to_markdown', {
    input: { repo },
    options,
  });
}

export async function exportIssues(
  repo: string,
  options: IssuesExportOptions
): Promise<IssuesExportResult> {
  return invoke<IssuesExportResult>('export_issues', {
    input: { repo },
    options,
  });
}

export async function cancelConversion(): Promise<void> {
  await invoke('cancel_conversion');
}

export async function downloadFile(
  sourcePath: string,
  suggestedName: string
): Promise<string | null> {
  const target = await save({
    defaultPath: suggestedName,
    filters: [{ name: 'Markdown', extensions: ['md'] }],
  });
  if (!target) return null;
  await copyFile(sourcePath, target);
  return target;
}

export async function tokenizeFile(
  path: string,
  readChunk: (offset: number, size: number) => Promise<string | null>,
  onProgress?: (percent: number) => void
): Promise<number> {
  const CHUNK = 512 * 1024;
  let offset = 0;
  let totalTokens = 0;

  // try to read size via invoke
  let fileSize = 0;
  try {
    fileSize = await invoke<number>('get_file_size', { path });
  } catch {
    fileSize = 0;
  }

  while (true) {
    const chunk = await readChunk(offset, CHUNK);
    if (!chunk) break;
    const tokens = encode(chunk);
    totalTokens += tokens.length;
    offset += chunk.length;
    if (fileSize > 0 && onProgress) {
      onProgress(Math.min(100, (offset / fileSize) * 100));
    }
  }

  return totalTokens;
}

export type UpdateDownloadProgress = {
  event: DownloadEvent['event'];
  percent?: number;
  downloadedBytes?: number;
  totalBytes?: number;
  chunkBytes?: number;
};

export type UpdateCheckResult = {
  available: boolean;
  version?: string;
  downloadAndInstall?: (onProgress?: (progress: UpdateDownloadProgress) => void) => Promise<void>;
  close?: () => Promise<void>;
};

export async function checkForUpdates(): Promise<UpdateCheckResult> {
  try {
    const update = await checkUpdate();
    if (!update) {
      return { available: false };
    }
    const version = update.version ?? update.currentVersion;
    return {
      available: true,
      version,
      close: () => update.close(),
      async downloadAndInstall(onProgress) {
        let total = 0;
        let downloaded = 0;
        await update.downloadAndInstall((event) => {
          if (!onProgress) return;
          if (event.event === 'Started') {
            total = event.data.contentLength ?? 0;
            onProgress({
              event: 'Started',
              totalBytes: total || undefined,
              downloadedBytes: 0,
              percent: total ? 0 : undefined,
            });
            return;
          }
          if (event.event === 'Progress') {
            downloaded += event.data.chunkLength ?? 0;
            const percent = total > 0 ? Math.min(100, (downloaded / total) * 100) : undefined;
            onProgress({
              event: 'Progress',
              chunkBytes: event.data.chunkLength ?? 0,
              downloadedBytes: downloaded,
              totalBytes: total || undefined,
              percent,
            });
            return;
          }
          if (event.event === 'Finished') {
            onProgress({
              event: 'Finished',
              downloadedBytes: downloaded || undefined,
              totalBytes: total || undefined,
              percent: total ? 100 : undefined,
            });
          }
        });
      },
    };
  } catch (error) {
    console.error('Update check failed', error);
    throw error;
  }
}

export async function restartApp(): Promise<void> {
  await relaunch();
}

export async function deleteFiles(paths: string[]): Promise<void> {
  const unique = Array.from(new Set(paths.filter(Boolean)));
  for (const path of unique) {
    try {
      await remove(path);
    } catch (error) {
      console.warn('Failed to remove file', path, error);
    }
  }
}
