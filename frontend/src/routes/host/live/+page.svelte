<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import AppFooter from '$lib/components/AppFooter.svelte';
	import ConfirmLiveResultsSharingDialog from '$lib/components/ConfirmLiveResultsSharingDialog.svelte';
	import EndVotingInstanceDialog from '$lib/components/EndVotingInstanceDialog.svelte';
	import HostFinalResultsDialog from '$lib/components/HostFinalResultsDialog.svelte';
	import linkUrl from '$lib/assets/host-link.svg?url';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';
	import { createHostLiveView } from '$lib/domain/hostLive';

	const liveView = createHostLiveView(page.url.searchParams);
	const overviewParams = new URLSearchParams({
		running: 'true',
		type: liveView.type,
		title: liveView.title,
		labels: liveView.options.map((option) => option.label).join('|'),
		style: page.url.searchParams.get('style') ?? liveView.votingStyle,
		quorum: page.url.searchParams.get('quorum') ?? '',
		submitted: String(liveView.votesSubmitted),
		eligible: String(liveView.eligibleVotes)
	});
	let resultsShared = $state(page.url.searchParams.get('shared') === 'true');
	let endDialogOpen = $state(page.url.searchParams.get('dialog') === 'end');
	let finalDialogOpen = $state(page.url.searchParams.get('dialog') === 'final');
	let sharingDialogOpen = $state(page.url.searchParams.get('dialog') === 'share');
	let instanceEnded = $state(false);

	function confirmEnd() {
		endDialogOpen = false;
		finalDialogOpen = true;
	}

	function confirmFinalResult() {
		instanceEnded = true;
		finalDialogOpen = false;
		void goto('/host');
	}

	function confirmLiveResultsSharing() {
		resultsShared = !resultsShared;
		sharingDialogOpen = false;
	}
</script>

<svelte:head>
	<title>TartanVote | Host Live View</title>
	<meta name="description" content="Monitor the active TartanVote voting instance." />
</svelte:head>

