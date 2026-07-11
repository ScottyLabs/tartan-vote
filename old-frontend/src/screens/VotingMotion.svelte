<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { apiUrl } from "../lib/api/base";
    import { User } from "../lib/models/User";

    type ActiveEvent = {
        id: number;
        name: string;
        event_type: string;
        data: EventData;
    };

    type VoteInstance = {
        voter_instance_id: number;
        is_proxy: boolean;
        proxy_for_user_id: number | null;
        proxy_for_name: string | null;
        has_voted: boolean;
    };

    let {
        event,
        user,
        sessionCode,
        onNext,
    }: {
        event: ActiveEvent | null;
        user: User | null;
        sessionCode: string | null;
        onNext: (destination: "results" | "session") => void;
    } = $props();

    let selectedVoteOption = $state("");
    let voteOptions = $state<string[]>([]);
    let voteInstances = $state<VoteInstance[]>([]);
    let availableVoteInstances = $state<VoteInstance[]>([]);
    let selectedVoterInstanceId = $state<number | null>(null);
    let loadingVoteData = $state(false);
    let submittingVote = $state(false);
    let error = $state<string | null>(null);
    let alreadyVoted = $state(false);
    let activeEventPollId: number | null = null;
    let leavingMotion = $state(false);

    const isElection = $derived(event?.data?.vote_type === "election");
    const eventLabel = $derived(isElection ? "Election" : "Motion");
    const descriptionLabel = $derived(
        isElection
            ? event?.name || "No election details provided."
            : event?.data?.description || "No motion details provided.",
    );
    const votePrompt = $derived(
        isElection
            ? "For this election I vote..."
            : "Concerning this motion I vote...",
    );

    const proxyAllowed = $derived(event?.data?.proxy === true);

    onMount(() => {
        void loadVoteData();
        activeEventPollId = window.setInterval(() => {
            void checkMotionStillActive();
        }, 3000);
    });

    onDestroy(() => {
        if (activeEventPollId !== null) {
            window.clearInterval(activeEventPollId);
            activeEventPollId = null;
        }
    });

    async function checkMotionStillActive() {
        if (!sessionCode || !event || leavingMotion) return;

        try {
            const response = await fetch(apiUrl(`/events/${sessionCode}/check`), {
                cache: "no-store",
                credentials: "include",
            });

            if (!response.ok) {
                return;
            }

            const payload: {
                active_event: {
                    id: number;
                } | null;
            } = await response.json();

            if (!payload.active_event || payload.active_event.id !== event.id) {
                leavingMotion = true;
                onNext?.("session");
            }
        } catch {
        }
    }

    async function loadVoteData() {
        if (!event) {
            error = "No active vote found.";
            return;
        }

        loadingVoteData = true;
        error = null;

        try {
            const response = await fetch(
                apiUrl(`/events/${event.id}/vote-instances`),
                {
                    cache: "no-store",
                    credentials: "include",
                },
            );

            if (!response.ok) {
                throw new Error(`Failed to load vote instances: ${response.status}`);
            }

            const instances: VoteInstance[] = await response.json();
            voteInstances = instances;

            availableVoteInstances = proxyAllowed
                ? instances
                : instances.filter((instance) => !instance.is_proxy);

            const unvoted = availableVoteInstances.find((instance) => !instance.has_voted);
            alreadyVoted = !unvoted;
            selectedVoterInstanceId = unvoted?.voter_instance_id ?? null;

            const configuredOptions = event.data?.vote_options?.length
                ? event.data.vote_options
                : ["Pass", "Reject", "Abstain"];

            voteOptions = proxyAllowed
                ? configuredOptions
                : configuredOptions.filter(
                    (option) => option.trim().toLowerCase() !== "proxy",
                );
        } catch (e) {
            error = "Unable to load your ballot right now.";
        } finally {
            loadingVoteData = false;
        }
    }

    async function submitDraft() {
        if (!event || !selectedVoteOption || submittingVote || alreadyVoted) return;

        submittingVote = true;
        error = null;

        try {
            const response = await fetch(apiUrl(`/events/${event.id}/vote`), {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                credentials: "include",
                body: JSON.stringify({
                    vote_response: [selectedVoteOption],
                    voter_instance_id: selectedVoterInstanceId,
                }),
            });

            if (!response.ok) {
                const payload = await response.json().catch(() => null);
                throw new Error(payload?.error ?? `Failed: ${response.status}`);
            }

            const resultsResponse = await fetch(apiUrl(`/events/${event.id}/results`), {
                cache: "no-store",
                credentials: "include",
            });

            if (resultsResponse.ok) {
                onNext?.("results");
                return;
            }

            onNext?.("session");
        } catch (e) {
            error = e instanceof Error ? e.message : "Unable to submit vote.";
        } finally {
            submittingVote = false;
        }
    }

    function vote(e: SubmitEvent) {
        e.preventDefault();
        submitDraft();
    }
