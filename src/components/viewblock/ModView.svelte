<script lang="ts">
	import { fly } from "svelte/transition";
	import { cubicOut } from "svelte/easing";
	import { Download, Clock, Trash2, User } from "lucide-svelte";
	import { currentModView } from "../../stores/modStore";
	import type { Mod } from "../../stores/modStore";

	let mod: Mod;
	$: mod = $currentModView!;

	function handleClose() {
		currentModView.set(null);
	}
</script>

{#if $currentModView}
	<div
		class="mod-view"
		transition:fly={{ x: 300, duration: 300, easing: cubicOut }}
	>
		<button class="back-button" on:click={handleClose}>&larr; Back</button>

		<div class="mod-content">
			<h2>{mod.title}</h2>

			<div class="content-grid">
				<div class="left-column">
					<img src={mod.image} alt={mod.title} />
					<div class="button-container">
						<button
							class="download-button"
							class:installed={mod.downloaded}
							disabled={mod.downloaded}
						>
							<Download size={16} />
							{mod.downloaded ? "Installed" : "Download"}
						</button>
						{#if mod.downloaded}
							<button class="delete-button">
								<Trash2 size={16} />
							</button>
						{/if}
					</div>
					<div class="mod-stats">
						<span><Download size={16} /> {mod.downloads}</span>
						<span><Clock size={16} /> {mod.lastUpdated}</span>
						<span><User size={16} /> {mod.publisher}</span>
					</div>
				</div>

				<div class="right-column">
					<p class="description">{mod.description}</p>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.mod-content {
		max-width: 1000px;
		margin: 2rem auto;
		padding: 2rem;
		color: #f4eee0;
	}

	.content-grid {
		display: grid;
		grid-template-columns: 300px 1fr;
		gap: 2rem;
		margin-top: 1.5rem;
	}

	.left-column {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.right-column {
		padding: 1rem;
	}

	img {
		width: 100%;
		height: 200px;
		object-fit: cover;
		border-radius: 8px;
	}

	.button-container {
		display: flex;
		gap: 0.5rem;
	}

	.mod-stats {
		display: flex;
		gap: 1rem;
	}

	.description {
		font-size: 1.1rem;
		line-height: 1.6;
		color: #f4eee0;
	}

	.mod-stats {
		display: flex;
		gap: 1rem;
		margin: 1rem 0;
	}

	.mod-stats span {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.button-container {
		display: flex;
		gap: 0.5rem;
		margin-top: 2rem;
	}

	.download-button {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
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

	.download-button.installed {
		background: #808080;
		outline-color: #666666;
		cursor: not-allowed;
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

	.mod-view {
		position: fixed;
		top: 0;
		right: 0;
		width: 100%;
		height: 100%;
		background: #393646;
		z-index: 1000;
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
	}

	.back-button {
		position: absolute;
		top: 1rem;
		left: 1rem;
		background: none;
		border: none;
		color: #f4eee0;
		font-size: 1.2rem;
		cursor: pointer;
		padding: 0.5rem 1rem;
		transition: transform 0.2s;
		font-size: 1.3rem;
		font-family: "M6X11", sans-serif;
	}

	.back-button:hover {
		transform: translateX(-5px);
	}
</style>
