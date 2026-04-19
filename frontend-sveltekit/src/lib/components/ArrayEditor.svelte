<script lang="ts">
  interface Props {
    items: string[];
    label?: string;
    addLabel?: string;
    placeholder?: string;
  }
  let {
    items = $bindable([]),
    label = '',
    addLabel = 'Add option',
    placeholder = ''
  }: Props = $props();

  function add() {
    items = [...items, ''];
  }
  function remove(i: number) {
    items = items.filter((_, idx) => idx !== i);
  }
</script>

{#if label}<div class="label mb-2">{label}</div>{/if}
<div class="space-y-2">
  {#each items as _, i}
    <div class="flex items-center gap-2">
      <input class="input flex-1" bind:value={items[i]} {placeholder} />
      <button
        type="button"
        onclick={() => remove(i)}
        class="w-10 h-10 rounded-lg border border-ink-200 grid place-items-center text-ink-400 hover:bg-ink-50"
        aria-label="Remove"
      >
        <svg width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
          <path d="M3 6h18" />
          <path d="M8 6V4h8v2" />
          <path d="M5 6l1 14h12l1-14" />
        </svg>
      </button>
    </div>
  {/each}
  <button
    type="button"
    onclick={add}
    class="btn-ghost w-full py-2 text-[13px] flex items-center justify-center gap-1.5"
  >
    <svg width="12" height="12" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
      <path d="M12 5v14" /><path d="M5 12h14" />
    </svg>
    {addLabel}
  </button>
</div>
