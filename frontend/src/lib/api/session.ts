import { apiUrl } from "$lib/api/base";
import { ApiRequestError, isRecord, parseError, readJsonUnknown } from "$lib/api/response";

export type SessionStatus = "Open" | "Locked" | "Closed";

export type SessionInvolvement = {
  session_code: string;
  status: SessionStatus;
};

export type StatusSessionResponse = {
  in_session: boolean;
  hosting: SessionInvolvement | null;
  participating: SessionInvolvement | null;
};

export type CreateSessionResponse = {
  session_code: string;
};

export type EndSessionResponse = {
  session_code: string;
  status: SessionStatus;
};

export type SetSessionProxyRequest = {
  is_senator: boolean;
  proxy_for: string | null;
};

export type SetSessionProxyResponse = {
  vote_instance_count: number;
  is_senator: boolean;
  has_proxy: boolean;
};

function isSessionStatus(value: unknown): value is SessionStatus {
  return value === "Open" || value === "Locked" || value === "Closed";
}

function isSessionInvolvement(value: unknown): value is SessionInvolvement {
  return isRecord(value) && typeof value.session_code === "string" && isSessionStatus(value.status);
}

function parseCreateSessionResponse(payload: unknown): CreateSessionResponse {
  if (isRecord(payload) && typeof payload.session_code === "string") {
    return { session_code: payload.session_code };
  }
  throw new Error("Invalid create session response");
}

function parseSetSessionProxyResponse(payload: unknown): SetSessionProxyResponse {
  if (
    isRecord(payload) &&
    typeof payload.vote_instance_count === "number" &&
    typeof payload.is_senator === "boolean" &&
    typeof payload.has_proxy === "boolean"
  ) {
    return {
      vote_instance_count: payload.vote_instance_count,
      is_senator: payload.is_senator,
      has_proxy: payload.has_proxy,
    };
  }
  throw new Error("Invalid proxy response");
}

function parseStatusSessionResponse(payload: unknown): StatusSessionResponse {
  if (!isRecord(payload) || typeof payload.in_session !== "boolean") {
    throw new Error("Invalid session status response");
  }

  const hosting = payload.hosting;
  const participating = payload.participating;
  if (hosting !== null && hosting !== undefined && !isSessionInvolvement(hosting)) {
    throw new Error("Invalid hosting session status");
  }
  if (
    participating !== null &&
    participating !== undefined &&
    !isSessionInvolvement(participating)
  ) {
    throw new Error("Invalid participating session status");
  }

  return {
    in_session: payload.in_session,
    hosting: hosting ?? null,
    participating: participating ?? null,
  };
}

function parseEndSessionResponse(payload: unknown): EndSessionResponse {
  if (
    isRecord(payload) &&
    typeof payload.session_code === "string" &&
    isSessionStatus(payload.status)
  ) {
    return {
      session_code: payload.session_code,
      status: payload.status,
    };
  }
  throw new Error("Invalid end session response");
}

export async function createSession(): Promise<CreateSessionResponse> {
  const response = await fetch(apiUrl("/session/create"), {
    method: "POST",
    credentials: "include",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseCreateSessionResponse(payload);
}

export async function joinSession(sessionCode: string): Promise<void> {
  const response = await fetch(apiUrl(`/session/join/${encodeURIComponent(sessionCode)}`), {
    method: "POST",
    credentials: "include",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }
}

export async function setSessionProxy(
  sessionCode: string,
  request: SetSessionProxyRequest,
): Promise<SetSessionProxyResponse> {
  const response = await fetch(apiUrl(`/session/${encodeURIComponent(sessionCode)}/proxy`), {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseSetSessionProxyResponse(payload);
}

export async function fetchSessionStatus(): Promise<StatusSessionResponse> {
  const response = await fetch(apiUrl("/session/status"), {
    credentials: "include",
    cache: "no-store",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseStatusSessionResponse(payload);
}

export async function endSession(): Promise<EndSessionResponse> {
  const response = await fetch(apiUrl("/session/end"), {
    method: "POST",
    credentials: "include",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseEndSessionResponse(payload);
}
