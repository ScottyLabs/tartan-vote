<script lang="ts">
	import { apiUrl } from '$lib/api/base';
	import AppFooter from '$lib/components/AppFooter.svelte';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';

	let sessionCode = $state('');

	function signOut() {
		window.location.href = apiUrl('/auth/logout');
	}

	function createSession() {
		window.location.href = '/host';
	}

	function updateSessionCode(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		sessionCode = input.value.replace(/^\s+/, '').replace(/\s+/g, '-');
	}

	function joinSession() {
		const normalizedSessionCode = sessionCode.replace(/^-+|-+$/g, '');
		if (!normalizedSessionCode) return;

		const searchParams = new URLSearchParams({ sessionCode: normalizedSessionCode });
		window.location.href = `/proxy?${searchParams.toString()}`;
	}
</script>

<svelte:head>
	<title>TartanVote | Home</title>
</svelte:head>

<main class="home-page">
	<header class="topbar">
		<a class="brand-lockup" href="/" aria-label="TartanVote home">
			<img class="brand-mark" src={logoUrl} alt="" width="33" height="25" />
			<span class="brand-wordmark"><span>Tartan</span>Vote</span>
		</a>

		<button class="signout-button" type="button" onclick={signOut}>
			<svg viewBox="0 0 16 16" aria-hidden="true">
				<path d="M6.7 3.2H4.1a1.4 1.4 0 0 0-1.4 1.4v6.8a1.4 1.4 0 0 0 1.4 1.4h2.6" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" />
				<path d="M9.4 4.9 12.5 8l-3.1 3.1M12.2 8H6.7" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
			Sign out
		</button>
	</header>

	<section class="home-content" aria-labelledby="home-title">
		<div class="intro-copy">
			<h1 id="home-title">Hello, Scottylabs</h1>
			<p>Join an existing session or host your own.</p>
		</div>

		<section class="session-card" aria-label="Session actions">
			<form class="session-form" onsubmit={(event) => { event.preventDefault(); joinSession(); }}>
				<label for="session-code">Session Code</label>
				<input
					id="session-code"
					type="text"
					value={sessionCode}
					oninput={updateSessionCode}
					placeholder="ex: happy-giraffe"
					autocomplete="off"
					autocapitalize="none"
					spellcheck="false"
				/>

				<button class="session-action primary-action join-action" type="submit" disabled={!sessionCode.trim()}>
					Join session
				</button>
			</form>

			<div class="or-divider" aria-hidden="true">
				<span></span>
				<strong>OR</strong>
				<span></span>
			</div>

			<button class="session-action mobile-unavailable" type="button" disabled>
				Host a session (not available on mobile)
			</button>

			<button class="session-action primary-action create-action" type="button" onclick={createSession}>
				Host a session
			</button>
		</section>
	</section>

	<AppFooter wide />
</main>

