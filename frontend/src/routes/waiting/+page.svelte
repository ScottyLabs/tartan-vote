<script lang="ts">
  import { page } from "$app/state";
  import ChevronLeft from "@lucide/svelte/icons/chevron-left";
  import { onMount } from "svelte";
  import { fetchAuthStatus } from "$lib/api/auth";
  import { apiUrl } from "$lib/api/base";
  import { checkMotion } from "$lib/api/motion";
  import { getVoteInstances } from "$lib/api/votes";
  import { participantVisibilityFromMotionData } from "$lib/domain/ballot";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import VoterHeader from "$lib/components/VoterHeader.svelte";

  const MOTION_POLL_MS = 2500;

  const sessionCode = $derived(
    page.url.searchParams.get("sessionCode")?.trim() || "Demo session",
  );
  const hasPendingProxyReview = $derived(
    page.url.searchParams.get("proxyStatus") === "pending",
  );
  const pendingProxyVotes = $derived(
    page.url.searchParams.get("proxyVotes")?.trim() ?? "",
  );
  const pendingProxyVoteList = $derived(
    pendingProxyVotes
      .split(",")
      .map((proxyVote) => proxyVote.trim())
      .filter(Boolean),
  );
  const pendingProxyVoteText = $derived(pendingProxyVoteList.join(", "));
  const pendingProxyVoteCount = $derived(pendingProxyVoteList.length);
  let userAndrewId = $state<string | null>(null);
  let pollError = $state("");
  let checkingMotion = $state(true);
  let votedWaitingForHost = $state(false);
  let noVoteInstances = $state(false);
  let activeMotionName = $state("");
  let activeMotionVisibility = $state<"live" | "hidden_until_release" | null>(
    null,
  );

  function ballotHref(): string {
    const params = new URLSearchParams();
    const code = page.url.searchParams.get("sessionCode")?.trim();
    if (code) params.set("sessionCode", code);
    const proxyVotes = page.url.searchParams.get("proxyVotes")?.trim();
    if (proxyVotes) params.set("proxyVotes", proxyVotes);
    const proxyStatus = page.url.searchParams.get("proxyStatus")?.trim();
    if (proxyStatus) params.set("proxyStatus", proxyStatus);
    const query = params.toString();
    return query ? `/ballot?${query}` : "/ballot";
  }

  function resultsHref(
    visibility: "live" | "hidden_until_release" | null = "hidden_until_release",
  ): string {
    const isLive = visibility === "live";
    const params = new URLSearchParams({
      variant: isLive ? "live" : "secret-final",
      type: isLive ? "standard" : "secret",
      phase: isLive ? "live" : "final",
    });
    const code = page.url.searchParams.get("sessionCode")?.trim();
    if (code) params.set("sessionCode", code);
    return `/results?${params.toString()}`;
  }

  onMount(() => {
    void fetchAuthStatus()
      .then((status) => {
        userAndrewId = status.user_andrew_id;
      })
      .catch(() => {
        userAndrewId = null;
      });

    let cancelled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;

    async function pollActiveMotion() {
      try {
        const { active_motion } = await checkMotion();
        if (cancelled) return;
        pollError = "";
        checkingMotion = false;

        if (!active_motion) {
          const wasVotedWaiting = votedWaitingForHost;
          const endedVisibility = activeMotionVisibility;
          votedWaitingForHost = false;
          noVoteInstances = false;
          activeMotionName = "";
          activeMotionVisibility = null;
          if (wasVotedWaiting) {
            window.location.href = resultsHref(endedVisibility);
            return;
          }
        } else {
          activeMotionName = active_motion.name;
          activeMotionVisibility = participantVisibilityFromMotionData(
            active_motion.data,
          );
          const voteInstances = await getVoteInstances();
          if (cancelled) return;

          if (voteInstances.length === 0) {
            votedWaitingForHost = false;
            noVoteInstances = true;
          } else if (voteInstances.some((instance) => !instance.has_voted)) {
            noVoteInstances = false;
            votedWaitingForHost = false;
            window.location.href = ballotHref();
            return;
          } else if (activeMotionVisibility === "live") {
            window.location.href = resultsHref("live");
            return;
          } else {
            noVoteInstances = false;
            votedWaitingForHost = true;
          }
        }
      } catch (error) {
        if (cancelled) return;
        checkingMotion = false;
        pollError =
          error instanceof Error && error.message.trim() !== ""
            ? error.message
            : "Unable to check for an active motion.";
      }

      if (!cancelled) {
        timer = setTimeout(() => {
          void pollActiveMotion();
        }, MOTION_POLL_MS);
      }
    }

    void pollActiveMotion();

    return () => {
      cancelled = true;
      if (timer) clearTimeout(timer);
    };
  });

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  function goBack() {
    const searchParams = new URLSearchParams({ sessionCode });
    if (pendingProxyVotes) searchParams.set("proxyVotes", pendingProxyVotes);
    window.location.href = `/proxy?${searchParams.toString()}`;
  }
