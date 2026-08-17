<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { toErrorMessage } from "$lib/errors";
  import Link2 from "@lucide/svelte/icons/link-2";
  import { fetchAttendance } from "$lib/api/attendance";
  import { checkMotion, endMotion, exportMotionResults } from "$lib/api/motion";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import EndSessionDialog from "$lib/components/EndSessionDialog.svelte";
  import EndVotingInstanceDialog from "$lib/components/EndVotingInstanceDialog.svelte";
  import HostFinalResultsDialog from "$lib/components/HostFinalResultsDialog.svelte";
  import {
    parseHostVoteKind,
    voteOptionsFromMotionData,
  } from "$lib/domain/host";
  import {
    createHostLiveView,
    hostLiveViewFromApi,
    updateHostLiveViewFromResults,
    type HostLiveViewModel,
  } from "$lib/domain/hostLive";
  import {
    displayHostSessionCode,
    endHostedSessionWithRedirect,
    ensureHostedSessionCode,
  } from "$lib/domain/hostSession";
  import {
    participantVisibilityFromMotionData,
    votingTypeFromMotionData,
  } from "$lib/domain/ballot";
  import { onMount } from "svelte";

  const RESULTS_POLL_MS = 2500;

  const initialLiveView = createHostLiveView(page.url.searchParams);
  let liveView = $state<HostLiveViewModel>(initialLiveView);
  let votingStyleLabel = $state(
    page.url.searchParams.get("style") ?? initialLiveView.votingStyle,
  );
  /** Reflects create-time participant visibility; not toggleable mid-motion. */
  let liveResultsVisibleToParticipants = $state(
    page.url.searchParams.get("shared") === "true",
  );
  let endDialogOpen = $state(page.url.searchParams.get("dialog") === "end");
  let finalDialogOpen = $state(page.url.searchParams.get("dialog") === "final");
  let instanceEnded = $state(false);
  let endSessionDialogOpen = $state(false);
  let sessionCode = $state(
    page.url.searchParams.get("sessionCode")?.trim() || "",
  );
  let endingSession = $state(false);
  let endingMotion = $state(false);
  let sessionError = $state("");
  let liveError = $state("");
  const displayedSessionCode = $derived(displayHostSessionCode(sessionCode));
  const configurationHref = $derived.by(() => {
    const params = new URLSearchParams();
    if (sessionCode) params.set("sessionCode", sessionCode);
    return params.size > 0
      ? `/host/configuration?${params.toString()}`
      : "/host/configuration";
  });
  const overviewHref = $derived.by(() => {
    const params = new URLSearchParams({
      running: instanceEnded ? "false" : "true",
      type: liveView.type,
      title: liveView.title,
      labels: liveView.options.map((option) => option.label).join("|"),
      style: votingStyleLabel,
      quorum: page.url.searchParams.get("quorum") ?? "",
      submitted: String(liveView.votesSubmitted),
      eligible: String(liveView.eligibleVotes),
    });
    if (sessionCode) params.set("sessionCode", sessionCode);
    return `/host?${params.toString()}`;
  });
  const liveResultsStatusLabel = $derived(
    liveResultsVisibleToParticipants
      ? "Live results shared"
      : "Share live results",
  );

  async function loadHostTotals() {
    const exported = await exportMotionResults();
    return exported.totals;
  }

  async function confirmEnd() {
    if (endingMotion) return;
    endingMotion = true;
    liveError = "";
    try {
      await endMotion();
      instanceEnded = true;
      endDialogOpen = false;
      finalDialogOpen = true;
      try {
        const totals = await loadHostTotals();
        liveView = updateHostLiveViewFromResults(liveView, totals);
      } catch {
        // Keep last polled tallies if final fetch fails.
      }
    } catch (error) {
      liveError = toErrorMessage(error, "Failed to end motion.");
      endDialogOpen = false;
    } finally {
      endingMotion = false;
    }
  }

  function confirmFinalResult() {
    instanceEnded = true;
    finalDialogOpen = false;
    void goto("/host");
  }

  onMount(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;

    async function refreshLive() {
      if (instanceEnded || cancelled) return;
      try {
        const { active_motion } = await checkMotion();
        if (cancelled || instanceEnded) return;

        if (!active_motion) {
          liveError = liveError || "No active motion.";
          return;
        }

        const kind = parseHostVoteKind(active_motion.data.kind);
        const optionLabels = voteOptionsFromMotionData(active_motion.data);
        votingStyleLabel = votingTypeFromMotionData(active_motion.data);
        liveResultsVisibleToParticipants =
          participantVisibilityFromMotionData(active_motion.data) === "live";

        let eligible = liveView.eligibleVotes;
        if (sessionCode) {
          try {
            const attendance = await fetchAttendance(sessionCode);
            if (!cancelled) {
              eligible = Math.max(
                attendance.headcount,
                attendance.attendees.reduce(
                  (total, attendee) => total + 1 + attendee.proxy_for.length,
                  0,
                ),
              );
            }
          } catch {
            // Keep prior eligible estimate when attendance is unavailable.
          }
        }

        const totals = await loadHostTotals();
        if (cancelled) return;
        liveView = hostLiveViewFromApi({
          title: active_motion.name,
          type: kind,
          votingStyle:
            votingStyleLabel === "Secret Ballot"
              ? "Secret Ballot"
              : "Standard",
          results: totals,
          eligibleVotes: Math.max(eligible, totals.total),
          optionLabels,
        });
        liveError = "";
      } catch (error) {
        if (cancelled) return;
        liveError = toErrorMessage(error, "Failed to load live view.");
      }
    }

    async function pollLoop() {
      if (instanceEnded) return;
      await refreshLive();
      if (!cancelled && !instanceEnded) {
        timer = setTimeout(() => {
          void pollLoop();
        }, RESULTS_POLL_MS);
      }
    }

    void ensureHostedSessionCode(sessionCode).then((resolvedSessionCode) => {
      if (cancelled) return;
      sessionCode = resolvedSessionCode;
      void pollLoop();
    });

    return () => {
      cancelled = true;
      if (timer) clearTimeout(timer);
    };
  });

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
  <title>TartanVote | Host Live View</title>
  <meta
    name="description"
    content="Monitor the active TartanVote voting instance."
  />
