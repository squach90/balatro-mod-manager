<script lang="ts">
	// TODO: Implement search functionality with meilisearch
	import { Search } from "lucide-svelte";
	import type { Mod } from "../../stores/modStore";

	let searchQuery = "";
	let searchResults: Mod[] = [];
	let isSearching = false;

	async function handleSearch() {
		isSearching = true;
		// Implement your search logic here
		// This is a placeholder implementation
		searchResults = []; // Reset results
		isSearching = false;
	}

	function handleInput() {
		if (searchQuery.length >= 2) {
			handleSearch();
		}
	}
</script>

<div class="search-container">
	<div class="search-bar">
		<form on:submit|preventDefault={handleSearch}>
			<input
				type="text"
				bind:value={searchQuery}
				on:input={handleInput}
				placeholder="Search mods..."
				class="search-input"
			/>
			<button type="submit" class="search-button">
				<Search size={20} />
			</button>
		</form>
	</div>

	<div class="results-container">
		{#if isSearching}
			<p>Searching...</p>
		{:else if searchResults.length === 0}
			<p>No mods found</p>
		{:else}
			{#each searchResults as mod}
				<div class="mod-card">
					<h3>{mod.title}</h3>
					<p>{mod.description}</p>
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
		padding: 1rem;
		border: 2px solid #f4eee0;
		border-radius: 6px;
		background: rgba(234, 150, 0, 0.1);
	}

	@media (max-width: 1160px) {
		.search-container {
			width: 70%;
		}
	}
</style>
