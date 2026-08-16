<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { fetchAuthStatus } from "$lib/api/auth";
  import { apiUrl } from "$lib/api/base";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import logoUrl from "$lib/assets/tartanvote-logo.svg?url";

  let checkingAuth = $state(true);

  onMount(() => {
    void fetchAuthStatus()
      .then((status) => {
        if (status.logged_in) {
          void goto("/home", { replaceState: true });
          return;
        }
        checkingAuth = false;
      })
      .catch(() => {
        checkingAuth = false;
      });
  });

  function signIn() {
    window.location.href = apiUrl("/auth/login");
  }
</script>

<svelte:head>
  <title>TartanVote</title>
</svelte:head>

<main
  class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-hidden bg-(image:--gradient-screen-signin) text-grey-900 sm:m-0 sm:w-full sm:rounded-none sm:shadow-none"
>
  <section
    class="flex w-full flex-col items-center gap-4 px-5 pt-47 pb-30 max-[374px]:px-4 sm:min-h-[calc(100svh-81px)] sm:gap-5 sm:px-6 sm:pt-[clamp(88px,12vh,120px)] sm:pb-32 min-[1100px]:gap-0 min-[1100px]:pt-[clamp(96px,13.5vh,146px)] min-[1100px]:pb-35"
    aria-labelledby="app-title"
  >
    <div
      class="flex h-14 w-75 items-center justify-center gap-2 max-[374px]:w-full sm:h-18 sm:w-[min(100%,390px)] min-[1100px]:h-25 min-[1100px]:w-[min(100%,545px)] min-[1100px]:gap-3"
    >
      <img
        class="block h-13 w-17 shrink-0 sm:h-16 sm:w-21 min-[1100px]:h-23 min-[1100px]:w-30"
        src={logoUrl}
        alt=""
        width="68"
        height="51"
      />
      <h1
        class="font-brand text-5xl leading-none text-red-600 [-webkit-text-stroke:8px_var(--color-white)] [paint-order:stroke_fill] sm:text-6xl min-[1100px]:text-8xl"
        id="app-title"
      >
        <span class="text-black">Tartan</span>Vote
      </h1>
    </div>

    <p
      class="mt-1 mb-0 w-86 text-center text-base leading-5 text-grey-600 max-[374px]:w-full sm:mt-2 sm:w-[min(calc(100vw-48px),760px)] sm:text-xl sm:leading-8 sm:text-balance min-[1100px]:mt-8 min-[1100px]:w-[min(calc(100vw-48px),1212px)] min-[1100px]:text-4xl min-[1100px]:leading-11"
    >
      An election management tool for proxy voting and campus governance.
    </p>

    <section
      class="mt-0 flex min-h-56 w-88 flex-col items-center gap-5 rounded-sm border border-grey-200 bg-white px-4 py-7 max-[374px]:w-full sm:mt-6 sm:min-h-78 sm:w-[min(calc(100vw-48px),620px)] sm:items-start sm:gap-6 sm:rounded-lg sm:px-7 sm:py-9 min-[1100px]:mt-14 min-[1100px]:min-h-95 min-[1100px]:w-[min(calc(100vw-48px),850px)] min-[1100px]:gap-0 min-[1100px]:rounded-lg min-[1100px]:px-9 min-[1100px]:py-10"
      aria-label="Sign in with CMU account"
    >
      <div
        class="ml-2 flex w-69 flex-col gap-2 self-start max-[374px]:ml-0 max-[374px]:w-full sm:ml-0 sm:w-full"
      >
        <h2
          class="text-lg leading-6 font-semibold text-black sm:text-3xl sm:leading-10 min-[1100px]:text-5xl min-[1100px]:leading-14"
        >
          Sign in
        </h2>
        <p
          class="w-81 text-xs leading-3 font-medium text-grey-900 max-[374px]:w-full sm:w-auto sm:text-sm sm:leading-5 sm:text-balance min-[1100px]:text-xl min-[1100px]:leading-7"
        >
          with your CMU Account. You'll continue to home after authenticating.
        </p>
      </div>

      <button
        type="button"
        class="h-10 w-65 cursor-pointer rounded-full bg-red-600 text-sm leading-4 font-semibold text-white shadow-[0_2.2px_2.2px_rgb(0_0_0_/_0.25)] hover:bg-red-700 focus-visible:outline-3 focus-visible:outline-offset-3 focus-visible:outline-[color-mix(in_srgb,var(--color-red-600),transparent_75%)] disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none disabled:hover:bg-red-200 max-[374px]:w-full sm:mx-auto sm:mt-7 sm:h-13 sm:w-[min(100%,420px)] sm:text-lg sm:leading-6 sm:shadow-[0_3px_2px_rgb(0_0_0_/_0.25)] min-[1100px]:mt-21 min-[1100px]:h-15 min-[1100px]:w-[min(100%,500px)] min-[1100px]:text-2xl min-[1100px]:leading-8 min-[1100px]:font-bold min-[1100px]:shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        onclick={signIn}
        disabled={checkingAuth}>Sign in with CMU SSO</button
      >

      <p
        class="flex min-h-10 w-68 items-center justify-center rounded-xs bg-grey-50 px-2 py-2 text-center text-xs leading-3 text-grey-600 max-[374px]:w-full sm:mx-auto sm:min-h-14 sm:w-[min(100%,420px)] sm:rounded-sm sm:px-3 sm:leading-5 min-[1100px]:mt-8 min-[1100px]:min-h-18 min-[1100px]:w-[min(100%,500px)] min-[1100px]:px-4 min-[1100px]:py-3 min-[1100px]:text-base min-[1100px]:leading-6 min-[1100px]:font-medium"
      >
        CMU authentication is required to vote. Only @andrew.cmu.edu accounts
        can join sessions.
      </p>
    </section>
  </section>

  <AppFooter wide />
</main>
