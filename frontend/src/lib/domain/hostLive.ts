import { calculatePercentage, resultColorForIndex, type ResultColor } from "$lib/domain/results";

export type HostLiveVoteType = "election" | "motion" | "quick-vote";

export interface HostLiveOption {
  id: string;
  label: string;
  votes: number;
  color: ResultColor;
  percentage: number;
}

export interface HostLiveViewModel {
  title: string;
  type: HostLiveVoteType;
  votingStyle: "Secret Ballot" | "Standard";
  eligibleVotes: number;
  votesSubmitted: number;
  options: HostLiveOption[];
}

function safeInteger(value: string | null, fallback: number, minimum = 0, maximum = 1000) {
  const parsed = Math.trunc(Number(value ?? ""));
  return Number.isFinite(parsed) ? Math.min(maximum, Math.max(minimum, parsed)) : fallback;
}

function demoOptions(
  type: HostLiveVoteType,
  optionCount: number,
  configuredLabels: string[],
  configuredVotes: number[],
) {
  const electionNames = ["Person 1", "Person 2", "Person 3"];
  const motionNames = ["Pass", "Reject", "Abstain"];
  const names =
    configuredLabels.length >= 2
      ? configuredLabels
      : type === "election"
        ? electionNames
        : motionNames;

  return Array.from({ length: optionCount }, (_, index) => ({
    id: `option-${index + 1}`,
    label: names[index] ?? `Option ${index + 1}`,
    votes: configuredVotes[index] ?? 0,
  }));
}

function defaultTitleForType(type: HostLiveVoteType): string {
  if (type === "motion") return "Fund the spring student showcase";
  if (type === "quick-vote") return "QuickVote";
  return "Best Rust StuCo Instructor";
}

export function createHostLiveView(searchParams: URLSearchParams): HostLiveViewModel {
  const requestedType = searchParams.get("type");
  const type: HostLiveVoteType =
    requestedType === "motion" || requestedType === "quick-vote" ? requestedType : "election";
  const configuredLabels = (searchParams.get("labels") ?? "")
    .split("|")
    .map((label) => label.trim())
    .filter((label) => label !== "")
    .slice(0, 30);
  const configuredVotes = (searchParams.get("votes") ?? "")
    .split(",")
    .map((votes) => safeInteger(votes, 0, 0, 10000));
  const optionCount =
    configuredLabels.length >= 2
      ? configuredLabels.length
      : safeInteger(searchParams.get("options"), 3, 2, 30);
  const rawOptions = demoOptions(type, optionCount, configuredLabels, configuredVotes);
  const votesSubmitted = rawOptions.reduce((total, option) => total + option.votes, 0);
  const eligibleVotes = Math.max(
    votesSubmitted,
    safeInteger(searchParams.get("eligible"), 0, 0, 10000),
  );
  const requestedTitle = searchParams.get("title")?.trim() ?? "";

  return {
    title: requestedTitle === "" ? defaultTitleForType(type) : requestedTitle,
    type,
    votingStyle: type === "election" ? "Secret Ballot" : "Standard",
    eligibleVotes,
    votesSubmitted,
    options: rawOptions.map((option, index) => ({
      id: option.id,
      label: option.label,
      votes: option.votes,
      color: resultColorForIndex(index),
      percentage: calculatePercentage(option.votes, votesSubmitted),
    })),
  };
}
