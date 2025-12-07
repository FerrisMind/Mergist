<script lang="ts">
  import { toast } from 'svelte-sonner';
  import { onDestroy } from 'svelte';
  import type { Status } from '$lib/types';

  let {
    message = '',
    variant = 'info',
    status = 'idle',
  }: {
    message?: string;
    variant?: 'info' | 'success' | 'warning' | 'error';
    status?: Status;
  } = $props();

  const toastId = 'status-message';
  let lastKey = '';

  const showToast = () => {
    const key = `${status}-${variant}-${message}`;
    if (key === lastKey) return;
    lastKey = key;

    if (!message) {
      toast.dismiss(toastId);
      return;
    }

    if (status === 'running') {
      toast.loading(message, { id: toastId, duration: Infinity });
      return;
    }

    const variantFn =
      variant === 'success'
        ? toast.success
        : variant === 'warning'
          ? toast.warning
          : variant === 'error'
            ? toast.error
            : toast;
    variantFn(message, { id: toastId });
  };

  $effect(() => {
    showToast();
  });

  onDestroy(() => {
    toast.dismiss(toastId);
  });
</script>

<!-- Toast-only component: визуально ничего не рендерим -->
