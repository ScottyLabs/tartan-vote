<script lang="ts">
  import { onMount } from "svelte";
  import Trash2 from "@lucide/svelte/icons/trash-2";
  import X from "@lucide/svelte/icons/x";
  import { loadOrganizationSettings } from "$lib/domain/organizationSettings";

  interface Props {
    kind: "motion" | "election";
    onclose: () => void;
    onsubmit?: (configuration: {
      kind: "motion" | "election";
      name: string;
      votingType: string;
      options: string[];
      displayLiveResults: boolean;
      allowProxyVoting: boolean;
      enableQuorum: boolean;
      quorum: string;
    }) => void;
  }

  let { kind, onclose, onsubmit = onclose }: Props = $props();
  let name = $state("");
  let votingType = $state("Standard");
  let options = $state<string[]>([]);
  let displayLiveResults = $state(false);
  let allowProxyVoting = $state(false);
  let enableQuorum = $state(false);
  let quorum = $state("");
  let error = $state("");

  const title = $derived(
    kind === "motion" ? "Motion Configuration" : "Election Configuration",
  );
  const noun = $derived(kind === "motion" ? "Motion" : "Election");
  const nameExample = $derived(
    kind === "motion"
      ? "ex. Fund the spring student showcase"
      : "ex. Best Rust StuCo Instructor",
  );

  onMount(() => {
    const organizationSettings = loadOrganizationSettings();
    options = kind === "motion" ? ["Pass", "Reject", ""] : ["", "", ""];
    quorum = organizationSettings.quorum;
    enableQuorum = Boolean(organizationSettings.quorum);
  });

  function updateOption(index: number, value: string) {
    options[index] = value.slice(0, 36);
  }

  function addOption() {
    options.push("");
  }

  function removeOption(index: number) {
    if (options.length <= 2) {
      error = "Voting requires at least two options.";
      return;
    }
    options.splice(index, 1);
    error = "";
  }

  function submit(event: SubmitEvent) {
    event.preventDefault();
    if (!name.trim()) {
      error = `Enter a ${noun.toLowerCase()} name.`;
      return;
    }
    const completedOptions = options
      .map((option) => option.trim())
      .filter(Boolean);
    if (completedOptions.length < 2) {
      error = "Enter at least two voting options.";
      return;
    }
    const normalizedOptions = completedOptions.map((option) =>
      option.toLocaleLowerCase(),
    );
    if (new Set(normalizedOptions).size !== normalizedOptions.length) {
      error = "Each voting option must be unique.";
      return;
    }
    if (enableQuorum && !quorum.trim()) {
      error = "Enter a quorum count when quorum setup is enabled.";
      return;
    }
    error = "";
    onsubmit({
      kind,
      name: name.trim(),
      votingType,
      options: completedOptions,
      displayLiveResults,
      allowProxyVoting,
      enableQuorum,
      quorum,
    });
  }
</script>

<div
  class="fixed inset-0 z-20 grid place-items-center bg-grey-700/40"
  role="presentation"
