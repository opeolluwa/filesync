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
      'sf_dark': {
        DEFAULT: '#1A1B1B',
        50: '#747979',
        100: '#6A6E6E',
        200: '#565959',
        300: '#424545',
        400: '#2E3030',
        500: '#1A1B1B',
        600: '#000000',
        700: '#000000',
        800: '#000000',
        900: '#000000'
      },
      'sf_dark_2': {
        DEFAULT: '#1A1B1B',
        50: '#B2B5B5',
        100: '#A8ABAB',
        200: '#939797',
        300: '#7E8383',
        400: '#6A6E6E',
        500: '#565959',
        600: '#424545',
        700: '#2E3030',
        800: '#1A1B1B',
        900: '#000000'
      },
      'mirage': {
        DEFAULT: '#1F1D36',
        50: '#6761AA',
        100: '#5C56A0',
        200: '#4D4886',
        300: '#3D3A6B',
        400: '#2E2B51',
        500: '#1F1D36',
        600: '#0A0912',
        700: '#000000',
        800: '#000000',
        900: '#000000'
      },
      'mirage-x': {
        DEFAULT: '#3F3351',
        50: '#9988B4',
        100: '#8F7BAC',
        200: '#79629C',
        300: '#665383',
        400: '#52436A',
        500: '#3F3351',
        600: '#241D2F',
        700: '#09080C',
        800: '#000000',
        900: '#000000'
      },
      'mirage-xx': {
        DEFAULT: '#864879',
        50: '#D5B1CD',
        100: '#CEA3C5',
        200: '#C089B4',
        300: '#B16EA3',
        400: '#A15691',
        500: '#864879',
        600: '#623458',
        700: '#3D2137',
        800: '#190D16',
        900: '#000000'
      },
      'shilo': {
        DEFAULT: '#E9A6A6',
        50: '#FFFFFF',
        100: '#FFFFFF',
        200: '#FFFFFF',
        300: '#F9E7E7',
        400: '#F1C7C7',
        500: '#E9A6A6',
        600: '#DE7979',
        700: '#D34C4C',
        800: '#B92E2E',
        900: '#8C2323'
      },
    },
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}