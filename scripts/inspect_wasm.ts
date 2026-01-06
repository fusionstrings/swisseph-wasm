const wasm = await Deno.readFile("lib/swisseph_wasm.wasm");
const module = new WebAssembly.Module(wasm);
console.log("Imports:", WebAssembly.Module.imports(module));
console.log("Exports:", WebAssembly.Module.exports(module));
