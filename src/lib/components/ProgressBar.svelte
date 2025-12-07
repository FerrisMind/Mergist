<script lang="ts">
  import { Progress } from '$lib/components/ui';

  let {
    current = 0,
    total = 0,
  }: {
    current?: number;
    total?: number;
  } = $props();

  const percent = $derived(total > 0 ? Math.min(100, Math.round((current / total) * 100)) : 0);
</script>

<div class="space-y-2">
  <Progress value={percent} />
  <p class="text-xs text-muted-foreground">{percent}%</p>
</div>
