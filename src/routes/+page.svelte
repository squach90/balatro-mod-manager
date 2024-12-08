<script lang="ts">
	import BalatroPicker from "../components/BalatroPicker.svelte";
	import { Menu, MenuItem } from "@tauri-apps/api/menu";
	import { onMount } from "svelte";

	// Add context menu listener
	window.addEventListener("contextmenu", async (e) => {
		e.preventDefault();

		const menuItems = [
			await MenuItem.new({
				text: "Copy",
				action: () => {
					// Copy action
				},
			}),
			await MenuItem.new({
				text: "Paste",
				action: () => {
					// Paste action
				},
			}),
		];

		const menu = await Menu.new({ items: menuItems });
		menu.popup();
	});

	onMount(() => {
		const prefersDark = window.matchMedia(
			"(prefers-color-scheme: dark)",
		).matches;
		document.documentElement.setAttribute(
			"data-theme",
			prefersDark ? "dark" : "light",
		);

		// Listen for system theme changes
		window
			.matchMedia("(prefers-color-scheme: dark)")
			.addEventListener("change", (e) => {
				document.documentElement.setAttribute(
					"data-theme",
					e.matches ? "dark" : "light",
				);
			});
	});
</script>

<div class="video-background">
	<video autoplay loop muted playsinline>
		<source src="/videos/BalatroGameplay.mov" type="video/quicktime" />
	</video>
</div>

<div class="app">
	<h1>Welcome to Balatro Mod Manager</h1>
	<BalatroPicker />
	<div class="version-text">v0.1.0</div>
</div>

<style>
	@font-face {
		font-family: "M6X11";
		src: url("/fonts/m6x11.ttf") format("truetype");
		font-display: swap;
	}
	.app {
		width: 100vw;
		height: 100vh;
		background-color: transparent;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 2rem;
		box-sizing: border-box;
		overflow: hidden;
	}

	h1 {
		color: #f4eee0;
		font-size: 3.5rem;
		margin-bottom: 3rem;
		font-family: "M6X11", sans-serif;
		text-shadow:
			-2px -2px 0 #000,
			2px -2px 0 #000,
			-2px 2px 0 #000,
			2px 2px 0 #000;
	}

	:global(html) {
		overflow: hidden;
	}

	:root {
		font-family: "M6X11", sans-serif;
		font-size: 1rem;
		line-height: 24px;
		font-weight: 400;
		color: var(--text-primary);
		background-color: var(--background-primary);
		font-synthesis: none;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		-webkit-text-size-adjust: 100%;
		-webkit-user-select: none;
		user-select: none;
		cursor: default;
		text-shadow:
			-1px -1px 0 #000,
			1px -1px 0 #000,
			-1px 1px 0 #000,
			1px 1px 0 #000;
	}

	:root {
		/* Base Colors */
		--color-dark: #459373;
		--color-medium: #56a786;
		--color-light: #74cca8;
		--color-cream: #f4eee0;

		/* Functional Colors */
		--text-primary: var(--color-cream);
		--text-secondary: var(--color-medium);
		--background-primary: var(--color-dark);
		--background-secondary: var(--color-medium);
		--accent: var(--color-light);

		/* System Colors */
		--error: rgb(244, 67, 54);
		--success: rgb(76, 175, 80);
		--warning: rgb(255, 152, 0);
	}

	.video-background {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: -1;
		overflow: hidden;
		filter: blur(1px);
	}
	.version-text {
		position: fixed;
		bottom: 1rem;
		right: 1rem;
		color: #f4eee0;
		font-size: 1rem;
		opacity: 0.8;
		font-family: "M6X11", sans-serif;
		text-shadow:
			-1px -1px 0 #000,
			1px -1px 0 #000,
			-1px 1px 0 #000,
			1px 1px 0 #000;
	}

	video {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0.8; /* Dim the video to maintain readability */
	}

	:global(body) {
		margin: 0;
		padding: 0;
		background-color: transparent;
	}
</style>
