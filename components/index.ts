import Button from "./src/Button";
import Heading from "./src/Heading";
import Text from "./src/Text";
import View from "./src/View";
import Card from "./src/Card";

/**
 * https://stackoverflow.com/questions/67696920/vite-rollup-failed-to-resolve-build-error
 * resolve the import in target components tust
 * 
 * 
 * // vite.config.js
 * export default defineConfig({
  plugins: [react(), legacy()],
  resolve: {
    alias: {
      "@filesync/components": path.resolve(__dirname, "../components/index.ts"),
      "@filesync/types/*": path.resolve(__dirname, "src/pages"),
      "@utils": path.resolve(__dirname, "src/utils"),
    },
  },
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: "./src/setupTests.ts",
  },
});
 * 
 * 
 * //tsconfig.json
 * {
 * compilerOptions:{
 * "paths": {
*      "@filesync/types/*": ["../desktop/core/bindings/*"],
*      "@filesync/components": ["../components"]
*    }
 * },
 * "include": ["src", "../components/*", "../core/*"],
*}
*/

export { Button, Heading, Text, View, Card };
