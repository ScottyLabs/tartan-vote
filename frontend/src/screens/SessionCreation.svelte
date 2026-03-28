<script lang="ts">
    import LongTextInput from "../lib/components/longTextInput.svelte";
    import SelectMenu from "../lib/components/selectMenu.svelte";
    import Popup from "../lib/components/popup.svelte";
    import backArr from "../lib/images/back_arrow.png";
    import ArrayEditor from "../lib/components/arrayEditor.svelte";
    import TimeScroller from "../lib/components/timeScroller.svelte";
    import HoverCard from "../lib/components/hoverCard.svelte";
    import { User } from "../lib/models/User";

    type CreatedEvent = {
        id: number;
        name: string;
        event_type: string;
        status: string;
        start_time: string;
    };

    let { onNext, onBack, onEventStarted, sessionCode } = $props();

    type MotionPreset = "standard" | "secret";

    type ProxyAssignmentDraft = {
        proxy_holder_user_id: number;
        proxied_senator_user_id: number;
    };

    function eventDraft_new(vote_type: "motion" | "election") {
        return {
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
                    participants:
                        vote_type === "motion"
                            ? "live"
                            : "hidden_until_release",
                },
                proxy: false,
                ballot_style: vote_type === "motion" ? "standard" : "default",
                meeting_display: vote_type === "motion"
                    ? "named_by_category"
                    : "totals_only",
                export_scope: "totals_only",
                vote_options:
                    vote_type === "motion" ? ["Yes", "No", "Abstain"] : [],
            },
            created_by_user_id: 0,
            organization_id: 0,
        };
    }

    let draft = $state(eventDraft_new("motion"));

    function goNext() {
        onNext?.();
    }

    let users: User[] = $state([
        new User({
            id: 69,
            name: "Max Tentype",
            andrew_id: "maxwen",
            oidc_client: "null",
            created_at: "2026-01-01T00:00:00Z",
        }),
        //     new User({
        //         id: 420,
        //         name: "Yiyoung Liu",
        //         created_at: "2026-01-01T00:00:00Z",
        //     }),
        //     new User({
        //         id: 67,
        //         name: "Anish Pallati",
        //         created_at: "2026-01-01T00:00:00Z",
        //     }),
    ]);

    let electionStyleOptions: string[] = [
        "Plurality Election",
        "Ranked Choice Election",
    ];

    let motionPreset = $state<MotionPreset>("standard");

    let motionPresetOptions: { label: string; value: MotionPreset }[] = [
        { label: "Standard Vote", value: "standard" },
        { label: "Secret Vote", value: "secret" },
    ];

    let motionDisplayOptions: { label: string; value: string }[] = [
        {
            label: "Show who voted in each category",
            value: "named_by_category",
        },
        { label: "Show totals only", value: "totals_only" },
    ];

    let resultVisibilityOptions: { label: string; value: string }[] = [
        { label: "Show live totals during meeting", value: "live" },
        {
            label: "Hide totals until vote closes",
            value: "hidden_until_release",
        },
    ];

    let exportScopeOptions: { label: string; value: string }[] = [
        { label: "Save only totals in meeting export", value: "totals_only" },
        { label: "Save detailed ballots in meeting export", value: "full_ballots" },
    ];

    let voteThresholds: { label: string; value: number }[] = [
        { label: "Majority", value: 0.5 },
        { label: "2/3", value: 0.667 },
        { label: "3/4", value: 0.75 },
        { label: "Unanimous", value: 1.0 },
    ];

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
    let endingMeeting = $state(false);
    let endMeetingError = $state<string | null>(null);
    let latestEventId = $state<number | null>(null);
    let proxyHolderIdInput = $state("");
    let proxiedSenatorIdInput = $state("");
    let proxyAssignmentError = $state("");
    let proxyAssignments = $state<ProxyAssignmentDraft[]>([]);

    function deleteUser(i: number) {
        users.splice(i, 1);
    }

    function pushMotion() {
        draft = eventDraft_new("motion");
        motionPreset = "standard";
        applyMotionPreset(motionPreset);
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
        endMeetingError = null;
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

    function getResults() {
        return "TBD";
    }

    function applyMotionPreset(preset: MotionPreset) {
        draft.data.vote_options = ["Yes", "No", "Abstain"];
        draft.data.export_scope = "totals_only";

        if (preset === "standard") {
            draft.data.ballot_style = "standard";
            draft.data.visibility.participants = "live";
            draft.data.meeting_display = "named_by_category";
            draft.data.proxy = false;
            return;
        }

        draft.data.ballot_style = "secret";
        draft.data.visibility.participants = "live";
        draft.data.meeting_display = "totals_only";
        draft.data.proxy = false;
    }

    function addProxyAssignment() {
        proxyAssignmentError = "";
        const proxy_holder_user_id = Number.parseInt(proxyHolderIdInput.trim(), 10);
        const proxied_senator_user_id = Number.parseInt(
            proxiedSenatorIdInput.trim(),
            10,
        );

        if (!Number.isFinite(proxy_holder_user_id) || !Number.isFinite(proxied_senator_user_id)) {
            proxyAssignmentError = "Both IDs must be valid numbers.";
            return;
        }

        if (proxy_holder_user_id === proxied_senator_user_id) {
            proxyAssignmentError = "A user cannot proxy for themself.";
            return;
        }

        if (
            proxyAssignments.some(
                (assignment) =>
                    assignment.proxy_holder_user_id === proxy_holder_user_id,
            )
        ) {
            proxyAssignmentError =
                "This participant already holds a proxy for this event.";
            return;
        }

        proxyAssignments = [
            ...proxyAssignments,
            {
                proxy_holder_user_id,
                proxied_senator_user_id,
            },
        ];

        proxyHolderIdInput = "";
        proxiedSenatorIdInput = "";
    }

    function removeProxyAssignment(index: number) {
        proxyAssignments = proxyAssignments.filter((_, i) => i !== index);
    }

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    async function exportLatestEvent() {
        if (!latestEventId) {
            console.warn("No event has been created yet; nothing to export.");
            return;
        }

        try {
            const response = await fetch(`${API_BASE}/events/${latestEventId}/export`, {
                method: "GET",
                credentials: "include",
            });

            if (!response.ok) throw new Error(`Export failed: ${response.status}`);

            const payload = await response.json();
            const blob = new Blob([JSON.stringify(payload, null, 2)], {
                type: "application/json",
            });
            const url = URL.createObjectURL(blob);
            const anchor = document.createElement("a");
            anchor.href = url;
            anchor.download = `event-${latestEventId}-export.json`;
            anchor.click();
            URL.revokeObjectURL(url);
        } catch (error) {
            console.error(error);
        }
    }

    function timerToEndTime(timer: Time): string {
        const now = Date.now();
        const ms =
            timer.days * 86400000 +
            timer.hours * 3600000 +
            timer.mins * 60000 +
            timer.secs * 1000;
        return new Date(now + ms).toISOString();
    }

    async function submitDraft() {
        try {
            const backendEventType =
                draft.event_type === "motion" ? "Motion" : "Election";

            const response = await fetch(`${API_BASE}/events/create/${sessionCode}`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                credentials: "include",
                body: JSON.stringify({
                    name: draft.name,
                    event_type: backendEventType,
                    start_time: new Date().toISOString(),
                    end_time: timerToEndTime(draftTime),
                    data: {
                        vote_type: draft.event_type,
                        description: draft.data.description,
                        threshold: draft.data.threshold,
                        vote_options: draft.data.vote_options,
                        proxy: draft.data.proxy,
                        ballot_style: draft.data.ballot_style,
                        meeting_display: draft.data.meeting_display,
                        export_scope: draft.data.export_scope,
                        anonymous: draft.data.ballot_style === "secret",
                        visibility: draft.data.visibility.participants,
                        eligible_voter_user_ids: users.map((u) => u.id),
                        proxy_assignments: proxyAssignments,
                    },
                }),
            });
            if (!response.ok) throw new Error(`Failed: ${response.status}`);
            const event: CreatedEvent = await response.json();
            latestEventId = event.id;
            console.log("Event created:", event);
            onPopupClose();
            onEventStarted?.(event);
        } catch (error) {
            console.error(error);
        }
    }

    async function endMeeting() {
        if (!sessionCode || endingMeeting) return;

        endingMeeting = true;
        endMeetingError = null;

        try {
            const response = await fetch(`${API_BASE}/session/${sessionCode}/end`, {
                cache: "no-store",
                credentials: "include",
            });

            if (!response.ok) {
                throw new Error(`Failed to end session: ${response.status}`);
            }

            onPopupClose();
            goNext();
        } catch (error) {
            console.error(error);
            endMeetingError = "Unable to end meeting. Please try again.";
        } finally {
            endingMeeting = false;
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
            onclick={endMeeting}
            class="btn"
            style="padding: 10px 50px; margin:0"
            disabled={endingMeeting}
        >
            {endingMeeting ? "Ending..." : "Yes"}
        </button>
        <button
            type="button"
            onclick={onPopupClose}
            class="btn"
            style="padding: 10px 50px; margin: 0"
            disabled={endingMeeting}
        >
            No
        </button>
    </div>
    {#if endMeetingError}
        <p class="error">{endMeetingError}</p>
    {/if}
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
    <form
        onsubmit={(e) => {
            e.preventDefault();
            submitDraft();
        }}
    >
        <label>
            <h3>Title:</h3>
            <input type="text" bind:value={draft.name} required />
        </label>

        <SelectMenu
            title="Preset:"
            bind:value={motionPreset}
            options={motionPresetOptions}
        ></SelectMenu>

        <button
            type="button"
            class="presetBtn"
            onclick={() => applyMotionPreset(motionPreset)}
        >
            Apply Preset
        </button>

        <LongTextInput
            title="Description:"
            bind:value={draft.data.description}
            emptyPlaceholder="Input Description"
        ></LongTextInput>

        <ArrayEditor title="Vote Options" bind:items={draft.data.vote_options}
        ></ArrayEditor>

        <SelectMenu
            title="Threshold:"
            bind:value={draft.data.threshold}
            options={voteThresholds}
        ></SelectMenu>

        <SelectMenu
            title="In-Meeting Display:"
            bind:value={draft.data.meeting_display}
            options={motionDisplayOptions}
        ></SelectMenu>

        <SelectMenu
            title="Result Visibility:"
            bind:value={draft.data.visibility.participants}
            options={resultVisibilityOptions}
        ></SelectMenu>

        <label class="toggleRow">
            <input type="checkbox" bind:checked={draft.data.proxy} />
            <span>Enable Proxy Voting</span>
        </label>

        <SelectMenu
            title="Meeting Export:"
            bind:value={draft.data.export_scope}
            options={exportScopeOptions}
        ></SelectMenu>

        {#if draft.data.proxy}
            <div class="proxy-panel">
                <h3 style="margin: 0 0 0.5rem 0;">Proxy Assignments</h3>
                <div class="proxy-row">
                    <input
                        type="number"
                        placeholder="Proxy Holder User ID"
                        bind:value={proxyHolderIdInput}
                    />
                    <input
                        type="number"
                        placeholder="Proxied Senator User ID"
                        bind:value={proxiedSenatorIdInput}
                    />
                    <button type="button" class="proxy-add" onclick={addProxyAssignment}
                        >Add</button
                    >
                </div>
                {#if proxyAssignmentError}
                    <div class="proxy-error">{proxyAssignmentError}</div>
                {/if}

                <div class="proxy-list">
                    {#if proxyAssignments.length === 0}
                        <div class="proxy-empty">No proxy assignments yet.</div>
                    {:else}
                        {#each proxyAssignments as assignment, i}
                            <div class="proxy-item">
                                <span>
                                    Holder #{assignment.proxy_holder_user_id} → Senator #{assignment.proxied_senator_user_id}
                                </span>
                                <button
                                    type="button"
                                    class="proxy-remove"
                                    onclick={() => removeProxyAssignment(i)}
                                >
                                    Remove
                                </button>
                            </div>
                        {/each}
                    {/if}
                </div>
            </div>
        {/if}

        <TimeScroller value={draftTime}></TimeScroller>

        <button type="submit" class="submitBtn">Push Motion</button>
    </form>
</Popup>

<Popup title={draft.name} open={creatingElection} onClose={onPopupClose}>
    <form
        onsubmit={(e) => {
            e.preventDefault();
            submitDraft();
        }}
    >
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

        <button type="submit" class="submitBtn">Push Election</button>
    </form>
</Popup>

<main>
    <h1>Voting App</h1>
    <div class="card">
        <div class="row">
            <h1>Meeting Code:</h1>
            <h1 style="color:var(--colors-primary)">{sessionCode}</h1>
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
                    <button onclick={inspectAllUsers} class="slot plus"
                        >+</button
                    >
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
            <button class="btn" style="padding: 10px 175px" onclick={exportLatestEvent}
                >EXPORT</button
            >
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
    .btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }
    .error {
        color: #b00020;
        margin-top: 0.5em;
    }
    .card {
        width: fit-content;
        padding: 1.5rem;
        border-radius: 12px;
        background: #e0e0e0;
    }

    .proxy-panel {
        border: 1px solid #ccc;
        border-radius: 8px;
        background: #f8f8f8;
        padding: 0.75rem;
    }

    .proxy-row {
        display: flex;
        gap: 0.5rem;
        align-items: center;
    }

    .proxy-row input {
        flex: 1;
        min-width: 0;
        padding: 8px;
        border: 1px solid #ccc;
        border-radius: 6px;
    }

    .proxy-add,
    .proxy-remove {
        border: none;
        border-radius: 6px;
        padding: 8px 12px;
        cursor: pointer;
        background-color: var(--colors-primary);
        color: white;
    }

    .proxy-list {
        margin-top: 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .proxy-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 0.5rem;
    }

    .proxy-error {
        margin-top: 0.5rem;
        color: #b00020;
        font-size: 0.9rem;
    }

    .proxy-empty {
        color: #666;
        font-size: 0.9rem;
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

    .presetBtn {
        margin-top: 0.3em;
        background-color: var(--colors-secondary);
        color: black;
        border: none;
        border-radius: 4px;
        font-size: 18px;
        padding: 10px 20px;
        cursor: pointer;
    }

    .presetBtn:hover {
        background-color: color-mix(
            in srgb,
            var(--colors-secondary),
            black 10%
        );
    }

    .toggleRow {
        display: flex;
        align-items: center;
        gap: 0.6em;
        margin-top: 0.8em;
        color: var(--colors-text);
    }

    .toggleRow input {
        width: 18px;
        height: 18px;
        margin: 0;
    }
</style>
