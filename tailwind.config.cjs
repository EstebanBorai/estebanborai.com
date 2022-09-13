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
          background: '#F6F5F1',
          'background-alt': '#FEFEFE',
        },
        dark: {
          background: '#0D0A12',
          'background-alt': '#16101D',
        },
      },
      fontFamily: {
        body: ['Spectral', 'serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
    },
  },
  plugins: [forms, typography],
};
