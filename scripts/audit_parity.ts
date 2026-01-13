import { join } from "jsr:@std/path";
import * as wasm from "../lib/swisseph_wasm.js";

// Path to the C header file
const headerPath = join(Deno.cwd(), "vendor/swisseph/swephexp.h");

async function main() {
  console.log("Reading swephexp.h...");
  const headerContent = await Deno.readTextFile(headerPath);

  // Extract C exports
  // Pattern: ext_def( ... ) func_name( ...
  const cExports = new Set<string>();
  const regex = /ext_def\s*\([^)]+\)\s+([a-zA-Z0-9_]+)\s*\(/g;
  let match;
  while ((match = regex.exec(headerContent)) !== null) {
    cExports.add(match[1]);
  }

  console.log(`Found ${cExports.size} exported functions in C header.`);

  // Extract Wasm exports from the loaded module
  const wasmExports = new Set<string>(Object.keys(wasm));
  console.log(`Found ${wasmExports.size} exported functions in Wasm module.`);

  // Logic to ignore expected internal helpers if any
  // (e.g., memory management, init, etc. provided by wasm-bindgen)
  // For now, we list everything.

  // explicit exclusions (Emscripten internals or wasm-bindgen specific)
  const ignoredWasmExports = new Set([
    "init",
    "default",
    "memory",
    // Add others if we find them
  ]);

  // Compare
  const missingInWasm: string[] = [];
  const extraInWasm: string[] = [];

  for (const func of cExports) {
    if (!wasmExports.has(func)) {
      missingInWasm.push(func);
    }
  }

  for (const func of wasmExports) {
    if (!cExports.has(func) && !ignoredWasmExports.has(func)) {
      extraInWasm.push(func);
    }
  }

  console.log("\n=== MISSING IN WASM (Present in C) ===");
  if (missingInWasm.length === 0) {
    console.log("None! Full coverage (of detected C exports).");
  } else {
    missingInWasm.sort().forEach((f) => console.log(`- ${f}`));
  }

  console.log("\n=== EXTRA IN WASM (Not in exposed C API) ===");
  if (extraInWasm.length === 0) {
    console.log("None.");
  } else {
    extraInWasm.sort().forEach((f) => console.log(`- ${f}`));
  }
}

if (import.meta.main) {
  await main();
}
