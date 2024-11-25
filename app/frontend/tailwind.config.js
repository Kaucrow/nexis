/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme')

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  prefix: "tw-",
  important: true,
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        dark: {
          background: '#131a20',  // Background
          primary: '#e2f1e7',
          card: '#1A252E',     
          shade: {                // Dark colors
            light: '#21313b',
            dark: '#0d1418',
          },
          accent: {               // Greenish colors
            light: '#387478',
            dark: '#629584',
          },
          gray: {                 // Gray colors
            light: '#ababab',
            dark: '#7d7d7d',
          },
        }
      }
    },
    fontFamily: {
      'poppinsmedium': ['Poppins Medium', ...defaultTheme.fontFamily.sans],
      'poppinsregular': ['Poppins Regular', ...defaultTheme.fontFamily.sans],
      'poppinslight': ['Poppins Light', ...defaultTheme.fontFamily.sans],
      'poppinsthin': ['Poppins Thin', ...defaultTheme.fontFamily.sans]
    }
  },
  plugins: []
};