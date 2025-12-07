<script lang="ts">
  import { Spinner } from '$lib/components/ui/spinner';
  import {
    checkForUpdates,
    restartApp,
    type UpdateCheckResult,
    type UpdateDownloadProgress,
  } from '$lib/api/tauri';
  import { t } from '$lib/i18n';
  import { toast } from 'svelte-sonner';

  let checking = $state(false);
  let downloading = $state(false);
  let needsRestart = $state(false);
  let availableVersion = $state<string | null>(null);
  let progress = $state<number | null>(null);
  let update: UpdateCheckResult | null = null;

  const handleCheck = async (event: MouseEvent) => {
    event.preventDefault();
    if (checking || downloading) return;
    checking = true;
    needsRestart = false;
    availableVersion = null;
    progress = null;

    try {
      const result = await toast.promise(checkForUpdates(), {
        id: 'check-updates',
        loading: $t('app.updatesChecking'),
        success: (res) =>
          res?.available
            ? $t('app.updateAvailable', { values: { version: res.version ?? '' } })
            : $t('app.updatesLatest'),
        error: $t('app.updatesError'),
      });

      if (result.available) {
        update = result;
        availableVersion = result.version ?? '';
      } else {
        update = null;
      }
    } finally {
      checking = false;
    }
  };

  const handleInstall = async (event: MouseEvent) => {
    event.preventDefault();
    if (!update?.downloadAndInstall || downloading) return;
    downloading = true;
    progress = 0;

    try {
      await toast.promise(
        update.downloadAndInstall((p?: UpdateDownloadProgress) => {
          if (!p) return;
          if (p.percent !== undefined) {
            progress = Math.min(100, Math.max(0, Math.round(p.percent)));
          }
        }),
        {
          id: 'download-update',
          loading: $t('app.updateDownloading'),
          success: $t('app.updateRestart'),
          error: $t('app.updatesError'),
        }
      );
      needsRestart = true;
      availableVersion = null;
      progress = null;
      update = null;
    } finally {
      downloading = false;
    }
  };

  const handleRestart = async (event: MouseEvent) => {
    event.preventDefault();
    await restartApp();
  };

  const handleLater = (event: MouseEvent) => {
    event.preventDefault();
    availableVersion = null;
    update = null;
    progress = null;
  };
</script>

<div class="flex flex-col gap-2">
  {#if needsRestart}
    <button
      type="button"
      data-button
      class="inline-flex items-center gap-2"
      onclick={handleRestart}
    >
      {$t('app.updateRestart')}
    </button>
  {:else if availableVersion}
    <div class="text-sm text-muted-foreground">
      {$t('app.updateAvailable', { values: { version: availableVersion } })}
    </div>
    <div class="flex items-center gap-2">
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
    </div>
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
