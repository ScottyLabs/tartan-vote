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

export type HostLiveApiResults = {
  total: number;
  options: Array<{ label: string; count: number }>;
};

export type HostLiveFromApiInput = {
  title: string;
  type?: HostLiveVoteType;
  votingStyle?: HostLiveViewModel["votingStyle"];
  results: HostLiveApiResults;
  eligibleVotes: number;
  optionLabels?: string[];
};

function mapHostLiveOptions(
  results: HostLiveApiResults,
  optionLabels: string[] | undefined,
): HostLiveOption[] {
  const labels =
    results.options.length > 0
      ? results.options.map((option) => option.label)
      : (optionLabels ?? []);
  const countByLabel = new Map(results.options.map((option) => [option.label, option.count]));
  const votesSubmitted = results.total;

  return labels.map((label, index) => {
    const votes = countByLabel.get(label) ?? 0;
    return {
      id: `option-${index + 1}`,
      label,
      votes,
      color: resultColorForIndex(index),
      percentage: calculatePercentage(votes, votesSubmitted),
    };
  });
}

export function hostLiveViewFromApi(input: HostLiveFromApiInput): HostLiveViewModel {
  const type = input.type ?? "motion";
  const options = mapHostLiveOptions(input.results, input.optionLabels);
  const votesSubmitted =
    input.results.total > 0
      ? input.results.total
      : options.reduce((total, option) => total + option.votes, 0);

  return {
    title: input.title.trim() === "" ? defaultTitleForType(type) : input.title.trim(),
    type,
    votingStyle: input.votingStyle ?? (type === "election" ? "Secret Ballot" : "Standard"),
    eligibleVotes: Math.max(votesSubmitted, input.eligibleVotes),
    votesSubmitted,
    options,
  };
}

export function updateHostLiveViewFromResults(
  current: HostLiveViewModel,
  results: HostLiveApiResults,
  eligibleVotes?: number,
): HostLiveViewModel {
  return hostLiveViewFromApi({
    title: current.title,
    type: current.type,
    votingStyle: current.votingStyle,
    results,
    eligibleVotes: eligibleVotes ?? current.eligibleVotes,
    optionLabels: current.options.map((option) => option.label),
  });
}
