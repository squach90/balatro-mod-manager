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

<main>
	<h1 id="welcome-message">Welcome to Balatro Mod Manager</h1>
	<BalatroPicker />
</main>

<style>
	@font-face {
		font-family: "Quicksand";
		src:
			url("/fonts/quicksand/Quicksand-Regular.otf") format("opentype"),
			url("/fonts/quicksand/Quicksand-Italic.otf") format("opentype"),
			url("/fonts/quicksand/Quicksand-Bold.otf") format("opentype"),
			url("/fonts/quicksand/Quicksand-BoldItalic.otf") format("opentype");
		font-display: swap;
	}
	@font-face {
		font-family: "Blokletters";
		src:
			url("/fonts/blokletters/Blokletters-Balpen.ttf") format("truetype"),
			url("/fonts/blokletters/Blokletters-Potlood.ttf") format("truetype"),
			url("/fonts/blokletters/Blokletters-Viltstift.ttf")
				format("truetype");
		font-display: swap;
	}

	:root {
		font-family: "Blokletters", sans-serif;
		font-size: 16px;
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
	}

	:root {
		/* Base Colors */
		--color-dark: #393646;
		--color-medium: #4f4557;
		--color-light: #6d5d6e;
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

	:root[data-theme="light"] {
		--text-primary: var(--color-dark);
		--text-secondary: var(--color-medium);
		--background-primary: var(--color-cream);
		--background-secondary: var(--color-light);
	}

	/* Dark Theme */
	:root[data-theme="dark"] {
		--text-primary: var(--color-cream);
		--text-secondary: var(--color-light);
		--background-primary: var(--color-dark);
		--background-secondary: var(--color-medium);
	}

	#welcome-message {
		text-align: center;
	}
	h1 {
		font-size: 2rem;
		font-weight: 700;
		margin: 0;
		color: var(--text-primary);
	}

	main {
		font-family: "Blokletters", sans-serif;
		margin: 1rem;
		margin-top: 3rem;
		margin-bottom: 2rem;
		-webkit-user-select: none; /* Safari */
		user-select: none; /* Standard syntax */
		cursor: default;
	}

	@media (prefers-color-scheme: dark) {
		:root {
			color: var(--text-primary);
			background-color: var(--background-primary);
		}
	}
</style>
