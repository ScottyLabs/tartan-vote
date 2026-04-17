<script lang="ts">
    import MotionLiveResults from "../lib/components/motionLiveResults.svelte";

    let {
        event,
        sessionCode,
        onBack,
        onEnd,
    }: {
        event: {
            id: number;
            name: string;
            event_type: string;
            status: string;
            start_time: string;
        } | null;
        sessionCode: string | null;
        onBack: () => void;
        onEnd: () => void;
    } = $props();

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    let ending = $state(false);
    let error = $state<string | null>(null);

    const eventLabel = $derived(
        event?.event_type?.toLowerCase() === "election" ? "Election" : "Motion",
    );

    async function handleEndEvent() {
        if (!event || ending) return;

        ending = true;
        error = null;

        try {
            const response = await fetch(`${API_BASE}/events/${event.id}/end`, {
                cache: "no-store",
                credentials: "include",
            });

            if (!response.ok) {
                throw new Error(`Failed to end event: ${response.status}`);
            }

            onEnd();
        } catch (e) {
            error = `Unable to end ${eventLabel.toLowerCase()}. Please try again.`;
        } finally {
            ending = false;
        }
    }
</script>

<main>
    <h1>Voting App</h1>
    <div class="card">
        {#if event}
            <h2>{eventLabel} Running</h2>
            <hr />
            <div class="details">
                <div><strong>Session:</strong> {sessionCode}</div>
                <div><strong>{eventLabel}:</strong> {event.name || "Untitled"}</div>
                <div><strong>Status:</strong> Live</div>
            </div>

            <hr />
            <MotionLiveResults
                eventId={event.id}
                eventType={event.event_type}
            />

            <div class="row">
                <button class="btn" onclick={onBack} disabled={ending}>
                    Back to Meeting
                </button>
                <button class="btn danger" onclick={handleEndEvent} disabled={ending}>
                    {ending
                        ? "Ending..."
                        : `End ${eventLabel.toUpperCase()}`}
                </button>
            </div>

            {#if error}
                <p class="error">{error}</p>
            {/if}
        {:else}
            <h2>No Active Motion</h2>
            <hr />
            <button class="btn" onclick={onBack}>Back to Meeting</button>
        {/if}
    </div>
</main>

<style>
    h1 {
        color: var(--colors-primary);
    }

    .card {
        width: 460px;
        display: flex;
        flex-direction: column;
        gap: 1em;
        padding: 2rem;
        border-radius: 12px;
        background: var(--color-surface);
        box-shadow: var(--shadow-card);
        max-height: 80vh;
        overflow-y: auto;
    }

    h2 {
        margin: 0;
        color: var(--colors-primary);
        text-align: left;
    }

    hr {
        width: 100%;
        border: none;
        border-top: 1px solid var(--color-divider);
        margin: 0;
    }

    .details {
        display: flex;
        flex-direction: column;
        gap: 0.6em;
        text-align: left;
    }

    .row {
        display: flex;
        gap: 1em;
    }

    .btn {
        margin-top: 0.5em;
        background-color: var(--colors-primary);
        color: var(--color-on-primary);
        border: none;
        border-radius: 4px;
        font-size: 18px;
        padding: 10px 16px;
        cursor: pointer;
    }

    .btn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }

    .danger {
        background-color: var(--color-danger);
    }

    .danger:hover {
        background-color: var(--color-danger-hover);
    }

    .btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .error {
        color: var(--color-danger);
        margin: 0;
    }
</style>
