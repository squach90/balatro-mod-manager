<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import MessageStack from "./MessageStack.svelte";

  let messageStack: MessageStack;
  let selectedPath = "";
  let placeholder = "Choose Balatro Path";

  const truncatePath = (path: string) => {
    const maxLength = 50;
    return path.length > maxLength
      ? path.substring(0, maxLength - 3) + "..."
      : path;
  };

  const getBalatroPath = async () => {
    const path = await invoke("get_balatro_path");
    if (path) {
      selectedPath = path as string;
      placeholder = path as string;
    } else {
      placeholder = "Choose Balatro Path";
    }
  };

  getBalatroPath();

  const handlePathSelect = async () => {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Balatro Path",
    });

    if (selected) {
      selectedPath = selected as string;
      const isValid = await invoke("check_custom_balatro", {
        path: selectedPath,
      });

      if (isValid) {
        messageStack.addMessage("Balatro path set successfully!", "success");
      } else {
        messageStack.addMessage(
          "Invalid Balatro path. Please select the correct directory.",
          "error"
        );
        selectedPath = "";
      }
    }
  };
</script>

<div class="path-selector">
  <div class="input-container">
    <input
      type="text"
      placeholder={truncatePath(placeholder)}
      value={selectedPath ? truncatePath(selectedPath) : ""}
      on:click={handlePathSelect}
      readonly
    />
  </div>
</div>

<MessageStack bind:this={messageStack} />

<style>
  .path-selector {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 1rem 0;
  }

  .input-container {
    width: 20rem;
  }

  input[type="text"] {
    width: 100%;
    padding: 0.75rem;
    border: 2px solid #fda200;
    border-radius: 8px;
    background-color: #c88000;
    font-family: "M6X11", sans-serif;
    color: white;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    -webkit-user-select: none;
    user-select: none;
  }

  input[type="text"]:hover {
    border-color: #f4eee0;
  }

  input[type="text"]::placeholder {
    color: white;
    -webkit-user-select: none;
    user-select: none;
  }
</style>

