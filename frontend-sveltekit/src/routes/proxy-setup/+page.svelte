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
  import VoteOption from '$components/VoteOption.svelte';

  let code = $state<string>('');
  let isSenator = $state<'yes' | 'no' | ''>('');
  let proxyFor = $state('');
  let submitting = $state(false);
  let error = $state('');

  onMount(() => {
    code = get(sessionCodeStore) ?? '';
    if (!code) goto('/join');
  });

  async function submit() {
    if (!isSenator) {
      error = 'Please answer whether you are a Senator.';
      return;
    }
    submitting = true;
    error = '';
    try {
      const res = await api.proxy(code, {
        is_senator: isSenator === 'yes',
        proxy_for: proxyFor.trim() ? proxyFor.trim() : null
      });
      const msg = res.has_proxy
        ? `You have ${res.vote_instance_count} vote instances (including proxy).`
        : `You have ${res.vote_instance_count} vote instance(s).`;
      waitingNotice.set(msg);
      goto('/waiting');
    } catch (e: any) {
      error = e?.message ?? 'Could not save proxy setup.';
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
      <span class="tag-pill">Proxy setup · Step 1 of 2</span>
      <div class="serif text-[26px] leading-tight mt-2">Confirm your role</div>
      <div class="text-[13px] text-ink-500 mt-1">
        We use this to assign the correct number of vote instances for the session.
      </div>

      <div class="mt-5">
        <div class="label mb-2">Are you an Undergraduate Senator?</div>
        <div class="grid grid-cols-2 gap-2">
          <VoteOption
            label="Yes"
            selected={isSenator === 'yes'}
            onselect={() => (isSenator = 'yes')}
          />
          <VoteOption
            label="No"
            selected={isSenator === 'no'}
            onselect={() => (isSenator = 'no')}
          />
        </div>
      </div>

      <div class="mt-4">
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
        <div class="text-[11px] text-ink-400 mt-1">
          Leave blank if you're only voting as yourself.
        </div>
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
