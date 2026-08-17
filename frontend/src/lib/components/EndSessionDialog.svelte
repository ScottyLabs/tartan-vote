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
    class="relative flex min-h-75 w-[min(1200px,calc(100vw-40px))] flex-col justify-between gap-8 rounded-lg border border-grey-500 bg-white px-5 pt-15 pb-8 text-grey-900"
    role="alertdialog"
    aria-modal="true"
    aria-labelledby="end-session-title"
    aria-describedby="end-session-description"
  >
    <button
      class="absolute top-4 right-5 size-8 cursor-pointer bg-transparent p-0"
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
        class="text-[clamp(18px,1.25vw,24px)] leading-[1.34] font-medium"
        id="end-session-title"
      >
        Are you sure you want to end this session?
      </h2>
      <p
        class="mx-0 mt-6 mb-0 text-[clamp(15px,1.042vw,20px)] leading-[1.4] font-medium"
        id="end-session-description"
      >
        All participants will leave the session and will no longer be able to
        vote. This action cannot be undone.
      </p>
    </div>

    <div class="flex justify-center gap-13">
      <button
        class="h-10 w-45 cursor-pointer rounded-full bg-slate-700 text-lg font-medium text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onclose}
      >
        Cancel
      </button>
      <button
        class="h-10 w-45 cursor-pointer rounded-full bg-red-500 text-lg font-medium text-grey-50 shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onconfirm}
      >
        End Session
      </button>
    </div>
  </div>
</div>
