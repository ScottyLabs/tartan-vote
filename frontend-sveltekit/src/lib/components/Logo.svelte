<script lang="ts">
  interface Props {
    size?: number;
    label?: boolean;
    tone?: 'default' | 'muted';
  }
  let { size = 36, label = true, tone = 'default' }: Props = $props();
</script>

<div class="flex items-center gap-2">
  <span
    class="logo-mark"
    style="width: {size}px; height: {size}px; border-radius: {Math.round(size / 3.6)}px"
  >
    <svg
      viewBox="0 0 24 24"
      width={Math.round(size * 0.5)}
      height={Math.round(size * 0.5)}
      fill="none"
      stroke="currentColor"
      stroke-width="2.2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M20 6 9 17l-5-5" />
    </svg>
  </span>
  {#if label}
    <div class="leading-tight">
      <div
        class="font-semibold text-[15px] tracking-tight"
        class:text-ink-900={tone === 'default'}
        class:text-white={tone === 'muted'}
      >
        TartanVote
      </div>
      {#if tone === 'muted'}
        <div class="text-[10px] uppercase tracking-widest text-[#7A8299]">
          Host console
        </div>
      {/if}
    </div>
  {/if}
</div>
