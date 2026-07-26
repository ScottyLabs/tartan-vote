<script lang="ts">
	import { page } from '$app/state';
	import { apiUrl } from '$lib/api/base';
	import AppFooter from '$lib/components/AppFooter.svelte';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';

	let proxyFor = $state(page.url.searchParams.get('proxyVotes') ?? '');
	const sessionCode = $derived(page.url.searchParams.get('sessionCode')?.trim() || 'Demo session');

	const hasProxyInput = $derived(proxyFor.trim().length > 0);

	function signOut() {
		window.location.href = apiUrl('/auth/logout');
	}

	function goBack() {
		window.location.href = '/home';
	}

	function submitProxy() {
		if (!hasProxyInput) return;
		// This will POST to /session/{sessionCode}/proxy once backend wiring is ready.
		// For now, show the pending proxy review waiting state.
		const searchParams = new URLSearchParams({
			sessionCode,
			proxyStatus: 'pending',
			proxyVotes: proxyFor.trim()
		});

		window.location.href = `/waiting?${searchParams.toString()}`;
	}

	function continueWithoutProxy() {
		if (hasProxyInput) return;
		const searchParams = new URLSearchParams({ sessionCode });
		window.location.href = `/waiting?${searchParams.toString()}`;
	}
</script>

<svelte:head>
	<title>TartanVote | Proxy Votes</title>
</svelte:head>

<main class="proxy-page">
	<header class="topbar">
		<a class="brand-lockup" href="/home" aria-label="TartanVote home">
			<img class="brand-mark" src={logoUrl} alt="" width="33" height="25" />
			<span class="brand-wordmark"><span>Tartan</span>Vote</span>
		</a>

		<p class="desktop-session-code">Session Code: {sessionCode}</p>

		<button class="signout-button" type="button" onclick={signOut}>
			<svg viewBox="0 0 16 16" aria-hidden="true">
				<path d="M6.7 3.2H4.1a1.4 1.4 0 0 0-1.4 1.4v6.8a1.4 1.4 0 0 0 1.4 1.4h2.6" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" />
				<path d="M9.4 4.9 12.5 8l-3.1 3.1M12.2 8H6.7" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
			Sign out
		</button>
	</header>

	<div class="mobile-session-row">
		<button class="back-button" type="button" aria-label="Back to home" onclick={goBack}>
			<svg viewBox="0 0 20 20" aria-hidden="true">
				<path d="M12.5 4.5 7 10l5.5 5.5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
		</button>

		<p>Session Code: {sessionCode}</p>
	</div>

	<section class="proxy-content" aria-labelledby="proxy-title">
		<div class="intro-copy">
			<h1 id="proxy-title">Proxy Votes<span class="desktop-title-space"> </span><span>(Optional)</span></h1>
			<p>If you are voting on behalf of someone, please enter their AndrewID(s) separated by a comma. Otherwise, continue.</p>
		</div>

		<section class="proxy-card" aria-label="Proxy vote information">
			<label for="proxy-for">I am proxying for:</label>
			<input
				id="proxy-for"
				type="text"
				bind:value={proxyFor}
				placeholder="ex: scottylabs0, scottylabs1, scottylabs123"
				autocomplete="off"
			/>
		</section>

		<div class="action-row">
			<button class="proxy-button submit-button" type="button" disabled={!hasProxyInput} onclick={submitProxy}>Submit proxy</button>
			<button class="proxy-button continue-without-button" type="button" disabled={hasProxyInput} onclick={continueWithoutProxy}>Continue without proxy</button>
		</div>
	</section>

	<AppFooter wide />
</main>

