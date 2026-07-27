<script lang="ts">
	import { page } from '$app/state';
	import AppFooter from '$lib/components/AppFooter.svelte';
	import HostConfigurationDialog from '$lib/components/HostConfigurationDialog.svelte';
	import ProxyRequestDialog from '$lib/components/ProxyRequestDialog.svelte';
	import actionsUrl from '$lib/assets/host-actions.svg?url';
	import fileUserUrl from '$lib/assets/host-file-user.svg?url';
	import linkUrl from '$lib/assets/host-link.svg?url';
	import radioUrl from '$lib/assets/host-radio.svg?url';
	import usersUrl from '$lib/assets/host-users.svg?url';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';
	import { exampleParticipants, type HostDialog, type HostParticipant } from '$lib/domain/host';
	import { loadOrganizationSettings } from '$lib/domain/organizationSettings';

	const initialState = page.url.searchParams.get('state');
	const initialDialog = page.url.searchParams.get('dialog');
	const initiallyCompact = page.url.searchParams.get('compact') === 'true';
	const initiallySelectAll = page.url.searchParams.get('selectAll') === 'true';
	const hasRunningVote = page.url.searchParams.get('running') === 'true';
	const runningTitle = page.url.searchParams.get('title')?.trim() || null;
	const runningKind =
		page.url.searchParams.get('type') === 'election'
			? 'election'
			: page.url.searchParams.get('type') === 'quick-vote'
				? 'quick-vote'
				: 'motion';
	const runningOptions = (page.url.searchParams.get('labels') ?? '')
		.split('|')
		.map((option) => option.trim())
		.filter(Boolean);
	const runningVotingType = page.url.searchParams.get('style')?.trim() || 'Standard';
	const runningSubmitted = Math.max(
		0,
		Number.parseInt(page.url.searchParams.get('submitted') ?? '0', 10) || 0
	);
	const runningEligible = Math.max(
		runningSubmitted,
		Number.parseInt(page.url.searchParams.get('eligible') ?? '0', 10) || 0
	);
	const extendedParticipants: HostParticipant[] = [
		...exampleParticipants,
		...Array.from({ length: 30 }, (_, index) => ({
			id: index + 21,
			name: 'Scotty Labs',
			initials: 'SL',
			proxyStatus: 'none' as const,
			proxyVotes: []
		}))
	];
	const initialParticipants =
		initialState === 'inactive'
			? []
			: initiallyCompact
				? initiallySelectAll
					? extendedParticipants
					: exampleParticipants.slice(0, 5)
				: exampleParticipants;
	let compactMode = $state(initiallyCompact);
	let dialog = $state<HostDialog>(
		initialDialog === 'motion' || initialDialog === 'election' || initialDialog === 'proxy'
			? initialDialog
			: null
	);
	let participants = $state<HostParticipant[]>(
		initialParticipants.map((participant) => ({ ...participant }))
	);
	let selectedIds = $state<number[]>(
		initiallySelectAll
			? initialParticipants.map((participant) => participant.id)
			: []
	);
	let activeParticipantId = $state<number | null>(null);
	let quickVoteStarted = $state(false);
	let activeVoteName = $state<string | null>(runningTitle);
	let activeVoteKind = $state<'motion' | 'election' | 'quick-vote'>(runningKind);
	let activeVoteOptions = $state<string[]>(runningOptions);
	let activeVotingType = $state(runningVotingType);
	let activeQuorum = $state(page.url.searchParams.get('quorum')?.trim() || '');
	let votesSubmitted = $state(runningSubmitted);
	let eligibleVotes = $state(runningEligible);

	const selectedParticipant = $derived(
		participants.find((participant) => participant.id === activeParticipantId)
	);
	const allSelected = $derived(
		participants.length > 0 && selectedIds.length === participants.length
	);
	const hasActiveVote = $derived(hasRunningVote || quickVoteStarted || Boolean(activeVoteName));
	const hasBulkSelection = $derived(selectedIds.length > 0);
	const liveViewHref = $derived.by(() => {
		const params = new URLSearchParams({
			type: activeVoteKind,
			title: activeVoteName ?? 'QuickVote',
			labels: activeVoteOptions.length >= 2 ? activeVoteOptions.join('|') : 'Pass|Reject|Abstain',
			style: activeVotingType,
			quorum: activeQuorum,
			eligible: String(eligibleVotes)
		});
		return `/host/live?${params.toString()}`;
	});

	function toggleCompactMode() {
		compactMode = !compactMode;
		selectedIds = [];
	}

	function toggleParticipant(id: number) {
		selectedIds = selectedIds.includes(id)
			? selectedIds.filter((selectedId) => selectedId !== id)
			: [...selectedIds, id];
	}

	function toggleAll() {
		selectedIds = allSelected ? [] : participants.map((participant) => participant.id);
	}

	function setSelectedProxyStatus(status: 'accepted' | 'declined') {
		const selected = new Set(selectedIds);
		participants = participants.map((participant) =>
			selected.has(participant.id) && participant.proxyStatus !== 'none'
				? { ...participant, proxyStatus: status }
				: participant
		);
	}

	function kickSelected() {
		const selected = new Set(selectedIds);
		participants = participants.filter((participant) => !selected.has(participant.id));
		selectedIds = [];
	}

	function showProxyRequest(participant: HostParticipant) {
		activeParticipantId = participant.id;
		dialog = 'proxy';
	}

	function closeDialog() {
		dialog = null;
		activeParticipantId = null;
	}

	function updateParticipant(status: 'accepted' | 'declined') {
		if (activeParticipantId === null) return;
		participants = participants.map((participant) =>
			participant.id === activeParticipantId ? { ...participant, proxyStatus: status } : participant
		);
		closeDialog();
	}

	function setParticipantStatus(id: number, status: 'accepted' | 'declined') {
		participants = participants.map((participant) =>
			participant.id === id ? { ...participant, proxyStatus: status } : participant
		);
	}

	function kickParticipant(id: number) {
		participants = participants.filter((participant) => participant.id !== id);
		if (activeParticipantId === id) closeDialog();
	}

	function proxySummary(participant: HostParticipant) {
		if (participant.proxyVotes.length === 0) return 'Proxy Vote(s): N/A';
		const prefix =
			participant.proxyStatus === 'accepted'
				? 'Accepted Proxy Vote(s):'
				: participant.proxyStatus === 'declined'
					? 'Unaccepted Proxy Vote(s):'
					: 'Proxy Vote(s):';
		return `${prefix} ${participant.proxyVotes.join(', ')}`;
	}

	function startQuickVote() {
		const organizationSettings = loadOrganizationSettings();
		quickVoteStarted = true;
		activeVoteName = 'QuickVote';
		activeVoteKind = 'quick-vote';
		activeVoteOptions =
			organizationSettings.quickVoteOptions.length >= 2
				? organizationSettings.quickVoteOptions
				: ['Yes', 'No'];
		activeVotingType = 'Standard';
		activeQuorum = organizationSettings.quorum;
		votesSubmitted = 0;
		eligibleVotes = 0;
	}

	function startConfiguredVote(configuration: {
		kind: 'motion' | 'election';
		name: string;
		options: string[];
		votingType: string;
		enableQuorum: boolean;
		quorum: string;
	}) {
		activeVoteName = configuration.name;
		activeVoteKind = configuration.kind;
		activeVoteOptions = configuration.options;
		activeVotingType = configuration.votingType;
		activeQuorum = configuration.enableQuorum ? configuration.quorum : '';
		votesSubmitted = 0;
		eligibleVotes = 0;
		closeDialog();
	}
