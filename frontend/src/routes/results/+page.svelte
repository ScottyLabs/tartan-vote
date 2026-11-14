<script lang="ts">
  import { page } from "$app/state";
  import { ApiRequestError } from "$lib/api/response";
  import { toErrorMessage } from "$lib/errors";
  import { apiUrl } from "$lib/api/base";
  import { checkMotion, type MotionResults } from "$lib/api/motion";
  import { getMotionResults } from "$lib/api/votes";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import ResultsDialog from "$lib/components/ResultsDialog.svelte";
  import VoterHeader from "$lib/components/VoterHeader.svelte";
  import {
    titleForMotion,
    motionKindFromMotionData,
    votingTypeFromMotionData,
  } from "$lib/domain/ballot";
  import {
    normalizeResultsPhase,
    normalizeResultsVariant,
    normalizeVoteType,
    requestForVariant,
    resultsViewModelFromApi,
    type ResultsViewModel,
  } from "$lib/domain/results";
  import { onMount } from "svelte";

  const RESULTS_POLL_MS = 2500;

  const variant = normalizeResultsVariant(page.url.searchParams.get("variant"));
  const hasExplicitVoteType = page.url.searchParams.has("type");
  const resultsRequest = hasExplicitVoteType
    ? {
        voteType: normalizeVoteType(page.url.searchParams.get("type")),
        phase: normalizeResultsPhase(page.url.searchParams.get("phase")),
        expanded: page.url.searchParams.get("expanded") === "true",
        compact: page.url.searchParams.get("compact") === "true",
      }
    : requestForVariant(variant);

  const sessionCode = $derived(
    page.url.searchParams.get("sessionCode")?.trim() || "Session",
  );

  let results = $state<ResultsViewModel | null>(null);
  let waitingForRelease = $state(false);
  let loadError = $state("");
  let motionEnded = $state(false);
  let backdropTitle = $state("Election");
  let backdropVotingType = $state("Standard");

  function updateBackdropFromMotion(motion: {
    name: string;
    data: Record<string, unknown>;
  }) {
    const kind = motionKindFromMotionData(motion.data);
    backdropTitle = titleForMotion(motion.name, kind);
    backdropVotingType = votingTypeFromMotionData(motion.data);
  }

  function applyTotals(totals: MotionResults, phase = resultsRequest.phase) {
    results = resultsViewModelFromApi(totals, {
      phase,
      voteType: resultsRequest.voteType,
      expanded: resultsRequest.expanded,
      compact: resultsRequest.compact,
    });
    waitingForRelease = false;
    loadError = "";
  }

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  onMount(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;

    async function pollResults() {
      try {
        const [totals, motionCheck] = await Promise.all([
          getMotionResults(),
          checkMotion().catch(() => null),
        ]);
        if (cancelled) return;
        if (motionCheck?.active_motion) {
          updateBackdropFromMotion(motionCheck.active_motion);
        }
        const ended = motionCheck ? motionCheck.active_motion === null : motionEnded;
        if (ended) motionEnded = true;
        applyTotals(totals, ended ? "final" : resultsRequest.phase);
      } catch (error) {
        if (cancelled) return;

        if (error instanceof ApiRequestError && error.status === 403) {
          waitingForRelease = true;
          loadError = "";
          try {
            const { active_motion } = await checkMotion();
            if (cancelled) return;
            if (!active_motion) {
              motionEnded = true;
              const totals = await getMotionResults();
              if (cancelled) return;
              applyTotals(totals, "final");
              return;
            }
            updateBackdropFromMotion(active_motion);
          } catch (checkError) {
            if (cancelled) return;
            loadError = toErrorMessage(
              checkError,
              "Waiting for results to be released.",
            );
          }
        } else {
          loadError = toErrorMessage(error, "Failed to load results.");
        }
      }

      if (!cancelled) {
        timer = setTimeout(() => {
          void pollResults();
        }, RESULTS_POLL_MS);
      }
    }

    void pollResults();
    return () => {
      cancelled = true;
      if (timer) clearTimeout(timer);
    };
  });
