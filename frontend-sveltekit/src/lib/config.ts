import {
  PUBLIC_API_BASE,
  PUBLIC_BETTER_AUTH_BASE_URL,
  PUBLIC_BETTER_AUTH_PROVIDER_ID
} from '$env/static/public';

export const API_BASE = PUBLIC_API_BASE ?? '';
export const AUTH_BASE_URL =
  PUBLIC_BETTER_AUTH_BASE_URL ?? 'http://localhost:3005/api/auth';
export const AUTH_PROVIDER_ID = PUBLIC_BETTER_AUTH_PROVIDER_ID ?? 'cmu-sso';
