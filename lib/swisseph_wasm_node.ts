// @ts-nocheck: generated
import * as fs from "node:fs";
import * as path from "node:path";
import * as url from "node:url";

import * as internal from "./swisseph_wasm.internal.js";
import { __wbg_set_wasm } from "./swisseph_wasm.internal.js";

const currDir = path.dirname(url.fileURLToPath(import.meta.url));
const wasmPath = path.join(currDir, "swisseph_wasm.wasm");

const wasmModule = new WebAssembly.Module(fs.readFileSync(wasmPath));
const wasmInstance = new WebAssembly.Instance(wasmModule, {
  "./swisseph_wasm.internal.js": internal,
});

const wasm = wasmInstance.exports;
__wbg_set_wasm(wasm);

if (wasm.__wbindgen_start) {
  wasm.__wbindgen_start();
}

export * from "./swisseph_wasm.internal.js";
