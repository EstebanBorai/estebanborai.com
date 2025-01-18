/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.rs'],
  theme: {
    extend: {
      fontFamily: {
        body: ['Inter', 'serif'],
        mono: ['Fira Code', 'monospace'],
      },
    }
  },
  plugins: []
};
