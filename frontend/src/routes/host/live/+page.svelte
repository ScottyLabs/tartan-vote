<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import Link2 from "@lucide/svelte/icons/link-2";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import ConfirmLiveResultsSharingDialog from "$lib/components/ConfirmLiveResultsSharingDialog.svelte";
  import EndSessionDialog from "$lib/components/EndSessionDialog.svelte";
  import EndVotingInstanceDialog from "$lib/components/EndVotingInstanceDialog.svelte";
  import HostFinalResultsDialog from "$lib/components/HostFinalResultsDialog.svelte";
  import { createHostLiveView } from "$lib/domain/hostLive";

  const liveView = createHostLiveView(page.url.searchParams);
  const overviewParams = new URLSearchParams({
    running: "true",
    type: liveView.type,
    title: liveView.title,
    labels: liveView.options.map((option) => option.label).join("|"),
    style: page.url.searchParams.get("style") ?? liveView.votingStyle,
    quorum: page.url.searchParams.get("quorum") ?? "",
    submitted: String(liveView.votesSubmitted),
    eligible: String(liveView.eligibleVotes),
  });
  let resultsShared = $state(page.url.searchParams.get("shared") === "true");
  let endDialogOpen = $state(page.url.searchParams.get("dialog") === "end");
  let finalDialogOpen = $state(page.url.searchParams.get("dialog") === "final");
  let sharingDialogOpen = $state(
    page.url.searchParams.get("dialog") === "share",
  );
  let instanceEnded = $state(false);
  let endSessionDialogOpen = $state(false);

  function confirmEnd() {
    endDialogOpen = false;
    finalDialogOpen = true;
  }

  function confirmFinalResult() {
    instanceEnded = true;
    finalDialogOpen = false;
    void goto("/host");
  }

  function confirmLiveResultsSharing() {
    resultsShared = !resultsShared;
    sharingDialogOpen = false;
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
          happy-giraffe
        </p>
        <button
          class="inline-flex h-6 max-w-full cursor-pointer items-center gap-2 border-0 bg-transparent p-0 font-sans text-[clamp(9px,0.833vw,16px)] text-red-200"
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
        class="mt-[clamp(12px,1.198vw,23px)] grid gap-[5px]"
        aria-label="Session sections"
      >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center border-0 bg-transparent px-[clamp(24px,2.344vw,45px)] font-sans text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
          href={`/host?${overviewParams.toString()}`}>Overview</a
        >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center border-0 bg-[#66363e] px-[clamp(24px,2.344vw,45px)] font-sans text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
          href="/host/live"
          aria-current="page">Live View</a
        >
        <a
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center border-0 bg-transparent px-[clamp(24px,2.344vw,45px)] font-sans text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white no-underline"
          href="/host/configuration">Session Configuration</a
        >
        <button
          class="flex h-[clamp(40px,3.646vw,70px)] w-full cursor-pointer items-center border-0 bg-transparent px-[clamp(24px,2.344vw,45px)] text-left font-sans text-[clamp(11px,0.938vw,18px)] leading-[26px] font-bold text-white"
          type="button">Comprehensive Session Results</button
        >
      </nav>

      <button
        class="absolute bottom-[35px] left-1/2 h-[clamp(34px,3.125vw,60px)] w-[min(calc(100%-42px),300px)] -translate-x-1/2 cursor-pointer rounded-[100px] border border-red-500 bg-transparent font-sans text-[clamp(14px,1.25vw,24px)] text-grey-200"
        type="button"
        onclick={() => (endSessionDialogOpen = true)}>End session</button
      >
    </aside>

    <section
      class="flex items-start px-[clamp(24px,3.385vw,65px)] py-[clamp(20px,1.979vw,38px)]"
      aria-labelledby="live-view-heading"
    >
      <article
        class="flex min-h-[clamp(376px,36.719vw,705px)] w-full flex-col rounded-[10px] border-2 border-grey-200 bg-white px-[clamp(34px,4.688vw,90px)] pt-[clamp(30px,3.125vw,60px)] pb-[clamp(28px,3.125vw,60px)]"
        class:opacity-78={instanceEnded}
      >
        <header class="flex items-start justify-between gap-[30px]">
          <h1
            class="m-0 max-w-[70%] text-[clamp(22px,1.875vw,36px)] leading-[1.3] font-semibold italic [overflow-wrap:anywhere]"
          >
            {liveView.title}
          </h1>
          <p
            class="m-0 flex-none text-[clamp(14px,1.042vw,20px)] leading-[1.4] font-semibold"
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
          {page.url.searchParams.get("style") ?? liveView.votingStyle}
        </p>

        <div class="mt-[clamp(12px,1.042vw,20px)] h-0.5 bg-grey-100"></div>

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
                  class="mt-[5px] h-2 w-full overflow-hidden rounded-[30px] bg-grey-200"
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
            class="h-[clamp(36px,2.604vw,50px)] w-[clamp(140px,10.417vw,200px)] cursor-pointer rounded-[100px] border border-red-500 bg-white font-sans text-[clamp(13px,0.938vw,18px)] font-medium text-red-500 disabled:cursor-not-allowed disabled:border-red-200 disabled:bg-red-200 disabled:text-white"
            type="button"
            disabled={instanceEnded}
            onclick={() => (endDialogOpen = true)}
          >
            {instanceEnded ? "Ended" : "End now"}
          </button>
          <button
            class="h-[clamp(36px,2.604vw,50px)] w-[clamp(180px,14.583vw,280px)] cursor-pointer rounded-[100px] border font-sans text-[clamp(13px,0.938vw,18px)] font-medium text-white disabled:cursor-not-allowed disabled:border-red-200 disabled:bg-red-200 disabled:text-white"
            class:border-red-600={!resultsShared}
            class:bg-red-600={!resultsShared}
            class:border-slate-700={resultsShared}
            class:bg-slate-700={resultsShared}
            type="button"
            disabled={instanceEnded}
            aria-pressed={resultsShared}
            onclick={() => (sharingDialogOpen = true)}
          >
            {resultsShared ? "Unshare live results" : "Share live results"}
          </button>
        </div>
      </article>
    </section>
  </div>

  <AppFooter wide flow />

{#if endDialogOpen}
  <EndVotingInstanceDialog
    onclose={() => (endDialogOpen = false)}
    onconfirm={confirmEnd}
  />
{/if}

{#if finalDialogOpen}
  <HostFinalResultsDialog
    options={liveView.options}
    totalVotes={liveView.votesSubmitted}
    shared={resultsShared}
    onconfirm={confirmFinalResult}
    ontoggleshare={() => (resultsShared = !resultsShared)}
  />
{/if}

{#if sharingDialogOpen}
  <ConfirmLiveResultsSharingDialog
    currentlyShared={resultsShared}
    onclose={() => (sharingDialogOpen = false)}
    onconfirm={confirmLiveResultsSharing}
  />
{/if}

{#if endSessionDialogOpen}
  <EndSessionDialog
    onclose={() => (endSessionDialogOpen = false)}
    onconfirm={() => (window.location.href = "/home")}
  />
{/if}
