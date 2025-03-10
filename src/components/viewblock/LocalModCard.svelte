<script lang="ts">
	import { Trash2 } from "lucide-svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { addMessage } from "$lib/stores";

	export let mod: any;
	export let onUninstall: (mod: any) => void;

	async function uninstallMod(e: Event) {
		e.stopPropagation(); // Prevent card click if we have one
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

	// Generate random colors like the ModCard does
	const bgColor = getRandomColor();
	const bgColor2 = darkenColor(bgColor, 20);

	function getRandomColor() {
		const colors = [
			"#4f6367",
			"#AA778D",
			"#A2615E",
			"#A48447",
			"#4F7869",
			"#728DBF",
			"#5D5E8F",
			"#796E9E",
			"#64825D",
			"#86A367",
			"#748C8A",
		];
		return colors[Math.floor(Math.random() * colors.length)];
	}

	function darkenColor(color: string, percent: number) {
		const num = parseInt(color.replace("#", ""), 16);
		const amt = Math.round(2.55 * percent);
		const R = (num >> 16) - amt;
		const G = ((num >> 8) & 0x00ff) - amt;
		const B = (num & 0x0000ff) - amt;
		return (
			"#" +
			(
				0x1000000 +
				(R < 0 ? 0 : R) * 0x10000 +
				(G < 0 ? 0 : G) * 0x100 +
				(B < 0 ? 0 : B)
			)
				.toString(16)
				.slice(1)
		);
	}
</script>

<div class="mod-card" style="--bg-color: {bgColor}; --bg-color-2: {bgColor2};">
	<div class="blur-bg"></div>
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

	<div class="button-container">
		<button class="delete-button" title="Remove Mod" onclick={uninstallMod}>
			<Trash2 size={18} />
			Remove
		</button>
	</div>
</div>

<style>
	.mod-card {
		--bg-color: var(--bg-color, #4f6367);
		--bg-color-2: var(--bg-color-2, #334461);

		display: flex;
		flex-direction: column;
		position: relative;
		border-radius: 8px;
		overflow: hidden;
		border: 2px solid #f4eee0;
		width: 300px;
		max-width: 300px;
		height: 330px;
		margin: 0 auto;
		padding: 1rem;
		box-sizing: border-box;
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

	.blur-bg {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		backdrop-filter: blur(5px);
		-webkit-backdrop-filter: blur(5px);
		background-color: rgba(0, 0, 0, 0.2);
		z-index: 1;
		pointer-events: none;
	}

	.mod-card:hover {
		animation: stripe-slide-up 1.5s linear infinite;
		transform: translateY(-4px);
		box-shadow: 0 6px 12px rgba(0, 0, 0, 0.15);
	}

	.mod-card:hover .blur-bg {
		backdrop-filter: blur(3px);
		-webkit-backdrop-filter: blur(3px);
		background-color: rgba(0, 0, 0, 0.1);
	}

	@keyframes stripe-slide-up {
		0% {
			background-position: 0 0;
		}
		100% {
			background-position: 0 -55px;
		}
	}

	.mod-content {
		flex: 1;
		padding: 0.5rem;
		position: relative;
		z-index: 2;
	}

	h3 {
		color: #fdcf51;
		font-size: 1.5rem;
		margin-bottom: 0.5rem;
		text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
	}

	.description {
		color: #f4eee0;
		font-size: 1.1rem;
		line-height: 1.3;
		margin-bottom: 1rem;
		overflow: hidden;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
	}

	.mod-meta {
		font-size: 1rem;
		color: #f4eee0;
		margin-bottom: 1rem;
	}

	.version {
		margin-top: 0.3rem;
	}

	/* Match the ModCard button styling */
	.button-container {
		display: flex;
		gap: 0.5rem;
		position: absolute;
		bottom: 1rem;
		left: 1rem;
		width: calc(100% - 2rem);
		z-index: 2;
	}

	.delete-button {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #c14139;
		color: #f4eee0;
		border: none;
		outline: #a13029 solid 2px;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: "M6X11", sans-serif;
		font-size: 1.1rem;
	}

	.delete-button:hover {
		background: #d4524a;
		transform: translateY(-2px);
	}

	.delete-button:active {
		transform: translateY(1px);
	}

	@media (max-width: 1160px) {
		.mod-card {
			width: 100%;
		}
	}
</style>

