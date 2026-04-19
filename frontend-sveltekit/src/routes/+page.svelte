<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authClient } from '$lib/auth-client';

  onMount(async () => {
    const session = await authClient.getSession();
    if (session?.data?.user) {
      goto('/join');
    } else {
      goto('/signin');
    }
  });
</script>

<div class="min-h-screen grid place-items-center">
  <div class="text-ink-500 text-sm">Loading TartanVote…</div>
</div>
