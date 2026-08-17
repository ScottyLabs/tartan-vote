<script lang="ts">
  import { page } from "$app/state";
  import { checkMotion, type MotionResults } from "$lib/api/motion";
  import { getMotionResults, isMotionRequestError } from "$lib/api/votes";
  import ResultsDialog from "$lib/components/ResultsDialog.svelte";
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

  let results = $state<ResultsViewModel | null>(null);
  let waitingForRelease = $state(false);
  let loadError = $state("");
  let motionEnded = $state(false);

  function toErrorMessage(error: unknown, fallback: string): string {
    if (error instanceof Error && error.message.trim() !== "") {
      return error.message;
    }
    return fallback;
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
        const ended = motionCheck ? motionCheck.active_motion === null : motionEnded;
        if (ended) motionEnded = true;
        applyTotals(totals, ended ? "final" : resultsRequest.phase);
      } catch (error) {
        if (cancelled) return;

        if (isMotionRequestError(error) && error.status === 403) {
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
  class="relative grid min-h-svh w-full place-items-center overflow-hidden py-5
    [background:linear-gradient(rgb(102_102_102_/_0.4),rgb(102_102_102_/_0.4)),var(--gradient-screen-signin)]
    [&_.results-dialog]:relative [&_.results-dialog]:z-[1]"
>
  <div class="absolute inset-0 bg-grey-700/6" aria-hidden="true"></div>
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
      class="relative z-[1] w-[min(100%,420px)] rounded-[10px] border border-grey-500 bg-white px-6 py-8 text-center"
      role="status"
      aria-live="polite"
    >
      {#if waitingForRelease}
        <h1 class="m-0 text-xl font-semibold text-grey-900">Results pending</h1>
        <p class="mt-3 mb-0 text-sm font-medium text-grey-700">
          Results are hidden until the host ends this motion. This page will
          update automatically.
        </p>
      {:else if loadError}
        <h1 class="m-0 text-xl font-semibold text-grey-900">Unable to load</h1>
        <p class="mt-3 mb-0 text-sm font-medium text-red-600">{loadError}</p>
      {:else}
        <h1 class="m-0 text-xl font-semibold text-grey-900">Loading results...</h1>
      {/if}
    </div>
  {/if}
</main>
