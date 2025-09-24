import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import wasmPackWatchPlugin from "vite-plugin-wasm-pack-watcher";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  build: {
    watch: {
      include: ["src/**/*.ts", "src/**/*.rs"],
    },
  },
  plugins: [wasmPackWatchPlugin(), wasm(), topLevelAwait(), react(), tailwindcss()],
});
