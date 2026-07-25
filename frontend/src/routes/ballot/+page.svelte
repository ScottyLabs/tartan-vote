<script lang="ts">
  import { page } from '$app/state';
  import { apiUrl } from '$lib/api/base';
  import AppFooter from '$lib/components/AppFooter.svelte';
  import VoterHeader from '$lib/components/VoterHeader.svelte';
  import VotingInstanceFieldset from '$lib/components/VotingInstanceFieldset.svelte';
  import { ballotChoices, votingInstances } from '$lib/domain/ballot';

  const sessionCode = page.url.searchParams.get('sessionCode')?.trim() || 'happy-giraffe';
  const hasProxy = page.url.searchParams.get('proxy') !== 'false';
  const confirmationKind = page.url.searchParams.get('confirmation') === 'secret' ? 'secret' : 'live';
  const resultsVariant = page.url.searchParams.get('results') === 'rollcall' ? 'rollcall-live' : 'live';
  const instances = hasProxy ? votingInstances : votingInstances.slice(0, 1);

  let selections = $state<Record<string, string>>({});
  let submitted = $state(false);
  let confirmationOpen = $state(false);

  const allSelected = $derived(instances.every((instance) => Boolean(selections[instance.id])));
  const voteNoun = $derived(hasProxy ? 'Votes' : 'Vote');

  function signOut() {
    window.location.href = apiUrl('/auth/logout');
  }

  function updateSelection(instanceId: string, choiceId: string) {
    if (submitted) return;
    selections[instanceId] = choiceId;
  }

  function submitBallot(event: SubmitEvent) {
    event.preventDefault();
    if (!allSelected || submitted) return;
    submitted = true;
    confirmationOpen = true;
  }

  function closeConfirmation() {
    confirmationOpen = false;
  }

  function handleConfirmationAction() {
    if (confirmationKind === 'secret') {
      closeConfirmation();
      return;
    }

    window.location.href = `/results?variant=${resultsVariant}`;
  }
</script>

<svelte:head>
  <title>TartanVote | Ballot</title>
  <meta name="description" content="Submit a vote in the active TartanVote election." />
</svelte:head>

