import { apiUrl } from "$lib/api/base";

export type AuthStatus = {
  logged_in: boolean;
  user_id: number | null;
  user_name: string | null;
  user_andrew_id: string | null;
};

function isStringOrNull(value: unknown): value is string | null {
  return typeof value === "string" || value === null;
}

function isNumberOrNull(value: unknown): value is number | null {
  return typeof value === "number" || value === null;
}

function isAuthStatus(payload: unknown): payload is AuthStatus {
  if (typeof payload !== "object" || payload === null) {
    return false;
  }

  if (
    !("logged_in" in payload) ||
    !("user_id" in payload) ||
    !("user_name" in payload) ||
    !("user_andrew_id" in payload)
  ) {
    return false;
  }

  return (
    typeof payload.logged_in === "boolean" &&
    isNumberOrNull(payload.user_id) &&
    isStringOrNull(payload.user_name) &&
    isStringOrNull(payload.user_andrew_id)
  );
}

export async function fetchAuthStatus(): Promise<AuthStatus> {
  const response = await fetch(apiUrl("/auth/status"), {
    credentials: "include",
    cache: "no-store",
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch auth status (${response.status})`);
  }

  const payload: unknown = await response.json();
  if (!isAuthStatus(payload)) {
    throw new Error("Invalid auth status response");
  }

  return payload;
}
