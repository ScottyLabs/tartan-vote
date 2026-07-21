<script lang="ts">
	import AppFooter from '$lib/components/AppFooter.svelte';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';
	import toggleUrl from '$lib/assets/toggle.svg?url';

	let compactMode = $state(false);

	function toggleCompactMode() {
		compactMode = !compactMode;
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

				<div class="copy-link-wrap">
					<button class="copy-link" type="button" disabled title="Available after backend connection">
						Invite link-not yet available
					</button>
				</div>
			</section>

			<nav class="sidebar-menu" aria-label="Session sections">
				<a class="menu-item active" href="/host" aria-current="page">Overview</a>
				<button class="menu-item" type="button" disabled title="Available after backend connection">Live View -not yet available</button>
				<button class="menu-item" type="button" disabled title="Available after backend connection">Session Configuration -not yet available</button>
				<button class="menu-item" type="button" disabled title="Available after backend connection">Session Results -not yet available</button>
			</nav>

			<button class="end-session" type="button" disabled title="Available after backend connection">End session -not yet available</button>
		</aside>

		<section class="overview" aria-label="Host overview">
			<section class="running-card" aria-label="Currently running voting instance">
				<div class="running-left">
					<div class="running-title">
						<svg class="radio-icon" viewBox="0 0 32 32" aria-hidden="true">
							<path d="M7.9 20.2a8 8 0 0 1 0-8.4M12 17.7a3.9 3.9 0 0 1 0-3.4M24.1 11.8a8 8 0 0 1 0 8.4M20 14.3a3.9 3.9 0 0 1 0 3.4" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" />
							<circle cx="16" cy="16" r="2.5" fill="currentColor" />
						</svg>
						<h1>Currently Running (N/A):</h1>
					</div>

					<p class="running-value">N/A</p>
				</div>

				<p class="votes-count">Votes Submitted: <span>N/A</span></p>
			</section>

			<div class="overview-grid">
				<section class="participants-card" aria-labelledby="participants-title">
					<header class="panel-header participants-header">
						<div class="panel-heading">
							<svg class="users-icon" viewBox="0 0 32 32" aria-hidden="true">
								<path d="M11.5 15.5a5 5 0 1 0 0-10 5 5 0 0 0 0 10ZM3.5 27v-3a6 6 0 0 1 6-6h4a6 6 0 0 1 6 6v3M22 14.5a4 4 0 1 0 0-8M24 18a6 6 0 0 1 5 6v3" fill="none" stroke="currentColor" stroke-width="2.7" stroke-linecap="round" stroke-linejoin="round" />
							</svg>
							<h2 id="participants-title">Participants: 0</h2>
						</div>

						<div class="compact-mode">
							<span class="compact-label">
								<svg class="file-user-icon" viewBox="0 0 30 30" aria-hidden="true">
									<path d="M8 3.5h9l5 5V26.5H8V3.5Z" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linejoin="round" />
									<path d="M17 3.5V9h5" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linejoin="round" />
									<circle cx="15" cy="16" r="2.4" fill="none" stroke="currentColor" stroke-width="2" />
									<path d="M10.8 23a4.4 4.4 0 0 1 8.4 0" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
								</svg>
								Compact Mode:
							</span>
							<button class="compact-toggle" class:enabled={compactMode} type="button" aria-pressed={compactMode} aria-label="Toggle compact mode" onclick={toggleCompactMode}>
								<img src={toggleUrl} alt="" width="56" height="36" />
								<span></span>
							</button>
						</div>
					</header>

					<div class="participants-empty" aria-label="No participants yet"></div>
				</section>

				<section class="actions-card" aria-labelledby="actions-title">
					<header class="panel-header actions-header">
						<div class="panel-heading">
							<svg class="briefcase-icon" viewBox="0 0 31 31" aria-hidden="true">
								<path d="M10.5 9.5V7.3c0-1.1.9-2 2-2h6c1.1 0 2 .9 2 2v2.2M4 10h23v15H4V10Z" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linejoin="round" />
								<path d="M4 15.5h23M13 17.5h5" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" />
							</svg>
							<h2 id="actions-title">Actions</h2>
						</div>
					</header>

					<div class="actions-body">
						<button class="action-button" type="button" disabled title="Available after backend connection">Push a Motion -not yet available</button>
						<button class="action-button" type="button" disabled title="Available after backend connection">Push an Election -not yet available</button>

						<p>Create a new voting instance</p>
					</div>
				</section>
			</div>
		</section>
	</div>

	<AppFooter wide />
</main>

<style>
	.host-page {
		position: relative;
		width: 100%;
		min-height: 100svh;
		overflow-x: hidden;
		background: var(--gradient-screen-signin);
		color: var(--color-grey-900);
	}

	.topbar {
		height: clamp(96px, 5.833vw, 112px);
		padding: clamp(22px, 1.354vw, 26px) clamp(44px, 3.542vw, 68px) clamp(22px, 1.406vw, 27px) clamp(28px, 1.771vw, 34px);
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: var(--color-red-600);
		box-shadow: 0 4px 4px rgb(0 0 0 / 0.25);
	}

	.brand-lockup {
		width: clamp(230px, 13.542vw, 260px);
		height: clamp(52px, 3.073vw, 59px);
		display: flex;
		align-items: center;
		gap: 8px;
		text-decoration: none;
	}

	.brand-mark {
		width: clamp(68px, 4.01vw, 77px);
		height: clamp(52px, 3.073vw, 59px);
		flex: 0 0 auto;
		display: block;
	}

	.brand-wordmark {
		color: var(--color-red-600);
		font-family: var(--font-brand);
		font-size: clamp(36px, 2.083vw, 40px);
		font-weight: 400;
		line-height: 1;
		letter-spacing: 0;
		-webkit-text-stroke: clamp(8px, 0.521vw, 10px) var(--color-white);
		paint-order: stroke fill;
	}

	.brand-wordmark span {
		color: var(--color-black);
	}

	.dashboard-title {
		width: clamp(300px, 17.188vw, 330px);
		margin: 0 8px 0 0;
		color: var(--color-white);
		font-size: clamp(26px, 1.563vw, 30px);
		font-weight: 400;
		line-height: clamp(34px, 1.979vw, 38px);
	}

	.host-shell {
		display: grid;
		grid-template-columns: clamp(270px, 19.792vw, 380px) minmax(0, 1fr);
		min-height: calc(100svh - clamp(96px, 5.833vw, 112px) - 81px);
		padding-bottom: 81px;
	}

	.sidebar {
		position: relative;
		min-height: calc(100svh - clamp(96px, 5.833vw, 112px) - 81px);
		background: var(--color-grey-900);
		color: var(--color-white);
	}

	.session-panel {
		height: clamp(190px, 11.198vw, 215px);
		padding: clamp(30px, 2.188vw, 42px) clamp(18px, 1.354vw, 26px) 0;
		background: var(--color-grey-800);
	}

	.session-panel h2 {
		margin: 0 0 20px;
		color: var(--color-grey-500);
		font-size: clamp(16px, 0.938vw, 18px);
		font-weight: 700;
		line-height: 26px;
	}

	.session-panel p {
		margin: 0 0 32px;
		color: var(--color-white);
		font-size: clamp(27px, 1.563vw, 30px);
		font-weight: 600;
		line-height: clamp(34px, 1.979vw, 38px);
	}

	.copy-link {
		height: 24px;
		border: 0;
		padding: 0;
		display: inline-flex;
		align-items: center;
		gap: 8px;
		background: transparent;
		color: var(--color-red-200);
		font: inherit;
		font-size: 16px;
		font-weight: 400;
		line-height: 24px;
		cursor: pointer;
	}

	.copy-link-wrap {
		position: relative;
		width: max-content;
	}

	.sidebar-menu {
		margin-top: 23px;
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	.menu-item {
		width: 100%;
		border: 0;
		height: 70px;
		padding: 0 clamp(28px, 2.344vw, 45px);
		display: flex;
		align-items: center;
		color: var(--color-white);
		font-size: clamp(16px, 0.938vw, 18px);
		font-weight: 700;
		line-height: 26px;
		text-decoration: none;
		text-align: left;
	}

	.menu-item.active {
		background: #66363e;
	}

	.menu-item:disabled {
	background: var(--color-grey-900);
	color: var(--color-grey-500);
	cursor: not-allowed;
	opacity: 1;
	}

	.end-session {
		position: absolute;
		left: 50%;
		bottom: 35px;
		width: min(calc(100% - 80px), 300px);
		height: 60px;
		transform: translateX(-50%);
		border: 1px solid var(--color-red-500);
		border-radius: 100px;
		background: transparent;
		color: var(--color-grey-200);
		font: inherit;
		font-size: 24px;
		font-weight: 500;
		line-height: 32px;
		cursor: pointer;
	}

	.overview {
		padding: clamp(30px, 2.188vw, 42px) clamp(24px, 3.229vw, 62px) 0 clamp(24px, 2.344vw, 45px);
	}

	.running-card {
		height: clamp(150px, 9.01vw, 173px);
		padding: clamp(18px, 1.146vw, 22px) clamp(22px, 1.51vw, 29px) 24px clamp(22px, 1.354vw, 26px);
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		background: var(--color-white);
		border: 1px solid var(--color-grey-200);
		border-radius: 10px;
	}

	.running-left {
		display: flex;
		flex-direction: column;
	}

	.running-title {
		height: 32px;
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.radio-icon {
		width: 32px;
		height: 32px;
		color: var(--color-black);
		flex: 0 0 auto;
	}

	h1,
	h2,
	.running-value,
	.votes-count {
		margin: 0;
	}

	h1,
	.votes-count {
		color: var(--color-grey-900);
		font-size: clamp(20px, 1.25vw, 24px);
		font-weight: 700;
		line-height: 32px;
	}

	.running-value {
		margin: 15px 0 0 8px;
		color: var(--color-grey-900);
		font-size: clamp(30px, 1.875vw, 36px);
		font-style: italic;
		font-weight: 400;
		line-height: 44px;
	}

	.votes-count {
		margin-top: 5px;
	}

	.votes-count span {
		color: var(--color-grey-600);
	}

	.overview-grid {
		margin-top: 23px;
		display: grid;
		grid-template-columns: minmax(0, 1fr) clamp(280px, 19.01vw, 365px);
		gap: clamp(20px, 1.615vw, 31px);
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
		height: clamp(500px, 33.177vw, 637px);
		min-width: 0;
	}

	.panel-header {
		height: 70px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: var(--color-grey-50);
		border-bottom: 1px solid var(--color-grey-300);
	}

	.participants-header {
		padding: 0 clamp(18px, 1.354vw, 26px) 0 clamp(20px, 1.458vw, 28px);
	}

	.panel-heading,
	.compact-label {
		display: inline-flex;
		align-items: center;
	}

	.panel-heading {
		gap: clamp(10px, 0.781vw, 15px);
	}

	.panel-heading h2,
	.compact-label {
		color: var(--color-grey-900);
		font-size: clamp(18px, 1.042vw, 20px);
		font-weight: 500;
		line-height: 28px;
	}

	.users-icon {
		width: 34px;
		height: 34px;
		color: var(--color-grey-900);
	}

	.file-user-icon {
		width: 30px;
		height: 30px;
		margin-right: 8px;
		color: var(--color-black);
	}

	.compact-mode {
		display: flex;
		align-items: center;
		gap: 0;
		flex: 0 0 auto;
	}

	.compact-toggle {
		position: relative;
		width: 56px;
		height: 36px;
		border: 0;
		padding: 0;
		background: transparent;
		cursor: pointer;
	}

	.compact-toggle::before {
		content: "";
		position: absolute;
		left: 6px;
		top: 6px;
		width: 44px;
		height: 24px;
		border-radius: 16px;
		background: transparent;
		transition: background 160ms ease;
	}

	.compact-toggle img {
		position: relative;
		z-index: 1;
		width: 56px;
		height: 36px;
		display: block;
		pointer-events: none;
	}

	.compact-toggle span {
		position: absolute;
		z-index: 2;
		left: 9px;
		top: 9px;
		width: 18px;
		height: 18px;
		border-radius: 999px;
		background: var(--color-slate-900);
		transition:
			background 160ms ease,
			transform 160ms ease;
	}

	.compact-toggle.enabled span {
		transform: translateX(19px);
		background: var(--color-white);
	}

	.compact-toggle.enabled::before {
		background: var(--color-grey-800);
	}

	.participants-empty {
		height: calc(100% - 70px);
		background: var(--color-white);
	}

	.actions-card {
		height: 325px;
		border-color: var(--color-grey-500);
		min-width: 0;
	}

	.actions-header {
		padding: 0 27px;
	}

	.briefcase-icon {
		width: 31px;
		height: 31px;
		color: var(--color-grey-900);
	}

	.actions-body {
		padding: 28px clamp(16px, 1.094vw, 21px) 26px;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.action-button {
		width: min(100%, 280px);
		height: 50px;
		border: 0;
		border-radius: 100px;
		background: var(--color-red-600);
		box-shadow: 0 4px 4px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: 18px;
		font-weight: 500;
		line-height: 26px;
		cursor: pointer;
	}

	.action-button + .action-button {
		margin-top: 20px;
	}

	.menu-item:disabled,
	.copy-link:disabled,
	.end-session:disabled,
	.action-button:disabled {
		cursor: not-allowed;
		opacity: 0.58;
	}

	.actions-body p {
		width: min(100%, 296px);
		height: 44px;
		margin: 36px 0 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-grey-50);
		border-radius: 2px;
		color: var(--color-grey-600);
		font-size: 16px;
		font-weight: 500;
		line-height: 24px;
	}

	button:hover,
	.menu-item:hover {
		filter: brightness(0.96);
	}

	button:focus-visible,
	a:focus-visible {
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 75%);
		outline-offset: 3px;
	}

	@media (max-width: 1500px) {
		.dashboard-title {
			margin-right: 0;
		}

		.overview-grid {
			grid-template-columns: minmax(0, 1fr) clamp(280px, 24vw, 330px);
			gap: 24px;
		}
	}

	@media (max-width: 1220px) {
		.host-shell {
			grid-template-columns: 260px minmax(0, 1fr);
		}

		.overview {
			padding-left: 20px;
			padding-right: 20px;
		}

		.overview-grid {
			grid-template-columns: minmax(0, 1fr) 280px;
			gap: 18px;
		}

		.compact-label {
			font-size: 16px;
		}

		.file-user-icon,
		.users-icon,
		.briefcase-icon {
			transform: scale(0.9);
		}
	}
</style>
