<script lang="ts">
	import {
		Download,
		Flame,
		// Clock,
		Star,
		Spade,
		Gamepad2,
		LayoutDashboard,
		FolderHeart,
		Search,
		BookOpen,
		Folder,
	} from "lucide-svelte";
	import ModView from "./ModView.svelte";
	import { fly } from "svelte/transition";
	import {
		SortOption,
		backgroundEnabled,
		currentSort,
	} from "../../stores/modStore";
	import { ArrowUpDown } from "lucide-svelte";

	import {
		currentModView,
		currentCategory,
		uninstallDialogStore,
	} from "../../stores/modStore";
	import type { LocalMod, Mod } from "../../stores/modStore";
	import { Category } from "../../stores/modStore";
	import {
		modsStore,
		installationStatus,
		loadingStates2 as loadingStates,
	} from "../../stores/modStore";
	import type { InstalledMod } from "../../stores/modStore";
	import { open } from "@tauri-apps/plugin-shell";
	import { invoke } from "@tauri-apps/api/core";
	import SearchView from "./SearchView.svelte";
	import { onMount } from "svelte";
	import { writable } from "svelte/store";
	import { addMessage } from "$lib/stores";
	import { currentPage, itemsPerPage } from "../../stores/modStore";
	import ModCard from "./ModCard.svelte";
	import LocalModCard from "./LocalModCard.svelte";
	import {
		checkModInCache,
		fetchCachedMods,
		forceRefreshCache,
	} from "../../stores/modCache";
	import { updateAvailableStore } from "../../stores/modStore";

	const loadingDots = writable(0);

	let installedMods: InstalledMod[] = [];

	// Animate the dots
	let dotInterval: number;

	async function handleModUninstalled() {
		// Refresh the local mods list
		getLocalMods();

		// Also refresh installed mods for consistency
		refreshInstalledMods();
	}

	// let mods: Mod[] = [];
	let isLoading = true;
	interface DependencyCheck {
		steamodded: boolean;
		talisman: boolean;
	}

	let localMods: LocalMod[] = [];
	let isLoadingLocalMods = false;

	async function getLocalMods() {
		if ($currentCategory === "Installed Mods") {
			isLoadingLocalMods = true;
			try {
				localMods = await invoke("get_detected_local_mods");
			} catch (error) {
				console.error("Failed to load local mods:", error);
				addMessage(`Failed to load local mods: ${error}`, "error");
				localMods = [];
			} finally {
				isLoadingLocalMods = false;
			}
		}
	}

	$: if ($currentCategory === "Installed Mods") {
		getLocalMods();
		refreshInstalledMods();
	}

	async function checkIfModIsInstalled(mod: Mod) {
		if (!mod?.title) return false;
		// Use checkModInCache (from modCache.ts) which takes a string title
		const status = await checkModInCache(mod.title);
		installationStatus.update((s) => ({ ...s, [mod.title]: status }));
		return status;
	}

	export let handleDependencyCheck: (
		requirements: DependencyCheck,
		downloadAction?: () => Promise<void>,
	) => void;

	// function onDependencyCheck(
	// 	event: CustomEvent<{ steamodded: boolean; talisman: boolean }>,
	// ) {
	// 	handleDependencyCheck(event.detail);
	// }

	export let mod: Mod | null;

	async function updateInstallStatus(mod: Mod | undefined) {
		if (!mod) return;
		const status: boolean = await checkIfModIsInstalled(mod);
		installationStatus.update((s) => ({ ...s, [mod.title]: status }));
	}

	$: {
		if (mod) {
			updateInstallStatus(mod);
		}
	}

	let isProgrammaticScroll = false;

	// Update the pagination functions to reset scroll position when switching pages
	function nextPage() {
		if ($currentPage < totalPages) {
			currentPage.update((n) => n + 1);
			updatePaginationWindow();
			scrollToTop();
		}
	}

	function previousPage() {
		if ($currentPage > 1) {
			currentPage.update((n) => n - 1);
			updatePaginationWindow();
			scrollToTop();
		}
	}

	function goToPage(page: number) {
		currentPage.set(page);
		updatePaginationWindow();
		scrollToTop();
	}

	// Add this helper function to handle scrolling to top
	function scrollToTop() {
		isProgrammaticScroll = true;
		const scrollContainer = document.querySelector(
			".mods-scroll-container",
		);
		if (scrollContainer) {
			scrollContainer.scrollTo({
				top: 0,
				behavior: "smooth",
			});
		}
		setTimeout(() => {
			isProgrammaticScroll = false;
		}, 500); // Delay to prevent scroll handler triggering during animated scroll
	}

	onMount(() => {
		// Animation dots initialization
		dotInterval = setInterval(() => {
			loadingDots.update((n) => (n + 1) % 4);
		}, 500);

		// Separate async function for initialization
		const initialize = async () => {
			const cached = await getFromCache();
			if (
				cached &&
				Date.now() - cached.timestamp * 1000 < CACHE_DURATION
			) {
				modsStore.set(cached.mods);
				isLoading = false;
			} else {
				try {
					isLoading = true;
					const freshMods = await fetchModDirectories();
					modsStore.set(freshMods);
				} finally {
					isLoading = false;
				}
			}

			try {
				await Promise.all(
					$modsStore.map(async (mod) => {
						const status = await checkIfModIsInstalled(mod);
						installationStatus.update((s) => ({
							...s,
							[mod.title]: status,
						}));
					}),
				);
			} catch (error) {
				console.error("Install status check failed:", error);
			}

			if ($currentCategory === "Installed Mods") {
				await getLocalMods();
			}
		};

		// Separate async function for background state
		const initBackgroundState = async () => {
			try {
				const isBackgroundAnimationEnabled: boolean = await invoke(
					"get_background_state",
				);
				backgroundEnabled.set(isBackgroundAnimationEnabled);
			} catch (error) {
				console.error("Failed to get background status:", error);
				addMessage(
					"Error fetching background animation status",
					"error",
				);
			}
		};

		// Call async functions without awaiting them directly in onMount
		initialize();
		initBackgroundState();

		// Return synchronous cleanup function
		return () => {
			clearInterval(dotInterval);
		};
	});

	const getAllInstalledMods = async () => {
		try {
			installedMods = await fetchCachedMods();
		} catch (error) {
			console.error("Failed to get installed mods:", error);
		}
	};

	const uninstallMod = async (mod: Mod) => {
		const isCoreMod = ["steamodded", "talisman"].includes(
			mod.title.toLowerCase(),
		);

		try {
			await getAllInstalledMods();
			const installedMod = installedMods.find(
				(m) => m.name.toLowerCase() === mod.title.toLowerCase(),
			);

			if (!installedMod) return;

			if (isCoreMod) {
				// Get dependents
				const dependents = await invoke<string[]>("get_dependents", {
					modName: mod.title,
				});

				// Set the dialog properties directly
				uninstallDialogStore.set({
					show: true,
					modName: mod.title,
					modPath: installedMod.path,
					dependents,
				});
			} else {
				// For non-core mods
				await invoke("remove_installed_mod", {
					name: mod.title,
					path: installedMod.path,
				});
				installationStatus.update((s) => ({
					...s,
					[mod.title]: false,
				}));
			}
		} catch (error) {
			console.error("Uninstall failed:", error);
			addMessage(`Uninstall failed: ${error}`, "error");
		}
	};

	const installMod = async (mod: Mod) => {
		if (!mod?.title || !mod?.downloadURL) return;

		// Define the actual download function that will be stored and executed later if needed
		const performDownload = async () => {
			try {
				loadingStates.update((s) => ({ ...s, [mod.title]: true }));

				// Create dependencies array for the database
				const dependencies = [];
				if (mod.requires_steamodded) dependencies.push("Steamodded");
				if (mod.requires_talisman) dependencies.push("Talisman");

				const installedPath = await invoke<string>("install_mod", {
					url: mod.downloadURL,
					folderName: mod.folderName || mod.title.replace(/\s+/g, ""),
				});

				await invoke("add_installed_mod", {
					name: mod.title,
					path: installedPath,
					dependencies,
					currentVersion: mod.version || "",
				});

				installationStatus.update((s) => ({ ...s, [mod.title]: true }));
				updateAvailableStore.update((s) => ({
					...s,
					[mod.title]: false,
				}));
				await refreshInstalledMods();
			} catch (error) {
				console.error("Failed to install mod:", error);
				addMessage(
					`Installation failed: ${error instanceof Error ? error.message : String(error)}`,
					"error",
				);
			} finally {
				loadingStates.update((s) => ({ ...s, [mod.title]: false }));
			}
		};

		try {
			// Check for dependencies
			if (mod.requires_steamodded || mod.requires_talisman) {
				// Check Steamodded if required
				const steamoddedInstalled = mod.requires_steamodded
					? await invoke<boolean>("check_mod_installation", {
							modType: "Steamodded",
						})
					: true;

				// Check Talisman if required
				const talismanInstalled = mod.requires_talisman
					? await invoke<boolean>("check_mod_installation", {
							modType: "Talisman",
						})
					: true;

				// If any dependency is missing, show the RequiresPopup
				if (
					(mod.requires_steamodded && !steamoddedInstalled) ||
					(mod.requires_talisman && !talismanInstalled)
				) {
					// Call the handler with the appropriate requirements and download action
					handleDependencyCheck(
						{
							steamodded:
								mod.requires_steamodded && !steamoddedInstalled,
							talisman:
								mod.requires_talisman && !talismanInstalled,
						},
						performDownload,
					);
					return; // Stop installation
				}
			}

			// If we get here, either no dependencies are required or all are installed
			// Proceed with installation directly
			await performDownload();
		} catch (error) {
			console.error("Failed to check dependencies:", error);
			addMessage(
				`Dependency check failed: ${error instanceof Error ? error.message : String(error)}`,
				"error",
			);
		}
	};

	interface ModMeta {
		title: string;
		"requires-steamodded": boolean;
		"requires-talisman": boolean;
		categories: string[];
		author: string;
		repo: string;
		downloadURL?: string;
		folderName?: string;
		version?: string;
	}

	const CACHE_DURATION = 15 * 60 * 1000; // 15 minutes
	// const CACHE_DURATION = 5 * 1000; // 5 seconds

	async function saveToCache(mods: Mod[]) {
		await invoke("save_mods_cache", { mods });
	}

	async function getFromCache(): Promise<{
		mods: Mod[];
		timestamp: number;
	} | null> {
		try {
			const cached = await invoke<[Mod[], number] | null>(
				"load_mods_cache",
			);
			if (!cached) return null;

			const [mods, timestamp] = cached;
			return { mods, timestamp };
		} catch (error) {
			console.error("Error loading cache:", error);
			return null;
		}
	}

	async function fetchModDirectories(): Promise<Mod[]> {
		try {
			isLoading = true;
			const repoPath = await cloneOrUpdateRepo();
			if (!repoPath) return [];

			const modDirs = await invoke<string[]>("list_directories", {
				path: `${repoPath}/mods`,
			});

			const mods = (
				await Promise.all(
					modDirs.map(async (dirName) => {
						try {
							const [meta, description] = await Promise.all([
								invoke<ModMeta>("read_json_file", {
									path: `${repoPath}/mods/${dirName}/meta.json`,
								}),
								invoke<string>("read_text_file", {
									path: `${repoPath}/mods/${dirName}/description.md`,
								}),
							]);

							const imageData: string | undefined =
								await invoke<string>("get_mod_thumbnail", {
									modPath: dirName,
								});

							// const lastUpdated =
							// 	timestamps[dirName] || Date.now();

							// Log category mapping for debugging

							// Ensure categories are properly mapped
							const mappedCategories = meta.categories
								.map((cat) => {
									return categoryMap[cat] ?? null;
								})
								.filter((cat): cat is Category => cat !== null);

							return {
								title: meta.title,
								description,
								image: imageData || "images/cover.jpg",
								// lastUpdated: lastUpdated.toString(),
								colors: getRandomColorPair(),
								categories: mappedCategories,
								requires_steamodded:
									meta["requires-steamodded"],
								requires_talisman: meta["requires-talisman"],
								publisher: meta.author,
								repo: meta.repo,
								downloadURL: meta.downloadURL || "",
								folderName: meta.folderName,
								version: meta.version,
								installed: false,
							} as Mod;
						} catch (error) {
							console.error(
								`Failed to process mod ${dirName}:`,
								error,
							);
							return null;
						}
					}),
				)
			).filter((mod): mod is Mod => mod !== null);

			await saveToCache(mods);
			return mods;
		} catch (error) {
			console.error("Failed to fetch mods:", error);
			return [];
		} finally {
			isLoading = false;
		}
	}

	async function cloneOrUpdateRepo() {
		try {
			const repoPath = await invoke<string>("get_repo_path");
			const exists = await invoke<boolean>("path_exists", {
				path: repoPath,
			});

			if (!exists) {
				await invoke("clone_repo", {
					url: "https://github.com/skyline69/balatro-mod-index.git",
					path: repoPath,
				});
			} else {
				const lastFetched = await invoke<number>("get_last_fetched");
				if (Date.now() - lastFetched > 3600 * 1000) {
					// 1 hour
					await invoke("pull_repo", { path: repoPath });
					await invoke("update_last_fetched");
				}
			}
			return repoPath;
		} catch (error) {
			console.error("Repo management failed:", error);
			return null;
		}
	}
	const categories = [
		{ name: "Installed Mods", icon: Download },
		{ name: "Search", icon: Search },
		{ name: "All Mods", icon: LayoutDashboard },
		{ name: "Content", icon: FolderHeart },
		{ name: "Miscellaneous", icon: BookOpen },
		{ name: "Joker", icon: Flame },
		{ name: "Quality of Life", icon: Star },
		{ name: "Technical", icon: Spade },
		{ name: "Resource Packs", icon: FolderHeart },
		{ name: "API", icon: Gamepad2 },
	];

	const colorPairs = [
		{ color1: "#4f6367", color2: "#425556" },
		{ color1: "#AA778D", color2: "#906577" },
		{ color1: "#A2615E", color2: "#89534F" },
		{ color1: "#A48447", color2: "#8B703C" },
		{ color1: "#4F7869", color2: "#436659" },
		{ color1: "#728DBF", color2: "#6177A3" },
		{ color1: "#5D5E8F", color2: "#4F4F78" },
		{ color1: "#796E9E", color2: "#655D86" },
		{ color1: "#64825D", color2: "#556E4E" },
		{ color1: "#86A367", color2: "#728A57" },
		{ color1: "#748C8A", color2: "#627775" },
	];

	const categoryMap: Record<string, Category> = {
		Content: Category.Content,
		content: Category.Content,
		Joker: Category.Joker,
		joker: Category.Joker,
		"Quality of Life": Category.QualityOfLife,
		"quality of life": Category.QualityOfLife,
		Technical: Category.Technical,
		technical: Category.Technical,
		Miscellaneous: Category.Miscellaneous,
		miscellaneous: Category.Miscellaneous,
		"Resource Packs": Category.ResourcePacks,
		"resource packs": Category.ResourcePacks,
		Resources: Category.ResourcePacks,
		resources: Category.ResourcePacks,
		API: Category.API,
		api: Category.API,
	};

	function getRandomColorPair() {
		return colorPairs[Math.floor(Math.random() * colorPairs.length)];
	}

	function handleModClick(mod: Mod) {
		currentModView.set(mod);
	}

	let showSearch: boolean = false;
	$currentCategory = "All Mods";

	$: showSearch = $currentCategory === "Search";

	$: filteredMods = $modsStore.filter((mod) => {
		switch ($currentCategory) {
			case "Content":
				return (
					mod.categories.includes(Category.Content) ||
					mod.categories.some((cat) => cat === 0) || // Assuming Content is enum value 0
					mod.title.toLowerCase().includes("content") ||
					(typeof mod.description === "string" &&
						mod.description.toLowerCase().includes("new content"))
				);
			case "Joker":
				return (
					mod.categories.includes(Category.Joker) ||
					mod.categories.some((cat) => cat === 1)
				);
			case "Quality of Life":
				return (
					mod.categories.includes(Category.QualityOfLife) ||
					mod.categories.some((cat) => cat === 2)
				);
			case "Technical":
				return (
					mod.categories.includes(Category.Technical) ||
					mod.categories.some((cat) => cat === 3)
				);
			case "Resource Packs":
				return (
					mod.categories.includes(Category.ResourcePacks) ||
					mod.categories.some((cat) => cat === 5)
				);
			case "API":
				return (
					mod.categories.includes(Category.API) ||
					mod.categories.some((cat) => cat === 6)
				);
			case "Miscellaneous":
				return (
					mod.categories.includes(Category.Miscellaneous) ||
					mod.categories.some((cat) => cat === 4)
				);
			case "Installed Mods":
				return Boolean($installationStatus[mod.title]);
			default:
				return true;
		}
	});

	function handleCategoryClick(category: string) {
		currentPage.set(1);
		startPage = 1; // Reset sliding window
		currentCategory.set(category);
	}

	document.addEventListener("click", (e) => {
		const target = e.target as HTMLElement;
		const anchor = target.closest("a");

		if (anchor && anchor.href.startsWith("https://") && anchor.href) {
			e.preventDefault();
			open(anchor.href);
		}
	});

	function sortMods(mods: Mod[], sortOption: SortOption): Mod[] {
		return [...mods].sort((a, b) => {
			switch (sortOption) {
				case SortOption.NameAsc:
					return a.title.localeCompare(b.title);
				case SortOption.NameDesc:
					return b.title.localeCompare(a.title);
				// case SortOption.LastUpdatedAsc:
				// 	return a.lastUpdated.localeCompare(b.lastUpdated);
				// case SortOption.LastUpdatedDesc:
				// 	return b.lastUpdated.localeCompare(a.lastUpdated);
				default:
					return 0;
			}
		});
	}

	// Add sort handler
	function handleSortChange(event: Event) {
		const select = event.target as HTMLSelectElement;
		currentSort.set(select.value as SortOption);
	}
	$: sortedAndFilteredMods = sortMods(filteredMods, $currentSort);

	$: totalPages = Math.ceil(sortedAndFilteredMods.length / $itemsPerPage);
	$: paginatedMods = sortedAndFilteredMods.slice(
		($currentPage - 1) * $itemsPerPage,
		$currentPage * $itemsPerPage,
	);

	const maxVisiblePages = 5;
	let startPage = 1;

	function updatePaginationWindow() {
		if ($currentPage > startPage + maxVisiblePages - 1) {
			startPage = $currentPage - maxVisiblePages + 1;
		} else if ($currentPage < startPage) {
			startPage = $currentPage;
		}
	}

	async function refreshInstalledMods() {
		try {
			await forceRefreshCache();
			installedMods = await fetchCachedMods();

			// Update installation status for all mods in the store
			for (const mod of $modsStore) {
				const installedMod = installedMods.find(
					(m) => m.name === mod.title,
				);

				// Check if the mod is installed
				const isInstalled = installedMod !== undefined;

				// Update installation status
				installationStatus.update((s) => ({
					...s,
					[mod.title]: isInstalled,
				}));

				// Only check for updates if the mod is installed
				if (isInstalled && installedMod) {
					try {
						// Check for updates using the invoke command
						const hasUpdate = await invoke<boolean>(
							"mod_update_available",
							{
								modName: mod.title,
							},
						);

						// Update the global store
						updateAvailableStore.update((s) => ({
							...s,
							[mod.title]: hasUpdate,
						}));

						// console.log(
						// 	`Update check for ${mod.title}: ${hasUpdate ? "Update available" : "No update"}`,
						// );
					} catch (error) {
						console.error(
							`Failed to check updates for ${mod.title}:`,
							error,
						);
					}
				}
			}
		} catch (error) {
			console.error("Failed to refresh installed mods:", error);
		}
	}

	async function openModsFolder() {
		try {
			// Get the repository path (which should be config_dir/Balatro/mod_index)
			const modsFolderPath: string = await invoke("get_mods_folder");

			// Get the parent directory (config_dir/Balatro) by finding the last path separator
			const lastSeparatorIndex = Math.max(
				modsFolderPath.lastIndexOf("/"),
				modsFolderPath.lastIndexOf("\\"),
			);
			if (lastSeparatorIndex === -1) {
				addMessage(
					"Failed to determine the parent directory of the repository path.",
					"error",
				);
				return;
			}

			const parentPath = modsFolderPath.substring(0, lastSeparatorIndex);
			const separator = modsFolderPath.includes("/") ? "/" : "\\"; // Determine the separator used in the path

			// Construct the mods path
			const modsPath = `${parentPath}${separator}Mods`;

			// Check if the path exists
			const pathExists = await invoke("path_exists", { path: modsPath });

			if (!pathExists) {
				addMessage(
					"Mods directory not found. It might not have been created yet.",
					"warning",
				);
				addMessage(
					"Install a mod using the mod manager to create the mods directory.",
					"info",
				);
				return;
			}

			// Open the directory
			await invoke("open_directory", { path: modsPath });
		} catch (error) {
			addMessage(`Failed to open mods directory: ${error}`, "error");
		}
	}

	$: if ($currentModView === null && $currentCategory === "Installed Mods") {
		refreshInstalledMods();
	}
