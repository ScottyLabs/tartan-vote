<script lang="ts">
    let {
        sessionCode,
        onBack,
        onNext,
    }: {
        sessionCode: string | null;
        onBack: () => void;
        onNext: (notice: string | null) => void;
    } = $props();

    let proxyFor = $state("");
    let senatorChoice = $state<"yes" | "no" | "">("");
    let submitting = $state(false);
    let error = $state<string | null>(null);

    type ParticipationResponse = {
        vote_instance_count: number;
        is_senator: boolean;
        has_proxy: boolean;
    };

    function buildNotice(payload: ParticipationResponse): string {
        if (payload.vote_instance_count === 2) {
            return "You now have 2 vote instances (your own vote + one proxy vote).";
        }

        if (payload.vote_instance_count === 1 && payload.has_proxy) {
            return "You now have 1 proxy vote instance.";
        }

        if (payload.vote_instance_count === 1) {
            return "You now have 1 vote instance.";
        }

        return "You currently have 0 vote instances for this session.";
    }

    async function continueToWaiting() {
        if (!sessionCode || submitting) return;

        if (!senatorChoice) {
            error = "Please choose whether you are an Undergraduate Senator.";
            return;
        }

        submitting = true;
        error = null;

        try {
            const trimmed = proxyFor.trim();
            const requestBody = {
                is_senator: senatorChoice === "yes",
                proxy_for: trimmed.length > 0 ? trimmed : null,
            };

            const url = `/session/${sessionCode}/proxy`;

            const response = await fetch(url, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify(requestBody),
            });

            if (!response.ok) {
                let errorMessage = `HTTP ${response.status} ${response.statusText}`;
                try {
                    const contentType = response.headers.get("content-type");
                    if (contentType?.includes("application/json")) {
                        const payload = await response.json();
                        errorMessage = payload?.error ?? errorMessage;
                    } else {
                        const text = await response.text();
                        if (text) errorMessage = text;
                    }
                } catch {
                }
                throw new Error(errorMessage);
            }

            const contentType = response.headers.get("content-type");
            if (!contentType?.includes("application/json")) {
                throw new Error(`Expected JSON response, got ${contentType}`);
            }

            const payload: ParticipationResponse = await response.json();
            onNext(buildNotice(payload));
        } catch (e) {
            const message = e instanceof Error ? e.message : String(e);
            error = message;
        } finally {
            submitting = false;
        }
    }
</script>

<main>
    <h1>Proxy Setup</h1>
    {#if !sessionCode}
        <p class="error">Error: Session code not provided. Please go back and try again.</p>
    {:else}
        <div class="card">
            <h2>Session: {sessionCode}</h2>
            <p>Tell us your role so we can assign the correct number of vote instances.</p>

            <label>
                <h3>Are you an Undergraduate Senator?</h3>
                <select bind:value={senatorChoice}>
                    <option value="">Select one...</option>
                    <option value="yes">Yes</option>
                    <option value="no">No</option>
                </select>
            </label>

            <label>
                <h3>Proxying for (optional)</h3>
                <input
                    type="text"
                    bind:value={proxyFor}
                    placeholder="e.g. Senator Jane Doe"
                    onkeydown={(event) => {
                        if (event.key === "Enter") {
                            continueToWaiting();
                        }
                    }}
                />
            </label>

            <div class="row">
                <button type="button" class="btn secondary" onclick={onBack} disabled={submitting}>
                    Back
                </button>
                <button type="button" class="btn" onclick={continueToWaiting} disabled={submitting}>
                    {submitting ? "Saving..." : "Continue"}
                </button>
            </div>

            {#if error}
                <p class="error">{error}</p>
            {/if}
        </div>
    {/if}
</main>

<style>
    h1 {
        color: var(--colors-primary);
    }

    .card {
        width: 460px;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        padding: 2rem;
        border-radius: 12px;
        background: #e0e0e0;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    }

    h2 {
        margin: 0;
        color: var(--colors-primary);
    }

    h3 {
        margin: 0 0 0.4rem 0;
        color: black;
        font-weight: normal;
    }

    p {
        margin: 0;
    }

    input,
    select {
        width: 100%;
        height: 48px;
        padding: 10px;
        border-radius: 6px;
        border: 1px solid #ccc;
        box-sizing: border-box;
        font-size: 18px;
    }

    .row {
        display: flex;
        gap: 0.75rem;
        justify-content: flex-end;
    }

    .btn {
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 16px;
        padding: 10px 16px;
        cursor: pointer;
    }

    .btn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }

    .secondary {
        background-color: #777;
    }

    .secondary:hover {
        background-color: color-mix(in srgb, #777, black 10%);
    }

    .btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .error {
        color: #b00020;
        background: #ffebee;
        border: 1px solid #ffcdd2;
        padding: 10px 12px;
        border-radius: 6px;
        margin-top: 0.5rem;
        font-size: 14px;
        word-wrap: break-word;
    }
</style>
