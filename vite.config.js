import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
  build: {
    sourcemap: true,
  },
  server: {
    port: 3000,
  },
  plugins: [sveltekit()],
};

export default config;