<main class="ballot-page">
  <VoterHeader {sessionCode} onSignOut={signOut} />

  <section class="ballot-content" aria-labelledby="ballot-title">
    <header class:with-proxy={hasProxy} class="election-card">
      <h1 id="ballot-title">Election: Best Rust Stuco Instructor</h1>
      <p>Voting Type: Standard</p>
      {#if hasProxy}
        <p class="instructions">Choose one response for each ballot assigned to you. Your proxy ballot is recorded separately from your own vote.</p>
      {/if}
    </header>

    <form onsubmit={submitBallot}>
      <div class="instance-list">
        {#each instances as instance (instance.id)}
          <VotingInstanceFieldset
            {instance}
            choices={ballotChoices}
            selectedChoiceId={selections[instance.id]}
            disabled={submitted}
            compact={hasProxy}
            onChange={updateSelection}
          />
        {/each}
      </div>

      <button
        class:submitted
        class="submit-button"
        type="submit"
        disabled={!allSelected || submitted}
      >
        {submitted ? 'Submitted' : `Submit ${voteNoun}`}
      </button>
    </form>
  </section>

  <AppFooter wide />

  {#if confirmationOpen}
    <div class="confirmation-backdrop" role="presentation">
      <div
        class="confirmation-card"
        role="dialog"
        aria-modal="true"
        aria-labelledby="confirmation-title"
      >
        <div>
          <h2 id="confirmation-title">{voteNoun} Submitted</h2>
          {#if confirmationKind === 'secret'}
            <p>You have successfully voted. Please wait for your host to push out results.</p>
          {:else}
            <p>You have successfully voted. Click below to view live results.</p>
          {/if}
        </div>

        <button type="button" onclick={handleConfirmationAction}>
          {confirmationKind === 'secret' ? 'OK' : 'View Live Results'}
        </button>
      </div>
    </div>
  {/if}
</main>

<style>
  .ballot-page {
    position: relative;
    width: min(100%, 393px);
    min-height: 100svh;
    margin: 0 auto;
    padding-bottom: 81px;
    overflow-x: hidden;
    background: var(--gradient-screen-signin);
    color: var(--color-grey-900);
  }

  .ballot-content {
    width: calc(100% - 42px);
    max-width: 350px;
    margin: 68px auto 0;
    padding-bottom: 48px;
  }

  .election-card,
  :global(.ballot-content fieldset) {
    border: 1px solid var(--color-grey-400);
    border-radius: 10px;
    background: var(--color-white);
  }

  .election-card {
    min-height: 112px;
    padding: 20px;
  }

  .election-card h1 {
    margin: 0;
    font-size: 16px;
    font-weight: 500;
    line-height: 22px;
  }

  .election-card p {
    margin: 8px 0 0;
    font-size: 13px;
    font-weight: 500;
    line-height: 18px;
  }

  .election-card .instructions {
    color: var(--color-slate-600);
    font-size: 12px;
    line-height: 17px;
  }

  form {
    margin-top: 15px;
  }

  .instance-list {
    display: flex;
    flex-direction: column;
    gap: 15px;
  }

  .submit-button {
    width: 174px;
    height: 42px;
    margin: 18px auto 0;
    border: 0;
    border-radius: 999px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-red-600);
    box-shadow: 0 3px 2px rgb(0 0 0 / 0.25);
    color: var(--color-white);
    font: inherit;
    font-size: 15px;
    font-weight: 700;
    cursor: pointer;
  }

  .submit-button:disabled:not(.submitted) {
    opacity: 0.65;
    cursor: not-allowed;
  }

  .submit-button.submitted {
    background: var(--color-red-200);
    box-shadow: 0 2px 2px rgb(0 0 0 / 0.18);
    cursor: default;
  }

  .confirmation-backdrop {
    position: fixed;
    z-index: 20;
    inset: 0;
    padding: 20px;
    display: grid;
    place-items: center;
    background: rgb(30 30 30 / 0.28);
  }

  .confirmation-card {
    width: min(100%, 350px);
    min-height: 220px;
    padding: 34px 20px 28px;
    border: 1px solid var(--color-grey-600);
    border-radius: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    gap: 30px;
    background: var(--color-white);
    text-align: center;
  }

  .confirmation-card h2 {
    margin: 0;
    color: var(--color-black);
    font-size: 26px;
    font-weight: 600;
    line-height: 34px;
  }

  .confirmation-card p {
    margin: 8px 0 0;
    color: var(--color-grey-900);
    font-size: 14px;
    font-weight: 500;
    line-height: 20px;
  }

  .confirmation-card button {
    width: min(100%, 240px);
    height: 48px;
    border: 0;
    border-radius: 999px;
    background: var(--color-red-500);
    box-shadow: 0 3px 2px rgb(0 0 0 / 0.25);
    color: var(--color-white);
    font: inherit;
    font-size: 16px;
    font-weight: 700;
    cursor: pointer;
  }

  button:focus-visible {
    outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 70%);
    outline-offset: 3px;
  }

  @media (min-width: 640px) {
    .ballot-page {
      width: 100%;
      min-height: 100svh;
      margin: 0;
    }

    .ballot-content {
      width: min(calc(100vw - 80px), 800px);
      max-width: none;
      margin-top: clamp(86px, 7.55vw, 145px);
      padding-bottom: clamp(70px, 5.2vw, 100px);
    }

    .election-card {
      min-height: clamp(110px, 6.25vw, 120px);
      padding: clamp(22px, 1.56vw, 30px) clamp(32px, 2.08vw, 40px);
    }

    .election-card.with-proxy {
      min-height: clamp(160px, 9.84vw, 189px);
    }

    .election-card h1 {
      font-size: clamp(20px, 1.25vw, 24px);
      line-height: clamp(28px, 1.67vw, 32px);
    }

    .election-card p {
      margin-top: 5px;
      font-size: clamp(16px, 0.94vw, 18px);
      line-height: clamp(24px, 1.35vw, 26px);
    }

    .election-card .instructions {
      margin-top: 12px;
      color: var(--color-slate-500);
      font-size: clamp(16px, 0.94vw, 18px);
      line-height: clamp(24px, 1.35vw, 26px);
    }

    form {
      margin-top: 15px;
    }

    .submit-button {
      width: clamp(280px, 18.59vw, 357px);
      height: clamp(52px, 3.13vw, 60px);
      margin-top: clamp(18px, 1.56vw, 30px);
      font-size: clamp(20px, 1.25vw, 24px);
      line-height: 32px;
    }

    .confirmation-card {
      width: min(calc(100vw - 80px), 1200px);
      min-height: clamp(260px, 15.63vw, 300px);
      padding: clamp(42px, 2.6vw, 50px) 20px clamp(28px, 1.56vw, 30px);
      gap: clamp(42px, 3.13vw, 60px);
    }

    .confirmation-card h2 {
      font-size: clamp(36px, 2.34vw, 45px);
      line-height: clamp(46px, 2.92vw, 56px);
    }

    .confirmation-card p {
      margin-top: 10px;
      font-size: clamp(18px, 1.04vw, 20px);
      line-height: clamp(26px, 1.46vw, 28px);
    }

    .confirmation-card button {
      width: clamp(260px, 16.41vw, 315px);
      height: clamp(54px, 3.28vw, 63px);
      font-size: clamp(20px, 1.25vw, 24px);
      line-height: 32px;
    }
  }
</style>
