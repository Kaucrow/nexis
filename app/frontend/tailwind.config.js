/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  prefix: "tw-",
  important: true,
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        primary: {
          background: '#121b21',
          dark: '#121b21',
          secondary: '#387478',
          light: '#E2F1E7'
          
        },
        accent: '#629584'

      }
    }
  },
  plugins: []
};