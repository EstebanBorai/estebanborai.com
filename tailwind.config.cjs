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
          background: '#FFFFFF',
          'background-alt': '#F6F8FA',
        },
        dark: {
          background: '#1C1C1C',
          'background-alt': '#1A1A1A',
        },
      },
      fontFamily: {
        body: ['DM Sans', 'sans'],
        display: ['Abril Fatface', 'serif'],
        mono: ['IBM Plex Mono', 'monospace'],
      },
    },
  },
  plugins: [forms, typography, lineClamp],
};
