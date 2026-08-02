<script lang="ts">
  import { onMount } from "svelte";
  import X from "@lucide/svelte/icons/x";

  interface Props {
    mode: "create" | "edit";
    initialName?: string;
    error?: string;
    onclose: () => void;
    onsubmit: (name: string) => void;
  }

  let {
    mode,
    initialName = "",
    error = "",
    onclose,
    onsubmit,
  }: Props = $props();
  let name = $state("");

  onMount(() => {
    name = initialName;
  });

  function submit(event: SubmitEvent) {
    event.preventDefault();
    onsubmit(name.trim());
  }
</script>

<div
  class="fixed inset-0 z-30 grid place-items-center bg-grey-700/40"
  role="presentation"
>
  <div
    class="relative min-h-75 w-[min(calc(100%-40px),720px)] rounded-[10px] border border-grey-600 bg-white px-13 pt-12 pb-8"
    role="dialog"
    aria-modal="true"
    aria-labelledby="organization-dialog-title"
  >
    <button
      class="absolute top-[19px] right-5 size-[22px] cursor-pointer border-0 bg-transparent p-0"
      type="button"
      aria-label="Close organization name dialog"
      onclick={onclose}
    >
      <X
        class="block size-full text-grey-400"
        strokeWidth={3}
        aria-hidden="true"
      />
    </button>
    <h2
      class="mx-0 mt-0 mb-[30px] text-center text-2xl leading-8"
      id="organization-dialog-title"
    >
      {mode === "create" ? "Add Organization" : "Edit Organization Name"}
    </h2>
    <form onsubmit={submit}>
      <label>
        <span class="mb-2.5 block text-lg leading-[26px] font-medium text-black"
          >Organization Name</span
        >
        <input
          class="h-[50px] w-full rounded-[10px] border border-slate-800 bg-white px-4 py-0 font-sans text-base text-black placeholder:text-grey-400 placeholder:italic placeholder:opacity-100"
          bind:value={name}
          maxlength="36"
          placeholder="ex. ScottyLabs Exec Board"
          autocomplete="off"
          required
        />
      </label>
      {#if error}<p
          class="mt-2 mb-0 text-center text-sm text-red-700"
          role="alert"
        >
          {error}
        </p>{/if}
      <div class="mt-8 flex justify-center gap-[50px]">
        <button
          class="h-10 w-[180px] cursor-pointer rounded-full border-0 bg-slate-700 font-sans text-lg font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
          type="button"
          onclick={onclose}
        >
          Cancel
        </button>
        <button
          class="h-10 w-[180px] cursor-pointer rounded-full border-0 bg-green-400 font-sans text-lg font-bold text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
          type="submit"
        >
          {mode === "create" ? "Add Organization" : "Save Name"}
        </button>
      </div>
    </form>
  </div>
</div>
