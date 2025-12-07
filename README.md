[![README EN](https://img.shields.io/badge/README-EN-0ea5e9?style=flat&logo=markdown&logoColor=white&labelColor=0f172a)](README.md)
[![README RU](https://img.shields.io/badge/README-RU-cbd5e1?style=flat&logo=markdown&logoColor=374151&labelColor=0f172a)](README.ru.md)
[![README PT-BR](https://img.shields.io/badge/README-PT--BR-cbd5e1?style=flat&logo=markdown&logoColor=374151&labelColor=0f172a)](README.pt-BR.md)

![Mergist](.github/assets/logo.svg)

Cross‑platform desktop app that turns a GitHub repository into a Markdown export (code and issues) with progress tracking, theming, and built‑in localization.

## Features
- Convert repositories to a single Markdown export with optional separators and filename listings.
- Export GitHub issues with configurable filename formatting.
- Live progress and tokenization status with rich toasts.
- Theme toggle (light/dark/system) and locale switcher (English, Русский, Português do Brasil).
- Tauri v2 desktop shell with Svelte 5 (runes), Vite, Tailwind CSS, and shadcn-svelte UI kit.

## Requirements
- Node.js 20+ and npm
- Rust toolchain for Tauri (see https://tauri.app/start/prerequisites/)
- (Optional) `@tauri-apps/cli` installed globally for faster builds

## Getting started
```bash
cd tauri-app
npm install
npm run dev          # launch Tauri + UI
```

Frontend-only dev server (Vite):
```bash
npm run dev:ui
```

## Scripts
- `npm run dev` — Tauri dev (backend + UI)
- `npm run dev:ui` — Vite dev server only
- `npm run build` — production desktop build via Tauri
- `npm run build:ui` — build frontend only (`dist/`)
- `npm run check` — Svelte type checks
- `npm run lint` — ESLint/Svelte linting

## Internationalization
Locale files live in `src/lib/i18n/locales` (`en`, `ru`, `pt-BR`). Switch languages from the header globe menu. New strings should be added to all locale JSONs to keep parity.

## Project structure (high level)
- `src/` — Svelte UI
- `src/lib` — components, stores, i18n, utilities
- `src-tauri/` — Rust side (commands, config)

## Acknowledgements
- Thanks to [Puter-Apps/repo-to-markdown](https://github.com/Puter-Apps/repo-to-markdown) for the inspiration.
- Desktop shell: [Tauri](https://tauri.app/).
- UI: [Svelte](https://svelte.dev/), icons from [Lucide](https://lucide.dev/), components via [shadcn-svelte](https://ui.shadcn.com/).

## License
Apache-2.0 — see `../LICENSE`.

