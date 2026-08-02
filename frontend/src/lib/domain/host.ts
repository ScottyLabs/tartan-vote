export type HostDialog = "motion" | "election" | "proxy" | null;
export type ProxyStatus = "pending" | "accepted" | "declined" | "none";

export interface HostParticipant {
  id: number;
  name: string;
  initials: string;
  proxyStatus: ProxyStatus;
  proxyVotes: string[];
}

export const exampleParticipants: HostParticipant[] = [
  {
    id: 1,
    name: "Scotty Labs",
    initials: "SL",
    proxyStatus: "pending",
    proxyVotes: ["scottylabs1", "scottylabs2"],
  },
  {
    id: 2,
    name: "Scotty Labs",
    initials: "SL",
    proxyStatus: "accepted",
    proxyVotes: ["scottylabs3", "scottylabs4"],
  },
  {
    id: 3,
    name: "Scotty Labs",
    initials: "SL",
    proxyStatus: "pending",
    proxyVotes: ["scottylabs5", "scottylabs6", "scottylabs7", "scottylabs8"],
  },
  ...Array.from({ length: 17 }, (_, index) => ({
    id: index + 4,
    name: "Scotty Labs",
    initials: "SL",
    proxyStatus: "none" as const,
    proxyVotes: [],
  })),
];
