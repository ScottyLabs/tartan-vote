<script lang="ts">
  import type { Snippet } from 'svelte';
  interface Props {
    open: boolean;
    title?: string;
    subtitle?: string;
    eyebrow?: string;
    size?: 'sm' | 'md' | 'lg';
    onClose?: () => void;
    children: Snippet;
    footer?: Snippet;
  }
  let {
    open,
    title,
    subtitle,
    eyebrow,
    size = 'md',
    onClose,
    children,
    footer
  }: Props = $props();

  const maxW = $derived({ sm: 'max-w-sm', md: 'max-w-xl', lg: 'max-w-4xl' }[size]);
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 grid place-items-center p-6"
    style="background: linear-gradient(180deg, rgba(15,17,23,0.42) 0%, rgba(15,17,23,0.55) 100%);"
    role="dialog"
    aria-modal="true"
  >
    <div
      class="bg-white w-full {maxW} rounded-2xl shadow-modal border border-white/60 overflow-hidden"
    >
      <div class="p-7">
        <div class="flex items-start justify-between gap-4">
          <div class="flex-1">
            {#if eyebrow}<span class="tag-pill">{eyebrow}</span>{/if}
            {#if title}
              <div class="serif text-[26px] leading-tight mt-2">{title}</div>
            {/if}
            {#if subtitle}
              <div class="text-[13px] text-ink-500 mt-1">{subtitle}</div>
            {/if}
          </div>
          {#if onClose}
            <button
              onclick={onClose}
              class="w-9 h-9 grid place-items-center rounded-full hover:bg-ink-100 text-ink-500"
              aria-label="Close"
            >
              <svg width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path d="M18 6L6 18" /><path d="M6 6l12 12" />
              </svg>
            </button>
          {/if}
        </div>

        <div class="mt-5">
          {@render children()}
        </div>
      </div>
      {#if footer}
        <div class="px-7 py-5 border-t border-ink-200 flex items-center gap-2">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}
