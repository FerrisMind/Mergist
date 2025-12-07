<script lang="ts">
  import { getContext } from 'svelte';
  import type { Snippet } from 'svelte';
  import { ACCORDION_KEY, type AccordionContext } from './context';
  import { cn } from '$lib/utils';

  let {
    value,
    class: className = '',
    children,
  }: {
    value: string;
    class?: string;
    children?: Snippet;
  } = $props();

  const ctx = getContext<AccordionContext>(ACCORDION_KEY);
  const isOpen = $derived(ctx?.active === value);
</script>

{#if isOpen}
  <div class={cn('px-4 pb-4 text-sm text-foreground', className)}>
    {#if children}{@render children()}{/if}
  </div>
{/if}
