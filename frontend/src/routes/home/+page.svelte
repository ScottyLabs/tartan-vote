<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { ApiRequestError } from "$lib/api/response";
  import LogOut from "@lucide/svelte/icons/log-out";
  import { onMount } from "svelte";
  import { fetchAuthStatus } from "$lib/api/auth";
  import { apiUrl } from "$lib/api/base";
  import {
    createSession as createSessionRequest,
    fetchSessionStatus,
    joinSession as joinSessionRequest,
  } from "$lib/api/session";
  import AppFooter from "$lib/components/AppFooter.svelte";
  import logoUrl from "$lib/assets/tartanvote-logo.svg?url";

  let sessionCode = $state(page.url.searchParams.get("sessionCode")?.trim() || "");
  let checkingAuth = $state(true);
  let authLoadError = $state("");
  let userName = $state<string | null>(null);
  let userAndrewId = $state<string | null>(null);
  let sessionActionError = $state("");
  let creatingSession = $state(false);
  let joiningSession = $state(false);
  let hasActiveHostedSession = $state(false);
  let activeParticipatingSessionCode = $state("");
  const hasActiveParticipatingSession = $derived(
    activeParticipatingSessionCode !== "",
  );

  const greetingName = $derived(userName?.trim() || userAndrewId?.trim() || "there");
  const hostButtonLabel = $derived.by(() => {
    if (creatingSession) {
      return hasActiveHostedSession ? "Resuming..." : "Creating...";
    }
    return hasActiveHostedSession
      ? "Resume hosted session"
      : "Create a new session";
  });
  const joinButtonLabel = $derived.by(() => {
    if (joiningSession) {
      return hasActiveParticipatingSession ? "Resuming..." : "Joining...";
    }
    return hasActiveParticipatingSession ? "Resume session" : "Join session";
  });

  async function loadAuthAndSessionState() {
    authLoadError = "";
    try {
      const status = await fetchAuthStatus();
      if (!status.logged_in) {
        void goto("/", { replaceState: true });
        return;
      }
      userName = status.user_name;
      userAndrewId = status.user_andrew_id;
      checkingAuth = false;
      await syncSessionState();
    } catch {
      checkingAuth = false;
      authLoadError =
        "Unable to reach the server. Check that the backend is running, then retry.";
    }
  }

  onMount(() => {
    void loadAuthAndSessionState();
  });

  function signOut() {
    window.location.href = apiUrl("/auth/logout");
  }

  async function getSessionInvolvementCodes(): Promise<{
    hostedSessionCode: string;
    participatingSessionCode: string;
  }> {
    try {
      const status = await fetchSessionStatus();
      return {
        hostedSessionCode: status.hosting?.session_code.trim() ?? "",
        participatingSessionCode: status.participating?.session_code.trim() ?? "",
      };
    } catch {
      return {
        hostedSessionCode: "",
        participatingSessionCode: "",
      };
    }
  }

  async function syncSessionState() {
    const { hostedSessionCode, participatingSessionCode } =
      await getSessionInvolvementCodes();
    hasActiveHostedSession = hostedSessionCode !== "";
    activeParticipatingSessionCode = participatingSessionCode;
  }

  async function redirectToHostedSessionIfPresent(): Promise<boolean> {
    const { hostedSessionCode, participatingSessionCode } =
      await getSessionInvolvementCodes();
    hasActiveHostedSession = hostedSessionCode !== "";
    activeParticipatingSessionCode = participatingSessionCode;
    if (!hostedSessionCode) {
      return false;
    }
    const searchParams = new URLSearchParams({
      sessionCode: hostedSessionCode,
    });
    window.location.href = `/host?${searchParams.toString()}`;
    return true;
  }

  function redirectToParticipatingSession(
    sessionCodeToOpen: string,
    destination: "proxy" | "waiting" = "proxy",
  ): void {
    const searchParams = new URLSearchParams({
      sessionCode: sessionCodeToOpen,
    });
    window.location.href = `/${destination}?${searchParams.toString()}`;
  }

  async function resumeParticipatingSession(sessionCodeToOpen: string): Promise<void> {
    await joinSessionRequest(sessionCodeToOpen);
    redirectToParticipatingSession(sessionCodeToOpen, "waiting");
  }

  async function createSession() {
    if (creatingSession || joiningSession) return;
    sessionActionError = "";
    creatingSession = true;
    try {
      if (await redirectToHostedSessionIfPresent()) {
        return;
      }
      const response = await createSessionRequest();
      const searchParams = new URLSearchParams({
        sessionCode: response.session_code,
      });
      window.location.href = `/host?${searchParams.toString()}`;
    } catch (error) {
      if (await redirectToHostedSessionIfPresent()) {
        return;
      }
      sessionActionError =
        error instanceof Error ? error.message : "Failed to create session.";
    } finally {
      creatingSession = false;
    }
  }

  function updateSessionCode(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    sessionCode = input.value.replaceAll(/^\s+/gu, "").replaceAll(/\s+/gu, "-");
  }

  async function joinSession() {
    if (joiningSession || creatingSession) return;
    const normalizedSessionCode = sessionCode.replaceAll(/^-+|-+$/gu, "");
    if (normalizedSessionCode === "" && !hasActiveParticipatingSession) return;
    sessionActionError = "";
    joiningSession = true;

    try {
      // Re-check server state so a leave/removal in another tab can't leave
      // this page blocking joins with a stale "already in session" guard.
      await syncSessionState();

      if (hasActiveParticipatingSession) {
        if (
          normalizedSessionCode === "" ||
          normalizedSessionCode === activeParticipatingSessionCode
        ) {
          await resumeParticipatingSession(activeParticipatingSessionCode);
          return;
        }

        sessionActionError =
          `You're already in session ${activeParticipatingSessionCode}. ` +
          "Resume it before joining another.";
        return;
      }

      if (normalizedSessionCode === "") {
        return;
      }

      await joinSessionRequest(normalizedSessionCode);
      redirectToParticipatingSession(normalizedSessionCode, "proxy");
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Failed to join session.";

      if (error instanceof ApiRequestError && error.status === 409) {
        await syncSessionState();
        if (hasActiveParticipatingSession) {
          if (
            normalizedSessionCode === "" ||
            normalizedSessionCode === activeParticipatingSessionCode
          ) {
            try {
              await resumeParticipatingSession(activeParticipatingSessionCode);
              return;
            } catch (resumeError) {
              sessionActionError =
                resumeError instanceof Error
                  ? resumeError.message
                  : "Failed to resume session.";
              return;
            }
          }
          sessionActionError =
            `You're already in session ${activeParticipatingSessionCode}. ` +
            "Resume it before joining another.";
          return;
        }
      }

      sessionActionError = message;
    } finally {
      joiningSession = false;
    }
  }
