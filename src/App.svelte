<script lang="ts">
  import { onDestroy } from 'svelte';
  import { Tabs, TabsList, TabsTrigger, TabsContent, Button, Card } from '$lib/components/ui';
  import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
  } from '$lib/components/ui/dropdown-menu';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import RepoForm from '$lib/components/RepoForm.svelte';
  import ConvertOptions from '$lib/components/ConvertOptions.svelte';
  import IssuesOptions from '$lib/components/IssuesOptions.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';
  import StatsPanel from '$lib/components/StatsPanel.svelte';
  import DownloadSection from '$lib/components/DownloadSection.svelte';
  import { Toaster } from '$lib/components/ui/sonner';
  import { Spinner } from '$lib/components/ui/spinner';
  import Globe2Icon from '@lucide/svelte/icons/globe-2';
  import SunIcon from '@lucide/svelte/icons/sun';
  import MoonIcon from '@lucide/svelte/icons/moon';
  import InfoIcon from '@lucide/svelte/icons/info';
  import { onMount } from 'svelte';
  import {
    conversionStore,
    resetState,
    setRepoUrl,
    setResult,
    setIssuesResult,
    setTab,
  } from '$lib/stores/conversion';
  import {
    downloadFile,
    listenConversionProgress,
    tokenizeFile,
    convertRepo,
    exportIssues,
  } from '$lib/api/tauri';
  import type {
    ConversionState,
    ConvertOptions as ConvertOpts,
    IssuesExportOptions,
    Tab,
  } from '$lib/types';
  import { ModeWatcher } from 'mode-watcher';
  import { toggleMode } from 'mode-watcher';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { getName, getVersion } from '@tauri-apps/api/app';
  import { toast } from 'svelte-sonner';
  import { AVAILABLE_LOCALES, locale as localeStore, setupI18n, t } from '$lib/i18n';

  setupI18n();
  let appState = $state<ConversionState | null>(null);
  const unsubscribe = conversionStore.subscribe((s) => {
    appState = s;
  });
  let unlistenProgress: (() => void) | null = null;
  const win = getCurrentWindow();
  let pendingProgress: { current: number; total: number } | null = null;
  let progressRaf: number | null = null;
  let pendingToken: number | null = null;
  let tokenRaf: number | null = null;
  const locales = AVAILABLE_LOCALES;
  let locale = $state('en');
  $effect(() => {
    const unsubLocale = localeStore.subscribe((value) => {
      locale = value ?? 'en';
    });
    return () => unsubLocale();
  });
  let localeMenuOpen = $state(false);
  const filePath = $derived(
    appState?.result?.file_path ?? appState?.issuesResult?.file_path ?? null
  );
  const hasStats = $derived(Boolean(appState?.result || appState?.issuesResult));
  let aboutName = $state('Repo to Markdown');
  let aboutVersion = $state('');

  const flushProgress = () => {
    if (!pendingProgress) {
      progressRaf = null;
      return;
    }
    const { current, total } = pendingProgress;
    pendingProgress = null;
    progressRaf = null;
    conversionStore.update((s) => ({ ...s, progress: { current, total } }));
  };

  const scheduleProgressUpdate = (current: number, total: number) => {
    pendingProgress = { current, total };
    if (progressRaf === null) {
      progressRaf = requestAnimationFrame(flushProgress);
    }
  };

  const flushToken = () => {
    if (pendingToken === null) {
      tokenRaf = null;
      return;
    }
    const percent = pendingToken;
    pendingToken = null;
    tokenRaf = null;
    conversionStore.update((s) => ({ ...s, tokenProgress: percent }));
  };

  const scheduleTokenUpdate = (percent: number) => {
    pendingToken = percent;
    if (tokenRaf === null) {
      tokenRaf = requestAnimationFrame(flushToken);
    }
  };

  const invalidRepoMessage = () => $t('repoForm.invalid');

  const normalizeRepoInput = (
    input: string
  ): { ok: true; repo: string } | { ok: false; error: string } => {
    const value = input.trim();
    if (!value) {
      return { ok: false, error: invalidRepoMessage() };
    }

    const sanitized = value.replace(/^git\+/, '').replace(/\.git$/i, '');

    // URL вида https://github.com/owner/repo[/...]
    try {
      const url = new URL(sanitized);
      if (url.hostname === 'github.com') {
        const parts = url.pathname.split('/').filter(Boolean);
        if (parts.length >= 2) {
          return { ok: true, repo: `${parts[0]}/${parts[1]}` };
        }
      }
    } catch {
      // не URL — продолжаем разбор
    }

    // SSH: git@github.com:owner/repo
    const sshMatch = sanitized.match(/^git@github\.com:([^/\s]+)\/([^/\s]+)$/i);
    if (sshMatch) {
      return { ok: true, repo: `${sshMatch[1]}/${sshMatch[2]}` };
    }

    // Короткий формат owner/repo
    const shortMatch = sanitized.match(/^([\w.-]+)\/([\w.-]+)$/);
    if (shortMatch) {
      return { ok: true, repo: `${shortMatch[1]}/${shortMatch[2]}` };
    }

    return { ok: false, error: invalidRepoMessage() };
  };

  onDestroy(() => {
    unsubscribe();
    if (unlistenProgress) unlistenProgress();
    if (progressRaf !== null) cancelAnimationFrame(progressRaf);
    if (tokenRaf !== null) cancelAnimationFrame(tokenRaf);
  });

  const onTabChange = (tab: string) => {
    setTab(tab as Tab);
    resetState(tab as Tab);
  };

  const handleConvert = async () => {
    const repoValidation = normalizeRepoInput(appState?.repoUrl ?? '');
    if (repoValidation.ok === false) {
      const message = repoValidation.error;
      conversionStore.update((s) => ({
        ...s,
        status: 'error',
        message,
        error: message,
      }));
      return;
    }
    const repoToUse = repoValidation.repo;

    conversionStore.update((s) => ({
      ...s,
      repoUrl: repoToUse,
      status: 'running',
      message: $t('status.loadingRepo'),
      error: null,
      result: null,
      issuesResult: null,
      progress: { current: 0, total: 0 },
      tokenStatus: 'idle',
      tokenProgress: 0,
    }));

    if (unlistenProgress) {
      unlistenProgress();
      unlistenProgress = null;
    }
    unlistenProgress = await listenConversionProgress('conversion-progress', (current, total) => {
      scheduleProgressUpdate(current, total);
    });

    const patternsStr = appState?.skipPatterns ?? '';
    const skipPatternsArr = patternsStr.trim().length
      ? patternsStr
          .split('\n')
          .map((p) => p.trim())
          .filter(Boolean)
      : [];

    const options: ConvertOpts = {
      include_filenames: true,
      add_separators: true,
      skip_large_files: appState?.skipLargeFiles ?? true,
      remove_license_headers: appState?.removeLicenseHeaders ?? true,
      // если пустая строка — отправляем пустой массив (значит без исключений)
      skip_patterns: skipPatternsArr,
    };

    try {
      const result = await convertRepo(repoToUse, options);
      setResult(result);
      conversionStore.update((s) => ({
        ...s,
        status: 'success',
        message: $t('status.done'),
        progress: {
          current: s.progress.total || s.progress.current,
          total: s.progress.total || s.progress.current,
        },
      }));
      await runTokenization(result.file_path, false);
    } catch (err) {
      conversionStore.update((s) => ({
        ...s,
        status: 'error',
        message: String(err),
        error: String(err),
      }));
    } finally {
      if (unlistenProgress) {
        unlistenProgress();
        unlistenProgress = null;
      }
    }
  };

  const handleIssues = async () => {
    const repoValidation = normalizeRepoInput(appState?.repoUrl ?? '');
    if (repoValidation.ok === false) {
      const message = repoValidation.error;
      conversionStore.update((s) => ({
        ...s,
        status: 'error',
        message,
        error: message,
      }));
      return;
    }
    const repoToUse = repoValidation.repo;

    conversionStore.update((s) => ({
      ...s,
      repoUrl: repoToUse,
      status: 'running',
      message: $t('status.loadingIssues'),
      error: null,
      result: null,
      issuesResult: null,
      progress: { current: 0, total: 0 },
      tokenStatus: 'idle',
      tokenProgress: 0,
    }));

    if (unlistenProgress) {
      unlistenProgress();
      unlistenProgress = null;
    }
    unlistenProgress = await listenConversionProgress('issues-progress', (current, total) => {
      scheduleProgressUpdate(current, total);
    });

    const options: IssuesExportOptions = {
      include_open: appState?.includeOpenIssues ?? true,
      include_closed: appState?.includeClosedIssues ?? true,
    };

    try {
      const result = await exportIssues(repoToUse, options);
      setIssuesResult(result);
      conversionStore.update((s) => ({
        ...s,
        status: 'success',
        message: $t('status.issuesExported'),
        progress: {
          current: s.progress.total || s.progress.current,
          total: s.progress.total || s.progress.current,
        },
      }));
      await runTokenization(result.file_path, true);
    } catch (err) {
      conversionStore.update((s) => ({
        ...s,
        status: 'error',
        message: String(err),
        error: String(err),
      }));
    } finally {
      if (unlistenProgress) {
        unlistenProgress();
        unlistenProgress = null;
      }
    }
  };

  const runTokenization = async (filePath: string, isIssues: boolean) => {
    conversionStore.update((s) => ({
      ...s,
      tokenStatus: 'running',
      tokenProgress: 0,
    }));
    try {
      const readChunk = (offset: number, size: number) =>
        invoke<string | null>('read_file_chunk', { path: filePath, offset, size });
      const tokens = await tokenizeFile(filePath, readChunk, (percent) =>
        scheduleTokenUpdate(percent)
      );
      conversionStore.update((s) => {
        if (isIssues && s.issuesResult) {
          return {
            ...s,
            tokenStatus: 'success',
            tokenProgress: 100,
            issuesResult: {
              ...s.issuesResult,
              stats: { ...s.issuesResult.stats, total: s.issuesResult.stats.total },
            },
          };
        }
        if (s.result) {
          return {
            ...s,
            tokenStatus: 'success',
            tokenProgress: 100,
            result: { ...s.result, stats: { ...s.result.stats, token_count: tokens } },
          };
        }
        return { ...s, tokenStatus: 'success', tokenProgress: 100 };
      });
    } catch (err) {
      conversionStore.update((s) => ({
        ...s,
        tokenStatus: 'error',
        message: String(err),
      }));
    }
  };

  const onDownload = async () => {
    const file = appState?.result?.file_path ?? appState?.issuesResult?.file_path;
    if (!file) return;
    const suggested = file.split(/[\\/]/).pop() ?? 'output.md';
    const target = await downloadFile(file, suggested);
    if (target) {
      conversionStore.update((s) => ({
        ...s,
        message: $t('status.saved', { values: { path: target } }),
      }));
    }
  };

  const handleMinimize = () => win.minimize();
  const handleClose = () => win.close();

  onMount(() => {
    // ModeWatcher handles syncing theme; fetch app info for About dialog
    getName().then((name) => (aboutName = name)).catch(() => {});
    getVersion().then((version) => (aboutVersion = version)).catch(() => {});
  });

  const handleAbout = () => {
    const versionText = aboutVersion ? ` v${aboutVersion}` : '';
  toast($t('app.aboutMessage', { values: { name: aboutName, version: versionText } }), {
    id: 'about-toast',
    duration: Infinity,
    class: 'about-toast',
    closeButton: true,
  });
  };
