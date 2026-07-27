<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import AppFooter from '$lib/components/AppFooter.svelte';
	import ConfirmDeleteOrganizationDialog from '$lib/components/ConfirmDeleteOrganizationDialog.svelte';
	import OrganizationNameDialog from '$lib/components/OrganizationNameDialog.svelte';
	import linkUrl from '$lib/assets/host-link.svg?url';
	import pencilUrl from '$lib/assets/host-pencil.svg?url';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';
	import {
		clearOrganizationSettings,
		defaultOrganizationSettings,
		exampleOrganizations,
		loadDeletedOrganizationNames,
		loadOrganizationSettings,
		saveDeletedOrganizationNames,
		saveOrganizationSettings,
		type OrganizationSettings
	} from '$lib/domain/organizationSettings';

	let organization = $state('Default');
	let defaultQuorum = $state('');
	let quickVoteOptions = $state('Yes, No');
	let approvedMembers = $state('');
	let saved = $state(false);
	let dropdownOpen = $state(page.url.searchParams.get('dropdown') === 'true');
	let error = $state('');
	let organizations = $state<OrganizationSettings[]>(exampleOrganizations.map((settings) => ({ ...settings })));
	let organizationPendingDelete = $state<string | null>(page.url.searchParams.get('delete'));
	let nameDialogMode = $state<'create' | 'edit' | null>(null);
	let nameDialogError = $state('');

	const isDefault = $derived(organization === 'Default');

	onMount(() => {
		const deletedNames = loadDeletedOrganizationNames();
		organizations = organizations.filter((settings) => !deletedNames.includes(settings.name));
		const storedSettings = loadOrganizationSettings();
		if (
			storedSettings.name !== 'Default' &&
			!organizations.some((settings) => settings.name === storedSettings.name)
		) {
			organizations.push(storedSettings);
		}
		const requestedOrganization = page.url.searchParams.get('organization');
		if (requestedOrganization) {
			selectOrganization(requestedOrganization);
			dropdownOpen = page.url.searchParams.get('dropdown') === 'true';
			return;
		}
		applyOrganization(storedSettings);
	});

	function applyOrganization(settings: OrganizationSettings) {
		organization = settings.name;
		defaultQuorum = settings.quorum;
		quickVoteOptions = settings.quickVoteOptions.join(', ');
		approvedMembers = settings.approvedMembers.join(', ');
		dropdownOpen = false;
		saved = false;
		error = '';
	}

	function selectOrganization(name: string) {
		const settings =
			organizations.find((candidate) => candidate.name === name) ??
			defaultOrganizationSettings;
		applyOrganization(settings);
	}

	function addOrganization() {
		dropdownOpen = false;
		nameDialogError = '';
		nameDialogMode = 'create';
	}

	function openEditOrganizationDialog() {
		if (isDefault) return;
		nameDialogError = '';
		nameDialogMode = 'edit';
	}

	function submitOrganizationName(name: string) {
		if (!name) {
			nameDialogError = 'Enter an organization name.';
			return;
		}
		const duplicate = organizations.some(
			(settings) =>
				settings.name.toLocaleLowerCase() === name.toLocaleLowerCase() &&
				(nameDialogMode === 'create' || settings.name !== organization)
		);
		if (duplicate) {
			nameDialogError = 'An organization with this name already exists.';
			return;
		}

		if (nameDialogMode === 'create') {
			applyOrganization({
				name,
				quorum: '',
				quickVoteOptions: ['Yes', 'No'],
				approvedMembers: []
			});
		} else {
			const oldName = organization;
			organization = name;
			organizations = organizations.map((settings) =>
				settings.name === oldName ? { ...settings, name } : settings
			);
			saved = false;
		}
		nameDialogMode = null;
		nameDialogError = '';
	}

	function requestOrganizationDeletion(name: string) {
		organizationPendingDelete = name;
		dropdownOpen = false;
	}

	function deleteOrganization() {
		if (!organizationPendingDelete || organizationPendingDelete === 'Default') return;
		const deletedName = organizationPendingDelete;
		organizations = organizations.filter((settings) => settings.name !== deletedName);
		saveDeletedOrganizationNames([
			...new Set([...loadDeletedOrganizationNames(), deletedName])
		]);
		if (organization === deletedName) {
			clearOrganizationSettings();
			applyOrganization(defaultOrganizationSettings);
		}
		organizationPendingDelete = null;
	}

	function keepDigits(event: Event) {
		defaultQuorum = (event.currentTarget as HTMLInputElement).value.replace(/\D/g, '').slice(0, 5);
		saved = false;
	}

	function keepQuickVoteOptions(event: Event) {
		quickVoteOptions = (event.currentTarget as HTMLInputElement).value
			.split(',')
			.slice(0, 10)
			.map((option) => option.slice(0, 36))
			.join(',');
		saved = false;
	}

	function keepApprovedMembers(event: Event) {
		approvedMembers = (event.currentTarget as HTMLTextAreaElement).value
			.split(',')
			.slice(0, 200)
			.map((andrewId) => andrewId.slice(0, 36))
			.join(',');
		saved = false;
	}

	function saveSettings(event: SubmitEvent) {
		event.preventDefault();
		if (isDefault) return;
		if (!organization.trim()) {
			error = 'Enter an organization name.';
			return;
		}
		const options = quickVoteOptions
			.split(',')
			.map((option) => option.trim())
			.filter(Boolean);
		if (options.length < 2) {
			error = 'Enter at least two Quick Vote options.';
			return;
		}
		if (new Set(options.map((option) => option.toLocaleLowerCase())).size !== options.length) {
			error = 'Each Quick Vote option must be unique.';
			return;
		}
		const savedSettings = {
			name: organization.trim(),
			quorum: defaultQuorum,
			quickVoteOptions: options,
			approvedMembers: approvedMembers
				.split(',')
				.map((andrewId) => andrewId.trim())
				.filter(Boolean)
		};
		saveOrganizationSettings(savedSettings);
		saveDeletedOrganizationNames(
			loadDeletedOrganizationNames().filter((name) => name !== savedSettings.name)
		);
		organizations = [
			...organizations.filter((settings) => settings.name !== savedSettings.name),
			savedSettings
		];
		error = '';
		saved = true;
	}
