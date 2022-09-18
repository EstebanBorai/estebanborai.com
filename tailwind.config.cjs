const forms = require('@tailwindcss/forms');
const typography = require('@tailwindcss/typography');

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
        body: ['IBM Plex Sans', 'serif'],
        mono: ['IBM Plex Mono', 'monospace'],
      },
    },
  },
  plugins: [forms, typography],
};
