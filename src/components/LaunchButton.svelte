<script lang="ts">
	const handleLaunch = () => {
		console.log("Launch button clicked");
	};
</script>

<div class="launch-container">
	<button class="launch-button" on:click={handleLaunch}> Launch </button>
</div>

<style>
	.launch-container {
		position: absolute;
		top: 2.5rem;
		right: 0rem;
	}

	.launch-button {
		background: #00a2ff;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 3.2rem;
		padding: 0.5rem 2.2rem;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
		text-shadow:
			-2px -2px 0 #000,
			2px -2px 0 #000,
			-2px 2px 0 #000,
			2px 2px 0 #000;
		border-radius: 8px;
		outline: 3px solid #334461;
		box-shadow: inset 0 0 10px rgba(0, 0, 0, 0.3);
	}

	.launch-button:hover {
		background: #0088ff;
		transform: translateY(-2px);
	}

	.launch-button:active {
		transform: translateY(0);
	}

	@media (max-width: 1160px) {
		.launch-button {
			font-size: 2.8rem;
			text-shadow:
				-1.8px -1.8px 0 #000,
				1.8px -1.8px 0 #000,
				-1.8px 1.8px 0 #000,
				1.8px 1.8px 0 #000;
		}
		.launch-container {
			top: 2.4rem;
		}
	}
</style>
