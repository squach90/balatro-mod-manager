<script>
	import ModSwitch from "../ModSwitch.svelte";
</script>

<div class="settings-container">
	<h2>Settings</h2>
	<div class="content">
	<h3>Mod Loader</h3>
		<ModSwitch />
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
</style>