<main class="host-page">
	<header class="topbar">
		<a class="brand-lockup" href="/home" aria-label="TartanVote home">
			<img class="brand-mark" src={logoUrl} alt="" width="77" height="59" />
			<span class="brand-wordmark"><span>Tartan</span>Vote</span>
		</a>
		<p class="dashboard-title">HOSTING DASHBOARD</p>
	</header>

	<div class="host-shell">
		<aside class="sidebar" aria-label="Host dashboard navigation">
			<section class="session-panel" aria-labelledby="session-code-label">
				<h2 id="session-code-label">Session Code</h2>
				<p>happy-giraffe</p>
				<button class="copy-link" type="button">
					<span>Copy invite link</span>
					<span class="link-icon" aria-hidden="true"><img src={linkUrl} alt="" /></span>
				</button>
			</section>

			<nav class="sidebar-menu" aria-label="Session sections">
				<a class="menu-item" href={`/host?${overviewParams.toString()}`}>Overview</a>
				<a class="menu-item active" href="/host/live" aria-current="page">Live View</a>
				<a class="menu-item" href="/host/configuration">Session Configuration</a>
				<button class="menu-item" type="button">Comprehensive Session Results</button>
			</nav>

			<button class="end-session" type="button">End session</button>
		</aside>

		<section class="live-page" aria-labelledby="live-view-heading">
			<article class="live-card" class:ended={instanceEnded}>
				<header class="instance-header">
					<h1>{liveView.title}</h1>
					<p>
						Votes Submitted:
						<strong>{instanceEnded ? 'Ended' : `${liveView.votesSubmitted}/${liveView.eligibleVotes}`}</strong>
					</p>
				</header>

				<p class="instance-meta">
					Type: {liveView.type === 'election'
						? 'Election'
						: liveView.type === 'quick-vote'
							? 'Quick Vote'
							: 'Motion'} // Voting Style:
					{page.url.searchParams.get('style') ?? liveView.votingStyle}
				</p>

				<div class="separator"></div>

				<section class="results-section" aria-labelledby="live-view-heading">
					<h2 id="live-view-heading">Live View</h2>
					<div class="results-list">
						{#each liveView.options as option (option.id)}
							<article class="result-row">
								<div class="result-heading">
									<strong>{option.label}: ({option.votes} {option.votes === 1 ? 'Vote' : 'Votes'})</strong>
									<strong>{option.percentage}%</strong>
								</div>
								<div
									class="progress-track"
									role="progressbar"
									aria-label={`${option.label}: ${option.percentage}%`}
									aria-valuenow={option.percentage}
									aria-valuemin="0"
									aria-valuemax="100"
								>
									<span class={option.color} style={`width: ${option.percentage}%`}></span>
								</div>
							</article>
						{/each}
					</div>
				</section>

				<div class="card-actions">
					<button
						class="end-now"
						type="button"
						disabled={instanceEnded}
						onclick={() => (endDialogOpen = true)}
					>
						{instanceEnded ? 'Ended' : 'End now'}
					</button>
					<button
						class="share-results"
						class:shared={resultsShared}
						type="button"
						disabled={instanceEnded}
						aria-pressed={resultsShared}
						onclick={() => (sharingDialogOpen = true)}
					>
						{resultsShared ? 'Unshare live results' : 'Share live results'}
					</button>
				</div>
			</article>
		</section>
	</div>

	<AppFooter wide flow />
</main>

{#if endDialogOpen}
	<EndVotingInstanceDialog onclose={() => (endDialogOpen = false)} onconfirm={confirmEnd} />
{/if}

{#if finalDialogOpen}
	<HostFinalResultsDialog
		options={liveView.options}
		totalVotes={liveView.votesSubmitted}
		shared={resultsShared}
		onconfirm={confirmFinalResult}
		ontoggleshare={() => (resultsShared = !resultsShared)}
	/>
{/if}

{#if sharingDialogOpen}
	<ConfirmLiveResultsSharingDialog
		currentlyShared={resultsShared}
		onclose={() => (sharingDialogOpen = false)}
		onconfirm={confirmLiveResultsSharing}
	/>
{/if}

<style>
	.host-page {
		width: 100%;
		min-width: 1024px;
		min-height: 100svh;
		display: flex;
		flex-direction: column;
		background: var(--gradient-screen-signin);
		color: var(--color-grey-900);
	}
	.topbar {
		height: clamp(60px, 5.833vw, 112px);
		flex: 0 0 clamp(60px, 5.833vw, 112px);
		padding: 0 clamp(34px, 3.542vw, 68px) 0 clamp(18px, 1.771vw, 34px);
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: var(--color-red-600);
		box-shadow: 0 4px 4px rgb(0 0 0 / 0.25);
	}
	.brand-lockup { display: flex; align-items: center; gap: 8px; text-decoration: none; }
	.brand-mark { width: clamp(42px, 4.01vw, 77px); height: clamp(32px, 3.073vw, 59px); display: block; }
	.brand-wordmark {
		color: var(--color-red-600);
		font-family: var(--font-brand);
		font-size: clamp(24px, 2.083vw, 40px);
		-webkit-text-stroke: clamp(5px, 0.521vw, 10px) var(--color-white);
		paint-order: stroke fill;
	}
	.brand-wordmark span { color: var(--color-black); }
	.dashboard-title { margin: 0; color: var(--color-white); font-size: clamp(18px, 1.563vw, 30px); line-height: 38px; }
	.host-shell {
		flex: 1 0 calc(100svh - clamp(60px, 5.833vw, 112px) - 81px);
		display: grid;
		grid-template-columns: clamp(203px, 19.792vw, 380px) minmax(0, 1fr);
	}
	.sidebar {
		position: relative;
		min-height: calc(100svh - clamp(60px, 5.833vw, 112px) - 81px);
		background: var(--color-grey-900);
		color: var(--color-white);
	}
	.session-panel {
		height: clamp(115px, 11.198vw, 215px);
		padding: clamp(22px, 2.188vw, 42px) clamp(14px, 1.354vw, 26px) 0;
		background: var(--color-grey-800);
	}
	.session-panel h2 {
		margin: 0 0 clamp(4px, 1.042vw, 20px);
		color: var(--color-grey-500);
		font-size: clamp(10px, 0.938vw, 18px);
		line-height: clamp(14px, 1.354vw, 26px);
	}
	.session-panel p {
		margin: 0 0 clamp(6px, 1.667vw, 32px);
		font-size: clamp(17px, 1.563vw, 30px);
		font-weight: 600;
		line-height: clamp(22px, 1.979vw, 38px);
	}
	.copy-link {
		max-width: 100%;
		height: 24px;
		border: 0;
		padding: 0;
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: transparent;
		color: var(--color-red-200);
		font: inherit;
		font-size: clamp(9px, 0.833vw, 16px);
		cursor: pointer;
	}
	.copy-link > span:first-child { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
	.link-icon { width: clamp(10px, 1.042vw, 20px); height: clamp(10px, 1.042vw, 20px); flex: 0 0 auto; display: grid; place-items: center; }
	.link-icon img { width: 91.6665%; height: 50%; display: block; object-fit: contain; }
	.sidebar-menu { margin-top: clamp(12px, 1.198vw, 23px); display: grid; gap: 5px; }
	.menu-item {
		width: 100%;
		height: clamp(40px, 3.646vw, 70px);
		border: 0;
		padding: 0 clamp(24px, 2.344vw, 45px);
		display: flex;
		align-items: center;
		background: transparent;
		color: var(--color-white);
		font: inherit;
		font-size: clamp(11px, 0.938vw, 18px);
		font-weight: 700;
		line-height: 26px;
		text-align: left;
		text-decoration: none;
		cursor: pointer;
	}
	.menu-item.active { background: #66363e; }
	.end-session {
		position: absolute;
		left: 50%;
		bottom: 35px;
		width: min(calc(100% - 42px), 300px);
		height: clamp(34px, 3.125vw, 60px);
		transform: translateX(-50%);
		border: 1px solid var(--color-red-500);
		border-radius: 100px;
		background: transparent;
		color: var(--color-grey-200);
		font: inherit;
		font-size: clamp(14px, 1.25vw, 24px);
		cursor: pointer;
	}
	.live-page {
		padding: clamp(20px, 1.979vw, 38px) clamp(24px, 3.385vw, 65px);
		display: flex;
		align-items: flex-start;
	}
	.live-card {
		width: 100%;
		min-height: clamp(376px, 36.719vw, 705px);
		padding: clamp(30px, 3.125vw, 60px) clamp(34px, 4.688vw, 90px) clamp(28px, 3.125vw, 60px);
		border: 2px solid var(--color-grey-200);
		border-radius: 10px;
		display: flex;
		flex-direction: column;
		background: var(--color-white);
	}
	.live-card.ended { opacity: 0.78; }
	.instance-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 30px; }
	.instance-header h1 {
		max-width: 70%;
		margin: 0;
		font-size: clamp(22px, 1.875vw, 36px);
		font-style: italic;
		font-weight: 600;
		line-height: 1.3;
		overflow-wrap: anywhere;
	}
	.instance-header p {
		margin: 0;
		flex: 0 0 auto;
		font-size: clamp(14px, 1.042vw, 20px);
		font-weight: 600;
		line-height: 1.4;
	}
	.instance-header strong { color: var(--color-red-400); }
	.instance-meta {
		margin: clamp(16px, 1.823vw, 35px) 0 0;
		font-size: clamp(13px, 0.938vw, 18px);
		font-weight: 500;
		line-height: 1.45;
	}
	.separator { height: 2px; margin-top: clamp(12px, 1.042vw, 20px); background: var(--color-grey-100); }
	.results-section {
		width: min(100%, 859px);
		margin: clamp(16px, 1.302vw, 25px) auto 0;
	}
	.results-section h2 {
		margin: 0 0 clamp(18px, 1.823vw, 35px);
		color: var(--color-slate-700);
		font-size: clamp(20px, 1.563vw, 30px);
		font-weight: 700;
		line-height: 1.27;
	}
	.results-list { display: flex; flex-direction: column; gap: clamp(18px, 1.563vw, 30px); }
	.result-heading {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 24px;
		font-size: clamp(13px, 0.938vw, 18px);
		line-height: 1.45;
	}
	.result-heading strong:first-child { min-width: 0; overflow-wrap: anywhere; }
	.result-heading strong:last-child {
		flex: 0 0 auto;
		font-size: clamp(16px, 1.25vw, 24px);
		line-height: 1.34;
	}
	.progress-track {
		width: 100%;
		height: 8px;
		margin-top: 5px;
		border-radius: 30px;
		overflow: hidden;
		background: var(--color-grey-200);
	}
	.progress-track span { height: 100%; border-radius: inherit; display: block; transition: width 180ms ease; }
	.progress-track .green { background: var(--color-green-300); }
	.progress-track .yellow { background: var(--color-yellow-300); }
	.progress-track .blue { background: var(--color-blue-300); }
	.progress-track .red { background: var(--color-red-500); }
	.progress-track .purple { background: var(--color-purple-300); }
	.card-actions {
		margin-top: auto;
		padding-top: clamp(28px, 3.125vw, 60px);
		display: flex;
		justify-content: flex-end;
		gap: clamp(14px, 1.042vw, 20px);
	}
	.card-actions button {
		height: clamp(36px, 2.604vw, 50px);
		border-radius: 100px;
		font: inherit;
		font-size: clamp(13px, 0.938vw, 18px);
		font-weight: 500;
		cursor: pointer;
	}
	.end-now {
		width: clamp(140px, 10.417vw, 200px);
		border: 1px solid var(--color-red-500);
		background: var(--color-white);
		color: var(--color-red-500);
	}
	.share-results {
		width: clamp(180px, 14.583vw, 280px);
		border: 1px solid var(--color-red-600);
		background: var(--color-red-600);
		color: var(--color-white);
	}
	.share-results.shared { border-color: var(--color-slate-700); background: var(--color-slate-700); }
	.card-actions button:disabled { border-color: var(--color-red-200); background: var(--color-red-200); color: var(--color-white); cursor: not-allowed; }
</style>
