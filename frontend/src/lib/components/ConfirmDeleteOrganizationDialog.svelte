<script lang="ts">
  import X from "@lucide/svelte/icons/x";

  interface Props {
    organizationName: string;
    onclose: () => void;
    ondelete: () => void;
  }

  let { organizationName, onclose, ondelete }: Props = $props();
</script>

<div
  class="fixed inset-0 z-30 grid place-items-center bg-grey-700/40"
  role="presentation"
>
  <div
    class="relative min-h-67.5 w-[min(calc(100%-40px),720px)] rounded-[10px]
    border border-grey-600 bg-white px-9.5 pt-12 pb-7.5 text-center"
    role="alertdialog"
    aria-modal="true"
    aria-labelledby="delete-title"
    aria-describedby="delete-description"
  >
    <button
      class="absolute top-[19px] right-5 size-[22px] cursor-pointer border-0 bg-transparent p-0"
      type="button"
      aria-label="Cancel deleting organization"
      onclick={onclose}
    >
      <X
        class="block size-full text-grey-400"
        strokeWidth={3}
        aria-hidden="true"
      />
    </button>
    <h2 class="m-0 text-2xl leading-8" id="delete-title">
      Delete {organizationName}?
    </h2>
    <p
      class="mx-auto mt-[27px] mb-[42px] text-lg leading-[26px] text-grey-700"
      id="delete-description"
    >
      Are you sure you want to delete this organization and its saved voting
      settings?
    </p>
    <div class="flex justify-center gap-[50px]">
      <button
        class="h-10 w-[180px] cursor-pointer rounded-full border-0 bg-slate-700 font-sans
    text-lg font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={onclose}
      >
        Cancel
      </button>
      <button
        class="h-10 w-[180px] cursor-pointer rounded-full border-0 bg-red-400 font-sans
    text-lg font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="button"
        onclick={ondelete}
      >
        Delete
      </button>
    </div>
  </div>
</div>
