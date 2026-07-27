<script lang="ts">
	import closeUrl from '$lib/assets/host-close.svg?url';

	interface Props {
		onclose: () => void;
		onconfirm: () => void;
	}

	let { onclose, onconfirm }: Props = $props();
</script>

<div class="dialog-backdrop" role="presentation">
	<div
		class="end-session-dialog"
		role="alertdialog"
		aria-modal="true"
		aria-labelledby="end-session-title"
		aria-describedby="end-session-description"
	>
		<button class="close-button" type="button" aria-label="Close" onclick={onclose}>
			<img src={closeUrl} alt="" />
		</button>

		<div class="copy">
			<h2 id="end-session-title">Are you sure you want to end this session?</h2>
			<p id="end-session-description">
				All participants will leave the session and will no longer be able to vote. This action
				cannot be undone.
			</p>
		</div>

		<div class="actions">
			<button class="cancel" type="button" onclick={onclose}>Cancel</button>
			<button class="confirm" type="button" onclick={onconfirm}>End Session</button>
		</div>
	</div>
</div>

<style>
	.dialog-backdrop {
		position: fixed;
		inset: 0;
		z-index: 20;
		padding: 20px;
		display: grid;
		place-items: center;
		background: rgb(102 102 102 / 0.4);
	}

	.end-session-dialog {
		position: relative;
		width: min(1200px, calc(100vw - 40px));
		min-height: 300px;
		padding: 60px 20px 30px;
		border: 1px solid var(--color-grey-500);
		border-radius: 10px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 32px;
		background: var(--color-white);
		color: var(--color-grey-900);
	}

	.close-button {
		position: absolute;
		top: 16px;
		right: 20px;
		width: 30px;
		height: 30px;
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

	.copy {
		text-align: center;
	}

	.copy h2 {
		margin: 0;
		font-size: clamp(18px, 1.25vw, 24px);
		font-weight: 500;
		line-height: 1.34;
	}

	.copy p {
		margin: 25px 0 0;
		font-size: clamp(15px, 1.042vw, 20px);
		font-weight: 500;
		line-height: 1.4;
	}

	.actions {
		display: flex;
		justify-content: center;
		gap: 50px;
	}

	.actions button {
		width: 180px;
		height: 40px;
		border: 0;
		border-radius: 100px;
		box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		color: var(--color-grey-50);
		font: inherit;
		font-size: 18px;
		font-weight: 500;
		cursor: pointer;
	}

	.cancel {
		background: var(--color-slate-700);
	}

	.confirm {
		background: var(--color-red-500);
	}
</style>
