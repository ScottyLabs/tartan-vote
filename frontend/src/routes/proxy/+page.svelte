<script lang="ts">
  import { page } from "$app/state";
  import ChevronLeft from "@lucide/svelte/icons/chevron-left";
  import { apiUrl } from "$lib/api/base";
  import { setSessionProxy } from "$lib/api/session";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import VoterHeader from "$lib/components/VoterHeader.svelte";

  let proxyFor = $state(page.url.searchParams.get("proxyVotes") ?? "");
  let submitting = $state(false);
  let error = $state("");
  const sessionCode = $derived(page.url.searchParams.get("sessionCode")?.trim() || "");
  const hasSessionCode = $derived(sessionCode.length > 0);
  const sessionCodeDisplay = $derived(hasSessionCode ? sessionCode : "N/A");

  const hasProxyInput = $derived(proxyFor.trim().length > 0);

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  function goBack() {
    window.location.href = "/home";
  }

  async function submitProxy() {
    if (!hasSessionCode) {
      error = "Missing session code. Rejoin from Home.";
      return;
    }
    if (!hasProxyInput || submitting) return;
    error = "";
    submitting = true;
    try {
      await setSessionProxy(sessionCode, {
        is_senator: true,
        proxy_for: proxyFor.trim(),
      });
      const searchParams = new URLSearchParams({
        sessionCode,
        proxyStatus: "pending",
        proxyVotes: proxyFor.trim(),
      });

      window.location.href = `/waiting?${searchParams.toString()}`;
    } catch (requestError) {
      error =
        requestError instanceof Error
          ? requestError.message
          : "Unable to save proxy votes.";
    } finally {
      submitting = false;
    }
  }

  async function continueWithoutProxy() {
    if (!hasSessionCode) {
      error = "Missing session code. Rejoin from Home.";
      return;
    }
    if (hasProxyInput || submitting) return;
    error = "";
    submitting = true;
    try {
      await setSessionProxy(sessionCode, {
        is_senator: true,
        proxy_for: null,
      });
      const searchParams = new URLSearchParams({ sessionCode });
      window.location.href = `/waiting?${searchParams.toString()}`;
    } catch (requestError) {
      error =
        requestError instanceof Error
          ? requestError.message
          : "Unable to continue without proxy.";
    } finally {
      submitting = false;
    }
  }
</script>

<svelte:head>
  <title>TartanVote | Proxy Votes</title>
</svelte:head>

<main
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-hidden bg-(image:--gradient-screen-signin) text-grey-900 sm:m-0 sm:w-full"
>

