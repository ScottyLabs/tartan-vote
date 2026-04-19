<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authClient } from '$lib/auth-client';
  import { api } from '$lib/api';
  import { sessionCode as sessionCodeStore } from '$stores/session';
  import Logo from '$components/Logo.svelte';
  import Button from '$components/Button.svelte';
  import Chip from '$components/Chip.svelte';

  let code = $state('');
  let joining = $state(false);
  let creating = $state(false);
  let joinError = $state('');
  let user = $state<null | { id: string; name: string; email?: string }>(null);
  let checkingAuth = $state(true);

  onMount(async () => {
    try {
      const s = await authClient.getSession();
      user = (s?.data?.user as any) ?? null;
      if (!user) goto('/signin');
    } finally {
      checkingAuth = false;
    }
  });

  async function joinAsVoter() {
    joinError = '';
    const trimmed = code.trim().toUpperCase();
    if (!trimmed) {
      joinError = 'Enter a session code.';
      return;
    }
    joining = true;
    try {
      await api.joinSession(trimmed);
      sessionCodeStore.set(trimmed);
      goto('/proxy-setup');
    } catch (e: any) {
      joinError = e?.message ?? "We couldn't find that session.";
    } finally {
      joining = false;
    }
  }

  async function createSession() {
    creating = true;
    joinError = '';
    try {
      const { session_code } = await api.createSession();
      sessionCodeStore.set(session_code);
      goto(`/host/${session_code}`);
    } catch (e: any) {
      joinError = e?.message ?? 'Could not create session.';
    } finally {
      creating = false;
    }
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Enter') joinAsVoter();
  }

  async function signOut() {
    await authClient.signOut();
    goto('/signin');
  }
</script>

<div class="min-h-screen p-6 md:p-10" style="background: #F7F8FC;">
  <div class="max-w-5xl mx-auto">
    <header class="flex items-center gap-2 mb-10">
      <Logo size={28} />
      <div class="ml-auto flex items-center gap-3">
        {#if user}
          <Chip variant="muted">{user.name}</Chip>
          <button class="text-xs text-ink-500 hover:text-ink-900" onclick={signOut}>Sign out</button>
        {/if}
      </div>
    </header>

    {#if checkingAuth}
      <div class="text-sm text-ink-500 text-center mt-20">Loading…</div>
    {:else}
      <div class="max-w-md mx-auto">
        <div class="label text-center">Step 1 of 1</div>
        <div class="serif text-center text-[40px] leading-tight mt-2">
          Welcome, <span class="text-scarlet-500">{user?.name?.split(' ')[0] ?? 'friend'}</span>.
        </div>
        <div class="text-center text-sm text-ink-500 mt-2">
          Enter a session code to join, or host your own.
        </div>

        <div class="card p-6 mt-8">
          <label for="session-code" class="label mb-2 block">Session code</label>
          <input
            id="session-code"
            class="input input-lg w-full text-center tracking-[0.25em]"
            bind:value={code}
            onkeydown={handleKey}
            oninput={() => (code = code.toUpperCase())}
            placeholder="e.g. BUBMGC"
            maxlength="10"
          />

          <Button full class="mt-4" onclick={joinAsVoter} disabled={joining}>
            {joining ? 'Joining…' : 'Join session'}
          </Button>

          <div class="flex items-center gap-3 my-5">
            <div class="h-px flex-1 bg-ink-200"></div>
            <div class="text-[11px] text-ink-400 uppercase tracking-widest">or</div>
            <div class="h-px flex-1 bg-ink-200"></div>
          </div>

          <Button variant="ghost" full onclick={createSession} disabled={creating}>
            {creating ? 'Creating…' : 'Create a new session'}
          </Button>

          {#if joinError}
            <div class="mt-4 text-sm text-scarlet-500">{joinError}</div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</div>
