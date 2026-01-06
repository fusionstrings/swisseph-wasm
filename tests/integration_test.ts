import { assertEquals } from "@std/assert";
import {
  SE_GREG_CAL,
  SE_MOON,
  SE_SUN,
  SEFLG_SPEED,
  SEFLG_SWIEPH,
  swe_calc_ut,
  swe_get_planet_name,
  swe_julday,
  swe_revjul,
} from "../mod.ts";

Deno.test("Julian Day Calculation", () => {
  // Jan 1, 2000, 12:00 UTC -> JD 2451545.0
  const jd = swe_julday(2000, 1, 1, 12.0, SE_GREG_CAL);
  assertEquals(jd, 2451545.0);
});

Deno.test("Reverse Julian Day", () => {
  const result = swe_revjul(2451545.0, SE_GREG_CAL);
  assertEquals(result.year, 2000);
  assertEquals(result.month, 1);
  assertEquals(result.day, 1);
  assertEquals(result.hour, 12.0);
});

Deno.test("Planet Name", () => {
  const name = swe_get_planet_name(SE_SUN);
  assertEquals(name, "Sun");
});

Deno.test("Sun Position (J2000)", () => {
  // Julian Day for J2000.0 is 2451545.0
  // Sun mean longitude is approx 280deg. True longitude varies.
  // We just verify it returns a valid structure and numbers are in range [0, 360).
  const jd = 2451545.0;
  const sun = swe_calc_ut(jd, SE_SUN, SEFLG_SWIEPH | SEFLG_SPEED);

  console.log("Sun @ J2000:", sun);

  assertEquals(typeof sun.longitude, "number");
  assertEquals(typeof sun.latitude, "number");
  assertEquals(typeof sun.distance, "number");

  // Roughly Capricorn (approx 280 degrees for Jan 1)
  // Actually Sun attempts to enter Capricorn on Dec 21/22 (270 deg).
  // Jan 1 is about 10 days later. ~1 deg per day.
  // So expected longitude ~280 +/- 2 degrees.
  const long = sun.longitude;
  if (long < 275 || long > 285) {
    throw new Error(
      `Sun longitude ${long} out of expected range for Jan 1 (approx 280)`,
    );
  }
});

Deno.test("Moon Position Consistency", () => {
  const jd = 2451545.0;
  const moon = swe_calc_ut(jd, SE_MOON, SEFLG_SWIEPH);

  console.log("Moon @ J2000:", moon);

  // Moon moves fast, but latitude should be small (-5 to +5 deg)
  if (moon.latitude < -6 || moon.latitude > 6) {
    throw new Error("Moon latitude impossibly high outside eclipse limits");
  }
});