<VoterHeader sessionCode={sessionCodeDisplay} onSignOut={signOut} />

  <button
    class="absolute top-[clamp(128px,8.33vw,160px)] left-[clamp(28px,3.6vw,69px)] hidden size-[clamp(32px,2.08vw,40px)] cursor-pointer items-center justify-center rounded-full bg-red-500 p-0 text-grey-50 sm:inline-flex"
    type="button"
    aria-label="Back to home"
    onclick={goBack}
  >
    <ChevronLeft
      class="size-[clamp(22px,1.46vw,28px)]"
      aria-hidden="true"
    />
  </button>

  <div
    class="mt-4 ml-5 flex h-5 w-93 items-center justify-between max-[374px]:w-[calc(100%-28px)] sm:hidden"
  >
    <button
      class="inline-flex size-5 cursor-pointer items-center justify-center rounded-full bg-red-500 p-0 text-grey-50"
      type="button"
      aria-label="Back to home"
      onclick={goBack}
    >
      <ChevronLeft class="size-4" aria-hidden="true" />
    </button>

  </div>

  <section
    class="mx-auto mt-19 flex w-88 flex-col items-center max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(145px,11.9vw,228px)] sm:w-[min(calc(100vw-80px),952px)]"
    aria-labelledby="proxy-title"
  >
    <div
      class="flex w-86 flex-col gap-3 max-[374px]:w-[calc(100%-28px)] sm:w-[min(100%,870px)]"
    >
      <h1
        class="text-center text-lg leading-5 font-semibold text-black sm:text-[clamp(22px,1.56vw,30px)] sm:leading-[clamp(30px,1.98vw,38px)]"
        id="proxy-title"
      >
        Proxy Votes <span>(Optional)</span>
      </h1>
      <p
        class="px-1 py-2 text-xs leading-5 font-medium text-grey-700 sm:py-[clamp(8px,0.52vw,10px)] sm:text-center sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
      >
        If you are voting on behalf of someone, please enter their AndrewID(s)
        separated by a comma. Otherwise, continue.
      </p>
    </div>

    <section
      class="mt-3 flex h-30 w-88 flex-col gap-3 rounded-md border border-grey-200 bg-white px-5 py-3 max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(10px,0.52vw,10px)] sm:h-[clamp(140px,8.75vw,168px)] sm:w-[min(100%,869px)] sm:gap-[clamp(14px,1.04vw,20px)] sm:rounded-lg sm:px-[clamp(36px,3.65vw,70px)] sm:py-[clamp(24px,1.56vw,30px)]"
      aria-label="Proxy vote information"
    >
      <label
        class="text-sm leading-5 font-medium text-slate-800 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)] sm:font-bold"
        for="proxy-for">I am proxying for:</label
      >
      <input
        class="h-10 w-75 rounded-md border border-slate-400 px-3 py-1 text-xs leading-5 italic text-grey-900 placeholder:text-slate-400 focus:border-red-600 focus:outline-3 focus:outline-[color-mix(in_srgb,var(--color-red-600),transparent_78%)] max-[374px]:w-full sm:h-[clamp(44px,2.6vw,50px)] sm:w-full sm:rounded-lg sm:px-4 sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)]"
        id="proxy-for"
        type="text"
        bind:value={proxyFor}
        placeholder="ex: scottylabs0, scottylabs1, scottylabs123"
        autocomplete="off"
        disabled={!hasSessionCode}
      />
    </section>

    <div
      class="mt-5 flex h-12 w-88 items-center gap-2 max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(0px,0.52vw,10px)] sm:h-20 sm:w-[min(100%,590px)] sm:justify-center sm:gap-[clamp(16px,1.25vw,24px)]"
    >
      <button
        class="h-9 min-w-0 flex-1 cursor-pointer rounded-full bg-red-600 text-xs leading-5 font-semibold text-white shadow-[0_2.31px_2.31px_rgb(0_0_0_/_0.25)] hover:brightness-96 disabled:cursor-not-allowed disabled:bg-red-200 disabled:hover:brightness-100 sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(240px,14.58vw,280px)] sm:flex-[0_0_clamp(240px,14.58vw,280px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)] sm:font-bold sm:shadow-[0_4px_4px_rgb(0_0_0_/_0.25)]"
        type="button"
        disabled={!hasSessionCode || !hasProxyInput || submitting}
        onclick={() => void submitProxy()}
        >{submitting ? "Submitting..." : "Submit proxy"}</button
      >
      <button
        class="h-9 min-w-0 flex-1 cursor-pointer rounded-full bg-red-600 text-xs leading-5 font-semibold text-white shadow-[0_2.31px_2.31px_rgb(0_0_0_/_0.25)] hover:brightness-96 disabled:cursor-not-allowed disabled:bg-red-200 disabled:hover:brightness-100 sm:h-[clamp(52px,3.13vw,60px)] sm:w-[clamp(240px,14.58vw,280px)] sm:flex-[0_0_clamp(240px,14.58vw,280px)] sm:text-[clamp(20px,1.25vw,24px)] sm:leading-[clamp(28px,1.67vw,32px)] sm:font-bold sm:shadow-[0_4px_4px_rgb(0_0_0_/_0.25)]"
        type="button"
        disabled={!hasSessionCode || hasProxyInput || submitting}
        onclick={() => void continueWithoutProxy()}
        >{submitting && !hasProxyInput ? "Continuing..." : "Continue without proxy"}</button
      >
    </div>
    {#if error}
      <p class="mt-4 text-center text-sm font-semibold text-red-700" role="alert">
        {error}
      </p>
    {/if}
  </section>

  <AppFooter wide />
</main>
