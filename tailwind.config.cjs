const forms = require('@tailwindcss/forms');
const typography = require('@tailwindcss/typography');
const lineClamp = require('@tailwindcss/line-clamp');

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        lt: {
          base: '#f9f9f9',
          alte: '#f1f1f1',
          text: '#353535',
          title: '#13192d',
        },
        dk: {
          base: '#0c0c0f',
          alte: '#17171b',
          text: '#9aa0b0',
          title: '#e0e1eb',
        },
      },
      fontFamily: {
        body: ['Literata', 'serif'],
        mono: ['IBM Plex Mono', 'monospace'],
      },
    },
  },
  plugins: [forms, typography, lineClamp],
};
