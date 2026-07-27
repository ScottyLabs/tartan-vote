<script lang="ts">
	import { onMount } from 'svelte';
	import closeUrl from '$lib/assets/host-close.svg?url';
	import trashUrl from '$lib/assets/host-trash.svg?url';
	import { loadOrganizationSettings } from '$lib/domain/organizationSettings';

	interface Props {
		kind: 'motion' | 'election';
		onclose: () => void;
		onsubmit?: (configuration: {
			kind: 'motion' | 'election';
			name: string;
			votingType: string;
			options: string[];
			displayLiveResults: boolean;
			allowProxyVoting: boolean;
			enableQuorum: boolean;
			quorum: string;
		}) => void;
	}

	let { kind, onclose, onsubmit = onclose }: Props = $props();
	let name = $state('');
	let votingType = $state('Standard');
	let options = $state<string[]>([]);
	let displayLiveResults = $state(false);
	let allowProxyVoting = $state(false);
	let enableQuorum = $state(false);
	let quorum = $state('');
	let error = $state('');

	const title = $derived(kind === 'motion' ? 'Motion Configuration' : 'Election Configuration');
	const noun = $derived(kind === 'motion' ? 'Motion' : 'Election');
	const nameExample = $derived(
		kind === 'motion' ? 'ex. Fund the spring student showcase' : 'ex. Best Rust StuCo Instructor'
	);

	onMount(() => {
		const organizationSettings = loadOrganizationSettings();
		options = kind === 'motion' ? ['Pass', 'Reject', ''] : ['', '', ''];
		quorum = organizationSettings.quorum;
		enableQuorum = Boolean(organizationSettings.quorum);
	});

	function updateOption(index: number, value: string) {
		options[index] = value.slice(0, 36);
	}

	function addOption() {
		options.push('');
	}

	function removeOption(index: number) {
		if (options.length <= 2) {
			error = 'Voting requires at least two options.';
			return;
		}
		options.splice(index, 1);
		error = '';
	}

	function submit(event: SubmitEvent) {
		event.preventDefault();
		if (!name.trim()) {
			error = `Enter a ${noun.toLowerCase()} name.`;
			return;
		}
		const completedOptions = options.map((option) => option.trim()).filter(Boolean);
		if (completedOptions.length < 2) {
			error = 'Enter at least two voting options.';
			return;
		}
		const normalizedOptions = completedOptions.map((option) => option.toLocaleLowerCase());
		if (new Set(normalizedOptions).size !== normalizedOptions.length) {
			error = 'Each voting option must be unique.';
			return;
		}
		if (enableQuorum && !quorum.trim()) {
			error = 'Enter a quorum count when quorum setup is enabled.';
			return;
		}
		error = '';
		onsubmit({
			kind,
			name: name.trim(),
			votingType,
			options: completedOptions,
			displayLiveResults,
			allowProxyVoting,
			enableQuorum,
			quorum
		});
	}
</script>

