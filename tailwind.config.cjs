/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')
const plugin = require('tailwindcss/plugin')


module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      black: colors.black,
      white: colors.white,
      gray: colors.gray,
      emerald: colors.emerald,
      indigo: colors.indigo,
      yellow: colors.yellow,
      'sf_orange': {
        DEFAULT: '#E95420',
        50: '#F9D4C7',
        100: '#F8C6B5',
        200: '#F4A98F',
        300: '#F08D6A',
        400: '#ED7045',
        500: '#E95420',
        600: '#BE3F13',
        700: '#8B2E0E',
        800: '#581D09',
        900: '#250C04'
      },
      'sf_green': {
        DEFAULT: '#5BC5AE',
        50: '#E3F5F1',
        100: '#D4F0EA',
        200: '#B5E5DB',
        300: '#97DACC',
        400: '#79D0BD',
        500: '#5BC5AE',
        600: '#3DAB93',
        700: '#2E8270',
        800: '#1F584C',
        900: '#112F28'
      },
    },
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}