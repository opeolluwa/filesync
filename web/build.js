const esbuild = require("esbuild")
const vuePlugin = require("esbuild-plugin-vue3")

esbuild.build({
    entryPoints: ["src/main.ts"],
    bundle: true,
    outfile: "dist/app.js",
    entryNames: '[dir]/[name]-[hash]',
    metafile: true,
    plugins: [vuePlugin({
        generateHTML: "src/index.html"
        // Or:
        // generateHTML: {
        //     sourceFile: "src/index.html",
        //     pathPrefix: "assets/",
        //     preload: [{ href: "https://example.com/my-external.css", as: "stylesheet" }]
        // }
    })]
})