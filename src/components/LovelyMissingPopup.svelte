<script lang="ts">
  import { fade, scale } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";
  import { addMessage } from "$lib/stores";
  import { lovelyPopupStore } from "../stores/modStore";

  let isWindows = $state<boolean>(false);
  let balatroPath = $state<string | null>(null);

  async function init() {
    // Simple OS check for showing Windows instructions
    const ua = navigator.userAgent.toLowerCase();
    isWindows = ua.includes("windows");

    try {
      const path = await invoke<string | null>("get_balatro_path");
      balatroPath = path ?? null;
    } catch (e) {
      balatroPath = null;
    }
  }

  $effect(() => {
    if ($lovelyPopupStore.visible) {
      init();
    }
  });

  async function copyPath() {
    if (!balatroPath) {
      addMessage("Game path not set; open Settings and set it first.", "warning");
      return;
    }
    try {
      await navigator.clipboard.writeText(balatroPath);
      addMessage("Copied to clipboard", "success");
    } catch (e) {
      addMessage("Failed to copy path", "error");
    }
  }

  function close() {
    lovelyPopupStore.update((s) => ({ ...s, visible: false }));
  }
</script>

{#if $lovelyPopupStore.visible}
  <div class="modal-background" transition:fade={{ duration: 100 }}>
    <div class="modal" transition:scale={{ duration: 200, start: 0.95, opacity: 1 }}>
      <h2>Lovely Not Detected</h2>
      <p>
        The Lovely injector was not detected. Mods require Lovely to load.
      </p>

      {#if isWindows}
        <div class="steps">
          <p>Windows Defender may remove the injector DLL. To prevent this:</p>
          <ol>
            <li>Open Windows Security → Virus & threat protection → Manage settings.</li>
            <li>Temporarily disable Real-time protection (Windows re-enables it automatically later).</li>
            <li>Scroll to Add or remove exclusions and confirm if prompted.</li>
            <li>Add a folder exclusion for the Balatro directory.</li>
          </ol>
          <div class="copy-row">
            <button class="copy-button" onclick={copyPath}>
              Copy Balatro Path
            </button>
            {#if balatroPath}
              <span class="path">{balatroPath}</span>
            {/if}
          </div>
        </div>
      {/if}

      <div class="buttons">
        <button class="close-button" onclick={close}>Close</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 999;
  }
  .modal {
    background: #2d2d2d;
    outline: 2px solid #fdcf51;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 0 15px rgba(0, 0, 0, 0.7);
    max-width: 560px;
    width: 92%;
    text-align: left;
  }
  h2 {
    color: #fdcf51;
    margin-bottom: 0.75rem;
    font-family: "M6X11", sans-serif;
  }
  p, li, .path {
    color: #f4eee0;
    font-size: 1.05rem;
    font-family: "M6X11", sans-serif;
  }
  ol {
    padding-left: 1.2rem;
    margin: 0.5rem 0 1rem 0;
  }
  .copy-row {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    margin-top: 0.5rem;
    flex-wrap: wrap;
  }
  .copy-button {
    background: #4f5a9c;
    outline: #3a4275 solid 2px;
    color: #f4eee0;
    padding: 0.6rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-family: "M6X11", sans-serif;
    font-size: 1rem;
    transition: all 0.2s ease;
  }
  .copy-button:hover { transform: translateY(-2px); }
  .buttons { display: flex; justify-content: flex-end; margin-top: 1rem; }
  .close-button {
    background: #56a786;
    outline: #74cca8 solid 2px;
    color: #fff;
    padding: 0.6rem 1.2rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-family: "M6X11", sans-serif;
  }
  @media (max-width: 1160px) {
    .modal { max-width: 90%; padding: 1.5rem; }
  }
</style>