<style>
	.home-page {
		position: relative;
		width: min(100%, 393px);
		min-height: 100svh;
		margin: 0 auto;
		overflow: hidden;
		background: var(--gradient-screen-signin);
		color: var(--color-grey-900);
	}

	.topbar {
		height: 112px;
		padding: 39px 17px 38px 15px;
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		background: var(--color-red-600);
		box-shadow: 0 4px 4px rgb(0 0 0 / 0.25);
	}

	.brand-lockup {
		width: 150px;
		height: 27px;
		display: flex;
		align-items: center;
		gap: 3px;
		color: var(--color-red-600);
		text-decoration: none;
	}

	.brand-mark {
		width: 33px;
		height: 25px;
		flex: 0 0 auto;
		display: block;
	}

	.brand-wordmark {
		color: var(--color-red-600);
		font-family: var(--font-brand);
		font-size: 26.03px;
		font-weight: 400;
		line-height: 1;
		letter-spacing: 0;
		-webkit-text-stroke: 5px var(--color-white);
		paint-order: stroke fill;
	}

	.brand-wordmark span {
		color: var(--color-black);
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
		font-weight: 400;
		line-height: 12.75px;
		cursor: pointer;
	}

	.signout-button svg {
		width: 10px;
		height: 10px;
	}

	.home-content {
		width: 350px;
		margin: 125px auto 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 9px;
	}

	.intro-copy {
		width: 350px;
		height: 57.55px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 5px;
		text-align: center;
	}

	h1 {
		margin: 0;
		color: var(--color-black);
		font-size: 18px;
		font-weight: 600;
		line-height: 21.95px;
		letter-spacing: 0;
	}

	.intro-copy p {
		width: 345px;
		height: 30.55px;
		margin: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-grey-700);
		font-size: 13.86px;
		font-weight: 400;
		line-height: 18.48px;
	}

	.session-card {
		width: 350px;
		height: 252px;
		padding: 20px 12px;
		display: flex;
		flex-direction: column;
		align-items: center;
		background: var(--color-white);
		border: 1px solid var(--color-grey-200);
		border-radius: 4.25px;
	}

	.session-form {
		width: 326px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
	}

	label {
		width: 314px;
		color: var(--color-black);
		font-size: 16px;
		font-weight: 600;
		line-height: 22.66px;
	}

	input {
		width: 275px;
		height: 40px;
		padding: 0 14px;
		border: 1px solid var(--color-slate-400);
		border-radius: 5.96px;
		color: var(--color-grey-900);
		font: inherit;
		font-size: 14px;
		font-weight: 500;
		line-height: 19.08px;
		text-align: center;
	}

	input::placeholder {
		color: var(--color-slate-400);
		opacity: 1;
	}

	input:focus {
		border-color: var(--color-red-600);
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 78%);
	}

	.session-action {
		box-sizing: border-box;
		width: 300px;
		height: 36px;
		min-height: 36px;
		padding: 0 16px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex: 0 0 36px;
		border: 0;
		border-radius: 999px;
		box-shadow: 0 2.39px 2.39px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: 13px;
		font-weight: 500;
		line-height: 20px;
	}

	.primary-action {
		background: var(--color-red-600);
		cursor: pointer;
	}

	.primary-action:hover {
		background: var(--color-red-700);
	}

	.primary-action:disabled {
		background: var(--color-red-200);
		box-shadow: none;
		cursor: not-allowed;
	}

	.primary-action:disabled:hover {
		background: var(--color-red-200);
	}

	.or-divider {
		width: 300px;
		height: 31px;
		margin: 10px 0;
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		align-items: center;
		gap: 8px;
		color: var(--color-grey-700);
		font-size: 14.31px;
		font-weight: 300;
	}

	.or-divider span {
		height: 1px;
		background: var(--color-grey-200);
	}

	.mobile-unavailable {
		background: var(--color-red-200);
		cursor: not-allowed;
	}

	.create-action {
		display: none;
	}

	@media (min-width: 640px) {
		.home-page {
			width: 100%;
			min-height: 100svh;
			margin: 0;
			border-radius: 0;
				box-shadow: none;
		}

		:global(body) {
			background: var(--gradient-screen-signin);
		}

		.topbar {
			height: 112px;
			padding: clamp(30px, 3vw, 39px) clamp(24px, 3.6vw, 69px);
			align-items: center;
		}

		.brand-lockup {
			width: clamp(150px, 13.5vw, 260px);
			height: clamp(35px, 3.2vw, 61px);
			gap: clamp(3px, 0.42vw, 8px);
		}

		.brand-mark {
			width: clamp(33px, 4vw, 77px);
			height: clamp(25px, 3.1vw, 59px);
		}

		.brand-wordmark {
			font-size: clamp(26.03px, 2.1vw, 40px);
			-webkit-text-stroke: clamp(5px, 0.26vw, 10px) var(--color-white);
		}

		.signout-button {
			width: clamp(110px, 10.4vw, 199px);
			height: clamp(42px, 3.2vw, 61px);
			gap: clamp(4px, 0.42vw, 8px);
			background: var(--color-grey-100);
			font-size: clamp(14px, 1.25vw, 24px);
			font-weight: 400;
			line-height: clamp(20px, 1.7vw, 32px);
		}

		.signout-button svg {
			width: clamp(12px, 1.05vw, 20px);
			height: clamp(12px, 1.05vw, 20px);
		}

		.home-content {
			width: min(calc(100vw - 48px), 870px);
			margin: clamp(82px, 10vw, 108px) auto 0;
			gap: clamp(12px, 1.15vw, 22px);
		}

		.intro-copy {
			width: 100%;
			height: clamp(66px, 5.2vw, 100px);
			gap: clamp(6px, 0.52vw, 10px);
		}

		h1 {
			font-size: clamp(20px, 1.56vw, 30px);
			line-height: clamp(26px, 1.98vw, 38px);
		}

		.intro-copy p {
			width: min(100%, 843px);
			height: auto;
			padding: clamp(5px, 0.52vw, 10px) 5px;
			font-size: clamp(16px, 1.25vw, 24px);
			font-weight: 400;
			line-height: clamp(22px, 1.67vw, 32px);
		}

		.session-card {
			width: min(calc(100vw - 48px), 700px);
			height: clamp(310px, 26.6vw, 510px);
			padding: clamp(28px, 2.5vw, 48px) clamp(24px, 3.9vw, 75px);
		}

		.session-form {
			width: min(100%, 550px);
			gap: clamp(18px, 1.46vw, 28px);
		}

		label {
			width: auto;
			font-size: clamp(20px, 1.56vw, 30px);
			line-height: clamp(26px, 1.98vw, 38px);
			text-align: center;
		}

		input {
			width: min(100%, 490px);
			height: clamp(46px, 3.65vw, 70px);
			border-radius: clamp(5.96px, 0.52vw, 10px);
			font-size: clamp(16px, 1.25vw, 24px);
			line-height: clamp(22px, 1.67vw, 32px);
		}

		.session-action {
			width: min(100%, 500px);
			height: clamp(42px, 3.13vw, 60px);
			min-height: clamp(42px, 3.13vw, 60px);
			font-size: clamp(15px, 1.04vw, 20px);
			font-weight: 500;
			line-height: clamp(21px, 1.46vw, 28px);
			box-shadow: 0 clamp(2.39px, 0.21vw, 4px) 2px rgb(0 0 0 / 0.25);
		}

		.or-divider {
			width: min(100%, 550px);
			height: clamp(32px, 2.08vw, 40px);
			margin: clamp(12px, 1.04vw, 20px) 0 clamp(14px, 1.15vw, 22px);
			gap: clamp(8px, 0.52vw, 10px);
			font-size: clamp(16px, 1.25vw, 24px);
			font-weight: 500;
			line-height: clamp(22px, 1.67vw, 32px);
		}

		.or-divider strong {
			padding: 0 clamp(8px, 0.52vw, 10px);
			background: var(--color-white);
		}

		.mobile-unavailable {
			display: none;
		}

		.create-action {
			display: inline-flex;
		}
	}

	@media (max-width: 374px) {
		.home-content,
		.intro-copy,
		.session-card {
			width: calc(100% - 28px);
		}

		.intro-copy p,
		label,
		.session-form,
		input,
		.primary-action,
		.mobile-unavailable,
		.create-action,
		.or-divider {
			width: 100%;
		}
	}
</style>
