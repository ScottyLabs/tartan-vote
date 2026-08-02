<script lang="ts">
  import X from "@lucide/svelte/icons/x";

  interface Props {
    currentlyShared: boolean;
    onclose: () => void;
    onconfirm: () => void;
  }

  let { currentlyShared, onclose, onconfirm }: Props = $props();
</script>

<div
  class="fixed inset-0 z-20 grid place-items-center bg-grey-700/40 p-5"
  role="presentation"
>
  <div
    class="relative flex min-h-75 w-[min(1200px,calc(100vw-40px))] flex-col justify-between gap-8 rounded-[10px] border border-grey-500 bg-white px-5 pt-15 pb-[30px] text-grey-900"
    role="dialog"
    aria-modal="true"
    aria-labelledby="sharing-title"
    aria-describedby="sharing-description"
  >
    <button
      class="absolute top-4 right-5 size-[30px] cursor-pointer border-0 bg-transparent p-0"
      type="button"
      aria-label="Close"
      onclick={onclose}
    >
      <X
        class="block size-full text-grey-400"
        strokeWidth={3}
        aria-hidden="true"
      />
    </button>

    <div class="text-center">
      <h2
        class="m-0 text-[clamp(18px,1.25vw,24px)] leading-[1.34] font-medium"
        id="sharing-title"
      >
        Are you sure you want to {currentlyShared ? "unshare" : "share"} live results?
      </h2>
      <p
        class="mx-0 mt-[25px] mb-0 text-[clamp(15px,1.042vw,20px)] leading-[1.4] font-medium"
        id="sharing-description"
      >
        {#if currentlyShared}
          Voters will no longer be able to see the live results. You can share
          them again at any time while this voting instance is active.
        {:else}
          Voters will be able to see the live vote totals and results as they
          update. You can unshare them again at any time.
        {/if}
      </p>
    </div>

    <div class="flex justify-center gap-[clamp(24px,2.604vw,50px)]">
      <button
        class="h-10 w-[180px] cursor-pointer rounded-[100px] border-0 bg-slate-700 font-sans text-lg font-medium text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onclose}
      >
        Cancel
      </button>
      <button
        class="h-10 w-[180px] cursor-pointer rounded-[100px] border-0 bg-red-500 font-sans text-lg font-medium text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onconfirm}
      >
        Confirm
      </button>
    </div>
  </div>
</div>
