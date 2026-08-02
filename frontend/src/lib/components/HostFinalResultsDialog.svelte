<script lang="ts">
  import type { HostLiveOption } from "$lib/domain/hostLive";

  interface Props {
    options: HostLiveOption[];
    totalVotes: number;
    shared: boolean;
    onconfirm: () => void;
    ontoggleshare: () => void;
  }

  let { options, totalVotes, shared, onconfirm, ontoggleshare }: Props =
    $props();

  const winningOption = $derived(
    options.reduce(
      (winner, option) => (option.votes > winner.votes ? option : winner),
      options[0],
    ),
  );
</script>

<div
  class="fixed inset-0 z-20 grid place-items-center bg-grey-700/40 p-5"
  role="presentation"
>
  <div
    class="flex min-h-[630px] w-[min(1200px,calc(100vw-40px))] flex-col rounded-[10px] border border-grey-500 bg-white px-5 pt-[clamp(48px,4.063vw,78px)] pb-[45px] text-grey-900 [@media(max-height:760px)]:max-h-[calc(100svh-40px)] [@media(max-height:760px)]:min-h-0 [@media(max-height:760px)]:overflow-y-auto"
    role="dialog"
    aria-modal="true"
    aria-labelledby="final-result-title"
  >
    <header class="text-center">
      <h2
        class="m-0 text-[clamp(30px,2.344vw,45px)] leading-[1.25] font-semibold"
        id="final-result-title"
      >
        Final Result: {winningOption.label}
      </h2>
      <p
        class="mx-0 mt-2.5 mb-0 text-[clamp(16px,1.042vw,20px)] leading-[1.4] font-medium"
      >
        Total Votes: {totalVotes}
      </p>
    </header>

    <div class="mx-auto mt-5 flex w-[min(100%,859px)] flex-col gap-[30px]">
      {#each options as option (option.id)}
        <article>
          <div
            class="flex items-start justify-between gap-5 text-[clamp(16px,1.042vw,20px)] leading-[1.4]"
          >
            <strong class="min-w-0 [overflow-wrap:anywhere]"
              >{option.label}({option.votes})</strong
            >
            <strong
              class="flex-none text-[clamp(19px,1.25vw,24px)] leading-[1.34]"
              >{option.percentage}%</strong
            >
          </div>
          <div
            class="mt-1 h-2 w-full overflow-hidden rounded-[30px] bg-grey-200"
            role="progressbar"
            aria-label={`${option.label}: ${option.percentage}%`}
            aria-valuenow={option.percentage}
            aria-valuemin="0"
            aria-valuemax="100"
          >
            <span
              class="block h-full rounded-[inherit]"
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

    <div class="mt-auto flex items-center justify-center gap-[22px]">
      <button
        class="h-[63px] w-[315px] cursor-pointer rounded-[100px] border-0 bg-red-500 font-sans text-2xl font-bold text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onconfirm}
      >
        Confirm
      </button>
      <button
        class="h-[63px] w-[315px] cursor-pointer rounded-[100px] border-0 font-sans text-2xl font-bold text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        class:bg-red-500={!shared}
        class:bg-slate-700={shared}
        type="button"
        aria-pressed={shared}
        onclick={ontoggleshare}
      >
        {shared ? "Unshare Final Results" : "Share Final Results"}
      </button>
    </div>
  </div>
</div>
