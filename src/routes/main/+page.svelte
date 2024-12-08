<script lang="ts">
	import ShaderBackground from "../../components/ShaderBackground.svelte";

	let currentSection = "mods";
</script>

<ShaderBackground />
<div class="main-page">
	<header>
		<h1>Balatro Mod Manager</h1>
		<nav>
			<button
				class:active={currentSection === "mods"}
				on:click={() => (currentSection = "mods")}
			>
				Mods
			</button>
			<button
				class:active={currentSection === "settings"}
				on:click={() => (currentSection = "settings")}
			>
				Settings
			</button>
			<button
				class:active={currentSection === "about"}
				on:click={() => (currentSection = "about")}
			>
				About
			</button>
		</nav>
	</header>

	<main>
		{#if currentSection === "mods"}
			<div class="content-section">
				<!-- Mods content -->
				Mods
			</div>
		{:else if currentSection === "settings"}
			<div class="content-section">
				<!-- Settings content -->
				Settings
			</div>
		{:else if currentSection === "about"}
			<div class="content-section">
				<!-- About content -->
				About
			</div>
		{/if}
	</main>

	<div class="content">
		<!-- Your section content here -->
		Content
	</div>

	<div class="version-text">v0.1.0</div>
</div>

<style>
	.main-page {
		width: 100vw;
		height: 100vh;
		display: flex;
		flex-direction: column;
		padding: 2rem;
		box-sizing: border-box;
		background: transparent;
	}

	h1 {
		color: #f4eee0;
		font-size: 3rem;
		margin-bottom: 2rem;
		font-family: "M6X11", sans-serif;
		text-shadow:
			-2px -2px 0 #000,
			2px -2px 0 #000,
			-2px 2px 0 #000,
			2px 2px 0 #000;
	}

	nav {
		display: flex;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	button {
		background: transparent;
		border: 2px solid #f4eee0;
		color: #f4eee0;
		padding: 0.6rem 1.2rem;
		border-radius: 8px;
		font-family: "M6X11", sans-serif;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	button:hover,
	button.active {
		background: #f4eee0;
		color: #393646;
	}

	.content {
		flex: 1;
		background: rgba(69, 147, 115, 0.8);
		border-radius: 15px;
		backdrop-filter: blur(10px);
		padding: 2rem;
		margin-bottom: 1.5rem;
	}

	.version-text {
		position: fixed;
		bottom: 1rem;
		right: 1rem;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		text-shadow:
			-1px -1px 0 #000,
			1px -1px 0 #000,
			-1px 1px 0 #000,
			1px 1px 0 #000;
	}

	.content-section {
		font-size: 1.5rem;
		margin-left: 0.5rem;
	}
</style>
