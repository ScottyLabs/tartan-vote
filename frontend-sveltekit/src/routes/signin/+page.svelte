<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { devSignIn, fetchAuthStatus } from "$lib/auth-client";
  import Logo from "$components/Logo.svelte";
  import Button from "$components/Button.svelte";

  let loading = $state(false);
  let error = $state("");

  onMount(async () => {
    try {
      const status = await fetchAuthStatus();
      if (status?.logged_in) goto("/join");
    } catch {
      /* ignore */
    }
  });

  async function signIn() {
    loading = true;
    error = "";
    try {
      await devSignIn();
      goto("/join");
    } catch (e: any) {
      error =
        e?.message ??
        "Dev sign-in failed. Is DEV_AUTH_BYPASS enabled on the backend?";
      loading = false;
    }
  }
</script>

<div class="min-h-screen grid place-items-center p-6">
  <div
    class="w-full max-w-4xl card overflow-hidden grid grid-cols-1 md:grid-cols-2"
  >
    <!-- Editorial panel -->
    <div
      class="p-10 relative"
      style="background: linear-gradient(160deg, #FFF1F0 0%, #FFFFFF 60%);"
    >
      <Logo size={28} />
      <div class="mt-8 serif text-[42px] leading-[1.05]">
        Motions, elections, and proxies for
        <span class="text-scarlet-500">Student Orgs</span>.
      </div>
      <div class="mt-10 flex items-center gap-2 text-[12px] text-ink-400">
        <span class="pulse-dot"></span>
        Powered by ScottyLabs
      </div>
    </div>

    <!-- Form -->
    <div class="p-10">
      <div class="label">Welcome back</div>
      <div class="mt-2 text-[24px] font-semibold tracking-tight">
        Sign in to continue
      </div>
      <div class="text-[13px] text-ink-500 mt-2">
        Dev mode: sign in creates a sample user in the database without CMU SSO.
      </div>

      <Button full onclick={signIn} disabled={loading} class="mt-8">
        {#if loading}
          Signing in…
        {:else}
          Sign in (dev)
        {/if}
      </Button>

      <div
        class="flex items-center gap-2 mt-5 p-3 rounded-xl border border-ink-200 bg-ink-50"
      >
        <svg
          width="14"
          height="14"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          viewBox="0 0 24 24"
          class="text-ink-500 shrink-0"
        >
          <rect x="4" y="11" width="16" height="10" rx="2" />
          <path d="M8 11V7a4 4 0 018 0v4" />
        </svg>
        <div class="text-xs text-ink-500 leading-snug">
          CMU authentication is required to vote. Only
          <b class="text-ink-800">@andrew.cmu.edu</b> accounts can join sessions.
        </div>
      </div>

      {#if error}
        <div class="mt-4 text-sm text-scarlet-500">{error}</div>
      {/if}
    </div>
  </div>
</div>
