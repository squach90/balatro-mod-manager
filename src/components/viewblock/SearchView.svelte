<script lang="ts">
	import { Clock, Download, Search, Trash2 } from "lucide-svelte";
	import type { Mod } from "../../stores/modStore";
	import { onMount } from "svelte";
	import { modsStore } from "../../stores/modStore";
	import { debounce } from "lodash";
	import FlexSearch from "flexsearch";
	import { stripMarkdown, truncateText } from "../../utils/helpers";
	import { currentModView } from "../../stores/modStore";

	let searchQuery = "";
	let searchResults: Mod[] = [];
	let isSearching = false;
	let searchIndex: any;
	let mods: Mod[] = [];

	function handleModClick(mod: Mod) {
		currentModView.set(mod);
	}

	onMount(() => {
		// Initialize the search index
		searchIndex = new FlexSearch.Index({
			tokenize: "forward",
			preset: "match",
			cache: true,
		});

		// Subscribe to mods store
		return modsStore.subscribe((currentMods) => {
			mods = currentMods;
			if (mods.length > 0) {
				// Instead of clear(), recreate the index
				searchIndex = new FlexSearch.Index({
					tokenize: "forward",
					preset: "match",
					cache: true,
				});

				mods.forEach((mod, idx) => {
					const searchText =
						`${mod.title} ${mod.publisher}`.toLowerCase();
					searchIndex.add(idx, searchText);
				});
			}
		});
	});

	const handleSearch = debounce(() => {
		if (!searchIndex || searchQuery.length < 2) {
			searchResults = [];
			return;
		}

		isSearching = true;
		try {
			const searchTerm = searchQuery.toLowerCase();
			const results = searchIndex.search(searchTerm);
			// console.log("Search results:", results); // Debug

			searchResults = results.map((idx: number) => mods[idx]);
			// console.log("Mapped results:", searchResults); // Debug
		} catch (error) {
			console.error("Search failed:", error);
			searchResults = [];
		} finally {
			isSearching = false;
		}
	}, 300);

	function handleInput() {
		handleSearch();
	}
</script>

<div class="search-container">
	<div class="search-bar">
		<form on:submit|preventDefault={handleSearch}>
			<input
				type="text"
				bind:value={searchQuery}
				on:input={handleInput}
				placeholder="Search mods... (Author or Title)"
				class="search-input"
			/>
			<button type="submit" class="search-button">
				<Search size={20} />
			</button>
		</form>
	</div>

	<div class="results-container">
		{#if isSearching}
			<p class="loading-text">Searching...</p>
		{:else if searchResults.length === 0 && searchQuery.length >= 2}
			<p>No mods found matching "{searchQuery}"</p>
		{:else if searchResults.length > 0}
			{#each searchResults as mod}
				<div
					class="mod-card"
					on:click={() => handleModClick(mod)}
					on:keydown={(e) => e.key === "Enter" && handleModClick(mod)}
					role="button"
					tabindex="0"
					style="--orig-color1: {mod.colors
						.color1}; --orig-color2: {mod.colors.color2};"
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
						>
							<Download size={16} />
							{mod.installed ? "Installed" : "Download"}
						</button>
						{#if mod.installed}
							<button class="delete-button">
								<Trash2 size={16} />
							</button>
						{/if}
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	.search-container {
		position: relative;
		width: 75%;
		padding: 1rem;
	}

	::selection {
		background: #ea9600;
		color: #f4eee0;
	}

	.search-bar {
		margin-bottom: 2rem;
	}

	.search-bar form {
		display: flex;
		gap: 0.5rem;
	}

	.search-input {
		flex: 1;
		padding: 0.75rem;
		border: 2px solid #f4eee0;
		border-radius: 6px;
		background-color: #393646;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 1.1rem;
	}
	.search-input:focus {
		outline: none;
		border-color: #ea9600;
		transition: border-color 0.2s ease;
	}
	.search-button {
		padding: 0.75rem 1rem;
		background: #ea9600;
		border: 2px solid #f4eee0;
		border-radius: 6px;
		color: #f4eee0;
		cursor: pointer;
		display: flex;
		align-items: center;
		transition: all 0.2s ease;
	}

	.search-button:hover {
		background: #f4eee0;
		color: #393646;
	}

	.search-button:active {
		transform: scale(0.95);
		padding: 0.75rem 0.95rem;
	}

	.results-container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
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
		top: 7.2rem;
		right: 0.35rem;
		display: flex;
		gap: 0.5rem;
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
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
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

	@media (max-width: 1160px) {
		.search-container {
			width: 70%;
		}
	}
</style>
