<script lang="ts">
	import {
		Download,
		Flame,
		Clock,
		Star,
		Spade,
		Gamepad2,
		LayoutDashboard,
		FolderHeart,
		Trash2,
		Search,
		Play,
	} from "lucide-svelte";
	import ModView from "./ModView.svelte";
	import { currentModView, currentCategory } from "../../stores/modStore";
	import type { Mod } from "../../stores/modStore";
	import { Category } from "../../stores/modStore";
	import { modsStore } from "../../stores/modStore";
	import { open } from "@tauri-apps/plugin-shell";
	import { invoke } from "@tauri-apps/api/core";
	import { stripMarkdown, truncateText } from "../../utils/helpers";
	import SearchView from "./SearchView.svelte";
	import { onMount } from "svelte";
	import { writable } from "svelte/store";

	const loadingDots = writable(0);

	// Animate the dots
	let dotInterval: number;
	onMount(() => {
		dotInterval = setInterval(() => {
			loadingDots.update((n) => (n + 1) % 4);
		}, 500);

		return () => {
			clearInterval(dotInterval);
		};
	});

	let currentModLoader: "steamodded" | "lovely-only";
	let mods: Mod[] = [];
	let isLoading = true;

	onMount(() => {
		const initialize = async () => {
			try {
				currentModLoader = (await invoke("get_modloader")) as
					| "lovely-only"
					| "steamodded";

				if (
					$currentCategory === "Active Mods" &&
					currentModLoader !== "lovely-only"
				) {
					currentCategory.set(baseCategories[2].name);
				}

				isLoading = true;
				mods = await fetchModDirectories();
				isLoading = false;
			} catch (error) {
				console.error("Failed to get modloader:", error);
				isLoading = false;
			}
		};

		initialize();

		// Return synchronous cleanup function
		return () => {
			// Cleanup code here if needed
		};
	});

	interface ModMeta {
		title: string;
		"requires-steamodded": boolean;
		categories: string[];
		author: string;
		repo: string;
		downloadURL?: string;
	}

	const CACHE_DURATION = 5 * 60 * 1000; // 5 minutes
	// const CACHE_DURATION = 5 * 1000; // 5 seconds

	function isValidCache(timestamp: number): boolean {
		return Date.now() - timestamp < CACHE_DURATION;
	}

	// Store in localStorage
	function saveToCache(mods: Mod[]) {
		const cache = {
			timestamp: Date.now(),
			mods: mods,
		};
		localStorage.setItem("mods-cache", JSON.stringify(cache));
	}

	// Get from localStorage
	function getFromCache(): { mods: Mod[]; timestamp: number } | null {
		const cached = localStorage.getItem("mods-cache");
		if (!cached) return null;
		return JSON.parse(cached);
	}

	async function checkImageExists(imageUrl: string): Promise<string> {
		try {
			const response = await fetch(imageUrl, { method: "HEAD" });
			return response.ok ? imageUrl : "images/cover.jpg";
		} catch {
			return "images/cover.jpg";
		}
	}

	async function getLastUpdated(repoUrl: string): Promise<string> {
		try {
			const [owner, repo] = repoUrl
				.replace("https://github.com/", "")
				.split("/");

			if (!owner || !repo) {
				return "Unknown";
			}

			const response = await fetch(
				`https://api.github.com/repos/${owner}/${repo}/commits/main`,
			);

			if (!response.ok) {
				return "Unknown";
			}

			const data = await response.json();
			if (!data?.commit?.committer?.date) {
				return "Unknown";
			}

			const commitDate = new Date(data.commit.committer.date);
			const currentDate = new Date();
			const diffTime = Math.abs(
				currentDate.getTime() - commitDate.getTime(),
			);
			const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

			if (diffDays === 0) return "Today";
			if (diffDays === 1) return "Yesterday";
			if (diffDays < 7) return `${diffDays} days ago`;
			if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
			if (diffDays < 365)
				return `${Math.floor(diffDays / 30)} months ago`;
			return `${Math.floor(diffDays / 365)} years ago`;
		} catch (error) {
			console.error("Failed to fetch last commit date:", error);
			return "Unknown";
		}
	}

	async function fetchModDirectories() {
		// Check cache
		const cached = getFromCache();
		if (cached && isValidCache(cached.timestamp)) {
			return cached.mods;
		}

		try {
			const response = await fetch(
				"https://api.github.com/repos/skyline69/balatro-mod-index/contents/mods",
			);
			if (!response.ok) {
				throw new Error(`GitHub API returned ${response.status}`);
			}

			const directories = await response.json();
			const mods = await Promise.all(
				directories.map(async (dir: any) => {
					try {
						const [metaResponse, descResponse] = await Promise.all([
							fetch(
								`https://raw.githubusercontent.com/skyline69/balatro-mod-index/main/mods/${dir.name}/meta.json`,
							),
							fetch(
								`https://raw.githubusercontent.com/skyline69/balatro-mod-index/main/mods/${dir.name}/description.md`,
							),
						]);

						if (!metaResponse.ok || !descResponse.ok) {
							throw new Error("Failed to fetch mod data");
						}

						const meta: ModMeta = await metaResponse.json();
						const description = await descResponse.text();

						// Handle last updated date more gracefully
						let lastUpdated = "Unknown";
						try {
							lastUpdated = await getLastUpdated(meta.repo);
						} catch {
							console.log(
								`Failed to fetch last updated for ${meta.title}`,
							);
						}

						// Check image existence without logging 404
						const imageUrl = await checkImageExists(
							`https://raw.githubusercontent.com/skyline69/balatro-mod-index/main/mods/${dir.name}/thumbnail.jpg`,
						);

						return {
							title: meta.title,
							description,
							image: imageUrl,
							lastUpdated,
							categories: meta.categories.map(
								(cat) => Category[cat as keyof typeof Category],
							),
							colors: getRandomColorPair(),
							installed: false,
							requires_steamodded: meta["requires-steamodded"],
							publisher: meta.author,
						};
					} catch (error) {
						console.error(
							`Failed to process mod ${dir.name}:`,
							error,
						);
						return null;
					}
				}),
			);

			saveToCache(mods);
			modsStore.set(mods);
			return mods;
		} catch (error) {
			console.error("Failed to fetch mods:", error);
			return cached?.mods || [];
		}
	}

	const baseCategories = [
		{ name: "Installed Mods", icon: Download },
		{ name: "Search", icon: Search },
		{ name: "All Mods", icon: LayoutDashboard },
		{ name: "Content", icon: FolderHeart },
		{ name: "Joker", icon: Flame },
		{ name: "Quality of Life", icon: Star },
		{ name: "Technical", icon: Spade },
		{ name: "Resource Packs", icon: FolderHeart },
		{ name: "API", icon: Gamepad2 },
	];

	$: categories =
		currentModLoader === "lovely-only"
			? [
					baseCategories[0],
					{ name: "Active Mods", icon: Play },
					...baseCategories.slice(1),
				]
			: baseCategories;

	const colorPairs = [
		{ color1: "#4f6367", color2: "#334461" }, // Blue-grey
		{ color1: "#7E4E60", color2: "#6A3D4F" }, // Wine red
		{ color1: "#4A6670", color2: "#395660" }, // Steel blue
		{ color1: "#5B6E4C", color2: "#4A5C3D" }, // Forest green
		{ color1: "#6B4C6E", color2: "#593E5C" }, // Purple
		{ color1: "#735D45", color2: "#604B35" }, // Brown
		{ color1: "#664E4C", color2: "#523D3B" }, // Rust
		{ color1: "#4E665C", color2: "#3D534A" }, // Pine green
	];

	function getRandomColorPair() {
		return colorPairs[Math.floor(Math.random() * colorPairs.length)];
	}

	function handleModClick(mod: Mod) {
		currentModView.set(mod);
	}

	let showSearch: boolean = false;

	$: showSearch = $currentCategory === "Search";

	$: filteredMods = mods.filter((mod) => {
		switch ($currentCategory) {
			case "Content":
				return Array.isArray(mod.categories)
					? mod.categories.includes(Category.Content)
					: mod.categories === Category.Content;
			case "Joker":
				return Array.isArray(mod.categories)
					? mod.categories.includes(Category.Joker)
					: mod.categories === Category.Joker;
			case "Quality of Life":
				return Array.isArray(mod.categories)
					? mod.categories.includes(Category.QualityOfLife)
					: mod.categories === Category.QualityOfLife;
			case "Technical":
				return Array.isArray(mod.categories)
					? mod.categories.includes(Category.Technical)
					: mod.categories === Category.Technical;
			case "Resource Packs":
				return Array.isArray(mod.categories)
					? mod.categories.includes(Category.ResourcePacks)
					: mod.categories === Category.ResourcePacks;
			case "API":
				return Array.isArray(mod.categories)
					? mod.categories.includes(Category.API)
					: mod.categories === Category.API;
			case "Installed Mods":
				return mod.installed;
			case "All Mods":
				return true;
			default:
				return true;
		}
	});

	function handleCategoryClick(category: string) {
		currentCategory.set(category);
	}

	document.addEventListener("click", (e) => {
		const target = e.target as HTMLElement;
		const anchor = target.closest("a");

		if (anchor && anchor.href.startsWith("https://") && anchor.href) {
			e.preventDefault();
			open(anchor.href);
		}
	});