</script>

<svelte:head>
	<title>TartanVote | Host Dashboard</title>
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
					<span class="link-icon" aria-hidden="true">
						<img src={linkUrl} alt="" />
					</span>
				</button>
			</section>

			<nav class="sidebar-menu" aria-label="Session sections">
				<a class="menu-item active" href="/host" aria-current="page">Overview</a>
				{#if hasActiveVote}
					<a class="menu-item" href={liveViewHref}>Live View</a>
				{:else}
					<button
						class="menu-item"
						type="button"
						disabled
						title="Live View is available while a vote is running"
					>
						Live View
					</button>
				{/if}
				<a class="menu-item" href="/host/configuration">Session Configuration</a>
				<button class="menu-item" type="button">Comprehensive Session Results</button>
			</nav>

			<button class="end-session" type="button">End session</button>
		</aside>

		<section class="overview" aria-label="Host overview">
			<section class="running-card" aria-label="Currently running voting instance">
				<div>
					<div class="running-title">
						<img src={radioUrl} alt="" />
					<h1>
						Currently Running ({hasActiveVote
							? activeVoteKind === 'election'
								? 'Election'
								: activeVoteKind === 'quick-vote'
									? 'Quick Vote'
									: 'Motion'
							: 'N/A'}):
					</h1>
					</div>
					<p class:inactive={!hasActiveVote} class="running-value">{activeVoteName ?? 'N/A'}</p>
				</div>
				<p class="votes-count">
					Votes Submitted:
					<span>{hasActiveVote ? `${votesSubmitted}/${eligibleVotes}` : 'N/A'}</span>
				</p>
			</section>

			<div class="overview-grid">
				<section class="participants-card" aria-labelledby="participants-title">
					<header class="panel-header participants-header">
						<div class="panel-heading">
							<img src={usersUrl} alt="" />
							<h2 id="participants-title">Participants: {participants.length}</h2>
						</div>
						<div class="compact-mode">
							<img src={fileUserUrl} alt="" />
							<span>Compact Mode:</span>
							<button class="compact-toggle" class:enabled={compactMode} type="button" role="switch" aria-label="Compact mode" aria-checked={compactMode} onclick={toggleCompactMode}>
								<span></span>
							</button>
						</div>
					</header>

					{#if compactMode}
						<div class="bulk-actions">
							<button class="accept-bulk" type="button" disabled={!hasBulkSelection} onclick={() => setSelectedProxyStatus('accepted')}>Accept all selected Proxy Votes</button>
							<button class="revoke-bulk" type="button" disabled={!hasBulkSelection} onclick={() => setSelectedProxyStatus('declined')}>Revoke all selected Proxy Votes</button>
							<button class="kick-bulk" type="button" disabled={!hasBulkSelection} onclick={kickSelected}>Kick all selected</button>
							<label>Select All <input type="checkbox" checked={allSelected} onchange={toggleAll} /></label>
						</div>
						<div class="compact-list">
							{#each participants as participant (participant.id)}
								<div class="compact-row">
									<button type="button" onclick={() => showProxyRequest(participant)}>
										<span class="participant-name">{participant.name}</span><span
											class="proxy-details"
											class:accepted={participant.proxyStatus === 'accepted'}
											class:pending={participant.proxyStatus === 'pending' || participant.proxyStatus === 'declined'}
										>{' - '}{proxySummary(participant)}</span>
									</button>
									<input
										type="checkbox"
										aria-label={`Select ${participant.name}`}
										checked={selectedIds.includes(participant.id)}
										onchange={() => toggleParticipant(participant.id)}
									/>
								</div>
							{/each}
						</div>
					{:else}
						<div class="participant-list">
							{#each participants as participant (participant.id)}
								<article class="participant-row">
									<div class="avatar" aria-hidden="true">{participant.initials}</div>
									<button class="participant-copy" type="button" onclick={() => showProxyRequest(participant)}>
										<strong>{participant.name}</strong>
										<span class:accepted={participant.proxyStatus === 'accepted'} class:pending={participant.proxyStatus === 'pending' || participant.proxyStatus === 'declined'}>
											{proxySummary(participant)}
										</span>
									</button>
									<div class="participant-actions">
										{#if participant.proxyStatus === 'pending'}
											<button class="accept-proxy" type="button" onclick={() => showProxyRequest(participant)}>Accept Proxy</button>
										{:else if participant.proxyStatus === 'accepted'}
											<button class="revoke-proxy" type="button" onclick={() => setParticipantStatus(participant.id, 'declined')}>Revoke Proxy</button>
										{/if}
										<button class="kick" type="button" onclick={() => kickParticipant(participant.id)}>Kick</button>
									</div>
								</article>
							{/each}
						</div>
					{/if}
				</section>

				<section class="actions-card" aria-labelledby="actions-title">
					<header class="panel-header actions-header">
						<div class="panel-heading">
							<img src={actionsUrl} alt="" />
							<h2 id="actions-title">Actions</h2>
						</div>
					</header>
					<div class="actions-body">
						<button class="action-button" type="button" disabled={hasActiveVote} onclick={() => (dialog = 'motion')}>+ Push a Motion</button>
						<button class="action-button" type="button" disabled={hasActiveVote} onclick={() => (dialog = 'election')}>+ Push an Election</button>
						<div class="noncompact-actions">
							<button class="action-button" type="button" disabled={hasActiveVote} onclick={startQuickVote}>+ Quick Vote</button>
							<p>
								{hasActiveVote
									? 'You must end your current voting instance to begin a new one.'
									: 'Create a new voting instance'}
							</p>
						</div>
					</div>
				</section>
			</div>
		</section>
	</div>

	<AppFooter wide />
</main>

{#if dialog === 'motion' || dialog === 'election'}
	<HostConfigurationDialog kind={dialog} onclose={closeDialog} onsubmit={startConfiguredVote} />
{:else if dialog === 'proxy'}
	<ProxyRequestDialog
		requester={selectedParticipant?.name ?? 'Scottylabs1'}
		proxyVotes={selectedParticipant?.proxyVotes}
		onclose={closeDialog}
		onaccept={() => updateParticipant('accepted')}
		ondecline={() => updateParticipant('declined')}
		onkick={() => activeParticipantId !== null && kickParticipant(activeParticipantId)}
	/>
{/if}

<style>
	.host-page {
		position: relative;
		width: 100%;
		min-width: 1024px;
		min-height: 100svh;
		overflow: hidden;
		background: var(--gradient-screen-signin);
		color: var(--color-grey-900);
	}

	.topbar {
		height: clamp(60px, 5.833vw, 112px);
		padding: 0 clamp(34px, 3.542vw, 68px) 0 clamp(18px, 1.771vw, 34px);
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: var(--color-red-600);
		box-shadow: 0 4px 4px rgb(0 0 0 / 0.25);
	}

	.brand-lockup {
		display: flex;
		align-items: center;
		gap: 8px;
		text-decoration: none;
	}

	.brand-mark {
		width: clamp(42px, 4.01vw, 77px);
		height: clamp(32px, 3.073vw, 59px);
		display: block;
	}

	.brand-wordmark {
		color: var(--color-red-600);
		font-family: var(--font-brand);
		font-size: clamp(24px, 2.083vw, 40px);
		-webkit-text-stroke: clamp(5px, 0.521vw, 10px) var(--color-white);
		paint-order: stroke fill;
	}

	.brand-wordmark span { color: var(--color-black); }

	.dashboard-title {
		margin: 0;
		color: var(--color-white);
		font-size: clamp(18px, 1.563vw, 30px);
		line-height: 38px;
	}

	.host-shell {
		display: grid;
		grid-template-columns: clamp(203px, 19.792vw, 380px) minmax(0, 1fr);
		height: calc(100svh - clamp(60px, 5.833vw, 112px) - 81px);
		min-height: calc(100svh - clamp(60px, 5.833vw, 112px) - 81px);
	}

	.sidebar {
		position: relative;
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

	.copy-link > span:first-child {
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.link-icon {
		width: clamp(10px, 1.042vw, 20px);
		height: clamp(10px, 1.042vw, 20px);
		flex: 0 0 auto;
		display: grid;
		place-items: center;
	}

	.link-icon img {
		width: 91.6665%;
		height: 50%;
		display: block;
		object-fit: contain;
	}

	.sidebar-menu {
		margin-top: clamp(12px, 1.198vw, 23px);
		display: grid;
		gap: 5px;
	}

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

	.menu-item:disabled {
		color: var(--color-grey-100);
		cursor: not-allowed;
		opacity: 1;
	}

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

	.overview {
		min-height: 0;
		padding: clamp(22px, 2.188vw, 42px) clamp(32px, 3.229vw, 62px) 12px clamp(24px, 2.344vw, 45px);
		display: flex;
		flex-direction: column;
	}

	.running-card {
		flex: 0 0 auto;
		height: clamp(93px, 9.01vw, 173px);
		padding: clamp(11px, 1.146vw, 22px) clamp(15px, 1.51vw, 29px);
		display: flex;
		justify-content: space-between;
		background: var(--color-white);
		border: 1px solid var(--color-grey-200);
		border-radius: 10px;
	}

	.running-title { display: flex; align-items: center; gap: 6px; }
	.running-title img { width: clamp(18px, 1.667vw, 32px); height: clamp(18px, 1.667vw, 32px); }

	h1,
	h2,
	.running-value,
	.votes-count { margin: 0; }

	h1,
	.votes-count {
		font-size: clamp(14px, 1.25vw, 24px);
		line-height: 32px;
	}

	.running-value {
		margin: clamp(2px, 0.781vw, 15px) 0 0 8px;
		font-size: clamp(22px, 1.875vw, 36px);
		font-style: italic;
		line-height: 44px;
	}
	.running-value.inactive { color: var(--color-grey-500); }

	.votes-count { margin-top: 5px; }
	.votes-count span { color: var(--color-grey-600); }

	.overview-grid {
		min-height: 0;
		flex: 1;
		margin-top: clamp(13px, 1.198vw, 23px);
		display: grid;
		grid-template-columns: minmax(0, 1fr) clamp(194px, 19.01vw, 365px);
		gap: clamp(16px, 1.615vw, 31px);
		align-items: start;
	}

	.participants-card,
	.actions-card {
		background: var(--color-white);
		border: 1px solid var(--color-grey-200);
		border-radius: 10px;
		overflow: hidden;
	}

	.participants-card {
		height: 100%;
		min-height: 0;
	}

	.panel-header {
		height: clamp(38px, 3.646vw, 70px);
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: var(--color-grey-50);
		border-bottom: 1px solid var(--color-grey-300);
	}

	.participants-header { padding: 0 clamp(13px, 1.354vw, 26px); }
	.panel-heading,
	.compact-mode { display: flex; align-items: center; }
	.panel-heading { gap: clamp(7px, 0.781vw, 15px); }
	.panel-heading img,
	.compact-mode > img { width: clamp(18px, 1.615vw, 31px); height: clamp(18px, 1.615vw, 31px); }

	.panel-heading h2,
	.compact-mode {
		font-size: clamp(12px, 1.042vw, 20px);
		font-weight: 500;
		line-height: 28px;
	}

	.compact-mode { gap: 7px; }

	.compact-toggle {
		position: relative;
		width: 44px;
		height: 24px;
		border: 1px solid var(--color-slate-900);
		border-radius: 16px;
		background: var(--color-white);
		cursor: pointer;
	}

	.compact-toggle span {
		position: absolute;
		left: 3px;
		top: 2px;
		width: 18px;
		height: 18px;
		border-radius: 50%;
		background: var(--color-slate-900);
		transition: transform 150ms ease;
	}

	.compact-toggle.enabled { background: var(--color-slate-900); }
	.compact-toggle.enabled span { transform: translateX(18px); background: var(--color-white); }

	.participant-list,
	.compact-list {
		height: calc(100% - clamp(38px, 3.646vw, 70px));
		overflow-y: auto;
	}

	.participant-row {
		min-height: clamp(42px, 4.167vw, 80px);
		padding: 5px clamp(14px, 2.604vw, 50px);
		display: grid;
		grid-template-columns: clamp(28px, 2.604vw, 50px) minmax(0, 1fr) auto;
		align-items: center;
		gap: clamp(8px, 0.833vw, 16px);
	}

	.avatar {
		width: clamp(28px, 2.604vw, 50px);
		height: clamp(28px, 2.604vw, 50px);
		border-radius: 50%;
		display: grid;
		place-items: center;
		background: var(--avatar-background, var(--color-blue-100));
		color: var(--avatar-foreground, var(--color-blue-500));
		font-size: clamp(14px, 1.25vw, 24px);
		font-weight: 700;
	}

	.participant-row:nth-child(8n + 1) {
		--avatar-background: var(--color-blue-100);
		--avatar-foreground: var(--color-blue-500);
	}

	.participant-row:nth-child(8n + 2) {
		--avatar-background: var(--color-blue-500);
		--avatar-foreground: var(--color-blue-100);
	}

	.participant-row:nth-child(8n + 3) {
		--avatar-background: var(--color-blue-200);
		--avatar-foreground: var(--color-blue-600);
	}

	.participant-row:nth-child(8n + 4) {
		--avatar-background: var(--color-blue-600);
		--avatar-foreground: var(--color-blue-200);
	}

	.participant-row:nth-child(8n + 5) {
		--avatar-background: var(--color-blue-300);
		--avatar-foreground: var(--color-blue-700);
	}

	.participant-row:nth-child(8n + 6) {
		--avatar-background: var(--color-blue-700);
		--avatar-foreground: var(--color-blue-300);
	}

	.participant-row:nth-child(8n + 7) {
		--avatar-background: var(--color-blue-400);
		--avatar-foreground: var(--color-blue-800);
	}

	.participant-row:nth-child(8n + 8) {
		--avatar-background: var(--color-blue-800);
		--avatar-foreground: var(--color-blue-400);
	}

	.participant-copy,
	.compact-row button {
		min-width: 0;
		border: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		overflow: hidden;
		background: transparent;
		color: inherit;
		font: inherit;
		text-align: left;
		cursor: pointer;
	}

	.participant-copy strong { font-size: clamp(12px, 0.938vw, 18px); line-height: 26px; }
	.participant-copy span { max-width: 100%; overflow: hidden; color: var(--color-grey-600); font-size: clamp(10px, 0.833vw, 16px); text-overflow: ellipsis; white-space: nowrap; }
	.participant-copy span.pending,
	.compact-row span.pending { color: var(--color-red-500); }
	.participant-copy span.accepted,
	.compact-row span.accepted { color: var(--color-green-500); }

	.participant-actions { display: flex; gap: 15px; }
	.participant-actions button,
	.bulk-actions button {
		height: clamp(24px, 2.083vw, 40px);
		border: 0;
		border-radius: 999px;
		box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		font: inherit;
		font-size: clamp(10px, 0.938vw, 18px);
		cursor: pointer;
	}

	.accept-proxy { width: clamp(96px, 9.375vw, 180px); background: var(--color-green-400); color: var(--color-white); }
	.revoke-proxy { width: clamp(96px, 9.375vw, 180px); border: 1px solid var(--color-grey-300) !important; background: var(--color-white); color: var(--color-red-400); }
	.kick { width: clamp(54px, 5.208vw, 100px); background: var(--color-slate-700); color: var(--color-white); }

	.bulk-actions {
		height: clamp(38px, 3.646vw, 70px);
		padding: 0 13px;
		display: grid;
		grid-template-columns: 1.05fr 1.1fr 0.7fr auto;
		align-items: center;
		gap: 10px;
		overflow: hidden;
	}

	.bulk-actions button {
		min-width: 0;
		height: clamp(24px, 2.083vw, 40px);
		padding: 0 clamp(10px, 0.833vw, 16px);
		overflow: hidden;
		color: var(--color-white);
		font-size: clamp(9px, 0.833vw, 16px);
		line-height: 24px;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.accept-bulk { background: var(--color-green-400); }
	.revoke-bulk { background: var(--color-slate-500); }
	.kick-bulk { background: var(--color-slate-800); }
	.accept-bulk:disabled {
		background: var(--color-green-100);
		color: var(--color-green-600);
	}
	.revoke-bulk:disabled {
		background: var(--color-slate-100);
		color: var(--color-slate-500);
	}
	.kick-bulk:disabled {
		background: var(--color-grey-200);
		color: var(--color-grey-600);
	}
	.bulk-actions button:disabled {
		box-shadow: none;
		cursor: not-allowed;
	}
	.bulk-actions label {
		height: clamp(24px, 2.083vw, 40px);
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 6px;
		line-height: 1;
		font-size: clamp(11px, 0.938vw, 18px);
		white-space: nowrap;
	}

	.bulk-actions input[type='checkbox'],
	.compact-row input[type='checkbox'] {
		width: clamp(14px, 1.042vw, 20px);
		height: clamp(14px, 1.042vw, 20px);
		margin: 0;
		accent-color: var(--color-slate-900);
	}

	.compact-list { height: calc(100% - clamp(76px, 7.292vw, 140px)); }
	.compact-row {
		height: clamp(18px, 1.823vw, 35px);
		padding: 0 clamp(14px, 1.458vw, 28px);
		display: flex;
		align-items: center;
		gap: 8px;
		background: var(--color-grey-50);
	}
	.compact-row:nth-child(even) { background: var(--color-white); }
	.compact-row button { flex: 1; flex-direction: row; font-size: clamp(9px, 0.833vw, 16px); white-space: nowrap; text-overflow: ellipsis; }
	.compact-row .participant-name { color: var(--color-black); }
	.compact-row .proxy-details { color: var(--color-grey-500); }

	.actions-card {
		height: auto;
		min-height: 0;
		border-color: var(--color-grey-500);
	}
	.actions-header { padding: 0 clamp(14px, 1.406vw, 27px); }
	.actions-body {
		padding: clamp(15px, 1.458vw, 28px) clamp(11px, 1.094vw, 21px)
			clamp(11px, 1.094vw, 21px);
		display: grid;
		justify-items: center;
		gap: clamp(10px, 1.042vw, 20px);
	}

	.action-button {
		width: min(100%, 280px);
		height: clamp(28px, 2.604vw, 50px);
		border: 0;
		border-radius: 100px;
		background: var(--color-red-600);
		box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: clamp(11px, 0.938vw, 18px);
		cursor: pointer;
	}
	.action-button:disabled {
		background: var(--color-red-200);
		box-shadow: none;
		cursor: not-allowed;
	}

	.noncompact-actions {
		width: 100%;
		display: grid;
		justify-items: center;
		gap: clamp(10px, 1.042vw, 20px);
	}

	.actions-body p {
		width: 100%;
		min-height: clamp(36px, 3.125vw, 60px);
		margin: clamp(8px, 0.833vw, 16px) 0 0;
		padding: 8px 12px;
		display: grid;
		place-items: center;
		background: var(--color-grey-50);
		color: var(--color-grey-600);
		font-size: clamp(9px, 0.833vw, 16px);
		line-height: 1.35;
		text-align: center;
	}

	button:focus-visible,
	a:focus-visible {
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 70%);
		outline-offset: 2px;
	}

	@media (max-width: 1200px) {
		.overview { padding-inline: 24px; }
		.participant-actions { gap: 8px; }
	}
</style>
