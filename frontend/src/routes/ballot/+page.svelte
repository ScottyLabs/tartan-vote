<script lang="ts">
  import { page } from "$app/state";
  import { apiUrl } from "$lib/api/base";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import VoterHeader from "$lib/components/VoterHeader.svelte";
  import VotingInstanceFieldset from "$lib/components/VotingInstanceFieldset.svelte";
  import { ballotChoices, votingInstances } from "$lib/domain/ballot";
  import { loadOrganizationSettings } from "$lib/domain/organizationSettings";
  import { onMount } from "svelte";

  const sessionCode =
    page.url.searchParams.get("sessionCode")?.trim() || "happy-giraffe";
  const hasProxy = page.url.searchParams.get("proxy") !== "false";
  const confirmationKind =
    page.url.searchParams.get("confirmation") === "secret" ? "secret" : "live";
  const resultsVariant =
    page.url.searchParams.get("results") === "rollcall"
      ? "rollcall-live"
      : "live";
  const isQuickVote = page.url.searchParams.get("quickvote") === "true";
  const instances = hasProxy ? votingInstances : votingInstances.slice(0, 1);
  let choices = $state(ballotChoices);

  let selections = $state<Record<string, string>>({});
  let submitted = $state(false);
  let confirmationOpen = $state(false);

  const allSelected = $derived(
    instances.every((instance) => Boolean(selections[instance.id])),
  );
  const voteNoun = $derived(hasProxy ? "Votes" : "Vote");

  onMount(() => {
    if (!isQuickVote) return;
    choices = loadOrganizationSettings().quickVoteOptions.map(
      (label, index) => ({
        id: `quick-option-${index + 1}`,
        label,
      }),
    );
  });

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  function updateSelection(instanceId: string, choiceId: string) {
    if (submitted) return;
    selections[instanceId] = choiceId;
  }

  function submitBallot(event: SubmitEvent) {
    event.preventDefault();
    if (!allSelected || submitted) return;
    submitted = true;
    confirmationOpen = true;
  }

  function closeConfirmation() {
    confirmationOpen = false;
  }

  function handleConfirmationAction() {
    if (confirmationKind === "secret") {
      closeConfirmation();
      return;
    }

    window.location.href = `/results?variant=${resultsVariant}`;
  }
</script>

<svelte:head>
  <title>TartanVote | Ballot</title>
  <meta
    name="description"
    content="Submit a vote in the active TartanVote election."
  />
</svelte:head>

