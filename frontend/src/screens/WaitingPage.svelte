<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    let {
        sessionCode,
        notice,
        onEventFound,
    }: {
        sessionCode: string | null;
        notice: string | null;
        onEventFound: (event: {
            id: number;
            name: string;
            event_type: string;
            data: EventData;
        }) => void;
    } = $props();

    const API_BASE = import.meta.env.VITE_API_BASE || "";
    let pollId: number | null = null;

    type VoteInstance = {
        voter_instance_id: number;
        is_proxy: boolean;
        proxy_for_user_id: number | null;
        proxy_for_name: string | null;
        has_voted: boolean;
    };

    async function checkActiveEvent() {
        if (!sessionCode) return;

        try {
            const response = await fetch(`${API_BASE}/events/${sessionCode}/check`, {
                cache: "no-store",
                credentials: "include",
            });

            if (!response.ok) {
                return;
            }

            const payload: {
                active_event: {
                    id: number;
                    name: string;
                    event_type: string;
                    data: EventData;
                } | null;
            } = await response.json();

            if (payload.active_event) {
                const voteInstancesResponse = await fetch(
                    `${API_BASE}/events/${payload.active_event.id}/vote-instances`,
                    {
                        cache: "no-store",
                        credentials: "include",
                    },
                );

                if (!voteInstancesResponse.ok) {
                    return;
                }

                const voteInstances: VoteInstance[] = await voteInstancesResponse.json();
                const proxyAllowed = payload.active_event.data?.proxy === true;
                const hasRemainingVote = voteInstances.some(
                    (instance) => !instance.has_voted && (proxyAllowed || !instance.is_proxy),
                );

                if (hasRemainingVote) {
                    onEventFound(payload.active_event);
                }
            }
        } catch (error) {
        }
    }

    onMount(() => {
        void checkActiveEvent();
        pollId = window.setInterval(() => {
            void checkActiveEvent();
        }, 3000);
    });

    onDestroy(() => {
        if (pollId !== null) {
            window.clearInterval(pollId);
            pollId = null;
        }
    });
</script>

<main>
    <h1>Waiting for Host to Start a Motion...</h1>
    <h2>Session Code: {sessionCode}</h2>
    {#if notice}
        <p class="notice">{notice}</p>
    {/if}
    <div
        class="spinner"
        style="width: 40px; height: 40px; border-top-color: #FF3B3F;"
    ></div>
</main>

<style>
    h1 {
        color: var(--colors-primary);
        margin-bottom: 0.5rem;
    }

    h2 {
        color: var(--colors-text);
        margin: 0.5rem 0 1rem 0;
    }

    .notice {
        background: #e7f3ff;
        border: 1px solid #b9dbff;
        color: #123a66;
        padding: 10px 12px;
        border-radius: 6px;
        margin: 0.5rem 0 1rem;
        max-width: 680px;
    }

    .spinner {
        border: 3px solid #ddd;
        border-radius: 50%;
        border-top-color: #555;
        animation: spin 1s linear infinite;
        position: fixed;
        top: 20px;
        right: 20px;
        z-index: 1000;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
