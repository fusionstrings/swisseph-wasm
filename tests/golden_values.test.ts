import {
  assertAlmostEquals,
} from "https://deno.land/std@0.211.0/assert/mod.ts";
import {
  injectEphemerisFile,
  SE_GREG_CAL,
  SE_JUPITER,
  SE_MARS,
  SE_MOON,
  SE_SATURN,
  SE_SUN,
  SEFLG_EQUATORIAL,
  SEFLG_MOSEPH,
  SEFLG_SPEED,
  SEFLG_SWIEPH,
  swe_calc_ut,
  swe_julday,
  swe_set_ephe_path,
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

Deno.test("Golden Values: Moon (JPL / SEFLG_SWIEPH) with VFS", async () => {
  // 1. Read real ephemeris files
  const semoData = await Deno.readFile("vendor/swisseph/ephe/semo_18.se1");
  const seplData = await Deno.readFile("vendor/swisseph/ephe/sepl_18.se1");

  // 2. Inject into VFS
  injectEphemerisFile("semo_18.se1", semoData);
  injectEphemerisFile("sepl_18.se1", seplData);

  // Set ephemeris path to current directory so VFS finds the files
  swe_set_ephe_path(".");

  // 3. Calc with SEFLG_SWIEPH
  const tjd = swe_julday(
    GOLDEN_DATE.year,
    GOLDEN_DATE.month,
    GOLDEN_DATE.day,
    GOLDEN_DATE.hour,
    SE_GREG_CAL,
  );

  // Request SWIEPH. If file is missing/rejected, it falls back to MOSEPH.
  const res = swe_calc_ut(tjd, SE_MOON, SEFLG_SWIEPH | SEFLG_SPEED);
  console.log("JPL Moon:", res);

  // 4. Verify we got SWIEPH mode
  // SEFLG_SWIEPH = 2
  // SEFLG_MOSEPH = 4
  if ((res.ret_flag & SEFLG_SWIEPH) === 0) {
    throw new Error(`Failed to use JPL mode. Fallback used: ${res.ret_flag}`);
  }

  // 5. Verify Precision (JPL vs Moshier distinction)
  // Moshier Moon was: 228.3733031995555
  // JPL Moon is: 228.37316257852348
  console.log("JPL Moon Long:", res.longitude);
  assertAlmostEquals(res.longitude, 228.37316257852348, 1e-12);
});
