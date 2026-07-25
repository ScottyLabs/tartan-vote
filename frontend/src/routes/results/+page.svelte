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

<main class="results-page">
  <div class="page-underlay" aria-hidden="true"></div>
  <ResultsDialog
    heading={results.heading}
    countLabel={results.countLabel}
    count={results.count}
    options={results.options}
    final={results.final}
    initiallyExpanded={results.initiallyExpanded}
  />
</main>

<style>
  .results-page {
    position: relative;
    width: 100%;
    min-height: 100svh;
    padding: 20px 0;
    display: grid;
    place-items: center;
    overflow: hidden;
    background:
      linear-gradient(rgb(102 102 102 / 0.4), rgb(102 102 102 / 0.4)),
      var(--gradient-screen-signin);
  }

  .page-underlay {
    position: absolute;
    inset: 0;
    background: rgb(102 102 102 / 0.06);
  }

  :global(.results-dialog) {
    position: relative;
    z-index: 1;
  }
</style>
