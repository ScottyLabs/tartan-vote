<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { User } from "../lib/models/User";

    type ActiveEvent = {
        id: number;
        name: string;
        event_type: string;
        data: EventData;
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
        onNext: (destination: "session" | "join") => void;
    } = $props();

    type MotionResults = {
        pass: number;
        reject: number;
        abstain: number;
        total: number;
        threshold: number;
        passed: boolean;
    };

    type ElectionResults = {
        vote_type: "election";
        total: number;
        options: Array<{
            label: string;
            count: number;
            percent: number;
        }>;
    };

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    let loading = $state(false);
    let error = $state<string | null>(null);
    let bars = $state<Array<{ label: string; percent: number; color: string; count: number }>>([]);
    let totalVotes = $state(0);
    let heading = $state("Results");
    let summary = $state("");
    let barColor = $state("#ffa500");
    let sessionPollId: number | null = null;
    let resultsPollId: number | null = null;
    let leavingResults = $state(false);

    function handleClick() {
        onNext("session");
    }

    onMount(() => {
        void loadResults();
        resultsPollId = window.setInterval(() => {
            void loadResults(false);
        }, 3000);
        sessionPollId = window.setInterval(() => {
            void checkSessionStatus();
        }, 3000);
    });

    onDestroy(() => {
        if (resultsPollId !== null) {
            window.clearInterval(resultsPollId);
            resultsPollId = null;
        }

        if (sessionPollId !== null) {
            window.clearInterval(sessionPollId);
            sessionPollId = null;
        }
    });

    async function checkSessionStatus() {
        if (!sessionCode || leavingResults) return;

        try {
            const response = await fetch(`${API_BASE}/session/${sessionCode}/status`, {
                cache: "no-store",
                credentials: "include",
            });

            if (!response.ok) {
                return;
            }

            const payload: { session_ended: boolean } = await response.json();
            if (payload.session_ended) {
                leavingResults = true;
                onNext("join");
            }
        } catch {
        }
    }

    async function loadResults(showLoading = true) {
        if (!event) {
            error = "No event selected.";
            return;
        }

        if (showLoading) {
            loading = true;
        }
        error = null;

        try {
            const response = await fetch(`${API_BASE}/events/${event.id}/results`, {
                cache: "no-store",
                credentials: "include",
            });

            if (!response.ok) {
                throw new Error(`Failed to load results: ${response.status}`);
            }

            if (event.data.vote_type === "election") {
                const payload: ElectionResults = await response.json();
                heading = "Election Results";
                totalVotes = payload.total;
                bars = payload.options.map((option, index) => ({
                    label: option.label,
                    percent: option.percent,
                    count: option.count,
                    color: ["#3fb991", "#4f8ef7", "#ffa500", "#ff7563"][index % 4],
                }));

                const leading = payload.options.reduce((best, option) =>
                    option.count > best.count ? option : best
                , payload.options[0] ?? { label: "No winner", count: 0, percent: 0 });

                barColor = "#4f8ef7";
                summary = payload.total > 0
                    ? `${leading.label} is currently leading.`
                    : "No votes have been cast yet.";
                return;
            }

            const payload: MotionResults = await response.json();
            heading = "Motion Results";
            totalVotes = payload.total;
            bars = [
                { label: "Pass", percent: payload.total > 0 ? Math.round((payload.pass / payload.total) * 100) : 0, color: "#3fb991", count: payload.pass },
                { label: "Reject", percent: payload.total > 0 ? Math.round((payload.reject / payload.total) * 100) : 0, color: "#ff7563", count: payload.reject },
                { label: "Abstain", percent: payload.total > 0 ? Math.round((payload.abstain / payload.total) * 100) : 0, color: "#ffa500", count: payload.abstain },
            ];

            if (payload.pass > payload.reject) {
                barColor = "#3fb991";
                summary = "Motion is passed.";
            } else if (payload.pass < payload.reject) {
                barColor = "#ff7563";
                summary = "Motion is rejected.";
            } else {
                barColor = "#ffa500";
                summary = "Motion is tied.";
            }
        } catch (e) {
            error = "Unable to load results right now.";
        } finally {
            if (showLoading) {
                loading = false;
            }
        }
    }
</script>

<main>
    <div class="card">
        <div class="topBar" style="background-color: {barColor}"></div>
        <h2>{heading}</h2>
        <hr />

        {#if loading}
            <p>Loading results...</p>
        {:else if error}
            <p>{error}</p>
        {:else}
            {#each bars as bar}
                <div class="resultRow">
                    <span class="label">{bar.label}:</span>
                    <div class="progress">
                        <div
                            class="fill"
                            style="width: {bar.percent}%; background: {bar.color}"
                        >
                            <span class="inside">{bar.percent}%</span>
                        </div>
                    </div>
                </div>
            {/each}

            <blockquote class="quote" style="border-left: 4px solid {barColor}">
                {summary}
            </blockquote>
        {/if}

        <hr />

        <div class="row">
            <div class="col">
                <div>Total Votes: {totalVotes}</div>
                <div>Event: {event?.name ?? "Unknown"}</div>
            </div>
            <button onclick={handleClick} class="btn">BACK TO SESSION</button>
        </div>
    </div>
</main>

<style>
    .topBar {
        width: 100%;
        height: 6px;
        background: var(--colors-secondary);
        border-radius: 6px 6px 0 0;
        position: absolute;
        top: 0;
        left: 0;
    }
    .card {
        width: 420px;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 1em;

        padding: 2rem;
        border-radius: 12px;
        background: #e0e0e0;

        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
        position: relative;
    }

    h2 {
        text-align: left;
        margin: 0;
        color: var(--colors-text);
    }

    hr {
        width: 100%;
        border: none;
        border-top: 1px solid black;
        margin: 0.5em 0;
    }

    .quote {
        align-self: stretch;
        text-align: left;
        border-left: 4px solid var(--colors-secondary);
        padding-left: 12px;
        margin: 0rem 0;
        color: #555;
        font-style: italic;
    }

    .resultRow {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        width: 100%;
        text-align: left;
    }

    .label {
        width: 60px;
        font-weight: bold;
        text-align: left;
    }

    .progress {
        flex: 1;
        height: 24px;
        background: var(--colors-text);
        border-radius: 12px;
        overflow: hidden;
        position: relative;
    }

    .fill {
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: flex-end;
        padding-right: 6px;
        font-weight: bold;
        color: white;
        transition: width 0.5s ease;
    }

    .inside {
        font-size: 14px;
    }

    .row {
        display: flex;
        justify-content: space-between;
        width: 100%;
        margin-top: 0em;
    }

    .col {
        display: flex;
        flex-direction: column;
        gap: 0.25em;
        text-align: left;
    }

    .btn {
        background-color: var(--colors-secondary);
        color: black;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        padding: 0.5em 1em;
    }

    .btn:hover {
        background-color: color-mix(
            in srgb,
            var(--colors-secondary),
            black 10%
        );
    }
</style>
