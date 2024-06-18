import * as esbuild from "esbuild";
import shell from "shelljs";
import { transform } from "lightningcss";

await esbuild.build({
  entryPoints: ["src/main.ts"],
  bundle: true,
  minify: true,
  sourcemap: true,
  // target: ["chrome58", "firefox57", "safari11", "edge16"],
  outfile: "dist/assets/app.js",
});


shell.exec("cp index.html dist/index.html")
shell.exec("npx tailwindcss -i ./src/app.css -o ./dist/assets/main.css");
