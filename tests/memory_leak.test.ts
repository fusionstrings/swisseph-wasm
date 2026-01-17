import { SE_SUN, SEFLG_MOSEPH, SEFLG_SPEED, swe_calc_ut } from "../mod.ts";
import { assert } from "jsr:@std/assert";

Deno.test("Memory Leak Check: 1,000,000 runs", () => {
  const startMem = Deno.memoryUsage().rss;
  console.log(`Start Memory: ${(startMem / 1024 / 1024).toFixed(2)} MB`);

  for (let i = 0; i < 1_000_000; i++) {
    swe_calc_ut(2451545.0, SE_SUN, SEFLG_MOSEPH | SEFLG_SPEED);
    // if (i % 100000 === 0) console.log(`Run ${i}`);
  }

  // @ts-ignore: expose-gc flag
  if (globalThis.gc) globalThis.gc();

  const endMem = Deno.memoryUsage().rss;
  console.log(`End Memory: ${(endMem / 1024 / 1024).toFixed(2)} MB`);
  const diff = endMem - startMem;
  console.log(`Memory Diff: ${(diff / 1024 / 1024).toFixed(2)} MB`);

  // Threshold: 50MB growth allowed (likely mostly V8 overhead if any)
  assert(diff < 50 * 1024 * 1024, "Memory usage grew by more than 50MB");
});
