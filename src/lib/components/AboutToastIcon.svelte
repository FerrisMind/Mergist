<script lang="ts">
  // Берём логотип из public/, он доступен как абсолютный путь
  const logoSrc = '/logo.svg';
</script>

<img src={logoSrc} alt="" class="h-12 w-12 rounded-lg shadow-sm" loading="lazy" />
