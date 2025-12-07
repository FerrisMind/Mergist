[![README EN](https://img.shields.io/badge/README-EN-cbd5e1?style=flat&logo=markdown&logoColor=374151&labelColor=0f172a)](README.md)
[![README RU](https://img.shields.io/badge/README-RU-cbd5e1?style=flat&logo=markdown&logoColor=374151&labelColor=0f172a)](README.ru.md)
[![README PT-BR](https://img.shields.io/badge/README-PT--BR-0ea5e9?style=flat&logo=markdown&logoColor=white&labelColor=0f172a)](README.pt-BR.md)

![Mergist](.github/assets/logo.svg)

Aplicativo desktop multiplataforma que transforma um repositório GitHub em um export Markdown (código e issues) com acompanhamento de progresso, temas e localização.

## Recursos
- Converte repositórios em um único Markdown com separadores opcionais e listagem de arquivos.
- Exporta GitHub Issues com formato de nome de arquivo configurável.
- Progresso em tempo real e status de tokenização com toasts ricos.
- Alterna tema (light/dark/system) e idioma (English, Русский, Português do Brasil).
- Shell Tauri v2 com Svelte 5 (runes), Vite, Tailwind CSS e UI kit shadcn-svelte.

## Requisitos
- Node.js 20+ e npm
- Toolchain Rust para Tauri (https://tauri.app/start/prerequisites/)
- (Opcional) `@tauri-apps/cli` global para builds mais rápidos

## Primeiros passos
```bash
cd tauri-app
npm install
npm run dev          # inicia Tauri + UI
```

Somente frontend (Vite):
```bash
npm run dev:ui
```

## Scripts
- `npm run dev` — modo dev do Tauri (backend + UI)
- `npm run dev:ui` — apenas Vite dev server
- `npm run build` — build de produção via Tauri
- `npm run build:ui` — build apenas do frontend (`dist/`)
- `npm run check` — verificações Svelte
- `npm run lint` — linting ESLint/Svelte

## Internacionalização
Arquivos de locale ficam em `src/lib/i18n/locales` (`en`, `ru`, `pt-BR`). O idioma é trocado pelo menu no cabeçalho (ícone de globo). Novas strings devem ser adicionadas em todos os JSONs para manter paridade.

## Estrutura (alto nível)
- `src/` — UI Svelte
- `src/lib` — componentes, stores, i18n, utilitários
- `src-tauri/` — lado Rust (comandos, config)

## Agradecimentos
- Obrigado ao [Puter-Apps/repo-to-markdown](https://github.com/Puter-Apps/repo-to-markdown) pela inspiração.
- Shell desktop: [Tauri](https://tauri.app/).
- UI: [Svelte](https://svelte.dev/), ícones de [Lucide](https://lucide.dev/), componentes de [shadcn-svelte](https://ui.shadcn.com/).

## Licença
Apache-2.0 — veja `../LICENSE`.

