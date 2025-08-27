<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from "svelte";

  export let src: string;
  export let alt: string = "";
  export let fallbackSrc: string | undefined;
  export let defaultSrc: string = "/images/cover.jpg";
  export let className: string = "";

  // Emit load/error for parent if needed
  const dispatch = createEventDispatcher();

  let wrapper: HTMLDivElement | null = null;
  let observer: IntersectionObserver | null = null;
  let currentSrc: string | null = null;
  let triedFallback = false;
  let visible = false;
  let loading = true;
  let usingDefault = false; // show static cover when thumbnail is missing

  function ensureObserver() {
    if (!observer) {
      observer = new IntersectionObserver((entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            visible = true;
            actuallyLoad();
            observer?.disconnect();
            observer = null;
            break;
          }
        }
      }, { rootMargin: "200px" });
      if (wrapper) observer.observe(wrapper);
    }
  }

  function actuallyLoad() {
    if (!visible) return;
    if (!currentSrc) {
      currentSrc = src;
    }
  }

  function handleLoad() {
    loading = false;
    dispatch("load");
  }

  function handleError() {
    if (!triedFallback && fallbackSrc && currentSrc !== fallbackSrc) {
      triedFallback = true;
      usingDefault = false;
      currentSrc = fallbackSrc;
      // keep loading true until fallback resolves
      loading = true;
    } else {
      // Switch to static default cover and hide the spinner
      usingDefault = true;
      currentSrc = null;
      loading = false;
      dispatch("error");
    }
  }

  onMount(() => {
    // Use IntersectionObserver to defer loading until visible
    ensureObserver();
  });

  onDestroy(() => {
    observer?.disconnect();
  });

  // Reset when src changes so pagination or prop updates reload correct image
  $: if (src) {
    if (currentSrc !== null && currentSrc !== src) {
      triedFallback = false;
      usingDefault = false;
      loading = true;
      if (visible) {
        currentSrc = src;
      } else {
        currentSrc = null;
        ensureObserver();
      }
    }
  }
</script>

<div class={`lazy-image ${className}`} bind:this={wrapper}>
  {#if usingDefault}
    <div class="default-cover" aria-hidden="true"></div>
  {:else}
    {#if currentSrc}
      {#key currentSrc}
        <img src={currentSrc} alt={alt} on:load={handleLoad} on:error={handleError} draggable="false" decoding="async" />
      {/key}
    {/if}
    {#if loading && !(usingDefault || (currentSrc && currentSrc === defaultSrc))}
      <div class="spinner-square" aria-hidden="true"></div>
    {/if}
  {/if}
</div>

<style>
  .lazy-image {
    position: relative;
    width: 100%;
    height: 100%;
    border-radius: 5px;
    overflow: hidden;
  }

  .lazy-image img {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 5px;
  }

  .default-cover {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    background: url('/images/cover.jpg') center/cover no-repeat;
    border-radius: 5px;
  }

  /* Square spinning throbber */
  .spinner-square {
    position: absolute;
    top: 50%;
    left: 50%;
    width: 28px;
    height: 28px;
    margin: -14px 0 0 -14px;
    border-radius: 4px;
    background: #fdcf51;
    box-shadow: 0 0 0 2px #f4eee0 inset;
    animation: square-spin 1s linear infinite;
  }

  @keyframes square-spin {
    0% { transform: translateZ(0) rotate(0deg); opacity: 0.8; }
    50% { transform: translateZ(0) rotate(180deg); opacity: 1; }
    100% { transform: translateZ(0) rotate(360deg); opacity: 0.8; }
  }
</style>
