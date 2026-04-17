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
    import ProxySetup from "./screens/ProxySetup.svelte";
    import { Event } from "./lib/models/Event";
    import { User } from "./lib/models/User";

    const API_BASE = import.meta.env.VITE_API_BASE || "";

    type ActiveEvent = {
        id: number;
        name: string;
        event_type: string;
        data: EventData;
    };

    let screen = $state("auth");

    // Current Events and User; this is passed to screens and is updated by screens
    let currentUser = $state<User | null>(null);
    let currentEvent = $state<ActiveEvent | null>(null);
    let adminActiveEvent = $state<{
        id: number;
        name: string;
        event_type: string;
        status: string;
        start_time: string;
    } | null>(null);
    let createSessionPayload = $state<string | null>(null);
    let globalSessionCode = $state<string | null>(null);
    let waitingNotice = $state<string | null>(null);
</script>

{#if screen === "auth"}
    <div transition:slide>
        <SignIn onNext={() => (screen = "join")} />
    </div>
{:else if screen === "join"}
    <div transition:slide>
        <Home
            toVoter={(sessionCode: string) => { globalSessionCode = sessionCode; screen = "proxySetup"}}
            toAdmin={(sessionCode: string) => { createSessionPayload = sessionCode; screen = "SessionCreation"}}
        />
    </div>
{:else if screen === "proxySetup"}
    <div transition:slide>
        <ProxySetup
            sessionCode={globalSessionCode}
            onBack={() => (screen = "join")}
            onNext={(notice: string | null) => {
                waitingNotice = notice;
                screen = "waiting";
            }}
        />
    </div>
{:else if screen === "waiting"}
    <div transition:slide>
        <WaitingPage
            sessionCode={globalSessionCode}
            notice={waitingNotice}
            onEventFound={(event: ActiveEvent) => {
                currentEvent = event;
                waitingNotice = null;
                screen = "votingMotion";
            }}
        />
    </div>
{:else if screen === "votingMotion"}
    <div transition:slide>
        <VotingMotion
            event={currentEvent}
            user={currentUser}
            sessionCode={globalSessionCode}
            onNext={(destination: "results" | "session") => {
                if (destination === "results") {
                    screen = "ResultsVoter";
                    return;
                }

                currentEvent = null;
                waitingNotice = null;
                screen = "waiting";
            }}
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
            sessionCode={globalSessionCode}
            onNext={(destination: "session" | "join") => {
                currentEvent = null;
                waitingNotice = null;

                if (destination === "join") {
                    globalSessionCode = null;
                    screen = "join";
                    return;
                }

                screen = "waiting";
            }}
        />
    </div>
{/if}
