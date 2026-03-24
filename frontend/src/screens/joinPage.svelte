<script lang="ts">
    import { onMount } from 'svelte';

    let {
        joinEvent,
        joinError,
        joining,
        toVoter,
        toAdmin,
    }: {
        joinEvent: (sessionCode: string) => Promise<void>;
        joinError: string | null;
        joining: boolean;
        toVoter: () => void;
        toAdmin: () => void;
    } = $props();

    let sessionID = $state("");
    let authStatus = $state<{ logged_in: boolean; user_id: number; user_name: string; user_andrew_id: string; oidc_subject: string | null } | null>(null);
    let authStatusError = $state<string | null>(null);

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    onMount(() => {
        void (async () => {
            try {
                const response = await fetch(`${API_BASE}/auth/status`, {
                    cache: 'no-store',
                    credentials: 'include',
                });

                if (!response.ok) {
                    throw new Error(`Auth status failed: ${response.status}`);
                }

                authStatus = await response.json();
            } catch (error) {
                authStatusError = 'Unable to load auth status.';
            }
        })();
    });

    function handleJoinClick() {
        void joinEvent(sessionID);
    }

    function handleSignInClick() {
        const redirectUri = encodeURIComponent(window.location.origin);
        window.location.href = `${API_BASE}/auth/login?redirect_uri=${redirectUri}`;
    }
</script>

<main>
    <div>
        <h1>CampusVoting</h1>
        <h3>Powered by ScottyLabs</h3>
        {#if authStatus}
            <h2>{authStatus.logged_in ? `Welcome, ${authStatus.user_name}` : 'Not signed in'}</h2>
        {:else if authStatusError}
            <h2>{authStatusError}</h2>
        {/if}
    </div>

    <div></div>

    <div class="card">
        <input type="text" bind:value={sessionID} placeholder="Session ID" />
        <button onclick={handleJoinClick} class="joinBtn" disabled={joining || !sessionID.trim()}>
            {joining ? 'JOINING...' : 'JOIN SESSION'}
        </button>
    </div>

    {#if joinError}
        <p class="error">{joinError}</p>
    {/if}

    <div class="row">
        <h3 class="session">Want to create a session?</h3>
        {#if authStatus?.logged_in}
            <button onclick={toAdmin} class="sessBtn">Create Session</button>
        {:else}
            <button onclick={handleSignInClick} class="sessBtn">Sign in to Create Session</button>
        {/if}
    </div>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    input {
        width: 100%;
        height: 50px;
        padding: 10px;
        border-radius: 6px;
        border: 1px solid #ccc;
        box-sizing: border-box;
        font-size: 20px;
        margin-bottom: 0em;
    }

    .card {
        width: "420";
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1em;

        padding: 2rem;
        border-radius: 12px;
        background: #e0e0e0;

        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    }

    .row {
        display: flex;
        gap: 1em;
    }

    .sessBtn {
        width: 100%;
        margin-top: none;
        background-color: transparent;
        color: var(--colors-primary);
        border: none;
        border-radius: 0px;
        font-size: 20px;
        padding: 0px 0px;
        cursor: pointer;
    }

    .joinBtn {
        margin-top: 0em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }

    .joinBtn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }

    .joinBtn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .error {
        color: #b00020;
        margin: 0.5em 0 0;
    }

    .session {
        margin-top: 1em;
        margin-bottom: 1em;
        color: black;
    }

    h1 {
        margin-top: 0.3em;
        margin-bottom: 0.3em;
        color: var(--colors-primary);
    }

    h2 {
        margin-top: 0.3em;
        margin-bottom: 0.3em;
        color: var(--colors-primary);
    }

    h3 {
        margin-top: 0.5em;
        margin-bottom: 2em;
        color: var(--colors-text);
        white-space: nowrap;
    }
</style>
