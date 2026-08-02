<script lang="ts">
  import { page } from '$app/state';
  import ResultsDialog from '$lib/components/ResultsDialog.svelte';
  import {
    createDemoResults,
    normalizeResultsPhase,
    normalizeResultsVariant,
    normalizeVoteType,
    requestForVariant
  } from '$lib/domain/results';

  const variant = normalizeResultsVariant(page.url.searchParams.get('variant'));
  const hasExplicitVoteType = page.url.searchParams.has('type');
  const resultsRequest = hasExplicitVoteType
    ? {
        voteType: normalizeVoteType(page.url.searchParams.get('type')),
        phase: normalizeResultsPhase(page.url.searchParams.get('phase')),
        expanded: page.url.searchParams.get('expanded') === 'true',
        compact: page.url.searchParams.get('compact') === 'true'
      }
    : requestForVariant(variant);
  const results = createDemoResults(resultsRequest);
</script>

<svelte:head>
  <title>TartanVote | {results.final ? 'Final Results' : 'Live Results'}</title>
  <meta name="description" content="View voting results for the active TartanVote election." />
</svelte:head>

<main
  class="relative grid min-h-svh w-full place-items-center overflow-hidden py-5
    [background:linear-gradient(rgb(102_102_102_/_0.4),rgb(102_102_102_/_0.4)),var(--gradient-screen-signin)]
    [&_.results-dialog]:relative [&_.results-dialog]:z-[1]"
>
  <div class="absolute inset-0 bg-grey-700/6" aria-hidden="true"></div>
  <ResultsDialog
    heading={results.heading}
    countLabel={results.countLabel}
    count={results.count}
    options={results.options}
    final={results.final}
    initiallyExpanded={results.initiallyExpanded}
  />
</main>
