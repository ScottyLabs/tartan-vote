import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vite.dev/config/
export default defineConfig({
  envDir: "..",
  plugins: [svelte()],
  server: {
    proxy: {
      "/api": "http://localhost:8080",
      "/auth": "http://localhost:8080",
      "/events": "http://localhost:8080",
      "/health": "http://localhost:8080",
      "/session": "http://localhost:8080",
    },
  },
});
