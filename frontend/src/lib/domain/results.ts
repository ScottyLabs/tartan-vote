export type ResultsVariant =
  | "live"
  | "rollcall-live"
  | "secret-final"
  | "final-short"
  | "final"
  | "final-expanded";
export type VoteType = "secret" | "standard" | "rollcall";
export type ResultsPhase = "live" | "final";

export const RESULT_COLOR_CYCLE = ["green", "yellow", "blue", "red", "purple"] as const;
export type ResultColor = (typeof RESULT_COLOR_CYCLE)[number];

export interface ResultOption {
  id: string;
  label: string;
  votes: number;
  color: ResultColor;
  voters?: string[];
}

export interface ResultsViewModel {
  heading: string;
  countLabel: string;
  count: string;
  options: ResultOption[];
  final: boolean;
  initiallyExpanded: boolean;
}

export interface ResultsRequest {
  voteType: VoteType;
  phase: ResultsPhase;
  expanded?: boolean;
  compact?: boolean;
}

function voterNames(count: number): string[] {
  return Array.from({ length: count }, () => "scottylabs1");
}

export function normalizeResultsVariant(value: string | null): ResultsVariant {
  switch (value) {
    case "live":
    case "rollcall-live":
    case "secret-final":
    case "final-short":
    case "final":
    case "final-expanded":
      return value;
    case null:
      return "live";
    default:
      return "live";
  }
}

export function resultColorForIndex(index: number): ResultColor {
  return RESULT_COLOR_CYCLE[index % RESULT_COLOR_CYCLE.length];
}

export function normalizeVoteType(value: string | null): VoteType {
  return value === "secret" || value === "rollcall" ? value : "standard";
}

export function normalizeResultsPhase(value: string | null): ResultsPhase {
  return value === "final" ? "final" : "live";
}

export function calculatePercentage(votes: number, votesCast: number): number {
  if (votesCast <= 0) return 0;
  return Math.min(100, Math.max(0, Math.round((votes / votesCast) * 100)));
}

export function requestForVariant(variant: ResultsVariant): ResultsRequest {
  switch (variant) {
    case "rollcall-live":
      return { voteType: "rollcall", phase: "live" };
    case "secret-final":
      return { voteType: "secret", phase: "final" };
    case "final-short":
      return { voteType: "standard", phase: "final", compact: true };
    case "final":
      return { voteType: "rollcall", phase: "final" };
    case "final-expanded":
      return { voteType: "rollcall", phase: "final", expanded: true };
    case "live":
      return { voteType: "standard", phase: "live" };
    default: {
      const exhaustive: never = variant;
      return exhaustive;
    }
  }
}

export function createDemoResults(request: ResultsRequest): ResultsViewModel {
  const { voteType, phase, expanded = false, compact = false } = request;
  const isLive = phase === "live";
  const isSecret = voteType === "secret";
  const showsVoters = voteType === "rollcall" || (!isLive && voteType === "standard");
  const eligibleVotes = voteType === "rollcall" ? 200 : 20;
  const optionCounts = isLive
    ? voteType === "rollcall"
      ? [50, 25, 25]
      : [6, 3, 3]
    : eligibleVotes === 200
      ? [100, 50, 50]
      : [10, 5, 5];
  const votesCast = optionCounts.reduce((total, votes) => total + votes, 0);
  const longOption = "Voting Options Have a 36 Chara Limit";
  const optionLabels = isSecret ? [longOption, "Reject", "Abstain"] : ["Pass", "Reject", "Abstain"];
  // These three colors match the supplied Figma examples. Backend-provided
  // options can use resultColorForIndex() to cycle through all five colors.
  const colors: ResultColor[] = ["green", "red", "yellow"];

  return {
    heading: isLive
      ? "Live Results(Unofficial)"
      : isSecret
        ? `Final Result: ${longOption}`
        : compact
          ? "Final Result: Pass"
          : "Result: Pass",
    countLabel: isLive ? "Votes Submitted" : "Total Votes",
    count: isLive ? `${votesCast}/${eligibleVotes}` : String(votesCast),
    final: !isLive,
    initiallyExpanded: expanded,
    options: optionLabels.map((label, index) => ({
      id: `option-${index + 1}`,
      label,
      votes: optionCounts[index],
      color: colors[index],
      voters: showsVoters ? voterNames(optionCounts[index]) : undefined,
    })),
  };
}
