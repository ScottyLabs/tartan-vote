<script lang="ts">
  import { apiUrl } from "$lib/api/base";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import logoUrl from "$lib/assets/tartanvote-logo.svg?url";

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
    class="flex w-full flex-col items-center gap-4 px-[21px] pt-[186px] pb-[120px] max-[374px]:px-3.5 sm:min-h-[calc(100svh-81px)] sm:gap-[18px] sm:px-6 sm:pt-[clamp(88px,12vh,120px)] sm:pb-32 min-[1100px]:gap-0 min-[1100px]:pt-[clamp(96px,13.5vh,146px)] min-[1100px]:pb-[140px]"
    aria-labelledby="app-title"
  >
    <div
      class="flex h-[54px] w-[300px] items-center justify-center gap-1.5 max-[374px]:w-full sm:h-[72px] sm:w-[min(100%,390px)] sm:gap-2 min-[1100px]:h-[100px] min-[1100px]:w-[min(100%,545px)] min-[1100px]:gap-2.5"
    >
      <img
        class="block h-[51px] w-[68px] shrink-0 sm:h-[62px] sm:w-[82px] min-[1100px]:h-[90px] min-[1100px]:w-[119px]"
        src={logoUrl}
        alt=""
        width="68"
        height="51"
      />
      <h1
        class="m-0 font-brand text-[52.05px] leading-none font-normal tracking-normal text-red-600 [-webkit-text-stroke:8px_var(--color-white)] [paint-order:stroke_fill] max-[374px]:text-[46px] sm:text-[64px] min-[1100px]:text-8xl"
        id="app-title"
      >
        <span class="text-black">Tartan</span>Vote
      </h1>
    </div>

    <p
      class="mt-1 mb-0 w-[345px] text-center text-base leading-5 font-normal text-grey-600 max-[374px]:w-full sm:mt-2 sm:w-[min(calc(100vw-48px),760px)] sm:text-[22px] sm:leading-[30px] sm:text-balance min-[1100px]:mt-8 min-[1100px]:w-[min(calc(100vw-48px),1212px)] min-[1100px]:text-4xl min-[1100px]:leading-11"
    >
      An election management tool for proxy voting and campus governance.
    </p>

    <section
      class="mt-0 flex min-h-[225px] w-[350px] flex-col items-center gap-5 rounded-sm border border-grey-200 bg-white px-4 py-[29px] max-[374px]:w-full sm:mt-[22px] sm:min-h-[310px] sm:w-[min(calc(100vw-48px),620px)] sm:items-start sm:gap-6 sm:rounded-lg sm:px-7 sm:py-[34px] min-[1100px]:mt-[54px] min-[1100px]:min-h-[380px] min-[1100px]:w-[min(calc(100vw-48px),850px)] min-[1100px]:gap-0 min-[1100px]:rounded-[10px] min-[1100px]:px-[37px] min-[1100px]:py-[38px]"
      aria-label="Sign in with CMU account"
    >
      <div
        class="ml-1.5 flex w-[274px] flex-col gap-1.5 self-start max-[374px]:ml-0 max-[374px]:w-full sm:ml-0 sm:w-full sm:gap-1.5"
      >
        <h2
          class="m-0 text-lg leading-[22px] font-semibold tracking-normal text-black sm:text-3xl sm:leading-[38px] min-[1100px]:text-[45px] min-[1100px]:leading-14"
        >
          Sign in
        </h2>
        <p
          class="m-0 w-[324px] text-[10px] leading-[11.2px] font-medium text-grey-900 max-[374px]:w-full sm:w-auto sm:text-sm sm:leading-5 sm:text-balance min-[1100px]:text-xl min-[1100px]:leading-7"
        >
          with your CMU Account. You'll return to this page after
          authenticating.
        </p>
      </div>

      <button
        type="button"
        class="h-[39px] w-[258px] cursor-pointer rounded-full border-0 bg-red-600 text-sm leading-[17.5px] font-semibold text-white shadow-[0_2.2px_2.2px_rgb(0_0_0_/_0.25)] hover:bg-red-700 focus-visible:outline-3 focus-visible:outline-offset-3 focus-visible:outline-[color-mix(in_srgb,var(--color-red-600),transparent_75%)] max-[374px]:w-full sm:mx-auto sm:mt-[26px] sm:h-[50px] sm:w-[min(100%,420px)] sm:text-lg sm:leading-6 sm:shadow-[0_3px_2px_rgb(0_0_0_/_0.25)] min-[1100px]:mt-[84px] min-[1100px]:h-[60px] min-[1100px]:w-[min(100%,500px)] min-[1100px]:text-2xl min-[1100px]:leading-8 min-[1100px]:font-bold min-[1100px]:shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        onclick={signIn}>Sign in with CMU SSO</button
      >

      <p
        class="m-0 flex min-h-[38px] w-[273px] items-center justify-center rounded-xs bg-grey-50 px-2 py-1.5 text-center text-[8.75px] leading-[13.1px] text-grey-600 max-[374px]:w-full sm:mx-auto sm:min-h-[54px] sm:w-[min(100%,420px)] sm:rounded-sm sm:px-3 sm:py-2 sm:text-xs sm:leading-[18px] min-[1100px]:mt-[30px] min-[1100px]:min-h-[70px] min-[1100px]:w-[min(100%,500px)] min-[1100px]:px-3.5 min-[1100px]:py-2.5 min-[1100px]:text-base min-[1100px]:leading-6 min-[1100px]:font-medium"
      >
        CMU authentication is required to vote. Only @andrew.cmu.edu accounts
        can join sessions.
      </p>
    </section>
  </section>

  <AppFooter wide />
</main>