<style>
	.proxy-page {
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

	.desktop-session-code {
		display: none;
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
		line-height: 12.75px;
		cursor: pointer;
	}

	.signout-button svg {
		width: 10px;
		height: 10px;
	}

	.mobile-session-row {
		width: 372px;
		height: 20px;
		margin: 16px 0 0 21px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.back-button {
		width: 20px;
		height: 20px;
		border: 0;
		border-radius: 999px;
		padding: 0;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: var(--color-red-500);
		color: var(--color-grey-50);
		cursor: pointer;
	}

	.back-button svg {
		width: 16px;
		height: 16px;
	}

	.mobile-session-row p {
		width: 194px;
		margin: 0;
		padding-right: 15px;
		color: var(--color-slate-400);
		font-size: 12px;
		font-weight: 500;
		line-height: 20px;
		text-align: right;
	}

	.proxy-content {
		width: 350px;
		margin: 77px auto 0;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.intro-copy {
		width: 345px;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	h1 {
		margin: 0;
		color: var(--color-black);
		font-size: 18px;
		font-weight: 600;
		line-height: 21.95px;
		text-align: center;
	}

	.intro-copy p {
		margin: 0;
		padding: 6px 3px;
		color: var(--color-grey-700);
		font-size: 13px;
		font-weight: 500;
		line-height: 18.48px;
	}

	.proxy-card {
		width: 350px;
		height: 120px;
		margin-top: 12px;
		padding: 12px 20px;
		display: flex;
		flex-direction: column;
		gap: 11px;
		background: var(--color-white);
		border: 0.58px solid var(--color-grey-200);
		border-radius: 5.77px;
	}

	label {
		color: var(--color-slate-800);
		font-size: 14px;
		font-weight: 500;
		line-height: 18.48px;
	}

	input {
		width: 300px;
		height: 40px;
		border: 0.58px solid var(--color-slate-400);
		border-radius: 5.77px;
		padding: 2px 10px;
		color: var(--color-grey-900);
		font: inherit;
		font-size: 13px;
		font-weight: 500;
		line-height: 18px;
	}

	input::placeholder {
		color: var(--color-slate-400);
		opacity: 1;
	}

	input:focus {
		border-color: var(--color-red-600);
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 78%);
	}

	.action-row {
		width: 350px;
		height: 48px;
		margin-top: 20px;
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.proxy-button {
		flex: 1 1 0;
		min-width: 0;
		height: 36px;
		border: 0;
		border-radius: 999px;
		box-shadow: 0 2.31px 2.31px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: 12px;
		font-weight: 600;
		line-height: 18.48px;
		cursor: pointer;
	}

	.proxy-button:disabled {
		background: var(--color-red-200);
		cursor: not-allowed;
	}

	.submit-button {
		background: var(--color-red-600);
	}

	.continue-without-button {
		background: var(--color-red-600);
	}

	.proxy-button:hover {
		filter: brightness(0.96);
	}

	.proxy-button:disabled:hover {
		filter: none;
	}

	button:focus-visible,
	a:focus-visible {
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 75%);
		outline-offset: 3px;
	}

	@media (min-width: 640px) {
		.proxy-page {
			width: 100%;
			min-height: 100svh;
			margin: 0;
		}

		.topbar {
			height: clamp(96px, 5.83vw, 112px);
			padding: clamp(24px, 1.3vw, 25px) clamp(28px, 3.6vw, 69px);
			align-items: center;
			justify-content: flex-start;
			gap: clamp(60px, 8vw, 180px);
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
			-webkit-text-stroke: clamp(3px, 0.52vw, 10px) var(--color-white);
		}

		.desktop-session-code {
			display: block;
			width: clamp(250px, 17.08vw, 328px);
			margin: 0 0 0 auto;
			color: var(--color-grey-50);
			font-size: clamp(18px, 1.25vw, 24px);
			font-weight: 500;
			line-height: clamp(26px, 1.67vw, 32px);
			text-align: center;
		}

		.signout-button {
			width: clamp(150px, 10.36vw, 199px);
			height: clamp(50px, 3.18vw, 61px);
			gap: clamp(6px, 0.42vw, 8px);
			font-size: clamp(18px, 1.25vw, 24px);
			font-weight: 700;
			line-height: clamp(26px, 1.67vw, 32px);
		}

		.signout-button svg {
			width: clamp(16px, 1.04vw, 20px);
			height: clamp(16px, 1.04vw, 20px);
		}

		.mobile-session-row {
			display: none;
		}

		.proxy-content {
			width: min(calc(100vw - 80px), 952px);
			margin-top: clamp(145px, 11.9vw, 228px);
		}

		.intro-copy {
			width: min(100%, 870px);
			gap: clamp(10px, 0.52vw, 10px);
		}

		h1 {
			font-size: clamp(22px, 1.56vw, 30px);
			line-height: clamp(30px, 1.98vw, 38px);
		}

		.intro-copy p {
			padding: clamp(8px, 0.52vw, 10px) 5px;
			font-size: clamp(18px, 1.25vw, 24px);
			line-height: clamp(26px, 1.67vw, 32px);
			text-align: center;
		}

		.proxy-card {
			width: min(100%, 869px);
			height: clamp(140px, 8.75vw, 168px);
			margin-top: clamp(10px, 0.52vw, 10px);
			padding: clamp(24px, 1.56vw, 30px) clamp(36px, 3.65vw, 70px);
			gap: clamp(14px, 1.04vw, 20px);
			border: 1px solid var(--color-grey-200);
			border-radius: 10px;
		}

		label {
			font-size: clamp(18px, 1.25vw, 24px);
			font-weight: 700;
			line-height: clamp(26px, 1.67vw, 32px);
		}

		input {
			width: 100%;
			height: clamp(44px, 2.6vw, 50px);
			border: 1px solid var(--color-slate-400);
			border-radius: 10px;
			padding: 4px 16px;
			font-size: clamp(18px, 1.25vw, 24px);
			line-height: clamp(26px, 1.67vw, 32px);
		}

		.action-row {
			width: min(100%, 590px);
			height: 80px;
			margin-top: clamp(0px, 0.52vw, 10px);
			justify-content: center;
			gap: clamp(16px, 1.25vw, 24px);
		}

		.proxy-button {
			width: clamp(240px, 14.58vw, 280px);
			flex: 0 0 clamp(240px, 14.58vw, 280px);
			height: clamp(52px, 3.13vw, 60px);
			font-size: clamp(20px, 1.25vw, 24px);
			font-weight: 700;
			line-height: clamp(28px, 1.67vw, 32px);
			box-shadow: 0 4px 4px rgb(0 0 0 / 0.25);
		}
	}

	@media (max-width: 374px) {
		.mobile-session-row,
		.proxy-content,
		.intro-copy,
		.proxy-card,
		.action-row {
			width: calc(100% - 28px);
		}

		input {
			width: 100%;
		}

		.action-row {
			gap: 8px;
		}

		.proxy-button {
			flex-basis: 0;
		}
	}
</style>
