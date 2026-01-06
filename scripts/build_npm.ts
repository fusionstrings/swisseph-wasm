import { build, emptyDir } from "jsr:@deno/dnt@^0.41.3";

await emptyDir("./npm");

await build({
  entryPoints: ["./mod.ts"],
  outDir: "./npm",
  test: false,
  shims: {
    // We don't need Deno shims for this pure/WASM lib usually,
    // but typically dnt adds some polyfills.
    // The WASM loader uses `URL` and `import.meta.url`, which dnt polyfills.
    deno: true,
  },
  mappings: {
    "./lib/swisseph_wasm.js": "./lib/swisseph_wasm_node.ts",
  },
  package: {
    // Must match package.json details
    name: "@fusionstrings/swisseph-wasm",
    version: "0.1.0",
    description:
      "High-precision Swiss Ephemeris bindings for WebAssembly (Rust based)",
    license: "MIT",
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
        import: "./browser/swisseph_wasm.js",
        types: "./browser/swisseph_wasm.d.ts",
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

    // Copy Browser Build (Inlined)
    const browserDir = "npm/browser";
    Deno.mkdirSync(browserDir, { recursive: true });
    Deno.copyFileSync(
      "lib/browser/swisseph_wasm.js",
      `${browserDir}/swisseph_wasm.js`,
    );
    Deno.copyFileSync(
      "lib/browser/swisseph_wasm.d.ts",
      `${browserDir}/swisseph_wasm.d.ts`,
    );
  },
});
