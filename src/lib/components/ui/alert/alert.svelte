<script lang="ts" module>
  import { type VariantProps, tv } from 'tailwind-variants';

  export const alertVariants = tv({
    base: 'relative grid w-full grid-cols-[0_1fr] items-start gap-y-2 rounded-lg border px-4 py-4 text-sm has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] has-[>svg]:gap-x-4 [&>svg]:size-4 [&>svg]:self-center [&>svg]:text-current',
    variants: {
      variant: {
        info: 'border-blue-200/70 bg-blue-50 text-blue-900 dark:border-blue-500/40 dark:bg-blue-900/40 dark:text-blue-50',
        success:
          'border-emerald-200/70 bg-emerald-50 text-emerald-900 dark:border-emerald-500/40 dark:bg-emerald-900/40 dark:text-emerald-50',
        warning:
          'border-amber-200/70 bg-amber-50 text-amber-900 dark:border-amber-500/40 dark:bg-amber-900/40 dark:text-amber-50',
        error:
          'border-red-200/70 bg-red-50 text-red-900 dark:border-red-500/40 dark:bg-red-900/40 dark:text-red-50',
      },
    },
    defaultVariants: {
      variant: 'info',
    },
  });

  export type AlertVariant = VariantProps<typeof alertVariants>['variant'];
</script>

<script lang="ts">
  import type { HTMLAttributes } from 'svelte/elements';
  import { cn, type WithElementRef } from '$lib/utils';
  import InfoIcon from '@lucide/svelte/icons/info';
  import CheckCircle2Icon from '@lucide/svelte/icons/check-circle-2';
  import AlertTriangleIcon from '@lucide/svelte/icons/alert-triangle';
  import CircleXIcon from '@lucide/svelte/icons/circle-x';

  let {
    ref = $bindable(null),
    class: className,
    variant = 'info',
    children,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> & {
    variant?: AlertVariant;
  } = $props();
</script>

<div
  bind:this={ref}
  data-slot="alert"
  class={cn(alertVariants({ variant }), className)}
  {...restProps}
  role="alert"
>
  {#if variant === 'info'}
    <InfoIcon aria-hidden="true" />
  {:else if variant === 'success'}
    <CheckCircle2Icon aria-hidden="true" />
  {:else if variant === 'warning'}
    <AlertTriangleIcon aria-hidden="true" />
  {:else if variant === 'error'}
    <CircleXIcon aria-hidden="true" />
  {/if}
  {@render children?.()}
</div>
