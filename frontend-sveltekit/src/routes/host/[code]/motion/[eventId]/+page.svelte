<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { api } from '$lib/api';
  import Chip from '$components/Chip.svelte';
  import Button from '$components/Button.svelte';
  import StatCard from '$components/StatCard.svelte';
  import HostShell from '$components/HostShell.svelte';
  import MotionLiveResults from '$components/MotionLiveResults.svelte';

  let code = $derived($page.params.code);
  let eventId = $derived($page.params.eventId);

  type Event = { id: number; name: string; event_type: string; data: EventData; end_time: string | null };
  let evt = $state<Event | null>(null);
  let loading = $state(true);
  let ending = $state(false);
  let error = $state('');
  let pollId: ReturnType<typeof setInterval> | null = null;

  // Computed time remaining
  let nowTick = $state(Date.now());

  async function load() {
    try {
      const { active_event } = await api.checkEvent(code);
      if (active_event && String(active_event.id) === eventId) {
        evt = active_event as unknown as Event;
      } else if (!evt) {
        // event may have ended; fetch results but no active data
        evt = null;
      }
      loading = false;
    } catch (e: any) {
      error = e?.message ?? '';
      loading = false;
    }
  }

  async function endMotion() {
    ending = true;
    try {
      await api.endEvent(eventId);
      goto(`/host/${code}`);
    } catch (e: any) {
      error = e?.message ?? 'Could not end motion.';
    } finally {
      ending = false;
    }
  }

  function formatRemaining(): string {
    if (!evt?.end_time) return '—';
    const ms = new Date(evt.end_time).getTime() - nowTick;
    if (ms <= 0) return '00:00';
    const total = Math.floor(ms / 1000);
    const m = Math.floor(total / 60);
    const s = total % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }

  let timeRemaining = $derived(formatRemaining());

  onMount(() => {
    load();
    pollId = setInterval(() => {
      nowTick = Date.now();
      load();
    }, 1000);
  });
  onDestroy(() => {
    if (pollId) clearInterval(pollId);
  });

  const kind = $derived<'motion' | 'election'>(evt?.event_type?.toLowerCase() === 'election' ? 'election' : 'motion');
</script>

<HostShell sessionCode={code} active="motions">
  <div class="flex items-center gap-2 mb-3">
    <Chip variant="live" pulse>{kind === 'election' ? 'Election running' : 'Motion running'}</Chip>
    <span class="tag-pill">
      <svg width="12" height="12" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <circle cx="12" cy="12" r="9" /><path d="M12 7v5l3 2" />
      </svg>
      {timeRemaining} left
    </span>
    <span class="tag-pill">Session {code}</span>
  </div>

  {#if loading}
    <div class="card p-6 text-sm text-ink-500">Loading…</div>
  {:else if !evt}
    <div class="card p-6">
      <div class="serif text-[28px]">Motion closed</div>
      <div class="text-sm text-ink-500 mt-1">This motion is no longer active. Review the final tally below.</div>
      <div class="mt-6"><MotionLiveResults eventId={eventId} eventType={kind} /></div>
      <div class="flex items-center gap-2 mt-6">
        <Button variant="ghost" size="sm" onclick={() => goto(`/host/${code}`)}>Back to meeting</Button>
      </div>
    </div>
  {:else}
    <div class="card p-7">
      <div class="flex items-start justify-between gap-4">
        <div>
          <div class="serif text-[32px] leading-tight">{evt.name}</div>
          <div class="text-[13px] text-ink-500 mt-1">
            Threshold: {Math.round((evt.data?.threshold ?? 0.5) * 100)}% ·
            {evt.data?.visibility?.participants === 'live' ? 'Live totals on' : 'Hidden until release'}
          </div>
        </div>
      </div>

      <div class="grid grid-cols-3 gap-4 mt-6">
        <StatCard label="Time remaining" value={timeRemaining} hint="auto-closes when zero" />
        <StatCard label="Kind" value={evt.event_type} />
        <StatCard label="Proxy voting" value={evt.data?.proxy ? 'Enabled' : 'Off'} />
      </div>

      <div class="divider my-6"></div>

      <div>
        <div class="label mb-3">Live tally</div>
        <MotionLiveResults eventId={eventId} eventType={kind} />
      </div>

      {#if error}
        <div class="mt-4 text-sm text-scarlet-500">{error}</div>
      {/if}

      <div class="flex items-center gap-2 mt-6">
        <Button variant="ghost" size="sm" onclick={() => goto(`/host/${code}`)}>Back to meeting</Button>
        <div class="ml-auto">
          <Button onclick={endMotion} disabled={ending}>
            {ending ? 'Ending…' : 'End early'}
          </Button>
        </div>
      </div>
    </div>
  {/if}
</HostShell>
