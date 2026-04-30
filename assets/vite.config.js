import { defineConfig } from "vite"
import tailwindcss from "@tailwindcss/vite"
import { resolve } from "path"

const buildPath = process.env.MIX_BUILD_PATH ?? resolve(__dirname, "../_build/dev")

export default defineConfig({
  plugins: [tailwindcss()],
  resolve: {
    alias: {
      "phoenix-colocated/qitto": resolve(buildPath, "phoenix-colocated/qitto/index.js"),
    },
  },
  build: {
    outDir: resolve(__dirname, "../priv/static/assets"),
    emptyOutDir: false,
    rollupOptions: {
      input: {
        app: resolve(__dirname, "js/app.js"),
      },
      output: {
        entryFileNames: "js/[name].js",
        chunkFileNames: "js/[name]-[hash].js",
        assetFileNames: ({ name }) => {
          if (name?.endsWith(".css")) return "css/[name][extname]"
          return "assets/[name][extname]"
        },
      },
    },
  },
})
