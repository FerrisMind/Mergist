<script lang="ts">
  import { Checkbox } from '$lib/components/ui';
  import { t } from '$lib/i18n';

  let {
    includeOpenIssues = $bindable(true),
    includeClosedIssues = $bindable(true),
    onChange = undefined,
  }: {
    includeOpenIssues?: boolean;
    includeClosedIssues?: boolean;
    onChange?: (payload: { includeOpenIssues: boolean; includeClosedIssues: boolean }) => void;
  } = $props();

  const emit = () => onChange?.({ includeOpenIssues, includeClosedIssues });
  $effect(() => emit());
</script>

<div class="flex flex-wrap items-center justify-center gap-4 sm:gap-4">
  <label class="flex items-center gap-2 text-sm text-foreground">
    <Checkbox bind:checked={includeOpenIssues} />
    <span>{$t('issuesOptions.includeOpen')}</span>
  </label>

  <label class="flex items-center gap-2 text-sm text-foreground">
    <Checkbox bind:checked={includeClosedIssues} />
    <span>{$t('issuesOptions.includeClosed')}</span>
  </label>
</div>
