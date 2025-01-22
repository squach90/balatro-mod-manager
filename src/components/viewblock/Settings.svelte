<script lang="ts">
	import PathSelector from "../PathSelector.svelte";
	import MessageStack from "../MessageStack.svelte";
	import { Settings2 } from "lucide-svelte"; // Import the Settings icon

	import { invoke } from "@tauri-apps/api/core";

	let messageStack: MessageStack;
	let isReindexing = false;

	async function reindexMods() {
		isReindexing = true;
		try {
			await invoke("refresh_mods_folder");
			messageStack.addMessage("Successfully re-indexed mods!", "success");
		} catch (error) {
			messageStack.addMessage(
				"Failed to re-index mods: " + error,
				"error",
			);
		} finally {
			isReindexing = false;
		}
	}
</script>

<div class="settings-container">
	<h2>Settings</h2>
	<div class="content">
		<h3>Game Path</h3>
		<PathSelector />

		<h3>Mods</h3>
		<div class="mods-settings">
			<button
				class="reindex-button"
				on:click={reindexMods}
				disabled={isReindexing}
			>
				{#if isReindexing}
					<div class="throbber"></div>
				{:else}
					<Settings2 size={20} />
					Reindex Mods
				{/if}
			</button>
			<p class="description">
				Removes any untracked mods from the Mods folder
			</p>
		</div>
	</div>
</div>

<MessageStack bind:this={messageStack} />

<style>
	h2 {
		font-size: 2.5rem;
		margin-bottom: 2rem;
		color: #fdcf51;
	}
	h3 {
		font-size: 1.8rem;
		margin-bottom: 1rem;
		align-self: flex-start;
		color: #fdcf51;
	}
	.content {
		flex: 1;
	}

	.reindex-button {
		background: #56a786;
		color: #f4eee0;
		border: none;
		outline: #459373 solid 2px;
		border-radius: 4px;
		padding: 0.75rem 1.5rem;
		font-family: "M6X11", sans-serif;
		font-size: 1.2rem;
		cursor: pointer;
		transition: all 0.2s ease;
		align-self: flex-start;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.reindex-button:hover {
		background: #74cca8;
		transform: translateY(-2px);
	}

	.throbber {
		width: 20px;
		height: 20px;
		border: 3px solid #f4eee0;
		border-radius: 50%;
		border-top-color: transparent;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.reindex-button:disabled {
		cursor: not-allowed;
		opacity: 0.8;
		transform: none;
	}

	@media (max-width: 1160px) {
		h2 {
			font-size: 2rem;
			transition: all 0.2s ease;
		}
		h3 {
			font-size: 1.5rem;
			transition: all 0.2s ease;
		}
		.reindex-button {
			font-size: 1rem;
			padding: 0.6rem 1.2rem;
		}
	}
</style>
