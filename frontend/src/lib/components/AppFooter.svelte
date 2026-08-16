<script lang="ts">
  import MessageSquareText from "@lucide/svelte/icons/message-square-text";
  import heartUrl from "$lib/assets/heart.svg?url";

  let { wide = false, flow = false } = $props();
</script>

{#if flow}
  <div class="h-20 w-full flex-[0_0_81px]" aria-hidden="true"></div>
{/if}

<footer
  class={[
    "right-0 bottom-0 left-0 flex h-20 items-center justify-between gap-5 border-t border-grey-400 bg-grey-50 pt-4 pr-5 pb-4 pl-6 text-slate-800",
    flow ? "fixed z-10" : "absolute",
    wide && "sm:px-6 sm:pt-5 sm:pb-5 min-[1100px]:px-8",
  ]}
>
  <p
    class={[
      "w-50 text-xs leading-4 ",
      wide &&
        "sm:w-auto sm:text-sm sm:leading-5 sm:whitespace-nowrap min-[1100px]:text-base min-[1100px]:leading-6",
    ]}
  >
    Designed, developed and maintained with
    <span
      class={[
        "mx-1 inline-flex h-4 w-4 align-[-3px]",
        wide &&
          "sm:mx-1 sm:h-5 sm:w-4 sm:align-[-4px] min-[1100px]:mx-1 min-[1100px]:h-6 min-[1100px]:w-5 min-[1100px]:align-[-5px]",
      ]}
      aria-label="love"
    >
      <img class="block size-full" src={heartUrl} alt="" />
    </span>
    by ScottyLabs.
  </p>

  <a
    class={[
      "inline-flex h-10 w-27 items-center justify-center gap-2 rounded-full border border-slate-400 py-2 pr-4 pl-4 text-xs leading-4 font-semibold text-slate-800 focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_75%)] focus-visible:outline-offset-3",
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
