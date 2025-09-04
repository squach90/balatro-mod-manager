import { writeFile, readFile, mkdir, exists } from "@tauri-apps/plugin-fs";
import { BaseDirectory } from "@tauri-apps/api/path";

// Use native fetch to reduce bundle size

const CACHE_DIR = "cache";

async function cacheImage(imageUrl: string): Promise<void> {
  try {
    const res = await fetch(imageUrl);
    if (!res.ok) throw new Error(`Failed to download image: ${res.status}`);
    const buf = await res.arrayBuffer();
    const imageName = imageUrl.substring(imageUrl.lastIndexOf("/") + 1);
    const imagePath = `${CACHE_DIR}/${imageName}`;

    await mkdir(CACHE_DIR, {
      recursive: true,
      baseDir: BaseDirectory.AppData,
    });

    await writeFile(imagePath, new Uint8Array(buf), {
      baseDir: BaseDirectory.AppData,
    });
  } catch (error) {
    console.error("Error caching image:", error);
  }
}

export async function displayCachedImage(imageUrl: string): Promise<string> {
  const imageName = imageUrl.substring(imageUrl.lastIndexOf("/") + 1);
  const imagePath = `${CACHE_DIR}/${imageName}`;

  const imageExists = await exists(imagePath);

  if (imageExists) {
    const imageData = await readFile(imagePath, {
      baseDir: BaseDirectory.AppData,
    });
    const base64 = btoa(String.fromCharCode(...imageData));
    return `data:image/jpg;base64,${base64}`;
  } else {
    await cacheImage(imageUrl);
    return imageUrl;
  }
}