</svelte:head>

  <div
    class="grid flex-[1_0_calc(100svh-clamp(60px,5.833vw,112px)-81px)] grid-cols-[clamp(203px,19.792vw,380px)_minmax(0,1fr)]"
  >
    <aside
      class="relative min-h-[calc(100svh-clamp(60px,5.833vw,112px)-81px)] bg-grey-900 text-white"
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
        >
          <span class="min-w-0 overflow-hidden text-ellipsis whitespace-nowrap"
            >Copy invite link</span
          >
          <span
            class="grid size-[clamp(10px,1.042vw,20px)] flex-none place-items-center"
            aria-hidden="true"
            ><Link2
              class="block size-full text-red-200"
              aria-hidden="true"
            /></span
          >
        </button>
      </section>

      <nav
        class="mt-[clamp(12px,1.198vw,23px)] grid gap-1"
        aria-label="Session sections"
      >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center bg-transparent px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-white"
          href={overviewHref}>Overview</a
        >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center bg-[#66363e] px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-white"
          href={sessionCode
            ? `/host/live?sessionCode=${encodeURIComponent(sessionCode)}`
            : "/host/live"}
          aria-current="page">Live View</a
        >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center bg-transparent px-[clamp(24px,2.344vw,45px)] text-[clamp(11px,0.938vw,18px)] leading-7 font-bold text-white"
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
      class="flex items-start px-[clamp(24px,3.385vw,65px)] py-[clamp(20px,1.979vw,38px)]"
      aria-labelledby="live-view-heading"
    >
      <article
        class="flex min-h-[clamp(376px,36.719vw,705px)] w-full flex-col rounded-lg border-2 border-grey-200 bg-white px-[clamp(34px,4.688vw,90px)] pt-[clamp(30px,3.125vw,60px)] pb-[clamp(28px,3.125vw,60px)]"
        class:opacity-78={instanceEnded}
      >
        <header class="flex items-start justify-between gap-8">
          <h1
            class="max-w-[70%] text-[clamp(22px,1.875vw,36px)] leading-[1.3] font-semibold italic [overflow-wrap:anywhere]"
          >
            {liveView.title}
          </h1>
          <p
            class="flex-none text-[clamp(14px,1.042vw,20px)] leading-[1.4] font-semibold"
          >
            Votes Submitted:
            <strong class="text-red-400">
              {instanceEnded
                ? "Ended"
                : `${liveView.votesSubmitted}/${liveView.eligibleVotes}`}
            </strong>
          </p>
        </header>

        <p
          class="mx-0 mt-[clamp(16px,1.823vw,35px)] mb-0 text-[clamp(13px,0.938vw,18px)] leading-[1.45] font-medium"
        >
          Type: {liveView.type === "election"
            ? "Election"
            : liveView.type === "quick-vote"
              ? "Quick Vote"
              : "Motion"} // Voting Style:
          {votingStyleLabel}
        </p>

        <div class="mt-[clamp(12px,1.042vw,20px)] h-1 bg-grey-100"></div>

        <section
          class="mx-auto mt-[clamp(16px,1.302vw,25px)] w-[min(100%,859px)]"
          aria-labelledby="live-view-heading"
        >
          <h2
            class="mt-0 mb-[clamp(18px,1.823vw,35px)] text-[clamp(20px,1.563vw,30px)] leading-[1.27] font-bold text-slate-700"
            id="live-view-heading"
          >
            Live View
          </h2>
          <div class="flex flex-col gap-[clamp(18px,1.563vw,30px)]">
            {#each liveView.options as option (option.id)}
              <article>
                <div
                  class="flex items-start justify-between gap-6 text-[clamp(13px,0.938vw,18px)] leading-[1.45]"
                >
                  <strong class="min-w-0 [overflow-wrap:anywhere]">
                    {option.label}: ({option.votes}
                    {option.votes === 1 ? "Vote" : "Votes"})
                  </strong>
                  <strong
                    class="flex-none text-[clamp(16px,1.25vw,24px)] leading-[1.34]"
                  >
                    {option.percentage}%
                  </strong>
                </div>
                <div
                  class="mt-1 h-2 w-full overflow-hidden rounded-4xl bg-grey-200"
                  role="progressbar"
                  aria-label={`${option.label}: ${option.percentage}%`}
                  aria-valuenow={option.percentage}
                  aria-valuemin="0"
                  aria-valuemax="100"
                >
                  <span
                    class="block h-full rounded-[inherit] transition-[width] duration-180 ease-[ease]"
                    class:bg-green-300={option.color === "green"}
                    class:bg-yellow-300={option.color === "yellow"}
                    class:bg-blue-300={option.color === "blue"}
                    class:bg-red-500={option.color === "red"}
                    class:bg-purple-300={option.color === "purple"}
                    style={`width: ${option.percentage}%`}
                  ></span>
                </div>
              </article>
            {/each}
          </div>
        </section>

        <div
          class="mt-auto flex justify-end gap-[clamp(14px,1.042vw,20px)] pt-[clamp(28px,3.125vw,60px)]"
        >
          <button
            class="h-[clamp(36px,2.604vw,50px)] w-[clamp(140px,10.417vw,200px)] cursor-pointer rounded-full border border-red-500 bg-white text-[clamp(13px,0.938vw,18px)] font-medium text-red-500 disabled:cursor-not-allowed disabled:border-red-200 disabled:bg-red-200 disabled:text-white"
            type="button"
            disabled={instanceEnded || endingMotion}
            onclick={() => (endDialogOpen = true)}
          >
            {instanceEnded ? "Ended" : endingMotion ? "Ending..." : "End now"}
          </button>
          <button
            class="h-[clamp(36px,2.604vw,50px)] w-[clamp(180px,14.583vw,280px)] cursor-not-allowed rounded-full border text-[clamp(13px,0.938vw,18px)] font-medium text-white"
            class:border-slate-700={liveResultsVisibleToParticipants}
            class:bg-slate-700={liveResultsVisibleToParticipants}
            class:border-red-600={!liveResultsVisibleToParticipants}
            class:bg-red-600={!liveResultsVisibleToParticipants}
            type="button"
            disabled
            aria-pressed={liveResultsVisibleToParticipants}
            title="Participant visibility is set when the motion is created and cannot be changed while it is active."
          >
            {liveResultsStatusLabel}
          </button>
        </div>
      </article>
      {#if liveError}
        <p class="mt-3 text-sm font-semibold text-red-700" role="alert">
          {liveError}
        </p>
      {/if}
      {#if sessionError}
        <p class="mt-3 text-sm font-semibold text-red-700" role="alert">
          {sessionError}
        </p>
      {/if}
    </section>
  </div>

  <AppFooter wide flow />

{#if endDialogOpen}
  <EndVotingInstanceDialog
    onclose={() => (endDialogOpen = false)}
    onconfirm={() => void confirmEnd()}
  />
{/if}

{#if finalDialogOpen}
  <HostFinalResultsDialog
    options={liveView.options}
    totalVotes={liveView.votesSubmitted}
    shared={true}
    shareDisabled={true}
    onconfirm={confirmFinalResult}
    ontoggleshare={() => {}}
  />
{/if}

{#if endSessionDialogOpen}
  <EndSessionDialog
    onclose={() => (endSessionDialogOpen = false)}
    onconfirm={() => void confirmEndSession()}
  />
{/if}
