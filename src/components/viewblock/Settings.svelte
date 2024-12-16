<script>
	import ModSwitch from "../ModSwitch.svelte";
	import PathSelector from "../PathSelector.svelte";
</script>

<div class="settings-container">
	<h2>Settings</h2>
	<div class="content">
		<h3>Mod Loader</h3>
		<ModSwitch />
		<h3>Game Path</h3>
		<PathSelector />
	</div>
</div>

<style>
	h2 {
		font-size: 2.5rem;
		margin-bottom: 2rem;
		color: #fdcf51;
	}
	h3 {
		font-size: 1.8rem;
		margin-bottom: 1rem;
		align-self: flex-start;
		color: #fde700;
	}
	.content {
		flex: 1;
	}
	
	@media (max-width: 1160px) {
		h2 {
			font-size: 2rem;
			transition: all 0.2s ease;
		}
		h3 {
			font-size: 1.5rem;
			transition: all 0.2s ease;
		}
	}


</style>
