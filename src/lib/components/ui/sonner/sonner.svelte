<script lang="ts">
  import { Toaster as Sonner, type ToasterProps as SonnerProps } from 'svelte-sonner';
  import { mode } from 'mode-watcher';

  const closeAlignClass = 'rt-close-right';

  let { ...restProps }: SonnerProps = $props();
</script>

<Sonner
  theme={mode.current}
  class={`toaster group ${closeAlignClass}`}
  style="--normal-bg: var(--color-popover); --normal-text: var(--color-popover-foreground); --normal-border: var(--color-border);"
  {...restProps}
/>

<style>
  /* По умолчанию кнопку закрытия скрываем */
  :global(.toaster [data-close-button]) {
    display: none !important;
  }

  /* Оставляем кнопку только для warning/error и тоста "О программе" */
  :global(.toaster [data-sonner-toast][data-type='warning']),
  :global(.toaster [data-sonner-toast][data-type='error']),
  :global(.toaster [data-sonner-toast].about-toast) {
    position: relative !important;
    padding-right: 2.25rem !important;
  }

  :global(.toaster [data-sonner-toast][data-type='warning'] [data-close-button]),
  :global(.toaster [data-sonner-toast][data-type='error'] [data-close-button]),
  :global(.toaster [data-sonner-toast].about-toast [data-close-button]) {
    display: inline-flex !important;
    position: absolute !important;
    top: 0.35rem !important;
    right: 0.5rem !important;
    left: auto !important;
    margin: 0 !important;
    order: unset !important;
    transform: none !important;
  }
</style>
