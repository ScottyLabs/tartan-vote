<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';
  import { api } from '$lib/api';
  import { sessionCode as sessionCodeStore, currentEvent } from '$stores/session';
  import Logo from '$components/Logo.svelte';
  import Chip from '$components/Chip.svelte';
  import Button from '$components/Button.svelte';
  import ProgressBar from '$components/ProgressBar.svelte';

  let code = $state('');
  let eventId = $derived($page.params.eventId);

  type ResultBar = { label: string; value: number; percent: number; color: 'emerald' | 'scarlet' | 'amber' | 'ink' };
  let bars = $state<ResultBar[]>([]);
  let totalVotes = $state(0);
  let summary = $state('');
  let passed = $state<boolean | null>(null);
  let loading = $state(true);
  let error = $state('');
  let pollId: ReturnType<typeof setInterval> | null = null;

  async function load() {
    try {
      const e = get(currentEvent);
      const r: any = await api.eventResults(eventId);
      if (r.vote_type === 'election') {
        totalVotes = r.total ?? 0;
        bars = (r.options ?? []).map((o: any, i: number) => ({
          label: o.label,
          value: o.count,
          percent: o.percent,
          color: (['emerald', 'scarlet', 'amber', 'ink'] as const)[i % 4]
        }));
        summary = '';
        passed = null;
      } else {
        const total = Math.max(1, r.total ?? 0);
        totalVotes = r.total ?? 0;
        bars = [
          { label: 'Pass', value: r.pass ?? 0, percent: ((r.pass ?? 0) / total) * 100, color: 'emerald' },
          { label: 'Reject', value: r.reject ?? 0, percent: ((r.reject ?? 0) / total) * 100, color: 'scarlet' },
          { label: 'Abstain', value: r.abstain ?? 0, percent: ((r.abstain ?? 0) / total) * 100, color: 'amber' }
        ];
        passed = !!r.passed;
        summary = r.passed ? 'Motion is passed.' : 'Motion did not pass.';
      }
      loading = false;
    } catch (e: any) {
      error = e?.message ?? 'Failed to load results.';
      loading = false;
    }

    try {
      const s = await api.sessionStatus(code);
      if (s.session_ended) goto('/join');
    } catch {
      /* ignore */
    }
  }

  onMount(() => {
    code = get(sessionCodeStore) ?? '';
    if (!code) goto('/join');
    load();
    pollId = setInterval(load, 3000);
  });
  onDestroy(() => {
    if (pollId) clearInterval(pollId);
  });
</script>

<div class="min-h-screen p-6 md:p-10" style="background: #F7F8FC;">
  <div class="max-w-xl mx-auto">
    <div class="flex items-center gap-2 mb-4">
      <Logo size={24} />
      <div class="ml-auto"><Chip variant="muted">Session {code}</Chip></div>
    </div>

    <div class="card overflow-hidden">
      <div class="h-1.5" style="background: {passed === false ? 'var(--scarlet-500)' : 'var(--accent-emerald)'}"></div>
      <div class="p-7">
        <div class="flex items-center gap-2 mb-2">
          {#if passed === true}
            <Chip variant="pass">
              <svg width="12" height="12" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                <path d="M5 12l4 4L19 6" />
              </svg>
              Motion passed
            </Chip>
          {:else if passed === false}
            <span class="chip" style="background: rgba(200,16,46,.1); color: var(--scarlet-500);">
              <svg width="12" height="12" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                <path d="M6 6l12 12" /><path d="M18 6L6 18" />
              </svg>
              Motion failed
            </span>
          {:else}
            <Chip variant="muted">Results</Chip>
          {/if}
          <span class="tag-pill">Session {code}</span>
        </div>
        <div class="serif text-[32px] leading-tight">Motion results</div>
        <div class="text-[13px] text-ink-500 mt-1">
          Final tally below. A full audit record is available in the meeting export.
        </div>

        {#if loading}
          <div class="mt-6 text-sm text-ink-500">Tallying…</div>
        {:else if error}
          <div class="mt-6 text-sm text-scarlet-500">{error}</div>
        {:else}
          <div class="mt-6 space-y-4">
            {#each bars as row}
              <ProgressBar label={row.label} value={row.value} percent={row.percent} color={row.color} minBar={2} />
            {/each}
          </div>

          {#if summary}
            <div
              class="soft-card p-3 mt-6 text-sm italic text-ink-500 border-l-4"
              style="border-left-color: {passed ? 'var(--accent-emerald)' : 'var(--scarlet-500)'}"
            >
              {summary}
            </div>
          {/if}

          <div class="divider my-5"></div>

          <div class="flex items-center gap-4">
            <div>
              <div class="label">Total votes</div>
              <div class="text-[20px] font-semibold">{totalVotes}</div>
            </div>
            <div class="ml-auto flex items-center gap-2">
              <Button variant="ghost" size="sm" onclick={() => goto('/waiting')}>Back to session</Button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
