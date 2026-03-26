<script lang="ts">
    import { Event } from "../lib/models/Event";
    import { User } from "../lib/models/User";
    import { Vote } from "../lib/models/Vote";

    let {
        event,
        user,
        onNext,
    }: {
        event: Event | null;
        user: User | null;
        onNext: () => void;
    } = $props();

    let draftVote = $state<Vote | null>(null);
    draftVote = {
        id: 0,
        cast_time: "",
        data: {
            vote_type: "",
            vote_response: [""],
        },
    };

    let time = $state(new Date());

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    async function submitDraft() {
        try {
            const response = await fetch(`${API_BASE}/${"event.id"}/vote`, {
                // TODO remove when event data is actually passed
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    id: draftVote?.id,
                    cast_time: time.toISOString(),
                    data: {
                        vote_type: draftVote?.data.vote_type,
                        vote_response: draftVote?.data.vote_response,
                    },
                }),
            });
            if (!response.ok) throw new Error(`Failed: ${response.status}`);
            const event = await response.json();
            console.log("Vote Casted:", event);
            onNext?.();
        } catch (error) {
            console.error(error);
        }
    }

    function vote(e: SubmitEvent) {
        e.preventDefault();
        submitDraft();
    }

    function handleClick() {
        onNext();
    }
</script>

<main>
    <h1>CampusVoting</h1>
    <div class="card">
        <h2>Vote on Current Motion</h2>
        <hr />
        <blockquote class="quote">
            {event?.isMotion ? "No Motion" : event?.data}
        </blockquote>
        <form onsubmit={vote}>
            <label>
                <h3>Concerning this motion I vote...</h3>
                <select bind:value={draftVote.data.vote_response[0]} required>
                    <option value="" disabled>Select one...</option>
                    {#each ["Pass", "Reject", "Abstain"] as option}
                        <option value={option}>{option}</option>
                    {/each}
                </select>
            </label>
            <button type="submit" class="submitBtn">Submit Vote</button>
        </form>
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
</style>
