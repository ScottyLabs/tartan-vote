import { endSession, fetchSessionStatus } from "$lib/api/session";

export function displayHostSessionCode(sessionCode: string): string {
  return sessionCode || "Not connected";
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

function toErrorMessage(error: unknown, fallback: string): string {
  if (error instanceof Error && error.message.trim() !== "") {
    return error.message;
  }
  return fallback;
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
