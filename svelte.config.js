import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import adapter from '@sveltejs/adapter-cloudflare';
import autoprefixer from 'autoprefixer';
import { mdsvex } from 'mdsvex';
import preprocess from 'svelte-preprocess';
import tailwindcss from 'tailwindcss';

/** @type {import('@sveltejs/kit').Config} */
export default {
  extensions: ['.svelte', '.svx'],
  preprocess: [
    preprocess({
      sourceMap: true,
      postcss: {
        plugins: [
          tailwindcss({
            darkMode: 'class',
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
            variants: {
              extend: {},
            },
          }),
          autoprefixer,
        ],
      },
    }),
    mdsvex({
      layout: './src/lib/modules/notes/components/Layout.svelte',
    }),
  ],
  kit: {
    adapter: adapter(),
  },
};
