<script lang="ts">
  import { page } from "$app/state";
  import { apiUrl } from "$lib/api/base";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import VoterHeader from "$lib/components/VoterHeader.svelte";

  let proxyFor = $state(page.url.searchParams.get("proxyVotes") ?? "");
  const sessionCode = $derived(
    page.url.searchParams.get("sessionCode")?.trim() || "Demo session",
  );

  const hasProxyInput = $derived(proxyFor.trim().length > 0);

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  function goBack() {
    window.location.href = "/home";
  }

  function submitProxy() {
    if (!hasProxyInput) return;
    // This will POST to /session/{sessionCode}/proxy once backend wiring is ready.
    // For now, show the pending proxy review waiting state.
    const searchParams = new URLSearchParams({
      sessionCode,
      proxyStatus: "pending",
      proxyVotes: proxyFor.trim(),
    });

    window.location.href = `/waiting?${searchParams.toString()}`;
  }

  function continueWithoutProxy() {
    if (hasProxyInput) return;
    const searchParams = new URLSearchParams({ sessionCode });
    window.location.href = `/waiting?${searchParams.toString()}`;
  }
</script>

<svelte:head>
  <title>TartanVote | Proxy Votes</title>
</svelte:head>

<main
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-hidden bg-(image:--gradient-screen-signin) text-grey-900 sm:m-0 sm:w-full"
>
  <VoterHeader {sessionCode} onSignOut={signOut} />

  <div
    class="mt-4 ml-[21px] flex h-5 w-[372px] items-center justify-between max-[374px]:w-[calc(100%-28px)] sm:hidden"
  >
    <button
      class="inline-flex size-5 cursor-pointer items-center justify-center rounded-full border-0 bg-red-500 p-0 text-grey-50"
      type="button"
      aria-label="Back to home"
      onclick={goBack}
    >
      <svg class="size-4" viewBox="0 0 20 20" aria-hidden="true">
        <path
          d="M12.5 4.5 7 10l5.5 5.5"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </button>

  </div>

  <section
    class="mx-auto mt-[77px] flex w-[350px] flex-col items-center max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(145px,11.9vw,228px)] sm:w-[min(calc(100vw-80px),952px)]"
    aria-labelledby="proxy-title"
  >
    <div
      class="flex w-[345px] flex-col gap-2.5 max-[374px]:w-[calc(100%-28px)] sm:w-[min(100%,870px)]"
    >
      <h1
        class="m-0 text-center text-lg leading-[21.95px] font-semibold text-black sm:text-[clamp(22px,1.56vw,30px)] sm:leading-[clamp(30px,1.98vw,38px)]"
        id="proxy-title"
      >
        Proxy Votes <span>(Optional)</span>
      </h1>
      <p
        class="m-0 px-0.75 py-1.5 text-[13px] leading-[18.48px] font-medium text-grey-700 sm:px-1.25 sm:py-[clamp(8px,0.52vw,10px)] sm:text-center sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
      >
        If you are voting on behalf of someone, please enter their AndrewID(s)
        separated by a comma. Otherwise, continue.
      </p>
    </div>

    <section
      class="mt-3 flex h-30 w-87.5 flex-col gap-2.75 rounded-[5.77px] border-[0.58px] border-grey-200 bg-white px-5 py-3 max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(10px,0.52vw,10px)] sm:h-[clamp(140px,8.75vw,168px)] sm:w-[min(100%,869px)] sm:gap-[clamp(14px,1.04vw,20px)] sm:rounded-[10px] sm:border sm:px-[clamp(36px,3.65vw,70px)] sm:py-[clamp(24px,1.56vw,30px)]"
      aria-label="Proxy vote information"
    >
      <label
        class="text-sm leading-[18.48px] font-medium text-slate-800 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)] sm:font-bold"
        for="proxy-for">I am proxying for:</label
      >
      <input
        class="h-10 w-75 rounded-[5.77px] border-[0.58px] border-slate-400 px-2.5 py-0.5 text-[13px] leading-4.5 font-normal italic text-grey-900 placeholder:text-slate-400 placeholder:opacity-100 focus:border-red-600 focus:outline-3 focus:outline-[color-mix(in_srgb,var(--color-red-600),transparent_78%)] max-[374px]:w-full sm:h-[clamp(44px,2.6vw,50px)] sm:w-full sm:rounded-[10px] sm:border sm:px-4 sm:py-1 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
        id="proxy-for"
        type="text"
        bind:value={proxyFor}
        placeholder="ex: scottylabs0, scottylabs1, scottylabs123"
        autocomplete="off"
      />
    </section>

    <div
      class="mt-5 flex h-12 w-87.5 items-center gap-1.5 max-[374px]:w-[calc(100%-28px)] max-[374px]:gap-2 sm:mt-[clamp(0px,0.52vw,10px)] sm:h-20 sm:w-[min(100%,590px)] sm:justify-center sm:gap-[clamp(16px,1.25vw,24px)]"
    >
      <button
        class="h-9 min-w-0 flex-1 cursor-pointer rounded-full border-0 bg-red-600 text-xs leading-[18.48px] font-semibold text-white shadow-[0_2.31px_2.31px_rgb(0_0_0_/_0.25)] hover:brightness-96 disabled:cursor-not-allowed disabled:bg-red-200 disabled:hover:brightness-100 sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(240px,14.58vw,280px)] sm:flex-[0_0_clamp(240px,14.58vw,280px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)] sm:font-bold sm:shadow-[0_4px_4px_rgb(0_0_0_/_0.25)]"
        type="button"
        disabled={!hasProxyInput}
        onclick={submitProxy}>Submit proxy</button
      >
      <button
        class="h-9 min-w-0 flex-1 cursor-pointer rounded-full border-0 bg-red-600 text-xs leading-[18.48px] font-semibold text-white shadow-[0_2.31px_2.31px_rgb(0_0_0_/_0.25)] hover:brightness-96 disabled:cursor-not-allowed disabled:bg-red-200 disabled:hover:brightness-100 sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(240px,14.58vw,280px)] sm:flex-[0_0_clamp(240px,14.58vw,280px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)] sm:font-bold sm:shadow-[0_4px_4px_rgb(0_0_0_/_0.25)]"
        type="button"
        disabled={hasProxyInput}
        onclick={continueWithoutProxy}>Continue without proxy</button
      >
    </div>
  </section>

  <AppFooter wide />
</main>
