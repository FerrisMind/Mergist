<script lang="ts">
  import { Spinner } from '$lib/components/ui/spinner';
  import {
    checkForUpdates,
    type UpdateCheckResult,
    type UpdateDownloadProgress,
  } from '$lib/api/tauri';
  import { t } from '$lib/i18n';
  import { toast } from 'svelte-sonner';
  import Badge from '$lib/components/ui/badge/badge.svelte';

  let checking = $state(false);
  let downloading = $state(false);
  let availableVersion = $state<string | null>(null);
  let progress = $state<number | null>(null);
  let update: UpdateCheckResult | null = null;
  let cancelRequested = $state(false);

  const handleCheck = async (event: MouseEvent) => {
    event.preventDefault();
    if (checking || downloading) return;
    checking = true;
    availableVersion = null;
    progress = null;

    try {
      const result = await checkForUpdates();
      if (result.available) {
        update = result;
        availableVersion = result.version ?? '';
        toast.success($t('app.updateAvailable', { values: { version: availableVersion } }), {
          duration: 1500,
        });
      } else {
        update = null;
        toast.success($t('app.updatesLatest'), { duration: 1500 });
      }
    } catch {
      toast.error($t('app.updatesError'), { duration: 2000 });
    } finally {
      checking = false;
    }
  };

  const handleInstall = async (event: MouseEvent) => {
    event.preventDefault();
    if (!update?.downloadAndInstall || downloading) return;
    downloading = true;
    progress = 0;
    cancelRequested = false;

    let cancelReject: ((reason?: unknown) => void) | null = null;
    const cancelPromise = new Promise<never>((_, reject) => {
      cancelReject = reject;
    });

    try {
      await toast.promise(
        Promise.race([
          update.downloadAndInstall((p?: UpdateDownloadProgress) => {
            if (!p || cancelRequested) return;
            if (p.percent !== undefined) {
              progress = Math.min(100, Math.max(0, Math.round(p.percent)));
            }
          }),
          cancelPromise,
        ]),
        {
          id: 'download-update',
          loading: $t('app.updateDownloading'),
          success: $t('app.updateAutoRestart'),
          error: () => (cancelRequested ? $t('app.updateCancelled') : $t('app.updatesError')),
          duration: 1500,
          action: {
            label: $t('app.updateCancel'),
            onClick: async () => {
              if (!downloading) return;
              cancelRequested = true;
              progress = null;
              cancelReject?.('cancelled');
              try {
                await update?.close?.();
              } catch {
                // ignore
              }
            },
          },
        }
      );
      availableVersion = null;
      progress = null;
      update = null;
    } catch {
      // ignore
    } finally {
      downloading = false;
      cancelReject = null;
    }
  };

  const handleCancel = async (event: MouseEvent) => {
    event.preventDefault();
    if (!downloading) return;
    cancelRequested = true;
    progress = null;
    try {
      await update?.close?.();
    } catch {
      // ignore
    }
  };

  const handleLater = (event: MouseEvent) => {
    event.preventDefault();
    availableVersion = null;
    update = null;
    progress = null;
  };
</script>

<div class="flex w-full flex-col gap-3">
  <div class="flex flex-row items-start justify-between gap-3 w-full">
    <div class="flex flex-col gap-1 min-w-0">
      {#if availableVersion}
        <div class="text-sm text-muted-foreground">
          {$t('app.updateAvailable', { values: { version: availableVersion } })}
        </div>
      {/if}
    </div>

    <div class="flex flex-wrap justify-end gap-2">
      {#if availableVersion}
        <button
          type="button"
          data-button
          class="inline-flex items-center gap-2"
          onclick={handleInstall}
          data-disabled={downloading}
          aria-busy={downloading}
        >
          {#if downloading}
            <Spinner class="size-4" aria-hidden="true" />
            <span>{$t('app.updateDownloading')}</span>
            {#if progress !== null}
              <span class="text-xs text-muted-foreground">{progress}%</span>
            {/if}
          {:else}
            {$t('app.updateInstall')}
          {/if}
        </button>
        <button
          type="button"
          data-button
          class="inline-flex items-center gap-2 text-muted-foreground hover:text-foreground"
          onclick={handleLater}
          data-disabled={downloading}
        >
          {$t('app.updateLater')}
        </button>
        {#if downloading}
          <button
            type="button"
            data-button
            class="inline-flex items-center gap-2 text-destructive hover:text-destructive"
            onclick={handleCancel}
            aria-busy={downloading}
          >
            {$t('app.updateCancel')}
          </button>
        {/if}
      {:else}
        <button
          type="button"
          data-button
          class="inline-flex items-center gap-2"
          onclick={handleCheck}
          data-disabled={checking}
          aria-busy={checking}
        >
          {#if checking}
            <Spinner class="size-4" aria-hidden="true" />
            <span>{$t('app.updatesCheckingShort')}</span>
          {:else}
            {$t('app.checkUpdates')}
          {/if}
        </button>
      {/if}
    </div>
  </div>
</div>
