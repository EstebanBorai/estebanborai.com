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
        light: {
          background: '#EBDFD1',
          'background-alt': '#FDF1E2',
        },
        dark: {
          background: '#0D0A12',
          'background-alt': '#16101D',
        },
      },
      fontFamily: {
        body: ['Inter', 'sans'],
        mono: ['Fira Code', 'monospace'],
      },
    },
  },
  plugins: [forms, typography, lineClamp],
};