<main
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-x-hidden pb-[81px] text-grey-900 [background:var(--gradient-screen-signin)] sm:m-0 sm:w-full"
>
  <VoterHeader {sessionCode} onSignOut={signOut} />

  <section
    class={[
      "mx-auto mt-[68px] w-[calc(100%-42px)] max-w-[350px] pb-12 sm:w-[min(calc(100vw-80px),800px)] sm:max-w-none sm:pb-[clamp(70px,5.2vw,100px)]",
      isQuickVote ? "sm:mt-[30px]" : "sm:mt-[clamp(86px,7.55vw,145px)]",
    ]}
    aria-labelledby="ballot-title"
  >
    <header
      class={[
        "min-h-28 rounded-[10px] border border-grey-400 bg-white p-5 sm:min-h-[clamp(110px,6.25vw,120px)] sm:px-[clamp(32px,2.08vw,40px)] sm:py-[clamp(22px,1.56vw,30px)]",
        hasProxy && !isQuickVote && "sm:min-h-[clamp(160px,9.84vw,189px)]",
      ]}
    >
      <h1
        class="m-0 text-base leading-[22px] font-medium sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)]"
        id="ballot-title"
      >
        {isQuickVote ? "QuickVote" : "Election: Best Rust Stuco Instructor"}
      </h1>
      <p
        class="mx-0 mt-2 mb-0 text-[13px] leading-[18px] font-medium sm:mt-[5px] sm:text-[clamp(16px,0.94vw,18px)] sm:leading-[clamp(24px,1.35vw,26px)]"
      >
        Voting Type: Standard
      </p>
      {#if hasProxy && !isQuickVote}
        <p
          class="mx-0 mt-2 mb-0 text-xs leading-[17px] font-medium text-slate-600 sm:mt-3 sm:text-[clamp(16px,0.94vw,18px)] sm:leading-[clamp(24px,1.35vw,26px)] sm:text-slate-500"
        >
          Choose one response for each ballot assigned to you. Your proxy ballot
          is recorded separately from your own vote.
        </p>
      {/if}
    </header>

    <form class="mt-[15px]" onsubmit={submitBallot}>
      <div class="flex flex-col gap-[15px]">
        {#each instances as instance (instance.id)}
          <VotingInstanceFieldset
            {instance}
            {choices}
            selectedChoiceId={selections[instance.id]}
            disabled={submitted}
            compact={hasProxy}
            quickVote={isQuickVote}
            onChange={updateSelection}
          />
        {/each}
      </div>

      <button
        class={[
          "mx-auto mt-[18px] flex h-[42px] w-[174px] items-center justify-center rounded-full border-0 font-sans text-[15px] font-bold text-white focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_70%)] focus-visible:outline-offset-[3px] sm:mt-[clamp(18px,1.56vw,30px)] sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(280px,18.59vw,357px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-8",
          submitted
            ? "cursor-default bg-red-200 shadow-[0_2px_2px_rgb(0_0_0_/_0.18)]"
            : "cursor-pointer bg-red-600 shadow-[0_3px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:opacity-65",
        ]}
        type="submit"
        disabled={!allSelected || submitted}
      >
        {submitted ? "Submitted" : `Submit ${voteNoun}`}
      </button>
    </form>
  </section>

  <AppFooter wide />

  {#if confirmationOpen}
    <div
      class="fixed inset-0 z-20 grid place-items-center bg-grey-900/28 p-5"
      role="presentation"
    >
      <div
        class="flex min-h-[220px] w-[min(100%,350px)] flex-col items-center justify-between gap-[30px] rounded-[10px] border border-grey-600 bg-white px-5 pt-[34px] pb-7 text-center sm:min-h-[clamp(260px,15.63vw,300px)] sm:w-[min(calc(100vw-80px),1200px)] sm:gap-[clamp(42px,3.13vw,60px)] sm:px-5 sm:pt-[clamp(42px,2.6vw,50px)] sm:pb-[clamp(28px,1.56vw,30px)]"
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirmation-title"
      >
        <div>
          <h2
            class="m-0 text-[26px] leading-[34px] font-semibold text-black sm:text-[clamp(36px,2.34vw,45px)] sm:leading-[clamp(46px,2.92vw,56px)]"
            id="confirmation-title"
          >
            {voteNoun} Submitted
          </h2>
          {#if confirmationKind === "secret"}
            <p
              class="mx-0 mt-2 mb-0 text-sm leading-5 font-medium text-grey-900 sm:mt-2.5 sm:text-[clamp(18px,1.04vw,20px)] sm:leading-[clamp(26px,1.46vw,28px)]"
            >
              You have successfully voted. Please wait for your host to push out
              results.
            </p>
          {:else}
            <p
              class="mx-0 mt-2 mb-0 text-sm leading-5 font-medium text-grey-900 sm:mt-2.5 sm:text-[clamp(18px,1.04vw,20px)] sm:leading-[clamp(26px,1.46vw,28px)]"
            >
              You have successfully voted. Click below to view live results.
            </p>
          {/if}
        </div>

        <button
          class="h-12 w-[min(100%,240px)] cursor-pointer rounded-full border-0 bg-red-500 font-sans text-base font-bold text-white shadow-[0_3px_2px_rgb(0_0_0_/_0.25)] focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_70%)] focus-visible:outline-offset-[3px] sm:h-[clamp(54px,3.28vw,63px)] sm:w-[clamp(260px,16.41vw,315px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-8"
          type="button"
          onclick={handleConfirmationAction}
        >
          {confirmationKind === "secret" ? "OK" : "View Live Results"}
        </button>
      </div>
    </div>
  {/if}
</main>