>
  <div
    class="relative min-h-[776px] w-[min(calc(100%-40px),800px)] overflow-y-auto rounded-[10px] border border-grey-600 bg-white px-[49px] pt-[74px] pb-[42px] max-[760px]:max-h-[calc(100svh-24px)] max-[760px]:px-6 max-[760px]:pt-[54px] max-[760px]:pb-7"
    role="dialog"
    aria-modal="true"
    aria-labelledby="configuration-title"
  >
    <button
      class="absolute top-[39px] right-[49px] size-5 cursor-pointer border-0 bg-transparent p-0 max-[760px]:top-6 max-[760px]:right-6"
      type="button"
      aria-label={`Close ${noun.toLowerCase()} configuration`}
      onclick={onclose}
    >
      <X
        class="block size-full text-grey-400"
        strokeWidth={3}
        aria-hidden="true"
      />
    </button>

    <h2
      class="mt-0 mb-[29px] text-center text-4xl leading-11 font-semibold max-[760px]:text-[28px] max-[760px]:leading-9"
      id="configuration-title"
    >
      {title}
    </h2>

    <form onsubmit={submit}>
      <label>
        <span class="mb-3 block text-xl leading-7 font-medium text-slate-900"
          >{noun} Name</span
        >
        <input
          class="h-10 w-full rounded-[10px] border border-slate-800 bg-white px-5 text-sm text-black placeholder:text-grey-500 placeholder:italic placeholder:opacity-100"
          bind:value={name}
          maxlength="98"
          placeholder={nameExample}
          required
        />
      </label>

      <div
        class="mt-[52px] grid grid-cols-[344px_1fr] gap-[30px] max-[760px]:grid-cols-1"
      >
        <div>
          <label>
            <span
              class="mb-3 block text-xl leading-7 font-medium text-slate-900"
              >Voting Type</span
            >
            <select
              class="h-10 w-[270px] rounded-[10px] border border-slate-800 bg-white px-3.5 text-sm text-black"
              bind:value={votingType}
            >
              <option>Standard</option>
              <option>Secret</option>
              <option>Roll Call</option>
            </select>
          </label>

          <fieldset class="mt-[26px] border-0 p-0">
            <legend
              class="mb-3 block text-xl leading-7 font-medium text-slate-900"
              >Voting Options</legend
            >
            <div class="grid gap-1.5">
              {#each options as option, index (index)}
                <div class="flex items-center gap-1.5">
                  <input
                    class="h-10 w-[270px] rounded-[10px] border border-slate-800 bg-white px-4 text-sm text-black placeholder:text-grey-500 placeholder:italic placeholder:opacity-100"
                    value={option}
                    maxlength="36"
                    placeholder={kind === "motion"
                      ? index === 0
                        ? "Pass"
                        : index === 1
                          ? "Reject"
                          : `ex. Option ${index + 1}`
                      : index < 2
                        ? `ex. Person${index + 1}`
                        : `ex. Option ${index + 1}`}
                    aria-label={`Voting option ${index + 1}`}
                    oninput={(event) =>
                      updateOption(index, event.currentTarget.value)}
                  />
                  {#if index >= 2}
                    <button
                      class="size-7 cursor-pointer border-0 bg-transparent p-0"
                      type="button"
                      aria-label={`Delete option ${index + 1}`}
                      onclick={() => removeOption(index)}
                    >
                      <Trash2
                        class="block size-full text-grey-200"
                        strokeWidth={2.5}
                        aria-hidden="true"
                      />
                    </button>
                  {/if}
                </div>
              {/each}
            </div>
            <button
              class="mt-3.5 h-10 w-[170px] cursor-pointer rounded-full border border-slate-900 bg-white text-sm text-slate-800"
              type="button"
              onclick={addOption}>+ &nbsp; Add additional</button
            >
          </fieldset>
        </div>

        <div class="grid content-start gap-[15px]">
          <label class="flex items-center justify-between gap-4">
            <span
              class="whitespace-nowrap text-xl leading-7 font-medium text-black"
              >Display Live Results</span
            >
            <input
              class="h-6 w-11 cursor-pointer appearance-none rounded-full border border-black bg-white before:mx-[3px] before:my-0.5 before:block before:size-[18px] before:rounded-full before:bg-slate-900 before:content-[''] before:transition-transform before:duration-150 checked:bg-slate-900 checked:before:translate-x-[18px] checked:before:bg-white"
              type="checkbox"
              role="switch"
              bind:checked={displayLiveResults}
            />
          </label>
          <label class="flex items-center justify-between gap-4">
            <span
              class="whitespace-nowrap text-xl leading-7 font-medium text-black"
              >Allow Proxy Voting</span
            >
            <input
              class="h-6 w-11 cursor-pointer appearance-none rounded-full border border-black bg-white before:mx-[3px] before:my-0.5 before:block before:size-[18px] before:rounded-full before:bg-slate-900 before:content-[''] before:transition-transform before:duration-150 checked:bg-slate-900 checked:before:translate-x-[18px] checked:before:bg-white"
              type="checkbox"
              role="switch"
              bind:checked={allowProxyVoting}
            />
          </label>
          <label class="flex items-center justify-between gap-4">
            <span
              class="whitespace-nowrap text-xl leading-7 font-medium text-black"
              >Enable Quorum Setup</span
            >
            <input
              class="h-6 w-11 cursor-pointer appearance-none rounded-full border border-black bg-white before:mx-[3px] before:my-0.5 before:block before:size-[18px] before:rounded-full before:bg-slate-900 before:content-[''] before:transition-transform before:duration-150 checked:bg-slate-900 checked:before:translate-x-[18px] checked:before:bg-white"
              type="checkbox"
              role="switch"
              bind:checked={enableQuorum}
            />
          </label>
          {#if enableQuorum}
            <input
              class="h-10 w-[175px] rounded-[10px] border border-slate-800 bg-white px-5 text-sm text-black placeholder:text-grey-500 placeholder:italic placeholder:opacity-100"
              value={quorum}
              inputmode="numeric"
              pattern="[0-9]+"
              placeholder="ex. 20"
              aria-label="Quorum count"
              oninput={(event) =>
                (quorum = event.currentTarget.value.replace(/\D/g, ""))}
              required
            />
          {/if}
        </div>
      </div>

      {#if error}<p class="mt-[15px] text-center text-red-700" role="alert">
          {error}
        </p>{/if}
      <button
        class="mx-auto mt-12 block h-[58px] w-[280px] cursor-pointer rounded-full border-0 bg-red-600 text-lg font-medium text-white shadow-[0_4px_2px_rgb(0_0_0_/_0.25)]"
        type="submit">+ Push a {noun}</button
      >
    </form>
  </div>
</div>
