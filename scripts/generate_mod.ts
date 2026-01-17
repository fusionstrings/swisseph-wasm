const wasmTsPath = "./src/typescript/wasm.ts";
const modTsPath = "./mod.ts";

const text = await Deno.readTextFile(wasmTsPath);
const methods = text.split("\n")
  .map((line) => line.match(/^\s*public\s+(swe_\w+)\(/))
  .filter((m) => m)
  .map((m) => m![1]);

// Deduplicate
const uniqueMethods = [...new Set(methods)];

console.log(`Found ${uniqueMethods.length} exports.`);

let content = `import { SwissEph } from "./src/typescript/wasm.ts";
export * from "./src/typescript/constants.ts";

// Load Wasm
const wasmUrl = new URL("./lib/swisseph_wasm.wasm", import.meta.url);
const wasmBinary = await Deno.readFile(wasmUrl);
const inst = await SwissEph.init(wasmBinary);

// Bind exports
`;

for (const method of uniqueMethods) {
  content += `export const ${method} = inst.${method}.bind(inst);\n`;
}

await Deno.writeTextFile(modTsPath, content);
console.log("Generated mod.ts");
