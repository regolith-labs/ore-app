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
      base: {
        bg: '#0F0E11',
        canvas: '#0C0C0E'
      },
      elements: {
        highEmphasis: '#FAFAFA',
        midEmphasis: '#D1D1D1',
        lowEmphasis: '#707070',
        gold: '#ECC771'
      },
      surface: {
        elevated: '#131216',
        elevatedHover: '#151419',
        floating: '#1A191F'
      },
      controls: {
        primary: '#F5F5F5',
        primaryHover: '#FAFAFA',
        secondary: '#26242D',
        secondaryHover: '#2B2932',
        tertiary: '#1D1C22',
        tertiaryHover: '#222027',
        danger: '#CC3B28',
        dangerHover: '#BC3625',
        disabled: '#18171C',
        handle: '#302E38'
      },
      on: {
        onColor: '#0D0D0D',
        onDisabled: '#585661',
        onHighContrast: '#0F0E11'
      },
      green: {
        400: '#5EDCA7',
        500: '#0acf86',
        // 500: '#03C167',
        600: '#00b87c',
        700: '#00a36e',
      },
      gray: {
        100: '#e8e8ea',
        200: '#D1D1D1',
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
