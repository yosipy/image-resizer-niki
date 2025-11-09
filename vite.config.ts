import { defineConfig } from "vite"
import wasm from "vite-plugin-wasm"
import topLevelAwait from "vite-plugin-top-level-await"

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [wasm(), topLevelAwait()],
    resolve: {
      alias: [{ find: "@lib", replacement: "/lib" }],
    },
    build: {
      outDir: "dist",
      lib: {
        entry: "lib/index.ts",
        name: "image-resizer-niki",
        fileName: "index",
        formats: ["es", "umd"],
      },
    },
  }
})
