import { createAuthClient } from 'better-auth/svelte';
import { genericOAuthClient } from 'better-auth/client/plugins';
import { AUTH_BASE_URL, AUTH_PROVIDER_ID } from './config';

export const authClient = createAuthClient({
  baseURL: AUTH_BASE_URL,
  plugins: [genericOAuthClient()]
});

export const PROVIDER_ID = AUTH_PROVIDER_ID;
