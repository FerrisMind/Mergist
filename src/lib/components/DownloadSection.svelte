<script lang="ts">
  import { Button } from '$lib/components/ui';
  import { t } from '$lib/i18n';

  let {
    filePath = null,
    onDownload = null,
  }: {
    filePath?: string | null;
    onDownload?: (() => Promise<void>) | null;
  } = $props();

  const handleClick = (e: MouseEvent) => {
    e.preventDefault();
    onDownload?.();
  };
</script>

{#if filePath}
  <div class="flex flex-col gap-4 rounded-xl border border-border bg-muted/50 p-4 mb-2">
    <div>
      <p class="text-xs uppercase tracking-wide text-muted-foreground">
        {$t('download.file')}
      </p>
      <p class="text-sm font-medium text-foreground break-all">{filePath}</p>
    </div>
    {#if onDownload}
      <Button onclick={handleClick} class="w-full">{$t('download.saveAs')}</Button>
    {/if}
  </div>
{/if}
