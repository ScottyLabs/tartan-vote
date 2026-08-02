<script lang="ts">
  import chevronUrl from "$lib/assets/results-chevron.svg?url";
  import { calculatePercentage, type ResultOption } from "$lib/domain/results";

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
    initiallyExpanded = false,
  }: Props = $props();

  let expandedOptions = $state<Record<string, boolean>>({});
  const votesCast = $derived(
    options.reduce((total, option) => total + option.votes, 0),
  );

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

<section
  class={[
    "results-dialog max-h-[calc(100svh-40px)] w-[min(calc(100%-32px),1200px)] overflow-y-auto rounded-[10px] bg-white px-[clamp(20px,4vw,170px)] pt-[clamp(32px,5vw,78px)] pb-[45px] text-grey-900",
    final ? "border-2 border-red-400" : "border border-grey-400",
    initiallyExpanded
      ? "sm:min-h-[min(815px,calc(100svh-40px))] sm:pt-[50px]"
      : final
        ? "sm:min-h-[min(630px,calc(100svh-40px))]"
        : "sm:min-h-[min(500px,calc(100svh-40px))]",
  ]}
  aria-labelledby="results-title"
>
  <header class="text-center">
    <h1
      class="m-0 text-[clamp(24px,4.5vw,45px)] leading-[1.24] font-semibold"
      id="results-title"
    >
      {heading}
    </h1>
    <p
      class="mx-0 mt-2.5 mb-0 text-[clamp(14px,2vw,20px)] leading-[1.4] font-medium"
    >
      {countLabel}: {count}
    </p>
  </header>

  <div
    class="mx-auto mt-[clamp(30px,4vw,35px)] flex w-[min(100%,859px)] flex-col gap-[30px]"
  >
    {#each options as option (option.id)}
      <article>
        <div
          class="flex min-w-0 items-start justify-between gap-4 text-[clamp(14px,1.65vw,20px)] leading-[1.4]"
        >
          <strong class="min-w-0 [overflow-wrap:anywhere]"
            >{option.label}: {option.votes}</strong
          >
          <strong class="flex-none text-[clamp(16px,2vw,24px)] leading-[1.33]">
            {percentageFor(option)}%
          </strong>
        </div>

        <div
          class="h-2 w-full overflow-hidden rounded-[30px] bg-grey-200"
          aria-label={`${option.label}: ${percentageFor(option)}%`}
        >
          <span
            class="block h-full rounded-[inherit]"
            class:bg-green-300={option.color === "green"}
            class:bg-yellow-300={option.color === "yellow"}
            class:bg-blue-300={option.color === "blue"}
            class:bg-red-500={option.color === "red"}
            class:bg-purple-300={option.color === "purple"}
            style={`width: ${percentageFor(option)}%`}
          ></span>
        </div>

        {#if option.voters}
          <div class="mt-[5px] bg-grey-50 px-0 pt-2.5 pb-0">
            <div
              class={[
                "grid grid-cols-[repeat(auto-fit,minmax(76px,max-content))] gap-x-2.5 gap-y-[5px] overflow-hidden sm:grid-cols-8 sm:gap-x-[15px]",
                isExpanded(option.id) ? "max-h-none" : "max-h-[52px]",
              ]}
              id={`voters-${option.id}`}
            >
              {#each isExpanded(option.id) ? option.voters : option.voters.slice(0, 16) as voter, voterIndex (`${voter}-${voterIndex}`)}
                <span
                  class="inline-flex min-h-6 items-center justify-center whitespace-nowrap rounded border border-grey-200 bg-white px-2 py-0.5 text-[11px] leading-[18px] font-semibold sm:text-xs"
                  >{voter}</span
                >
              {/each}
            </div>

            {#if option.voters.length > 16}
              <button
                class="flex min-h-[34px] w-full cursor-pointer items-center justify-end gap-2 border-0 bg-transparent pt-1 pr-0 pb-0 pl-0 font-sans text-sm font-medium text-slate-600 focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_72%)] focus-visible:outline-offset-2 sm:text-lg sm:leading-[26px]"
                type="button"
                aria-expanded={isExpanded(option.id)}
                aria-controls={`voters-${option.id}`}
                onclick={() => toggleOption(option.id)}
              >
                {isExpanded(option.id) ? "View Less" : "View All"}
                <img
                  class="h-5 w-4 transition-transform duration-160 ease-[ease] motion-reduce:transition-none sm:h-[26px] sm:w-5"
                  class:rotate-180={isExpanded(option.id)}
                  src={chevronUrl}
                  alt=""
                />
              </button>
            {/if}
          </div>
        {/if}
      </article>
    {/each}
  </div>
</section>
