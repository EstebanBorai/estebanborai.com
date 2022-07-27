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
          background: '#222222',
          'background-alt': '#2C2C2E',
        },
      },
      fontFamily: {
        body: ['Inter', 'sans-serif'],
      },
    },
  },
  plugins: [forms, typography],
};
