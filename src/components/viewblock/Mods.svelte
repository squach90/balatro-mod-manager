<script lang="ts">
	import { Clock, Download } from "lucide-svelte";

	interface Mod {
		title: string;
		description: string;
		image: string;
		downloads: string;
		lastUpdated: string;
		category: string;
	}

	const categories = [
		"Popular",
		"Recent",
		"Featured",
		"Card Mods",
		"Gameplay",
		"UI",
		"Collections",
	];

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
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
		},
		{
			title: "Extended Deck",
			description:
				"Adds 50+ new cards to the game with unique mechanics.",
			image: "/images/cover.jpg",
			downloads: "2.5k",
			lastUpdated: "2 days",
			category: "Card Mods",
		},
		// Your other mods...
	];
</script>

<div class="mods-container">
	<div class="categories">
		{#each categories as category}
			<button
				class:active={selectedCategory === category}
				on:click={() => (selectedCategory = category)}
			>
				{category}
			</button>
		{/each}
	</div>

	<div class="separator"></div>

	<div class="mods-grid">
		{#each mods as mod}
			<div class="mod-card">
				<div class="mod-image">
					<img src={mod.image} alt={mod.title} />
					<div class="tags">
						<span class="tag downloads">
							<Download size={12} />
							{mod.downloads}
						</span>
						<span class="tag updated">
							<Clock size={12} />
							{mod.lastUpdated}
						</span>
					</div>
				</div>
				<div class="mod-info">
					<h3>{mod.title}</h3>
					<p>{mod.description}</p>
				</div>
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
			width: 0;
		}

		&::-webkit-scrollbar-track {
			background: transparent;
			border-radius: 0;
			width: 0;
		}

		&::-webkit-scrollbar-thumb {
			background: transparent;
		}

		scrollbar-width: 0;
		scrollbar-color: transparent transparent;
	}

	.categories button {
		text-align: left;
		padding: 1rem 1rem;
		background: transparent;
		border: 2px solid #f4eee0;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 1.2rem;
		cursor: pointer;
		transition: all 0.2s ease;
		border-radius: 6px;
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

		scrollbar-width: thin;
		scrollbar-color: #f4eee0 transparent;
	}

	.mod-card {
		background: rgba(57, 54, 70, 0.3);
		border-radius: 6px;
		overflow: hidden;
		border: 2px solid #f4eee0;
		width: 300px;
		height: 290px;
		margin: 0 auto;
		padding: 1rem; /* Added padding */
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

	.tag {
		display: flex;
		align-items: center;
		gap: 0.2rem;
		padding: 0.15rem 0.3rem;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #f4eee0;
	}

	.mod-info {
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
