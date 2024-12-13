/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.rs'],
  theme: {
    extend: {
      fontFamily: {
        body: ['Literata', 'serif'],
        mono: ['IBM Plex Mono', 'monospace'],
      },
    }
  },
  plugins: []
};
