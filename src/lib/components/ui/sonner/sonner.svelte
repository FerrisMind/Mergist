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

  /* Крупнее иконка и чуть больше отступ в тосте "О программе" */
  :global(.toaster [data-sonner-toast].about-toast) {
    column-gap: 0.9rem !important;
    align-items: center !important;
  }
  :global(.toaster [data-sonner-toast].about-toast [data-icon]) {
    width: 7rem !important;
    height: 3rem !important;
  }
  :global(.toaster [data-sonner-toast].about-toast [data-icon] > *) {
    width: 100% !important;
    height: 100% !important;
  }

  /* Убираем «приподнимание» тостов при наведении (expanded state) */
  :global(.toaster [data-sonner-toast][data-expanded='true']:not([data-removed='true']):not(
      [data-swipe-out='true']
    ):not([data-swiping='true'])) {
    /* сохраняем исходное положение стека даже при expanded */
    --y: translateY(calc(var(--lift-amount) * var(--toasts-before))) !important;
    height: auto !important;
    transform: var(--y) !important;
  }
  :global(.toaster [data-sonner-toast][data-expanded='true'][data-front='false'][data-styled='true'])
    > :global(*) {
    opacity: 0 !important;
  }

  /* Крупнее заголовок для тоста "О программе" */
  :global(.toaster [data-sonner-toast].about-toast .rt-title) {
    font-size: 1.05rem !important;
    font-weight: 700 !important;
    line-height: 1.3 !important;
  }
</style>
