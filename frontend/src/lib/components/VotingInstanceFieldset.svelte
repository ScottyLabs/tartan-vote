<script lang="ts">
  import type { BallotChoice, VotingInstance } from '$lib/domain/ballot';
  import radioUrl from '$lib/assets/ballot-radio.svg?url';
  import selectedRadioUrl from '$lib/assets/ballot-radio-selected.svg?url';

  interface Props {
    instance: VotingInstance;
    choices: BallotChoice[];
    selectedChoiceId?: string;
    disabled?: boolean;
    compact?: boolean;
    onChange: (instanceId: string, choiceId: string) => void;
  }

  let {
    instance,
    choices,
    selectedChoiceId,
    disabled = false,
    compact = false,
    onChange
  }: Props = $props();

  const groupName = $derived(`vote-${instance.id}`);
</script>

<fieldset class:compact {disabled}>
  <legend>Ballot for {instance.votingId}{instance.proxy ? ', proxy vote' : ''}</legend>

  <div class="instance-header">
    <span>VotingID: {instance.votingId}</span>
    {#if instance.proxy}
      <span class="proxy-label">Proxy Vote</span>
    {/if}
  </div>

  <div class="choice-list">
    {#each choices as choice (choice.id)}
      <label class:selected={selectedChoiceId === choice.id}>
        <input
          type="radio"
          name={groupName}
          value={choice.id}
          checked={selectedChoiceId === choice.id}
          onchange={() => onChange(instance.id, choice.id)}
        />
        <img
          class="radio-mark"
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

<style>
  fieldset {
    min-width: 0;
    min-height: 220px;
    margin: 0;
    padding: 22px 20px 24px;
    border: 1px solid var(--color-grey-400);
    border-radius: 10px;
    background: var(--color-white);
  }

  legend {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0 0 0 0);
    white-space: nowrap;
    border: 0;
  }

  .instance-header {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    color: var(--color-grey-900);
    font-size: 14px;
    font-weight: 500;
    line-height: 20px;
  }

  .proxy-label {
    color: var(--color-grey-700);
  }

  .choice-list {
    width: min(100%, 300px);
    margin: 22px auto 0;
    display: flex;
    flex-direction: column;
    gap: 7px;
  }

  label {
    position: relative;
    min-height: 42px;
    padding: 9px 14px;
    border: 1px solid var(--color-slate-900);
    border-radius: 8px;
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--color-white);
    color: var(--color-slate-900);
    font-size: 14px;
    font-weight: 500;
    line-height: 20px;
    cursor: pointer;
  }

  label:hover {
    border-color: var(--color-red-400);
  }

  label.selected {
    border: 2px solid var(--color-red-400);
    padding: 8px 13px;
    background: var(--color-red-50);
  }

  input {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    overflow: hidden;
    clip: rect(0 0 0 0);
  }

  .radio-mark {
    width: 14px;
    height: 14px;
    display: block;
    flex: 0 0 auto;
  }

  input:focus-visible + .radio-mark {
    outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 72%);
    outline-offset: 3px;
  }

  @media (min-width: 640px) {
    fieldset {
      min-height: clamp(240px, 15.63vw, 300px);
      padding: clamp(28px, 1.88vw, 36px) clamp(32px, 2.24vw, 43px);
    }

    fieldset.compact {
      min-height: clamp(230px, 13.54vw, 260px);
      padding-top: clamp(24px, 1.56vw, 30px);
      padding-bottom: clamp(24px, 1.56vw, 30px);
    }

    .instance-header {
      font-size: clamp(16px, 0.94vw, 18px);
      line-height: clamp(24px, 1.35vw, 26px);
    }

    .choice-list {
      width: min(100%, 431px);
      margin-top: clamp(24px, 1.41vw, 27px);
    }

    label {
      min-height: 50px;
      padding: 11px 14px;
      border-radius: 10px;
      font-size: 18px;
      line-height: 26px;
    }

    label.selected {
      padding: 10px 13px;
    }

    .radio-mark {
      width: 18px;
      height: 18px;
    }

    label.selected .radio-mark::after {
      width: 7px;
      height: 7px;
    }
  }
</style>
