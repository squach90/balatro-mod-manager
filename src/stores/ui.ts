import { writable } from "svelte/store";

function createPersistentNumber(
  key: string,
  fallback: number,
  opts?: { min?: number; max?: number },
) {
  const isBrowser = typeof window !== "undefined";
  let initial = fallback;
  if (isBrowser) {
    try {
      const raw = localStorage.getItem(key);
      if (raw != null) {
        const n = Number(raw);
        if (!Number.isNaN(n)) initial = n;
      }
    } catch (_) {
      // ignore storage read errors
    }
  }
  // Clamp to optional bounds
  if (typeof opts?.min === "number") initial = Math.max(opts.min, initial);
  if (typeof opts?.max === "number") initial = Math.min(opts.max, initial);

  const store = writable<number>(initial);
  if (isBrowser) {
    store.subscribe((val) => {
      try {
        let v = val;
        if (typeof opts?.min === "number") v = Math.max(opts.min, v);
        if (typeof opts?.max === "number") v = Math.min(opts.max, v);
        localStorage.setItem(key, String(v));
      } catch (_) {
        // ignore storage write errors
      }
    });
  }
  return store;
}

// Controls how large mod cards render in the grid/search views
// Range: 0.75x – 1.4x
export const cardScale = createPersistentNumber("ui.cardScale", 1, {
  min: 0.75,
  max: 1.4,
});
