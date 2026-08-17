import { apiUrl } from "$lib/api/base";
import { ApiRequestError, isRecord, parseError, readJsonUnknown } from "$lib/api/response";
import { parseMotionResults, type MotionResults } from "$lib/api/motion";

export type VoteInstance = {
  voter_instance_id: number;
  is_proxy: boolean;
  proxy_for_user_id: number | null;
  proxy_for_name: string | null;
  has_voted: boolean;
};

export type CastVoteRequest = {
  vote_response: string[];
  voter_instance_id?: number;
};

export type CastVoteResponse = {
  voter_instance_id: number;
  vote_id: number;
};

function parseVoteInstance(payload: unknown): VoteInstance {
  if (
    !isRecord(payload) ||
    typeof payload.voter_instance_id !== "number" ||
    typeof payload.is_proxy !== "boolean" ||
    typeof payload.has_voted !== "boolean" ||
    !(payload.proxy_for_user_id === null || typeof payload.proxy_for_user_id === "number") ||
    !(payload.proxy_for_name === null || typeof payload.proxy_for_name === "string")
  ) {
    throw new TypeError("Invalid vote instance payload");
  }

  return {
    voter_instance_id: payload.voter_instance_id,
    is_proxy: payload.is_proxy,
    proxy_for_user_id: payload.proxy_for_user_id,
    proxy_for_name: payload.proxy_for_name,
    has_voted: payload.has_voted,
  };
}

function parseVoteInstances(payload: unknown): VoteInstance[] {
  if (!Array.isArray(payload)) {
    throw new TypeError("Invalid vote instances response");
  }
  return payload.map((item) => parseVoteInstance(item));
}

function parseCastVoteResponse(payload: unknown): CastVoteResponse {
  if (
    !isRecord(payload) ||
    typeof payload.voter_instance_id !== "number" ||
    typeof payload.vote_id !== "number"
  ) {
    throw new TypeError("Invalid cast vote response");
  }

  return {
    voter_instance_id: payload.voter_instance_id,
    vote_id: payload.vote_id,
  };
}

export async function getVoteInstances(): Promise<VoteInstance[]> {
  const response = await fetch(apiUrl("/motions/vote-instances"), {
    credentials: "include",
    cache: "no-store",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseVoteInstances(payload);
}

export async function castVote(request: CastVoteRequest): Promise<CastVoteResponse> {
  const response = await fetch(apiUrl("/motions/vote"), {
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
  return parseCastVoteResponse(payload);
}

export async function getMotionResults(): Promise<MotionResults> {
  const response = await fetch(apiUrl("/motions/results"), {
    credentials: "include",
    cache: "no-store",
  });

  if (!response.ok) {
    throw new ApiRequestError(response.status, await parseError(response));
  }

  const payload = await readJsonUnknown(response);
  return parseMotionResults(payload);
}
