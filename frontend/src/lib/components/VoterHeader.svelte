<script lang="ts">
  import logoUrl from '$lib/assets/tartanvote-logo.svg?url';
  import signoutUrl from '$lib/assets/signout.svg?url';

  interface Props {
    sessionCode: string;
    onSignOut: () => void;
  }

  let { sessionCode, onSignOut }: Props = $props();
</script>

<header class="topbar">
  <a class="brand-lockup" href="/home" aria-label="TartanVote home">
    <img class="brand-mark" src={logoUrl} alt="" width="77" height="59" />
    <span class="brand-wordmark"><span>Tartan</span>Vote</span>
  </a>

  <p class="session-code">Session Code: {sessionCode}</p>

  <button class="signout-button" type="button" onclick={onSignOut}>
    <img src={signoutUrl} alt="" width="20" height="20" />
    <span>Sign out</span>
  </button>
</header>

<style>
  .topbar {
    height: 112px;
    padding: 39px 17px 38px 15px;
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
    background: var(--color-red-600);
    box-shadow: 0 4px 4px rgb(0 0 0 / 0.25);
  }

  .brand-lockup {
    width: 150px;
    height: 27px;
    display: flex;
    align-items: center;
    gap: 3px;
    text-decoration: none;
  }

  .brand-mark {
    width: 33px;
    height: 25px;
    display: block;
    flex: 0 0 auto;
  }

  .brand-wordmark {
    color: var(--color-red-600);
    font-family: var(--font-brand);
    font-size: 26px;
    line-height: 1;
    -webkit-text-stroke: 5px var(--color-white);
    paint-order: stroke fill;
  }

  .brand-wordmark span {
    color: var(--color-black);
  }

  .session-code {
    position: absolute;
    top: 128px;
    right: 15px;
    margin: 0;
    color: var(--color-slate-400);
    font-size: 12px;
    font-weight: 500;
    line-height: 20px;
  }

  .signout-button {
    width: 80px;
    height: 35px;
    border: 0;
    border-radius: 999px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: var(--color-grey-100);
    color: var(--color-slate-800);
    font: inherit;
    font-size: 10px;
    font-weight: 500;
    cursor: pointer;
  }

  .signout-button img {
    width: 10px;
    height: 10px;
  }

  @media (min-width: 640px) {
    .topbar {
      height: clamp(96px, 5.83vw, 112px);
      padding: clamp(24px, 1.3vw, 25px) clamp(28px, 3.6vw, 69px);
      align-items: center;
    }

    .brand-lockup {
      width: clamp(210px, 13.54vw, 260px);
      height: clamp(52px, 3.07vw, 59px);
      gap: clamp(6px, 0.42vw, 8px);
    }

    .brand-mark {
      width: clamp(68px, 4.01vw, 77px);
      height: clamp(52px, 3.07vw, 59px);
    }

    .brand-wordmark {
      font-size: clamp(36px, 2.08vw, 40px);
      -webkit-text-stroke: clamp(7px, 0.52vw, 10px) var(--color-white);
    }

    .session-code {
      position: static;
      margin: 0 0 0 auto;
      padding-right: clamp(20px, 1.56vw, 30px);
      color: var(--color-grey-50);
      font-size: clamp(18px, 1.25vw, 24px);
      line-height: clamp(26px, 1.67vw, 32px);
      text-align: center;
    }

    .signout-button {
      width: clamp(150px, 10.36vw, 199px);
      height: clamp(50px, 3.18vw, 61px);
      gap: clamp(6px, 0.42vw, 8px);
      font-size: clamp(18px, 1.25vw, 24px);
      font-weight: 700;
    }

    .signout-button img {
      width: clamp(16px, 1.04vw, 20px);
      height: clamp(16px, 1.04vw, 20px);
    }
  }
</style>
