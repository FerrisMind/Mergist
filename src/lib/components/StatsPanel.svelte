<script lang="ts">
  import FileIcon from '@lucide/svelte/icons/file';
  import ListIcon from '@lucide/svelte/icons/list';
  import GaugeIcon from '@lucide/svelte/icons/gauge';
  import FolderIcon from '@lucide/svelte/icons/folder';
  import FileXIcon from '@lucide/svelte/icons/file-x';
  import HardDriveIcon from '@lucide/svelte/icons/hard-drive';
  import HashIcon from '@lucide/svelte/icons/hash';
  import CircleDotIcon from '@lucide/svelte/icons/circle-dot';
  import CircleCheckIcon from '@lucide/svelte/icons/circle-check';
  import Clock3Icon from '@lucide/svelte/icons/clock-3';
  import type { ConversionResult, IssuesExportResult } from '$lib/types';
  import { t } from '$lib/i18n';

  let {
    conversion = null,
    issues = null,
    tokenStatus = 'idle',
    tokenProgress = 0,
  }: {
    conversion?: ConversionResult | null;
    issues?: IssuesExportResult | null;
    tokenStatus?: 'idle' | 'running' | 'success' | 'error';
    tokenProgress?: number;
  } = $props();

</script>

{#if conversion}
  <div class="grid gap-2 sm:gap-4 grid-cols-2 sm:grid-cols-3">
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <FileIcon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground"
          >{$t('stats.filesProcessed')}</span
        >
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right">
          {conversion.stats.files_processed}
        </span>
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <ListIcon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground">
          {$t('stats.lines')}
        </span>
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right">
          {conversion.stats.total_lines}
        </span>
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <GaugeIcon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground">
          {$t('stats.tokens')}
        </span>
        {#if tokenStatus === 'running'}
          <span class="ml-auto text-base font-semibold text-primary leading-tight text-right"
            >{Math.round(tokenProgress)}%</span
          >
        {:else}
          <span
            class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right"
          >
            {conversion.stats.token_count ?? '—'}
          </span>
        {/if}
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <FolderIcon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground">
          {$t('stats.totalFiles')}
        </span>
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right">
          {conversion.stats.total_files ??
            conversion.stats.files_processed + conversion.stats.files_skipped}
        </span>
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <FileXIcon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground"
          >{$t('stats.skippedFiles')}</span
        >
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right">
          {conversion.stats.files_skipped}
        </span>
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <HardDriveIcon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground">
          {$t('stats.fileSize')}
        </span>
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right">
          {Math.round(conversion.stats.total_size_bytes / 1024)} KB
        </span>
      </p>
    </div>
  </div>
{/if}

{#if issues}
  <div class="grid gap-2 sm:gap-4 grid-cols-2 sm:grid-cols-4">
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <HashIcon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground">
          {$t('stats.issuesTotal')}
        </span>
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right"
          >{issues.stats.total}</span
        >
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <CircleDotIcon class="h-4 w-4 text-amber-500 shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground">
          {$t('stats.issuesOpen')}
        </span>
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right"
          >{issues.stats.open}</span
        >
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <CircleCheckIcon class="h-4 w-4 text-emerald-500 shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground">
          {$t('stats.issuesClosed')}
        </span>
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right"
          >{issues.stats.closed}</span
        >
      </p>
    </div>
    <div class="rounded-lg border border-border bg-card p-4 flex items-center gap-4">
      <Clock3Icon class="h-4 w-4 text-muted-foreground shrink-0" />
      <p class="flex w-full items-center gap-2 text-sm text-foreground">
        <span class="text-[11px] uppercase tracking-wide text-muted-foreground"
          >{$t('stats.lastUpdated')}</span
        >
        <span class="ml-auto text-base font-semibold text-card-foreground leading-tight text-right">
          {issues.stats.latest_updated ?? '—'}
        </span>
      </p>
    </div>
  </div>
{/if}
