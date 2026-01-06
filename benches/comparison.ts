import {
  SE_GREG_CAL,
  SE_MOON,
  SE_SUN,
  SEFLG_SPEED,
  SEFLG_SWIEPH,
  swe_calc_ut,
  swe_julday,
} from "../mod.ts";

// @ts-ignore -- allowing usage of npm import in Deno
import * as Astronomy from "astronomy-engine";

console.log("🚀 Benchmarking: swisseph-wasm vs astronomy-engine");

Deno.bench("swisseph-wasm: Calculate Sun Position", {
  group: "Sun",
  baseline: true,
}, () => {
  const jd = swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
  swe_calc_ut(jd, SE_SUN, SEFLG_SWIEPH | SEFLG_SPEED);
});

Deno.bench("astronomy-engine: Calculate Sun Position", { group: "Sun" }, () => {
  Astronomy.GeoVector(
    Astronomy.Body.Sun,
    new Date("2000-01-01T12:00:00Z"),
    true,
  );
});

Deno.bench("swisseph-wasm: Calculate Moon Position", {
  group: "Moon",
  baseline: true,
}, () => {
  const jd = swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
  swe_calc_ut(jd, SE_MOON, SEFLG_SWIEPH | SEFLG_SPEED);
});

Deno.bench(
  "astronomy-engine: Calculate Moon Position",
  { group: "Moon" },
  () => {
    Astronomy.GeoVector(
      Astronomy.Body.Moon,
      new Date("2000-01-01T12:00:00Z"),
      true,
    );
  },
);
