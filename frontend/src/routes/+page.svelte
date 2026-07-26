<script lang="ts">
	import { apiUrl } from '$lib/api/base';
	import AppFooter from '$lib/components/AppFooter.svelte';
	import logoUrl from '$lib/assets/tartanvote-logo.svg?url';

	function signIn() {
		window.location.href = apiUrl('/auth/login');
	}
</script>

<svelte:head>
	<title>TartanVote</title>
</svelte:head>

<main class="signin-page">
	<section class="signin-content" aria-labelledby="app-title">
		<div class="brand-lockup">
			<img class="brand-mark" src={logoUrl} alt="" width="68" height="51" />
			<h1 id="app-title"><span style="color: var(--color-black);">Tartan</span>Vote</h1>
		</div>

		<p class="description">An election management tool for proxy voting and campus governance.</p>

		<section class="signin-card" aria-label="Sign in with CMU account">
			<div class="signin-copy">
				<h2>Sign in</h2>
				<p>with your CMU Account. You'll return to this page after authenticating.</p>
			</div>

			<button type="button" class="sso-button" onclick={signIn}>Sign in with CMU SSO</button>

			<p class="auth-note">
				CMU authentication is required to vote. Only @andrew.cmu.edu accounts can join sessions.
			</p>
		</section>
	</section>

	<AppFooter wide />
</main>

<style>
	.signin-page {
		position: relative;
		width: min(100%, 393px);
		min-height: 100svh;
		margin: 0 auto;
		overflow: hidden;
		background: var(--gradient-screen-signin);
		color: var(--color-grey-900);
	}

	.signin-content {
		width: 100%;
		padding: 186px 21px 120px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
	}

	.brand-lockup {
		width: 300px;
		height: 54px;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
	}

	.brand-mark {
		width: 68px;
		height: 51px;
		flex: 0 0 auto;
		display: block;
	}

	h1 {
		margin: 0;
		color: var(--color-red-600);
		font-family: var(--font-brand);
		font-size: 52.05px;
		font-weight: 400;
		line-height: 1;
		letter-spacing: 0;
		-webkit-text-stroke: 8px var(--color-white);
		paint-order: stroke fill;
	}

	.description {
		width: 345px;
		margin: 4px 0 0;
		color: var(--color-grey-600);
		font-size: 16px;
		font-weight: 400;
		line-height: 20px;
		text-align: center;
	}

	.signin-card {
		width: 350px;
		min-height: 225px;
		margin-top: 0;
		padding: 29px 16px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 20px;
		background: var(--color-white);
		border: 1px solid var(--color-grey-200);
		border-radius: 4px;
	}

	.signin-copy {
		width: 274px;
		display: flex;
		flex-direction: column;
		gap: 5px;
		align-self: flex-start;
		margin-left: 6px;
	}

	h2 {
		margin: 0;
		color: var(--color-black);
		font-size: 18px;
		font-weight: 600;
		line-height: 22px;
		letter-spacing: 0;
	}

	.signin-copy p {
		width: 324px;
		margin: 0;
		color: var(--color-grey-900);
		font-size: 10px;
		font-weight: 500;
		line-height: 11.2px;
	}

	.sso-button {
		width: 258px;
		height: 39px;
		border: 0;
		border-radius: 999px;
		background: var(--color-red-600);
		box-shadow: 0 2.2px 2.2px rgb(0 0 0 / 0.25);
		color: var(--color-white);
		font: inherit;
		font-size: 14px;
		font-weight: 600;
		line-height: 17.5px;
		cursor: pointer;
	}

	.sso-button:hover {
		background: var(--color-red-700);
	}

	.sso-button:focus-visible {
		outline: 3px solid color-mix(in srgb, var(--color-red-600), transparent 75%);
		outline-offset: 3px;
	}

	.auth-note {
		width: 273px;
		min-height: 38px;
		margin: 0;
		padding: 5px 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-grey-50);
		border-radius: 2px;
		color: var(--color-grey-600);
		font-size: 8.75px;
		line-height: 13.1px;
		text-align: center;
	}

	@media (min-width: 640px) {
		.signin-page {
			width: 100%;
			min-height: 100svh;
			margin: 0;
			border-radius: 0;
			box-shadow: none;
		}

		.signin-content {
			min-height: calc(100svh - 81px);
			padding: clamp(88px, 12vh, 120px) 24px 128px;
			gap: 18px;
		}

		.brand-lockup {
			width: min(100%, 390px);
			height: 72px;
			gap: 8px;
		}

		.brand-mark {
			width: 82px;
			height: 62px;
		}

		h1 {
			font-size: 64px;
		}

		.description {
			width: min(calc(100vw - 48px), 760px);
			margin-top: 8px;
			font-size: 22px;
			line-height: 30px;
			text-wrap: balance;
		}

		.signin-card {
			width: min(calc(100vw - 48px), 620px);
			min-height: 310px;
			margin-top: 22px;
			padding: 34px 28px;
			align-items: flex-start;
			gap: 24px;
			border-radius: 8px;
		}

		.signin-copy {
			width: 100%;
			margin-left: 0;
			gap: 5px;
		}

		h2 {
			font-size: 30px;
			line-height: 38px;
		}

		.signin-copy p {
			width: auto;
			font-size: 14px;
			line-height: 20px;
			text-wrap: balance;
		}

		.sso-button {
			width: min(100%, 420px);
			height: 50px;
			margin: 26px auto 0;
			font-size: 18px;
			line-height: 24px;
			box-shadow: 0 3px 2px rgb(0 0 0 / 0.25);
		}

		.auth-note {
			width: min(100%, 420px);
			min-height: 54px;
			margin: 0 auto;
			padding: 8px 12px;
			font-size: 12px;
			line-height: 18px;
			border-radius: 4px;
		}

	}

	@media (min-width: 1100px) {
		.signin-content {
			padding: clamp(96px, 13.5vh, 146px) 24px 140px;
			gap: 0;
		}

		.brand-lockup {
			width: min(100%, 545px);
			height: 100px;
			gap: 10px;
		}

		.brand-mark {
			width: 119px;
			height: 90px;
		}

		h1 {
			font-size: 96px;
		}

		.description {
			width: min(calc(100vw - 48px), 1212px);
			margin-top: 32px;
			font-size: 36px;
			line-height: 44px;
		}

		.signin-card {
			width: min(calc(100vw - 48px), 850px);
			min-height: 380px;
			margin-top: 54px;
			padding: 38px 37px;
			gap: 0;
			border-radius: 10px;
		}

		h2 {
			font-size: 45px;
			line-height: 56px;
		}

		.signin-copy p {
			font-size: 20px;
			line-height: 28px;
		}

		.sso-button {
			width: min(100%, 500px);
			height: 60px;
			margin-top: 84px;
			font-size: 24px;
			font-weight: 700;
			line-height: 32px;
			box-shadow: 0 4px 2px rgb(0 0 0 / 0.25);
		}

		.auth-note {
			width: min(100%, 500px);
			min-height: 70px;
			margin-top: 30px;
			padding: 10px 14px;
			font-size: 16px;
			font-weight: 500;
			line-height: 24px;
		}
	}

	@media (max-width: 374px) {
		.signin-content {
			padding-inline: 14px;
		}

		.brand-lockup,
		.description,
		.signin-card {
			width: 100%;
		}

		h1 {
			font-size: 46px;
		}

		.signin-copy,
		.signin-copy p,
		.sso-button,
		.auth-note {
			width: 100%;
		}
	}
</style>
