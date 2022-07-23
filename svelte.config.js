import adapter from '@sveltejs/adapter-cloudflare';
import preprocess from 'svelte-preprocess';
import { mdsvex } from 'mdsvex';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // an array of file extensions that should be treated as Svelte components
  extensions: ['.svelte', '.svx'],

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

  kit: {
    adapter: adapter(),
  },
};

export default config;