</script>

<div class="mods-container">
	<div class="categories">
		{#each categories as category}
			<button
				class:active={$currentCategory === category.name}
				on:click={() => handleCategoryClick(category.name)}
			>
				<svelte:component this={category.icon} size={16} />
				{category.name}
			</button>
		{/each}
	</div>

	<div class="separator"></div>

	{#if isLoading}
		<div class="loading-container">
			<p class="loading-text">Loading mods{".".repeat($loadingDots)}</p>
		</div>
	{:else if showSearch}
		<SearchView />
	{:else}
		<div class="mods-grid">
			{#each filteredMods as mod}
				<div
					class="mod-card"
					style="--orig-color1: {mod.colors
						.color1}; --orig-color2: {mod.colors.color2};"
					on:click={() => handleModClick(mod)}
					on:keydown={(e) => e.key === "Enter" && handleModClick(mod)}
					role="button"
					tabindex="0"
				>
					<div class="mod-image">
						<img
							src={mod.image}
							alt={mod.title}
							draggable="false"
						/>
						<div class="tags">
							<span class="tag updated">
								<Clock size={13} />
								{mod.lastUpdated}
							</span>
						</div>
					</div>
					<div class="mod-info">
						<h3>{mod.title}</h3>
						<p>{truncateText(stripMarkdown(mod.description))}</p>
					</div>
					<div class="button-container">
						<button
							class="download-button"
							class:installed={mod.installed}
							disabled={mod.installed}
							on:click|stopPropagation={() => {
								/* handle download */
							}}
						>
							<Download size={16} />
							{mod.installed ? "Installed" : "Download"}
						</button>
						{#if mod.installed}
							<button
								class="delete-button"
								on:click|stopPropagation={() => {
									/* handle delete */
								}}
							>
								<Trash2 size={16} />
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<ModView />

<style>
	.mods-container {
		display: flex;
		gap: 1rem;
		height: 100%;
	}

	.separator {
		width: 2px;
		background: #f4eee0;
		height: 100%;
	}

	.categories {
		width: 190px;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		overflow-y: auto;

		&::-webkit-scrollbar {
			width: 10px;
		}

		&::-webkit-scrollbar-track {
			background: transparent;
			border-radius: 15px;
		}

		&::-webkit-scrollbar-thumb {
			background: #f4eee0;
			border: 2px solid rgba(193, 65, 57, 0.8);
			border-radius: 15px;
		}
		&::-webkit-scrollbar:horizontal {
			display: none;
		}
		&::-webkit-scrollbar-corner {
			background-color: transparent;
		}

		scrollbar-width: 0;
		scrollbar-color: transparent transparent;
	}

	.categories button {
		text-align: left;
		padding: 1rem 1rem;
		background: #ea9600;
		border: 2px solid #f4eee0;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 1.1rem;
		cursor: pointer;
		transition: all 0.2s ease;
		border-radius: 6px;
		margin-right: 0.3rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.categories button:hover {
		background: #f4eee0;
		color: #393646;
	}

	.categories button.active {
		background: #f4eee0;
		color: #393646;
	}

	.mods-grid {
		flex: 1;
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 30px;
		overflow-y: auto;
		padding: 1rem;

		&::-webkit-scrollbar {
			width: 10px;
		}

		&::-webkit-scrollbar-track {
			background: transparent;
			border-radius: 15px;
		}

		&::-webkit-scrollbar-thumb {
			background: #f4eee0;
			border: 2px solid rgba(193, 65, 57, 0.8);
			border-radius: 15px;
		}
		&::-webkit-scrollbar:horizontal {
			display: none;
		}
		&::-webkit-scrollbar-corner {
			background-color: transparent;
		}

		/* scrollbar-width: 0; */
		/* scrollbar-color: transparent transparent; */
	}

	.mod-card {
		--bg-color: var(--orig-color1, #4f6367);
		--bg-color-2: var(--orig-color2, #334461);

		display: flex;
		flex-direction: column;
		position: relative;
		border-radius: 8px;
		overflow: hidden;
		border: 2px solid #f4eee0;
		width: 300px;
		height: 330px;
		margin: 0 auto;
		padding: 1rem;
		box-sizing: border-box;
		cursor: pointer;
		background-size: 100% 200%;
		transition: all 0.3s ease;
		/* Remove the duplicate background property and keep only this one */
		background-image: repeating-linear-gradient(
			-45deg,
			var(--bg-color),
			var(--bg-color) 10px,
			var(--bg-color-2) 10px,
			var(--bg-color-2) 20px
		);
	}

	.mod-card:hover {
		animation: stripe-slide-up 1s linear infinite;
		scale: 1.05;
	}

	@keyframes stripe-slide-up {
		0% {
			background-position: 0 0;
		}
		100% {
			background-position: 0 -20px;
		}
	}

	.mod-image {
		position: relative;
		height: 150px;
	}

	.mod-image img {
		width: 100%;
		height: 100%;
		border-radius: 5px;
		object-fit: cover;
	}

	.tags {
		position: absolute;
		top: 7.2rem; /* Adjusted top position */
		right: 0.35rem; /* Adjusted right position */
		display: flex;
		gap: 0.5rem;
	}
	.tag :global(svg) {
		position: relative;
		top: -1px; /* Adds subtle upward adjustment to the icons */
	}

	.tag {
		display: flex;
		align-items: center;
		position: relative;
		gap: 0.2rem;
		padding: 0.15rem 0.3rem;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 4px;
		font-size: 0.9rem;
		color: #f4eee0;
	}
	.download-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: calc(100% - 2rem); /* Account for parent padding */
		padding: 0.75rem;
		background: #56a786;
		color: #f4eee0;
		border: none;
		outline: #459373 solid 2px;

		border-radius: 4px;
		font-family: "M6X11", sans-serif;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s ease;
		position: absolute;
		bottom: 1rem;
		left: 1rem;

		&.installed {
			background: #808080;
			outline-color: #666666;
			cursor: not-allowed;
		}

		&.installed:hover {
			background: #808080;
			transform: none;
		}
	}

	.download-button:hover {
		background: #74cca8;
		transform: translateY(-2px);
	}

	.download-button:active {
		transform: translateY(0);
	}

	.mod-info {
		flex: 1;
		padding: 0.5rem;
		position: relative;
		bottom: 1rem;
	}

	.mod-info h3 {
		color: #fdcf51;
		font-size: 1.5rem;
		margin-bottom: 0.2rem;
	}

	.mod-info p {
		color: #f4eee0;
		font-size: 1rem;
		line-height: 1.2;
	}

	.button-container {
		display: flex;
		gap: 0.5rem;
		position: absolute;
		bottom: 1rem;
		left: 1rem;
		width: calc(100% - 2rem);
	}

	.download-button {
		flex: 1;
		position: static;
		bottom: auto;
		left: auto;
	}

	.delete-button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.75rem;
		background: #c14139;
		color: #f4eee0;
		border: none;
		outline: #a13029 solid 2px;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.delete-button:hover {
		background: #d4524a;
		transform: translateY(-2px);
	}

	.delete-button:active {
		transform: translateY(0);
	}

	.loading-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		flex: 1;
	}

	.loading-text {
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 1.5rem;
		min-width: 150px;
	}
</style>
