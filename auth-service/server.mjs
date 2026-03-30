import cors from "cors";
import express from "express";
import { toNodeHandler } from "better-auth/node";
import { auth } from "./auth.mjs";

const port = Number(process.env.BETTER_AUTH_PORT || 3005);
const frontendBaseUrl = process.env.FRONTEND_BASE_URL || "http://localhost:5173";
const appBaseUrl = process.env.APP_BASE_URL || "http://localhost:8080";
const allowedOrigins =
    process.env.CORS_ALLOWED_ORIGINS?.split(",")
        .map((origin) => origin.trim())
        .filter((origin) => origin.length > 0) ?? [];

const corsOrigins = Array.from(
    new Set([
        ...allowedOrigins,
        frontendBaseUrl,
        appBaseUrl,
    ]),
);

const app = express();

app.use(
    cors({
        origin: corsOrigins,
        credentials: true,
    }),
);

app.get("/health", (_req, res) => {
    res.json({ ok: true });
});

app.all("/api/auth/*", toNodeHandler(auth));
app.all("/api/auth/*splat", toNodeHandler(auth));

app.listen(port, () => {
    console.log(`Auth service listening on http://localhost:${port}`);
});
