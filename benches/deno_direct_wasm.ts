import { SE_GREG_CAL, swe_julday } from "../mod.ts";

const wasmBytes = await Deno.readFile("lib/swisseph_wasm.wasm");
const module = new WebAssembly.Module(wasmBytes);

// Mock imports minimally to allow instantiation
const imports = {
  "./swisseph_wasm.internal.js": {
    __wbg_new_1acc0b6eea89d040: () => {},
    __wbg_set_c2abbebe8b9ebee1: () => {},
    __wbg___wbindgen_debug_string_df47ffb5e35e6763: () => {},
    __wbg___wbindgen_throw_b855445ff6a94295: () => {},
    __wbindgen_init_externref_table: () => {},
    __wbindgen_cast_d6cd19b81560fd6e: () => {},
    __wbindgen_cast_2241b6af4c4b2941: () => {},
  },
};

const instance = new WebAssembly.Instance(module, imports);
const direct_julday = instance.exports.swe_julday as (
  y: number,
  m: number,
  d: number,
  h: number,
  f: number,
) => number;

const ITERATIONS = 1_000_000; // Higher count for small function

console.log(
  `Running ${ITERATIONS.toLocaleString()} iterations of Julian Day...`,
);

// --- Direct WASM ---
{
  const start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    direct_julday(2000, 1, 1, 12, SE_GREG_CAL);
  }
  const end = performance.now();
  const duration = end - start;
  const ops = Math.round(ITERATIONS / (duration / 1000));
  console.log(
    `[Direct WASM] Time: ${
      duration.toFixed(2)
    }ms | Throughput: ${ops.toLocaleString()} ops/sec`,
  );
}

// --- JS Wrapper (mod.ts) ---
{
  const start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
  }
  const end = performance.now();
  const duration = end - start;
  const ops = Math.round(ITERATIONS / (duration / 1000));
  console.log(
    `[JS Wrapper]  Time: ${
      duration.toFixed(2)
    }ms | Throughput: ${ops.toLocaleString()} ops/sec`,
  );
}
