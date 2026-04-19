<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { get } from "svelte/store";
  import { api } from "$lib/api";
  import {
    sessionCode as sessionCodeStore,
    currentEvent,
    currentEventEndTime,
  } from "$stores/session";
  import Logo from "$components/Logo.svelte";
  import Chip from "$components/Chip.svelte";
  import Button from "$components/Button.svelte";
  import VoteOption from "$components/VoteOption.svelte";

  let code = $state("");
  let eventId = $derived($page.params.eventId);

  type Event = {
    id: number;
    name: string;
    event_type: string;
    data: EventData;
  } | null;
  let evt = $state<Event>(null);

  let voteOptions = $state<string[]>([]);
  let voteInstances = $state<
    Array<{
      voter_instance_id: number;
      is_proxy: boolean;
      proxy_for_user_id: number | null;
      has_voted: boolean;
    }>
  >([]);
  let availableInstances = $derived(
    voteInstances.filter(
      (vi) => !vi.has_voted && ((evt?.data?.proxy ?? true) || !vi.is_proxy),
    ),
  );

  let selectedOption = $state<string>("");
  let rankedChoices = $state<string[]>([]); // for ranked-choice elections
  let approvalChoices = $state<Set<string>>(new Set()); // for approval elections
  let selectedInstanceId = $state<number | null>(null);
  let loading = $state(true);
  let submitting = $state(false);
  let error = $state("");

  // Countdown timer
  let nowTick = $state(Date.now());
  let tickId: ReturnType<typeof setInterval> | null = null;

  function formatTimeRemaining(): string {
    const endTime = get(currentEventEndTime);
    if (!endTime) return '';
    const ms = new Date(endTime).getTime() - nowTick;
    if (ms <= 0) return '00:00';
    const total = Math.floor(ms / 1000);
    const h = Math.floor(total / 3600);
    const m = Math.floor((total % 3600) / 60);
    const s = total % 60;
    if (h > 0) return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }

  let timeRemaining = $derived(formatTimeRemaining());

  // Derived election style flags
  let electionStyle = $derived(
    (evt?.data as any)?.election_style ?? "plurality",
  );
  let isRanked = $derived(electionStyle === "ranked");
  let isApproval = $derived(electionStyle === "approval");

  let pollId: ReturnType<typeof setInterval> | null = null;

  function accentFor(opt: string): "emerald" | "scarlet" | "amber" | "none" {
    const o = opt.toLowerCase();
    if (["yes", "pass", "approve"].includes(o)) return "emerald";
    if (["no", "reject", "deny"].includes(o)) return "scarlet";
    if (["abstain"].includes(o)) return "amber";
    return "none";
  }

  function descFor(opt: string): string {
    const o = opt.toLowerCase();
    if (["yes", "pass", "approve"].includes(o)) return "In favor of the motion";
    if (["no", "reject", "deny"].includes(o)) return "Against the motion";
    if (o === "abstain") return "Decline to vote";
    return "";
  }

  async function load() {
    try {
      const e = get(currentEvent);
      if (e && String(e.id) === eventId) evt = e;
      else {
        // fall back: fetch active event for code
        const { active_event } = await api.checkEvent(code);
        if (active_event && String(active_event.id) === eventId)
          evt = active_event;
      }

      const instances = await api.voteInstances(eventId);
      voteInstances = instances;

      if (evt) {
        const configured = evt.data?.vote_options?.length
          ? evt.data.vote_options
          : ["Pass", "Reject", "Abstain"];
        const proxyAllowed = evt.data?.proxy ?? true;
        voteOptions = proxyAllowed
          ? configured
          : configured.filter((o) => o.toLowerCase() !== "proxy");
      }

      if (!selectedInstanceId && availableInstances.length > 0) {
        selectedInstanceId = availableInstances[0].voter_instance_id;
      }
    } catch (e: any) {
      error = e?.message ?? "Failed to load ballot.";
    } finally {
      loading = false;
    }
  }

  async function watchActive() {
    try {
      const { active_event } = await api.checkEvent(code);
      if (!active_event || String(active_event.id) !== eventId) {
        // motion was closed — move on to session/results
        goto("/waiting");
      }
    } catch {
      /* ignore */
    }
  }

  function toggleApproval(opt: string) {
    const next = new Set(approvalChoices);
    if (next.has(opt)) next.delete(opt);
    else next.add(opt);
    approvalChoices = next;
  }

  function toggleRank(opt: string) {
    if (rankedChoices.includes(opt)) {
      rankedChoices = rankedChoices.filter((o) => o !== opt);
    } else {
      rankedChoices = [...rankedChoices, opt];
    }
  }

  async function submit() {
    let voteResponse: string[];
    if (isRanked) {
      if (rankedChoices.length === 0) {
        error = "Please rank at least one candidate.";
        return;
      }
      voteResponse = rankedChoices;
    } else if (isApproval) {
      if (approvalChoices.size === 0) {
        error = "Please approve at least one candidate.";
        return;
      }
      voteResponse = Array.from(approvalChoices);
    } else {
      if (!selectedOption) {
        error = "Please pick an option before submitting.";
        return;
      }
      voteResponse = [selectedOption];
    }
    if (!selectedInstanceId) {
      error = "No vote instance available.";
      return;
    }
    submitting = true;
    error = "";
    try {
      await api.vote(eventId, {
        vote_response: voteResponse,
        voter_instance_id: selectedInstanceId,
      });
      goto(`/results/${eventId}`);
    } catch (e: any) {
      error = e?.message ?? "Vote failed.";
      goto("/waiting");
    } finally {
      submitting = false;
    }
  }

  onMount(() => {
    code = get(sessionCodeStore) ?? "";
    if (!code) {
      goto("/join");
      return;
    }
    load();
    pollId = setInterval(watchActive, 3000);
    tickId = setInterval(() => { nowTick = Date.now(); }, 1000);
  });
  onDestroy(() => {
    if (pollId) clearInterval(pollId);
    if (tickId) clearInterval(tickId);
  });
