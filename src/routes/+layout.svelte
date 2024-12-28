<script lang="ts">
  import { blur } from "svelte/transition";
  export let data;
</script>

<div class="layout-container">
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
  }

  .page-content {
    width: 100%;
    height: 100%;
  }
</style>

