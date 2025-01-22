import { writable, type Writable } from 'svelte/store';

export interface Mod {
	title: string;
	description: string;
	image: string;
	lastUpdated: string;
	categories: Category[] | Category;
	colors: { color1: string; color2: string };
	installed: boolean;
	requires_steamodded: boolean;
	requires_talisman: boolean;
	publisher: string;
	downloadURL: string;
}


export interface DependencyCheck {
	steamodded: boolean;
	talisman: boolean;
}

export interface InstalledMod {
	name: string;
	path: string;
	// collection_hash: string | null;
}

interface InstallationStatus {
	[key: string]: boolean;
}


export enum Category {
	Content,
	Joker,
	QualityOfLife,
	Technical,
	Miscellaneous,
	ResourcePacks,
	API
}


export const currentModView = writable<Mod | null>(null);
export const currentJokerView = writable<Mod | null>(null);
export const searchResults = writable<Mod[]>([]);
export const modsStore = writable<Mod[]>([]);
export const installationStatus: Writable<InstallationStatus> = writable({});

export const loadingStates2 = writable<{ [key: string]: boolean }>({});




function createPersistentCategory() {
	const storedCategory = localStorage.getItem('currentCategory') || "Popular";
	const { subscribe, set } = writable(storedCategory);

	return {
		subscribe,
		set: (value: string) => {
			localStorage.setItem('currentCategory', value);
			set(value);
		}
	};
}

export const currentCategory = createPersistentCategory();