<div class="dialog-backdrop" role="presentation">
	<div class="configuration-dialog" role="dialog" aria-modal="true" aria-labelledby="configuration-title">
		<button class="close-button" type="button" aria-label={`Close ${noun.toLowerCase()} configuration`} onclick={onclose}>
			<img src={closeUrl} alt="" />
		</button>

		<h2 id="configuration-title">{title}</h2>

		<form onsubmit={submit}>
			<label class="name-field">
				<span>{noun} Name</span>
				<input
					bind:value={name}
					maxlength="98"
					placeholder={nameExample}
					required
				/>
			</label>

			<div class="settings-grid">
				<div class="ballot-settings">
					<label>
						<span>Voting Type</span>
						<select bind:value={votingType}>
							<option>Standard</option>
							<option>Secret</option>
							<option>Roll Call</option>
						</select>
					</label>

					<fieldset>
						<legend>Voting Options</legend>
						<div class="option-list">
							{#each options as option, index (index)}
								<div class="option-row">
									<input
										value={option}
										maxlength="36"
										placeholder={kind === 'motion'
											? index === 0
												? 'Pass'
												: index === 1
													? 'Reject'
													: `ex. Option ${index + 1}`
											: index < 2
												? `ex. Person${index + 1}`
												: `ex. Option ${index + 1}`}
										aria-label={`Voting option ${index + 1}`}
										oninput={(event) => updateOption(index, event.currentTarget.value)}
									/>
									{#if index >= 2}
										<button class="trash-button" type="button" aria-label={`Delete option ${index + 1}`} onclick={() => removeOption(index)}>
											<img src={trashUrl} alt="" />
										</button>
									{/if}
								</div>
							{/each}
						</div>
						<button class="add-option" type="button" onclick={addOption}>+ &nbsp; Add additional</button>
					</fieldset>
				</div>

				<div class="toggle-settings">
					<label>
						<span>Display Live Results</span>
						<input type="checkbox" role="switch" bind:checked={displayLiveResults} />
					</label>
					<label>
						<span>Allow Proxy Voting</span>
						<input type="checkbox" role="switch" bind:checked={allowProxyVoting} />
					</label>
					<label>
						<span>Enable Quorum Setup</span>
						<input type="checkbox" role="switch" bind:checked={enableQuorum} />
					</label>
					{#if enableQuorum}
						<input
							class="quorum-input"
							value={quorum}
							inputmode="numeric"
							pattern="[0-9]+"
							placeholder="ex. 20"
							aria-label="Quorum count"
							oninput={(event) => (quorum = event.currentTarget.value.replace(/\D/g, ''))}
							required
						/>
					{/if}
				</div>
			</div>

			{#if error}<p class="form-error" role="alert">{error}</p>{/if}
			<button class="push-button" type="submit">+ Push a {noun}</button>
		</form>
	</div>
</div>

<style>
	.dialog-backdrop {
		position: fixed;
		z-index: 20;
		inset: 0;
		display: grid;
		place-items: center;
		background: rgb(102 102 102 / 0.4);
	}

	.configuration-dialog {
		position: relative;
		width: min(calc(100% - 40px), 800px);
		min-height: 776px;
		padding: 74px 49px 42px;
		border: 1px solid var(--color-grey-600);
		border-radius: 10px;
		background: var(--color-white);
	}

	.close-button {
		position: absolute;
		top: 39px;
		right: 49px;
		width: 20px;
		height: 20px;
		border: 0;
		padding: 0;
		background: transparent;
		cursor: pointer;
	}

	.close-button img {
		width: 100%;
		height: 100%;
		display: block;
	}

	h2 {
		margin: 0 0 29px;
		font-size: 36px;
		font-weight: 600;
		line-height: 44px;
		text-align: center;
	}

	label > span,
	legend {
		margin-bottom: 12px;
		display: block;
		color: var(--color-slate-900);
		font-size: 20px;
		font-weight: 500;
		line-height: 28px;
	}

	input,
	select {
		height: 40px;
		border: 1px solid var(--color-slate-800);
		border-radius: 10px;
		background: var(--color-white);
		color: var(--color-black);
		font: inherit;
		font-size: 14px;
	}

	input::placeholder {
		color: var(--color-grey-500);
		font-style: italic;
		opacity: 1;
	}

	select {
		color: var(--color-black);
	}

	select option {
		color: var(--color-black);
	}

	.name-field input {
		width: 100%;
		padding: 0 20px;
	}

	.settings-grid {
		margin-top: 52px;
		display: grid;
		grid-template-columns: 344px 1fr;
		gap: 30px;
	}

	select {
		width: 270px;
		padding: 0 14px;
	}

	fieldset {
		margin: 26px 0 0;
		padding: 0;
		border: 0;
	}

	.option-list {
		display: grid;
		gap: 6px;
	}

	.option-row {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.option-row input {
		width: 270px;
		padding: 0 16px;
	}

	.trash-button {
		width: 28px;
		height: 28px;
		border: 0;
		padding: 0;
		background: transparent;
		cursor: pointer;
	}

	.trash-button img {
		width: 100%;
		height: 100%;
		display: block;
	}

	.add-option {
		width: 170px;
		height: 40px;
		margin-top: 14px;
		border: 1px solid var(--color-slate-900);
		border-radius: 999px;
		background: var(--color-white);
		color: var(--color-slate-800);
		font: inherit;
		font-size: 14px;
		cursor: pointer;
	}

	.toggle-settings {
		display: grid;
		align-content: start;
		gap: 15px;
	}

	.toggle-settings label {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 16px;
	}

	.toggle-settings label > span {
		margin: 0;
		color: var(--color-black);
		white-space: nowrap;
	}

	input[role='switch'] {
		width: 44px;
		height: 24px;
		margin: 0;
		border: 1px solid var(--color-black);
		border-radius: 999px;
		appearance: none;
		background: var(--color-white);
		cursor: pointer;
	}

	input[role='switch']::before {
		content: '';
		width: 18px;
		height: 18px;
		margin: 2px 3px;
		border-radius: 50%;
		display: block;
		background: var(--color-slate-900);
		transition: transform 150ms ease;
	}

	input[role='switch']:checked {
		background: var(--color-slate-900);
	}

	input[role='switch']:checked::before {
		transform: translateX(18px);
		background: var(--color-white);
	}

	.quorum-input {
		width: 175px;
		padding: 0 20px;
	}

	.form-error {
		margin: 15px 0 0;
		color: var(--color-red-700);
		text-align: center;
	}

	.push-button {
		width: 280px;
		height: 58px;
		margin: 48px auto 0;
		border: 0;
		border-radius: 999px;
		display: block;
		background: var(--color-red-600);
		box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: 18px;
		font-weight: 500;
		cursor: pointer;
	}

	@media (max-width: 760px) {
		.configuration-dialog {
			max-height: calc(100svh - 24px);
			padding: 54px 24px 28px;
			overflow-y: auto;
		}

		.close-button {
			top: 24px;
			right: 24px;
		}

		h2 {
			font-size: 28px;
			line-height: 36px;
		}

		.settings-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
