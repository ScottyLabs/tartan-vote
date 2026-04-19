<script lang="ts">
  interface Props {
    label: string;
    value: number; // raw count
    percent: number; // 0..100
    color?: 'emerald' | 'scarlet' | 'amber' | 'ink';
    minBar?: number;
  }
  let {
    label,
    value,
    percent,
    color = 'emerald',
    minBar = 0
  }: Props = $props();

  const barColor = {
    emerald: 'var(--accent-emerald)',
    scarlet: 'var(--scarlet-500)',
    amber: 'var(--accent-amber)',
    ink: 'var(--ink-500)'
  }[color];

  const width = $derived(Math.max(minBar, Math.min(100, percent)));
</script>

<div>
  <div class="flex items-center justify-between mb-1.5">
    <div class="text-sm font-semibold">{label}</div>
    <div class="text-xs text-ink-500">
      <b class="text-ink-900">{value}</b> · {percent.toFixed(0)}%
    </div>
  </div>
  <div class="progress-track h-3">
    <div
      class="progress-bar transition-all"
      style="width: {width}%; background: {barColor};"
    ></div>
  </div>
</div>
