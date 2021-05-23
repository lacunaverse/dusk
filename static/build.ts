require("esbuild")
    .build({
        entryPoints: ["./src/scripts/index.ts", "./src/scripts/delete.ts"],
        bundle: true,
        outdir: "dist/",
        minify: true,
        sourcemap: true,
        treeShaking: true,
        watch: true,
        keepNames: true,
    })
    .catch(() => process.exit(1));

require("esbuild")
    .build({
        entryPoints: ["./src/styles/index.css"],
        bundle: true,
        outdir: "dist/",
        minify: true,
        treeShaking: true,
        watch: true,
        keepNames: true,
    })
    .catch(() => process.exit(1));
