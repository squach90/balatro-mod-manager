import { writable, type Writable } from 'svelte/store';

export interface Mod {
    title: string;
    description: string;
    image: string;
    imageFallback?: string;
    categories: Category[];
    colors: {
        color1: string;
        color2: string;
    };
	requires_steamodded: boolean;
	requires_talisman: boolean;
	publisher: string;
	repo: string;
	downloadURL: string;
	folderName: string;
	version: string;
	installed: boolean;
	last_updated: number;
}

export interface LocalMod {
	name: string;
	id: string;
	author: string[];
	description: string;
	prefix: string;
	version?: string;
	path: string;
	dependencies: string[];
	conflicts: string[];
	is_tracked: boolean;
}

export enum SortOption {
	NameAsc = "name_asc",
	NameDesc = "name_desc",
	LastUpdatedAsc = "updated_asc",
	LastUpdatedDesc = "updated_desc"
}

export const backgroundEnabled = writable(false);

export const currentSort = writable<SortOption>(SortOption.LastUpdatedDesc);

export const updateAvailableStore = writable<{ [key: string]: boolean }>({});

export const modEnabledStore = writable<Record<string, boolean>>({});



export interface UninstallDialogState {
	show: boolean;
	modName: string;
	modPath: string;
	dependents: string[];
}


export const uninstallDialogStore = writable<UninstallDialogState>({
	show: false,
	modName: "",
	modPath: "",
	dependents: []
});

export const selectedModStore = writable<{ name: string; path: string } | null>(null);
export const dependentsStore = writable<string[]>([]);
export const currentPage = writable(1);
export const itemsPerPage = writable(12);

export type UninstallResult = {
	success: boolean;
	action: "cascade" | "force" | "single";
};

export const cachedVersions = writable<{
	steamodded: string[];
	talisman: string[];
}>({
	steamodded: typeof window !== 'undefined'
		? JSON.parse(localStorage.getItem('version-cache-steamodded') || '[]')
		: [],
	talisman: typeof window !== 'undefined'
		? JSON.parse(localStorage.getItem('version-cache-talisman') || '[]')
		: []
});

if (typeof window !== 'undefined') {
	cachedVersions.subscribe(value => {
		localStorage.setItem('version-cache-steamodded', JSON.stringify(value.steamodded));
		localStorage.setItem('version-cache-talisman', JSON.stringify(value.talisman));
	});
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
	Content = 0,
	Joker = 1,
	QualityOfLife = 2,
	Technical = 3,
	Miscellaneous = 4,
	ResourcePacks = 5,
	API = 6
}


export const currentModView = writable<Mod | null>(null);
export const currentJokerView = writable<Mod | null>(null);
export const searchResults = writable<Mod[]>([]);
export const modsStore = writable<Mod[]>([]);

// Background catalog loading state and last refresh time
export const catalogLoading = writable(false);
export const catalogLastRefreshed = writable<number | null>(null);

// Persist and hydrate the mods catalog for instant UI + offline fallback
if (typeof window !== 'undefined') {
    try {
        const cached = localStorage.getItem('mods-cache');
        if (cached) {
            const parsed: Mod[] = JSON.parse(cached);
            if (Array.isArray(parsed)) {
                modsStore.set(parsed);
            }
        }
        const ts = localStorage.getItem('mods-cache-ts');
        if (ts) {
            const n = Number(ts);
            if (!Number.isNaN(n)) catalogLastRefreshed.set(n);
        }
    } catch (_) {
        // ignore cache read errors
    }

    modsStore.subscribe((value) => {
        try {
            localStorage.setItem('mods-cache', JSON.stringify(value));
            const now = Date.now();
            localStorage.setItem('mods-cache-ts', String(now));
            catalogLastRefreshed.set(now);
        } catch (_) {
            // ignore cache write errors
        }
    });
}


export const installationStatus: Writable<InstallationStatus> = writable({});

export const loadingStates2 = writable<{ [key: string]: boolean }>({});
//
//
// modsStore.subscribe(value => {
// 	if (typeof window !== 'undefined') {
// 		localStorage.setItem('mods', JSON.stringify(value));
// 	}
// });


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


export interface WarningPopupState {
	visible: boolean;
	message: string;
	onConfirm: () => void;
	onCancel: () => void;
}

export const showWarningPopup = writable<WarningPopupState>({
    visible: false,
    message: "",
    onConfirm: () => { },
    onCancel: () => { }
});

// Popup state to warn users when Lovely is not detected
export interface LovelyPopupState {
    visible: boolean;
    source?: 'launch' | 'other';
    onLaunchAnyway?: () => void | Promise<void>;
}

export const lovelyPopupStore = writable<LovelyPopupState>({
    visible: false,
});
