<script lang="ts">
    import LongTextInput from "./../lib/components/longTextInput.svelte";
    import SelectMenu from "./../lib/components/selectMenu.svelte";
    import Popup from "../lib/components/popup.svelte";
    import backArr from "../lib/images/back_arrow.png";
    import ArrayEditor from "../lib/components/arrayEditor.svelte";
    import TimeScroller from "../lib/components/timeScroller.svelte";
    import HoverCard from "../lib/components/hoverCard.svelte";
    import { User } from "../lib/models/User";
    import { Event as VoteEvent } from "../lib/models/Event";

    let { onNext, onBack } = $props();
    const env = import.meta.env as Record<string, string | undefined>;
    const backendBaseUrl = (env.VITE_BACKEND_URL ?? env.BACKEND_URL ?? "")
        .trim()
        .replace(/\/+$/, "");

    function eventDraft_new(vote_type: "motion" | "election"): VoteEvent {
        return new VoteEvent({
            id: 0,
            event_type: vote_type,
            name: "",
            status: "",
            start_time: "",
            end_time: null,
            data: {
                description: "",
                session_code: "",
                vote_type: vote_type,
                threshold: 0.5,
                visibility: {
                    participants: "hidden_until_release",
                },
                proxy: false,
                vote_options:
                    vote_type === "motion" ? ["Pass", "Reject", "Abstain"] : [],
            },
            created_by_user_id: 0,
            organization_id: 0,
        });
    }

    function goNext() {
        onNext?.();
    }

    let users: User[] = $state([
        new User({
            id: 69,
            name: "Max Tentype",
            created_at: "2026-01-01T00:00:00Z",
        }),
        new User({
            id: 420,
            name: "Yiyoung Liu",
            created_at: "2026-01-01T00:00:00Z",
        }),
        new User({
            id: 67,
            name: "Anish Pallati",
            created_at: "2026-01-01T00:00:00Z",
        }),
    ]);

    let meetingCode: string = "3CMU67";

    let electionStyleOptions: string[] = [
        "Plurality Election",
        "Ranked Choice Election",
    ];

    let voteStyleOptions: string[] = [
        "Standard Vote",
        "Recorded (roll-call) Vote",
        "Secret Vote",
    ];

    let voteThresholds: { label: string; value: number }[] = [
        { label: "Majority", value: 0.5 },
        { label: "2/3", value: 0.667 },
        { label: "3/4", value: 0.75 },
        { label: "Unanimous", value: 1.0 },
    ];

    let draft = $state<VoteEvent>(eventDraft_new("motion"));
    let draftTime: Time = $state({
        days: 0,
        hours: 0,
        mins: 0,
        secs: 0,
    });

    // Popup Booleans
    let creatingMotion = $state(false);
    let creatingElection = $state(false);
    let inspectingUser = $state<User | null>(null);
    let inspectingAllUsers = $state(false);
    let timerEnded = $state(false);

    let creatingMotionRequest = $state(false);
    let creatingElectionRequest = $state(false);
    let createError = $state<string | null>(null);

    function deleteUser(i: number) {
        users.splice(i, 1);
    }

    function pushMotion() {
        draft = eventDraft_new("motion");
        creatingMotion = true;
    }

    function pushElection() {
        draft = eventDraft_new("election");
        creatingElection = true;
    }

    function inspectAllUsers() {
        inspectingAllUsers = true;
    }

    function onPopupClose() {
        creatingElection = false;
        creatingMotion = false;
        inspectingAllUsers = false;
        timerEnded = false;
        createError = null;
    }

    function inspectUser(user: User) {
        inspectingUser = user;
    }

    function clearInspect() {
        inspectingUser = null;
    }

    function endTimer() {
        timerEnded = true;
    }

    function thresholdToDecimal(label: string): number {
        switch (label) {
            case "2/3":
                return 2 / 3;
            case "3/4":
                return 3 / 4;
            case "Unanimous":
                return 1;
            case "Majority":
            default:
                return 0.5;
        }
    }

    function timerToEndTime(timer: {
        days: number;
        hours: number;
        mins: number;
        secs: number;
    }): string | undefined {
        const totalSeconds =
            timer.days * 24 * 60 * 60 +
            timer.hours * 60 * 60 +
            timer.mins * 60 +
            timer.secs;
        if (totalSeconds <= 0) return undefined;
        return new Date(Date.now() + totalSeconds * 1000).toISOString();
    }

    async function readErrorMessage(response: Response): Promise<string> {
        try {
            const json = (await response.json()) as { error?: string };
            return json.error ?? `Request failed with status ${response.status}`;
        } catch {
            return `Request failed with status ${response.status}`;
        }
    }

    async function createEvent(payload: unknown): Promise<void> {
        if (!backendBaseUrl) {
            throw new Error(
                "BACKEND_URL is missing. Set VITE_BACKEND_URL (or BACKEND_URL) in frontend env.",
            );
        }

        // Backend route from backend/src/event/router.rs: POST /api/event/create
        const response = await fetch(`${backendBaseUrl}/api/event/create`, {
            method: "POST",
            headers: { "content-type": "application/json" },
            body: JSON.stringify(payload),
        });

        if (!response.ok) {
            throw new Error(await readErrorMessage(response));
        }
    }

    async function submitMotion(event: SubmitEvent) {
        event.preventDefault();
        createError = null;
        creatingMotionRequest = true;

        try {
            const payload = {
                event_type: "motion",
                name: draft.name.trim() || "Motion",
                created_by_user_id: users[0]?.id ?? 1,
                organization_id: 1,
                end_time: timerToEndTime(draftTime),
                data: {
                    motion: {
                        description: draft.data.description.trim(),
                        threshold: thresholdToDecimal(String(draft.data.threshold)),
                        visibility: { participants: "hidden_until_release" },
                        proxy: false,
                        vote_options: ["yes", "no", "abstain"],
                    },
                },
            };

            await createEvent(payload);
            creatingMotion = false;
        } catch (error) {
            createError =
                error instanceof Error
                    ? error.message
                    : "Unable to create motion.";
        } finally {
            creatingMotionRequest = false;
        }
    }

    async function submitElection(event: SubmitEvent) {
        event.preventDefault();
        createError = null;
        creatingElectionRequest = true;

        try {
            const voteOptions = draft.data.vote_options
                .map((candidate: string) => candidate.trim())
                .filter((candidate: string) => candidate.length > 0);

            const payload = {
                event_type: "vote",
                name: draft.name.trim() || "Election",
                created_by_user_id: users[0]?.id ?? 1,
                organization_id: 1,
                end_time: timerToEndTime(draftTime),
                data: {
                    vote: {
                        description: draft.name.trim() || "Election",
                        vote_type: "election",
                        threshold: 0.5,
                        visibility: { participants: "hidden_until_release" },
                        proxy: false,
                        vote_options: voteOptions,
                    },
                },
            };

            await createEvent(payload);
            creatingElection = false;
        } catch (error) {
            createError =
                error instanceof Error
                    ? error.message
                    : "Unable to create election.";
        } finally {
            creatingElectionRequest = false;
        }
    }

    // TODO: after backend exists it can pass on live results to here
    function getResults() {
        return "TBD";
    }

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    async function submitDraft() {
        try {
            const response = await fetch(`${API_BASE}/api/events`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    name: draft.name,
                    vote_type: draft.data.vote_type,
                    description: draft.data.description,
                    threshold: draft.data.threshold,
                    vote_options: draft.data.vote_options,
                    proxy: draft.data.proxy,
                    visibility: draft.data.visibility.participants,
                    organization_id: 1, // TODO: real org from auth
                    start_time: new Date().toISOString(),
                    end_time: timerToEndTime(draftTime),
                }),
            });
            if (!response.ok) throw new Error(`Failed: ${response.status}`);
            const event = await response.json();
            console.log("Event created:", event);
            onPopupClose();
            goNext();
        } catch (error) {
            console.error(error);
        }
    }
