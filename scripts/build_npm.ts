import { build, emptyDir } from "@deno/dnt";
import denoJson from "../deno.json" with { type: "json" };

await emptyDir("./npm");

await build({
  entryPoints: [
    "./mod.ts",
    {
      name: "./browser",
      path: "./lib/browser/swisseph_wasm.js",
    },
    {
      name: "./constants",
      path: "./src/typescript/constants.ts",
    },
  ],
  outDir: "./npm",
  typeCheck: false,
  test: false,
  shims: {
    // We don't need Deno shims for this pure/WASM lib usually,
    // but typically dnt adds some polyfills.
    // The WASM loader uses `URL` and `import.meta.url`, which dnt polyfills.
    deno: true,
  },
  compilerOptions: {
    lib: ["ESNext", "DOM"],
    skipLibCheck: true,
  },
  package: {
    // Must match package.json details
    name: denoJson.name,
    version: denoJson.version,
    description: denoJson.description,
    license: denoJson.license,
    publishConfig: {
      access: "public",
    },
    repository: {
      type: "git",
      url: "git+https://github.com/fusionstrings/swisseph-wasm.git",
    },
    bugs: {
      url: "https://github.com/fusionstrings/swisseph-wasm/issues",
    },
    exports: {
      ".": {
        import: "./esm/mod.js",
        require: "./script/mod.js",
      },
      "./browser": {
        import: "./esm/lib/browser/swisseph_wasm.js",
        require: "./script/lib/browser/swisseph_wasm.js",
      },
      "./constants": {
        import: "./esm/src/typescript/constants.js",
        require: "./script/src/typescript/constants.js",
      },
      "./wasm": "./esm/lib/swisseph_wasm.wasm",
    },
  },
  postBuild() {
    // Copy steps
    Deno.copyFileSync("LICENSE", "npm/LICENSE");
    Deno.copyFileSync("README.md", "npm/README.md");

    // Copy the WASM file to the correct location in the output.
    // DNT places compiled JS in npm/script/ and npm/esm/, so we must ensure
    // the WASM file is available relative to those files for the loader.

    const libDirEsm = "npm/esm/lib";
    const libDirScript = "npm/script/lib";

    Deno.mkdirSync(libDirEsm, { recursive: true });
    Deno.mkdirSync(libDirScript, { recursive: true });

    Deno.copyFileSync(
      "lib/swisseph_wasm.wasm",
      `${libDirEsm}/swisseph_wasm.wasm`,
    );
    Deno.copyFileSync(
      "lib/swisseph_wasm.wasm",
      `${libDirScript}/swisseph_wasm.wasm`,
    );

    // Browser files are already copied/transformed by dnt because they were in entryPoints
    // But we might need to ensure the .wasm file if it's referenced.
    // However, the browser build is "inlined" so it shouldn't need a separate .wasm file.
  },
});
