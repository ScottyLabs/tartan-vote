<script lang="ts">
	import type { HostLiveOption } from '$lib/domain/hostLive';

	interface Props {
		options: HostLiveOption[];
		totalVotes: number;
		shared: boolean;
		onconfirm: () => void;
		ontoggleshare: () => void;
	}

	let { options, totalVotes, shared, onconfirm, ontoggleshare }: Props = $props();

	const winningOption = $derived(
		options.reduce((winner, option) => (option.votes > winner.votes ? option : winner), options[0])
	);
</script>

<div class="dialog-backdrop" role="presentation">
	<div
		class="final-dialog"
		role="dialog"
		aria-modal="true"
		aria-labelledby="final-result-title"
	>
		<header>
			<h2 id="final-result-title">Final Result: {winningOption.label}</h2>
			<p>Total Votes: {totalVotes}</p>
		</header>

		<div class="results-list">
			{#each options as option (option.id)}
				<article class="result-row">
					<div class="result-heading">
						<strong>{option.label}({option.votes})</strong>
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

		<div class="actions">
			<button type="button" onclick={onconfirm}>Confirm</button>
			<button
				class:shared
				type="button"
				aria-pressed={shared}
				onclick={ontoggleshare}
			>
				{shared ? 'Unshare Final Results' : 'Share Final Results'}
			</button>
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
	.final-dialog {
		width: min(1200px, calc(100vw - 40px));
		min-height: 630px;
		padding: clamp(48px, 4.063vw, 78px) 20px 45px;
		border: 1px solid var(--color-grey-500);
		border-radius: 10px;
		display: flex;
		flex-direction: column;
		background: var(--color-white);
		color: var(--color-grey-900);
	}
	header { text-align: center; }
	header h2 {
		margin: 0;
		font-size: clamp(30px, 2.344vw, 45px);
		font-weight: 600;
		line-height: 1.25;
	}
	header p {
		margin: 10px 0 0;
		font-size: clamp(16px, 1.042vw, 20px);
		font-weight: 500;
		line-height: 1.4;
	}
	.results-list {
		width: min(100%, 859px);
		margin: 20px auto 0;
		display: flex;
		flex-direction: column;
		gap: 30px;
	}
	.result-heading {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 20px;
		font-size: clamp(16px, 1.042vw, 20px);
		line-height: 1.4;
	}
	.result-heading strong:first-child { min-width: 0; overflow-wrap: anywhere; }
	.result-heading strong:last-child {
		flex: 0 0 auto;
		font-size: clamp(19px, 1.25vw, 24px);
		line-height: 1.34;
	}
	.progress-track {
		width: 100%;
		height: 8px;
		margin-top: 4px;
		border-radius: 30px;
		overflow: hidden;
		background: var(--color-grey-200);
	}
	.progress-track span { height: 100%; border-radius: inherit; display: block; }
	.progress-track .green { background: var(--color-green-300); }
	.progress-track .yellow { background: var(--color-yellow-300); }
	.progress-track .blue { background: var(--color-blue-300); }
	.progress-track .red { background: var(--color-red-500); }
	.progress-track .purple { background: var(--color-purple-300); }
	.actions {
		margin-top: auto;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 22px;
	}
	.actions button {
		width: 315px;
		height: 63px;
		border: 0;
		border-radius: 100px;
		box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		background: var(--color-red-500);
		color: var(--color-grey-50);
		font: inherit;
		font-size: 24px;
		font-weight: 700;
		cursor: pointer;
	}
	.actions button.shared { background: var(--color-slate-700); }

	@media (max-height: 760px) {
		.final-dialog {
			max-height: calc(100svh - 40px);
			min-height: 0;
			overflow-y: auto;
		}
	}
</style>
