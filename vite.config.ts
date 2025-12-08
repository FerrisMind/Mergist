import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  plugins: [
    svelte({
      compilerOptions: {
        runes: true,
      },
    }),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      $lib: path.resolve(__dirname, 'src/lib'),
    },
  },
  server: {
    port: 1420,
    strictPort: true,
  },
  build: {
    target: 'es2022',
    sourcemap: false,
    reportCompressedSize: false,
    rollupOptions: {
      external: ['fsevents'],
      output: {
        manualChunks: {
          'vendor-svelte': ['svelte'],
          'vendor-tauri': [
            '@tauri-apps/api',
            '@tauri-apps/plugin-dialog',
            '@tauri-apps/plugin-fs',
            '@tauri-apps/plugin-process',
            '@tauri-apps/plugin-updater',
          ],
          'vendor-icons': ['@lucide/svelte'],
        },
      },
    },
  },
  optimizeDeps: {
    exclude: [
      '@tauri-apps/api',
      '@tauri-apps/plugin-dialog',
      '@tauri-apps/plugin-fs',
      '@tauri-apps/plugin-process',
      '@tauri-apps/plugin-updater',
    ],
  },
  clearScreen: false,
});
