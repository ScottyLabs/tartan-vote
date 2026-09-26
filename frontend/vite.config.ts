import adapter from "@sveltejs/adapter-static";
import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => ({
  plugins: [
    tailwindcss(),
    sveltekit({
      compilerOptions: {
        // Force runes mode for the project, except for libraries. Can be removed in svelte 6.
        runes: ({ filename }: { filename: string }) =>
          filename.split(/[/\\]/u).includes("node_modules") ? undefined : true,
      },

      adapter: adapter({ pages: "dist", assets: "dist", fallback: "index.html" }),
    }),
  ],
  build: {
    rolldownOptions: {
      watch: {
        exclude: ["**/.svelte-kit/**", "**/dist/**", "**/*.timestamp-*.mjs"],
      },
    },
  },
  // Bundle SSR deps so Deno's postbuild analyse worker does not need every
  // transitive package listed as a direct dependency of this project.
  ssr: {
    noExternal: true,
  },
  server: {
    proxy: {
      // Empty prefix loads unprefixed vars too, so PORT from devenv is visible.
      "/api": `http://localhost:${loadEnv(mode, ".", "").PORT ?? "8080"}`,
    },
  },
}));