</script>

<div class="container default-scrollbar">
	<div class="mods-container">
		<div class="categories">
			{#each categories as category}
				<button
					class:active={$currentCategory === category.name}
					onclick={() => handleCategoryClick(category.name)}
				>
					<svelte:component this={category.icon} size={16} />
					{category.name}
				</button>
			{/each}
		</div>

		<div class="separator"></div>

		{#if isLoading}
			<div class="loading-container">
				<p class="loading-text">
					Loading mods{".".repeat($loadingDots)}
				</p>
			</div>
		{:else if showSearch}
			<SearchView onCheckDependencies={handleDependencyCheck} />
		{:else}
			<div class="mods-wrapper">
				<div class="controls-container">
					<div
						class="pagination-controls"
						in:fly={{ duration: 400, y: 10, opacity: 0.2 }}
					>
						<button
							onclick={previousPage}
							disabled={$currentPage === 1}
						>
							Previous
						</button>

						{#each Array(Math.min(maxVisiblePages, totalPages)) as _, i}
							{#if startPage + i <= totalPages}
								<button
									class:active={$currentPage ===
										startPage + i}
									onclick={() => goToPage(startPage + i)}
								>
									{startPage + i}
								</button>
							{/if}
						{/each}
						<button
							onclick={nextPage}
							disabled={$currentPage === totalPages}
						>
							Next
						</button>
					</div>
					<div
						class="sort-controls"
						in:fly={{ duration: 400, y: 10, opacity: 0.2 }}
					>
						<div class="sort-wrapper">
							<ArrowUpDown size={16} />
							<select
								value={$currentSort}
								onchange={handleSortChange}
							>
								<option value={SortOption.NameAsc}
									>Name (A-Z)</option
								>
								<option value={SortOption.NameDesc}
									>Name (Z-A)</option
								>
								<!-- <option value={SortOption.LastUpdatedDesc} -->
								<!-- 	>Latest Updated</option -->
								<!-- > -->
								<!-- <option value={SortOption.LastUpdatedAsc} -->
								<!-- 	>Oldest Updated</option -->
								<!-- > -->
							</select>
						</div>
					</div>
				</div>

				<div class="mods-scroll-container default-scrollbar">
					{#if $currentCategory === "Installed Mods"}
						{#if isLoadingLocalMods}
							<div class="section-header">
								<h3>Local Mods</h3>
								<p>
									Loading local mods{".".repeat($loadingDots)}
								</p>
							</div>
						{:else if localMods.length > 0}
							<div class="section-header">
								<div class="section-header-content">
									<h3>Local Mods</h3>
									<p>
										These mods were installed manually
										(outside the mod manager)
									</p>
								</div>
								<button
									class="open-folder-button"
									onclick={openModsFolder}
									title="Open mods folder"
								>
									<Folder size={20} />
									Open Mods Folder
								</button>
							</div>

							<div class="mods-grid local-mods-grid">
								{#each localMods as mod}
									<LocalModCard
										{mod}
										onUninstall={handleModUninstalled}
									/>
								{/each}
							</div>

							<div class="section-header">
								<h3>Mod Manager Catalog</h3>
								<p>
									These mods are available from the online
									catalog
								</p>
							</div>
						{:else if !isLoadingLocalMods && localMods.length === 0 && paginatedMods.length === 0}
							<div class="no-mods-message">
								<p>No installed mods.</p>
								<button
									class="open-folder-button"
									onclick={openModsFolder}
									title="Open mods folder"
								>
									<Folder size={20} />
									Open Mods Folder
								</button>
							</div>
						{/if}
					{/if}

					<div
						class="mods-grid"
						class:has-local-mods={$currentCategory ===
							"Installed Mods" && localMods.length > 0}
					>
						{#each paginatedMods as mod}
							<ModCard
								{mod}
								onmodclick={handleModClick}
								oninstallclick={installMod}
								onuninstallclick={uninstallMod}
							/>
						{/each}
					</div>
				</div>
			</div>
		{/if}
	</div>

	{#if $currentModView}
		<ModView
			mod={$currentModView!}
			onCheckDependencies={handleDependencyCheck}
		/>
	{/if}
</div>

<style>
	.open-folder-button {
		background: #4caf50;
		color: #f4eee0;
		border: none;
		outline: #3d8b40 solid 2px;
		border-radius: 4px;
		padding: 0.75rem 1.5rem;
		font-family: "M6X11", sans-serif;
		font-size: 1.2rem;
		cursor: pointer;
		transition: all 0.2s ease;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.open-folder-button:hover {
		background: #45a049;
		transform: translateY(-2px);
	}

	.open-folder-button:active {
		transform: translateY(1px);
	}

	.section-header {
		background: #c14139;
		border: 2px solid #f4eee0;
		border-radius: 8px;
		padding: 1rem 2rem;
		margin: 0 2rem 1rem 2rem;
		margin-top: 2rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.section-header-content {
		flex: 1;
	}

	.section-header h3 {
		margin: 0;
		font-size: 1.8rem;
		color: #f4eee0;
		text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
	}

	.section-header p {
		margin: 0.5rem 0 0 0;
		font-size: 1.1rem;
		color: #f4eee0;
	}
	.mods-container {
		display: flex;
		gap: 1rem;
		padding: 0 2rem;
		overflow: hidden;

		height: 100%;
	}

	.no-mods-message {
		display: flex;
		justify-content: center;
		flex-direction: column;
		align-items: center;
		height: 100%;
		width: 100%;
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		margin: auto;
		padding-top: 3rem; /* Add space for the controls at the top */
	}

	.no-mods-message p {
		font-family: "M6X11", sans-serif;
		font-size: 1.8rem;
		color: #f4eee0;
		text-align: center;
		/* Add black stroke with two methods for better browser compatibility */
		-webkit-text-stroke: 0.1px black;
		/* Fallback using text-shadow for browsers that don't support text-stroke */
		text-shadow:
			-1px -1px 0 #000,
			1px -1px 0 #000,
			-1px 1px 0 #000,
			1px 1px 0 #000,
			2px 2px 3px rgba(0, 0, 0, 0.5);
	}

	.separator {
		width: 2px;
		background: #f4eee0;
		height: 100%;
	}

	.pagination-controls {
		position: absolute;
		/* top: 0.05rem; */
		left: 50%;
		transform: translateX(-50%);
		z-index: 1000;
		background: #c14139;
		border: 2px solid #f4eee0;
		border-radius: 8px;
		padding: 0.5rem 1rem;
		display: flex;
		gap: 0.5rem;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.pagination-controls button {
		padding: 0.5rem 1rem;
		background: #ea9600;
		border: 2px solid #f4eee0;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 0.8rem;
		cursor: pointer;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.pagination-controls button:hover:not(:disabled) {
		background: #f4eee0;
		color: #393646;
	}

	.pagination-controls button.active {
		background: #f4eee0;
		color: #393646;
	}

	.pagination-controls button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.controls-container {
		height: 75px;
		width: 100%;
		display: flex;
		position: absolute;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.categories {
		width: 190px;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		overflow-y: auto;
		padding: 2rem 0;

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

		scrollbar-width: 0;
		scrollbar-color: transparent transparent;
	}

	.categories button {
		text-align: left;
		padding: 1rem 1rem;
		background: #ea9600;
		border: 2px solid #f4eee0;
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 1.1rem;
		cursor: pointer;
		transition: all 0.2s ease;
		border-radius: 6px;
		margin-right: 0.3rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.categories button:hover {
		background: #f4eee0;
		color: #393646;
	}

	.categories button.active {
		background: #f4eee0;
		color: #393646;
	}

	.mods-scroll-container {
		overflow-y: auto;
		height: 100%;
	}

	.mods-grid {
		padding: 1rem 2rem 2rem 2rem;
		flex: 1;
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 30px;
	}

	.local-mods-grid {
		padding-top: 0.5rem;
		padding-bottom: 1rem;
	}

	/* Catalog section gets proper top padding */
	.local-mods-grid + .section-header + .mods-grid {
		padding-top: 1rem;
	}

	.sort-controls {
		position: absolute;
		/* top: 0.25rem; Increased from 2rem */
		right: 4rem; /* Increased from 2.5rem */
		z-index: 1000;
		margin: 0;
		background: transparent;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
		/* transform: translateY(0); /* Reset any transforms */
	}
	/**/
	/* 	.sort-controls { */
	/*     position: absolute; */
	/*     top: 1rem; */
	/*     right: 3rem; */
	/*     z-index: 1000; */
	/*     margin: 0; */
	/*     background: transparent; */
	/* } */

	.sort-wrapper {
		background: #ea9600;
		border: 2px solid #f4eee0;
		padding: 0.5rem;
		border-radius: 6px;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		transition: all 0.2s ease;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
	}

	.mods-wrapper {
		position: relative;
		/* 192px being the width of the catagories + seperator */
		width: calc(100% - 192px);
		padding: 0 1rem;
	}

	.sort-wrapper :global(svg) {
		color: #f4eee0;
	}

	select {
		background: #ea9600;
		color: #f4eee0;
		border: none;
		font-family: "M6X11", sans-serif;
		font-size: 1rem;
		padding: 0.25rem 1.5rem 0.25rem 0.5rem;
		border-radius: 4px;
		cursor: pointer;
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
		background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23F4EEE0%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.4-12.8z%22%2F%3E%3C%2Fsvg%3E");
		background-repeat: no-repeat;
		background-position: right 0.5em top 50%;
		background-size: 0.65em auto;
	}

	select:hover {
		background-color: #f0a620;
	}

	select:focus {
		outline: none;
		box-shadow: 0 0 0 2px #f4eee0;
	}

	select option {
		background: #ea9600;
		color: #f4eee0;
		padding: 0.5rem;
	}

	.sort-wrapper:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	.loading-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		flex: 1;
	}

	.loading-text {
		color: #f4eee0;
		font-family: "M6X11", sans-serif;
		font-size: 1.5rem;
		min-width: 150px;
	}

	@media (max-width: 1160px) {
		.pagination-controls button {
			min-width: 3rem;
			padding: 0.4rem 0.6rem;
			font-size: 0.75rem;
		}

		.pagination-controls {
			left: 13.6rem;
		}

		.controls-container {
			margin-bottom: 0.5rem;
		}

		.sort-controls {
			right: 1rem;
		}
	}
</style>
