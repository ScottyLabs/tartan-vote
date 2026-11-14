<script lang="ts">
  import { page } from "$app/state";
  import { toErrorMessage } from "$lib/errors";
  import BriefcaseBusiness from "@lucide/svelte/icons/briefcase-business";
  import FileUser from "@lucide/svelte/icons/file-user";
  import Link2 from "@lucide/svelte/icons/link-2";
  import Radio from "@lucide/svelte/icons/radio";
  import Users from "@lucide/svelte/icons/users";
  import { fetchAttendance } from "$lib/api/attendance";
  import {
    checkMotion,
    createMotion,
    exportMotionResults,
  } from "$lib/api/motion";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import EndSessionDialog from "$lib/components/EndSessionDialog.svelte";
  import HostConfigurationDialog from "$lib/components/HostConfigurationDialog.svelte";
  import ProxyRequestDialog from "$lib/components/ProxyRequestDialog.svelte";
  import {
    attendeeToHostParticipant,
    buildMotionData,
    parseHostVoteKind,
    voteOptionsFromMotionData,
    type HostedVoteInput,
    type HostDialog,
    type HostParticipant,
    type HostVoteKind,
  } from "$lib/domain/host";
  import {
    displayHostSessionCode,
    endHostedSessionWithRedirect,
    ensureHostedSessionCode,
    hostInviteUrl,
  } from "$lib/domain/hostSession";
  import { loadOrganizationSettings } from "$lib/domain/organizationSettings";
  import { onMount } from "svelte";

  const initialDialog = page.url.searchParams.get("dialog");
  const initiallyCompact = page.url.searchParams.get("compact") === "true";
  const avatarClasses = [
    "bg-blue-100 text-blue-500",
    "bg-blue-500 text-blue-100",
    "bg-blue-200 text-blue-600",
    "bg-blue-600 text-blue-200",
    "bg-blue-300 text-blue-700",
    "bg-blue-700 text-blue-300",
    "bg-blue-400 text-blue-800",
    "bg-blue-800 text-blue-400",
  ];
  const refreshIntervalMs = 5000;

  let compactMode = $state(initiallyCompact);
  let dialog = $state<HostDialog>(
    initialDialog === "motion" ||
      initialDialog === "election" ||
      initialDialog === "proxy"
      ? initialDialog
      : null,
  );
  let participants = $state<HostParticipant[]>([]);
  let selectedIds = $state<number[]>([]);
  let activeParticipantId = $state<number | null>(null);
  let activeVoteName = $state<string | null>(null);
  let activeVoteKind = $state<HostVoteKind>("motion");
  let activeVoteOptions = $state<string[]>([]);
  let activeVotingType = $state("Standard");
  let activeQuorum = $state("");
  let votesSubmitted = $state(0);
  let eligibleVotes = $state(0);
  let endSessionDialogOpen = $state(false);
  let sessionCode = $state(page.url.searchParams.get("sessionCode")?.trim() || "");
  let endingSession = $state(false);
  let startingVote = $state(false);
  let sessionError = $state("");
  let overviewError = $state("");
  let inviteCopied = $state(false);

  const selectedParticipant = $derived(
    participants.find((participant) => participant.id === activeParticipantId),
  );
  const allSelected = $derived(
    participants.length > 0 && selectedIds.length === participants.length,
  );
  const hasActiveVote = $derived(Boolean(activeVoteName));
  const hasBulkSelection = $derived(selectedIds.length > 0);
  const displayedSessionCode = $derived(displayHostSessionCode(sessionCode));
  const overviewHref = $derived.by(() => {
    const params = new URLSearchParams();
    if (sessionCode) params.set("sessionCode", sessionCode);
    return params.size > 0 ? `/host?${params.toString()}` : "/host";
  });
  const configurationHref = $derived.by(() => {
    const params = new URLSearchParams();
    if (sessionCode) params.set("sessionCode", sessionCode);
    return params.size > 0 ? `/host/configuration?${params.toString()}` : "/host/configuration";
  });
  const liveViewHref = $derived.by(() => {
    const params = new URLSearchParams({
      type: activeVoteKind,
      title: activeVoteName ?? "QuickVote",
      labels:
        activeVoteOptions.length >= 2
          ? activeVoteOptions.join("|")
          : "Pass|Reject|Abstain",
      style: activeVotingType,
      quorum: activeQuorum,
      eligible: String(eligibleVotes),
      submitted: String(votesSubmitted),
    });
    if (sessionCode) {
      params.set("sessionCode", sessionCode);
    }
    return `/host/live?${params.toString()}`;
  });

  function applyActiveMotion(motion: {
    name: string;
    data: Record<string, unknown>;
  }) {
    activeVoteName = motion.name;
    activeVoteKind = parseHostVoteKind(motion.data.kind);
    activeVoteOptions = voteOptionsFromMotionData(motion.data);
    activeVotingType =
      typeof motion.data.voting_type === "string" && motion.data.voting_type.trim()
        ? motion.data.voting_type
        : "Standard";
    activeQuorum =
      typeof motion.data.quorum === "string" ? motion.data.quorum.trim() : "";
  }

  function clearActiveMotion() {
    activeVoteName = null;
    activeVoteKind = "motion";
    activeVoteOptions = [];
    activeVotingType = "Standard";
    activeQuorum = "";
    votesSubmitted = 0;
  }

  async function refreshAttendance(code: string) {
    const attendance = await fetchAttendance(code);
    const nextParticipants = attendance.attendees.map(attendeeToHostParticipant);
    const nextIds = new Set(nextParticipants.map((participant) => participant.id));
    participants = nextParticipants;
    selectedIds = selectedIds.filter((id) => nextIds.has(id));
    eligibleVotes = attendance.headcount;
  }

  async function refreshMotionState() {
    const { active_motion: activeMotion } = await checkMotion();
    if (!activeMotion) {
      clearActiveMotion();
      return;
    }

    applyActiveMotion(activeMotion);
    try {
      const exported = await exportMotionResults();
      votesSubmitted = exported.totals.total;
    } catch {
      // Host totals are best-effort while a vote is running.
    }
  }

  async function refreshHostOverview(code = sessionCode) {
    if (!code) return;
    try {
      await Promise.all([refreshAttendance(code), refreshMotionState()]);
      overviewError = "";
    } catch (error) {
      overviewError = toErrorMessage(error, "Failed to load host overview.");
    }
  }

  onMount(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;

    async function refresh() {
      await refreshHostOverview();
      if (!cancelled) timer = setTimeout(() => void refresh(), refreshIntervalMs);
    }

    void ensureHostedSessionCode(sessionCode).then((code) => {
      if (cancelled || !code) return;
      sessionCode = code;
      void refresh();
    });

    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  });

  function toggleCompactMode() {
    compactMode = !compactMode;
    selectedIds = [];
  }

  function toggleParticipant(id: number) {
    selectedIds = selectedIds.includes(id)
      ? selectedIds.filter((selectedId) => selectedId !== id)
      : [...selectedIds, id];
  }

  function toggleAll() {
    selectedIds = allSelected
      ? []
      : participants.map((participant) => participant.id);
  }

  function unsupportedParticipantAction(action: string) {
    sessionError = `${action} is not available yet.`;
  }

  function setSelectedProxyStatus(status: "accepted" | "declined") {
    unsupportedParticipantAction(
      status === "accepted" ? "Accepting proxy votes" : "Revoking proxy votes",
    );
  }

  function kickSelected() {
    unsupportedParticipantAction("Kicking participants");
  }

  function showProxyRequest(participant: HostParticipant) {
    activeParticipantId = participant.id;
    dialog = "proxy";
  }

  function closeDialog() {
    dialog = null;
    activeParticipantId = null;
  }

  function updateParticipant(_status: "accepted" | "declined") {
    unsupportedParticipantAction("Updating proxy status");
    closeDialog();
  }

  function setParticipantStatus(_id: number, _status: "accepted" | "declined") {
    unsupportedParticipantAction("Revoking proxy votes");
  }

  function kickParticipant(id: number) {
    unsupportedParticipantAction("Kicking participants");
    if (activeParticipantId === id) closeDialog();
  }

  function proxySummary(participant: HostParticipant) {
    if (participant.proxyVotes.length === 0) return "Proxy Vote(s): N/A";
    const prefix =
      participant.proxyStatus === "accepted"
        ? "Accepted Proxy Vote(s):"
        : participant.proxyStatus === "declined"
          ? "Unaccepted Proxy Vote(s):"
          : "Proxy Vote(s):";
    return `${prefix} ${participant.proxyVotes.join(", ")}`;
  }

  async function copyInviteLink() {
    if (!sessionCode) {
      sessionError = "Session code is not available yet.";
      return;
    }
    try {
      await navigator.clipboard.writeText(hostInviteUrl(sessionCode));
      inviteCopied = true;
      sessionError = "";
      window.setTimeout(() => {
        inviteCopied = false;
      }, 2000);
    } catch {
      sessionError = "Failed to copy invite link.";
    }
  }

  async function createHostedVote(input: HostedVoteInput) {
    if (startingVote || hasActiveVote) return;
    startingVote = true;
    sessionError = "";
    try {
      const data = buildMotionData(input);
      const created = await createMotion({
        name: input.name,
        data,
      });
      applyActiveMotion({
        name: created.name,
        data,
      });
      votesSubmitted = 0;
      eligibleVotes = participants.length;
      closeDialog();
      await refreshHostOverview();
    } catch (error) {
      sessionError = toErrorMessage(error, "Failed to start voting instance.");
    } finally {
      startingVote = false;
    }
  }

  async function startQuickVote() {
    const organizationSettings = loadOrganizationSettings();
    const options =
      organizationSettings.quickVoteOptions.length >= 2
        ? organizationSettings.quickVoteOptions
        : ["Yes", "No"];
    await createHostedVote({
      name: "QuickVote",
      kind: "quick-vote",
      options,
      votingType: "Standard",
      displayLiveResults: true,
      allowProxyVoting: false,
      quorum: organizationSettings.quorum,
      description: "QuickVote",
    });
  }

  async function startConfiguredVote(configuration: {
    kind: "motion" | "election";
    name: string;
    options: string[];
    votingType: string;
    displayLiveResults: boolean;
    allowProxyVoting: boolean;
    enableQuorum: boolean;
    quorum: string;
  }) {
    await createHostedVote({
      name: configuration.name,
      kind: configuration.kind,
      options: configuration.options,
      votingType: configuration.votingType,
      displayLiveResults: configuration.displayLiveResults,
      allowProxyVoting: configuration.allowProxyVoting,
      quorum: configuration.enableQuorum ? configuration.quorum : "",
      description: configuration.name,
    });
  }

  async function confirmEndSession() {
    if (endingSession) return;
    endingSession = true;
    const success = await endHostedSessionWithRedirect((message) => {
      sessionError = message;
    });
    if (!success) {
      endingSession = false;
      endSessionDialogOpen = false;
    }
  }
