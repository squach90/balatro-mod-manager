<script lang="ts">
	import { fly } from "svelte/transition";

	type Message = {
		id: number;
		text: string;
		type: "success" | "error" | "warning" | "info";
		position: number;
	};

	const types = {
		success: {
			bg: "#459373",
			border: "white",
			icon: "✓",
		},
		error: {
			bg: "#644047",
			border: "#A25B5B",
			icon: "✕",
		},
		warning: {
			bg: "#625C43",
			border: "#A39358",
			icon: "!",
		},
		info: {
			bg: "#435662",
			border: "#587A93",
			icon: "i",
		},
	};

	let messages: Message[] = [];
	let counter = 0;

	export function addMessage(
		text: string,
		type: "success" | "error" | "warning" | "info",
	) {
		const id = counter++;
		// Find the lowest available position
		const usedPositions = messages.map((m) => m.position);
		let position = 0;
		while (usedPositions.includes(position)) {
			position++;
		}

		messages = [...messages, { id, text, type, position }];
		setTimeout(() => {
			messages = messages.filter((m) => m.id !== id);
		}, 3000);
	}
</script>

<div class="message-stack">
	{#each messages as message (message.id)}
		<div
			class="message-box {message.type}"
			in:fly={{ x: 300, duration: 300 }}
			out:fly={{ y: 100, duration: 200 }}
			style="top: {2 + message.position * 4.5}rem"
		>
			<span class="icon">{types[message.type].icon}</span>
			<p>{message.text}</p>
		</div>
	{/each}
</div>

<style>
	.message-box {
		position: fixed;
		top: 2rem;
		right: 2rem;
		padding: 1rem 1.5rem;
		border-radius: 12px;
		display: flex;
		align-items: center;
		gap: 1rem;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		z-index: 1000;
		min-width: 300px;
		max-width: 400px;
		color: #f4eee0;
		font-family: inherit;
	}

	.success {
		background-color: #459373;
		border: 2px solid white;
	}

	.error {
		background-color: #644047;
		border: 2px solid #a25b5b;
	}

	.info {
		background-color: #435662;
		border: 2px solid #587a93;
	}

	.warning {
		background-color: #625c43;
		border: 2px solid #a39358;
	}

	.icon {
		font-size: 1.2rem;
		font-weight: bold;
	}

	p {
		margin: 0;
		font-size: 1.2rem;
		line-height: 1.4;
	}
</style>
