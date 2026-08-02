<script lang="ts">
  import X from "@lucide/svelte/icons/x";

  interface Props {
    onclose: () => void;
    onconfirm: () => void;
  }

  let { onclose, onconfirm }: Props = $props();
</script>

<div
  class="fixed inset-0 z-20 grid place-items-center bg-grey-700/40 p-5"
  role="presentation"
>
  <div
    class="relative flex min-h-75 w-[min(1200px,calc(100vw-40px))] flex-col justify-between gap-8 rounded-[10px] border border-grey-500 bg-white px-5 pt-15 pb-[30px] text-grey-900 max-sm:min-h-85 max-sm:pt-16"
    role="dialog"
    aria-modal="true"
    aria-labelledby="end-vote-title"
    aria-describedby="end-vote-description"
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
        id="end-vote-title"
      >
        Are you sure you want to redirect voters back to the waiting room?
      </h2>
      <p
        class="mx-0 mt-[25px] mb-0 text-[clamp(15px,1.042vw,20px)] leading-[1.4] font-medium"
        id="end-vote-description"
      >
        The result of this instance has been saved. Upon confirmation, all
        voters will be redirected back to the Waiting Room to await another
        motion/election push.
      </p>
    </div>
    <div
      class="flex justify-center gap-[clamp(24px,2.604vw,50px)] max-sm:w-full"
    >
      <button
        class="h-10 w-[180px] cursor-pointer rounded-[100px] border-0 bg-slate-700 font-sans text-lg font-medium text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] max-sm:w-[min(180px,45%)]"
        type="button"
        onclick={onclose}
      >
        Cancel
      </button>
      <button
        class="h-10 w-[180px] cursor-pointer rounded-[100px] border-0 bg-red-500 font-sans text-lg font-medium text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)] max-sm:w-[min(180px,45%)]"
        type="button"
        onclick={onconfirm}
      >
        Confirm
      </button>
    </div>
  </div>
</div>
