import { createAuthClient } from "better-auth/svelte";
import { genericOAuthClient } from "better-auth/client/plugins";

const baseURL =
    import.meta.env.VITE_BETTER_AUTH_BASE_URL ||
    "http://localhost:3005/api/auth";

export const authClient = createAuthClient({
    baseURL,
    plugins: [genericOAuthClient()],
});
