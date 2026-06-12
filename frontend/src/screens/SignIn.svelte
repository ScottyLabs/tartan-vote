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

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    let bypassName = $state("Demo User");
    let bypassAndrewId = $state("demo");
    let bypassLoading = $state(false);
    let bypassError = $state<string | null>(null);

    onMount(() => {
        void (async () => {
            try {
                const res = await fetch(`${API_BASE}/auth/status`, {
                    cache: "no-store",
                    credentials: "include",
                });
                if (res.ok && (await res.json()).logged_in) {
                    onNext();
                    return;
                }

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

    async function handleBypass() {
        bypassLoading = true;
        bypassError = null;

        try {
            const response = await fetch(`${API_BASE}/auth/bypass/login`, {
                method: "POST",
                cache: "no-store",
                credentials: "include",
                headers: { "content-type": "application/json" },
                body: JSON.stringify({
                    name: bypassName,
                    andrew_id: bypassAndrewId,
                }),
            });

            if (!response.ok) {
                bypassError = `Bypass sign-in failed (${response.status}).`;
                return;
            }

            onNext();
        } catch (error) {
            console.error("Bypass sign-in failed", error);
            bypassError = "Bypass sign-in failed. Is the backend running?";
        } finally {
            bypassLoading = false;
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

    <div class="bypass">
        <span class="bypassLabel">Dev Bypass</span>
        <input class="bypassInput" placeholder="Name" bind:value={bypassName} />
        <input
            class="bypassInput"
            placeholder="Andrew ID"
            bind:value={bypassAndrewId}
        />
        <button
            onclick={handleBypass}
            class="bypassBtn"
            disabled={bypassLoading}
        >
            {bypassLoading ? "Signing in..." : "Bypass Sign In"}
        </button>
        {#if bypassError}
            <span class="bypassError">{bypassError}</span>
        {/if}
    </div>
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

    .bypass {
        margin-top: 1.5em;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5em;
        padding: 1em;
        border: 1px dashed var(--colors-text);
        border-radius: 8px;
    }

    .bypassLabel {
        font-size: 0.85em;
        opacity: 0.7;
        color: var(--colors-text);
    }

    .bypassInput {
        padding: 6px 10px;
        border-radius: 6px;
        border: 1px solid var(--colors-text);
        font-size: 14px;
    }

    .bypassBtn {
        background-color: var(--colors-text);
        color: white;
        border: none;
        border-radius: 6px;
        font-size: 14px;
        padding: 8px 20px;
        cursor: pointer;
    }

    .bypassBtn:disabled {
        opacity: 0.6;
        cursor: default;
    }

    .bypassError {
        color: crimson;
        font-size: 0.8em;
    }
</style>
