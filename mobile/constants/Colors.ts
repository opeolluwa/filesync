/**
 * Below are the colors that are used in the app. The colors are defined in the light and dark mode.
 * There are many other ways to style your app. For example, [Nativewind](https://www.nativewind.dev/), [Tamagui](https://tamagui.dev/), [unistyles](https://reactnativeunistyles.vercel.app), etc.
 */

const tintColorLight = "#578EF7";
const tintColorDark = '#fff';

export const Colors = {
  accent: "rgba(226,233,252,255)",
  gray:
  '#4b5563',
  card: "#f9fbfe",
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
  },
  light: {
    text: "#11181C",
    background: "#fff",
    tint: tintColorLight,
    icon: "#687076",
    tabIconDefault: "#687076",
    tabIconSelected: tintColorLight,
  },
  dark: {
    text: "#ECEDEE",
    background: "#151718",
    tint: tintColorDark,
    icon: "#9BA1A6",
    tabIconDefault: "#9BA1A6",
    tabIconSelected: tintColorDark,
  },
};
