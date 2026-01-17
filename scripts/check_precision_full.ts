import { join } from "jsr:@std/path";
import * as wasm from "../lib/swisseph_wasm.js";

// Tolerance for floating point comparison (relaxed slightly for Moshier)
// If parity is perfect it should be 1e-15. If it's algorithmic noise, maybe 1e-13.
const EPSILON = 1e-13;

async function main() {
  console.log(`Starting Precision Audit (Moshier Parity Check)...`);

  const version = wasm.swe_version();
  console.log(`Wasm Library Version: ${version}`);

  // Read reference values
  let references: any[];
  try {
    const jsonContent = await Deno.readTextFile("reference_values.json");
    references = JSON.parse(jsonContent);
    console.log(
      `Loaded ${references.length} reference calculations from C (Moshier mode).`,
    );
  } catch (e) {
    console.error("Failed to read reference_values.json:", e);
    Deno.exit(1);
  }

  let maxDiff = 0;
  let failures = 0;
  let totalTests = 0;

  for (const ref of references) {
    // Run Wasm calculation
    // Enforce Moshier to match reference
    const iflag = wasm.SEFLG_MOSEPH | wasm.SEFLG_SPEED;
    const result = wasm.swe_calc(ref.jd, ref.planet, iflag);

    if (result.error) {
      console.error(
        `Error for JD ${ref.jd}, Planet ${ref.planet}: ${result.error}`,
      );
      failures++;
      continue;
    }

    // Compare Longitude
    let diffLon = Math.abs(ref.lon - result.longitude);
    if (diffLon > 180) diffLon = 360 - diffLon; // Normalize angle diff

    // Compare Latitude
    let diffLat = Math.abs(ref.lat - result.latitude);

    // Compare Distance
    let diffDist = Math.abs(ref.dist - result.distance);

    const currentMax = Math.max(diffLon, diffLat, diffDist);
    if (currentMax > maxDiff) {
      maxDiff = currentMax;
    }

    if (diffLon > EPSILON || diffLat > EPSILON || diffDist > EPSILON) {
      failures++;
      if (failures < 5) {
        console.log(`FAIL [P${ref.planet} @ ${ref.jd}]:`);
        console.log(`  J Reference: ${ref.lon}, ${ref.lat}, ${ref.dist}`);
        console.log(
          `  W Received : ${result.longitude}, ${result.latitude}, ${result.distance}`,
        );
        console.log(`  Diffs      : ${diffLon}, ${diffLat}, ${diffDist}`);
        console.log(
          `  W Flags    : ${result.rc_flags} (Expected bitmask with ${iflag})`,
        );
      }
    }

    totalTests++;
  }

  console.log(`\nAudit Complete.`);
  console.log(`Total Calculations: ${totalTests}`);
  console.log(`Max Difference: ${maxDiff}`);
  console.log(`Failures (> ${EPSILON}): ${failures}`);

  if (failures === 0 && totalTests > 0) {
    console.log("Result: PASS");
  } else {
    console.log("Result: FAIL");
    Deno.exit(1);
  }
}

if (import.meta.main) {
  await main();
}
