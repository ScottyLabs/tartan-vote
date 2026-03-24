<script lang="ts">
    import { onMount } from 'svelte';
    import logo from "../lib/images/logoplaceholder.png";

    type Props = {
        onNext: () => void;
    };

    let { onNext }: Props = $props();

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    onMount(() => {
        void (async () => {
            try {
                const response = await fetch(`${API_BASE}/auth/status`, {
                    cache: 'no-store',
                    credentials: 'include',
                });

                if (!response.ok) {
                    return;
                }

                const status: { logged_in: boolean } = await response.json();
                if (status.logged_in) {
                    onNext();
                }
            } catch (error) {
            }
        })();
    });

    async function handleClick() {
        const redirectUri = encodeURIComponent(window.location.origin);
        window.location.href = `${API_BASE}/auth/login?redirect_uri=${redirectUri}`;
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
