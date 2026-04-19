<script lang="ts">
  interface Props {
    label: string;
    description?: string;
    selected?: boolean;
    accent?: 'emerald' | 'scarlet' | 'amber' | 'none';
    onselect?: () => void;
  }
  let {
    label,
    description,
    selected = false,
    accent = 'none',
    onselect
  }: Props = $props();

  const dotColor = {
    emerald: 'bg-[var(--accent-emerald)]',
    scarlet: 'bg-[var(--scarlet-500)]',
    amber: 'bg-[var(--accent-amber)]',
    none: 'bg-transparent'
  }[accent];
</script>

<button
  type="button"
  class="vote-option w-full text-left {selected ? 'selected' : ''}"
  onclick={onselect}
>
  <div class="flex items-center gap-3">
    <div class="radio-mark"></div>
    <div>
      <div class="font-semibold text-sm">{label}</div>
      {#if description}
        <div class="text-xs text-ink-500">{description}</div>
      {/if}
    </div>
  </div>
  {#if accent !== 'none'}
    <div class="w-2 h-2 rounded-full {dotColor}"></div>
  {/if}
</button>
