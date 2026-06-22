import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { resolve } from "node:path";

// Screenshot-only build: aliases the Tauri IPC layer to a static mock so the
// React UI renders fully populated in a plain browser. Not used by Tauri.
export default defineConfig({
  root: resolve(__dirname, ".."),
  plugins: [react()],
  resolve: {
    alias: {
      "@tauri-apps/api/core": resolve(__dirname, "mock-tauri-core.ts"),
    },
  },
  build: {
    target: "safari13",
    outDir: resolve(__dirname, "../dist-screenshot"),
    emptyOutDir: true,
  },
});
