<script lang="ts">
	import { Trash2 } from "lucide-svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { addMessage } from "$lib/stores";

	export let mod: any;
	export let onUninstall: (mod: any) => void;

	async function uninstallMod() {
		try {
			await invoke("delete_manual_mod", {
				path: mod.path,
			});

			addMessage(`Removed ${mod.name}`, "success");
			onUninstall(mod);
		} catch (error) {
			addMessage(`Failed to remove mod: ${error}`, "error");
		}
	}
</script>

<div class="mod-card">
	<div class="mod-content">
		<h3>{mod.name}</h3>
		<p class="description">{mod.description}</p>

		<div class="mod-meta">
			<div class="author">
				<span>By: {mod.author.join(", ")}</span>
			</div>
			{#if mod.version}
				<div class="version">
					<span>Version: {mod.version}</span>
				</div>
			{/if}
		</div>
	</div>

	<div class="actions">
		<button class="uninstall-button" on:click={uninstallMod}>
			<Trash2 size={16} />
			Remove
		</button>
	</div>
</div>

<style>
	.mod-card {
		background: #4f6367;
		border: 2px solid #f4eee0;
		border-radius: 8px;
		overflow: hidden;
		position: relative;
		display: flex;
		flex-direction: column;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		transition:
			transform 0.2s,
			box-shadow 0.2s;
	}

	.mod-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
	}

	.mod-content {
		padding: 1.5rem;
		flex: 1;
	}

	h3 {
		margin: 0 0 0.5rem 0;
		font-size: 1.25rem;
		color: #f4eee0;
	}

	.description {
		font-size: 0.9rem;
		color: #f4eee0;
		margin-bottom: 1rem;
		overflow: hidden;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
	}

	.mod-meta {
		font-size: 0.8rem;
		color: #f4eee0;
		margin-bottom: 1rem;
	}

	.actions {
		display: flex;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.1);
		border-top: 1px solid rgba(244, 238, 224, 0.2);
	}

	.uninstall-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #c14139;
		color: #f4eee0;
		border: none;
		border-radius: 4px;
		padding: 0.5rem 1rem;
		font-family: "M6X11", sans-serif;
		cursor: pointer;
		transition: background 0.2s;
	}

	.uninstall-button:hover {
		background: #d45a53;
	}
</style>

