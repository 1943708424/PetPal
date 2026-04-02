/// <reference types="node" />
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [sveltekit()],

  // Vitest 下解析 Svelte 的 browser 条件，避免 `mount` 走到 server 入口
  resolve: process.env.VITEST
    ? { conditions: ["browser"] }
    : undefined,

  test: {
    environment: "jsdom",
    include: ["src/**/*.{test,spec}.ts"],
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
