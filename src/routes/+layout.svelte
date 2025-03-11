<script lang="ts">
	import { blur } from "svelte/transition";
	import MessageStack from "../components/MessageStack.svelte";
	import { backgroundEnabled } from "../stores/modStore";

	import "../app.css";

	export let data;
</script>

<MessageStack />
<div
	class="layout-container"
	style:--gradient-opacity={$backgroundEnabled ? 0 : 1}
>
	{#key data.url}
		<div
			in:blur={{ duration: 300, delay: 150 }}
			out:blur={{ duration: 150 }}
			class="page-content"
		>
			<slot />
		</div>
	{/key}
</div>

<style>
	.layout-container {
		width: 100%;
		height: 100%;
		position: fixed;
		top: 0;
		left: 0;
		overflow: hidden;
	}

	.layout-container::before {
		content: "";
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		opacity: var(--gradient-opacity, 1);
		transition: opacity 0.4s cubic-bezier(0.4, 0, 0.2, 1);
		background-color: #a53535;
		background-image: radial-gradient(#d66060 0.45px, transparent 0.45px),
			radial-gradient(#d66060 0.45px, #a53535 0.45px);
		background-size: 18px 18px;
		background-position:
			0 0,
			9px 9px;
		z-index: -1;
	}

	.page-content {
		width: 100%;
		height: 100%;
		position: relative;
		overflow: hidden;
	}
</style>

