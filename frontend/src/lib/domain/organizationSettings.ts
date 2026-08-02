export interface OrganizationSettings {
  name: string;
  quorum: string;
  quickVoteOptions: string[];
  approvedMembers: string[];
}

const storageKey = "tartan-vote.organization-settings";
const deletedOrganizationsKey = "tartan-vote.deleted-organizations";

export const defaultOrganizationSettings: OrganizationSettings = {
  name: "Default",
  quorum: "",
  quickVoteOptions: ["Yes", "No"],
  approvedMembers: [],
};

export const exampleOrganizations: OrganizationSettings[] = [
  defaultOrganizationSettings,
  {
    name: "ScottyLabs Exec Board",
    quorum: "20",
    quickVoteOptions: ["Yes", "No"],
    approvedMembers: [],
  },
  {
    name: "StudentSenate",
    quorum: "30",
    quickVoteOptions: ["Approve", "Reject", "Abstain"],
    approvedMembers: [],
  },
];

function isStringArray(value: unknown): value is string[] {
  return Array.isArray(value) && value.every((item) => typeof item === "string");
}

function parseOrganizationSettings(value: unknown): OrganizationSettings | null {
  if (typeof value !== "object" || value === null) return null;
  if (!("name" in value) || typeof value.name !== "string" || value.name === "") {
    return null;
  }
  if (!("quorum" in value) || typeof value.quorum !== "string") return null;
  if (!("quickVoteOptions" in value) || !isStringArray(value.quickVoteOptions)) {
    return null;
  }
  return {
    name: value.name,
    quorum: value.quorum,
    quickVoteOptions: value.quickVoteOptions,
    approvedMembers:
      "approvedMembers" in value && isStringArray(value.approvedMembers)
        ? value.approvedMembers
        : [],
  };
}

export function loadOrganizationSettings(): OrganizationSettings {
  if (typeof window === "undefined") return defaultOrganizationSettings;

  const storedSettings = window.localStorage.getItem(storageKey);
  if (storedSettings === null) return defaultOrganizationSettings;

  try {
    return parseOrganizationSettings(JSON.parse(storedSettings)) ?? defaultOrganizationSettings;
  } catch {
    return defaultOrganizationSettings;
  }
}

export function saveOrganizationSettings(settings: OrganizationSettings) {
  if (typeof window === "undefined") return;
  window.localStorage.setItem(storageKey, JSON.stringify(settings));
}

export function clearOrganizationSettings() {
  if (typeof window === "undefined") return;
  window.localStorage.removeItem(storageKey);
}

export function loadDeletedOrganizationNames(): string[] {
  if (typeof window === "undefined") return [];
  try {
    const names: unknown = JSON.parse(window.localStorage.getItem(deletedOrganizationsKey) ?? "[]");
    return Array.isArray(names)
      ? names.filter((name): name is string => typeof name === "string")
      : [];
  } catch {
    return [];
  }
}

export function saveDeletedOrganizationNames(names: string[]) {
  if (typeof window === "undefined") return;
  window.localStorage.setItem(deletedOrganizationsKey, JSON.stringify(names));
}
