import { toErrorMessage } from "$lib/errors";
import { endSession, fetchSessionStatus } from "$lib/api/session";

export function displayHostSessionCode(sessionCode: string): string {
  return sessionCode || "Not connected";
}

export function hostInviteUrl(sessionCode: string): string {
  const url = new URL("/home", window.location.origin);
  if (sessionCode) {
    url.searchParams.set("sessionCode", sessionCode);
  }
  return url.toString();
}

export async function ensureHostedSessionCode(currentSessionCode: string): Promise<string> {
  if (currentSessionCode) {
    return currentSessionCode;
  }

  try {
    const status = await fetchSessionStatus();
    return status.hosting?.session_code ?? "";
  } catch {
    return "";
  }
}

export async function endHostedSessionWithRedirect(
  setError: (message: string) => void,
): Promise<boolean> {
  setError("");
  try {
    await endSession();
    window.location.href = "/home";
    return true;
  } catch (error) {
    setError(toErrorMessage(error, "Failed to end session."));
    return false;
  }
}
