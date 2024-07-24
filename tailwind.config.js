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
      white: '#f5f5f7',
      black: '#131416',
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
        700: '#707071',
        800: '#464648',
        900: '#313133',
      },
    }),
    extend: {
      fontFamily: {
        mono: ['GeistMono', 'ui-monospace', 'monospace'],
        sans: ['Geist', 'ui-sans-serif', 'system-ui'],
        serif: ['ui-serif'],
        hero: ['RoobertTRIAL', 'ui-sans-serif', 'system-ui']
      },
      transitionProperty: {
        'height': 'height'
      },
    },
  },
}