</script>

<main>
    <h1>CampusVoting</h1>
    <div class="card">
        <h2>Vote on Current {eventLabel}</h2>
        <hr />
        <blockquote class="quote">
            {descriptionLabel}
        </blockquote>

        {#if loadingVoteData}
            <p>Loading your ballot...</p>
        {:else if alreadyVoted}
            <p>You have already voted in this {eventLabel.toLowerCase()}.</p>
            <button
                type="button"
                class="submitBtn"
                onclick={() => onNext?.("session")}
            >
                Back to Session
            </button>
        {:else}
            <form onsubmit={vote}>
            <label>
                <h3>{votePrompt}</h3>
                <select bind:value={selectedVoteOption} required>
                    <option value="" disabled>Select one...</option>
                    {#each voteOptions as option}
                        <option value={option}>{option}</option>
                    {/each}
                </select>
            </label>
            {#if availableVoteInstances.length > 1}
                <label>
                    <h3>Voting As...</h3>
                    <select bind:value={selectedVoterInstanceId} required>
                        {#each availableVoteInstances.filter((instance) => !instance.has_voted) as instance}
                            <option value={instance.voter_instance_id}>
                                {instance.is_proxy && instance.proxy_for_name
                                    ? `Proxy for ${instance.proxy_for_name}`
                                    : "Your vote"}
                            </option>
                        {/each}
                    </select>
                </label>
            {/if}
            <button type="submit" class="submitBtn" disabled={submittingVote}>
                {submittingVote ? "Submitting..." : "Submit Vote"}
            </button>
            </form>
        {/if}

        {#if error}
            <p class="error">{error}</p>
        {/if}
    </div>
</main>

<style>
    h1 {
        color: white;
    }

    h2 {
        margin-top: 0em;
        margin-bottom: 0em;
        color: var(--colors-primary);
    }

    h3 {
        text-align: left;
        color: black;
        margin-bottom: 0.5em;
        font-weight: normal;
    }

    form {
        margin-top: 0em;
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 1rem;
    }

    select {
        width: 100%;
        height: 50px;
        padding: 10px;
        border-radius: 6px;
        border: 1px solid #ccc;
        box-sizing: border-box;
        font-size: 20px;
        margin-bottom: 0em;
    }

    hr {
        width: 100%;
        border: none;
        border-top: 1px solid #bdbdbd;
        margin: 0 0;
    }

    .quote {
        align-self: stretch;
        text-align: left;
        border-left: 4px solid var(--colors-primary);
        padding-left: 12px;
        margin: 1rem 0;
        color: #555;
        font-style: italic;
    }

    .submitBtn {
        margin-top: 1em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }

    .card {
        width: 420px;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1em;

        padding: 2rem;
        border-radius: 12px;
        background: #e0e0e0;

        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    }

    .error {
        color: #b00020;
        margin: 0;
    }
</style>
