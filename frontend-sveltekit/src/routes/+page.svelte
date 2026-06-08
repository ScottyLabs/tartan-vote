<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fetchAuthStatus } from '$lib/auth-client';

  onMount(async () => {
    const status = await fetchAuthStatus();
    if (status?.logged_in) {
      goto('/join');
    } else {
      goto('/signin');
    }
  });
</script>

<div class="min-h-screen grid place-items-center">
  <div class="text-ink-500 text-sm">Loading TartanVote…</div>
</div>
