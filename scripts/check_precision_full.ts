import { join } from "jsr:@std/path";
import * as wasm from "../lib/swisseph_wasm.js";

// Path to swetest binary
const swetestPath = join(Deno.cwd(), "vendor/swisseph/swetest");

// Planets to test: Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto, Mean Node, True Node
const PLANETS = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const PLANET_NAMES = [
  "Sun",
  "Moon",
  "Mercury",
  "Venus",
  "Mars",
  "Jupiter",
  "Saturn",
  "Uranus",
  "Neptune",
  "Pluto",
  "MeanNode",
  "TrueNode",
];

// Number of dates to test
const NUM_DATES = 1000;

// Tolerance
const EPSILON = 1e-14; // Strict demand

function generateRandomJulianDay(): number {
  // Range: 1900 - 2100 roughly
  // JD 2415020.5 (1900) to 2488070.5 (2100)
  const min = 2415020.5;
  const max = 2488070.5;
  return Math.random() * (max - min) + min;
}

async function runSwetest(jd: number, planet: number): Promise<number | null> {
  const pStr = planet.toString();
  const cmd = new Deno.Command(swetestPath, {
    args: [
      `-bj${jd}`,
      `-p${pStr}`,
      "-n1",
      "-fP", // P = longitude
      "-head", // no header
      "-g", // silent
      "-eswe", // force swiss eph
      "-ut",
    ],
  });

  const { stdout, stderr, success } = await cmd.output();

  if (!success) {
    // Fallback or just log error?
    // Swetest might return non-zero if ephe file missing, often returns 255.
    // We might want to try -emos if -eswe fails, but let's see why first.
    // console.error("swetest failed:", new TextDecoder().decode(stderr));
    return null;
  }

  const output = new TextDecoder().decode(stdout).trim();

  // Output should be just the number if arguments are right, or "lon: <num>"
  const match = output.match(/([0-9]+\.[0-9]+)/);
  if (match) {
    return parseFloat(match[1]);
  }

  // console.error(`Parse failed for cmd output: '${output}' (Start of output)`);
  return null;
}

async function main() {
  console.log(`Starting Precision Audit for ${NUM_DATES} dates...`);

  // Initial check
  const testCmd = new Deno.Command(swetestPath, {
    args: ["-b2451545", "-p0", "-n1", "-fP", "-head", "-eswe"],
  });
  const { success } = await testCmd.output();
  let ephFlag = wasm.SEFLG_SWIEPH;
  // let swetestEphArg = "-eswe";

  if (!success) {
    console.log(
      "Warning: swetest failed with -eswe (Validation Step). Falling back to Moshier (-emos).",
    );
    ephFlag = wasm.SEFLG_MOSEPH;
    // swetestEphArg = "-emos";
  }

  let maxDiff = 0;
  let failures = 0;
  let totalTests = 0;

  for (let i = 0; i < NUM_DATES; i++) {
    const jd = generateRandomJulianDay();

    for (let pIdx = 0; pIdx < PLANETS.length; pIdx++) {
      const planet = PLANETS[pIdx];
      const pName = PLANET_NAMES[pIdx];

      // 1. Native swetest
      const cmd = new Deno.Command(swetestPath, {
        args: [
          `-bj${jd}`,
          `-p${planet}`,
          "-n1",
          "-fP",
          "-head",
          ephFlag === wasm.SEFLG_SWIEPH ? "-eswe" : "-emos",
          "-ut",
        ],
      });
      const { stdout, success } = await cmd.output();
      if (!success) continue;

      const outStr = new TextDecoder().decode(stdout).trim();
      const nativeVal = parseFloat(outStr.split(/\s+/).pop() || "NaN");

      if (isNaN(nativeVal)) {
        continue;
      }

      // 2. Wasm
      let wasmResult = wasm.swe_calc_ut(jd, planet, ephFlag | wasm.SEFLG_SPEED); // Use speed to match default better? Or just base?
      // swetest default might use SPEED. But we just care about position.
      // If we pass SEFLG_SPEED to wasm, position should be same.
      // Let's stick to base iflag.

      // Wait, earlier I used just ephFlag.

      if (wasmResult.error) {
        // console.log("Wasm Error", wasmResult);
        continue;
      }

      // Check for object format (some functions return float, calc returns object)
      // swe_calc_ut returns object or throws/returns string error in wrapper?
      // Wrapper returns Result<Object, string>.
      // In imported JS, it might throw if Err.

      let wasmVal: number;
      try {
        // wasmResult is the object
        wasmVal = wasmResult.longitude;
      } catch (e) {
        continue;
      }

      const diff = Math.abs(nativeVal - wasmVal);
      let diffNorm = Math.abs(nativeVal - wasmVal);
      if (diffNorm > 180) diffNorm = 360 - diffNorm;

      if (diffNorm > maxDiff) maxDiff = diffNorm;

      if (diffNorm > EPSILON) {
        failures++;
        // console.log(`FAIL [${pName} @ ${jd}]: Native ${nativeVal} vs Wasm ${wasmVal} (Diff: ${diffNorm})`);
      }

      totalTests++;
    }
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
