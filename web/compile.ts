console.log(new Date(), "builing...");
console.time("build-time");

await Bun.build({
  entrypoints: ["./index.tsx"],
  outdir: "./build",
  target: "browser",
  splitting: true,
  sourcemap: "external",
  minify: {
    whitespace: true,
    identifiers: true,
    syntax: true,
  },
});

console.log(new Date(), "Done!");
console.timeEnd("build-time");
