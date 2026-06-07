<script lang="ts">
    import { onMount } from "svelte";
    import { authClient } from "../lib/auth-client";
    import logo from "../lib/images/logoplaceholder.png";

    type Props = {
        onNext: () => void;
    };

    let { onNext }: Props = $props();

    const BETTER_AUTH_PROVIDER_ID =
        import.meta.env.VITE_BETTER_AUTH_PROVIDER_ID || "cmu-sso";

    onMount(() => {
        void (async () => {
            try {
                const { data } = await authClient.getSession();
                if (data?.user) {
                    onNext();
                }
            } catch (error) {}
        })();
    });

    async function handleClick() {
        try {
            await authClient.signIn.oauth2({
                providerId: BETTER_AUTH_PROVIDER_ID,
                callbackURL: window.location.origin,
            });
        } catch (error) {
            console.error("SSO sign-in failed", error);
            alert("Sign-in failed. Please try again or contact ScottyLabs.");
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
    <button onclick={handleClick} class="authBtn">
        SIGN IN WITH CMU SSO
    </button>
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
        color: white;
        border: none;
        border-radius: 8px;
        font-size: 20px;
        padding: 10px 35px;
        cursor: pointer;
    }

    .authBtn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
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
