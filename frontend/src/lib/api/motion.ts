import { apiUrl } from "$lib/api/base";
import { ApiRequestError, isRecord, parseError, readJsonUnknown } from "$lib/api/response";

export type CreateMotionRequest = {
  name: string;
  start_time?: string;
  end_time?: string;
  data?: Record<string, unknown>;
};

export type CreateMotionResponse = {
  id: number;
  name: string;
  status: string;
  start_time: string;
};

export type CheckMotionActiveMotion = {
  id: number;
  name: string;
  data: Record<string, unknown>;
};

export type CheckMotionResponse = {
  active_motion: CheckMotionActiveMotion | null;
};

export type MotionOptionResult = {
  label: string;
  count: number;
};

export type MotionResults = {
  total: number;
  options: MotionOptionResult[];
};

export type MotionExportResponse = {
  motion_id: number;
  motion_name: string;
  totals: MotionResults;
};

export type EndMotionResponse = {
  id: number;
  status: string;
};

function parseCreateMotionResponse(payload: unknown): CreateMotionResponse {
  if (
    !isRecord(payload) ||
    typeof payload.id !== "number" ||
    typeof payload.name !== "string" ||
    typeof payload.status !== "string" ||
    typeof payload.start_time !== "string"
  ) {
    throw new TypeError("Invalid create motion response");
  }

  return {
    id: payload.id,
    name: payload.name,
    status: payload.status,
    start_time: payload.start_time,
  };
}

function parseCheckMotionResponse(payload: unknown): CheckMotionResponse {
  if (!isRecord(payload)) {
    throw new TypeError("Invalid check motion response");
  }

  const activeMotion = payload.active_motion;
  if (activeMotion === null || activeMotion === undefined) {
    return { active_motion: null };
  }

  if (
    !isRecord(activeMotion) ||
    typeof activeMotion.id !== "number" ||
    typeof activeMotion.name !== "string" ||
    !isRecord(activeMotion.data)
  ) {
    throw new TypeError("Invalid active motion payload");
  }

  return {
    active_motion: {
      id: activeMotion.id,
      name: activeMotion.name,
      data: activeMotion.data,
    },
  };
}

export function parseMotionResults(payload: unknown): MotionResults {
  if (!isRecord(payload) || typeof payload.total !== "number" || !Array.isArray(payload.options)) {
    throw new TypeError("Invalid motion results response");
  }

  const options: MotionOptionResult[] = [];
  for (const option of payload.options) {
    if (!isRecord(option) || typeof option.label !== "string" || typeof option.count !== "number") {
      throw new TypeError("Invalid motion results option");
    }
    options.push({ label: option.label, count: option.count });
  }

  return {
    total: payload.total,
    options,
  };
}

function parseMotionExportResponse(payload: unknown): MotionExportResponse {
  if (
    !isRecord(payload) ||
    typeof payload.motion_id !== "number" ||
    typeof payload.motion_name !== "string" ||
    !isRecord(payload.totals)
  ) {
    throw new TypeError("Invalid motion export response");
  }

  return {
    motion_id: payload.motion_id,
    motion_name: payload.motion_name,
    totals: parseMotionResults(payload.totals),
  };
}

function parseEndMotionResponse(payload: unknown): EndMotionResponse {
  if (!isRecord(payload) || typeof payload.id !== "number" || typeof payload.status !== "string") {
    throw new TypeError("Invalid end motion response");
  }

  return {
    id: payload.id,
    status: payload.status,
  };
}

export async function createMotion(request: CreateMotionRequest): Promise<CreateMotionResponse> {
  const response = await fetch(apiUrl("/motions/create"), {
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
  return parseCreateMotionResponse(payload);
}

export async function checkMotion(): Promise<CheckMotionResponse> {
  const response = await fetch(apiUrl("/motions/check"), {
    credentials: "include",
    cache: "no-store",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseCheckMotionResponse(payload);
}

export async function exportMotionResults(): Promise<MotionExportResponse> {
  const response = await fetch(apiUrl("/motions/export"), {
    credentials: "include",
    cache: "no-store",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseMotionExportResponse(payload);
}

export async function endMotion(): Promise<EndMotionResponse> {
  const response = await fetch(apiUrl("/motions/end"), {
    method: "POST",
    credentials: "include",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseEndMotionResponse(payload);
}
