<script lang="ts">
	import heartUrl from '$lib/assets/heart.svg?url';

	let { wide = false, flow = false } = $props();
</script>

{#if flow}
	<div class="footer-spacer" aria-hidden="true"></div>
{/if}

<footer class="footer" class:wide={wide} class:fixed={flow}>
	<p>
		Designed, developed and maintained with
		<span class="heart" aria-label="love">
			<img src={heartUrl} alt="" />
		</span>
		by ScottyLabs.
	</p>

	<a class="feedback-link" href="https://docs.google.com/forms/u/2/d/e/1FAIpQLSdyrmvSSwyizNVZ97jyVT-r8D-bNVg_x0DYFBecJ9Mwhhteug/viewform?usp=send_form">
		<svg viewBox="0 0 20 20" aria-hidden="true">
			<path d="M4 4.5h12a1.5 1.5 0 0 1 1.5 1.5v6.7a1.5 1.5 0 0 1-1.5 1.5H9.2L5.5 17v-2.8H4A1.5 1.5 0 0 1 2.5 12.7V6A1.5 1.5 0 0 1 4 4.5Z" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round" />
			<path d="M6 8h8M6 11h5" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
		</svg>
		Feedback
	</a>
</footer>

<style>
	.footer {
		position: absolute;
		left: 0;
		right: 0;
		bottom: 0;
		height: 81px;
		padding: 16px 20px 15px 24px;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 18px;
		background: var(--color-grey-50);
		border-top: 1px solid var(--color-grey-400);
		color: var(--color-slate-800);
	}

	.footer.fixed {
		position: fixed;
		z-index: 10;
		bottom: 0;
	}

	.footer-spacer {
		width: 100%;
		height: 81px;
		flex: 0 0 81px;
	}

	.footer p {
		width: 200px;
		margin: 0;
		font-size: 11.33px;
		font-weight: 400;
		line-height: 17px;
	}

	.heart {
		display: inline-flex;
		width: 14.26px;
		height: 16.29px;
		margin: 0 2px;
		vertical-align: -3px;
	}

	.heart img {
		width: 100%;
		height: 100%;
		display: block;
	}

	.feedback-link {
		width: 107px;
		height: 38px;
		padding: 9px 15px 9px 16px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		border: 1px solid var(--color-slate-400);
		border-radius: 999px;
		color: var(--color-slate-800);
		font-size: 11.33px;
		font-weight: 600;
		line-height: 17px;
		text-decoration: none;
	}

	.feedback-link:focus-visible {
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 75%);
		outline-offset: 3px;
	}

	.feedback-link svg {
		width: 20px;
		height: 20px;
		flex: 0 0 auto;
	}

	@media (min-width: 640px) {
		.footer.wide {
			height: 81px;
			padding: 21px 24px 20px;
		}

		.footer.wide p {
			width: auto;
			font-size: 14px;
			line-height: 20px;
			white-space: nowrap;
		}

		.footer.wide .heart {
			width: 17.5px;
			height: 20px;
			margin: 0 3px;
			vertical-align: -4px;
		}

		.footer.wide .feedback-link {
			width: auto;
			height: 38px;
			gap: 8px;
			font-size: 14px;
			font-weight: 600;
			line-height: 20px;
		}
	}

	@media (min-width: 1100px) {
		.footer.wide {
			padding: 21px 33px 20px;
		}

		.footer.wide p {
			font-size: 16px;
			line-height: 24px;
		}

		.footer.wide .heart {
			width: 20.13px;
			height: 23px;
			margin: 0 4px;
			vertical-align: -5px;
		}
	}
</style>
