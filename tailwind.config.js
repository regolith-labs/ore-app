/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "./dist/**/*.html",
  ],
  darkMode: 'selector',
  plugins: [],
  theme: {
    colors: ({ colors }) => ({
      ...colors,
      transparent: colors.transparent,
      white: '#FAFAFA',
      black: '#0F0E11',
      green: {
        400: '#5EDCA7',
        500: '#0acf86',
        // 500: '#03C167',
        600: '#00b87c',
        700: '#00a36e',
      },
      gray: {
        100: '#e8e8ea',
        200: '#d6d6d8',
        300: '#b0b0b2',
        700: '#707070',
        800: '#27252B',
        900: '#151419',
      },
    }),
    extend: {
      fontFamily: {
        mono: ['ui-monospace', 'monospace'],
        sans: ['Pilat', 'ui-sans-serif', 'system-ui'],
        serif: ['ui-serif'],
        wide: ['PilatWide', 'ui-sans-serif', 'system-ui']
      },
    },
  },
}