</script>

<svelte:head>
	<title>TartanVote | Session Configuration</title>
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
				<a class="menu-item" href="/host">Overview</a>
				<button class="menu-item" type="button" disabled title="Live View is available while a vote is running">Live View</button>
				<a class="menu-item active" href="/host/configuration" aria-current="page">Session Configuration</a>
				<button class="menu-item" type="button">Comprehensive Session Results</button>
			</nav>

			<button class="end-session" type="button">End session</button>
		</aside>

		<section class="configuration-page" aria-labelledby="configuration-title">
			<h1 id="configuration-title">Session Configuration</h1>

			<form class="configuration-card" onsubmit={saveSettings}>
				<header class="card-heading">
					<div>
						<h2>Customize Room (Optional)</h2>
						<p>Configure organization specific settings</p>
					</div>
					<button
						class="edit-organization"
						type="button"
						disabled={isDefault}
						onclick={openEditOrganizationDialog}
					>
						<img src={pencilUrl} alt="" />
						<span>{isDefault ? 'Default cannot be edited' : 'Edit Organization Name'}</span>
					</button>
				</header>

				{#if isDefault}
					<p class="default-notice">Default settings are read-only. Add another organization to create and save custom settings.</p>
				{/if}

				<div class="settings-grid">
					<div class="organization-settings">
						<label>
							<span>Selected Organization:</span>
							<small>Configure an organization or use Default settings</small>
							<div class="organization-picker">
								<button
									class="picker-trigger"
									type="button"
									aria-haspopup="listbox"
									aria-expanded={dropdownOpen}
									onclick={() => (dropdownOpen = !dropdownOpen)}
								>
									<span>{organization}</span><span aria-hidden="true">›</span>
								</button>

								{#if dropdownOpen}
									<div class="organization-menu" role="listbox" aria-label="Organizations">
										{#each organizations as settings}
											<div class:selected={settings.name === organization} class="organization-option">
												{#if settings.name !== 'Default'}
													<button
														class="delete-organization"
														type="button"
														aria-label={`Delete ${settings.name}`}
														onclick={() => requestOrganizationDeletion(settings.name)}
													>
														×
													</button>
												{:else}
													<span class="delete-placeholder" aria-hidden="true"></span>
												{/if}
												<button
													class="select-organization"
													type="button"
													role="option"
													aria-selected={settings.name === organization}
													onclick={() => selectOrganization(settings.name)}
												>
													{settings.name}
												</button>
											</div>
										{/each}
										<button class="add-organization" type="button" onclick={addOrganization}>+ Add another organization</button>
									</div>
								{/if}
							</div>
						</label>

						<label>
							<span>Default Organization Quorum:</span>
							<small>Input the default minimum number of active participants required to begin a vote.</small>
							<input
								value={defaultQuorum}
								inputmode="numeric"
								pattern="[0-9]*"
								maxlength="5"
								placeholder="N/A"
								aria-label="Default organization quorum"
								oninput={keepDigits}
								disabled={isDefault}
							/>
						</label>

						<label>
							<span>Quick Vote Voting Options:</span>
							<small>Input the default quick voting selections separated by a comma.</small>
							<input
								value={quickVoteOptions}
								maxlength="369"
								oninput={keepQuickVoteOptions}
								disabled={isDefault}
							/>
						</label>
					</div>

					<label class="member-settings">
						<span>Approved Voting Members:</span>
						<small>Input the AndrewID(s) of allowed participants separated by a comma. Leave blank to allow unrestricted session joining.</small>
						<textarea
							value={approvedMembers}
							maxlength="7399"
							placeholder="N/A"
							oninput={keepApprovedMembers}
							disabled={isDefault}
						></textarea>
					</label>
				</div>

				<div class="form-feedback" aria-live="polite">
					{#if error}<p class="form-error" role="alert">{error}</p>{/if}
					{#if saved}<p class="save-status">Settings saved.</p>{/if}
				</div>
				<button class="save-button" type="submit" disabled={isDefault}>Save Settings</button>
			</form>
		</section>
	</div>

	<AppFooter wide flow />
</main>

{#if organizationPendingDelete}
	<ConfirmDeleteOrganizationDialog
		organizationName={organizationPendingDelete}
		onclose={() => (organizationPendingDelete = null)}
		ondelete={deleteOrganization}
	/>
{/if}

{#if nameDialogMode}
	<OrganizationNameDialog
		mode={nameDialogMode}
		initialName={nameDialogMode === 'edit' ? organization : ''}
		error={nameDialogError}
		onclose={() => (nameDialogMode = null)}
		onsubmit={submitOrganizationName}
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
		min-height: calc(100svh - clamp(60px, 5.833vw, 112px) - 81px);
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
	.menu-item:disabled { color: var(--color-grey-100); cursor: not-allowed; opacity: 1; }
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

	.configuration-page {
		padding: clamp(16px, 1.719vw, 33px) clamp(19px, 1.823vw, 35px) clamp(10px, 0.938vw, 18px);
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: safe center;
	}
	h1 {
		width: 100%;
		margin: 0 0 clamp(6px, 1.042vw, 20px);
		font-size: clamp(25px, 2.344vw, 45px);
		font-weight: 600;
		line-height: clamp(32px, 2.917vw, 56px);
		text-align: center;
	}
	.configuration-card {
		width: 100%;
		max-width: 1470px;
		border: 2px solid var(--color-grey-200);
		border-radius: 10px;
		padding: clamp(12px, 1.979vw, 38px) clamp(20px, 2.083vw, 40px)
			clamp(22px, 2.083vw, 40px);
		background: var(--color-white);
	}
	.card-heading { display: flex; align-items: flex-start; justify-content: space-between; }
	.card-heading h2 {
		margin: 0 0 3px;
		color: var(--color-black);
		font-size: clamp(14px, 1.25vw, 24px);
		font-weight: 500;
		line-height: clamp(20px, 1.667vw, 32px);
	}
	.card-heading p,
	small {
		margin: 0;
		display: block;
		color: var(--color-grey-700);
		font-size: clamp(9px, 0.833vw, 16px);
		font-weight: 500;
		line-height: clamp(13px, 1.25vw, 24px);
	}
	.edit-organization {
		border: 0;
		padding: 0;
		display: inline-flex;
		align-items: center;
		gap: 6px;
		background: transparent;
		color: var(--color-grey-700);
		font: inherit;
		font-size: clamp(9px, 0.938vw, 18px);
		text-decoration: underline;
		cursor: pointer;
	}
	.edit-organization img { width: clamp(12px, 1.094vw, 21px); height: clamp(12px, 1.094vw, 21px); object-fit: contain; }
	.edit-organization:disabled { cursor: not-allowed; opacity: 0.72; }
	.default-notice {
		margin: 8px 0 -4px;
		color: var(--color-grey-600);
		font-size: clamp(9px, 0.833vw, 16px);
		font-style: italic;
		line-height: clamp(13px, 1.25vw, 24px);
	}

	.settings-grid {
		margin-top: clamp(14px, 1.458vw, 28px);
		display: grid;
		grid-template-columns: clamp(210px, 20.833vw, 400px) minmax(0, 1fr);
		gap: clamp(30px, 3.646vw, 70px);
	}
	.organization-settings { display: grid; gap: clamp(12px, 1.042vw, 20px); }
	.organization-picker {
		position: relative;
		width: clamp(160px, 15.781vw, 303px);
		margin-top: 5px;
	}
	.picker-trigger {
		width: 100%;
		height: clamp(28px, 2.604vw, 50px);
		border: 1px solid var(--color-black);
		border-radius: 10px;
		background: var(--color-white);
		color: var(--color-black);
		font: inherit;
		font-size: clamp(9px, 0.833vw, 16px);
		font-weight: 500;
	}
	.picker-trigger {
		padding: 0 14px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		cursor: pointer;
	}
	.organization-menu {
		position: absolute;
		z-index: 5;
		top: calc(100% + 1px);
		left: 0;
		width: 100%;
		padding: 2px 0;
		overflow: hidden;
		border: 0.5px solid var(--color-grey-100);
		border-radius: 5px;
		background: var(--color-white);
		box-shadow: 0 2px 3px rgb(0 0 0 / 0.25);
	}
	.organization-option {
		height: clamp(28px, 2.604vw, 50px);
		display: grid;
		grid-template-columns: 38px minmax(0, 1fr);
		align-items: center;
		background: var(--color-white);
	}
	.organization-option.selected,
	.organization-option:hover { background: var(--color-grey-100); }
	.organization-menu button {
		height: clamp(28px, 2.604vw, 50px);
		border: 0;
		display: flex;
		align-items: center;
		background: transparent;
		color: var(--color-black);
		font: inherit;
		font-size: clamp(9px, 0.833vw, 16px);
		font-weight: 500;
		cursor: pointer;
	}
	.organization-menu .select-organization {
		width: 100%;
		padding: 0 14px 0 0;
		text-align: left;
	}
	.organization-menu .delete-organization {
		width: 38px;
		padding: 0;
		justify-content: center;
		color: var(--color-grey-500);
		font-size: clamp(16px, 1.25vw, 24px);
		line-height: 1;
	}
	.organization-menu .delete-organization:hover { color: var(--color-red-500); }
	.delete-placeholder { width: 38px; }
	.organization-menu button.add-organization {
		width: 100%;
		padding: 0 14px;
		color: var(--color-grey-400);
		font-style: italic;
		text-align: left;
	}
	label > span {
		display: block;
		color: var(--color-black);
		font-size: clamp(9px, 0.833vw, 16px);
		font-weight: 500;
		line-height: clamp(13px, 1.25vw, 24px);
	}
	input,
	textarea {
		width: 100%;
		border: 1px solid var(--color-black);
		border-radius: 10px;
		background: var(--color-white);
		color: var(--color-black);
		font: inherit;
		font-size: clamp(9px, 0.833vw, 16px);
	}
	input {
		width: clamp(160px, 15.781vw, 303px);
		height: clamp(28px, 2.604vw, 50px);
		margin-top: 5px;
		padding: 0 14px;
	}
	input::placeholder,
	textarea::placeholder { color: var(--color-grey-400); opacity: 1; }
	input:disabled,
	textarea:disabled {
		background: var(--color-grey-50);
		color: var(--color-grey-400);
		cursor: not-allowed;
	}
	.member-settings textarea {
		height: clamp(182px, 17.813vw, 342px);
		min-height: clamp(182px, 17.813vw, 342px);
		max-height: 520px;
		margin-top: 10px;
		padding: 14px 16px;
		resize: vertical;
	}
	.form-feedback {
		min-height: 24px;
		margin-top: clamp(16px, 1.563vw, 30px);
		display: grid;
		place-items: center;
	}
	.save-button {
		width: clamp(268px, 26.042vw, 500px);
		height: clamp(34px, 3.125vw, 60px);
		margin: 0 auto;
		display: block;
		border: 0;
		border-radius: 100px;
		background: var(--color-red-600);
		box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: clamp(14px, 1.25vw, 24px);
		font-weight: 700;
		cursor: pointer;
	}
	.save-button:disabled {
		background: var(--color-grey-400);
		box-shadow: none;
		cursor: not-allowed;
	}
	.form-error {
		margin: 0;
		color: var(--color-red-700);
		font-size: clamp(11px, 0.729vw, 14px);
		text-align: center;
	}
	.save-status {
		margin: 0;
		color: var(--color-green-600);
		font-size: clamp(11px, 0.729vw, 14px);
		font-weight: 600;
	}
</style>
