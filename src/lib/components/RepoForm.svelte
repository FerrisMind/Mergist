<script lang="ts">
  import { Input } from '$lib/components/ui';
  import { t } from '$lib/i18n';

  let {
    repoUrl = $bindable(''),
    onSubmit,
    onChange,
  }: {
    repoUrl?: string;
    onSubmit?: (payload: { repo: string }) => void;
    onChange?: (payload: { repo: string }) => void;
  } = $props();

  const handleSubmit = (e: Event) => {
    e.preventDefault();
    const value = repoUrl.trim();
    if (!value) return;
    onSubmit?.({ repo: value });
  };

  const handleInput = () => {
    onChange?.({ repo: repoUrl });
  };
</script>

<form class="space-y-2 sm:space-y-4" onsubmit={handleSubmit}>
  <label class="flex flex-col gap-2">
    <span class="inline-block -translate-y-1 text-sm font-semibold text-foreground"
      >{$t('repoForm.label')}</span
    >
    <Input
      type="url"
      required
      bind:value={repoUrl}
      oninput={handleInput}
      placeholder={$t('repoForm.placeholder')}
    />
  </label>
</form>
