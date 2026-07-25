<script lang="ts">
  import chevronUrl from '$lib/assets/results-chevron.svg?url';
  import { calculatePercentage, type ResultOption } from '$lib/domain/results';

  interface Props {
    heading: string;
    countLabel: string;
    count: string;
    options: ResultOption[];
    final?: boolean;
    initiallyExpanded?: boolean;
  }

  let {
    heading,
    countLabel,
    count,
    options,
    final = false,
    initiallyExpanded = false
  }: Props = $props();

  let expandedOptions = $state<Record<string, boolean>>({});
  const votesCast = $derived(options.reduce((total, option) => total + option.votes, 0));

  function isExpanded(optionId: string) {
    return expandedOptions[optionId] ?? initiallyExpanded;
  }

  function percentageFor(option: ResultOption) {
    return calculatePercentage(option.votes, votesCast);
  }

  function toggleOption(optionId: string) {
    expandedOptions[optionId] = !isExpanded(optionId);
  }
</script>

<section class:final class:expanded={initiallyExpanded} class="results-dialog" aria-labelledby="results-title">
  <header>
    <h1 id="results-title">{heading}</h1>
    <p>{countLabel}: {count}</p>
  </header>

  <div class="results-list">
    {#each options as option (option.id)}
      <article class="result">
        <div class="result-heading">
          <strong>{option.label}: {option.votes}</strong>
          <strong class="percentage">{percentageFor(option)}%</strong>
        </div>

        <div class="progress" aria-label={`${option.label}: ${percentageFor(option)}%`}>
          <span
            class={option.color}
            style={`width: ${percentageFor(option)}%`}
          ></span>
        </div>

        {#if option.voters}
          <div class:open={isExpanded(option.id)} class="voter-details">
            <div id={`voters-${option.id}`} class="voter-list">
              {#each (isExpanded(option.id) ? option.voters : option.voters.slice(0, 16)) as voter, voterIndex (`${voter}-${voterIndex}`)}
                <span>{voter}</span>
              {/each}
            </div>

            {#if option.voters.length > 16}
              <button
                type="button"
                aria-expanded={isExpanded(option.id)}
                aria-controls={`voters-${option.id}`}
                onclick={() => toggleOption(option.id)}
              >
                {isExpanded(option.id) ? 'View Less' : 'View All'}
                <img class:rotated={isExpanded(option.id)} src={chevronUrl} alt="" />
              </button>
            {/if}
          </div>
        {/if}
      </article>
    {/each}
  </div>
</section>

<style>
  .results-dialog {
    width: min(calc(100% - 32px), 1200px);
    max-height: calc(100svh - 40px);
    padding: clamp(32px, 5vw, 78px) clamp(20px, 4vw, 170px) 45px;
    border: 1px solid var(--color-grey-400);
    border-radius: 10px;
    overflow-y: auto;
    background: var(--color-white);
    color: var(--color-grey-900);
  }

  .results-dialog.final {
    border: 2px solid var(--color-red-400);
  }

  header {
    text-align: center;
  }

  h1,
  header p {
    margin: 0;
  }

  h1 {
    font-size: clamp(24px, 4.5vw, 45px);
    font-weight: 600;
    line-height: 1.24;
  }

  header p {
    margin-top: 10px;
    font-size: clamp(14px, 2vw, 20px);
    font-weight: 500;
    line-height: 1.4;
  }

  .results-list {
    width: min(100%, 859px);
    margin: clamp(30px, 4vw, 35px) auto 0;
    display: flex;
    flex-direction: column;
    gap: 30px;
  }

  .result-heading {
    min-width: 0;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 16px;
    font-size: clamp(14px, 1.65vw, 20px);
    line-height: 1.4;
  }

  .result-heading strong:first-child {
    min-width: 0;
    overflow-wrap: anywhere;
  }

  .percentage {
    flex: 0 0 auto;
    font-size: clamp(16px, 2vw, 24px);
    line-height: 1.33;
  }

  .progress {
    width: 100%;
    height: 8px;
    border-radius: 30px;
    overflow: hidden;
    background: var(--color-grey-200);
  }

  .progress span {
    height: 100%;
    border-radius: inherit;
    display: block;
  }

  .progress .green {
    background: var(--color-green-300);
  }

  .progress .yellow {
    background: var(--color-yellow-300);
  }

  .progress .blue {
    background: var(--color-blue-300);
  }

  .progress .red {
    background: var(--color-red-500);
  }

  .progress .purple {
    background: var(--color-purple-300);
  }

  .voter-details {
    margin-top: 5px;
    padding: 10px 0 0;
    background: var(--color-grey-50);
  }

  .voter-list {
    max-height: 52px;
    overflow: hidden;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(76px, max-content));
    gap: 5px 10px;
  }

  .voter-details.open .voter-list {
    max-height: none;
  }

  .voter-list span {
    min-height: 24px;
    padding: 2px 8px;
    border: 1px solid var(--color-grey-200);
    border-radius: 4px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    background: var(--color-white);
    font-size: 11px;
    font-weight: 600;
    line-height: 18px;
    white-space: nowrap;
  }

  .voter-details button {
    width: 100%;
    min-height: 34px;
    padding: 4px 0 0;
    border: 0;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    background: transparent;
    color: var(--color-slate-600);
    font: inherit;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
  }

  .voter-details button img {
    width: 16px;
    height: 20px;
    transition: transform 160ms ease;
  }

  .voter-details button img.rotated {
    transform: rotate(180deg);
  }

  .voter-details button:focus-visible {
    outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 72%);
    outline-offset: 2px;
  }

  @media (min-width: 640px) {
    .results-dialog {
      min-height: min(500px, calc(100svh - 40px));
    }

    .results-dialog.final {
      min-height: min(630px, calc(100svh - 40px));
    }

    .results-dialog.expanded {
      min-height: min(815px, calc(100svh - 40px));
      padding-top: 50px;
    }

    .voter-list {
      grid-template-columns: repeat(8, minmax(0, 1fr));
      gap: 5px 15px;
    }

    .voter-list span {
      font-size: 12px;
    }

    .voter-details button {
      font-size: 18px;
      line-height: 26px;
    }

    .voter-details button img {
      width: 20px;
      height: 26px;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .voter-details button img {
      transition: none;
    }
  }
</style>
