import { SE_GREG_CAL, swe_deltat, swe_julday } from "../mod.ts";
import { assertExists } from "https://deno.land/std@0.211.0/assert/mod.ts";

const DATE = { year: 2026, month: 1, day: 13, hour: 0 };

Deno.test("Debug: Delta T", () => {
  const tjd = swe_julday(
    DATE.year,
    DATE.month,
    DATE.day,
    DATE.hour,
    SE_GREG_CAL,
  );
  const dt = swe_deltat(tjd);
  console.log(`Delta T for 2026-01-13: ${dt} days (= ${dt * 86400} seconds)`);
  assertExists(dt);
});
