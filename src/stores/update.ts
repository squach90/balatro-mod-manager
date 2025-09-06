import { writable } from "svelte/store";

function createPersistentBoolean(key: string, fallback: boolean) {
  const isBrowser = typeof window !== "undefined";
  let initial = fallback;
  if (isBrowser) {
    try {
      const raw = localStorage.getItem(key);
      if (raw != null) initial = raw === "true";
    } catch (_) {
      // ignore
    }
  }
  const store = writable<boolean>(initial);
  if (isBrowser) {
    store.subscribe((val) => {
      try {
        localStorage.setItem(key, val ? "true" : "false");
      } catch (_) {
        // ignore
      }
    });
  }
  return store;
}

// If true, never show the update-available popup
export const updatePromptDisabled = createPersistentBoolean(
  "ui.updatePromptDisabled",
  false,
);
