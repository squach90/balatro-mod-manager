<script lang="ts">
	import { fly } from "svelte/transition";
	import { cubicOut } from "svelte/easing";
	import { Download, Clock, Trash2, User, ArrowLeft } from "lucide-svelte";
	import { currentModView } from "../../stores/modStore";
	import type { Mod } from "../../stores/modStore";
	import { marked } from "marked";

	let mod: Mod;
	$: mod = $currentModView!;
	$: renderedDescription = mod?.description ? marked(mod.description) : "";

	function handleClose() {
		currentModView.set(null);
	}
</script>

{#if $currentModView}
	<div
		class="mod-view"
		transition:fly={{ x: 300, duration: 300, easing: cubicOut }}
	>
		<button class="back-button" on:click={handleClose}>
			<ArrowLeft size={20} />
			<span>Back</span>
		</button>

		<div class="mod-content">
			<h2>{mod.title}</h2>

			<div class="content-grid">
				<div class="left-column">
					<div class="image-container">
						<img
							src={mod.image}
							alt={mod.title}
							draggable="false"
						/>
					</div>

					<div class="button-container">
						<button
							class="download-button"
							class:installed={mod.downloaded}
							disabled={mod.downloaded}
						>
							<Download size={18} />
							{mod.downloaded ? "Installed" : "Download"}
						</button>
						{#if mod.downloaded}
							<button class="delete-button" title="Remove Mod">
								<Trash2 size={18} />
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
					<div class="description">{@html renderedDescription}</div>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.mod-view {
		position: fixed;
		top: 0;
		right: 0;
		width: 100%;
		height: 100%;
		background: linear-gradient(to bottom, #393646, #4a4458);
		z-index: 1000;
		overflow-y: auto;
		font-family: "M6X11", sans-serif;

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

	.mod-content {
		max-width: 1000px;
		padding: 3rem;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
	}

	h2 {
		margin-bottom: 2rem;
		font-size: 1.8rem;
	}

	.content-grid {
		display: grid;
		grid-template-columns: 350px 1fr;
		gap: 3rem;
	}

	.image-container {
		border-radius: 8px;
		height: 250px;
		overflow: hidden;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	img {
		width: 100%;
		height: 250px;
		object-fit: cover;
		transition: transform 0.2s ease;
	}

	img:hover {
		transform: scale(1.02);
	}

	.button-container {
		display: flex;
		gap: 0.5rem;
		margin: 1rem 0;
	}

	.download-button {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 1rem;
		background: #56a786;
		color: #f4eee0;
		border: none;
		border-radius: 6px;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: "M6X11", sans-serif;
	}

	.download-button:hover:not(.installed) {
		background: #63b897;
		transform: translateY(-2px);
	}

	.download-button.installed {
		background: #808080;
		cursor: not-allowed;
	}

	.download-button:active:not(.installed) {
		transform: translateY(1px);
	}

	.delete-button {
		padding: 0.75rem;
		background: #c14139;
		color: #f4eee0;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.delete-button:hover {
		background: #d4524a;
		transform: translateY(-2px);
	}

	.mod-stats {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		margin-top: 1rem;
		font-size: 1.1rem;
		padding: 1rem;
		background: rgba(244, 238, 224, 0.1);
		border-radius: 6px;
		justify-content: center;
		align-items: center;
	}

	.mod-stats span {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #f4eee0;
	}

	.description {
		font-size: 1.2rem;
		line-height: 1;
		color: #f4eee0;
		background: rgba(244, 238, 224, 0.05);
		padding: 1.25rem;
		border-radius: 6px;
		width: 50rem;
		height: 21.5rem;
	}

	.back-button {
		position: absolute;
		top: 1rem;
		left: 1rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: rgba(244, 238, 224, 0.1);
		border: none;
		color: #f4eee0;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: "M6X11", sans-serif;
		font-size: 1rem;
	}

	.back-button:hover {
		background: rgba(244, 238, 224, 0.2);
		transform: translateX(-5px);
	}

	.description :global(h1),
	.description :global(h2),
	.description :global(h3),
	.description :global(h4) {
		margin-top: 1em;
		margin-bottom: 0.5em;
		color: #f4eee0;
	}

	.description :global(p) {
		margin-bottom: 1em;
	}

	.description :global(ul),
	.description :global(ol) {
		margin-left: 1.5em;
		margin-bottom: 1em;
	}

	.description :global(li) {
		margin-bottom: 0.5em;
	}

	.description :global(code) {
		background: rgba(244, 238, 224, 0.1);
		padding: 0.2em 0.4em;
		border-radius: 3px;
		font-family: monospace;
	}

	.description :global(pre) {
		background: rgba(244, 238, 224, 0.1);
		padding: 1em;
		border-radius: 6px;
		overflow-x: auto;
		margin: 1em 0;
	}

	.description :global(a) {
		color: #56a786;
		text-decoration: none;
	}

	.description :global(a:hover) {
		text-decoration: underline;
	}

	.description :global(blockquote) {
		border-left: 3px solid #56a786;
		margin: 1em 0;
		padding-left: 1em;
		color: rgba(244, 238, 224, 0.8);
	}

	.description :global(a) {
		-webkit-user-drag: none;
		user-select: none;
		-moz-user-select: none;
		-webkit-user-select: none;
		-ms-user-select: none;
	}

	.delete-button:active {
		transform: translateY(1px);
	}
	@media (max-width: 1160px) {
		.content-grid {
			grid-template-columns: 1fr;
		}
		.image-container {
			width: 100%;
		}
		.right-column {
			bottom: 2rem;
			position: relative;
		}
	}
</style>
