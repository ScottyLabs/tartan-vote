<script lang="ts">
    import { onMount } from "svelte";
    import { devSignIn, fetchAuthStatus } from "../lib/auth-client";
    import logo from "../lib/images/logoplaceholder.png";

    type Props = {
        onNext: () => void;
    };

    let { onNext }: Props = $props();
    let signingIn = $state(false);
    let error = $state<string | null>(null);

    onMount(() => {
        void (async () => {
            try {
                const status = await fetchAuthStatus();
                if (status?.logged_in) {
                    onNext();
                }
            } catch (error) {}
        })();
    });

    async function handleClick() {
        signingIn = true;
        error = null;
        try {
            await devSignIn();
            onNext();
        } catch (signInError) {
            error = "Dev sign-in failed. Is DEV_AUTH_BYPASS enabled on the backend?";
            signingIn = false;
        }
    }
</script>

<main>
    <div>
        <h1>CampusVoting</h1>
        <h3>Powered by ScottyLabs</h3>
    </div>
    <img
        src={logo}
        alt="CampusVoting Logo"
        class="logo"
        width="350px"
        height="350px"
    />
    <button onclick={handleClick} class="authBtn" disabled={signingIn}>
        {signingIn ? "SIGNING IN..." : "SIGN IN (DEV)"}
    </button>
    {#if error}
        <p class="error">{error}</p>
    {/if}
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .authBtn {
        margin-top: 1em;
        background-color: var(--colors-primary);
        color: var(--color-on-primary);
        border: none;
        border-radius: 8px;
        font-size: 20px;
        padding: 10px 35px;
        cursor: pointer;
    }

    .authBtn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }

    .authBtn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .error {
        color: var(--color-danger);
        margin-top: 1em;
    }

    h1 {
        margin-top: 0.3em;
        margin-bottom: 0.3em;
        color: var(--colors-primary);
    }

    h3 {
        margin-top: 0.5em;
        color: var(--colors-text);
    }
</style>
