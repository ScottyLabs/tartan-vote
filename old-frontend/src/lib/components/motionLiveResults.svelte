<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { apiUrl } from "../api/base";

    let {
        eventId = 0,
        eventType = "motion",
    }: {
        eventId: number;
        eventType: string;
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

    let pollError = $state<string | null>(null);
    let loadingResults = $state(false);
    let motionResults = $state<MotionResults | null>(null);
    let electionResults = $state<ElectionResults | null>(null);
    let pollId: number | null = null;

    function pct(value: number, total: number) {
        if (total <= 0) return 0;
        return Math.round((value / total) * 100);
    }

    async function loadResults() {
        loadingResults = true;

        try {
            const response = await fetch(apiUrl(`/events/${eventId}/results`), {
                cache: "no-store",
                credentials: "include",
            });

            if (response.status === 403) {
                pollError = "Results are hidden until release.";
                return;
            }

            if (!response.ok) {
                throw new Error(`Failed to get results: ${response.status}`);
            }

            if (eventType.toLowerCase() === "election") {
                electionResults = await response.json();
                motionResults = null;
            } else {
                motionResults = await response.json();
                electionResults = null;
            }
            pollError = null;
        } catch (error) {
            pollError = "Unable to refresh live results right now.";
        } finally {
            loadingResults = false;
        }
    }

    onMount(() => {
        void loadResults();
        pollId = window.setInterval(() => {
            void loadResults();
        }, 3000);
    });

    onDestroy(() => {
        if (pollId !== null) {
            window.clearInterval(pollId);
            pollId = null;
        }
    });
</script>

<div class="resultsBlock">
    <h3>Live Results</h3>

    <div class="resultsScroll">
        {#if motionResults}
            <div class="resultRow">
                <span>Pass</span>
                <span>{motionResults.pass} ({pct(motionResults.pass, motionResults.total)}%)</span>
            </div>
            <div class="resultRow">
                <span>Reject</span>
                <span>{motionResults.reject} ({pct(motionResults.reject, motionResults.total)}%)</span>
            </div>
            <div class="resultRow">
                <span>Abstain</span>
                <span>{motionResults.abstain} ({pct(motionResults.abstain, motionResults.total)}%)</span>
            </div>
            <div class="resultRow total">
                <span>Total</span>
                <span>{motionResults.total}</span>
            </div>
        {:else if electionResults}
            {#each electionResults.options as option}
                <div class="resultRow">
                    <span>{option.label}</span>
                    <span>{option.count} ({option.percent}%)</span>
                </div>
            {/each}
            <div class="resultRow total">
                <span>Total</span>
                <span>{electionResults.total}</span>
            </div>
        {:else if loadingResults}
            <p>Loading live results...</p>
        {:else}
            <p>No votes yet.</p>
        {/if}
    </div>

    {#if pollError}
        <p class="error">{pollError}</p>
    {/if}
</div>

<style>
    .resultsBlock {
        text-align: left;
        display: flex;
        flex-direction: column;
        gap: 0.5em;
        height: 96px;
        overflow: hidden;
    }

    .resultsScroll {
        flex: 1;
        min-height: 0;
        overflow-y: scroll;
        padding-right: 0.25em;
    }

    .resultsBlock h3 {
        margin: 0;
        color: var(--colors-text);
    }

    .resultRow {
        display: flex;
        justify-content: space-between;
        gap: 1.5em;
    }

    .resultRow.total {
        font-weight: 700;
    }

    .error {
        color: #b00020;
        margin: 0;
    }
</style>