</script>

<svelte:head>
  <title>TartanVote | Host Dashboard</title>
</svelte:head>

<div
  class="grid h-[calc(100svh-clamp(60px,5.833vw,112px)-81px)] min-h-[calc(100svh-clamp(60px,5.833vw,112px)-81px)] grid-cols-[clamp(203px,19.792vw,380px)_minmax(0,1fr)]"
>
    <aside
      class="relative bg-grey-900 text-white"
      aria-label="Host dashboard navigation"
    >
      <section
        class="h-[clamp(115px,11.198vw,215px)] bg-grey-800 px-[clamp(14px,1.354vw,26px)] pt-[clamp(22px,2.188vw,42px)]"
        aria-labelledby="session-code-label"
      >
        <h2
          class="mt-0 mb-[clamp(4px,1.042vw,20px)] text-[clamp(10px,0.938vw,18px)] leading-[clamp(14px,1.354vw,26px)] text-grey-500"
          id="session-code-label"
        >
          Session Code
        </h2>
        <p
          class="mt-0 mb-[clamp(6px,1.667vw,32px)] text-[clamp(17px,1.563vw,30px)] leading-[clamp(22px,1.979vw,38px)] font-semibold"
        >
          {displayedSessionCode}
        </p>
        <button
          class="inline-flex h-6 max-w-full cursor-pointer items-center gap-2 bg-transparent p-0 text-[clamp(9px,0.833vw,16px)] text-red-200"
          type="button"
          onclick={() => void copyInviteLink()}
        >
          <span class="min-w-0 overflow-hidden text-ellipsis whitespace-nowrap"
            >{inviteCopied ? "Invite link copied" : "Copy invite link"}</span
          >
          <span
            class="grid size-[clamp(10px,1.042vw,20px)] shrink-0 place-items-center"
            aria-hidden="true"
          >
            <Link2
              class="block size-full text-red-200"
              aria-hidden="true"
            />
          </span>
        </button>
      </section>

      <nav
        class="mt-[clamp(12px,1.198vw,23px)] grid gap-1"
        aria-label="Session sections"
      >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full items-center bg-[#66363e] px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-white"
          href={overviewHref}
          aria-current="page">Overview</a
        >
        {#if hasActiveVote}
          <a
            class="flex h-[clamp(40px,3.646vw,70px)] w-full items-center px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-white"
            href={liveViewHref}>Live View</a
          >
        {:else}
          <button
            class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-not-allowed items-center bg-transparent px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-grey-100"
            type="button"
            disabled
            title="Live View is available while a vote is running"
          >
            Live View
          </button>
        {/if}
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full items-center px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-white"
          href={configurationHref}>Session Configuration</a
        >
        <button
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center bg-transparent px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-white"
          type="button">Comprehensive Session Results</button
        >
      </nav>

      <button
        class="absolute bottom-9 left-1/2 h-[clamp(34px,3.125vw,60px)] w-[min(calc(100%-42px),300px)] -translate-x-1/2 cursor-pointer rounded-full border border-red-500 bg-transparent text-[clamp(14px,1.25vw,24px)] text-grey-200"
        type="button"
        onclick={() => (endSessionDialogOpen = true)}>End session</button
      >
    </aside>

    <section
      class="flex min-h-0 flex-col pt-[clamp(22px,2.188vw,42px)] pr-[clamp(32px,3.229vw,62px)] pb-3 pl-[clamp(24px,2.344vw,45px)] max-[1200px]:px-6"
      aria-label="Host overview"
    >
      <section
        class="flex h-[clamp(93px,9.01vw,173px)] shrink-0 justify-between rounded-lg border border-grey-200 bg-white px-[clamp(15px,1.51vw,29px)] py-[clamp(11px,1.146vw,22px)]"
        aria-label="Currently running voting instance"
      >
        <div>
          <div class="flex items-center gap-2">
            <Radio
              class="size-[clamp(18px,1.667vw,32px)]"
              strokeWidth={2.5}
              aria-hidden="true"
            />
            <h1 class="text-[clamp(14px,1.25vw,24px)] leading-8">
              Currently Running ({hasActiveVote
                ? activeVoteKind === "election"
                  ? "Election"
                  : activeVoteKind === "quick-vote"
                    ? "Quick Vote"
                    : "Motion"
                : "N/A"}):
            </h1>
          </div>
          <p
            class={[
              "mt-[clamp(2px,0.781vw,15px)] mr-0 mb-0 ml-2 text-[clamp(22px,1.875vw,36px)] leading-11 italic",
              !hasActiveVote && "text-grey-500",
            ]}
          >
            {activeVoteName ?? "N/A"}
          </p>
        </div>
        <p class="mt-1 mb-0 text-[clamp(14px,1.25vw,24px)] leading-8">
          Votes Submitted:
          <span class="text-grey-600"
            >{hasActiveVote
              ? `${votesSubmitted}/${eligibleVotes}`
              : "N/A"}</span
          >
        </p>
      </section>

      <div
        class="mt-[clamp(13px,1.198vw,23px)] grid min-h-0 flex-1 grid-cols-[minmax(0,1fr)_clamp(194px,19.01vw,365px)] items-start gap-[clamp(16px,1.615vw,31px)]"
      >
        <section
          class="h-full min-h-0 overflow-hidden rounded-lg border border-grey-200 bg-white"
          aria-labelledby="participants-title"
        >
          <header
            class="flex h-[clamp(38px,3.646vw,70px)] items-center justify-between border-b border-grey-300 bg-grey-50 px-[clamp(13px,1.354vw,26px)]"
          >
            <div class="flex items-center gap-[clamp(7px,0.781vw,15px)]">
              <Users
                class="size-[clamp(18px,1.615vw,31px)] text-grey-900"
                strokeWidth={2.5}
                aria-hidden="true"
              />
              <h2
                class="text-[clamp(12px,1.042vw,20px)] leading-7 font-medium"
                id="participants-title"
              >
                Participants: {participants.length}
              </h2>
            </div>
            <div
              class="flex items-center gap-2 text-[clamp(12px,1.042vw,20px)] leading-7 font-medium"
            >
              <FileUser
                class="size-[clamp(18px,1.615vw,31px)] text-black"
                strokeWidth={2.5}
                aria-hidden="true"
              />
              <span>Compact Mode:</span>
              <button
                class={[
                  "relative h-6 w-11 cursor-pointer rounded-2xl border border-slate-900",
                  compactMode ? "bg-slate-900" : "bg-white",
                ]}
                type="button"
                role="switch"
                aria-label="Compact mode"
                aria-checked={compactMode}
                onclick={toggleCompactMode}
              >
                <span
                  class={[
                    "absolute top-1 left-1 size-4 rounded-full transition-transform duration-150",
                    compactMode
                      ? "translate-x-5 bg-white"
                      : "bg-slate-900",
                  ]}
                ></span>
              </button>
            </div>
          </header>

          {#if compactMode}
            <div
              class="grid h-[clamp(38px,3.646vw,70px)] grid-cols-[1.05fr_1.1fr_0.7fr_auto] items-center gap-3 overflow-hidden px-3"
            >
              <button
                class="h-[clamp(24px,2.083vw,40px)] min-w-0 cursor-pointer truncate rounded-full bg-green-400 px-[clamp(10px,0.833vw,16px)] text-[clamp(9px,0.833vw,16px)] leading-6 text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-green-100 disabled:text-green-600 disabled:shadow-none"
                type="button"
                disabled={!hasBulkSelection}
                onclick={() => setSelectedProxyStatus("accepted")}
                >Accept all selected Proxy Votes</button
              >
              <button
                class="h-[clamp(24px,2.083vw,40px)] min-w-0 cursor-pointer truncate rounded-full bg-slate-500 px-[clamp(10px,0.833vw,16px)] text-[clamp(9px,0.833vw,16px)] leading-6 text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500 disabled:shadow-none"
                type="button"
                disabled={!hasBulkSelection}
                onclick={() => setSelectedProxyStatus("declined")}
                >Revoke all selected Proxy Votes</button
              >
              <button
                class="h-[clamp(24px,2.083vw,40px)] min-w-0 cursor-pointer truncate rounded-full bg-slate-800 px-[clamp(10px,0.833vw,16px)] text-[clamp(9px,0.833vw,16px)] leading-6 text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-grey-200 disabled:text-grey-600 disabled:shadow-none"
                type="button"
                disabled={!hasBulkSelection}
                onclick={kickSelected}>Kick all selected</button
              >
              <label
                class="flex h-[clamp(24px,2.083vw,40px)] items-center justify-end gap-2 text-[clamp(11px,0.938vw,18px)] leading-none whitespace-nowrap"
                >Select All <input
                  class="size-[clamp(14px,1.042vw,20px)] accent-slate-900"
                  type="checkbox"
                  checked={allSelected}
                  onchange={toggleAll}
                /></label
              >
            </div>
            <div
              class="h-[calc(100%-clamp(76px,7.292vw,140px))] overflow-y-auto"
            >
              {#each participants as participant (participant.id)}
                <div
                  class="flex h-[clamp(18px,1.823vw,35px)] items-center gap-2 bg-grey-50 px-[clamp(14px,1.458vw,28px)] even:bg-white"
                >
                  <button
                    class="flex min-w-0 flex-1 cursor-pointer flex-row items-start overflow-hidden bg-transparent p-0 text-left text-[clamp(9px,0.833vw,16px)] whitespace-nowrap"
                    type="button"
                    onclick={() => showProxyRequest(participant)}
                  >
                    <span class="text-black">{participant.name}</span><span
                      class={participant.proxyStatus === "accepted"
                        ? "text-green-500"
                        : participant.proxyStatus === "pending" ||
                            participant.proxyStatus === "declined"
                          ? "text-red-500"
                          : "text-grey-500"}
                      >{" - "}{proxySummary(participant)}</span
                    >
                  </button>
                  <input
                    class="size-[clamp(14px,1.042vw,20px)] accent-slate-900"
                    type="checkbox"
                    aria-label={`Select ${participant.name}`}
                    checked={selectedIds.includes(participant.id)}
                    onchange={() => toggleParticipant(participant.id)}
                  />
                </div>
              {/each}
            </div>
          {:else}
            <div
              class="h-[calc(100%-clamp(38px,3.646vw,70px))] overflow-y-auto"
            >
              {#each participants as participant, index (participant.id)}
                <article
                  class="grid min-h-[clamp(42px,4.167vw,80px)] grid-cols-[clamp(28px,2.604vw,50px)_minmax(0,1fr)_auto] items-center gap-[clamp(8px,0.833vw,16px)] bg-grey-50 px-[clamp(14px,2.604vw,50px)] py-1 even:bg-white"
                >
                  <div
                    class={[
                      "grid size-[clamp(28px,2.604vw,50px)] place-items-center rounded-full text-[clamp(14px,1.25vw,24px)] font-bold",
                      avatarClasses[index % avatarClasses.length],
                    ]}
                    aria-hidden="true"
                  >
                    {participant.initials}
                  </div>
                  <button
                    class="flex min-w-0 cursor-pointer flex-col items-start overflow-hidden bg-transparent p-0 text-left"
                    type="button"
                    onclick={() => showProxyRequest(participant)}
                  >
                    <strong
                      class="text-[clamp(12px,0.938vw,18px)] leading-7"
                      >{participant.name}</strong
                    >
                    <span
                      class={[
                        "max-w-full truncate text-[clamp(10px,0.833vw,16px)]",
                        participant.proxyStatus === "accepted"
                          ? "text-green-500"
                          : participant.proxyStatus === "pending" ||
                              participant.proxyStatus === "declined"
                            ? "text-red-500"
                            : "text-grey-600",
                      ]}
                    >
                      {proxySummary(participant)}
                    </span>
                  </button>
                  <div class="flex gap-4 max-[1200px]:gap-2">
                    {#if participant.proxyStatus === "pending"}
                      <button
                        class="h-[clamp(24px,2.083vw,40px)] w-[clamp(96px,9.375vw,180px)] cursor-pointer rounded-full bg-green-400 text-[clamp(10px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
                        type="button"
                        onclick={() => showProxyRequest(participant)}
                        >Accept Proxy</button
                      >
                    {:else if participant.proxyStatus === "accepted"}
                      <button
                        class="h-[clamp(24px,2.083vw,40px)] w-[clamp(96px,9.375vw,180px)] cursor-pointer rounded-full bg-slate-500 text-[clamp(10px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
                        type="button"
                        onclick={() =>
                          setParticipantStatus(participant.id, "declined")}
                        >Revoke Proxy</button
                      >
                    {/if}
                    <button
                      class="h-[clamp(24px,2.083vw,40px)] w-[clamp(54px,5.208vw,100px)] cursor-pointer rounded-full bg-slate-800 text-[clamp(10px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
                      type="button"
                      onclick={() => kickParticipant(participant.id)}
                      >Kick</button
                    >
                  </div>
                </article>
              {/each}
            </div>
          {/if}
        </section>

        <section
          class="min-h-0 overflow-hidden rounded-lg border border-grey-500 bg-white"
          aria-labelledby="actions-title"
        >
          <header
            class="flex h-[clamp(38px,3.646vw,70px)] items-center justify-between border-b border-grey-300 bg-grey-50 px-[clamp(14px,1.406vw,27px)]"
          >
            <div class="flex items-center gap-[clamp(7px,0.781vw,15px)]">
              <BriefcaseBusiness
                class="size-[clamp(18px,1.615vw,31px)] text-grey-900"
                strokeWidth={2.5}
                aria-hidden="true"
              />
              <h2
                class="text-[clamp(12px,1.042vw,20px)] leading-7 font-medium"
                id="actions-title"
              >
                Actions
              </h2>
            </div>
          </header>
          <div
            class="grid justify-items-center gap-[clamp(10px,1.042vw,20px)] pt-[clamp(15px,1.458vw,28px)] pr-[clamp(11px,1.094vw,21px)] pb-[clamp(11px,1.094vw,21px)] pl-[clamp(11px,1.094vw,21px)]"
          >
            <button
              class="h-[clamp(28px,2.604vw,50px)] w-[min(100%,280px)] cursor-pointer rounded-full bg-red-600 text-[clamp(11px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none"
              type="button"
              disabled={hasActiveVote || startingVote}
              onclick={() => (dialog = "motion")}>+ Push a Motion</button
            >
            <button
              class="h-[clamp(28px,2.604vw,50px)] w-[min(100%,280px)] cursor-pointer rounded-full bg-red-600 text-[clamp(11px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none"
              type="button"
              disabled={hasActiveVote || startingVote}
              onclick={() => (dialog = "election")}>+ Push an Election</button
            >
            <div
              class="grid w-full justify-items-center gap-[clamp(10px,1.042vw,20px)]"
            >
              <button
                class="h-[clamp(28px,2.604vw,50px)] w-[min(100%,280px)] cursor-pointer rounded-full bg-red-600 text-[clamp(11px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none"
                type="button"
                disabled={hasActiveVote || startingVote}
                onclick={() => void startQuickVote()}
                >{startingVote ? "Starting..." : "+ Quick Vote"}</button
              >
              <p
                class="mt-[clamp(8px,0.833vw,16px)] mb-0 grid min-h-[clamp(36px,3.125vw,60px)] w-full place-items-center bg-grey-50 px-3 py-2 text-center text-[clamp(9px,0.833vw,16px)] leading-[1.35] text-grey-600"
              >
                {hasActiveVote
                  ? "You must end your current voting instance to begin a new one."
                  : "Create a new voting instance"}
              </p>
            </div>
          </div>
        </section>
      </div>
      {#if sessionError || overviewError}
        <p class="mt-2 text-sm font-semibold text-red-700" role="alert">
          {sessionError || overviewError}
        </p>
      {/if}
    </section>
  </div>

<AppFooter wide />

{#if dialog === "motion" || dialog === "election"}
  <HostConfigurationDialog
    kind={dialog}
    onclose={closeDialog}
    onsubmit={(configuration) => void startConfiguredVote(configuration)}
  />
{:else if dialog === "proxy"}
  <ProxyRequestDialog
    requester={selectedParticipant?.name ?? "Participant"}
    proxyVotes={selectedParticipant?.proxyVotes}
    onclose={closeDialog}
    onaccept={() => updateParticipant("accepted")}
    ondecline={() => updateParticipant("declined")}
    onkick={() =>
      activeParticipantId !== null && kickParticipant(activeParticipantId)}
  />
{/if}

{#if endSessionDialogOpen}
  <EndSessionDialog
    onclose={() => (endSessionDialogOpen = false)}
    onconfirm={() => void confirmEndSession()}
  />
{/if}
