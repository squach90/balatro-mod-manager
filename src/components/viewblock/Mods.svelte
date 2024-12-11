<script lang="ts">
	// TODO: Do a style for the download button, if the mod is already installed, the button should be disabled and show "Installed" instead of "Download"
	// TODO: Also add a way to get more information about the mod when clicking on the card (somewherd (somewhere)
	// TODO: Move Mod switch to Settings
	import {
		Download,
		Flame,
		Clock,
		Star,
		Spade,
		Gamepad2,
		LayoutDashboard,
		FolderHeart,
	} from "lucide-svelte";

	const categories = [
		{ name: "Installed Mods", icon: Download },
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

	interface Mod {
		title: string;
		description: string;
		image: string;
		downloads: string;
		lastUpdated: string;
		category: string;
		colors: { color1: string; color2: string };
	}

	let selectedCategory = "Popular";

	const mods: Mod[] = [
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
			colors: getRandomColorPair(),
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
		},
		// Your other mods...
	];
</script>

<div class="mods-container">
	<div class="categories">
		{#each categories as category}
			<button
				class:active={selectedCategory === category.name}
				on:click={() => (selectedCategory = category.name)}
			>
				<svelte:component this={category.icon} size={16} />
				{category.name}
			</button>
		{/each}
	</div>

	<div class="separator"></div>

	<div class="mods-grid">
		{#each mods as mod}
			<div
				class="mod-card"
				style="--bg-color: {mod.colors.color1}; --bg-color-2: {mod
					.colors.color2};"
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
					<p>{mod.description}</p>
				</div>
				<button class="download-button">
					<Download size={16} />
					Download
				</button>
			</div>
		{/each}
	</div>
</div>

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

		scrollbar-width: thin;
		scrollbar-color: #f4eee0 transparent;
	}

	.mod-card {
		--bg-color: #4f6367;
		--bg-color-2: #334461;
		display: flex;
		flex-direction: column;
		position: relative;
		background: repeating-linear-gradient(
			45deg,
			var(--bg-color),
			var(--bg-color) 10px,
			var(--bg-color-2) 10px,
			var(--bg-color-2) 20px
		);
		border-radius: 8px;
		overflow: hidden;
		border: 2px solid #f4eee0;
		width: 300px;
		height: 330px;
		margin: 0 auto;
		padding: 1rem;
		box-sizing: border-box;
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
</style>
