<script lang="ts">
// TODO: Add Transtition & animation to the alert box
// FIX: Steam won't get detected as running for some reason
import { invoke } from "@tauri-apps/api/core";

export let show = false;
export let onClose: () => void;

async function handleLaunch() {
    try {
        await invoke('launch_balatro');
        onClose();
    } catch (error) {
        console.error('Failed to launch game:', error);
    }
}

async function handleCheckAgain() {
    // Add Steam check logic here
    onClose();
}
</script>

{#if show}
<div class="overlay">
    <div class="alert-box">
        <h2>Steam is not running. Are you sure?</h2>
        
        <div class="button-container">
            <button 
                class="launch-button" 
                on:click={handleLaunch}
            >
                Yes, launch without Steam
            </button>
            
            <button 
                class="check-button" 
                on:click={handleCheckAgain}
            >
                Check again
            </button>
        </div>
    </div>
</div>
{/if}

<style>
.overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
}

.alert-box {
    background: #393646;
    border: 2px solid #f4eee0;
    border-radius: 8px;
    padding: 2rem;
    width: 400px;
    text-align: center;
}

.alert-box h2 {
    color: #f4eee0;
    margin-bottom: 2rem;
    font-family: "M6X11", sans-serif;
}

.button-container {
    display: flex;
    gap: 1rem;
    justify-content: center;
}

.launch-button {
    background: #c14139;
    border: 2px solid #a13029;
    color: #f4eee0;
    padding: 0.75rem 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-family: "M6X11", sans-serif;
    transition: all 0.2s ease;
}

.launch-button:hover {
    background: #d4524a;
    transform: translateY(-2px);
}

.check-button {
    background: #56a786;
    border: 2px solid #459373;
    color: #f4eee0;
    padding: 0.75rem 1.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-family: "M6X11", sans-serif;
    transition: all 0.2s ease;
}

.check-button:hover {
    background: #74cca8;
    transform: translateY(-2px);
}
</style>

