import { writable } from 'svelte/store';

export interface Mod {
	title: string;
	description: string;
	image: string;
	downloads: string;
	lastUpdated: string;
	categories: Category[] | Category;
	colors: { color1: string; color2: string };
	installed: boolean;
	requires_steamodded: boolean;
	publisher: string;
}

export enum Category {
	Popular,
	Recent,
	Featured,
	Content,
	Joker,
	QualityOfLife,
	Technical,
	Miscellaneous,
	ResourcePacks,
	API
}


export const currentModView = writable<Mod | null>(null);
export const searchResults = writable<Mod[]>([]);

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



export async function searchMods(query: string): Promise<Mod[]> {
	// Implement your search logic here
	return [];
}


