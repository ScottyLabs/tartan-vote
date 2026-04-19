<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';
  import { api } from '$lib/api';
  import {
    sessionCode as sessionCodeStore,
    currentEvent
  } from '$stores/session';
  import Logo from '$components/Logo.svelte';
  import Chip from '$components/Chip.svelte';
  import Button from '$components/Button.svelte';
  import VoteOption from '$components/VoteOption.svelte';

  let code = $state('');
  let eventId = $derived($page.params.eventId);

  type Event = { id: number; name: string; event_type: string; data: EventData } | null;
  let evt = $state<Event>(null);

  let voteOptions = $state<string[]>([]);
  let voteInstances = $state<
    Array<{ voter_instance_id: number; is_proxy: boolean; proxy_for_user_id: number | null; has_voted: boolean }>
  >([]);
  let availableInstances = $derived(
    voteInstances.filter((vi) => !vi.has_voted && ((evt?.data?.proxy ?? true) || !vi.is_proxy))
  );

  let selectedOption = $state<string>('');
  let selectedInstanceId = $state<number | null>(null);
  let loading = $state(true);
  let submitting = $state(false);
  let error = $state('');

  let pollId: ReturnType<typeof setInterval> | null = null;

  function accentFor(opt: string): 'emerald' | 'scarlet' | 'amber' | 'none' {
    const o = opt.toLowerCase();
    if (['yes', 'pass', 'approve'].includes(o)) return 'emerald';
    if (['no', 'reject', 'deny'].includes(o)) return 'scarlet';
    if (['abstain'].includes(o)) return 'amber';
    return 'none';
  }

  function descFor(opt: string): string {
    const o = opt.toLowerCase();
    if (['yes', 'pass', 'approve'].includes(o)) return 'In favor of the motion';
    if (['no', 'reject', 'deny'].includes(o)) return 'Against the motion';
    if (o === 'abstain') return 'Decline to vote';
    return '';
  }

  async function load() {
    try {
      const e = get(currentEvent);
      if (e && String(e.id) === eventId) evt = e;
      else {
        // fall back: fetch active event for code
        const { active_event } = await api.checkEvent(code);
        if (active_event && String(active_event.id) === eventId) evt = active_event;
      }

      const instances = await api.voteInstances(eventId);
      voteInstances = instances;

      if (evt) {
        const configured = evt.data?.vote_options?.length
          ? evt.data.vote_options
          : ['Pass', 'Reject', 'Abstain'];
        const proxyAllowed = evt.data?.proxy ?? true;
        voteOptions = proxyAllowed ? configured : configured.filter((o) => o.toLowerCase() !== 'proxy');
      }

      if (!selectedInstanceId && availableInstances.length > 0) {
        selectedInstanceId = availableInstances[0].voter_instance_id;
      }
    } catch (e: any) {
      error = e?.message ?? 'Failed to load ballot.';
    } finally {
      loading = false;
    }
  }

  async function watchActive() {
    try {
      const { active_event } = await api.checkEvent(code);
      if (!active_event || String(active_event.id) !== eventId) {
        // motion was closed — move on to session/results
        goto('/waiting');
      }
    } catch {
      /* ignore */
    }
  }

  async function submit() {
    if (!selectedOption || !selectedInstanceId) {
      error = 'Please pick an option before submitting.';
      return;
    }
    submitting = true;
    error = '';
    try {
      await api.vote(eventId, {
        vote_response: [selectedOption],
        voter_instance_id: selectedInstanceId
      });
      goto(`/results/${eventId}`);
    } catch (e: any) {
      error = e?.message ?? 'Vote failed.';
      goto('/waiting');
    } finally {
      submitting = false;
    }
  }

  onMount(() => {
    code = get(sessionCodeStore) ?? '';
    if (!code) {
      goto('/join');
      return;
    }
    load();
    pollId = setInterval(watchActive, 3000);
  });
  onDestroy(() => {
    if (pollId) clearInterval(pollId);
  });
</script>

<div class="min-h-screen p-6 md:p-10" style="background: #F7F8FC;">
  <div class="max-w-lg mx-auto">
    <div class="flex items-center gap-2 mb-4">
      <Logo size={24} />
      <div class="ml-auto"><Chip variant="muted">Session {code}</Chip></div>
    </div>

    {#if loading}
      <div class="card p-6 text-sm text-ink-500">Loading ballot…</div>
    {:else if !evt}
      <div class="card p-6 text-sm text-scarlet-500">Ballot not found.</div>
    {:else}
      <div class="card p-6">
        <div class="flex items-center gap-2 mb-2">
          <Chip variant="live" pulse>Live</Chip>
          <span class="tag-pill">
            <svg width="12" height="12" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
              <circle cx="12" cy="12" r="9" /><path d="M12 7v5l3 2" />
            </svg>
            In session
          </span>
        </div>
        <div class="label">
          {evt.event_type} · Session {code}
        </div>
        <div class="serif text-[26px] leading-tight mt-1">{evt.name}</div>
        {#if evt.data?.description}
          <div
            class="soft-card p-3 mt-3 text-sm italic text-ink-500 border-l-4 border-l-scarlet-500"
          >
            {evt.data.description}
          </div>
        {:else}
          <div
            class="soft-card p-3 mt-3 text-sm italic text-ink-500 border-l-4 border-l-scarlet-500"
          >
            No details provided.
          </div>
        {/if}

        {#if availableInstances.length > 1}
          <label for="voter-instance" class="label mt-5 mb-2 block">Voting as</label>
          <select id="voter-instance" bind:value={selectedInstanceId} class="input">
            {#each availableInstances as vi}
              <option value={vi.voter_instance_id}>
                {vi.is_proxy ? `Proxy · instance #${vi.voter_instance_id}` : `You · instance #${vi.voter_instance_id}`}
              </option>
            {/each}
          </select>
        {/if}

        <div class="label mt-5 mb-2">
          {evt.event_type?.toLowerCase() === 'election' || evt.data?.vote_type === 'election'
            ? 'For this election, I vote for'
            : 'Concerning this motion, I vote'}
        </div>
        <div class="space-y-2">
          {#each voteOptions as opt}
            <VoteOption
              label={opt}
              description={descFor(opt)}
              accent={accentFor(opt)}
              selected={selectedOption === opt}
              onselect={() => (selectedOption = opt)}
            />
          {/each}
        </div>

        {#if error}
          <div class="mt-4 text-sm text-scarlet-500">{error}</div>
        {/if}

        <Button full class="mt-5" onclick={submit} disabled={submitting}>
          {submitting ? 'Submitting…' : 'Submit vote'}
        </Button>
        <div class="text-[11px] text-ink-400 mt-2 text-center">
          You can change your vote until the motion closes.
        </div>
      </div>
    {/if}
  </div>
</div>
