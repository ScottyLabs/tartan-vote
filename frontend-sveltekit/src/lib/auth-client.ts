import { createAuthClient } from 'better-auth/svelte';
import { genericOAuthClient } from 'better-auth/client/plugins';
import { API_BASE, AUTH_BASE_URL, AUTH_PROVIDER_ID } from './config';

export const authClient = createAuthClient({
  baseURL: AUTH_BASE_URL,
  plugins: [genericOAuthClient()]
});

export const PROVIDER_ID = AUTH_PROVIDER_ID;

export type AuthStatus = {
  logged_in: boolean;
  user_id: number | null;
  user_name: string | null;
  user_andrew_id: string | null;
};

export async function fetchAuthStatus(): Promise<AuthStatus | null> {
  const response = await fetch(`${API_BASE}/auth/status`, {
    credentials: 'include',
    cache: 'no-store'
  });
  if (!response.ok) return null;
  return (await response.json()) as AuthStatus;
}

export async function devSignIn(): Promise<AuthStatus> {
  const response = await fetch(`${API_BASE}/auth/dev-signin`, {
    method: 'POST',
    credentials: 'include',
    cache: 'no-store'
  });
  if (!response.ok) {
    throw new Error(`Dev sign-in failed (${response.status})`);
  }
  return (await response.json()) as AuthStatus;
}

export async function devSignOut(): Promise<void> {
  await fetch(`${API_BASE}/auth/dev-signout`, {
    method: 'POST',
    credentials: 'include',
    cache: 'no-store'
  });
}
