<script lang="ts">
	import { page } from '$app/state';
	import { apiUrl } from '$lib/api/base';
	import AppFooter from '$lib/components/AppFooter.svelte';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';

	const sessionCode = $derived(page.url.searchParams.get('sessionCode')?.trim() || 'Demo session');
	const hasPendingProxyReview = $derived(page.url.searchParams.get('proxyStatus') === 'pending');
	const pendingProxyVotes = $derived(page.url.searchParams.get('proxyVotes')?.trim() ?? '');
	const pendingProxyVoteList = $derived(
		pendingProxyVotes
			.split(',')
			.map((proxyVote) => proxyVote.trim())
			.filter(Boolean)
	);
	const pendingProxyVoteText = $derived(pendingProxyVoteList.join(', '));
	const pendingProxyVoteCount = $derived(pendingProxyVoteList.length);

	function signOut() {
		window.location.href = apiUrl('/auth/logout');
	}

	function goBack() {
		const searchParams = new URLSearchParams({ sessionCode });
		if (pendingProxyVotes) searchParams.set('proxyVotes', pendingProxyVotes);
		window.location.href = `/proxy?${searchParams.toString()}`;
	}
</script>

<svelte:head>
	<title>TartanVote | Waiting Room</title>
</svelte:head>

<main class="waiting-page">
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

	<button class="desktop-back-button back-button" type="button" aria-label="Back to proxy setup" onclick={goBack}>
		<svg viewBox="0 0 20 20" aria-hidden="true">
			<path d="M12.5 4.5 7 10l5.5 5.5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
		</svg>
	</button>

	<div class="mobile-session-row">
		<button class="back-button" type="button" aria-label="Back to proxy setup" onclick={goBack}>
			<svg viewBox="0 0 20 20" aria-hidden="true">
				<path d="M12.5 4.5 7 10l5.5 5.5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
			</svg>
		</button>

		<p>Session Code: {sessionCode}</p>
	</div>

	<section class="waiting-content" aria-labelledby="waiting-title">
		<div class="waiting-copy">
			<h1 id="waiting-title">You are in the Waiting Room</h1>
			<p class="proxy-request">
				Proxy Request:
				{#if hasPendingProxyReview}
					<strong>Not Yet Requested</strong>
				{:else}
					N/A
				{/if}
			</p>
			{#if hasPendingProxyReview}
				<p class="description">Your host will push a motion or election shortly. Please ensure that all inputted Proxy IDs are accurate before requesting the host’s confirmation.</p>
			{:else}
				<p class="desktop-description">Your host will push a motion or election shortly. You are continuing without proxy votes.</p>
				<p class="mobile-description">Your host will push a motion or election shortly. You are continuing without proxy votes.</p>
			{/if}
		</div>

		<section class="user-info" aria-label="Current voter information">
			<p><strong>Your AndrewID:</strong> Available after backend connection</p>
			{#if hasPendingProxyReview}
				<p class="desktop-proxy-votes"><strong>Your Proxy Votes({pendingProxyVoteCount}):</strong> <em>{pendingProxyVoteText}</em></p>
				<p class="mobile-proxy-votes"><strong>Your Pending Proxy Votes ({pendingProxyVoteCount}):</strong> <em>{pendingProxyVoteText}</em></p>
			{/if}
		</section>

		{#if hasPendingProxyReview}
			<button class="request-review-button" type="button" disabled title="Available after backend connection">Proxy Review-not yet available</button>
		{/if}
	</section>

	<AppFooter wide />
</main>

<style>
	.waiting-page {
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

	.desktop-back-button {
		display: none;
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

	.waiting-content {
		width: 350px;
		margin: 62px auto 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 20px;
	}

	.waiting-copy {
		width: 345px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
		text-align: center;
	}

	.proxy-request,
	.description,
	.desktop-description,
	.mobile-description {
		margin: 0;
		color: var(--color-grey-700);
		font-size: 13px;
		font-weight: 500;
		line-height: 20px;
	}

	.proxy-request {
		line-height: 14px;
	}

	.proxy-request strong {
		color: var(--color-red-500);
		font-weight: 700;
	}

	.desktop-description {
		display: none;
	}

	.user-info {
		width: 350px;
		height: 120px;
		padding: 30px 50px;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: var(--color-white);
		border: 1px solid var(--color-grey-200);
		border-radius: 10px;
		text-align: center;
	}

	h1 {
		margin: 0;
		color: var(--color-black);
		font-size: 18px;
		font-weight: 600;
		line-height: 21.95px;
	}

	.user-info p {
		width: 325px;
		margin: 0;
		color: var(--color-slate-800);
		font-size: 14px;
		font-weight: 500;
		line-height: 20px;
	}

	.user-info p + p {
		margin-top: 20px;
	}

	.user-info strong {
		font-weight: 700;
	}

	.mobile-proxy-votes em,
	.desktop-proxy-votes em {
		color: var(--color-slate-300);
		font-style: italic;
	}

	.desktop-proxy-votes {
		display: none;
	}

	.request-review-button {
		width: 174px;
		height: 36px;
		border: 0;
		border-radius: 50px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: var(--color-red-500);
		box-shadow: 0 2.31px 1.16px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: 14px;
		font-weight: 600;
		line-height: 18.48px;
		cursor: pointer;
	}

	.request-review-button:hover {
		background: var(--color-red-600);
	}

	.request-review-button:disabled {
		background: var(--color-red-200);
		box-shadow: none;
		cursor: not-allowed;
	}

	.request-review-button:disabled:hover {
		background: var(--color-red-200);
	}

	button:focus-visible,
	a:focus-visible {
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 75%);
		outline-offset: 3px;
	}

	@media (min-width: 640px) {
		.waiting-page {
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

		.desktop-back-button {
			position: absolute;
			top: clamp(128px, 8.33vw, 160px);
			left: clamp(28px, 3.6vw, 69px);
			display: inline-flex;
			width: clamp(32px, 2.08vw, 40px);
			height: clamp(32px, 2.08vw, 40px);
		}

		.desktop-back-button svg {
			width: clamp(22px, 1.46vw, 28px);
			height: clamp(22px, 1.46vw, 28px);
		}

		.waiting-content {
			width: min(calc(100vw - 80px), 916px);
			margin-top: clamp(132px, 13.33vw, 256px);
			gap: 30px;
		}

		.waiting-copy {
			width: min(100%, 916px);
		}

		.proxy-request,
		.description,
		.desktop-description,
		.mobile-description {
			font-size: clamp(18px, 1.25vw, 24px);
			line-height: clamp(26px, 1.67vw, 32px);
		}

		.desktop-description {
			display: block;
			width: min(100%, 778px);
		}

		.description {
			width: min(100%, 780px);
		}

		.mobile-description {
			display: none;
		}

		.user-info {
			width: min(100%, 780px);
			min-height: 82px;
			height: auto;
			padding: 30px 50px;
			border-radius: 10px;
		}

		h1 {
			font-size: clamp(22px, 1.56vw, 30px);
			line-height: clamp(30px, 1.98vw, 38px);
		}

		.user-info p {
			font-size: clamp(18px, 1.25vw, 24px);
			line-height: clamp(26px, 1.67vw, 32px);
		}

		.mobile-proxy-votes {
			display: none;
		}

		.desktop-proxy-votes {
			display: block;
		}

		.request-review-button {
			width: clamp(250px, 18.59vw, 357px);
			height: clamp(52px, 3.13vw, 60px);
			font-size: clamp(20px, 1.25vw, 24px);
			font-weight: 700;
			line-height: clamp(28px, 1.67vw, 32px);
			box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		}
	}

	@media (max-width: 374px) {
		.mobile-session-row,
		.waiting-content,
		.waiting-copy,
		.user-info {
			width: calc(100% - 28px);
		}
	}
</style>
