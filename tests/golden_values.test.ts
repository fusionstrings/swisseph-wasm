import {
  assertAlmostEquals,
} from "https://deno.land/std@0.211.0/assert/mod.ts";
import {
  SE_GREG_CAL,
  SE_JUPITER,
  SE_MARS,
  SE_MOON,
  SE_SATURN,
  SE_SUN,
  SEFLG_EQUATORIAL,
  SEFLG_MOSEPH,
  SEFLG_SPEED,
  swe_calc_ut,
  swe_julday,
} from "../mod.ts";

// Golden Values for 2026-01-13 00:00:00 UTC (Moshier Mode)
// These values are established from the initial Zig port and serve as a baseline
// to prevent regression in the math implementation.
const GOLDEN_DATE = { year: 2026, month: 1, day: 13, hour: 0 };

Deno.test("Golden Values: Sun (Moshier)", () => {
  const tjd = swe_julday(
    GOLDEN_DATE.year,
    GOLDEN_DATE.month,
    GOLDEN_DATE.day,
    GOLDEN_DATE.hour,
    SE_GREG_CAL,
  );

  const res = swe_calc_ut(tjd, SE_SUN, SEFLG_MOSEPH | SEFLG_SPEED);

  // Baseline values (to be filled/verified on first run)
  // Expected roughly: Longitude ~292.8 deg (Capricorn)
  // Let's run and see, then lock it in.
  console.log("Golden Sun:", res);

  // Baseline values verified against swetest (version 2.10.03)
  // Sun: 292.7951404
  const expectedLong = 292.7951403623997;
  console.log("Golden Sun:", res.longitude);
  assertAlmostEquals(res.longitude, expectedLong, 1e-7);
});

Deno.test("Golden Values: Moon (Moshier)", () => {
  const tjd = swe_julday(
    GOLDEN_DATE.year,
    GOLDEN_DATE.month,
    GOLDEN_DATE.day,
    GOLDEN_DATE.hour,
    SE_GREG_CAL,
  );
  const res = swe_calc_ut(tjd, SE_MOON, SEFLG_MOSEPH | SEFLG_SPEED);
  console.log("Golden Moon:", res.longitude);

  // Baseline verified against swetest
  // Moon: 228.3733032
  const expectedLong = 228.3733031995555;
  assertAlmostEquals(res.longitude, expectedLong, 1e-7);
});
