<script lang="ts">
  import '$lib/styles/tokens.css';
  import tartanvoteLogo from '$lib/assets/tartanvote-logo.svg';

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={tartanvoteLogo} />
</svelte:head>

{@render children()}
