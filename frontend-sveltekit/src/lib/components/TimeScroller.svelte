<script lang="ts">
  interface Props {
    value: Time;
    label?: string;
  }
  let {
    value = $bindable({ days: 0, hours: 0, mins: 0, secs: 0 }),
    label = 'Time limit'
  }: Props = $props();

  function clamp(n: number, min = 0, max = Infinity) {
    if (Number.isNaN(n)) return min;
    return Math.max(min, Math.min(max, Math.floor(n)));
  }
</script>

{#if label}<div class="label mb-2">{label}</div>{/if}
<div class="grid grid-cols-4 gap-2">
  <div class="input flex flex-col items-center py-2">
    <input
      class="w-full bg-transparent text-[20px] font-semibold text-center outline-none"
      type="number"
      min="0"
      bind:value={value.days}
      oninput={() => (value.days = clamp(value.days))}
    />
    <div class="text-[10px] text-ink-500 uppercase tracking-widest">day</div>
  </div>
  <div class="input flex flex-col items-center py-2">
    <input
      class="w-full bg-transparent text-[20px] font-semibold text-center outline-none"
      type="number"
      min="0"
      max="23"
      bind:value={value.hours}
      oninput={() => (value.hours = clamp(value.hours, 0, 23))}
    />
    <div class="text-[10px] text-ink-500 uppercase tracking-widest">hour</div>
  </div>
  <div class="input flex flex-col items-center py-2">
    <input
      class="w-full bg-transparent text-[20px] font-semibold text-center outline-none"
      type="number"
      min="0"
      max="59"
      bind:value={value.mins}
      oninput={() => (value.mins = clamp(value.mins, 0, 59))}
    />
    <div class="text-[10px] text-ink-500 uppercase tracking-widest">minute</div>
  </div>
  <div class="input flex flex-col items-center py-2">
    <input
      class="w-full bg-transparent text-[20px] font-semibold text-center outline-none"
      type="number"
      min="0"
      max="59"
      bind:value={value.secs}
      oninput={() => (value.secs = clamp(value.secs, 0, 59))}
    />
    <div class="text-[10px] text-ink-500 uppercase tracking-widest">second</div>
  </div>
</div>
