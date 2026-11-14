import { apiUrl } from "$lib/api/base";
import { ApiRequestError, isRecord, parseError, readJsonUnknown } from "$lib/api/response";

export type AttendeeRecord = {
  id: number;
  name: string;
  andrew_id: string;
  is_proxy_holder: boolean;
  proxy_for: string[];
};

export type AttendanceResponse = {
  session_code: string;
  headcount: number;
  attendees: AttendeeRecord[];
};

function isAttendeeRecord(value: unknown): value is AttendeeRecord {
  if (!isRecord(value)) return false;
  return (
    typeof value.id === "number" &&
    typeof value.name === "string" &&
    typeof value.andrew_id === "string" &&
    typeof value.is_proxy_holder === "boolean" &&
    Array.isArray(value.proxy_for) &&
    value.proxy_for.every((entry) => typeof entry === "string")
  );
}

function parseAttendanceResponse(payload: unknown): AttendanceResponse {
  if (
    !isRecord(payload) ||
    typeof payload.session_code !== "string" ||
    typeof payload.headcount !== "number" ||
    !Array.isArray(payload.attendees) ||
    !payload.attendees.every(isAttendeeRecord)
  ) {
    throw new Error("Invalid attendance response");
  }

  return {
    session_code: payload.session_code,
    headcount: payload.headcount,
    attendees: payload.attendees,
  };
}

export async function fetchAttendance(sessionCode: string): Promise<AttendanceResponse> {
  const response = await fetch(apiUrl(`/session/${encodeURIComponent(sessionCode)}/attendance`), {
    credentials: "include",
    cache: "no-store",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseAttendanceResponse(payload);
}
