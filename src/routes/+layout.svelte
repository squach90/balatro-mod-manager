<script lang="ts">
	import { blur } from "svelte/transition";
	import MessageStack from "../components/MessageStack.svelte";
	import { backgroundEnabled } from "../stores/modStore";
	import { onMount } from "svelte";
    import DragDropOverlay from "../components/DragDropOverlay.svelte";

	import "../app.css";

	export let data;

	let isWindows = false;

	onMount(() => {
		isWindows = navigator.userAgent.indexOf("Windows") !== -1;
	});
</script>

<MessageStack />
<DragDropOverlay />
<div
	class="layout-container"
	style:--gradient-opacity={$backgroundEnabled ? 0 : 1}
	style:--dot-size={isWindows ? "1.5px" : "0.45px"}
	style:--dot-color={isWindows ? "#ff9999" : "#d66060"}
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