</script>

<svelte:head>
  <title>TartanVote | Waiting Room</title>
</svelte:head>

<main
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-hidden bg-(image:--gradient-screen-signin) text-grey-900 sm:m-0 sm:w-full"
>
  <VoterHeader {sessionCode} onSignOut={signOut} />

  <button
    class="absolute top-[clamp(128px,8.33vw,160px)] left-[clamp(28px,3.6vw,69px)] hidden size-[clamp(32px,2.08vw,40px)] cursor-pointer items-center justify-center rounded-full border-0 bg-red-500 p-0 text-grey-50 sm:inline-flex"
    type="button"
    aria-label="Back to proxy setup"
    onclick={goBack}
  >
    <ChevronLeft
      class="size-[clamp(22px,1.46vw,28px)]"
      aria-hidden="true"
    />
  </button>

  <div
    class="mt-4 ml-[21px] flex h-5 w-[372px] items-center justify-between max-[374px]:w-[calc(100%-28px)] sm:hidden"
  >
    <button
      class="inline-flex size-5 cursor-pointer items-center justify-center rounded-full border-0 bg-red-500 p-0 text-grey-50"
      type="button"
      aria-label="Back to proxy setup"
      onclick={goBack}
    >
      <ChevronLeft class="size-4" aria-hidden="true" />
    </button>

  </div>

  <section
    class="mx-auto mt-[62px] flex w-[350px] flex-col items-center gap-5 max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(132px,13.33vw,256px)] sm:w-[min(calc(100vw-80px),916px)] sm:gap-[30px]"
    aria-labelledby="waiting-title"
  >
    <div
      class="flex w-[345px] flex-col items-center gap-2.5 text-center max-[374px]:w-[calc(100%-28px)] sm:w-[min(100%,916px)]"
    >
      {#if votedWaitingForHost}
        <h1
          class="m-0 text-lg leading-[21.95px] font-semibold text-black sm:text-[clamp(22px,1.56vw,30px)] sm:leading-[clamp(30px,1.98vw,38px)]"
          id="waiting-title"
        >
          Vote submitted
        </h1>
        <p
          class="m-0 text-[13px] leading-5 font-medium text-grey-700 sm:w-[min(100%,780px)] sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
        >
          {#if activeMotionName}
            You have voted on <strong class="font-bold">{activeMotionName}</strong>.
          {:else}
            You have voted on the active motion.
          {/if}
          Please wait for your host to release results.
        </p>
      {:else}
        <h1
          class="m-0 text-lg leading-[21.95px] font-semibold text-black sm:text-[clamp(22px,1.56vw,30px)] sm:leading-[clamp(30px,1.98vw,38px)]"
          id="waiting-title"
        >
          You are in the Waiting Room
        </h1>
        <p
          class="m-0 text-[13px] leading-3.5 font-medium text-grey-700 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
        >
          Proxy Request:
          {#if hasPendingProxyReview}
            <strong class="font-bold text-red-500">Not Yet Requested</strong>
          {:else}
            N/A
          {/if}
        </p>
        {#if hasPendingProxyReview}
          <p
            class="m-0 text-[13px] leading-5 font-medium text-grey-700 sm:w-[min(100%,780px)] sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
          >
            Your host will push a motion or election shortly. Please ensure that
            all inputted Proxy IDs are accurate before requesting the host's
            confirmation.
          </p>
        {:else}
          <p
            class="m-0 hidden text-[clamp(18px,1.25vw,24px)] leading-[clamp(26px,1.67vw,32px)] font-medium text-grey-700 sm:block sm:w-[min(100%,778px)]"
          >
            Your host will push a motion or election shortly. You are continuing
            without proxy votes.
          </p>
          <p
            class="m-0 text-[13px] leading-5 font-medium text-grey-700 sm:hidden"
          >
            Your host will push a motion or election shortly. You are continuing
            without proxy votes.
          </p>
        {/if}
      {/if}
    </div>

    <section
      class="flex h-[120px] w-[350px] flex-col items-center justify-center rounded-[10px] border border-grey-200 bg-white px-[50px] py-[30px] text-center max-[374px]:w-[calc(100%-28px)] sm:h-auto sm:min-h-[82px] sm:w-[min(100%,780px)]"
      aria-label="Current voter information"
    >
      <p
        class="m-0 w-[325px] text-sm leading-5 font-medium text-slate-800 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
      >
        <strong class="font-bold">Your AndrewID:</strong>
        {userAndrewId ?? "Unavailable"}
      </p>
      {#if hasPendingProxyReview && !votedWaitingForHost}
        <p
          class="mt-5 hidden w-[325px] text-[clamp(18px,1.25vw,24px)] leading-[clamp(26px,1.67vw,32px)] font-medium text-slate-800 sm:block"
        >
          <strong class="font-bold"
            >Your Proxy Votes({pendingProxyVoteCount}):</strong
          > <em class="italic text-slate-300">{pendingProxyVoteText}</em>
        </p>
        <p
          class="mt-5 w-[325px] text-sm leading-5 font-medium text-slate-800 sm:hidden"
        >
          <strong class="font-bold"
            >Your Pending Proxy Votes ({pendingProxyVoteCount}):</strong
          > <em class="italic text-slate-300">{pendingProxyVoteText}</em>
        </p>
      {/if}
      {#if votedWaitingForHost}
        <p
          class="mt-5 w-[325px] text-sm leading-5 font-medium text-slate-800 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
        >
          <strong class="font-bold text-red-500">Status:</strong> Voted, waiting
          for host
        </p>
      {:else if noVoteInstances}
        <p
          class="mt-5 w-[325px] text-sm leading-5 font-medium text-slate-800 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
        >
          <strong class="font-bold text-red-500">Status:</strong> No voting
          instances for this motion
        </p>
      {/if}
    </section>

    {#if hasPendingProxyReview && !votedWaitingForHost}
      <button
        class="inline-flex h-9 w-[174px] cursor-pointer items-center justify-center rounded-[50px] border-0 bg-red-500 text-sm leading-[18.48px] font-semibold text-white shadow-[0_2.31px_1.16px_rgb(0_0_0_/_0.25)] hover:bg-red-600 disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none disabled:hover:bg-red-200 sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(250px,18.59vw,357px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)] sm:font-bold sm:shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        disabled
        title="Available after backend connection"
        >Proxy Review-not yet available</button
      >
    {/if}

    {#if checkingMotion}
      <p
        class="m-0 text-[13px] leading-5 font-medium text-grey-600 sm:text-[clamp(16px,1.04vw,20px)]"
        aria-live="polite"
      >
        Checking for an active motion...
      </p>
    {:else if pollError}
      <p
        class="m-0 text-[13px] leading-5 font-medium text-red-600 sm:text-[clamp(16px,1.04vw,20px)]"
        role="alert"
      >
        {pollError} Retrying...
      </p>
    {:else if votedWaitingForHost}
      <p
        class="m-0 text-[13px] leading-5 font-medium text-grey-600 sm:text-[clamp(16px,1.04vw,20px)]"
        aria-live="polite"
      >
        Waiting for the host to end this motion and release results...
      </p>
    {:else if noVoteInstances}
      <p
        class="m-0 text-[13px] leading-5 font-medium text-grey-600 sm:text-[clamp(16px,1.04vw,20px)]"
        aria-live="polite"
      >
        A motion is active, but you don't have any vote instances. Stay here or
        rejoin from Home if needed.
      </p>
    {/if}
  </section>

  <AppFooter wide />
</main>
