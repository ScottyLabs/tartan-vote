<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';
  import { api } from '$lib/api';
  import {
    sessionCode as sessionCodeStore,
    waitingNotice,
    currentEvent
  } from '$stores/session';
  import Logo from '$components/Logo.svelte';
  import Chip from '$components/Chip.svelte';

  let code = $state('');
  let notice = $state('');
  let pollId: ReturnType<typeof setInterval> | null = null;

  async function check() {
    if (!code) return;
    try {
      const { active_event } = await api.checkEvent(code);
      if (!active_event) return;

      // Confirm this user has an unvoted instance
      try {
        const instances = await api.voteInstances(active_event.id);
        const eligible = instances.filter((vi) => !vi.has_voted);
        const proxyAllowed = active_event.data?.proxy ?? true;
        const valid = eligible.filter((vi) => proxyAllowed || !vi.is_proxy);
        if (valid.length === 0) return;
      } catch {
        /* continue anyway — server may allow vote page to handle it */
      }

      currentEvent.set(active_event);
      goto(`/vote/${active_event.id}`);
    } catch {
      /* retry next tick */
    }
  }

  onMount(() => {
    code = get(sessionCodeStore) ?? '';
    notice = get(waitingNotice);
    if (!code) {
      goto('/join');
      return;
    }
    check();
    pollId = setInterval(check, 3000);
  });

  onDestroy(() => {
    if (pollId) clearInterval(pollId);
  });
</script>

<div class="min-h-screen p-6 md:p-10" style="background: #F7F8FC;">
  <div class="max-w-lg mx-auto">
    <div class="flex items-center gap-2 mb-6">
      <Logo size={24} />
      <div class="ml-auto"><Chip variant="muted">Session {code}</Chip></div>
    </div>

    <div class="card p-8 text-center">
      <div class="mx-auto w-12 h-12 grid place-items-center rounded-full bg-scarlet-50">
        <div
          class="w-6 h-6 border-2 border-scarlet-500 border-t-transparent rounded-full animate-spin"
        ></div>
      </div>
      <div class="serif text-[26px] leading-tight mt-4">Waiting for the next vote</div>
      <div class="text-sm text-ink-500 mt-2">
        The host will push a motion or election shortly. This page will automatically advance.
      </div>

      {#if notice}
        <div
          class="soft-card p-3 mt-5 text-sm italic text-ink-500 border-l-4 border-l-scarlet-500 text-left"
        >
          {notice}
        </div>
      {/if}
    </div>
  </div>
</div>
