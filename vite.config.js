import { sveltekit } from '@sveltejs/kit/vite';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import autoprefixer from 'autoprefixer';
import { mdsvex } from 'mdsvex';
import tailwindcss from 'tailwindcss';

/** @type {import('vite').UserConfig} */
const config = {
  build: {
    sourcemap: true,
  },
  css: {
    postcss: {
      plugins: [
        tailwindcss({
          darkMode: 'class',
          mode: 'jit',
          content: ['./src/**/*.{html,js,svelte,ts}'],
          theme: {
            extend: {
              colors: {
                light: {
                  background: '#F6F5F1',
                  'background-alt': '#FEFEFE',
                },
                dark: {
                  background: '#222222',
                  'background-alt': '#2C2C2E',
                },
              },
              fontFamily: {
                body: ['Inter', 'sans-serif'],
              },
            },
          },
          plugins: [typography, forms],
        }),
        autoprefixer,
      ],
    },
  },
  server: {
    port: 3000,
  },
  // an array of file extensions that should be treated as Svelte components
  extensions: ['.svelte', '.svx'],
  assetsInclude: ['**/*.css', '**/*.svx'],
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    mdsvex({
      layout: './src/lib/modules/notes/components/Layout.svelte',
    }),
  ],
  plugins: [sveltekit()],
};

export default config;
