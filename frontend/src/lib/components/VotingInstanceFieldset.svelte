<script lang="ts">
  import type { BallotChoice, VotingInstance } from "$lib/domain/ballot";
  import radioUrl from "$lib/assets/ballot-radio.svg?url";
  import selectedRadioUrl from "$lib/assets/ballot-radio-selected.svg?url";

  interface Props {
    instance: VotingInstance;
    choices: BallotChoice[];
    selectedChoiceId?: string;
    disabled?: boolean;
    compact?: boolean;
    quickVote?: boolean;
    onChange: (instanceId: string, choiceId: string) => void;
  }

  let {
    instance,
    choices,
    selectedChoiceId,
    disabled = false,
    compact = false,
    quickVote = false,
    onChange,
  }: Props = $props();

  const groupName = $derived(`vote-${instance.id}`);
</script>

<fieldset
  class={[
    "m-0 min-h-[220px] min-w-0 rounded-[10px] border border-grey-400 bg-white px-5 pt-[22px] pb-6 sm:px-[clamp(32px,2.24vw,43px)]",
    quickVote
      ? "sm:min-h-[clamp(190px,10.938vw,210px)] sm:py-[clamp(20px,1.563vw,30px)]"
      : compact
        ? "sm:min-h-[clamp(230px,13.54vw,260px)] sm:py-[clamp(24px,1.56vw,30px)]"
        : "sm:min-h-[clamp(240px,15.63vw,300px)] sm:py-[clamp(28px,1.88vw,36px)]",
  ]}
  {disabled}
>
  <legend class="sr-only">
    Ballot for {instance.votingId}{instance.proxy ? ", proxy vote" : ""}
  </legend>

  <div
    class="flex w-full items-center justify-between gap-4 text-sm leading-5 font-medium text-grey-900 sm:text-[clamp(16px,0.94vw,18px)] sm:leading-[clamp(24px,1.35vw,26px)]"
  >
    <span>VotingID: {instance.votingId}</span>
    {#if instance.proxy}
      <span class="text-grey-700">Proxy Vote</span>
    {/if}
  </div>

  <div
    class="mx-auto mt-[22px] flex w-[min(100%,300px)] flex-col gap-[7px] sm:mt-[clamp(24px,1.41vw,27px)] sm:w-[min(100%,431px)]"
  >
    {#each choices as choice (choice.id)}
      <label
        class={[
          "relative flex min-h-[42px] cursor-pointer items-center gap-2 rounded-lg text-sm leading-5 font-medium text-slate-900 hover:border-red-400 sm:min-h-[50px] sm:rounded-[10px] sm:text-lg sm:leading-[26px]",
          selectedChoiceId === choice.id
            ? "border-2 border-red-400 bg-red-50 px-[13px] py-2 sm:py-2.5"
            : "border border-slate-900 bg-white px-3.5 py-[9px] sm:py-[11px]",
        ]}
      >
        <input
          class="peer sr-only"
          type="radio"
          name={groupName}
          value={choice.id}
          checked={selectedChoiceId === choice.id}
          onchange={() => onChange(instance.id, choice.id)}
        />
        <img
          class="block size-3.5 flex-none peer-focus-visible:[outline:3px_solid_color-mix(in_srgb,var(--color-red-600),transparent_72%)] peer-focus-visible:outline-offset-[3px] sm:size-[18px]"
          src={selectedChoiceId === choice.id ? selectedRadioUrl : radioUrl}
          alt=""
          width="20"
          height="20"
        />
        <span>{choice.label}</span>
      </label>
    {/each}
  </div>
</fieldset>
