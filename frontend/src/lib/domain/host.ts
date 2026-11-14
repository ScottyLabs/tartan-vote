import type { AttendeeRecord } from "$lib/api/attendance";

export type HostDialog = "motion" | "election" | "proxy" | null;
export type ProxyStatus = "pending" | "accepted" | "declined" | "none";
export type HostVoteKind = "motion" | "election" | "quick-vote";

export interface HostParticipant {
  id: number;
  name: string;
  initials: string;
  proxyStatus: ProxyStatus;
  proxyVotes: string[];
}

export function initialsFromName(name: string): string {
  const parts = name.trim().split(/\s+/u).filter(Boolean);
  if (parts.length === 0) return "?";
  if (parts.length === 1) return parts[0].slice(0, 2).toUpperCase();
  return `${parts[0][0] ?? ""}${parts[1][0] ?? ""}`.toUpperCase();
}

export function attendeeToHostParticipant(attendee: AttendeeRecord): HostParticipant {
  return {
    id: attendee.id,
    name: attendee.name.trim() || attendee.andrew_id,
    initials: initialsFromName(attendee.name.trim() || attendee.andrew_id),
    proxyStatus: attendee.is_proxy_holder ? "accepted" : "none",
    proxyVotes: [...attendee.proxy_for],
  };
}

export function parseHostVoteKind(value: unknown): HostVoteKind {
  if (value === "election" || value === "quick-vote" || value === "motion") {
    return value;
  }
  return "motion";
}

export function voteOptionsFromMotionData(data: Record<string, unknown>): string[] {
  const rawOptions = data.vote_options;
  if (!Array.isArray(rawOptions)) return [];
  return rawOptions
    .filter((option): option is string => typeof option === "string")
    .map((option) => option.trim())
    .filter(Boolean);
}

export type HostedVoteInput = {
  name: string;
  kind: HostVoteKind;
  options: string[];
  votingType: string;
  displayLiveResults: boolean;
  allowProxyVoting: boolean;
  quorum: string;
  description?: string;
};

export function buildMotionData(input: HostedVoteInput): Record<string, unknown> {
  return {
    description: input.description ?? "",
    vote_options: input.options,
    proxy: input.allowProxyVoting,
    visibility: input.displayLiveResults ? "live" : "hidden_until_release",
    kind: input.kind,
    voting_type: input.votingType,
    quorum: input.quorum || null,
  };
}