</script>

<ModeWatcher defaultMode="system" />
<svelte:head>
  <title>{$t('app.title')}</title>
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
</svelte:head>

<div class="h-screen bg-background text-foreground flex flex-col overflow-hidden">
  <Toaster position="bottom-center" closeButton richColors />
  <header class="sticky top-0 z-30 bg-background/80 backdrop-blur" data-tauri-drag-region>
    <div
      class="mx-auto w-full max-w-[900px] px-4 py-2 sm:px-4 sm:py-4 flex items-center gap-2 sm:gap-4"
    >
      <Tabs
        value={appState?.tab ?? 'code'}
        onValueChange={onTabChange}
        class="flex-1 min-w-0"
        data-tauri-drag-region="false"
      >
        <TabsList class="w-full max-w-full flex flex-nowrap gap-2 overflow-hidden">
          <TabsTrigger
            value="code"
            class="truncate"
            title={$t('app.tabs.code')}
          >
            {$t('app.tabs.code')}
          </TabsTrigger>
          <TabsTrigger
            value="issues"
            class="truncate"
            title={$t('app.tabs.issues')}
          >
            {$t('app.tabs.issues')}
          </TabsTrigger>
        </TabsList>
      </Tabs>
      <div class="flex items-center gap-0 shrink-0">
        <div class="flex items-center gap-2" data-tauri-drag-region="false">
          <DropdownMenu bind:open={localeMenuOpen}>
            <DropdownMenuTrigger>
              <Button
                variant="ghost"
                size="sm"
                class="h-8 px-2 flex items-center gap-2 border border-transparent hover:border-border data-[state=open]:border-border data-[state=open]:bg-accent/30 dark:data-[state=open]:bg-accent/50 data-[state=open]:text-accent-foreground transition-colors"
                aria-label={$t('app.languageSelect')}
                data-state={localeMenuOpen ? 'open' : 'closed'}
              >
                <Globe2Icon class="h-4 w-4" />
                <span class="text-sm"
                  >{locales.find((l) => l.value === locale)?.label ?? $t('app.language')}</span
                >
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent class="w-44">
              {#each locales as item (item.value)}
                <DropdownMenuItem
                  class="flex items-center justify-between"
                  onclick={() => localeStore.set(item.value)}
                  data-state={locale === item.value ? 'checked' : 'unchecked'}
                >
                  <span>{item.label}</span>
                  {#if locale === item.value}
                    <span class="text-xs text-muted-foreground">✓</span>
                  {/if}
                </DropdownMenuItem>
              {/each}
            </DropdownMenuContent>
          </DropdownMenu>
          <Button
            onclick={toggleMode}
            variant="ghost"
            size="icon"
            class="relative h-8 w-8 border border-transparent hover:border-border transition-colors"
            aria-label={$t('app.themeToggle')}
          >
            <SunIcon
              class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 !transition-all dark:-rotate-90 dark:scale-0"
            />
            <MoonIcon
              class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 !transition-all dark:rotate-0 dark:scale-100"
            />
            <span class="sr-only">Toggle theme</span>
          </Button>
          <Button
            onclick={handleAbout}
            variant="ghost"
            size="icon"
            class="relative h-8 w-8 border border-transparent hover:border-border transition-colors"
            aria-label={$t('app.about')}
            title={$t('app.about')}
          >
            <InfoIcon class="h-[1.1rem] w-[1.1rem]" />
          </Button>
        </div>
        <div class="w-8 h-8 sm:h-10 shrink-0" data-tauri-drag-region aria-hidden="true"></div>
        <div class="flex items-center gap-2" data-tauri-drag-region="false">
          <Button
            variant="ghost"
            size="icon-sm"
            class="h-8 w-8 border border-transparent hover:border-border"
            aria-label={$t('app.minimize')}
            onclick={handleMinimize}
          >
            –
          </Button>
          <Button
            variant="destructive"
            size="icon-sm"
            class="h-8 w-8 border border-transparent hover:border-border"
            aria-label={$t('app.close')}
            onclick={handleClose}
          >
            ×
          </Button>
        </div>
      </div>
    </div>
  </header>

  <main class="flex-1 overflow-hidden">
    <ScrollArea orientation="vertical" class="h-full">
      <div
        class="mx-auto w-full max-w-[900px] flex flex-col gap-4 sm:gap-6 px-4 py-4 sm:px-6 sm:py-6 min-w-0"
      >
        <Card class="w-full px-4 pt-4 pb-3 sm:px-6 sm:pt-6 sm:pb-3 !gap-0">
          <Tabs value={appState?.tab ?? 'code'} onValueChange={onTabChange}>
            <TabsContent value="code">
              <div class="flex flex-col gap-4 sm:gap-6">
                <RepoForm
                  repoUrl={appState?.repoUrl ?? ''}
                  onSubmit={({ repo }) => setRepoUrl(repo)}
                  onChange={({ repo }) => setRepoUrl(repo)}
                />

                <ConvertOptions
                  skipLargeFiles={appState?.skipLargeFiles ?? true}
                  removeLicenseHeaders={appState?.removeLicenseHeaders ?? true}
                  skipPatterns={appState?.skipPatterns ?? ''}
                  onChange={(v) =>
                    conversionStore.update((s) => ({
                      ...s,
                      skipLargeFiles: v.skipLargeFiles,
                      removeLicenseHeaders: v.removeLicenseHeaders,
                      skipPatterns: v.skipPatterns,
                    }))}
                />

                <div
                  class="flex flex-col gap-2 sm:gap-4 sm:flex-row sm:items-center sm:justify-start"
                >
                  <Button
                    onclick={(e) => {
                      e.preventDefault();
                      handleConvert();
                    }}
                    disabled={appState?.status === 'running'}
                    class="w-full sm:w-auto sm:min-w-[220px] -translate-y-1"
                  >
                    {#if appState?.status === 'running'}
                      <Spinner class="size-4" />
                      <span>{$t('actions.converting')}</span>
                    {:else}
                      {$t('actions.convertCode')}
                    {/if}
                  </Button>
                </div>

                {#if appState?.message}
                  <div class="mt-4">
                    <StatusBar
                      message={appState.message}
                      status={appState?.status ?? 'idle'}
                      variant={appState.status === 'error'
                        ? 'error'
                        : appState.status === 'success'
                          ? 'success'
                          : 'info'}
                    />
                  </div>
                {/if}
              </div>
            </TabsContent>

            <TabsContent value="issues">
              <div class="flex flex-col gap-4 sm:gap-6 pb-1 sm:pb-1">
                <RepoForm
                  repoUrl={appState?.repoUrl ?? ''}
                  onSubmit={({ repo }) => setRepoUrl(repo)}
                  onChange={({ repo }) => setRepoUrl(repo)}
                />

                <IssuesOptions
                  includeOpenIssues={appState?.includeOpenIssues ?? true}
                  includeClosedIssues={appState?.includeClosedIssues ?? true}
                  onChange={(v) =>
                    conversionStore.update((s) => ({
                      ...s,
                      includeOpenIssues: v.includeOpenIssues,
                      includeClosedIssues: v.includeClosedIssues,
                    }))}
                />

                <div
                  class="flex flex-col gap-2 sm:gap-4 sm:flex-row sm:items-center sm:justify-start"
                >
                  <Button
                    onclick={(e) => {
                      e.preventDefault();
                      handleIssues();
                    }}
                    disabled={appState?.status === 'running'}
                    class="w-full sm:w-auto sm:min-w-[220px]"
                  >
                    {#if appState?.status === 'running'}
                      <Spinner class="size-4" />
                      <span>{$t('actions.converting')}</span>
                    {:else}
                      {$t('actions.convertIssues')}
                    {/if}
                  </Button>
                </div>

                {#if appState?.message}
                  <div class="mt-4">
                    <StatusBar
                      message={appState.message}
                      status={appState?.status ?? 'idle'}
                      variant={appState.status === 'error'
                        ? 'error'
                        : appState.status === 'success'
                          ? 'success'
                          : 'info'}
                    />
                  </div>
                {/if}
              </div>
            </TabsContent>
          </Tabs>

          {#if appState?.status === 'running' && (appState?.progress.total ?? 0) > 0}
            <div class="-translate-y-1">
              <ProgressBar
                current={appState?.progress.current ?? 0}
                total={appState?.progress.total || appState?.progress.current || 1}
              />
            </div>
          {/if}

          {#if filePath}
            <div class="sm:mt-4 -translate-y-2">
              <DownloadSection {filePath} {onDownload} />
            </div>
          {/if}

          {#if hasStats}
            <div class="mt-4 sm:mt-4 -translate-y-2">
              <StatsPanel
                conversion={appState?.result}
                issues={appState?.issuesResult}
                tokenStatus={appState?.tokenStatus ?? 'idle'}
                tokenProgress={appState?.tokenProgress ?? 0}
              />
            </div>
          {/if}
        </Card>
      </div>
    </ScrollArea>
  </main>
</div>
