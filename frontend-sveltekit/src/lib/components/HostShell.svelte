<script lang="ts">
  import type { Snippet } from 'svelte';
  import Logo from './Logo.svelte';
  import { goto } from '$app/navigation';

  interface Props {
    sessionCode: string;
    active?: 'overview' | 'motions' | 'elections' | 'participants' | 'exports';
    motionCount?: number;
    participantCount?: number;
    onEndMeeting?: () => void;
    children: Snippet;
  }
  let {
    sessionCode,
    active = 'overview',
    motionCount = 0,
    participantCount = 0,
    onEndMeeting,
    children
  }: Props = $props();

  function copyLink() {
    const url = `${window.location.origin}/join?code=${sessionCode}`;
    navigator.clipboard.writeText(url);
  }
</script>

<div class="min-h-screen grid" style="grid-template-columns: 230px 1fr;">
  <aside class="bg-ink-900 text-[#C3C9D6] p-4 flex flex-col">
    <div class="px-2 py-3"><Logo size={30} tone="muted" /></div>
    <div class="h-px bg-white/5 my-3"></div>
    <div class="text-[10px] uppercase tracking-widest text-[#7A8299] px-3 mb-2">Meeting</div>

    <button
      class="nav-item {active === 'overview' ? 'active' : ''}"
      onclick={() => goto(`/host/${sessionCode}`)}
    >
      <svg width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <rect x="3" y="4" width="18" height="16" rx="2" /><path d="M3 10h18" />
      </svg>
      Overview
    </button>
    <button class="nav-item {active === 'motions' ? 'active' : ''}">
      <svg width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <path d="M3 6l9 6 9-6" /><rect x="3" y="6" width="18" height="12" rx="2" />
      </svg>
      Motions
      <span class="ml-auto text-[10px] bg-white/10 px-1.5 py-0.5 rounded">{motionCount}</span>
    </button>
    <button class="nav-item {active === 'elections' ? 'active' : ''}">
      <svg width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <path d="M16 4v16" /><path d="M8 4v16" /><path d="M4 12h16" />
      </svg>
      Elections
    </button>
    <button class="nav-item {active === 'participants' ? 'active' : ''}">
      <svg width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <circle cx="9" cy="8" r="4" /><path d="M17 11l2 2 4-4" /><path d="M2 20c1.5-3 4-4 7-4s5.5 1 7 4" />
      </svg>
      Participants
      <span class="ml-auto text-[10px] bg-white/10 px-1.5 py-0.5 rounded">{participantCount}</span>
    </button>
    <button class="nav-item {active === 'exports' ? 'active' : ''}">
      <svg width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <path d="M14 3v5h5" /><path d="M6 3h8l5 5v13H6z" />
      </svg>
      Exports
    </button>

    <div class="h-px bg-white/5 my-3"></div>
    <div class="text-[10px] uppercase tracking-widest text-[#7A8299] px-3 mb-2">Session</div>
    <div class="px-3 py-2 rounded-lg bg-white/[0.03] border border-white/5">
      <div class="text-[10px] uppercase tracking-widest text-[#7A8299]">Meeting code</div>
      <div class="font-mono text-[18px] tracking-[0.2em] text-white mt-0.5">{sessionCode}</div>
      <button class="text-[11px] text-scarlet-300 mt-1 hover:text-white" onclick={copyLink}>Copy link</button>
    </div>

    <div class="mt-auto pt-6">
      <button
        class="w-full py-2.5 text-[13px] border border-white/10 rounded-xl flex items-center justify-center gap-2"
        style="background: rgba(255,255,255,0.04); color: white;"
        onclick={onEndMeeting}
      >
        <svg width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
          <path d="M15 3h6v6" /><path d="M10 14L21 3" /><path d="M21 14v7H3V3h7" />
        </svg>
        End meeting
      </button>
    </div>
  </aside>

  <main class="p-8 bg-[#F7F8FC]">
    {@render children()}
  </main>
</div>

<style>
  :global(.nav-item) {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border-radius: 10px;
    font-size: 14px;
    font-weight: 500;
    color: #a7b0c4;
    cursor: pointer;
    width: 100%;
    text-align: left;
    transition: background 0.15s, color 0.15s;
  }
  :global(.nav-item:hover) {
    background: rgba(255, 255, 255, 0.04);
    color: #fff;
  }
  :global(.nav-item.active) {
    background: rgba(200, 16, 46, 0.12);
    color: #fff;
    box-shadow: inset 0 0 0 1px rgba(200, 16, 46, 0.35);
  }
</style>