</script>

<div class="min-h-screen p-6 md:p-10" style="background: #F7F8FC;">
  <div class="max-w-lg mx-auto">
    <div class="flex items-center gap-2 mb-4">
      <Logo size={24} />
      <div class="ml-auto"><Chip variant="muted">Session {code}</Chip></div>
    </div>

    {#if loading}
      <div class="card p-6 text-sm text-ink-500">Loading ballot…</div>
    {:else if !evt}
      <div class="card p-6 text-sm text-scarlet-500">Ballot not found.</div>
    {:else}
      <div class="card p-6">
        <div class="flex items-center justify-between mb-2">
          <div class="label">{evt.event_type} · Session {code}</div>
          <Chip variant="live" pulse>Live</Chip>
        </div>
        <div class="serif text-[26px] leading-tight">{evt.name}</div>
        {#if timeRemaining}
          <div class="flex items-center gap-1.5 mt-1 text-[12px] text-ink-500">
            <svg width="11" height="11" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
              <circle cx="12" cy="12" r="9" /><path d="M12 7v5l3 2" />
            </svg>
            {timeRemaining} remaining
          </div>
        {/if}
        {#if evt.data?.description}
          <div
            class="soft-card p-3 mt-3 text-sm italic text-ink-500 border-l-4 border-l-scarlet-500"
          >
            {evt.data.description}
          </div>
        {:else}
          <div class="soft-card p-3 mt-3 text-sm italic text-ink-500">
            No details provided.
          </div>
        {/if}

        {#if availableInstances.length > 1}
          <label for="voter-instance" class="label mt-5 mb-2 block"
            >Voting as</label
          >
          <select
            id="voter-instance"
            bind:value={selectedInstanceId}
            class="input"
          >
            {#each availableInstances as vi}
              <option value={vi.voter_instance_id}>
                {vi.is_proxy
                  ? `Proxy · instance #${vi.voter_instance_id}`
                  : `You · instance #${vi.voter_instance_id}`}
              </option>
            {/each}
          </select>
        {/if}

        {#if isRanked}
          <!-- Ranked choice: click candidates in preference order -->
          <div class="label mt-5 mb-1">Rank your choices</div>
          <div class="text-[11px] text-ink-400 mb-3">
            Click candidates in order of preference. Click again to remove.
          </div>
          <div class="space-y-2">
            {#each voteOptions as opt}
              {@const rank = rankedChoices.indexOf(opt)}
              <button
                class="w-full flex items-center gap-3 p-3 rounded-xl border text-left transition"
                style={rank >= 0
                  ? "border-color: var(--scarlet-500); background: rgba(200,16,46,0.05);"
                  : "border-color: var(--ink-200); background: white;"}
                onclick={() => toggleRank(opt)}
              >
                <span
                  class="w-7 h-7 rounded-full flex items-center justify-center text-xs font-bold shrink-0"
                  style={rank >= 0
                    ? "background: var(--scarlet-500); color: white;"
                    : "background: var(--ink-100); color: var(--ink-400);"}
                >
                  {rank >= 0 ? rank + 1 : "—"}
                </span>
                <span class="text-sm font-semibold">{opt}</span>
                {#if rank >= 0}
                  <span class="ml-auto text-[11px] text-scarlet-500"
                    >#{rank + 1} choice</span
                  >
                {/if}
              </button>
            {/each}
          </div>
          {#if rankedChoices.length > 0}
            <div class="text-[11px] text-ink-400 mt-2">
              Order: {rankedChoices.join(" → ")}
            </div>
          {/if}
        {:else if isApproval}
          <!-- Approval voting: check all you approve of -->
          <div class="label mt-5 mb-1">Approve all that apply</div>
          <div class="text-[11px] text-ink-400 mb-3">
            Select every candidate you approve of.
          </div>
          <div class="space-y-2">
            {#each voteOptions as opt}
              {@const approved = approvalChoices.has(opt)}
              <button
                class="w-full flex items-center gap-3 p-3 rounded-xl border text-left transition"
                style={approved
                  ? "border-color: var(--scarlet-500); background: rgba(200,16,46,0.05);"
                  : "border-color: var(--ink-200); background: white;"}
                onclick={() => toggleApproval(opt)}
              >
                <span
                  class="w-5 h-5 rounded border-2 flex items-center justify-center shrink-0"
                  style={approved
                    ? "border-color: var(--scarlet-500); background: var(--scarlet-500);"
                    : "border-color: var(--ink-300);"}
                >
                  {#if approved}
                    <svg
                      width="10"
                      height="10"
                      fill="none"
                      stroke="white"
                      stroke-width="3"
                      viewBox="0 0 24 24"
                    >
                      <path d="M5 12l4 4L19 6" />
                    </svg>
                  {/if}
                </span>
                <span class="text-sm font-semibold">{opt}</span>
              </button>
            {/each}
          </div>
        {:else}
          <!-- Plurality / motion: single select -->
          <div class="label mt-5 mb-2">
            {evt.event_type?.toLowerCase() === "election" ||
            evt.data?.vote_type === "election"
              ? "For this election, I vote for"
              : "Concerning this motion, I vote"}
          </div>
          <div class="space-y-2">
            {#each voteOptions as opt}
              <VoteOption
                label={opt}
                description={descFor(opt)}
                accent={accentFor(opt)}
                selected={selectedOption === opt}
                onselect={() => (selectedOption = opt)}
              />
            {/each}
          </div>
        {/if}

        {#if error}
          <div class="mt-4 text-sm text-scarlet-500">{error}</div>
        {/if}

        <Button full class="mt-5" onclick={submit} disabled={submitting}>
          {submitting ? "Submitting…" : "Submit vote"}
        </Button>
        <div class="text-[11px] text-ink-400 mt-2 text-center">
          You can change your vote until the {evt.data?.vote_type === "election"
            ? "election"
            : "motion"} closes.
        </div>
      </div>
    {/if}
  </div>
</div>
