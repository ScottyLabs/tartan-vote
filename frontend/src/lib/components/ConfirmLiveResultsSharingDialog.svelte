<script lang="ts">
	import closeUrl from '$lib/assets/host-close.svg?url';

	interface Props {
		currentlyShared: boolean;
		onclose: () => void;
		onconfirm: () => void;
	}

	let { currentlyShared, onclose, onconfirm }: Props = $props();
</script>

<div class="dialog-backdrop" role="presentation">
	<div
		class="confirmation-dialog"
		role="dialog"
		aria-modal="true"
		aria-labelledby="sharing-title"
		aria-describedby="sharing-description"
	>
		<button class="close-button" type="button" aria-label="Close" onclick={onclose}>
			<img src={closeUrl} alt="" />
		</button>

		<div class="copy">
			<h2 id="sharing-title">
				Are you sure you want to {currentlyShared ? 'unshare' : 'share'} live results?
			</h2>
			<p id="sharing-description">
				{#if currentlyShared}
					Voters will no longer be able to see the live results. You can share them again at any
					time while this voting instance is active.
				{:else}
					Voters will be able to see the live vote totals and results as they update. You can
					unshare them again at any time.
				{/if}
			</p>
		</div>

		<div class="actions">
			<button class="cancel" type="button" onclick={onclose}>Cancel</button>
			<button class="confirm" type="button" onclick={onconfirm}>Confirm</button>
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
	.confirmation-dialog {
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
	.close-button img { width: 100%; height: 100%; display: block; }
	.copy { text-align: center; }
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
		gap: clamp(24px, 2.604vw, 50px);
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
	.cancel { background: var(--color-slate-700); }
	.confirm { background: var(--color-red-500); }
</style>
