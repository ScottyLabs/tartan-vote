import { API_BASE } from './config';

type Json = Record<string, unknown> | unknown[] | null;

async function request<T>(
  path: string,
  init: RequestInit = {}
): Promise<T> {
  const url = `${API_BASE}${path}`;
  const res = await fetch(url, {
    credentials: 'include',
    headers: {
      Accept: 'application/json',
      ...(init.body ? { 'Content-Type': 'application/json' } : {}),
      ...(init.headers ?? {})
    },
    ...init
  });
  if (!res.ok) {
    let message = `HTTP ${res.status}`;
    try {
      const body = await res.json();
      if (body && typeof body === 'object' && 'message' in body)
        message = String((body as { message: unknown }).message);
    } catch {
      /* ignore */
    }
    throw Object.assign(new Error(message), { status: res.status });
  }
  if (res.status === 204) return undefined as T;
  const text = await res.text();
  if (!text || !text.trim()) return undefined as T;
  return JSON.parse(text) as T;
}

export const api = {
  // Session
  joinSession: (code: string) => request<Json>(`/session/join/${code}`),
  sessionStatus: (code: string) =>
    request<{ session_ended: boolean }>(`/session/${code}/status`),
  createSession: () => request<{ session_code: string }>(`/session/create`),
  endSession: (code: string) => request<Json>(`/session/${code}/end`),
  attendance: (code: string) =>
    request<{ session_code: string; headcount: number; attendees: unknown[] }>(
      `/session/${code}/attendance`
    ),
  proxy: (
    code: string,
    body: { is_senator: boolean; proxy_for: string | null }
  ) =>
    request<{
      vote_instance_count: number;
      is_senator: boolean;
      has_proxy: boolean;
    }>(`/session/${code}/proxy`, {
      method: 'POST',
      body: JSON.stringify(body)
    }),

  // Exports — returns a Blob URL for the caller to download
  async exportBlob(
    code: string,
    kind: 'attendance' | 'votes',
    format: 'pdf' | 'csv'
  ): Promise<Blob> {
    const res = await fetch(
      `${API_BASE}/session/${code}/export/${kind}/${format}`,
      { credentials: 'include' }
    );
    if (!res.ok) throw new Error(`Export failed (${res.status})`);
    return await res.blob();
  },

  // Events
  checkEvent: (code: string) =>
    request<{ active_event: null | {
      id: number;
      name: string;
      event_type: string;
      status?: string;
      start_time?: string;
      end_time?: string | null;
      data: EventData;
    } }>(`/events/${code}/check`),
  voteInstances: (eventId: number | string) =>
    request<
      Array<{
        voter_instance_id: number;
        is_proxy: boolean;
        proxy_for_user_id: number | null;
        has_voted: boolean;
      }>
    >(`/events/${eventId}/vote-instances`),
  createEvent: (
    code: string,
    body: {
      name: string;
      event_type: 'Motion' | 'Election';
      start_time: string;
      end_time: string;
      data: Partial<EventData> & Record<string, unknown>;
    }
  ) =>
    request<{
      id: number;
      name: string;
      event_type: string;
      status: string;
      start_time: string;
      data: EventData;
    }>(`/events/create/${code}`, {
      method: 'POST',
      body: JSON.stringify(body)
    }),
  endEvent: (eventId: number | string) =>
    request<Json>(`/events/${eventId}/end`),
  releaseResults: (eventId: number | string) =>
    request<Json>(`/events/${eventId}/release`, { method: 'POST' }),
  eventResults: (eventId: number | string) =>
    request<Record<string, unknown>>(`/events/${eventId}/results`),
  vote: (
    eventId: number | string,
    body: { vote_response: string[]; voter_instance_id: number }
  ) =>
    request<Json>(`/events/${eventId}/vote`, {
      method: 'POST',
      body: JSON.stringify(body)
    })
};

export function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}
