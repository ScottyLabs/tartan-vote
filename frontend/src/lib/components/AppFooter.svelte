<script lang="ts">
  import MessageSquareText from "@lucide/svelte/icons/message-square-text";
  import heartUrl from "$lib/assets/heart.svg?url";

  let { wide = false, flow = false } = $props();
</script>

{#if flow}
  <div class="h-[81px] w-full flex-[0_0_81px]" aria-hidden="true"></div>
{/if}

<footer
  class={[
    "right-0 bottom-0 left-0 flex h-[81px] items-center justify-between gap-[18px] border-t border-grey-400 bg-grey-50 pt-4 pr-5 pb-[15px] pl-6 text-slate-800",
    flow ? "fixed z-10" : "absolute",
    wide && "sm:px-6 sm:pt-[21px] sm:pb-5 min-[1100px]:px-[33px]",
  ]}
>
  <p
    class={[
      "m-0 w-[200px] text-[11.33px] leading-[17px] font-normal",
      wide &&
        "sm:w-auto sm:text-sm sm:leading-5 sm:whitespace-nowrap min-[1100px]:text-base min-[1100px]:leading-6",
    ]}
  >
    Designed, developed and maintained with
    <span
      class={[
        "mx-0.5 inline-flex h-[16.29px] w-[14.26px] align-[-3px]",
        wide &&
          "sm:mx-[3px] sm:h-5 sm:w-[17.5px] sm:align-[-4px] min-[1100px]:mx-1 min-[1100px]:h-[23px] min-[1100px]:w-[20.13px] min-[1100px]:align-[-5px]",
      ]}
      aria-label="love"
    >
      <img class="block size-full" src={heartUrl} alt="" />
    </span>
    by ScottyLabs.
  </p>

  <a
    class={[
      "inline-flex h-[38px] w-[107px] items-center justify-center gap-1.5 rounded-full border border-slate-400 py-[9px] pr-[15px] pl-4 text-[11.33px] leading-[17px] font-semibold text-slate-800 no-underline focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_75%)] focus-visible:outline-offset-[3px]",
      wide && "sm:w-auto sm:gap-2 sm:text-sm sm:leading-5",
    ]}
    href="https://docs.google.com/forms/u/2/d/e/1FAIpQLSdyrmvSSwyizNVZ97jyVT-r8D-bNVg_x0DYFBecJ9Mwhhteug/viewform?usp=send_form"
  >
    <MessageSquareText
      class="size-5 flex-none"
      strokeWidth={1.9}
      aria-hidden="true"
    />
    Feedback
  </a>
</footer>
