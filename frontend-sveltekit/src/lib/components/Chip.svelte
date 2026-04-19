<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    variant?: 'live' | 'muted' | 'pass';
    pulse?: boolean;
    children: Snippet;
  }
  let { variant = 'muted', pulse = false, children }: Props = $props();
  const cls = {
    live: 'chip chip-live',
    muted: 'chip chip-muted',
    pass: 'chip chip-pass'
  }[variant];
</script>

<span class={cls}>
  {#if pulse}<span class="pulse-dot"></span>{/if}
  {@render children()}
</span>
