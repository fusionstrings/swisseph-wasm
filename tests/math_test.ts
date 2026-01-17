import {
  assertAlmostEquals,
} from "https://deno.land/std@0.211.0/assert/mod.ts";
import {
  acos,
  asin,
  atan,
  atan2,
  cos,
  exp,
  log,
  pow,
  sin,
  sqrt,
  tan,
} from "../mod.ts";

const EPSILON = 1e-9;

// Helper to check against JS native
function check(
  name: string,
  wasmFn: (x: number) => number,
  jsFn: (x: number) => number,
  input: number,
) {
  const w = wasmFn(input);
  const j = jsFn(input);
  const diff = Math.abs(w - j);
  // For large numbers, check relative error
  if (Math.abs(j) > 1e10) {
    const rel = diff / Math.abs(j);
    if (rel > 1e-14) {
      console.error(
        `FAIL ${name}(${input}): Wasm=${w}, JS=${j}, RelDiff=${rel}`,
      );
      throw new Error(`Precision failed for ${name}`);
    }
  } else if (diff > EPSILON) {
    console.error(`FAIL ${name}(${input}): Wasm=${w}, JS=${j}, Diff=${diff}`);
    throw new Error(`Precision failed for ${name}`);
  }
}

function check2(
  name: string,
  wasmFn: (y: number, x: number) => number,
  jsFn: (y: number, x: number) => number,
  y: number,
  x: number,
) {
  const w = wasmFn(y, x);
  const j = jsFn(y, x);
  const diff = Math.abs(w - j);
  if (diff > EPSILON) {
    console.error(`FAIL ${name}(${y}, ${x}): Wasm=${w}, JS=${j}, Diff=${diff}`);
    throw new Error(`Precision failed for ${name}`);
  }
}

Deno.test("Math Precision: Trigonometry", () => {
  // Check range edges and common values
  [0, 0.5, 1, 1.5, 2, 3, 3.14, -1, -3, 10, 100].forEach((x) => {
    check("sin", sin, Math.sin, x);
    check("cos", cos, Math.cos, x);
    // tan approaches infinity, limit range
    if (Math.abs(Math.cos(x)) > 0.01) check("tan", tan, Math.tan, x);
  });
});

Deno.test("Math Precision: Inverse Trig", () => {
  [-0.99, -0.5, 0, 0.5, 0.99].forEach((x) => {
    check("asin", asin, Math.asin, x);
    check("acos", acos, Math.acos, x);
  });
  [-10, -1, 0, 1, 10].forEach((x) => {
    check("atan", atan, Math.atan, x);
  });
  check2("atan2", atan2, Math.atan2, 1, 2);
  check2("atan2", atan2, Math.atan2, -1, -2);
  check2("atan2", atan2, Math.atan2, 0.1, 0.1);
});

Deno.test("Math Precision: Log/Exp/Pow", () => {
  [0.1, 1, 2, 10, 100].forEach((x) => {
    check("log", log, Math.log, x);
    check("exp", exp, Math.exp, x);
    check("sqrt", sqrt, Math.sqrt, x);
  });
  check2("pow", pow, Math.pow, 2, 3);
  check2("pow", pow, Math.pow, 2.5, 3.5);
  check2("pow", pow, Math.pow, 10, -2);
});
