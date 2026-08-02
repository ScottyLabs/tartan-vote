<script lang="ts">
  import { page } from "$app/state";
  import logoUrl from "$lib/assets/tartanvote-logo.svg?url";

  let { children } = $props();

  const isOverview = $derived(page.url.pathname === "/host");
</script>

<main
  class={[
    "min-h-svh w-full min-w-5xl text-grey-900",
    isOverview
      ? "relative overflow-hidden bg-(image:--gradient-screen-signin) [&_a:focus-visible]:outline-3 [&_a:focus-visible]:outline-offset-2 [&_a:focus-visible]:outline-[color-mix(in_srgb,var(--color-red-600),transparent_70%)] [&_button:focus-visible]:outline-3 [&_button:focus-visible]:outline-offset-2 [&_button:focus-visible]:outline-[color-mix(in_srgb,var(--color-red-600),transparent_70%)]"
      : "flex flex-col bg-(image:--gradient-screen-signin)",
  ]}
>
  <header
    class={[
      "flex h-[clamp(60px,5.833vw,112px)] items-center justify-between bg-red-600 pr-[clamp(34px,3.542vw,68px)] pl-[clamp(18px,1.771vw,34px)] shadow-[0_4px_4px_rgb(0_0_0_/_0.25)]",
      !isOverview && "shrink-0 basis-[clamp(60px,5.833vw,112px)]",
    ]}
  >
    <a
      class="flex items-center gap-2 no-underline"
      href="/home"
      aria-label="TartanVote home"
    >
      <img
        class="block h-[clamp(32px,3.073vw,59px)] w-[clamp(42px,4.01vw,77px)]"
        src={logoUrl}
        alt=""
        width="77"
        height="59"
      />
      <span
        class="font-brand text-[clamp(24px,2.083vw,40px)] text-red-600 [-webkit-text-stroke:clamp(5px,0.521vw,10px)_var(--color-white)] [paint-order:stroke_fill]"
      >
        <span class="text-black">Tartan</span>Vote
      </span>
    </a>

    <p class="m-0 text-[clamp(18px,1.563vw,30px)] leading-[38px] text-white">
      HOSTING DASHBOARD
    </p>
  </header>

  {@render children()}
</main>
