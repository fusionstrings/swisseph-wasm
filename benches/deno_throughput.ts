import {
  SE_GREG_CAL,
  SE_SUN,
  SEFLG_SPEED,
  SEFLG_SWIEPH,
  swe_calc_ut,
  swe_julday,
} from "../mod.ts";

// @ts-ignore -- allowing usage of npm import
// @ts-ignore -- allowing usage of npm import
import * as Astronomy from "astronomy-engine";

const ITERATIONS = 100_000;

console.log(`Running ${ITERATIONS} iterations of Sun position calculation...`);

// --- swisseph-wasm ---
const jd = swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
const flags = SEFLG_SWIEPH | SEFLG_SPEED;

const startSwe = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  swe_calc_ut(jd, SE_SUN, flags);
}
const endSwe = performance.now();
const durationSwe = endSwe - startSwe;
const opsSwe = Math.round(ITERATIONS / (durationSwe / 1000));

console.log(
  `[swisseph-wasm]    Time: ${
    durationSwe.toFixed(2)
  }ms | Throughput: ${opsSwe.toLocaleString()} ops/sec`,
);

// --- astronomy-engine ---
const date = new Date("2000-01-01T12:00:00Z");
Astronomy.GeoVector(Astronomy.Body.Sun, date, true); // warmup

const startAstro = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  Astronomy.GeoVector(Astronomy.Body.Sun, date, true);
}
const endAstro = performance.now();
const durationAstro = endAstro - startAstro;
const opsAstro = Math.round(ITERATIONS / (durationAstro / 1000));

console.log(
  `[astronomy-engine] Time: ${
    durationAstro.toFixed(2)
  }ms | Throughput: ${opsAstro.toLocaleString()} ops/sec`,
);