</script>

<svelte:head>
  <title>TartanVote | {results?.final ? "Final Results" : "Live Results"}</title>
  <meta
    name="description"
    content="View voting results for the active TartanVote election."
  />
</svelte:head>

<main
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-hidden pb-20 text-grey-900 [background:var(--gradient-screen-signin)] sm:m-0 sm:w-full"
>
  <VoterHeader {sessionCode} onSignOut={signOut} />

  <section
    class="pointer-events-none mx-auto mt-17 w-[calc(100%-42px)] max-w-88 select-none pb-12 sm:mt-[clamp(86px,7.55vw,145px)] sm:w-[min(calc(100vw-80px),800px)] sm:max-w-none sm:pb-[clamp(70px,5.2vw,100px)]"
    aria-hidden="true"
  >
    <header
      class="min-h-28 rounded-lg border border-grey-400 bg-white p-5 sm:min-h-[clamp(110px,6.25vw,120px)] sm:px-[clamp(32px,2.08vw,40px)] sm:py-[clamp(22px,1.56vw,30px)]"
    >
      <h1
        class="text-base leading-6 font-medium sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)]"
      >
        {backdropTitle}
      </h1>
      <p
        class="mx-0 mt-2 mb-0 text-xs leading-5 font-medium sm:mt-1 sm:text-[clamp(16px,0.94vw,18px)] sm:leading-[clamp(24px,1.35vw,26px)]"
      >
        Voting Type: {backdropVotingType}
      </p>
    </header>

    <div
      class="mt-4 rounded-lg border border-grey-400 bg-white px-5 py-6 sm:px-[clamp(32px,2.08vw,40px)] sm:py-[clamp(24px,1.56vw,30px)]"
    >
      <div class="flex flex-col gap-3">
        <div class="h-11 rounded-lg border border-red-400 bg-red-50"></div>
        <div class="h-11 rounded-lg border border-grey-300 bg-white"></div>
        <div class="h-11 rounded-lg border border-grey-300 bg-white"></div>
      </div>
      <div
        class="mx-auto mt-5 flex h-11 w-44 items-center justify-center rounded-full bg-red-200 text-sm font-bold text-white sm:mt-[clamp(18px,1.56vw,30px)] sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(280px,18.59vw,357px)] sm:text-[clamp(20px,1.25vw,24px)]"
      >
        Submitted
      </div>
    </div>
  </section>

  <AppFooter wide />

  <div
    class="fixed inset-0 z-20 grid place-items-center overflow-y-auto bg-grey-700/40 px-4 py-5 [&_.results-dialog]:relative [&_.results-dialog]:z-[1]"
  >
    {#if results}
      <ResultsDialog
        heading={results.heading}
        countLabel={results.countLabel}
        count={results.count}
        options={results.options}
        final={results.final}
        initiallyExpanded={results.initiallyExpanded}
      />
    {:else}
      <div
        class="relative z-[1] w-[min(100%,420px)] rounded-lg border border-grey-500 bg-white px-6 py-8 text-center"
        role="status"
        aria-live="polite"
      >
        {#if waitingForRelease}
          <h1 class="text-xl font-semibold text-grey-900">Results pending</h1>
          <p class="mt-3 mb-0 text-sm font-medium text-grey-700">
            Results are hidden until the host ends this motion. This page will
            update automatically.
          </p>
        {:else if loadError}
          <h1 class="text-xl font-semibold text-grey-900">Unable to load</h1>
          <p class="mt-3 mb-0 text-sm font-medium text-red-600">{loadError}</p>
        {:else}
          <h1 class="text-xl font-semibold text-grey-900">Loading results...</h1>
        {/if}
      </div>
    {/if}
  </div>
</main>
