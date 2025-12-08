<script lang="ts">
  import { onMount } from 'svelte';
  import { getVersion } from '@tauri-apps/api/app';
  import Badge from '$lib/components/ui/badge/badge.svelte';
  import { t } from '$lib/i18n';

  let version = $state('');

  onMount(() => {
    getVersion()
      .then((v) => (version = v))
      .catch(() => {});
  });
</script>

<div class="about-badges">
  {#if version}
    <Badge variant="default">v{version}</Badge>
  {/if}
  <Badge variant="secondary">
    {$t('app.aboutDeveloper', { values: { developer: 'FerrisMind' } })}
  </Badge>
  <Badge variant="outline">
    {$t('app.aboutLicense', { values: { license: 'Apache 2.0' } })}
  </Badge>
</div>

<style>
  .about-badges {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
    margin-left: 1rem;
    margin-top: 0;
    min-height: 2.4rem;
  }
</style>
