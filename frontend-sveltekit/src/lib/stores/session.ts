import { writable } from 'svelte/store';

/** Holds the active session/meeting code the user is joined to. Persisted for nav. */
export const sessionCode = writable<string | null>(null);

/** Short notice banner shown on waiting/voting pages (e.g. "You are a proxy for…") */
export const waitingNotice = writable<string>('');

/** The last/current active event the voter is interacting with. */
export const currentEvent = writable<null | {
  id: number;
  name: string;
  event_type: string;
  data: EventData;
}>(null);
