const config = {
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
  // Only add this if you installed the TailwindCSS-plugins:
  plugins: [require('@tailwindcss/typography'), require('@tailwindcss/forms')],
};

module.exports = config;
