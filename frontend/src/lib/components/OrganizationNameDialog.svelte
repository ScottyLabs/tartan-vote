<script lang="ts">
	import { onMount } from 'svelte';
	import closeUrl from '$lib/assets/host-close.svg?url';

	interface Props {
		mode: 'create' | 'edit';
		initialName?: string;
		error?: string;
		onclose: () => void;
		onsubmit: (name: string) => void;
	}

	let { mode, initialName = '', error = '', onclose, onsubmit }: Props = $props();
	let name = $state('');

	onMount(() => {
		name = initialName;
	});

	function submit(event: SubmitEvent) {
		event.preventDefault();
		onsubmit(name.trim());
	}
</script>

<div class="dialog-backdrop" role="presentation">
	<div class="name-dialog" role="dialog" aria-modal="true" aria-labelledby="organization-dialog-title">
		<button class="close-button" type="button" aria-label="Close organization name dialog" onclick={onclose}>
			<img src={closeUrl} alt="" />
		</button>
		<h2 id="organization-dialog-title">{mode === 'create' ? 'Add Organization' : 'Edit Organization Name'}</h2>
		<form onsubmit={submit}>
			<label>
				<span>Organization Name</span>
				<input
					bind:value={name}
					maxlength="36"
					placeholder="ex. ScottyLabs Exec Board"
					autocomplete="off"
					required
				/>
			</label>
			{#if error}<p class="error" role="alert">{error}</p>{/if}
			<div class="dialog-actions">
				<button class="cancel" type="button" onclick={onclose}>Cancel</button>
				<button class="save" type="submit">{mode === 'create' ? 'Add Organization' : 'Save Name'}</button>
			</div>
		</form>
	</div>
</div>

<style>
	.dialog-backdrop {
		position: fixed;
		z-index: 30;
		inset: 0;
		display: grid;
		place-items: center;
		background: rgb(102 102 102 / 0.4);
	}
	.name-dialog {
		position: relative;
		width: min(calc(100% - 40px), 720px);
		min-height: 300px;
		padding: 48px 52px 32px;
		border: 1px solid var(--color-grey-600);
		border-radius: 10px;
		background: var(--color-white);
	}
	.close-button {
		position: absolute;
		top: 19px;
		right: 20px;
		width: 22px;
		height: 22px;
		border: 0;
		padding: 0;
		background: transparent;
		cursor: pointer;
	}
	.close-button img { width: 100%; height: 100%; display: block; }
	h2 { margin: 0 0 30px; font-size: 24px; line-height: 32px; text-align: center; }
	label span {
		margin-bottom: 10px;
		display: block;
		color: var(--color-black);
		font-size: 18px;
		font-weight: 500;
		line-height: 26px;
	}
	input {
		width: 100%;
		height: 50px;
		border: 1px solid var(--color-slate-800);
		border-radius: 10px;
		padding: 0 16px;
		background: var(--color-white);
		color: var(--color-black);
		font: inherit;
		font-size: 16px;
	}
	input::placeholder { color: var(--color-grey-400); font-style: italic; opacity: 1; }
	.error { margin: 8px 0 0; color: var(--color-red-700); font-size: 14px; text-align: center; }
	.dialog-actions { margin-top: 32px; display: flex; justify-content: center; gap: 50px; }
	.dialog-actions button {
		width: 180px;
		height: 40px;
		border: 0;
		border-radius: 999px;
		box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: 18px;
		font-weight: 700;
		cursor: pointer;
	}
	.cancel { background: var(--color-slate-700); }
	.save { background: var(--color-green-400); }
</style>
