import { addMessages, init, locale, t } from 'svelte-i18n';
import en from './locales/en.json';
import ru from './locales/ru.json';
import ptBR from './locales/pt-BR.json';

export type TranslateFn = (key: string, options?: Record<string, unknown>) => string;

export const AVAILABLE_LOCALES = [
  { value: 'ru', label: 'RU' },
  { value: 'en', label: 'EN' },
  { value: 'pt-BR', label: 'PT-BR' },
] as const;

let initialized = false;

export function detectLocale(preferred?: string) {
  const lang =
    preferred ??
    (typeof navigator !== 'undefined' && navigator.language ? navigator.language : 'en');
  const lower = lang.toLowerCase();
  if (lower.startsWith('ru')) return 'ru';
  if (lower.startsWith('pt')) return 'pt-BR';
  return 'en';
}

export function setupI18n(preferred?: string) {
  if (initialized) return;

  addMessages('en', en);
  addMessages('ru', ru);
  addMessages('pt-BR', ptBR);

  init({
    fallbackLocale: 'en',
    initialLocale: detectLocale(preferred),
  });

  initialized = true;
}

export { locale, t };
