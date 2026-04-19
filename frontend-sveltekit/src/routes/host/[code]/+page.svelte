<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { get } from "svelte/store";
  import { api, downloadBlob } from "$lib/api";
  import { currentEventEndTime } from "$stores/session";
  import Logo from "$components/Logo.svelte";
  import Chip from "$components/Chip.svelte";
  import Button from "$components/Button.svelte";
  import StatCard from "$components/StatCard.svelte";
  import Modal from "$components/Modal.svelte";
  import LongTextInput from "$components/LongTextInput.svelte";
  import SelectMenu from "$components/SelectMenu.svelte";
  import ArrayEditor from "$components/ArrayEditor.svelte";
  import TimeScroller from "$components/TimeScroller.svelte";
  import HostShell from "$components/HostShell.svelte";

  let code = $derived($page.params.code);
  let attendees = $state<any[]>([]);
  let headcount = $state(0);
  let activeEvent = $state<null | {
    id: number;
    name: string;
    event_type: string;
    end_time?: string | null;
  }>(null);

  // Overview timer tick
  let nowTick = $state(Date.now());
  let tickId: ReturnType<typeof setInterval> | null = null;
  let endingActive = $state(false);

  function formatActiveRemaining(): string {
    const endTime = activeEvent?.end_time ?? get(currentEventEndTime);
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

  let activeTimeRemaining = $derived(formatActiveRemaining());

  // Motion draft
  let showMotion = $state(false);
  let m_title = $state("");
  let m_description = $state("");
  let m_preset = $state<"standard" | "secret">("standard");
  let m_voteOptions = $state<string[]>(["Pass", "Reject", "Abstain"]);
  let m_threshold = $state<"majority" | "two-thirds" | "unanimous">("majority");
  let m_meetingDisplay = $state<"named_by_category" | "totals_only">(
    "named_by_category",
  );
  let m_resultVisibility = $state<"live" | "hidden_until_release">("live");
  let m_proxy = $state(true);
  let m_exportScope = $state<"totals_only" | "full_ballots">("totals_only");
  let m_time = $state<Time>({ days: 0, hours: 0, mins: 3, secs: 0 });
  let m_submitting = $state(false);
  let m_error = $state("");

  // Election draft
  let showElection = $state(false);
  let e_title = $state("");
  let e_candidates = $state<string[]>([""]);
  let e_style = $state<"plurality" | "ranked" | "approval">("plurality");
  let e_ballotStyle = $state<"secret" | "open">("secret");
  let e_time = $state<Time>({ days: 0, hours: 0, mins: 5, secs: 0 });
  let e_submitting = $state(false);
  let e_error = $state("");

  // Export
  let exportKind = $state<
    "attendance-pdf" | "attendance-csv" | "votes-pdf" | "votes-csv"
  >("attendance-pdf");
  let exporting = $state(false);

  // End meeting confirmation
  let showEndConfirm = $state(false);
  let endingMeeting = $state(false);

  let pollId: ReturnType<typeof setInterval> | null = null;

  async function refresh() {
    try {
      const att = await api.attendance(code);
      attendees = att.attendees ?? [];
      headcount = att.headcount ?? 0;
    } catch {
      /* ignore */
    }
    try {
      const { active_event } = await api.checkEvent(code);
      activeEvent = active_event as any;
    } catch {
      /* ignore */
    }
  }

  function timeToSeconds(t: Time): number {
    return t.days * 86400 + t.hours * 3600 + t.mins * 60 + t.secs;
  }

  function thresholdFraction(v: typeof m_threshold): number {
    if (v === "two-thirds") return 2 / 3;
    if (v === "unanimous") return 1;
    return 0.5;
  }

  async function pushMotion() {
    if (!m_title.trim()) {
      m_error = "Title is required.";
      return;
    }
    m_submitting = true;
    m_error = "";
    try {
      const durationSec = Math.max(1, timeToSeconds(m_time));
      const start = new Date();
      const end = new Date(start.getTime() + durationSec * 1000);

      currentEventEndTime.set(end.toISOString());
      const event = await api.createEvent(code, {
        name: m_title.trim(),
        event_type: "Motion",
        start_time: start.toISOString(),
        end_time: end.toISOString(),
        data: {
          session_code: code,
          vote_type: "motion",
          threshold: thresholdFraction(m_threshold),
          description: m_description,
          vote_options: m_voteOptions.filter((o) => o.trim()),
          proxy: m_proxy,
          ballot_style: m_preset === "secret" ? "secret" : "standard",
          meeting_display: m_meetingDisplay,
          export_scope: m_exportScope,
          visibility: {
            participants:
              m_resultVisibility === "live" ? "live" : "hidden_until_release",
          },
        },
      });

      showMotion = false;
      goto(`/host/${code}/motion/${event.id}`);
    } catch (e: any) {
      m_error = e?.message ?? "Could not push motion.";
    } finally {
      m_submitting = false;
    }
  }

  async function pushElection() {
    if (!e_title.trim()) {
      e_error = "Title is required.";
      return;
    }
    const candidates = e_candidates.map((c) => c.trim()).filter(Boolean);
    if (candidates.length < 2) {
      e_error = "Add at least two candidates.";
      return;
    }

    e_submitting = true;
    e_error = "";
    try {
      const durationSec = Math.max(1, timeToSeconds(e_time));
      const start = new Date();
      const end = new Date(start.getTime() + durationSec * 1000);

      currentEventEndTime.set(end.toISOString());
      const event = await api.createEvent(code, {
        name: e_title.trim(),
        event_type: "Election",
        start_time: start.toISOString(),
        end_time: end.toISOString(),
        data: {
          session_code: code,
          vote_type: "election",
          election_style: e_style,
          threshold: 0.5,
          vote_options: candidates,
          proxy: true,
          ballot_style: e_ballotStyle === "secret" ? "secret" : "standard",
          visibility: { participants: "live" },
        },
      });

      showElection = false;
      goto(`/host/${code}/motion/${event.id}`);
    } catch (err: any) {
      e_error = err?.message ?? "Could not push election.";
    } finally {
      e_submitting = false;
    }
  }

  async function doExport() {
    exporting = true;
    try {
      const [kind, format] = exportKind.split("-") as [
        "attendance" | "votes",
        "pdf" | "csv",
      ];
      const blob = await api.exportBlob(code, kind, format);
      downloadBlob(blob, `${code}-${kind}.${format}`);
    } catch {
      /* noop */
    } finally {
      exporting = false;
    }
  }

  async function endActiveEvent() {
    if (!activeEvent) return;
    endingActive = true;
    try {
      await api.endEvent(activeEvent.id);
      activeEvent = null;
      currentEventEndTime.set(null);
      await refresh();
    } catch {
      /* noop */
    } finally {
      endingActive = false;
    }
  }

  async function endMeeting() {
    endingMeeting = true;
    try {
      await api.endSession(code);
      showEndConfirm = false;
      goto("/join");
    } catch {
      /* noop */
    } finally {
      endingMeeting = false;
    }
  }

  onMount(() => {
    refresh();
    pollId = setInterval(refresh, 3000);
    tickId = setInterval(() => { nowTick = Date.now(); }, 1000);
  });
  onDestroy(() => {
    if (pollId) clearInterval(pollId);
    if (tickId) clearInterval(tickId);
  });
</script>

<HostShell
  sessionCode={code}
  active="overview"
  participantCount={headcount}
  onEndMeeting={() => (showEndConfirm = true)}
>
  <div class="flex items-start justify-between gap-4 mb-6">
    <div>
      <h1 class="serif text-[36px] md:text-[40px] leading-[1.05] tracking-tight">
        Meeting overview
      </h1>
      <div class="flex items-center gap-1.5 text-[13px] text-ink-500 mt-1 flex-wrap">
        <span>Meeting code <b class="text-ink-900">{code}</b></span>
        <span>·</span>
        <span>{headcount} in attendance</span>
        <span>·</span>
        <span class="flex items-center gap-1 font-medium" style="color: var(--accent-emerald, #10b981);">
          <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse inline-block"></span>
          Live
        </span>
      </div>
    </div>
    <div class="flex items-center gap-2">
      <div class="flex">
        <select
          class="input text-[13px] rounded-r-none border-r-0 py-2.5 pr-8"
          bind:value={exportKind}
        >
          <option value="attendance-pdf">Attendance — PDF</option>
          <option value="attendance-csv">Attendance — CSV</option>
          <option value="votes-pdf">Votes — PDF</option>
          <option value="votes-csv">Votes — CSV</option>
        </select>
        <Button onclick={doExport} disabled={exporting} class="rounded-l-none">
          {exporting ? "Working…" : "Download"}
        </Button>
      </div>
    </div>
  </div>

  <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
    <StatCard
      label="Participants"
      value={headcount}
      hint={`${headcount} checked in`}
    />
    <StatCard
      label="Active event"
      value={activeEvent ? activeEvent.name : "—"}
      hint={activeEvent ? activeEvent.event_type : "None running"}
    />
    <StatCard label="Meeting code" value={code} hint="Share with voters" />
    <StatCard
      label="Quorum"
      value={headcount >= 20 ? "Met" : "Waiting"}
      tone={headcount >= 20 ? "emerald" : "default"}
      hint={`${headcount} / 20 required`}
    />
  </div>

  <div class="grid grid-cols-12 gap-5">
    <!-- Participants -->
    <div class="col-span-12 lg:col-span-7 card p-5">
      <div class="flex items-center justify-between mb-3">
        <div class="font-semibold text-[15px]">Participants</div>
      </div>

      {#if attendees.length === 0}
        <div class="text-sm text-ink-500 py-6 text-center">
          No participants yet. Share the meeting code <b class="text-ink-900"
            >{code}</b
          >.
        </div>
      {:else}
        <div class="divide-y divide-ink-200">
          {#each attendees as person}
            <div class="flex items-center gap-3 py-2.5">
              <div
                class="w-8 h-8 rounded-full bg-scarlet-100 text-scarlet-700 grid place-items-center text-xs font-bold"
              >
                {(person.name ?? "??")
                  .toString()
                  .split(" ")
                  .map((w: string) => w[0])
                  .join("")
                  .slice(0, 2)
                  .toUpperCase()}
              </div>
              <div class="flex-1">
                <div class="text-sm font-semibold">
                  {person.name ?? "Unknown"}
                </div>
                <div class="text-[11px] text-ink-500">
                  {person.role ?? "Member"}
                  {#if person.proxy_for?.toString().trim()}
                    · proxy for {person.proxy_for}
                  {/if}
                </div>
              </div>
              <Chip variant="live" pulse>Present</Chip>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Quick actions -->
    <div class="col-span-12 lg:col-span-5 space-y-5">
      <div class="card p-5">
        <div class="font-semibold text-[15px] mb-3">Quick actions</div>
        <div class="grid grid-cols-2 gap-3">
          <button
            class="btn-primary py-3.5 flex flex-col items-start px-4"
            onclick={() => (showMotion = true)}
          >
            <span
              class="flex items-center gap-1.5 text-[11px] uppercase tracking-widest opacity-80 font-semibold"
            >
              <svg
                width="12"
                height="12"
                fill="none"
                stroke="currentColor"
                stroke-width="2.2"
                viewBox="0 0 24 24"
              >
                <path d="M12 5v14" /><path d="M5 12h14" />
              </svg>
              Motion
            </span>
            <span class="text-[15px] font-semibold mt-1">Push a motion</span>
          </button>
          <button
            class="btn-primary py-3.5 flex flex-col items-start px-4"
            onclick={() => (showElection = true)}
          >
            <span
              class="flex items-center gap-1.5 text-[11px] uppercase tracking-widest opacity-80 font-semibold"
            >
              <svg
                width="12"
                height="12"
                fill="none"
                stroke="currentColor"
                stroke-width="2.2"
                viewBox="0 0 24 24"
              >
                <path d="M8 7V5a4 4 0 118 0v2" /><rect
                  x="4"
                  y="7"
                  width="16"
                  height="14"
                  rx="2"
                />
              </svg>
              Election
            </span>
            <span class="text-[15px] font-semibold mt-1">Push an election</span>
          </button>
        </div>
      </div>

      {#if activeEvent}
        <div class="card p-5">
          <div class="flex items-center justify-between">
            <div>
              <div class="font-semibold text-[15px]">Currently running</div>
              <div class="text-xs text-ink-500 mt-0.5">{activeEvent.event_type}</div>
            </div>
            <Chip variant="live" pulse>Live</Chip>
          </div>
          <div class="serif text-[22px] mt-2">{activeEvent.name}</div>

          {#if activeTimeRemaining}
            <div class="flex items-center gap-1.5 mt-2 text-[12px] text-ink-500">
              <svg width="11" height="11" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <circle cx="12" cy="12" r="9" /><path d="M12 7v5l3 2" />
              </svg>
              {activeTimeRemaining} remaining
            </div>
          {/if}

          <div class="grid grid-cols-2 gap-2 mt-4">
            <Button
              variant="ghost"
              onclick={endActiveEvent}
              disabled={endingActive}
            >
              {endingActive ? 'Ending…' : 'End now'}
            </Button>
            <Button
              onclick={() => goto(`/host/${code}/motion/${activeEvent!.id}`)}
            >
              Live view
            </Button>
          </div>
        </div>
      {/if}
    </div>
  </div>
</HostShell>

<!-- Push Motion Modal -->
<Modal
  open={showMotion}
  eyebrow="Motion"
  title="New motion"
  subtitle="Draft before pushing. Voters won't see this until you publish."
  size="lg"
  onClose={() => (showMotion = false)}
>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <div class="space-y-5">
      <div>
        <label for="m-title" class="label mb-2 block">Title</label>
        <input
          id="m-title"
          class="input w-full"
          bind:value={m_title}
          placeholder="e.g. Approve the Q2 budget"
        />
      </div>
      <div class="grid grid-cols-2 gap-3">
        <SelectMenu
          label="Preset"
          bind:value={m_preset}
          options={[
            { label: "Standard vote", value: "standard" },
            { label: "Secret ballot", value: "secret" },
          ]}
        />
        <SelectMenu
          label="Threshold"
          bind:value={m_threshold}
          options={[
            { label: "Majority", value: "majority" },
            { label: "Two-thirds", value: "two-thirds" },
            { label: "Unanimous", value: "unanimous" },
          ]}
        />
      </div>
      <LongTextInput
        label="Description"
        bind:value={m_description}
        placeholder="Motion text as it will appear to voters."
      />
      <ArrayEditor
        label="Vote options"
        bind:items={m_voteOptions}
        placeholder="Option label"
      />
    </div>

    <div class="space-y-5">
      <div class="grid grid-cols-2 gap-3">
        <SelectMenu
          label="In-meeting display"
          bind:value={m_meetingDisplay}
          options={[
            { label: "Show who voted", value: "named_by_category" },
            { label: "Totals only", value: "totals_only" },
          ]}
        />
        <SelectMenu
          label="Result visibility"
          bind:value={m_resultVisibility}
          options={[
            { label: "Live totals", value: "live" },
            { label: "Hidden until release", value: "hidden_until_release" },
          ]}
        />
      </div>

      <SelectMenu
        label="Meeting export"
        bind:value={m_exportScope}
        options={[
          { label: "Save totals only", value: "totals_only" },
          { label: "Save full ballots", value: "full_ballots" },
        ]}
      />

      <label
        class="flex items-center gap-3 p-3 rounded-xl border border-ink-200 cursor-pointer hover:bg-ink-50"
      >
        <input type="checkbox" class="hidden" bind:checked={m_proxy} />
        <span
          class="w-10 h-6 rounded-full relative transition"
          style="background: {m_proxy ? 'var(--scarlet-500)' : '#B7BECD'}"
        >
          <span
            class="absolute top-0.5 w-5 h-5 rounded-full bg-white shadow transition-all"
            style="left: {m_proxy ? '18px' : '2px'}"
          ></span>
        </span>
        <div>
          <div class="text-sm font-semibold">Enable proxy voting</div>
          <div class="text-[11px] text-ink-500">
            Delegates can cast on behalf of absent senators.
          </div>
        </div>
      </label>

      <TimeScroller label="Time limit" bind:value={m_time} />

      {#if m_error}
        <div class="text-sm text-scarlet-500">{m_error}</div>
      {/if}
    </div>
  </div>

  {#snippet footer()}
    <Button variant="ghost" size="sm" onclick={() => (showMotion = false)}
      >Cancel</Button
    >
    <div class="ml-auto">
      <Button onclick={pushMotion} disabled={m_submitting}>
        {m_submitting ? "Pushing…" : "Push motion to voters"}
      </Button>
    </div>
  {/snippet}
</Modal>

<!-- Push Election Modal -->
<Modal
  open={showElection}
  eyebrow="Election"
  title="Create election"
  subtitle="Candidates, style, and a time limit. Voters see a plurality or ranked ballot."
  onClose={() => (showElection = false)}
>
  <div class="space-y-5">
    <div>
      <label for="e-title" class="label mb-2 block">Title</label>
      <input
        id="e-title"
        class="input w-full"
        bind:value={e_title}
        placeholder="e.g. VP Finance 2026"
      />
    </div>

    <ArrayEditor
      label="Candidates"
      bind:items={e_candidates}
      addLabel="Add candidate"
      placeholder="Candidate name"
    />

    <div class="grid grid-cols-2 gap-3">
      <SelectMenu
        label="Election style"
        bind:value={e_style}
        options={[
          { label: "Plurality (pick one)", value: "plurality" },
          { label: "Ranked choice", value: "ranked" },
          { label: "Approval", value: "approval" },
        ]}
      />
      <SelectMenu
        label="Ballot visibility"
        bind:value={e_ballotStyle}
        options={[
          { label: "Secret ballot", value: "secret" },
          { label: "Open ballot", value: "open" },
        ]}
      />
    </div>

    <TimeScroller label="Time limit" bind:value={e_time} />

    {#if e_error}
      <div class="text-sm text-scarlet-500">{e_error}</div>
    {/if}
  </div>

  {#snippet footer()}
    <Button variant="ghost" size="sm" onclick={() => (showElection = false)}
      >Cancel</Button
    >
    <div class="ml-auto">
      <Button onclick={pushElection} disabled={e_submitting}>
        {e_submitting ? "Pushing…" : "Push election"}
      </Button>
    </div>
  {/snippet}
</Modal>

<!-- End meeting confirmation -->
<Modal
  open={showEndConfirm}
  title="End this meeting?"
  subtitle="All voters will be returned to the join screen and the session will close."
  onClose={() => (showEndConfirm = false)}
>
  <div class="text-sm text-ink-500">
    Meeting code <b class="text-ink-900">{code}</b> · {headcount} attendee(s). You
    can still export records after closing.
  </div>
  {#snippet footer()}
    <Button variant="ghost" size="sm" onclick={() => (showEndConfirm = false)}
      >Cancel</Button
    >
    <div class="ml-auto">
      <Button onclick={endMeeting} disabled={endingMeeting}>
        {endingMeeting ? "Ending…" : "Yes, end meeting"}
      </Button>
    </div>
  {/snippet}
</Modal>
