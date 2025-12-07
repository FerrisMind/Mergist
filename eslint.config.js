import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';
import globals from 'globals';

const svelteRecommended = svelte.configs['flat/recommended'].map((config) => {
  if (config.name === 'svelte:base:setup-for-svelte') {
    return {
      ...config,
      languageOptions: {
        ...config.languageOptions,
        parser: svelteParser,
        parserOptions: {
          ...(config.languageOptions?.parserOptions ?? {}),
          parser: tseslint.parser,
          project: './tsconfig.json',
          tsconfigRootDir: import.meta.dirname,
          extraFileExtensions: ['.svelte'],
        },
        globals: {
          ...globals.browser,
          ...globals.es2021,
          ...(config.languageOptions?.globals ?? {}),
        },
      },
      plugins: {
        ...(config.plugins ?? {}),
        '@typescript-eslint': tseslint.plugin,
      },
      rules: {
        ...(config.rules ?? {}),
        ...tseslint.configs.recommended.rules,
        ...tseslint.configs.recommendedTypeChecked.rules,
        'no-unused-vars': 'off',
        '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
      },
    };
  }

  if (config.name === 'svelte:base:setup-for-svelte-script') {
    return {
      ...config,
      languageOptions: {
        ...config.languageOptions,
        parserOptions: {
          ...(config.languageOptions?.parserOptions ?? {}),
          project: './tsconfig.json',
          tsconfigRootDir: import.meta.dirname,
        },
      },
    };
  }

  return config;
});

export default tseslint.config(
  {
    ignores: ['node_modules', 'dist', 'build', '.svelte-kit', 'src-tauri'],
  },
  ...svelteRecommended,
  {
    files: ['**/*.ts', '**/*.tsx'],
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        project: './tsconfig.json',
        tsconfigRootDir: import.meta.dirname,
      },
      globals: {
        ...globals.browser,
        ...globals.es2021,
      },
    },
    plugins: {
      '@typescript-eslint': tseslint.plugin,
    },
    rules: {
      ...js.configs.recommended.rules,
      ...tseslint.configs.recommended.rules,
      ...tseslint.configs.recommendedTypeChecked.rules,
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
    },
  },
  {
    files: ['**/*.js'],
    languageOptions: {
      globals: {
        ...globals.node,
        ...globals.es2021,
      },
    },
    rules: {
      ...js.configs.recommended.rules,
    },
  }
);

