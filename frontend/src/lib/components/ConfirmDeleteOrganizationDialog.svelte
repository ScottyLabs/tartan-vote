<script lang="ts">
	import closeUrl from '$lib/assets/host-close.svg?url';

	interface Props {
		organizationName: string;
		onclose: () => void;
		ondelete: () => void;
	}

	let { organizationName, onclose, ondelete }: Props = $props();
</script>

<div class="dialog-backdrop" role="presentation">
	<div class="delete-dialog" role="alertdialog" aria-modal="true" aria-labelledby="delete-title" aria-describedby="delete-description">
		<button class="close-button" type="button" aria-label="Cancel deleting organization" onclick={onclose}>
			<img src={closeUrl} alt="" />
		</button>
		<h2 id="delete-title">Delete {organizationName}?</h2>
		<p id="delete-description">Are you sure you want to delete this organization and its saved voting settings?</p>
		<div class="dialog-actions">
			<button class="cancel" type="button" onclick={onclose}>Cancel</button>
			<button class="delete" type="button" onclick={ondelete}>Delete</button>
		</div>
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
	.delete-dialog {
		position: relative;
		width: min(calc(100% - 40px), 720px);
		min-height: 270px;
		padding: 48px 38px 30px;
		border: 1px solid var(--color-grey-600);
		border-radius: 10px;
		background: var(--color-white);
		text-align: center;
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
	h2 { margin: 0; font-size: 24px; line-height: 32px; }
	p { margin: 27px auto 42px; color: var(--color-grey-700); font-size: 18px; line-height: 26px; }
	.dialog-actions { display: flex; justify-content: center; gap: 50px; }
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
	.delete { background: var(--color-red-400); }
</style>
