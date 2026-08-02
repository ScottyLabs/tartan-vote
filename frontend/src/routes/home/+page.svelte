<script lang="ts">
  import { apiUrl } from "$lib/api/base";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import logoUrl from "$lib/assets/tartanvote-logo.svg?url";

  let sessionCode = $state("");

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  function createSession() {
    window.location.href = "/host";
  }

  function updateSessionCode(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    sessionCode = input.value.replace(/^\s+/, "").replace(/\s+/g, "-");
  }

  function joinSession() {
    const normalizedSessionCode = sessionCode.replace(/^-+|-+$/g, "");
    if (!normalizedSessionCode) return;

    const searchParams = new URLSearchParams({
      sessionCode: normalizedSessionCode,
    });
    window.location.href = `/proxy?${searchParams.toString()}`;
  }
</script>

<svelte:head>
  <title>TartanVote | Home</title>
</svelte:head>

<main
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-hidden bg-(image:--gradient-screen-signin) text-grey-900 sm:m-0 sm:w-full sm:rounded-none sm:shadow-none"
>
  <header
    class="flex h-28 items-start justify-between bg-red-600 pt-[39px] pr-[17px] pb-[38px] pl-[15px] shadow-[0_4px_4px_rgb(0_0_0_/_0.25)] sm:items-center sm:px-[clamp(24px,3.6vw,69px)] sm:py-[clamp(30px,3vw,39px)]"
  >
    <a
      class="flex h-[27px] w-[150px] items-center gap-[3px] text-red-600 no-underline sm:h-[clamp(35px,3.2vw,61px)] sm:w-[clamp(150px,13.5vw,260px)] sm:gap-[clamp(3px,0.42vw,8px)]"
      href="/"
      aria-label="TartanVote home"
    >
      <img
        class="block h-[25px] w-[33px] shrink-0 sm:h-[clamp(25px,3.1vw,59px)] sm:w-[clamp(33px,4vw,77px)]"
        src={logoUrl}
        alt=""
        width="33"
        height="25"
      />
      <span
        class="font-brand text-[26.03px] leading-none font-normal tracking-normal text-red-600 [-webkit-text-stroke:5px_var(--color-white)] [paint-order:stroke_fill] sm:text-[clamp(26.03px,2.1vw,40px)] sm:[-webkit-text-stroke:clamp(5px,0.26vw,10px)_var(--color-white)]"
        ><span class="text-black">Tartan</span>Vote</span
      >
    </a>

    <button
      class="inline-flex h-[35px] w-20 cursor-pointer items-center justify-center gap-1 rounded-full border-0 bg-grey-100 text-[10px] leading-[12.75px] font-normal text-slate-800 sm:h-[clamp(42px,3.2vw,61px)] sm:w-[clamp(110px,10.4vw,199px)] sm:gap-[clamp(4px,0.42vw,8px)] sm:text-[clamp(14px,1.25vw,24px)] sm:leading-[clamp(20px,1.7vw,32px)]"
      type="button"
      onclick={signOut}
    >
      <svg
        class="size-2.5 sm:size-[clamp(12px,1.05vw,20px)]"
        viewBox="0 0 16 16"
        aria-hidden="true"
      >
        <path
          d="M6.7 3.2H4.1a1.4 1.4 0 0 0-1.4 1.4v6.8a1.4 1.4 0 0 0 1.4 1.4h2.6"
          fill="none"
          stroke="currentColor"
          stroke-width="1.4"
          stroke-linecap="round"
        />
        <path
          d="M9.4 4.9 12.5 8l-3.1 3.1M12.2 8H6.7"
          fill="none"
          stroke="currentColor"
          stroke-width="1.4"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
      Sign out
    </button>
  </header>

  <section
    class="mx-auto mt-[125px] flex w-[350px] flex-col items-center gap-[9px] max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(82px,10vw,108px)] sm:w-[min(calc(100vw-48px),870px)] sm:gap-[clamp(12px,1.15vw,22px)]"
    aria-labelledby="home-title"
  >
    <div
      class="flex h-[57.55px] w-[350px] flex-col items-center gap-[5px] text-center max-[374px]:w-[calc(100%-28px)] sm:h-[clamp(66px,5.2vw,100px)] sm:w-full sm:gap-[clamp(6px,0.52vw,10px)]"
    >
      <h1
        class="m-0 text-lg leading-[21.95px] font-semibold tracking-normal text-black sm:text-[clamp(20px,1.56vw,30px)] sm:leading-[clamp(26px,1.98vw,38px)]"
        id="home-title"
      >
        Hello, Scottylabs
      </h1>
      <p
        class="m-0 flex h-[30.55px] w-[345px] items-center justify-center text-[13.86px] leading-[18.48px] font-normal text-grey-700 max-[374px]:w-full sm:h-auto sm:w-[min(100%,843px)] sm:px-[5px] sm:py-[clamp(5px,0.52vw,10px)] sm:text-[clamp(16px,1.25vw,24px)] sm:leading-[clamp(22px,1.67vw,32px)]"
      >
        Join an existing session or host your own.
      </p>
    </div>

    <section
      class="flex h-[252px] w-[350px] flex-col items-center rounded-[4.25px] border border-grey-200 bg-white px-3 py-5 max-[374px]:w-[calc(100%-28px)] sm:h-[clamp(310px,26.6vw,510px)] sm:w-[min(calc(100vw-48px),700px)] sm:px-[clamp(24px,3.9vw,75px)] sm:py-[clamp(28px,2.5vw,48px)]"
      aria-label="Session actions"
    >
      <form
        class="flex w-[326px] flex-col items-center gap-4 max-[374px]:w-full sm:w-[min(100%,550px)] sm:gap-[clamp(18px,1.46vw,28px)]"
        onsubmit={(event) => {
          event.preventDefault();
          joinSession();
        }}
      >
        <label
          class="w-[314px] text-base leading-[22.66px] font-semibold text-black max-[374px]:w-full sm:w-auto sm:text-center sm:text-[clamp(20px,1.56vw,30px)] sm:leading-[clamp(26px,1.98vw,38px)]"
          for="session-code">Session Code</label
        >
        <input
          class="h-10 w-[275px] rounded-[5.96px] border border-slate-400 px-3.5 text-center text-sm leading-[19.08px] font-medium text-grey-900 placeholder:text-slate-400 placeholder:opacity-100 focus:border-red-600 focus:outline-3 focus:outline-[color-mix(in_srgb,var(--color-red-600),transparent_78%)] max-[374px]:w-full sm:h-[clamp(46px,3.65vw,70px)] sm:w-[min(100%,490px)] sm:rounded-[clamp(5.96px,0.52vw,10px)] sm:text-[clamp(16px,1.25vw,24px)] sm:leading-[clamp(22px,1.67vw,32px)]"
          id="session-code"
          type="text"
          value={sessionCode}
          oninput={updateSessionCode}
          placeholder="ex: happy-giraffe"
          autocomplete="off"
          autocapitalize="none"
          spellcheck="false"
        />
        <button
          class="inline-flex h-9 min-h-9 w-[300px] shrink-0 basis-9 cursor-pointer items-center justify-center rounded-full border-0 bg-red-600 px-4 text-[13px] leading-5 font-medium text-white shadow-[0_2.39px_2.39px_rgb(0_0_0_/_0.25)] hover:bg-red-700 disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none disabled:hover:bg-red-200 max-[374px]:w-full sm:h-[clamp(42px,3.13vw,60px)] sm:min-h-[clamp(42px,3.13vw,60px)] sm:w-[min(100%,500px)] sm:text-[clamp(15px,1.04vw,20px)] sm:leading-[clamp(21px,1.46vw,28px)] sm:shadow-[0_clamp(2.39px,0.21vw,4px)_2px_rgb(0_0_0_/_0.25)]"
          type="submit"
          disabled={!sessionCode.trim()}>Join session</button
        >
      </form>

      <div
        class="my-2.5 grid h-[31px] w-[300px] grid-cols-[1fr_auto_1fr] items-center gap-2 text-[14.31px] font-light text-grey-700 max-[374px]:w-full sm:mt-[clamp(12px,1.04vw,20px)] sm:mb-[clamp(14px,1.15vw,22px)] sm:h-[clamp(32px,2.08vw,40px)] sm:w-[min(100%,550px)] sm:gap-[clamp(8px,0.52vw,10px)] sm:text-[clamp(16px,1.25vw,24px)] sm:leading-[clamp(22px,1.67vw,32px)] sm:font-medium"
        aria-hidden="true"
      >
        <span class="h-px bg-grey-200"></span><strong
          class="sm:bg-white sm:px-[clamp(8px,0.52vw,10px)]">OR</strong
        ><span class="h-px bg-grey-200"></span>
      </div>

      <button
        class="inline-flex h-9 min-h-9 w-[300px] shrink-0 basis-9 cursor-not-allowed items-center justify-center rounded-full border-0 bg-red-200 px-4 text-[13px] leading-5 font-medium text-white shadow-[0_2.39px_2.39px_rgb(0_0_0_/_0.25)] max-[374px]:w-full sm:hidden"
        type="button"
        disabled>Host a session (not available on mobile)</button
      >
      <button
        class="hidden h-[clamp(42px,3.13vw,60px)] min-h-[clamp(42px,3.13vw,60px)] w-[min(100%,500px)] shrink-0 cursor-pointer items-center justify-center rounded-full border-0 bg-red-600 px-4 text-[clamp(15px,1.04vw,20px)] leading-[clamp(21px,1.46vw,28px)] font-medium text-white shadow-[0_clamp(2.39px,0.21vw,4px)_2px_rgb(0_0_0_/_0.25)] hover:bg-red-700 max-[374px]:w-full sm:inline-flex"
        type="button"
        onclick={createSession}>Host a session</button
      >
    </section>
  </section>

  <AppFooter wide />
</main>
