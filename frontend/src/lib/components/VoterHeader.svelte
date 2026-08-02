<script lang="ts">
  import logoUrl from "$lib/assets/tartanvote-logo.svg?url";
  import signoutUrl from "$lib/assets/signout.svg?url";

  interface Props {
    sessionCode: string;
    onSignOut: () => void;
  }

  let { sessionCode, onSignOut }: Props = $props();
</script>

<header
  class="flex h-28 items-start justify-between gap-3 bg-red-600 pt-[39px] pr-[17px] pb-[38px] pl-[15px] shadow-[0_4px_4px_rgb(0_0_0_/_0.25)] sm:h-[clamp(96px,5.83vw,112px)] sm:items-center sm:px-[clamp(28px,3.6vw,69px)] sm:py-[clamp(24px,1.3vw,25px)]"
>
  <a
    class="flex h-[27px] w-[150px] items-center gap-[3px] no-underline sm:h-[clamp(52px,3.07vw,59px)] sm:w-[clamp(210px,13.54vw,260px)] sm:gap-[clamp(6px,0.42vw,8px)]"
    href="/home"
    aria-label="TartanVote home"
  >
    <img
      class="block h-[25px] w-[33px] flex-none sm:h-[clamp(52px,3.07vw,59px)] sm:w-[clamp(68px,4.01vw,77px)]"
      src={logoUrl}
      alt=""
      width="77"
      height="59"
    />
    <span
      class="font-brand text-[26px] leading-none text-red-600 [-webkit-text-stroke:5px_var(--color-white)] [paint-order:stroke_fill] sm:text-[clamp(36px,2.08vw,40px)] sm:[-webkit-text-stroke:clamp(7px,0.52vw,10px)_var(--color-white)]"
      ><span class="text-black">Tartan</span>Vote</span
    >
  </a>

  <p
    class="absolute top-32 right-[15px] m-0 text-xs leading-5 font-medium text-slate-400 sm:static sm:ml-auto sm:pr-[clamp(20px,1.56vw,30px)] sm:text-center sm:text-[clamp(18px,1.25vw,24px)] sm:leading-[clamp(26px,1.67vw,32px)] sm:text-grey-50"
  >
    Session Code: {sessionCode}
  </p>

  <button
    class="inline-flex h-[35px] w-20 cursor-pointer items-center justify-center gap-1 rounded-full border-0 bg-grey-100 font-sans text-[10px] font-medium text-slate-800 sm:h-[clamp(50px,3.18vw,61px)] sm:w-[clamp(150px,10.36vw,199px)] sm:gap-[clamp(6px,0.42vw,8px)] sm:text-[clamp(18px,1.25vw,24px)] sm:font-bold"
    type="button"
    onclick={onSignOut}
  >
    <img
      class="size-2.5 sm:size-[clamp(16px,1.04vw,20px)]"
      src={signoutUrl}
      alt=""
      width="20"
      height="20"
    />
    <span>Sign out</span>
  </button>
</header>
