<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import LongTextInput from "../lib/components/longTextInput.svelte";
    import SelectMenu from "../lib/components/selectMenu.svelte";
    import Popup from "../lib/components/popup.svelte";
    import backArr from "../lib/images/back_arrow.png";
    import ArrayEditor from "../lib/components/arrayEditor.svelte";
    import TimeScroller from "../lib/components/timeScroller.svelte";
    import HoverCard from "../lib/components/hoverCard.svelte";

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

    type AttendanceResponse = {
        session_code: string;
        headcount: number;
        attendees: Array<{
            id: number;
            name: string;
            andrew_id: string;
            is_proxy_holder: boolean;
            proxy_for: string[];
        }>;
    };

    type Participant = {
        id: number;
        name: string;
        andrew_id: string;
        is_proxy_holder: boolean;
        proxy_for: string[];
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
                meeting_display:
                    vote_type === "motion"
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

    let users: Participant[] = $state([]);
    let proxyHolders = $derived(
        users.filter(
            (user) => user.is_proxy_holder && user.proxy_for.length > 0,
        ),
    );

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
        {
            label: "Save detailed ballots in meeting export",
            value: "full_ballots",
        },
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
    let inspectingUser = $state<Participant | null>(null);
    let inspectingAllUsers = $state(false);
    let timerEnded = $state(false);
    let endingMeeting = $state(false);
    let endMeetingError = $state<string | null>(null);
    let latestEventId = $state<number | null>(null);
    let proxyHolderIdInput = $state("");
    let proxiedSenatorIdInput = $state("");
    let proxyAssignmentError = $state("");
    let proxyAssignments = $state<ProxyAssignmentDraft[]>([]);
    let loadingParticipants = $state(false);
    let participantsError = $state<string | null>(null);
    let participantsPollId: number | null = null;

    type ExportKind = "attendance" | "votes";
    type ExportFormat = "pdf" | "csv";

    const exportMenuOptions: {
        label: string;
        kind: ExportKind;
        format: ExportFormat;
    }[] = [
        { label: "Attendance — PDF", kind: "attendance", format: "pdf" },
        { label: "Attendance — CSV", kind: "attendance", format: "csv" },
        { label: "Votes — PDF", kind: "votes", format: "pdf" },
        { label: "Votes — CSV", kind: "votes", format: "csv" },
    ];

    let exporting = $state(false);
    let exportError = $state<string | null>(null);

    function exportMenuValue(kind: ExportKind, format: ExportFormat) {
        return `${kind}:${format}`;
    }

    let exportMenuSelection = $state(exportMenuValue("attendance", "pdf"));

    async function downloadExport() {
        if (!sessionCode || exporting) return;
        exportError = null;
        const [kind, format] = exportMenuSelection.split(":") as [
            ExportKind,
            ExportFormat,
        ];
        exporting = true;
        try {
            await exportFile(kind, format);
        } catch {
            exportError = "Download failed. Please try again.";
        } finally {
            exporting = false;
        }
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

    function inspectUser(user: Participant) {
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
        const proxy_holder_user_id = Number.parseInt(
            proxyHolderIdInput.trim(),
            10,
        );
        const proxied_senator_user_id = Number.parseInt(
            proxiedSenatorIdInput.trim(),
            10,
        );

        if (
            !Number.isFinite(proxy_holder_user_id) ||
            !Number.isFinite(proxied_senator_user_id)
        ) {
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

    async function loadParticipants() {
        if (!sessionCode) return;

        loadingParticipants = true;

        try {
            const response = await fetch(
                `${API_BASE}/session/${sessionCode}/attendance`,
                {
                    cache: "no-store",
                    credentials: "include",
                },
            );

            if (!response.ok) {
                throw new Error(
                    `Failed to fetch participants: ${response.status}`,
                );
            }

            const payload: AttendanceResponse = await response.json();
            users = payload.attendees.map((attendee) => ({
                id: attendee.id,
                name: attendee.name,
                andrew_id: attendee.andrew_id,
                is_proxy_holder: attendee.is_proxy_holder,
                proxy_for: attendee.proxy_for,
            }));
            participantsError = null;
        } catch (error) {
            participantsError = "Unable to refresh participants right now.";
        } finally {
            loadingParticipants = false;
        }
    }

    onMount(() => {
        void loadParticipants();
        participantsPollId = window.setInterval(() => {
            void loadParticipants();
        }, 3000);
    });

    onDestroy(() => {
        if (participantsPollId !== null) {
            window.clearInterval(participantsPollId);
            participantsPollId = null;
        }
    });

    async function exportFile(kind: ExportKind, format: ExportFormat) {
        const response = await fetch(
            `${API_BASE}/session/${sessionCode}/export/${kind}/${format}`,
            { method: "GET", credentials: "include" },
        );

        if (!response.ok) throw new Error(`Export failed: ${response.status}`);

        const blob = await response.blob();
        const url = URL.createObjectURL(blob);
        const anchor = document.createElement("a");
        anchor.href = url;
        anchor.download = `${sessionCode}-${kind}.${format}`;
        anchor.click();
        URL.revokeObjectURL(url);
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

            const response = await fetch(
                `${API_BASE}/events/create/${sessionCode}`,
                {
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
                },
            );
            if (!response.ok) throw new Error(`Failed: ${response.status}`);
            const event: CreatedEvent = await response.json();
            latestEventId = event.id;
            onPopupClose();
            onEventStarted?.(event);
        } catch (error) {}
    }

    async function endMeeting() {
        if (!sessionCode || endingMeeting) return;

        endingMeeting = true;
        endMeetingError = null;

        try {
            const response = await fetch(
                `${API_BASE}/session/${sessionCode}/end`,
                {
                    cache: "no-store",
                    credentials: "include",
                },
            );

            if (!response.ok) {
                throw new Error(`Failed to end session: ${response.status}`);
            }

            onPopupClose();
            goNext();
        } catch (error) {
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
        {#each users as user}
            <div
                class="slot-wrapper"
                role="group"
                onmouseenter={() => inspectUser(user)}
                onmouseleave={clearInspect}
            >
                <button class="slot">
                    {user.name?.charAt(0)}
                </button>
                <HoverCard open={inspectingUser?.id === user.id}>
                    <div class="col">
                        <div>Name: {user.name}</div>
                        <div>UserID: {user.id}</div>
                        <div>Time Created: {user.id}</div>
                        <div>
                            Proxy: {user.is_proxy_holder
                                ? `Yes (${user.proxy_for.join(", ")})`
                                : "No"}
                        </div>
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
        class="motionForm"
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
                    <button
                        type="button"
                        class="proxy-add"
                        onclick={addProxyAssignment}>Add</button
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
                                    Holder #{assignment.proxy_holder_user_id} → Senator
                                    #{assignment.proxied_senator_user_id}
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

        <div class="card-sections">
            <h1 class="card-section-label">Participants</h1>
            <div class="participants-slot">
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
                                        <div>
                                            Proxy: {user.is_proxy_holder
                                                ? `Yes (${user.proxy_for.join(", ")})`
                                                : "No"}
                                        </div>
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
            {#if participantsError}
                <p class="error card-sections-full">{participantsError}</p>
            {/if}
            <h1 class="card-section-label">Active Proxies</h1>
            <div class="container proxy-overview-body">
                {#if proxyHolders.length === 0}
                    <p class="proxy-overview-empty">
                        No active proxies in this session.
                    </p>
                {:else}
                    <div class="proxy-overview-list">
                        {#each proxyHolders as holder}
                            <div class="proxy-overview-item">
                                <strong>{holder.name}</strong>
                                <span>(ID: {holder.id})</span>
                                <span>→ {holder.proxy_for.join(", ")}</span>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>
        </div>
        <hr />
        <div class="session-actions">
            <div class="session-actions-row">
                <button onclick={pushMotion} class="btn session-action-btn">
                    Push a Motion
                </button>
                <button onclick={pushElection} class="btn session-action-btn">
                    Push an Election
                </button>
            </div>
            <div class="session-actions-row">
                <button onclick={endTimer} class="btn end-meeting-btn">
                    END MEETING
                </button>
                <div class="export-toolbar">
                    <label class="export-field" for="session-export-select">
                        <select
                            id="session-export-select"
                            class="export-select"
                            bind:value={exportMenuSelection}
                            disabled={exporting}
                        >
                            {#each exportMenuOptions as opt}
                                <option
                                    value={exportMenuValue(
                                        opt.kind,
                                        opt.format,
                                    )}
                                >
                                    {opt.label}
                                </option>
                            {/each}
                        </select>
                    </label>
                    <button
                        type="button"
                        class="btn export-btn"
                        onclick={downloadExport}
                        disabled={exporting || !sessionCode}
                    >
                        {exporting ? "Downloading…" : "Download"}
                    </button>
                </div>
            </div>
        </div>
        {#if exportError}
            <p class="error export-error">{exportError}</p>
        {/if}
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
        color: var(--color-on-primary);
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
    .export-btn {
        font-size: 14px;
        font-weight: 600;
        padding: 10px 22px;
        border-radius: 6px;
        margin-top: 0;
        flex-shrink: 0;
        box-shadow: 0 1px 2px rgb(0 0 0 / 0.06);
    }

    .session-actions {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        width: 100%;
        margin-top: 0.5em;
    }

    .session-actions-row {
        display: grid;
        grid-template-columns: 1fr 1fr;
        align-items: stretch;
        gap: 1em;
        width: 100%;
        margin-top: 0;
    }

    .session-actions .session-action-btn,
    .session-actions .end-meeting-btn {
        margin-top: 0;
        width: 100%;
        box-sizing: border-box;
        min-height: 3.25rem;
        border-radius: 6px;
        box-shadow: 0 1px 2px rgb(0 0 0 / 0.06);
    }

    .end-meeting-btn {
        padding: 10px 1.25rem;
    }

    .export-toolbar {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        align-self: stretch;
        gap: 0.75rem;
        padding: 0 0.9rem;
        border-radius: 10px;
        border: 1px solid var(--color-border);
        background: var(--color-surface-muted);
        box-shadow: inset 0 1px 0 rgb(255 255 255 / 0.65);
        box-sizing: border-box;
        min-width: 0;
    }

    .export-field {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 0.65rem;
        margin: 0;
        flex: 1 1 12rem;
        min-width: 0;
    }

    .export-field-label {
        font-size: 0.75rem;
        font-weight: 600;
        letter-spacing: 0.02em;
        text-transform: uppercase;
        color: var(--color-text-secondary);
        white-space: nowrap;
        flex-shrink: 0;
    }

    .export-select {
        flex: 1 1 auto;
        min-width: 0;
        width: 100%;
        max-width: 22rem;
        height: 42px;
        padding: 0 12px;
        border-radius: 8px;
        border: 1px solid var(--color-border);
        background: var(--color-surface);
        color: var(--color-text);
        font-size: 14px;
        line-height: 1.3;
        cursor: pointer;
        box-sizing: border-box;
        transition:
            border-color 0.15s ease,
            box-shadow 0.15s ease;
    }

    .export-select:hover:not(:disabled) {
        border-color: var(--color-border-subtle);
    }

    .export-select:focus {
        outline: none;
    }

    .export-select:focus-visible {
        border-color: color-mix(
            in srgb,
            var(--colors-primary),
            var(--color-border) 35%
        );
        box-shadow: 0 0 0 3px
            color-mix(in srgb, var(--colors-primary), transparent 78%);
    }

    .export-select:disabled {
        opacity: 0.65;
        cursor: not-allowed;
    }

    .export-error {
        margin-top: 0.35em;
        margin-bottom: 0;
    }
    .error {
        color: var(--color-danger);
        margin-top: 0.5em;
    }
    .card {
        width: fit-content;
        padding: 1.5rem;
        border-radius: 12px;
        background: var(--color-surface);
        box-shadow: var(--shadow-card);
    }

    .proxy-panel {
        border: 1px solid var(--color-border);
        border-radius: 8px;
        background: var(--color-surface-muted);
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
        border: 1px solid var(--color-border);
        border-radius: 6px;
    }

    .proxy-add,
    .proxy-remove {
        border: none;
        border-radius: 6px;
        padding: 8px 12px;
        cursor: pointer;
        background-color: var(--colors-primary);
        color: var(--color-on-primary);
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
        color: var(--color-danger);
        font-size: 0.9rem;
    }

    .proxy-empty {
        color: var(--color-text-secondary);
        font-size: 0.9rem;
    }

    .card-sections {
        display: grid;
        grid-template-columns: max-content 1fr;
        gap: 1em;
        align-items: center;
        margin-top: 0.5em;
    }

    .card-section-label {
        margin: 0;
        font-size: 1.25rem;
        color: var(--colors-primary);
    }

    .card-sections-full {
        grid-column: 1 / -1;
        margin: 0;
    }

    .participants-slot {
        display: flex;
        justify-content: center;
        align-items: center;
        justify-self: stretch;
        min-width: 0;
        width: 100%;
    }

    .participants-slot .button-list {
        display: flex;
        flex-wrap: wrap;
        align-content: center;
        gap: 1rem;
        margin-right: 0;
        min-width: calc(10 * 32px + 9 * 1rem);
    }

    .proxy-overview-body {
        min-width: 0;
        text-align: left;
    }

    .proxy-overview-empty {
        margin: 0;
        color: var(--color-text-secondary);
    }

    .proxy-overview-list {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
    }

    .proxy-overview-item {
        display: flex;
        gap: 0.4rem;
        flex-wrap: wrap;
        align-items: baseline;
    }

    .container {
        border: 2px solid var(--color-border);
        padding: 8px;
        border-radius: 8px;
        width: fit-content;
        background: var(--color-surface-muted);
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
        border: 1px solid var(--color-border);
        border-radius: 4px;
        background: var(--color-surface);
        cursor: pointer;

        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--colors-text);
    }

    .plus {
        font-weight: bold;
    }

    hr {
        width: 100%;
        border: none;
        border-top: 2px solid var(--color-divider);
        margin-top: 1em;
        margin-bottom: 1em;
    }

    h1 {
        color: var(--colors-primary);
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

    .motionForm {
        max-height: 72vh;
        overflow-y: auto;
        padding-right: 0.35rem;
    }

    .motionForm input {
        height: 44px;
        font-size: 18px;
    }

    .motionForm .submitBtn {
        padding: 10px 90px;
        font-size: 18px;
    }

    input {
        width: 100%;
        height: 50px;
        padding: 10px;
        border-radius: 6px;
        border: 1px solid var(--color-border);
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
        color: var(--color-on-primary);
        border: none;
        border-radius: 4px;
        font-size: 20px;
        padding: 10px 140px;
        cursor: pointer;
    }

    .presetBtn {
        margin-top: 0.3em;
        background-color: var(--colors-secondary);
        color: var(--color-text);
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
