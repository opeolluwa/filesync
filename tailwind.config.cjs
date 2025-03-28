/** @type {import('tailwindcss').Config} */

const colors = require("tailwindcss/colors");
const plugin = require("tailwindcss/plugin");

module.exports = {
  content: [
    "./index.html",
    "./src/**/*.rs",
    "./mobile/**/*.rs",
    "desktop/**/*.rs",
  ],
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      black: colors.black,
      white: colors.white,
      gray: colors.gray,
      emerald: colors.emerald,
      indigo: colors.indigo,
      yellow: colors.yellow,
      dark: colors.neutral,
      red: colors.red,
      accent: "rgba(226,233,252,255)",
      card: "#f9fbfe",
      slate: colors.slate,
      app: {
        DEFAULT: "#3074F5",
        50: "#DFEAFD",
        100: "#CCDDFD",
        200: "#A5C2FB",
        300: "#7EA8F9",
        400: "#578EF7",
        500: "#3074F5",
        600: "#0B55E2",
        700: "#0841AC",
        800: "#062D77",
        900: "#031941",
        950: "#020F27",
        dark: "#1d232a",
      },
    },
    extend: {},
  },
  plugins: [
    require("@tailwindcss/forms"),
    require("daisyui"),
    require("tailwind-plugin-ripple-effect"),
  ],
};