</script>

<svelte:head>
  <title>TartanVote | Home</title>
</svelte:head>

{#if !checkingAuth}
  <main
    class="relative mx-auto min-h-svh w-[min(100%,393px)] overflow-hidden bg-(image:--gradient-screen-signin) text-grey-900 sm:m-0 sm:w-full sm:rounded-none sm:shadow-none"
  >
    <header
      class="flex h-28 items-start justify-between bg-red-600 pt-10 pr-4 pb-10 pl-4 shadow-[0_4px_4px_rgb(0_0_0_/_0.25)] sm:items-center sm:px-[clamp(24px,3.6vw,69px)] sm:py-[clamp(30px,3vw,39px)]"
    >
      <a
        class="flex h-7 w-38 items-center gap-1 text-red-600 sm:h-[clamp(35px,3.2vw,61px)] sm:w-[clamp(150px,13.5vw,260px)] sm:gap-[clamp(3px,0.42vw,8px)]"
        href="/"
        aria-label="TartanVote home"
      >
        <img
          class="block h-6 w-8 shrink-0 sm:h-[clamp(25px,3.1vw,59px)] sm:w-[clamp(33px,4vw,77px)]"
          src={logoUrl}
          alt=""
          width="33"
          height="25"
        />
        <span
          class="font-brand text-2xl leading-none text-red-600 [-webkit-text-stroke:5px_var(--color-white)] [paint-order:stroke_fill] sm:text-[clamp(26.03px,2.1vw,40px)] sm:[-webkit-text-stroke:clamp(5px,0.26vw,10px)_var(--color-white)]"
          ><span class="text-black">Tartan</span>Vote</span
        >
      </a>

      <div class="flex flex-col items-end gap-1 sm:flex-row sm:items-center sm:gap-3">
        {#if userAndrewId}
          <p
            class="text-xs leading-3 font-medium text-white sm:text-[clamp(12px,1.04vw,18px)] sm:leading-[clamp(16px,1.4vw,24px)]"
          >
            {userAndrewId}
          </p>
        {/if}
        <button
          class="inline-flex h-9 w-20 cursor-pointer items-center justify-center gap-1 rounded-full bg-grey-100 text-xs leading-3 text-slate-800 sm:h-[clamp(42px,3.2vw,61px)] sm:w-[clamp(110px,10.4vw,199px)] sm:gap-[clamp(4px,0.42vw,8px)] sm:text-[clamp(14px,1.25vw,24px)] sm:leading-[clamp(20px,1.7vw,32px)]"
          type="button"
          onclick={signOut}
        >
          <LogOut
            class="size-3 sm:size-[clamp(12px,1.05vw,20px)]"
            strokeWidth={2.5}
            aria-hidden="true"
          />
          Sign out
        </button>
      </div>
    </header>

    <section
      class="mx-auto mt-31 flex w-88 flex-col items-center gap-2 max-[374px]:w-[calc(100%-28px)] sm:mt-[clamp(82px,10vw,108px)] sm:w-[min(calc(100vw-48px),870px)] sm:gap-[clamp(12px,1.15vw,22px)]"
      aria-labelledby="home-title"
    >
      <div
        class="flex h-14 w-88 flex-col items-center gap-1 text-center max-[374px]:w-[calc(100%-28px)] sm:h-[clamp(66px,5.2vw,100px)] sm:w-full sm:gap-[clamp(6px,0.52vw,10px)]"
      >
        <h1
          class="text-lg leading-5 font-semibold text-black sm:text-[clamp(20px,1.56vw,30px)] sm:leading-[clamp(26px,1.98vw,38px)]"
          id="home-title"
        >
          Hello, {greetingName}
        </h1>
        <p
          class="flex h-8 w-86 items-center justify-center text-sm leading-5 text-grey-700 max-[374px]:w-full sm:h-auto sm:w-[min(100%,843px)] sm:px-1 sm:py-[clamp(5px,0.52vw,10px)] sm:text-[clamp(16px,1.25vw,24px)] sm:leading-[clamp(22px,1.67vw,32px)]"
        >
          Join an existing session or host your own.
        </p>
      </div>

      {#if authLoadError}
        <div
          class="flex w-88 flex-col items-center gap-3 rounded-sm border border-red-200 bg-white px-4 py-4 max-[374px]:w-[calc(100%-28px)] sm:w-[min(calc(100vw-48px),700px)]"
          role="alert"
        >
          <p class="text-center text-sm font-semibold text-red-700">{authLoadError}</p>
          <button
            class="inline-flex h-9 cursor-pointer items-center justify-center rounded-full bg-red-600 px-4 text-xs font-medium text-white hover:bg-red-700"
            type="button"
            onclick={() => {
              checkingAuth = true;
              void loadAuthAndSessionState();
            }}
          >
            Retry
          </button>
        </div>
      {:else}
      <section
        class="flex h-63 w-88 flex-col items-center rounded-sm border border-grey-200 bg-white px-3 py-5 max-[374px]:w-[calc(100%-28px)] sm:h-[clamp(310px,26.6vw,510px)] sm:w-[min(calc(100vw-48px),700px)] sm:px-[clamp(24px,3.9vw,75px)] sm:py-[clamp(28px,2.5vw,48px)]"
        aria-label="Session actions"
      >
        <form
          class="flex w-82 flex-col items-center gap-4 max-[374px]:w-full sm:w-[min(100%,550px)] sm:gap-[clamp(18px,1.46vw,28px)]"
          onsubmit={(event) => {
            event.preventDefault();
            void joinSession();
          }}
        >
          <label
            class="w-79 text-base leading-6 font-semibold text-black max-[374px]:w-full sm:w-auto sm:text-center sm:text-[clamp(20px,1.56vw,30px)] sm:leading-[clamp(26px,1.98vw,38px)]"
            for="session-code">Session Code</label
          >
          <input
            class="h-10 w-69 rounded-md border border-slate-400 px-4 text-center text-sm leading-5 font-medium text-grey-900 placeholder:text-slate-400 focus:border-red-600 focus:outline-3 focus:outline-[color-mix(in_srgb,var(--color-red-600),transparent_78%)] max-[374px]:w-full sm:h-[clamp(46px,3.65vw,70px)] sm:w-[min(100%,490px)] sm:rounded-[clamp(5.96px,0.52vw,10px)] sm:text-[clamp(16px,1.25vw,24px)] sm:leading-[clamp(22px,1.67vw,32px)]"
            id="session-code"
            type="text"
            value={sessionCode}
            oninput={updateSessionCode}
            placeholder="ex: happy-giraffe"
            autocomplete="off"
            autocapitalize="none"
            spellcheck="false"
          />
          <button
            class="inline-flex h-9 min-h-9 w-75 shrink-0 basis-9 cursor-pointer items-center justify-center gap-1 rounded-full bg-red-600 px-4 text-xs leading-5 font-medium text-white shadow-[0_2.39px_2.39px_rgb(0_0_0_/_0.25)] hover:bg-red-700 disabled:cursor-not-allowed disabled:bg-red-200 disabled:shadow-none disabled:hover:bg-red-200 max-[374px]:w-full sm:h-[clamp(42px,3.13vw,60px)] sm:min-h-[clamp(42px,3.13vw,60px)] sm:w-[min(100%,500px)] sm:text-[clamp(15px,1.04vw,20px)] sm:leading-[clamp(21px,1.46vw,28px)] sm:shadow-[0_clamp(2.39px,0.21vw,4px)_2px_rgb(0_0_0_/_0.25)]"
            type="submit"
            disabled={(!sessionCode.trim() && !hasActiveParticipatingSession) || joiningSession || creatingSession}
          >
            <div class="flex items-baseline justify-center gap-1">
              <span>{joinButtonLabel}</span>
              {#if hasActiveParticipatingSession && !joiningSession}
                <span class="font-mono"
                  >({activeParticipatingSessionCode})</span
                >
              {/if}
            </div>
          </button
          >
        </form>

        <div
          class="my-3 grid h-8 w-75 grid-cols-[1fr_auto_1fr] items-center gap-2 text-sm font-light text-grey-700 max-[374px]:w-full sm:mt-[clamp(12px,1.04vw,20px)] sm:mb-[clamp(14px,1.15vw,22px)] sm:h-[clamp(32px,2.08vw,40px)] sm:w-[min(100%,550px)] sm:gap-[clamp(8px,0.52vw,10px)] sm:text-[clamp(16px,1.25vw,24px)] sm:leading-[clamp(22px,1.67vw,32px)] sm:font-medium"
          aria-hidden="true"
        >
          <span class="h-px bg-grey-200"></span><strong
            class="sm:bg-white sm:px-[clamp(8px,0.52vw,10px)]">OR</strong
          ><span class="h-px bg-grey-200"></span>
        </div>

        <button
          class="inline-flex h-9 min-h-9 w-75 shrink-0 basis-9 cursor-not-allowed items-center justify-center rounded-full bg-red-200 px-4 text-xs leading-5 font-medium text-white shadow-[0_2.39px_2.39px_rgb(0_0_0_/_0.25)] max-[374px]:w-full sm:hidden"
          type="button"
          disabled>Create a new session (not available on mobile)</button
        >
        <button
          class="hidden h-[clamp(42px,3.13vw,60px)] min-h-[clamp(42px,3.13vw,60px)] w-[min(100%,500px)] shrink-0 cursor-pointer items-center justify-center rounded-full bg-red-600 px-4 text-[clamp(15px,1.04vw,20px)] leading-[clamp(21px,1.46vw,28px)] font-medium text-white shadow-[0_clamp(2.39px,0.21vw,4px)_2px_rgb(0_0_0_/_0.25)] hover:bg-red-700 max-[374px]:w-full sm:inline-flex"
          type="button"
          onclick={() => void createSession()}
          disabled={creatingSession || joiningSession}
          >{hostButtonLabel}</button
        >
        {#if sessionActionError}
          <p class="mt-3 text-center text-sm font-semibold text-red-700" role="alert">
            {sessionActionError}
          </p>
        {/if}
      </section>
      {/if}
    </section>

    <AppFooter wide />
  </main>
{/if}
