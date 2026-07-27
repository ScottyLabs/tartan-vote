<script lang="ts">
	import closeUrl from '$lib/assets/host-close.svg?url';

	interface Props {
		requester?: string;
		proxyVotes?: string[];
		onclose: () => void;
		onaccept?: () => void;
		ondecline?: () => void;
		onkick?: () => void;
	}

	let {
		requester = 'Scottylabs1',
		proxyVotes = [
			'scottylabs1234567',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67',
			'scottylabs67'
		],
		onclose,
		onaccept = onclose,
		ondecline = onclose,
		onkick = onclose
	}: Props = $props();
</script>

<div class="dialog-backdrop" role="presentation">
	<div class="proxy-dialog" role="dialog" aria-modal="true" aria-labelledby="proxy-dialog-title">
		<button class="close-button" type="button" aria-label="Close proxy request" onclick={onclose}>
			<img src={closeUrl} alt="" />
		</button>
		<h2 id="proxy-dialog-title">{requester}’s Requested Proxy Votes:</h2>
		<p>{proxyVotes.join(', ')}</p>
		<div class="dialog-actions">
			<button class="accept" type="button" onclick={onaccept}>Accept Proxy</button>
			<button class="decline" type="button" onclick={ondecline}>Decline Proxy</button>
			<button class="kick" type="button" onclick={onkick}>Kick</button>
		</div>
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

	.proxy-dialog {
		position: relative;
		width: min(calc(100% - 40px), 1200px);
		min-height: 298px;
		padding: 48px 38px 24px;
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

	.close-button img {
		width: 100%;
		height: 100%;
		display: block;
	}

	h2 {
		margin: 0;
		font-size: 24px;
		line-height: 32px;
	}

	p {
		max-width: 1120px;
		margin: 27px auto 45px;
		font-size: 18px;
		line-height: 26px;
	}

	.dialog-actions {
		display: flex;
		justify-content: center;
		gap: 50px;
	}

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

	.accept { background: var(--color-green-400); }
	.decline { background: var(--color-red-400); }
	.kick { background: var(--color-slate-700); }

	@media (max-width: 700px) {
		.proxy-dialog {
			padding-inline: 24px;
		}

		.dialog-actions {
			flex-wrap: wrap;
			gap: 12px;
		}
	}
</style>
