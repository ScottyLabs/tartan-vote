<script lang="ts">
    import WaitingPage from "./screens/WaitingPage.svelte";
    import { slide } from "svelte/transition";
    import SignIn from "./screens/SignIn.svelte";
    import Home from "./screens/Home.svelte";
    import VotingMotion from "./screens/VotingMotion.svelte";
    import SessionCreation from "./screens/SessionCreation.svelte";
    import ResultsAdmin from "./screens/ResultsAdmin.svelte";
    import ResultsVoter from "./screens/ResultsVoter.svelte";
    import MotionRunningAdmin from "./screens/MotionRunningAdmin.svelte";
    import { Event } from "./lib/models/Event";
    import { User } from "./lib/models/User";

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    let screen = $state("auth");

    // Current Events and User; this is passed to screens and is updated by screens
    let currentUser = $state<User | null>(null);
    let currentEvent = $state<Event | null>(null);
    let adminActiveEvent = $state<{
        id: number;
        name: string;
        event_type: string;
        status: string;
        start_time: string;
    } | null>(null);
    let createSessionPayload = $state<string | null>(null);
    let globalSessionCode = $state<string | null>(null);

    let bgDark = getComputedStyle(document.documentElement)
        .getPropertyValue("--colors-backgroundDark")
        .trim();
    let bgLight = getComputedStyle(document.documentElement)
        .getPropertyValue("--colors-background")
        .trim();

    $effect(() => {
        if (screen === "auth") {
            document.body.style.backgroundColor = bgLight;
        } else if (screen === "join") {
            document.body.style.backgroundColor = bgLight;
        } else if (screen === "voting") {
            document.body.style.backgroundColor = bgDark;
        } else if (screen === "votingMotion") {
            document.body.style.backgroundColor = bgDark;
        } else if (screen === "SessionCreation") {
            document.body.style.backgroundColor = bgDark;
        } else if (screen === "MotionRunningAdmin") {
            document.body.style.backgroundColor = bgDark;
        } else if (screen === "ResultsAdmin") {
            document.body.style.backgroundColor = bgLight;
        } else {
            document.body.style.backgroundColor = bgLight;
        }
    });
</script>

{#if screen === "auth"}
    <div transition:slide>
        <SignIn onNext={() => (screen = "join")} />
    </div>
{:else if screen === "join"}
    <div transition:slide>
        <Home
            toVoter={(sessionCode: string) => { globalSessionCode = sessionCode; screen = "waiting"}}
            toAdmin={(sessionCode: string) => { createSessionPayload = sessionCode; screen = "SessionCreation"}}
        />
    </div>
{:else if screen === "waiting"}
    <div transition:slide>
        <WaitingPage
            sessionCode={globalSessionCode}
        />
    </div>
{:else if screen === "votingMotion"}
    <div transition:slide>
        <VotingMotion
            event={currentEvent}
            user={currentUser}
            onNext={() => (screen = "ResultsVoter")}
        />
    </div>
{:else if screen === "SessionCreation"}
    <div transition:slide>
        <SessionCreation
            onNext={() => (screen = "ResultsAdmin")}
            onBack={() => (screen = "join")}
            onEventStarted={(event: {
                id: number;
                name: string;
                event_type: string;
                status: string;
                start_time: string;
            }) => {
                adminActiveEvent = event;
                screen = "MotionRunningAdmin";
            }}
            sessionCode = {createSessionPayload}
        />
    </div>
{:else if screen === "MotionRunningAdmin"}
    <div transition:slide>
        <MotionRunningAdmin
            event={adminActiveEvent}
            sessionCode={createSessionPayload}
            onBack={() => (screen = "SessionCreation")}
            onEnd={() => {
                adminActiveEvent = null;
                screen = "SessionCreation";
            }}
        />
    </div>
{:else if screen === "ResultsAdmin"}
    <div transition:slide>
        <ResultsAdmin
            onNext={() => {
                createSessionPayload = null;
                screen = "join";
            }}
        />
    </div>
{:else if screen === "ResultsVoter"}
    <div transition:slide>
        <ResultsVoter
            event={currentEvent}
            user={currentUser}
            onNext={() => (screen = "join")}
        />
    </div>
{/if}
