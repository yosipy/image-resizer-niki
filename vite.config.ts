import { defineConfig } from "vite"

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  return {
    resolve: {
      alias: [{ find: "@lib", replacement: "/lib" }],
    },
    build: {
      outDir: "dist",
      lib: {
        entry: "lib/index.ts",
        name: "image-resize-niki",
        fileName: "index",
        formats: ["es", "umd"],
      },
    },
  }
})
