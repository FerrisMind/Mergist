<script lang="ts">
  import { ScrollArea as ScrollAreaPrimitive } from 'bits-ui';
  import { cn, type WithoutChild } from '$lib/utils';

  let {
    ref = $bindable(null),
    class: className,
    orientation = 'vertical',
    children,
    ...restProps
  }: WithoutChild<ScrollAreaPrimitive.ScrollbarProps> = $props();
</script>

<ScrollAreaPrimitive.Scrollbar
  bind:ref
  data-slot="scroll-area-scrollbar"
  {orientation}
  class={cn(
    'flex touch-none select-none transition-colors',
    orientation === 'vertical' && 'h-full w-2',
    orientation === 'horizontal' && 'h-2 flex-col',
    className
  )}
  {...restProps}
>
  {@render children?.()}
  <ScrollAreaPrimitive.Thumb
    data-slot="scroll-area-thumb"
    class="relative flex-1 rounded-full bg-muted-foreground/30 hover:bg-muted-foreground/50"
  />
</ScrollAreaPrimitive.Scrollbar>
