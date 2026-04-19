import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  // Reads .env files from the project root (monorepo-style),
  // matching the original frontend's behavior.
  envDir: '..',
  plugins: [sveltekit()],
  server: {
    port: 5173
  }
});
