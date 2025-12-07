[![README EN](https://img.shields.io/badge/README-EN-cbd5e1?style=flat&logo=markdown&logoColor=374151&labelColor=0f172a)](README.md)
[![README RU](https://img.shields.io/badge/README-RU-0ea5e9?style=flat&logo=markdown&logoColor=white&labelColor=0f172a)](README.ru.md)
[![README PT-BR](https://img.shields.io/badge/README-PT--BR-cbd5e1?style=flat&logo=markdown&logoColor=374151&labelColor=0f172a)](README.pt-BR.md)

![Mergist](.github/assets/logo.svg)

Кроссплатформенное десктоп‑приложение, которое превращает репозиторий GitHub в Markdown‑экспорт (код и задачи) с прогрессом, темизацией и локализацией.

## Возможности
- Конвертация репозитория в единый Markdown с опциональными разделителями и перечнем файлов.
- Экспорт GitHub Issues с настраиваемым форматом имени файла.
- Живой прогресс и статус токенизации, уведомления через toasts.
- Переключение темы (light/dark/system) и языка (English, Русский, Português do Brasil).
- Оболочка Tauri v2 + Svelte 5 (runes), Vite, Tailwind CSS и UI‑кит shadcn-svelte.

## Требования
- Node.js 20+ и npm
- Rust toolchain для Tauri (https://tauri.app/start/prerequisites/)
- (Опционально) `@tauri-apps/cli` глобально для быстрых сборок

## Быстрый старт
```bash
cd tauri-app
npm install
npm run dev          # запустить Tauri + UI
```

Только фронтенд (Vite):
```bash
npm run dev:ui
```

## Скрипты
- `npm run dev` — dev-режим Tauri (бэкенд + UI)
- `npm run dev:ui` — только Vite dev server
- `npm run build` — продакшн-сборка через Tauri
- `npm run build:ui` — сборка фронтенда (`dist/`)
- `npm run check` — проверки Svelte
- `npm run lint` — ESLint/Svelte линтинг

## Локализация
Файлы локалей: `src/lib/i18n/locales` (`en`, `ru`, `pt-BR`). Язык переключается в шапке (иконка глобуса). Новые строки добавляйте во все JSON для синхронности.

## Структура (кратко)
- `src/` — Svelte UI
- `src/lib` — компоненты, сторы, i18n, утилиты
- `src-tauri/` — часть на Rust (команды, конфиг)

## Благодарности
- Спасибо [Puter-Apps/repo-to-markdown](https://github.com/Puter-Apps/repo-to-markdown) за вдохновение.
- Десктоп-оболочка: [Tauri](https://tauri.app/).
- UI: [Svelte](https://svelte.dev/), иконки — [Lucide](https://lucide.dev/), компоненты — [shadcn-svelte](https://ui.shadcn.com/).

## Лицензия
Apache-2.0 — см. `../LICENSE`.

