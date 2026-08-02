<script lang="ts">
  import { page } from "$app/state";
  import BriefcaseBusiness from "@lucide/svelte/icons/briefcase-business";
  import FileUser from "@lucide/svelte/icons/file-user";
  import Link2 from "@lucide/svelte/icons/link-2";
  import Radio from "@lucide/svelte/icons/radio";
  import Users from "@lucide/svelte/icons/users";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import EndSessionDialog from "$lib/components/EndSessionDialog.svelte";
  import HostConfigurationDialog from "$lib/components/HostConfigurationDialog.svelte";
  import ProxyRequestDialog from "$lib/components/ProxyRequestDialog.svelte";
  import logoUrl from "$lib/assets/tartanvote-logo.svg?url";
  import {
    exampleParticipants,
    type HostDialog,
    type HostParticipant,
  } from "$lib/domain/host";
  import { loadOrganizationSettings } from "$lib/domain/organizationSettings";

  const initialState = page.url.searchParams.get("state");
  const initialDialog = page.url.searchParams.get("dialog");
  const initiallyCompact = page.url.searchParams.get("compact") === "true";
  const initiallySelectAll = page.url.searchParams.get("selectAll") === "true";
  const hasRunningVote = page.url.searchParams.get("running") === "true";
  const runningTitle = page.url.searchParams.get("title")?.trim() || null;
  const runningKind =
    page.url.searchParams.get("type") === "election"
      ? "election"
      : page.url.searchParams.get("type") === "quick-vote"
        ? "quick-vote"
        : "motion";
  const runningOptions = (page.url.searchParams.get("labels") ?? "")
    .split("|")
    .map((option) => option.trim())
    .filter(Boolean);
  const runningVotingType =
    page.url.searchParams.get("style")?.trim() || "Standard";
  const runningSubmitted = Math.max(
    0,
    Number.parseInt(page.url.searchParams.get("submitted") ?? "0", 10) || 0,
  );
  const runningEligible = Math.max(
    runningSubmitted,
    Number.parseInt(page.url.searchParams.get("eligible") ?? "0", 10) || 0,
  );
  const extendedParticipants: HostParticipant[] = [
    ...exampleParticipants,
    ...Array.from({ length: 30 }, (_, index) => ({
      id: index + 21,
      name: "Scotty Labs",
      initials: "SL",
      proxyStatus: "none" as const,
      proxyVotes: [],
    })),
  ];
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
  const initialParticipants =
    initialState === "inactive"
      ? []
      : initiallyCompact
        ? initiallySelectAll
          ? extendedParticipants
          : exampleParticipants.slice(0, 5)
        : exampleParticipants;
  let compactMode = $state(initiallyCompact);
  let dialog = $state<HostDialog>(
    initialDialog === "motion" ||
      initialDialog === "election" ||
      initialDialog === "proxy"
      ? initialDialog
      : null,
  );
  let participants = $state<HostParticipant[]>(
    initialParticipants.map((participant) => ({ ...participant })),
  );
  let selectedIds = $state<number[]>(
    initiallySelectAll
      ? initialParticipants.map((participant) => participant.id)
      : [],
  );
  let activeParticipantId = $state<number | null>(null);
  let quickVoteStarted = $state(false);
  let activeVoteName = $state<string | null>(runningTitle);
  let activeVoteKind = $state<"motion" | "election" | "quick-vote">(
    runningKind,
  );
  let activeVoteOptions = $state<string[]>(runningOptions);
  let activeVotingType = $state(runningVotingType);
  let activeQuorum = $state(page.url.searchParams.get("quorum")?.trim() || "");
  let votesSubmitted = $state(runningSubmitted);
  let eligibleVotes = $state(runningEligible);
  let endSessionDialogOpen = $state(false);

  const selectedParticipant = $derived(
    participants.find((participant) => participant.id === activeParticipantId),
  );
  const allSelected = $derived(
    participants.length > 0 && selectedIds.length === participants.length,
  );
  const hasActiveVote = $derived(
    hasRunningVote || quickVoteStarted || Boolean(activeVoteName),
  );
  const hasBulkSelection = $derived(selectedIds.length > 0);
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
    });
    return `/host/live?${params.toString()}`;
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

  function setSelectedProxyStatus(status: "accepted" | "declined") {
    const selected = new Set(selectedIds);
    participants = participants.map((participant) =>
      selected.has(participant.id) && participant.proxyStatus !== "none"
        ? { ...participant, proxyStatus: status }
        : participant,
    );
  }

  function kickSelected() {
    const selected = new Set(selectedIds);
    participants = participants.filter(
      (participant) => !selected.has(participant.id),
    );
    selectedIds = [];
  }

  function showProxyRequest(participant: HostParticipant) {
    activeParticipantId = participant.id;
    dialog = "proxy";
  }

  function closeDialog() {
    dialog = null;
    activeParticipantId = null;
  }

  function updateParticipant(status: "accepted" | "declined") {
    if (activeParticipantId === null) return;
    participants = participants.map((participant) =>
      participant.id === activeParticipantId
        ? { ...participant, proxyStatus: status }
        : participant,
    );
    closeDialog();
  }

  function setParticipantStatus(id: number, status: "accepted" | "declined") {
    participants = participants.map((participant) =>
      participant.id === id
        ? { ...participant, proxyStatus: status }
        : participant,
    );
  }

  function kickParticipant(id: number) {
    participants = participants.filter((participant) => participant.id !== id);
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

  function startQuickVote() {
    const organizationSettings = loadOrganizationSettings();
    quickVoteStarted = true;
    activeVoteName = "QuickVote";
    activeVoteKind = "quick-vote";
    activeVoteOptions =
      organizationSettings.quickVoteOptions.length >= 2
        ? organizationSettings.quickVoteOptions
        : ["Yes", "No"];
    activeVotingType = "Standard";
    activeQuorum = organizationSettings.quorum;
    votesSubmitted = 0;
    eligibleVotes = 0;
  }

  function startConfiguredVote(configuration: {
    kind: "motion" | "election";
    name: string;
    options: string[];
    votingType: string;
    enableQuorum: boolean;
    quorum: string;
  }) {
    activeVoteName = configuration.name;
    activeVoteKind = configuration.kind;
    activeVoteOptions = configuration.options;
    activeVotingType = configuration.votingType;
    activeQuorum = configuration.enableQuorum ? configuration.quorum : "";
    votesSubmitted = 0;
    eligibleVotes = 0;
    closeDialog();
  }
</script>

<svelte:head>
  <title>TartanVote | Host Dashboard</title>
</svelte:head>

<main
  class="relative min-h-svh w-full min-w-[1024px] overflow-hidden bg-(image:--gradient-screen-signin) text-grey-900 [&_a:focus-visible]:outline-3 [&_a:focus-visible]:outline-offset-2 [&_a:focus-visible]:outline-[color-mix(in_srgb,var(--color-red-600),transparent_70%)] [&_button:focus-visible]:outline-3 [&_button:focus-visible]:outline-offset-2 [&_button:focus-visible]:outline-[color-mix(in_srgb,var(--color-red-600),transparent_70%)]"
>
  <header
    class="flex h-[clamp(60px,5.833vw,112px)] items-center justify-between bg-red-600 pr-[clamp(34px,3.542vw,68px)] pl-[clamp(18px,1.771vw,34px)] shadow-[0_4px_4px_rgb(0_0_0_/_0.25)]"
  >
    <a
      class="flex items-center gap-2 no-underline focus-visible:outline-3 focus-visible:outline-offset-2 focus-visible:outline-[color-mix(in_srgb,var(--color-red-600),transparent_70%)]"
      href="/home"
      aria-label="TartanVote home"
    >
      <img
        class="block h-[clamp(32px,3.073vw,59px)] w-[clamp(42px,4.01vw,77px)]"
        src={logoUrl}
        alt=""
        width="77"
        height="59"
      />
      <span
        class="font-brand text-[clamp(24px,2.083vw,40px)] text-red-600 [-webkit-text-stroke:clamp(5px,0.521vw,10px)_var(--color-white)] [paint-order:stroke_fill]"
        ><span class="text-black">Tartan</span>Vote</span
      >
    </a>
    <p class="m-0 text-[clamp(18px,1.563vw,30px)] leading-[38px] text-white">
      HOSTING DASHBOARD
    </p>
  </header>

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
          happy-giraffe
        </p>
        <button
          class="inline-flex h-6 max-w-full cursor-pointer items-center gap-2 border-0 bg-transparent p-0 text-[clamp(9px,0.833vw,16px)] text-red-200"
          type="button"
        >
          <span class="min-w-0 overflow-hidden text-ellipsis whitespace-nowrap"
            >Copy invite link</span
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
        class="mt-[clamp(12px,1.198vw,23px)] grid gap-[5px]"
        aria-label="Session sections"
      >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full items-center bg-[#66363e] px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
          href="/host"
          aria-current="page">Overview</a
        >
        {#if hasActiveVote}
          <a
            class="flex h-[clamp(40px,3.646vw,70px)] w-full items-center px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
            href={liveViewHref}>Live View</a
          >
        {:else}
          <button
            class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-not-allowed items-center border-0 bg-transparent px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-grey-100"
            type="button"
            disabled
            title="Live View is available while a vote is running"
          >
            Live View
          </button>
        {/if}
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full items-center px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
          href="/host/configuration">Session Configuration</a
        >
        <button
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center border-0 bg-transparent px-[clamp(24px,2.344vw,45px)] text-left text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white"
          type="button">Comprehensive Session Results</button
        >
      </nav>

      <button
        class="absolute bottom-[35px] left-1/2 h-[clamp(34px,3.125vw,60px)] w-[min(calc(100%-42px),300px)] -translate-x-1/2 cursor-pointer rounded-[100px] border border-red-500 bg-transparent text-[clamp(14px,1.25vw,24px)] text-grey-200"
        type="button"
        onclick={() => (endSessionDialogOpen = true)}>End session</button
      >
    </aside>

    <section
      class="flex min-h-0 flex-col pt-[clamp(22px,2.188vw,42px)] pr-[clamp(32px,3.229vw,62px)] pb-3 pl-[clamp(24px,2.344vw,45px)] max-[1200px]:px-6"
      aria-label="Host overview"
    >
      <section
        class="flex h-[clamp(93px,9.01vw,173px)] shrink-0 justify-between rounded-[10px] border border-grey-200 bg-white px-[clamp(15px,1.51vw,29px)] py-[clamp(11px,1.146vw,22px)]"
        aria-label="Currently running voting instance"
      >
        <div>
          <div class="flex items-center gap-1.5">
            <Radio
              class="size-[clamp(18px,1.667vw,32px)]"
              strokeWidth={2.5}
              aria-hidden="true"
            />
            <h1 class="m-0 text-[clamp(14px,1.25vw,24px)] leading-8">
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
        <p class="mt-[5px] mb-0 text-[clamp(14px,1.25vw,24px)] leading-8">
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
          class="h-full min-h-0 overflow-hidden rounded-[10px] border border-grey-200 bg-white"
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
                class="m-0 text-[clamp(12px,1.042vw,20px)] leading-7 font-medium"
                id="participants-title"
              >
                Participants: {participants.length}
              </h2>
            </div>
            <div
              class="flex items-center gap-[7px] text-[clamp(12px,1.042vw,20px)] leading-7 font-medium"
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
                    "absolute top-0.5 left-[3px] size-[18px] rounded-full transition-transform duration-150",
                    compactMode
                      ? "translate-x-[18px] bg-white"
                      : "bg-slate-900",
                  ]}
                ></span>
              </button>
            </div>
          </header>

          {#if compactMode}
            <div
              class="grid h-[clamp(38px,3.646vw,70px)] grid-cols-[1.05fr_1.1fr_0.7fr_auto] items-center gap-2.5 overflow-hidden px-[13px]"
            >
              <button
                class="h-[clamp(24px,2.083vw,40px)] min-w-0 cursor-pointer truncate rounded-full border-0 bg-green-400 px-[clamp(10px,0.833vw,16px)] text-[clamp(9px,0.833vw,16px)] leading-6 text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-green-100 disabled:text-green-600 disabled:shadow-none"
                type="button"
                disabled={!hasBulkSelection}
                onclick={() => setSelectedProxyStatus("accepted")}
                >Accept all selected Proxy Votes</button
              >
              <button
                class="h-[clamp(24px,2.083vw,40px)] min-w-0 cursor-pointer truncate rounded-full border-0 bg-slate-500 px-[clamp(10px,0.833vw,16px)] text-[clamp(9px,0.833vw,16px)] leading-6 text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-slate-100 disabled:text-slate-500 disabled:shadow-none"
                type="button"
                disabled={!hasBulkSelection}
                onclick={() => setSelectedProxyStatus("declined")}
                >Revoke all selected Proxy Votes</button
              >
              <button
                class="h-[clamp(24px,2.083vw,40px)] min-w-0 cursor-pointer truncate rounded-full border-0 bg-slate-800 px-[clamp(10px,0.833vw,16px)] text-[clamp(9px,0.833vw,16px)] leading-6 text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-grey-200 disabled:text-grey-600 disabled:shadow-none"
                type="button"
                disabled={!hasBulkSelection}
                onclick={kickSelected}>Kick all selected</button
              >
              <label
                class="flex h-[clamp(24px,2.083vw,40px)] items-center justify-end gap-1.5 text-[clamp(11px,0.938vw,18px)] leading-none whitespace-nowrap"
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
                    class="flex min-w-0 flex-1 cursor-pointer flex-row items-start overflow-hidden border-0 bg-transparent p-0 text-left text-[clamp(9px,0.833vw,16px)] whitespace-nowrap"
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
                  class="grid min-h-[clamp(42px,4.167vw,80px)] grid-cols-[clamp(28px,2.604vw,50px)_minmax(0,1fr)_auto] items-center gap-[clamp(8px,0.833vw,16px)] bg-grey-50 px-[clamp(14px,2.604vw,50px)] py-[5px] even:bg-white"
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
                    class="flex min-w-0 cursor-pointer flex-col items-start overflow-hidden border-0 bg-transparent p-0 text-left"
                    type="button"
                    onclick={() => showProxyRequest(participant)}
                  >
                    <strong
                      class="text-[clamp(12px,0.938vw,18px)] leading-[26px]"
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
                  <div class="flex gap-[15px] max-[1200px]:gap-2">
                    {#if participant.proxyStatus === "pending"}
                      <button
                        class="h-[clamp(24px,2.083vw,40px)] w-[clamp(96px,9.375vw,180px)] cursor-pointer rounded-full border-0 bg-green-400 text-[clamp(10px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
                        type="button"
                        onclick={() => showProxyRequest(participant)}
                        >Accept Proxy</button
                      >
                    {:else if participant.proxyStatus === "accepted"}
                      <button
                        class="h-[clamp(24px,2.083vw,40px)] w-[clamp(96px,9.375vw,180px)] cursor-pointer rounded-full border-0 bg-slate-500 text-[clamp(10px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
                        type="button"
                        onclick={() =>
                          setParticipantStatus(participant.id, "declined")}
                        >Revoke Proxy</button
                      >
                    {/if}
                    <button
                      class="h-[clamp(24px,2.083vw,40px)] w-[clamp(54px,5.208vw,100px)] cursor-pointer rounded-full border-0 bg-slate-800 text-[clamp(10px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
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
          class="min-h-0 overflow-hidden rounded-[10px] border border-grey-500 bg-white"
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
                class="m-0 text-[clamp(12px,1.042vw,20px)] leading-7 font-medium"
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
              class="h-[clamp(28px,2.604vw,50px)] w-[min(100%,280px)] cursor-pointer rounded-[100px] border-0 bg-red-600 text-[clamp(11px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none"
              type="button"
              disabled={hasActiveVote}
              onclick={() => (dialog = "motion")}>+ Push a Motion</button
            >
            <button
              class="h-[clamp(28px,2.604vw,50px)] w-[min(100%,280px)] cursor-pointer rounded-[100px] border-0 bg-red-600 text-[clamp(11px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none"
              type="button"
              disabled={hasActiveVote}
              onclick={() => (dialog = "election")}>+ Push an Election</button
            >
            <div
              class="grid w-full justify-items-center gap-[clamp(10px,1.042vw,20px)]"
            >
              <button
                class="h-[clamp(28px,2.604vw,50px)] w-[min(100%,280px)] cursor-pointer rounded-[100px] border-0 bg-red-600 text-[clamp(11px,0.938vw,18px)] text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none"
                type="button"
                disabled={hasActiveVote}
                onclick={startQuickVote}>+ Quick Vote</button
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
    </section>
  </div>

  <AppFooter wide />
</main>

{#if dialog === "motion" || dialog === "election"}
  <HostConfigurationDialog
    kind={dialog}
    onclose={closeDialog}
    onsubmit={startConfiguredVote}
  />
{:else if dialog === "proxy"}
  <ProxyRequestDialog
    requester={selectedParticipant?.name ?? "Scottylabs1"}
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
    onconfirm={() => (window.location.href = "/home")}
  />
{/if}
