<script lang="ts">
  interface Props {
    size?: number;
    label?: boolean;
    tone?: "default" | "muted";
  }
  let { size = 36, label = true, tone = "default" }: Props = $props();
</script>

<div class="flex items-center gap-2">
  {#if label}
    <div class="leading-tight">
      <div
        class="font-semibold text-[15px] tracking-tight"
        class:text-ink-900={tone === "default"}
        class:text-white={tone === "muted"}
      >
        TartanVote
      </div>
      {#if tone === "muted"}
        <div class="text-[10px] uppercase tracking-widest text-[#7A8299]">
          Host console
        </div>
      {/if}
    </div>
  {/if}
</div>
