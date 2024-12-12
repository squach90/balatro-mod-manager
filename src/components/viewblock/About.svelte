<div class="about-container">
	<div class="content">
		<h2>About Balatro Mod Manager</h2>

		<div>
			<h3>What is BMM?</h3>
			<p>
				Balatro Mod Manager (BMM) is a tool designed to help you manage
				and install mods for the game Balatro. It provides an
				easy-to-use interface for mod management while maintaining the
				game's unique aesthetic.
			</p>
		</div>

		<div>
			<h3>Features</h3>
			<ul>
				<li>Easy mod installation and management</li>
				<li>Automatic game path detection</li>
				<li>Mod compatibility checking</li>
				<li>Clean, pixel-perfect interface</li>
			</ul>
		</div>
		<p id="versiontext">Current version: v0.1.0</p>
	</div>

	<div class="profile-section">
		<div class="profile">
			<img src="/images/pb.jpg" alt="" />
		</div>
		<span class="profile-title">Efe/Skyline - The Creator of BMM</span>
	</div>
</div>

<style>
	.about-container {
		display: flex;
		justify-content: space-between;
		gap: 2rem;
		align-items: center;
		&::-webkit-scrollbar {
			width: 10px;
		}

		&::-webkit-scrollbar-track {
			background: transparent;
			border-radius: 15px;
		}

		&::-webkit-scrollbar-thumb {
			background: #f4eee0;
			border: 2px solid rgba(193, 65, 57, 0.8);
			border-radius: 15px;
		}

		&::-webkit-scrollbar:horizontal {
			display: none;
		}

		&::-webkit-scrollbar-corner {
			background-color: transparent;
		}

		/* scrollbar-width: thin; */
		/* scrollbar-color: #f4eee0 transparent; */
	}

	.profile-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.profile {
		flex-shrink: 0;
		width: 200px;
		height: 200px;
		border-radius: 50%;
		overflow: hidden;
		border: 4px solid #f7f1e4;
		box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
	}

	.profile img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.content {
		flex: 1;
	}

	.profile-title {
		color: #f7f1e4;
		font-size: 1rem;
		font-family: "M6X11", sans-serif;
		text-shadow:
			-1px -1px 0 #000,
			1px -1px 0 #000,
			-1px 1px 0 #000,
			1px 1px 0 #000;
	}

	h2 {
		font-size: 2.5rem;
		margin-bottom: 1rem;
		color: #fdcf51;
	}

	h3 {
		font-size: 1.8rem;
		margin-bottom: 0.5rem;
		color: #fde700;
	}

	p {
		font-size: 1.2rem;
		margin-bottom: 1rem;
	}

	#versiontext {
		color: #fde700;
		margin-top: 1rem;
	}

	ul {
		list-style-type: disc;
		margin-left: 1rem;
		margin-bottom: 1rem;
	}

	li {
		font-size: 1.2rem;
		margin-bottom: 0.5rem;
	}

	@media (max-width: 1160px) {
		p {
			font-size: 1rem;
		}
		li {
			font-size: 1rem;
		}

		.profile {
			width: 170px;
			height: 170px;
		}
		.profile-title {
			font-size: 0.8rem;
		}

		h2 {
			font-size: 2rem;
		}
		h3 {
			font-size: 1.5rem;
		}
	}
</style>
