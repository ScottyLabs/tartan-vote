<script lang="ts">
  import { page } from "$app/state";
  import { ApiRequestError } from "$lib/api/response";
  import { toErrorMessage } from "$lib/errors";
  import { apiUrl } from "$lib/api/base";
  import { checkMotion } from "$lib/api/motion";
  import { castVote, getVoteInstances } from "$lib/api/votes";
  import { fetchSessionStatus } from "$lib/api/session";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import VoterHeader from "$lib/components/VoterHeader.svelte";
  import VotingInstanceFieldset from "$lib/components/VotingInstanceFieldset.svelte";
  import {
    ballotChoicesFromMotionData,
    titleForMotion,
    motionKindFromMotionData,
    participantVisibilityFromMotionData,
    votingInstancesFromApi,
    votingTypeFromMotionData,
    type BallotChoice,
    type VotingInstance,
  } from "$lib/domain/ballot";
  import { onMount } from "svelte";

  let sessionCode = $state(
    page.url.searchParams.get("sessionCode")?.trim() || "",
  );
  let motionTitle = $state("Loading motion...");
  let votingType = $state("Standard");
  let motionKind = $state("motion");
  let confirmationKind = $state<"secret" | "live">("live");
  let choices = $state<BallotChoice[]>([]);
  let instances = $state<VotingInstance[]>([]);
  let selections = $state<Record<string, string>>({});
  let submitted = $state(false);
  let confirmationOpen = $state(false);
  let confirmationVoteNoun = $state("Vote");
  let loading = $state(true);
  let loadError = $state("");
  let submitError = $state("");
  let submitting = $state(false);

  const pendingInstances = $derived(
    instances.filter((instance) => !instance.hasVoted),
  );
  const hasProxy = $derived(instances.some((instance) => instance.proxy));
  const isQuickVote = $derived(motionKind === "quick-vote");
  const allSelected = $derived(
    pendingInstances.length > 0 &&
      pendingInstances.every((instance) => Boolean(selections[instance.id])),
  );
  const allAlreadyVoted = $derived(
    !loading && instances.length > 0 && pendingInstances.length === 0,
  );

  function markInstancesVoted(instanceIds: Iterable<string>) {
    const voted = new Set(instanceIds);
    if (voted.size === 0) return;
    instances = instances.map((instance) =>
      voted.has(instance.id) ? { ...instance, hasVoted: true } : instance,
    );
  }

  async function resolveSessionCode(current: string): Promise<string> {
    if (current) return current;
    try {
      const status = await fetchSessionStatus();
      return status.participating?.session_code ?? status.hosting?.session_code ?? "";
    } catch {
      return "";
    }
  }

  onMount(() => {
    let cancelled = false;

    async function loadBallot() {
      loading = true;
      loadError = "";
      try {
        sessionCode = await resolveSessionCode(sessionCode);
        const [{ active_motion }, voteInstances] = await Promise.all([
          checkMotion(),
          getVoteInstances(),
        ]);
        if (cancelled) return;

        if (!active_motion) {
          loadError = "No active motion. Return to the waiting room.";
          instances = [];
          choices = [];
          return;
        }

        const data = active_motion.data;
        motionKind = motionKindFromMotionData(data);
        votingType = votingTypeFromMotionData(data);
        motionTitle = titleForMotion(active_motion.name, motionKind);
        const visibility = participantVisibilityFromMotionData(data);
        confirmationKind =
          visibility === "hidden_until_release" ? "secret" : "live";
        choices = ballotChoicesFromMotionData(data);
        instances = votingInstancesFromApi(voteInstances);

        if (choices.length === 0) {
          loadError = "This motion has no vote options.";
        } else if (instances.length === 0) {
          loadError = "No voting instances are available for you.";
        }
      } catch (error) {
        if (cancelled) return;
        loadError = toErrorMessage(error, "Failed to load the ballot.");
      } finally {
        if (!cancelled) loading = false;
      }
    }

    void loadBallot();
    return () => {
      cancelled = true;
    };
  });

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  function updateSelection(instanceId: string, choiceId: string) {
    if (submitted || submitting) return;
    const instance = instances.find((entry) => entry.id === instanceId);
    if (instance?.hasVoted) return;
    selections[instanceId] = choiceId;
  }

  async function submitBallot(event: SubmitEvent) {
    event.preventDefault();
    if (!allSelected || submitted || submitting) return;
    submitting = true;
    submitError = "";

    const toCast = pendingInstances.slice();
    confirmationVoteNoun =
      toCast.length > 1 || toCast.some((instance) => instance.proxy)
        ? "Votes"
        : "Vote";
    const succeededIds: string[] = [];

    try {
      for (const instance of toCast) {
        const choiceLabel = selections[instance.id];
        if (!choiceLabel) continue;
        try {
          await castVote({
            vote_response: [choiceLabel],
            voter_instance_id: Number(instance.id),
          });
          succeededIds.push(instance.id);
        } catch (error) {
          if (error instanceof ApiRequestError && error.status === 409) {
            succeededIds.push(instance.id);
            continue;
          }
          throw error;
        }
      }
      markInstancesVoted(succeededIds);
      submitted = true;
      confirmationOpen = true;
    } catch (error) {
      markInstancesVoted(succeededIds);
      submitError = toErrorMessage(error, "Failed to submit vote.");
    } finally {
      submitting = false;
    }
  }

  function resultsHref(): string {
    const params = new URLSearchParams({ phase: "live" });
    if (sessionCode) params.set("sessionCode", sessionCode);

    const normalizedVotingType = votingType.trim().toLowerCase();
    if (normalizedVotingType.includes("secret")) {
      params.set("type", "secret");
      params.set("variant", "live");
    } else if (normalizedVotingType.includes("roll")) {
      params.set("type", "rollcall");
      params.set("variant", "rollcall-live");
    } else {
      params.set("type", "standard");
      params.set("variant", "live");
    }

    return `/results?${params.toString()}`;
  }

  function waitingHref(): string {
    const params = new URLSearchParams();
    if (sessionCode) params.set("sessionCode", sessionCode);
    return `/waiting?${params.toString()}`;
  }

  function handleConfirmationAction() {
    if (confirmationKind === "secret") {
      window.location.href = waitingHref();
      return;
    }

    window.location.href = resultsHref();
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
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-x-hidden pb-20 text-grey-900 [background:var(--gradient-screen-signin)] sm:m-0 sm:w-full"
>
  <VoterHeader
    sessionCode={sessionCode || "Connecting..."}
    onSignOut={signOut}
  />

  <section
    class={[
      "mx-auto mt-17 w-[calc(100%-42px)] max-w-88 pb-12 sm:w-[min(calc(100vw-80px),800px)] sm:max-w-none sm:pb-[clamp(70px,5.2vw,100px)]",
      isQuickVote ? "sm:mt-8" : "sm:mt-[clamp(86px,7.55vw,145px)]",
    ]}
    aria-labelledby="ballot-title"
  >
    <header
      class={[
        "min-h-28 rounded-lg border border-grey-400 bg-white p-5 sm:min-h-[clamp(110px,6.25vw,120px)] sm:px-[clamp(32px,2.08vw,40px)] sm:py-[clamp(22px,1.56vw,30px)]",
        hasProxy && !isQuickVote && "sm:min-h-[clamp(160px,9.84vw,189px)]",
      ]}
    >
      <h1
        class="text-base leading-6 font-medium sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)]"
        id="ballot-title"
      >
        {motionTitle}
      </h1>
      <p
        class="mx-0 mt-2 mb-0 text-xs leading-5 font-medium sm:mt-1 sm:text-[clamp(16px,0.94vw,18px)] sm:leading-[clamp(24px,1.35vw,26px)]"
      >
        Voting Type: {votingType}
      </p>
      {#if hasProxy && !isQuickVote}
        <p
          class="mx-0 mt-2 mb-0 text-xs leading-4 font-medium text-slate-600 sm:mt-3 sm:text-[clamp(16px,0.94vw,18px)] sm:leading-[clamp(24px,1.35vw,26px)] sm:text-slate-500"
        >
          Choose one response for each ballot assigned to you. Your proxy ballot
          is recorded separately from your own vote.
        </p>
      {/if}
    </header>

    {#if loading}
      <p class="mt-4 text-sm font-medium text-grey-700" aria-live="polite">
        Loading ballot...
      </p>
    {:else if loadError}
      <p class="mt-4 text-sm font-medium text-red-600" role="alert">{loadError}</p>
    {:else}
      <form class="mt-4" onsubmit={(event) => void submitBallot(event)}>
        <div class="flex flex-col gap-4">
          {#each instances as instance (instance.id)}
            <VotingInstanceFieldset
              {instance}
              {choices}
              selectedChoiceId={selections[instance.id]}
              disabled={submitted || Boolean(instance.hasVoted) || submitting}
              compact={hasProxy}
              quickVote={isQuickVote}
              onChange={updateSelection}
            />
          {/each}
        </div>

        {#if submitError}
          <p class="mt-3 text-center text-sm font-medium text-red-600" role="alert">
            {submitError}
          </p>
        {/if}

        {#if allAlreadyVoted}
          <p class="mt-4 text-center text-sm font-medium text-grey-700">
            You have already voted on this motion.
          </p>
        {/if}

        <button
          class={[
            "mx-auto mt-5 flex h-11 w-44 items-center justify-center rounded-full text-sm font-bold text-white focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_70%)] focus-visible:outline-offset-3 sm:mt-[clamp(18px,1.56vw,30px)] sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(280px,18.59vw,357px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-8",
            submitted || allAlreadyVoted
              ? "cursor-default bg-red-200 shadow-[0_2px_2px_rgb(0_0_0_/_0.18)]"
              : "cursor-pointer bg-red-600 shadow-[0_3px_2px_rgb(0_0_0_/_0.25)] disabled:cursor-not-allowed disabled:opacity-65",
          ]}
          type="submit"
          disabled={!allSelected || submitted || submitting || allAlreadyVoted}
        >
          {submitted || allAlreadyVoted
            ? "Submitted"
            : submitting
              ? "Submitting..."
              : "Submit"}
        </button>
      </form>
    {/if}
  </section>

  <AppFooter wide />

  {#if confirmationOpen}
    <div
      class="fixed inset-0 z-20 grid place-items-center bg-grey-900/28 p-5"
      role="presentation"
    >
      <div
        class="flex min-h-55 w-[min(100%,350px)] flex-col items-center justify-between gap-8 rounded-lg border border-grey-600 bg-white px-5 pt-9 pb-7 text-center sm:min-h-[clamp(260px,15.63vw,300px)] sm:w-[min(calc(100vw-80px),1200px)] sm:gap-[clamp(42px,3.13vw,60px)] sm:pt-[clamp(42px,2.6vw,50px)] sm:pb-[clamp(28px,1.56vw,30px)]"
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirmation-title"
      >
        <div>
          <h2
            class="text-2xl leading-9 font-semibold text-black sm:text-[clamp(36px,2.34vw,45px)] sm:leading-[clamp(46px,2.92vw,56px)]"
            id="confirmation-title"
          >
            {confirmationVoteNoun} Submitted
          </h2>
          {#if confirmationKind === "secret"}
            <p
              class="mx-0 mt-2 mb-0 text-sm leading-5 font-medium text-grey-900 sm:mt-3 sm:text-[clamp(18px,1.04vw,20px)] sm:leading-[clamp(26px,1.46vw,28px)]"
            >
              You have successfully voted. Please wait for your host to push out
              results.
            </p>
          {:else}
            <p
              class="mx-0 mt-2 mb-0 text-sm leading-5 font-medium text-grey-900 sm:mt-3 sm:text-[clamp(18px,1.04vw,20px)] sm:leading-[clamp(26px,1.46vw,28px)]"
            >
              You have successfully voted. Click below to view live results.
            </p>
          {/if}
        </div>

        <button
          class="h-12 w-[min(100%,240px)] cursor-pointer rounded-full bg-red-500 text-base font-bold text-white shadow-[0_3px_2px_rgb(0_0_0_/_0.25)] focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_70%)] focus-visible:outline-offset-3 sm:h-[clamp(54px,3.28vw,63px)] sm:w-[clamp(260px,16.41vw,315px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-8"
          type="button"
          onclick={handleConfirmationAction}
        >
          {confirmationKind === "secret" ? "OK" : "View Live Results"}
        </button>
      </div>
    </div>
  {/if}
</main>
