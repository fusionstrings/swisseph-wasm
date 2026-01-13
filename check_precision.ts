import { SE_SUN, SEFLG_MOSEPH, SEFLG_SPEED, swe_calc_ut } from "./mod.ts";

// Force load to ensure WASM is ready
try {
  const result = swe_calc_ut(2451545.0, SE_SUN, SEFLG_MOSEPH | SEFLG_SPEED);
  console.log("Wasm Result:", result.longitude.toFixed(20));
} catch (e) {
  console.error("Error:", e);
}
