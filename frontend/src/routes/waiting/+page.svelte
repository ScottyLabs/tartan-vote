<script lang="ts">
  import { page } from "$app/state";
  import ChevronLeft from "@lucide/svelte/icons/chevron-left";
  import { apiUrl } from "$lib/api/base";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import VoterHeader from "$lib/components/VoterHeader.svelte";

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
          all inputted Proxy IDs are accurate before requesting the host’s
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
    </div>

    <section
      class="flex h-[120px] w-[350px] flex-col items-center justify-center rounded-[10px] border border-grey-200 bg-white px-[50px] py-[30px] text-center max-[374px]:w-[calc(100%-28px)] sm:h-auto sm:min-h-[82px] sm:w-[min(100%,780px)]"
      aria-label="Current voter information"
    >
      <p
        class="m-0 w-[325px] text-sm leading-5 font-medium text-slate-800 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
      >
        <strong class="font-bold">Your AndrewID:</strong> Available after backend
        connection
      </p>
      {#if hasPendingProxyReview}
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
    </section>

    {#if hasPendingProxyReview}
      <button
        class="inline-flex h-9 w-[174px] cursor-pointer items-center justify-center rounded-[50px] border-0 bg-red-500 text-sm leading-[18.48px] font-semibold text-white shadow-[0_2.31px_1.16px_rgb(0_0_0_/_0.25)] hover:bg-red-600 disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none disabled:hover:bg-red-200 sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(250px,18.59vw,357px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)] sm:font-bold sm:shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        disabled
        title="Available after backend connection"
        >Proxy Review-not yet available</button
      >
    {/if}
  </section>

  <AppFooter wide />
</main>
