import preprocess from 'svelte-preprocess';
import { mdsvex } from 'mdsvex';
import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
  build: {
    sourcemap: true,
  },
  // an array of file extensions that should be treated as Svelte components
  extensions: ['.svelte', '.svx'],
  assetsInclude: ['**/*.svx'],
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess({
      postcss: true,
    }),
    mdsvex({
      layout: './src/lib/modules/notes/components/Layout.svelte',
    }),
  ],
  plugins: [sveltekit()],
};

export default config;
