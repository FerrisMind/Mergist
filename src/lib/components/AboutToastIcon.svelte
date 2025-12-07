<script lang="ts">
  // Импортируем как asset, чтобы Vite/Tauri корректно положили файл в сборку
  import logoSrc from '../../../static/logo.svg';
</script>

<img src={logoSrc} alt="" class="h-12 w-12 rounded-lg shadow-sm" loading="lazy" />

