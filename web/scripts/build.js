import * as esbuild from "esbuild";
import shell from "shelljs";

await esbuild.build({
  entryPoints: ["src/main.ts"],
  bundle: true,
  minify: true,
  outfile: "dist/assets/app.js",
});


shell.exec("cp index.html dist/index.html")
shell.exec("npx tailwindcss -i ./src/app.css -o ./dist/assets/main.css");
