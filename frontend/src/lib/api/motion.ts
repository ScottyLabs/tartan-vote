import { apiUrl } from "$lib/api/base";

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

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

async function readJsonUnknown(response: Response): Promise<unknown> {
  const text = await response.text();
  if (text.trim() === "") {
    return null;
  }
  return JSON.parse(text) as unknown;
}

async function parseError(response: Response): Promise<string> {
  try {
    const payload = await readJsonUnknown(response);
    if (isRecord(payload) && typeof payload.error === "string" && payload.error.trim() !== "") {
      return payload.error;
    }
  } catch {
    // no-op
  }

  return `Request failed (${response.status})`;
}

function parseCreateMotionResponse(payload: unknown): CreateMotionResponse {
  if (
    !isRecord(payload) ||
    typeof payload.id !== "number" ||
    typeof payload.name !== "string" ||
    typeof payload.status !== "string" ||
    typeof payload.start_time !== "string"
  ) {
    throw new Error("Invalid create motion response");
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
    throw new Error("Invalid check motion response");
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
    throw new Error("Invalid active motion payload");
  }

  return {
    active_motion: {
      id: activeMotion.id,
      name: activeMotion.name,
      data: activeMotion.data,
    },
  };
}

function parseMotionExportResponse(payload: unknown): MotionExportResponse {
  if (
    !isRecord(payload) ||
    typeof payload.motion_id !== "number" ||
    typeof payload.motion_name !== "string" ||
    !isRecord(payload.totals) ||
    typeof payload.totals.total !== "number" ||
    !Array.isArray(payload.totals.options)
  ) {
    throw new Error("Invalid motion export response");
  }

  const options: MotionOptionResult[] = [];
  for (const option of payload.totals.options) {
    if (!isRecord(option) || typeof option.label !== "string" || typeof option.count !== "number") {
      throw new Error("Invalid motion export totals");
    }
    options.push({ label: option.label, count: option.count });
  }

  return {
    motion_id: payload.motion_id,
    motion_name: payload.motion_name,
    totals: {
      total: payload.totals.total,
      options,
    },
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
    throw new Error(await parseError(response));
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
    throw new Error(await parseError(response));
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
    throw new Error(await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseMotionExportResponse(payload);
}
