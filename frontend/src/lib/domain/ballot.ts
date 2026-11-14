import type { VoteInstance } from "$lib/api/votes";
import { voteOptionsFromMotionData } from "$lib/domain/host";

export interface BallotChoice {
  id: string;
  label: string;
}

export interface VotingInstance {
  id: string;
  votingId: string;
  proxy: boolean;
  hasVoted?: boolean;
}

export type ParticipantVisibility = "live" | "hidden_until_release";

/** Demo fixtures retained for local/story use; live ballot path maps from API. */
export const ballotChoices: BallotChoice[] = [
  { id: "option-1", label: "Option1" },
  { id: "option-2", label: "Option2" },
  { id: "option-3", label: "Option3" },
];

/** Demo fixtures retained for local/story use; live ballot path maps from API. */
export const votingInstances: VotingInstance[] = [
  { id: "self", votingId: "ScottyLabs", proxy: false },
  { id: "proxy", votingId: "ScottyLabs1", proxy: true },
];

function displayNameFromProxy(proxyForName: string | null, fallback: string): string {
  const trimmed = proxyForName?.trim() ?? "";
  return trimmed === "" ? fallback : trimmed;
}

export function ballotChoicesFromVoteOptions(voteOptions: string[]): BallotChoice[] {
  return voteOptions.map((label) => ({
    id: label,
    label,
  }));
}

export function ballotChoicesFromMotionData(data: Record<string, unknown>): BallotChoice[] {
  return ballotChoicesFromVoteOptions(voteOptionsFromMotionData(data));
}

export function votingInstancesFromApi(instances: VoteInstance[]): VotingInstance[] {
  return instances.map((instance) => ({
    id: String(instance.voter_instance_id),
    votingId: instance.is_proxy
      ? displayNameFromProxy(instance.proxy_for_name, "Proxy")
      : displayNameFromProxy(instance.proxy_for_name, "You"),
    proxy: instance.is_proxy,
    hasVoted: instance.has_voted,
  }));
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

export function participantVisibilityFromMotionData(
  data: Record<string, unknown>,
): ParticipantVisibility | null {
  const visibility = data.visibility;
  if (visibility === "live" || visibility === "hidden_until_release") {
    return visibility;
  }
  if (isRecord(visibility)) {
    const participants = visibility.participants;
    if (participants === "live" || participants === "hidden_until_release") {
      return participants;
    }
  }
  return null;
}

export function votingTypeFromMotionData(data: Record<string, unknown>): string {
  if (typeof data.voting_type === "string" && data.voting_type.trim() !== "") {
    return data.voting_type.trim();
  }
  return "Standard";
}

export function motionKindFromMotionData(data: Record<string, unknown>): string {
  if (typeof data.kind === "string" && data.kind.trim() !== "") {
    return data.kind.trim();
  }
  return "motion";
}

export function titleForMotion(name: string, kind: string): string {
  if (kind === "quick-vote") return name || "QuickVote";
  if (kind === "election") return `Election: ${name}`;
  if (kind === "motion") return `Motion: ${name}`;
  return name;
}
