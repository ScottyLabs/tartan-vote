<script lang="ts">
    import { onMount } from "svelte";
    import { devSignIn, fetchAuthStatus } from "../lib/auth-client";

    let {
        toVoter,
        toAdmin,
    }: {
        toVoter: (sessionCode: string) => void;
        toAdmin: (sessionCode: string) => void;
    } = $props();

    let sessionCode = $state("");

    let authStatus = $state<{
        logged_in: boolean;
        user_id: number;
        user_name: string;
        user_andrew_id: string;
        oidc_subject: string | null;
    } | null>(null);
    let authStatusError = $state<string | null>(null);

    let joinError = $state<string | null>(null);
    let joining = $state<boolean>(false);

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    onMount(() => {
        void (async () => {
            try {
                const status = await fetchAuthStatus();
                if (status) {
                    authStatus = {
                        logged_in: status.logged_in,
                        user_id: status.user_id ?? -1,
                        user_name: status.user_name ?? "",
                        user_andrew_id: status.user_andrew_id ?? "",
                        oidc_subject: null,
                    };
                } else {
                    authStatusError = "Unable to load auth status.";
                }
            } catch (error) {
                authStatusError = "Unable to load auth status.";
            }
        })();
    });

    async function handleJoinClick() {
        joining = true;
        sessionCode = sessionCode.toUpperCase();

        const response = await fetch(
            `${API_BASE}/session/join/${sessionCode}`,
            { cache: "no-store", credentials: "include" },
        );

        if (!response.ok) {
            joining = false;
            joinError = "Invalid session code";
            return;
        }

        toVoter(sessionCode);
    }

    async function handleSignInClick() {
        const status = await devSignIn();
        authStatus = {
            logged_in: status.logged_in,
            user_id: status.user_id ?? -1,
            user_name: status.user_name ?? "",
            user_andrew_id: status.user_andrew_id ?? "",
            oidc_subject: null,
        };
    }

    async function handleCreateSessionClick() {
        const response = await fetch(`${API_BASE}/session/create`, {
            cache: "no-store",
            credentials: "include",
        });

        if (!response.ok) {
            throw new Error(`Session creation failed: ${response.status}`);
        }

        const data = await response.json();
        toAdmin(data.session_code);
    }
</script>

<main>
    <div>
        <h1>CampusVoting</h1>
        <h3>Powered by ScottyLabs</h3>
        {#if authStatus}
            <h2>
                {authStatus.logged_in
                    ? `Welcome, ${authStatus.user_name}`
                    : "Not signed in"}
            </h2>
        {:else if authStatusError}
            <h2>{authStatusError}</h2>
        {/if}
    </div>

    <div></div>

    <div class="card">
        <input
            type="text"
            style="text-transform: uppercase;"
            bind:value={sessionCode}
            placeholder="Session Code"
            onkeydown={(event) => {
                if (event.key === "Enter") {
                    handleJoinClick();
                }
            }}
        />
        <button
            onclick={handleJoinClick}
            class="joinBtn"
            disabled={joining || !sessionCode.trim()}
        >
            {joining ? "JOINING..." : "JOIN SESSION"}
        </button>
    </div>

    {#if joinError}
        <p class="error">{joinError}</p>
    {/if}

    <div class="row">
        <h3 class="session">Want to create a session?</h3>
        {#if authStatus?.logged_in}
            <button onclick={handleCreateSessionClick} class="sessBtn"
                >Create Session</button
            >
        {:else}
            <button onclick={handleSignInClick} class="sessBtn"
                >Sign in to Create Session</button
            >
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
        border: 1px solid var(--color-border);
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
        background: var(--color-surface);

        box-shadow: var(--shadow-card);
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
        color: var(--color-on-primary);
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
        color: var(--color-danger);
        margin: 0.5em 0 0;
    }

    .session {
        margin-top: 1em;
        margin-bottom: 1em;
        color: var(--color-text);
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
