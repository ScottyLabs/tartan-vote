# TartanVote — SvelteKit + Tailwind frontend

A redesigned frontend for the CMU Student Senate voting app, implemented against the same backend as `../frontend/`.

The original Vite + Svelte 5 app in `../frontend/` is untouched. This package mirrors its API usage and screen flow one-for-one, but in SvelteKit with Tailwind and the new design system.

## Scripts

```bash
bun install          # or: npm install / pnpm install
bun run dev          # http://localhost:5173
bun run build
bun run preview
bun run check        # svelte-check
```

## Environment

Uses the same backend + better-auth setup as the original app. It reads its `.env` from the monorepo root (`envDir: '..'` in `vite.config.ts`), matching the original.

Public variables — either `VITE_*` (default in the monorepo `.env`) or `PUBLIC_*` are accepted; `src/lib/config.ts` reads both:

| Variable | Purpose | Default |
| --- | --- | --- |
| `VITE_API_BASE` / `PUBLIC_API_BASE` | Backend base URL | empty (same-origin) |
| `VITE_BETTER_AUTH_BASE_URL` / `PUBLIC_BETTER_AUTH_BASE_URL` | better-auth endpoint | `http://localhost:3005/api/auth` |
| `VITE_BETTER_AUTH_PROVIDER_ID` / `PUBLIC_BETTER_AUTH_PROVIDER_ID` | OAuth provider id | `cmu-sso` |

> The monorepo's root `.env` already ships the `VITE_*` versions used by the original frontend, so this app works without any additional setup. See `.env.example`.

## Routes

| Path | Screen | Equivalent in original |
| --- | --- | --- |
| `/` | Redirect based on auth | `App.svelte` boot |
| `/signin` | CMU SSO sign-in | `SignIn.svelte` |
| `/join` | Join a session / create one | `Home.svelte` |
| `/proxy-setup` | Role + proxy setup (voter) | `ProxySetup.svelte` |
| `/waiting` | Waiting for next event | `WaitingPage.svelte` |
| `/vote/[eventId]` | Voter ballot | `VotingMotion.svelte` |
| `/results/[eventId]` | Voter results | `ResultsVoter.svelte` |
| `/host/[code]` | Host dashboard + push modals | `SessionCreation.svelte` |
| `/host/[code]/motion/[eventId]` | Live motion/election monitor | `MotionRunningAdmin.svelte` |

## Components

All in `src/lib/components/`:

- `Logo`, `Button`, `Chip`, `TagPill`
- `Input`, `LongTextInput`, `SelectMenu`, `ArrayEditor`, `TimeScroller`
- `Modal` (with `eyebrow`, `title`, `subtitle`, `footer` snippet)
- `VoteOption` (pill-style radio)
- `ProgressBar`, `StatCard`
- `HostShell` (dark rail + navigation)
- `MotionLiveResults` (polls `/events/:id/results`)

## Design system

Tokens live in `tailwind.config.js` and the component layer in `src/app.css`:

- Scarlet palette (`scarlet-50 … 700`, 500 = `#C8102E`)
- Ink palette (greys for text, borders, surfaces)
- Emerald + Amber semantic accents
- `Instrument Serif` for display, `Inter` for UI

## Design decisions vs. the original

- Uses SvelteKit file-system routing instead of a state-driven `screen` variable in `App.svelte`. Persistent session state lives in Svelte stores (`src/lib/stores/session.ts`).
- Every backend endpoint and payload shape is preserved (see `src/lib/api.ts`).
- Auth uses better-auth/svelte with the `cmu-sso` provider only — no guest access.
- Polling intervals (3s) match the original for attendance, active events, and results.
- The modal-heavy authoring flow has been kept, but gets a two-column layout (form + voter preview) on the push-motion screen.

## What is *not* included

- The `Counter.svelte` / logo placeholder demo assets from the old frontend.
- The original `hoverCard.svelte` tooltip (not currently used on any live screen in this port — add back as needed).
- SSR-side data loading — all fetches happen client-side in `onMount` to mirror the original.

## Running alongside the original

The two apps can run side-by-side during migration:

```bash
# terminal 1 — original
cd frontend && bun run dev          # port 5173

# terminal 2 — new (pick a different port)
cd frontend-sveltekit && bun run dev -- --port 5174
```
