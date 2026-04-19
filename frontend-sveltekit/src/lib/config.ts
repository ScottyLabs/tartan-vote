// Read env vars straight from Vite so the monorepo's existing VITE_* variables
// in `../.env` are picked up without renaming. SvelteKit's $env/static/public
// would require a PUBLIC_ prefix (or a customized publicPrefix), which the
// original frontend doesn't use.
const env = (import.meta as ImportMeta & {
  env: Record<string, string | undefined>;
}).env;

export const API_BASE =
  env.VITE_API_BASE ?? env.PUBLIC_API_BASE ?? '';

export const AUTH_BASE_URL =
  env.VITE_BETTER_AUTH_BASE_URL ??
  env.PUBLIC_BETTER_AUTH_BASE_URL ??
  'http://localhost:3005/api/auth';

export const AUTH_PROVIDER_ID =
  env.VITE_BETTER_AUTH_PROVIDER_ID ??
  env.PUBLIC_BETTER_AUTH_PROVIDER_ID ??
  'cmu-sso';
