<script lang="ts">
	import { Download, Trash2, AlertCircle, CheckCircle2 } from "lucide-svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { addMessage } from "$lib/stores";

	export let mod: any;
	export let onRegister: (mod: any) => void;
	export let onUninstall: (mod: any) => void;

	async function registerMod() {
		try {
			await invoke("register_local_mod", {
				modName: mod.name,
				modPath: mod.path,
				dependencies: mod.dependencies,
				version: mod.version,
			});

			addMessage(`Registered ${mod.name} in mod manager`, "success");
			onRegister(mod);
		} catch (error) {
			addMessage(`Failed to register mod: ${error}`, "error");
		}
	}

	async function uninstallMod() {
		try {
			// If the mod is tracked, use the remove_installed_mod API
			if (mod.is_tracked) {
				await invoke("remove_installed_mod", {
					name: mod.name,
					path: mod.path,
				});
			} else {
				// For untracked mods, we can use a direct deletion
				await invoke("delete_untracked_mod", {
					path: mod.path,
				});
			}

			addMessage(`Removed ${mod.name}`, "success");
			onUninstall(mod);
		} catch (error) {
			addMessage(`Failed to remove mod: ${error}`, "error");
		}
	}
</script>

<div class="mod-card {mod.is_tracked ? 'tracked-mod' : ''}">
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

		<div class="status-indicator">
			{#if mod.is_tracked}
				<div class="tracked">
					<CheckCircle2 size={16} />
					<span>Tracked in mod manager</span>
				</div>
			{:else}
				<div class="untracked">
					<AlertCircle size={16} />
					<span>Not tracked</span>
				</div>
			{/if}
		</div>
	</div>

	<div class="actions">
		{#if !mod.is_tracked}
			<button class="register-button" on:click={registerMod}>
				<Download size={16} />
				Register Mod
			</button>
		{/if}
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

	.status-indicator {
		margin-top: 0.5rem;
		font-size: 0.85rem;
	}

	.tracked {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #4caf50;
	}

	.untracked {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #ffc107;
	}

	.actions {
		display: flex;
		padding: 1rem;
		gap: 0.5rem; /* Add gap between buttons */
		background: rgba(0, 0, 0, 0.1);
		border-top: 1px solid rgba(244, 238, 224, 0.2);
	}

	.register-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: #ea9600;
		color: #f4eee0;
		border: none;
		border-radius: 4px;
		padding: 0.5rem 1rem;
		font-family: "M6X11", sans-serif;
		cursor: pointer;
		transition: background 0.2s;
	}

	.register-button:hover {
		background: #f4eee0;
		color: #393646;
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
