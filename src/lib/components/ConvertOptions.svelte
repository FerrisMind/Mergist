<script lang="ts">
  import { Button, Checkbox, Textarea } from '$lib/components/ui';
  import { ScrollArea } from '$lib/components/ui/scroll-area';
  import * as Accordion from '$lib/components/ui/accordion';
  import { t } from '$lib/i18n';
  import { DEFAULT_SKIP_PATTERNS } from '$lib/stores/conversion';
  import RotateCcwIcon from '@lucide/svelte/icons/rotate-ccw';

  let {
    skipLargeFiles = $bindable(true),
    removeLicenseHeaders = $bindable(true),
    skipPatterns = $bindable(''),
    onChange = undefined,
  }: {
    skipLargeFiles?: boolean;
    removeLicenseHeaders?: boolean;
    skipPatterns?: string;
    onChange?: (payload: {
      skipLargeFiles: boolean;
      removeLicenseHeaders: boolean;
      skipPatterns: string;
    }) => void;
  } = $props();

  const emit = () =>
    onChange?.({
      skipLargeFiles,
      removeLicenseHeaders,
      // если поле очищено — передаем пустую строку, чтобы бэкенд не применял дефолтные паттерны
      skipPatterns,
    });

  const skipPatternsCount = $derived(
    skipPatterns.split('\n').filter((l) => l.trim().length).length
  );

  const isPatternsDefault = $derived(skipPatterns === DEFAULT_SKIP_PATTERNS);

  const handleResetPatterns = () => {
    skipPatterns = DEFAULT_SKIP_PATTERNS;
  };

  let lastSnapshot = '';
  let emitTimeout: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    // Trigger onChange когда значения реально изменились, с лёгким дебаунсом для больших textarea
    const snapshot = `${skipLargeFiles}|${removeLicenseHeaders}|${skipPatterns}`;
    if (snapshot === lastSnapshot) return;
    if (emitTimeout) clearTimeout(emitTimeout);
    emitTimeout = setTimeout(() => {
      lastSnapshot = snapshot;
      emit();
    }, 120);

    return () => {
      if (emitTimeout) {
        clearTimeout(emitTimeout);
        emitTimeout = null;
      }
    };
  });
</script>

<div class="flex flex-wrap items-center justify-center gap-4 sm:gap-4">
  <label class="flex items-center gap-2 text-sm text-foreground">
    <Checkbox bind:checked={skipLargeFiles} />
    <span>{$t('convertOptions.skipLarge')}</span>
  </label>

  <label class="flex items-center gap-2 text-sm text-foreground">
    <Checkbox bind:checked={removeLicenseHeaders} />
    <span>{$t('convertOptions.removeLicenses')}</span>
  </label>
</div>

<Accordion.Root type="single" value={null} class="w-full">
  <Accordion.Item value="patterns">
    <div class="flex items-center justify-between gap-3">
      <Accordion.Trigger class="flex-1">
        <span class="text-sm font-semibold">{$t('convertOptions.patterns')}</span>
        <span class="text-xs text-muted-foreground">
          {$t('convertOptions.patternsCount', { values: { count: skipPatternsCount } })}
        </span>
      </Accordion.Trigger>
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        class={`shrink-0 ${isPatternsDefault ? 'hidden' : ''}`}
        onclick={handleResetPatterns}
        aria-label={$t('convertOptions.resetDefaults')}
      >
        <RotateCcwIcon class="size-4" />
        <span class="sr-only">{$t('convertOptions.resetDefaults')}</span>
      </Button>
    </div>
    <Accordion.Content class="space-y-2 pt-0">
      <ScrollArea
        orientation="vertical"
        class="w-full h-40 sm:h-56 rounded-md border border-input bg-background"
        viewportRef={null}
      >
        <Textarea
          bind:value={skipPatterns}
          rows={6}
          placeholder={$t('convertOptions.placeholder')}
          class="w-full min-h-[224px] sm:min-h-[320px] resize-none border-0 bg-transparent outline-none p-4"
        />
      </ScrollArea>
    </Accordion.Content>
  </Accordion.Item>
</Accordion.Root>
