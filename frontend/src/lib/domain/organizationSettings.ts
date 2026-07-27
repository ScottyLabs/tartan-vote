export interface OrganizationSettings {
	name: string;
	quorum: string;
	quickVoteOptions: string[];
	approvedMembers: string[];
}

const storageKey = 'tartan-vote.organization-settings';
const deletedOrganizationsKey = 'tartan-vote.deleted-organizations';

export const defaultOrganizationSettings: OrganizationSettings = {
	name: 'Default',
	quorum: '',
	quickVoteOptions: ['Yes', 'No'],
	approvedMembers: []
};

export const exampleOrganizations: OrganizationSettings[] = [
	defaultOrganizationSettings,
	{
		name: 'ScottyLabs Exec Board',
		quorum: '20',
		quickVoteOptions: ['Yes', 'No'],
		approvedMembers: []
	},
	{
		name: 'StudentSenate',
		quorum: '30',
		quickVoteOptions: ['Approve', 'Reject', 'Abstain'],
		approvedMembers: []
	}
];

export function loadOrganizationSettings(): OrganizationSettings {
	if (typeof window === 'undefined') return defaultOrganizationSettings;

	const storedSettings = window.localStorage.getItem(storageKey);
	if (!storedSettings) return defaultOrganizationSettings;

	try {
		const parsed = JSON.parse(storedSettings) as OrganizationSettings;
		if (!parsed.name || !Array.isArray(parsed.quickVoteOptions)) {
			return defaultOrganizationSettings;
		}
		return {
			...parsed,
			approvedMembers: Array.isArray(parsed.approvedMembers) ? parsed.approvedMembers : []
		};
	} catch {
		return defaultOrganizationSettings;
	}
}

export function saveOrganizationSettings(settings: OrganizationSettings) {
	if (typeof window === 'undefined') return;
	window.localStorage.setItem(storageKey, JSON.stringify(settings));
}

export function clearOrganizationSettings() {
	if (typeof window === 'undefined') return;
	window.localStorage.removeItem(storageKey);
}

export function loadDeletedOrganizationNames(): string[] {
	if (typeof window === 'undefined') return [];
	try {
		const names = JSON.parse(window.localStorage.getItem(deletedOrganizationsKey) ?? '[]');
		return Array.isArray(names) ? names.filter((name): name is string => typeof name === 'string') : [];
	} catch {
		return [];
	}
}

export function saveDeletedOrganizationNames(names: string[]) {
	if (typeof window === 'undefined') return;
	window.localStorage.setItem(deletedOrganizationsKey, JSON.stringify(names));
}
