export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

export async function readJsonUnknown(response: Response): Promise<unknown> {
  const text = await response.text();
  if (text.trim() === "") {
    return null;
  }
  return JSON.parse(text) as unknown;
}

export async function parseError(response: Response): Promise<string> {
  try {
    const payload = await readJsonUnknown(response);
    if (isRecord(payload) && typeof payload.error === "string" && payload.error.trim() !== "") {
      return payload.error;
    }
  } catch {
    // Non-JSON error responses use the HTTP status fallback below.
  }

  return `Request failed (${response.status})`;
}

export class ApiRequestError extends Error {
  constructor(
    readonly status: number,
    message: string,
  ) {
    super(message);
    this.name = "ApiRequestError";
  }
}
