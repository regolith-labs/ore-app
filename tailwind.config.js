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
      black: '#1d1d1f',
      green: {
        // 500: '#00cc8a',
        // 500: '#15d180',
        500: '#0acf86',
        600: '#00b87c',
        700: '#00a36e',
      },
      gray: {
        100: '#e8e8ea',
        200: '#d6d6d8',
        300: '#b0b0b2',
        700: '#5a5a5c',
        800: '#464648',
        900: '#313133',
      },
    }),
    lineHeight: {
        'relaxed': '1.3',
    },
    extend: {
      fontFamily: {
        mono: ['GeistMono', 'ui-monospace', 'monospace'],
        sans: ['Geist', 'ui-sans-serif', 'system-ui'],
        serif: ['ui-serif'],
      },
      transitionProperty: {
        'height': 'height'
      },
    },
  },
}
