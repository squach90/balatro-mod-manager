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
	} from "lucide-svelte";
	import ModView from "./ModView.svelte";
	import { currentModView } from "../../stores/modStore";
	import type { Mod } from "../../stores/modStore";
	import { open } from "@tauri-apps/plugin-shell";
	import { stripMarkdown, truncateText } from "../../utils/helpers";
	import SearchView from "./SearchView.svelte";

	const categories = [
		{ name: "Installed Mods", icon: Download },
		{ name: "Search", icon: Search },
		{ name: "Popular", icon: Flame },
		{ name: "Recent", icon: Clock },
		{ name: "Featured", icon: Star },
		{ name: "Card Mods", icon: Spade },
		{ name: "Gameplay", icon: Gamepad2 },
		{ name: "UI", icon: LayoutDashboard },
		{ name: "Collections", icon: FolderHeart },
	];

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

	document.addEventListener("click", (e) => {
		const target = e.target as HTMLElement;
		const anchor = target.closest("a");

		if (anchor && anchor.href.startsWith("https://") && anchor.href) {
			e.preventDefault();
			open(anchor.href);
		}
	});

	let selectedCategory = "Popular";
	let showSearch = false;

	function handleCategoryClick(category: string) {
		selectedCategory = category;
		showSearch = category === "Search";
	}

	const mods: Mod[] = [
		{
			title: "Extended Deck",
			description:
				"# Lorem ipsum\n## dolor sit amet,\n [consetetur sadipscing](https://dasguney.com) elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
			colors: getRandomColorPair(),
			downloaded: false,
			publisher: "Joe Mama",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
			colors: getRandomColorPair(),
			downloaded: true,
			publisher: "Joe Mama",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
			colors: getRandomColorPair(),
			downloaded: false,
			publisher: "Joe Mama",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
			colors: getRandomColorPair(),
			downloaded: false,
			publisher: "Joe Mama",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
			colors: getRandomColorPair(),
			downloaded: true,
			publisher: "Joe Mama",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
			colors: getRandomColorPair(),
			downloaded: true,
			publisher: "Joe Mama",
		},
	];
</script>

<div class="mods-container">
	<div class="categories">
		{#each categories as category}
			<button
				class:active={selectedCategory === category.name}
				on:click={() => handleCategoryClick(category.name)}
			>
				<svelte:component this={category.icon} size={16} />
				{category.name}
			</button>
		{/each}
	</div>

	<div class="separator"></div>

	{#if showSearch}
		<SearchView />
	{:else}
		<div class="mods-grid">
			{#each mods as mod}
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
						<img src={mod.image} alt={mod.title} />
						<div class="tags">
							<span class="tag downloads">
								<Download size={13} />
								{mod.downloads}
							</span>
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
							class:installed={mod.downloaded}
							disabled={mod.downloaded}
							on:click|stopPropagation={() => {
								/* handle download */
							}}
						>
							<Download size={16} />
							{mod.downloaded ? "Installed" : "Download"}
						</button>
						{#if mod.downloaded}
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
		width: 180px;
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
</style>
