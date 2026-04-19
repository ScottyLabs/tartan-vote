import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import dotenv from "dotenv";

import { betterAuth } from "better-auth";
import { genericOAuth } from "better-auth/plugins";
import { Pool } from "pg";

const thisFilePath = fileURLToPath(import.meta.url);
const thisDirPath = dirname(thisFilePath);
dotenv.config({ path: resolve(thisDirPath, "../.env") });

const port = Number(process.env.BETTER_AUTH_PORT || 3005);
const frontendBaseUrl = process.env.FRONTEND_BASE_URL || "http://localhost:5173";
const appBaseUrl = process.env.APP_BASE_URL || "http://localhost:8080";
const betterAuthUrl = process.env.BETTER_AUTH_URL || `http://localhost:${port}`;
const providerId = process.env.BETTER_AUTH_PROVIDER_ID || "cmu-sso";
const issuer = process.env.OIDC_ISSUER;
const clientId = process.env.OIDC_CLIENT_ID;
const clientSecret = process.env.OIDC_CLIENT_SECRET;
const redirectURI =
    process.env.OIDC_REDIRECT_URI ||
    `${(process.env.APP_BASE_URL || "http://localhost:8080").replace(/\/$/, "")}/auth/callback`;

const allowedOrigins =
    process.env.CORS_ALLOWED_ORIGINS?.split(",")
        .map((origin) => origin.trim())
        .filter((origin) => origin.length > 0) ?? [];

const trustedOrigins = Array.from(
    new Set([...allowedOrigins, frontendBaseUrl, appBaseUrl, betterAuthUrl]),
);

if (!issuer || !clientId || !clientSecret) {
    throw new Error("OIDC_ISSUER, OIDC_CLIENT_ID, and OIDC_CLIENT_SECRET must be set");
}

const authConfig = {
    baseURL: betterAuthUrl,
    trustedOrigins,
    user: {
        modelName: "auth_user",
    },
    session: {
        modelName: "auth_session",
    },
    account: {
        modelName: "auth_account",
    },
    verification: {
        modelName: "auth_verification",
    },
    plugins: [
        genericOAuth({
            config: [
                {
                    providerId,
                    discoveryUrl: `${issuer.replace(/\/$/, "")}/.well-known/openid-configuration`,
                    clientId,
                    clientSecret,
                    ...(redirectURI ? { redirectURI } : {}),
                    scopes: ["openid", "email", "profile"],
                    requireIssuerValidation: true,
                },
            ],
        }),
    ],
};

if (process.env.DATABASE_URL) {
    authConfig.database = new Pool({
        connectionString: process.env.DATABASE_URL,
    });
}

export const auth = betterAuth(authConfig);
