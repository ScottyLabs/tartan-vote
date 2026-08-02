<script lang="ts">
  import X from "@lucide/svelte/icons/x";

  interface Props {
    requester?: string;
    proxyVotes?: string[];
    onclose: () => void;
    onaccept?: () => void;
    ondecline?: () => void;
    onkick?: () => void;
  }

  let {
    requester = "Scottylabs1",
    proxyVotes = [
      "scottylabs1234567",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
      "scottylabs67",
    ],
    onclose,
    onaccept = onclose,
    ondecline = onclose,
    onkick = onclose,
  }: Props = $props();
</script>

<div
  class="fixed inset-0 z-20 grid place-items-center bg-grey-700/40"
  role="presentation"
>
  <div
    class="relative min-h-[298px] w-[min(calc(100%-40px),1200px)] rounded-[10px] border border-grey-600 bg-white px-[38px] pt-12 pb-6 text-center max-[700px]:px-6"
    role="dialog"
    aria-modal="true"
    aria-labelledby="proxy-dialog-title"
  >
    <button
      class="absolute top-[19px] right-5 size-[22px] cursor-pointer border-0 bg-transparent p-0"
      type="button"
      aria-label="Close proxy request"
      onclick={onclose}
    >
      <X
        class="block size-full text-grey-400"
        strokeWidth={3}
        aria-hidden="true"
      />
    </button>
    <h2 class="m-0 text-2xl leading-8" id="proxy-dialog-title">
      {requester}'s Requested Proxy Votes:
    </h2>
    <p
      class="mx-auto mt-[27px] mb-[45px] max-w-[1120px] text-lg leading-[26px]"
    >
      {proxyVotes.join(", ")}
    </p>
    <div
      class="flex justify-center gap-[50px] max-[700px]:flex-wrap max-[700px]:gap-3"
    >
      <button
        class="h-10 w-[180px] cursor-pointer rounded-full border-0 bg-green-400 font-sans text-lg font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onaccept}
      >
        Accept Proxy
      </button>
      <button
        class="h-10 w-[180px] cursor-pointer rounded-full border-0 bg-red-400 font-sans text-lg font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={ondecline}
      >
        Decline Proxy
      </button>
      <button
        class="h-10 w-[180px] cursor-pointer rounded-full border-0 bg-slate-700 font-sans text-lg font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onkick}
      >
        Kick
      </button>
    </div>
  </div>
</div>
