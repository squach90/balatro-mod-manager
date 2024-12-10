<script lang="ts">
	// TODO: Please make the Mod Switch more central. Look at the window
	// TODO: Make the switch more responsive
	let selectedFramework: "balamod" | "steamodded" = "balamod";
</script>

<div class="switch-container">
	<div class="switch-wrapper">
		<span class:active={selectedFramework === "balamod"}>Balamod</span>
		<label class="switch">
			<input
				type="checkbox"
				checked={selectedFramework === "steamodded"}
				on:change={(e) =>
					(selectedFramework = e.target.checked
						? "steamodded"
						: "balamod")}
			/>
			<span class="slider"></span>
		</label>
		<span class:active={selectedFramework === "steamodded"}>Steamodded</span
		>
	</div>
</div>

<style>
	.switch-container {
		position: fixed;
		top: 94%;
		left: 50%; /* Position at 50% of window width */
		right: 50%; /* Position at 50% of window width */
		transform: translateX(
			-50%
		); /* Center by shifting back half of element width */
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.switch-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	span {
		color: #808080;
		font-family: "M6X11", sans-serif;
		font-size: 1.4rem;
		transition: color 0.3s ease;
	}

	span.active {
		color: #f4eee0;
	}

	.switch {
		position: relative;
		display: inline-block;
		bottom: 2px;
		width: 60px;
		height: 32px;
	}

	.switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(57, 54, 70, 0.5);
		transition: 0.3s;
		border-radius: 10px;
		border: 2px solid #f4eee0;
	}

	.slider:before {
		position: absolute;
		content: "";
		height: 24px;
		width: 24px;
		left: 2px;
		bottom: 2px;
		background-color: #f4eee0;
		transition: 0.3s;
		border-radius: 5px;
	}

	input:checked + .slider:before {
		transform: translateX(28px);
	}
</style>
