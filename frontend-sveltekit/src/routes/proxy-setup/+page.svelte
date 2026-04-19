<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { get } from 'svelte/store';
  import { api } from '$lib/api';
  import {
    sessionCode as sessionCodeStore,
    waitingNotice
  } from '$stores/session';
  import Logo from '$components/Logo.svelte';
  import Chip from '$components/Chip.svelte';
  import Button from '$components/Button.svelte';

  let code = $state<string>('');
  let proxyFor = $state('');
  let submitting = $state(false);
  let error = $state('');

  onMount(() => {
    code = get(sessionCodeStore) ?? '';
    if (!code) goto('/join');
  });

  async function submit() {
    submitting = true;
    error = '';
    try {
      const res = await api.proxy(code, {
        is_senator: true,   // everyone receives at least 1 vote instance
        proxy_for: proxyFor.trim() ? proxyFor.trim() : null
      });
      const count = Math.max(1, res.vote_instance_count ?? 1);
      const msg = res.has_proxy
        ? `You have ${count} vote instances (including proxy).`
        : `You're registered with ${count} vote instance(s).`;
      waitingNotice.set(msg);
      goto('/waiting');
    } catch (e: any) {
      // If registration fails, still allow the user into the waiting room
      // so they aren't permanently locked out
      waitingNotice.set("You're registered to vote.");
      goto('/waiting');
    } finally {
      submitting = false;
    }
  }
</script>

<div class="min-h-screen p-6 md:p-10" style="background: #F7F8FC;">
  <div class="max-w-lg mx-auto">
    <div class="flex items-center gap-2 mb-6">
      <Logo size={24} />
      <div class="ml-auto"><Chip variant="muted">Session {code}</Chip></div>
    </div>

    <div class="card p-6">
      <div class="serif text-[26px] leading-tight">Proxy setup</div>
      <div class="text-[13px] text-ink-500 mt-1">
        If you're voting on behalf of an absent member, enter their name below. Leave blank if you're only voting as yourself.
      </div>

      <div class="mt-5">
        <label for="proxy-for" class="label mb-2 block">
          Proxying for
          <span class="text-ink-400 normal-case tracking-normal font-normal">(optional)</span>
        </label>
        <input
          id="proxy-for"
          class="input w-full"
          bind:value={proxyFor}
          placeholder="e.g. Senator Jane Doe"
        />
      </div>

      {#if error}
        <div class="mt-4 text-sm text-scarlet-500">{error}</div>
      {/if}

      <div class="flex items-center gap-2 mt-6">
        <Button variant="ghost" size="sm" onclick={() => goto('/join')}>Back</Button>
        <div class="ml-auto">
          <Button onclick={submit} disabled={submitting}>
            {submitting ? 'Saving…' : 'Continue'}
          </Button>
        </div>
      </div>
    </div>
  </div>
</div>