</script>

<Popup title="Results:" open={timerEnded} onClose={onPopupClose}>
    <div>
        {getResults()}
    </div>
    <hr style="margin-bottom: 0;" />
    <h3 style="margin: 0;">Push Results?</h3>
    <div
        class="row"
        style="justify-content: center; gap: 1rem; align-items: center;"
    >
        <button
            type="button"
            onclick={goNext}
            class="btn"
            style="padding: 10px 50px; margin:0"
        >
            Yes
        </button>
        <button
            type="button"
            onclick={onPopupClose}
            class="btn"
            style="padding: 10px 50px; margin: 0"
        >
            No
        </button>
    </div>
</Popup>

<Popup
    title="Participants (Headcount: {users.length})"
    open={inspectingAllUsers}
    onClose={onPopupClose}
>
    <div class="button-list">
        {#each users as user, i}
            <div
                class="slot-wrapper"
                role="group"
                onmouseenter={() => inspectUser(user)}
                onmouseleave={clearInspect}
            >
                <button onclick={() => deleteUser(i)} class="slotDel">
                    {user.name?.charAt(0)}
                </button>
                <HoverCard open={inspectingUser?.id === user.id}>
                    <div class="col">
                        <div>Name: {user.name}</div>
                        <div>UserID: {user.id}</div>
                        <div>Time Created: {user.id}</div>
                    </div>
                </HoverCard>
            </div>
        {/each}
    </div>
</Popup>

<Popup
    title="Motion #{draft.name}"
    open={creatingMotion}
    onClose={onPopupClose}
>
    <form onsubmit={submitMotion}>
        <LongTextInput
            title="Description:"
            bind:value={draft.data.description}
            emptyPlaceholder="Input Description"
        ></LongTextInput>

        <SelectMenu
            title="Threshold:"
            bind:value={draft.data.threshold}
            options={voteThresholds}
        ></SelectMenu>

        <SelectMenu
            title="Voting Style:"
            bind:value={draft.data.vote_type}
            options={voteStyleOptions}
        ></SelectMenu>

        <TimeScroller value={draftTime}></TimeScroller>

        <button type="submit" class="submitBtn" disabled={creatingMotionRequest}
            >{creatingMotionRequest ? "Creating..." : "Push Motion"}</button
        >
        {#if createError}
            <p class="errorText">{createError}</p>
        {/if}
    </form>
</Popup>

<Popup title={draft.name || "Election"} open={creatingElection} onClose={onPopupClose}>
    <form onsubmit={submitElection}>
        <label>
            <h3>Title:</h3>
            <input type="text" bind:value={draft.name} required />
        </label>

        <ArrayEditor title="Candidates" bind:items={draft.data.vote_options}
        ></ArrayEditor>

        <SelectMenu
            title="Election Style:"
            bind:value={draft.data.vote_type}
            options={electionStyleOptions}
        ></SelectMenu>

        <TimeScroller bind:value={draftTime}></TimeScroller>

        <button
            type="submit"
            class="submitBtn"
            disabled={creatingElectionRequest}
            >{creatingElectionRequest ? "Creating..." : "Push Election"}</button
        >
        {#if createError}
            <p class="errorText">{createError}</p>
        {/if}
    </form>
</Popup>

<main>
    <h1>Voting App</h1>
    <div class="card">
        <div class="row">
            <h1>Meeting Code:</h1>
            <h1 style="color:var(--colors-primary)">{meetingCode}</h1>
        </div>
        <hr />

        <div class="row">
            <h1>Participants</h1>
            <div class="container">
                <div class="button-list">
                    {#each users.slice(0, 30 - 1) as user}
                        <div
                            class="slot-wrapper"
                            role="group"
                            onmouseenter={() => inspectUser(user)}
                            onmouseleave={clearInspect}
                        >
                            <button class="slot">
                                {user.name?.charAt(0)}
                            </button>
                            <HoverCard
                                open={inspectingUser?.id === user.id &&
                                    !inspectingAllUsers}
                            >
                                <div class="col">
                                    <div>Name: {user.name}</div>
                                    <div>UserID: {user.id}</div>
                                    <div>Time Created: {user.created_at}</div>
                                </div>
                            </HoverCard>
                        </div>
                    {/each}
                    {#if users.length >= 30}
                        <button onclick={inspectAllUsers} class="slot plus"
                            >+</button
                        >
                    {/if}
                </div>
            </div>
        </div>
        <hr />
        <div class="row" style="margin-bottom: 0em">
            <button onclick={pushMotion} class="btn">Push a Motion</button>
            <button onclick={pushElection} class="btn">Push an Election</button>
        </div>
        <div class="row" style="marging-top=0em">
            <button onclick={endTimer} class="btn">END MEETING</button>
            <button class="btn" style="padding: 10px 175px">EXPORT</button>
        </div>
    </div>
    {#if !creatingElection && !creatingMotion && !inspectingAllUsers}
        <button onclick={onBack} class="backBtn">
            <img src={backArr} alt="Click me" />
        </button>
    {/if}
</main>

<style>
    .slot-wrapper {
        position: relative;
        width: 32px;
    }
    .btn {
        margin-top: 1em;
        background-color: var(--colors-primary);
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }
    .btn:hover {
        background-color: color-mix(in srgb, var(--colors-primary), black 10%);
    }
    .card {
        width: fit-content;
        padding: 1.5rem;
        border-radius: 12px;
        background: #e0e0e0;
    }
    .container {
        border: 2px solid #ccc;
        padding: 8px;
        border-radius: 8px;
        width: fit-content;
        background: #f8f8f8;
        overflow: visible;
    }
    .row {
        display: flex;
        justify-content: flex-start;
        width: 100%;
        margin-top: 0.5em;
        gap: 1em;
        overflow: visible;
    }

    .button-list {
        margin-right: 0.75em;
        display: grid;
        grid-template-columns: repeat(10, 32px);
        grid-auto-rows: 20px;
        overflow: visible;
        gap: 1rem;
    }

    .slot {
        height: 28px;
        min-width: 28px;
        font-size: 0.8rem;
        border: 1px solid #aaa;
        border-radius: 4px;
        background: white;
        cursor: pointer;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .slot:hover {
        background: #eee;
    }

    .slotDel {
        height: 28px;
        min-width: 28px;
        font-size: 0.8rem;
        border: 1px solid #aaa;
        border-radius: 4px;
        background: white;
        cursor: pointer;

        display: flex;
        align-items: center;
        justify-content: center;
    }

    .slotDel:hover {
        background: #f44a4a;
    }

    .plus {
        font-weight: bold;
    }

    hr {
        width: 100%;
        border: none;
        border-top: 2px solid var(--colors-text);
        margin-top: 1em;
        margin-bottom: 1em;
    }

    h1 {
        color: var(--colors-text);
        margin-bottom: 0.5em;
    }

    h3 {
        text-align: left;
        color: var(--colors-text);
        margin-bottom: 0.5em;
        font-weight: normal;
    }

    form {
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 0rem;
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

    .backBtn {
        position: fixed;
        top: 20px;
        left: 20px;

        width: 40px;
        height: 40px;

        display: flex;
        justify-content: center;
        align-items: center;

        padding: 0;
        border: none;
        background: none;
    }

    .col {
        display: flex;
        flex-direction: column;
        gap: 0.25em;
        text-align: left;
    }

    .backBtn img {
        width: 24px;
        height: 24px;
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

    .submitBtn:disabled {
        cursor: not-allowed;
        opacity: 0.7;
    }

    .errorText {
        margin: 0.5rem 0 0;
        color: #d92d20;
        font-size: 0.9rem;
    }
</style>
