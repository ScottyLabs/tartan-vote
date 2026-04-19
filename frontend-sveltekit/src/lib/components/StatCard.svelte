<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    label: string;
    value: string | number;
    hint?: string;
    tone?: 'default' | 'emerald' | 'scarlet';
    trailing?: Snippet;
  }
  let { label, value, hint, tone = 'default', trailing }: Props = $props();

  const valueColor = $derived({
    default: 'text-ink-900',
    emerald: 'text-[var(--accent-emerald)]',
    scarlet: 'text-[var(--scarlet-500)]'
  }[tone]);
</script>

<div class="border border-ink-200 rounded-2xl p-[18px] bg-gradient-to-b from-white to-[#FAFBFE]">
  <div class="label">{label}</div>
  <div class="flex items-end justify-between mt-2 gap-2">
    <div class="text-[28px] font-semibold tracking-tight {valueColor} truncate">{value}</div>
    {#if trailing}
      {@render trailing()}
    {:else if hint}
      <div class="text-[11px] text-ink-500">{hint}</div>
    {/if}
  </div>
</div>
