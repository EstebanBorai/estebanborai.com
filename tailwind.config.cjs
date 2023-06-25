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
        head: ['Wix Madefor Display', 'sans'],
        body: ['Wix Madefor Text', 'sans'],
        mono: ['Fira Code', 'monospace'],
      },
    },
  },
  plugins: [forms, typography],
};
