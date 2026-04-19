<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { api } from '$lib/api';
  import ProgressBar from './ProgressBar.svelte';

  interface Props {
    eventId: number | string;
    eventType: 'motion' | 'election';
  }
  let { eventId, eventType }: Props = $props();

  let results = $state<Record<string, unknown> | null>(null);
  let errorMsg = $state<string>('');
  let pollId: ReturnType<typeof setInterval> | null = null;

  async function tick() {
    try {
      const r = await api.eventResults(eventId);
      results = r;
      errorMsg = '';
    } catch (e: any) {
      if (e.status === 403) {
        errorMsg = 'Results hidden until release';
      } else {
        errorMsg = e.message ?? 'Failed to load results';
      }
    }
  }

  onMount(() => {
    tick();
    pollId = setInterval(tick, 3000);
  });
  onDestroy(() => {
    if (pollId) clearInterval(pollId);
  });

  // Normalized view-model for UI rendering
  const view = $derived.by(() => {
    if (!results) return null;
    if (eventType === 'motion') {
      const r = results as { pass: number; reject: number; abstain: number; total: number };
      const total = Math.max(1, r.total ?? 0);
      return {
        total: r.total ?? 0,
        rows: [
          { label: 'Pass', value: r.pass ?? 0, percent: ((r.pass ?? 0) / total) * 100, color: 'emerald' as const },
          { label: 'Reject', value: r.reject ?? 0, percent: ((r.reject ?? 0) / total) * 100, color: 'scarlet' as const },
          { label: 'Abstain', value: r.abstain ?? 0, percent: ((r.abstain ?? 0) / total) * 100, color: 'amber' as const }
        ]
      };
    } else {
      const r = results as { total: number; options: Array<{ label: string; count: number; percent: number }> };
      return {
        total: r.total ?? 0,
        rows: (r.options ?? []).map((o, i) => ({
          label: o.label,
          value: o.count,
          percent: o.percent,
          color: (['emerald', 'scarlet', 'amber', 'ink'] as const)[i % 4]
        }))
      };
    }
  });
</script>

{#if errorMsg}
  <div class="soft-card p-3 text-sm text-ink-500 italic">{errorMsg}</div>
{:else if view}
  <div class="space-y-4">
    {#each view.rows as row}
      <ProgressBar label={row.label} value={row.value} percent={row.percent} color={row.color} minBar={2} />
    {/each}
  </div>
  <div class="text-xs text-ink-500 mt-4">Total votes: <b class="text-ink-900">{view.total}</b></div>
{:else}
  <div class="text-sm text-ink-500">Loading results…</div>
{/if}
