export interface BallotChoice {
  id: string;
  label: string;
}

export interface VotingInstance {
  id: string;
  votingId: string;
  proxy: boolean;
}

export const ballotChoices: BallotChoice[] = [
  { id: "option-1", label: "Option1" },
  { id: "option-2", label: "Option2" },
  { id: "option-3", label: "Option3" },
];

export const votingInstances: VotingInstance[] = [
  { id: "self", votingId: "ScottyLabs", proxy: false },
  { id: "proxy", votingId: "ScottyLabs1", proxy: true },
];
