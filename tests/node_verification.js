
const assert = require('assert');
const path = require('path');

// Resolve the built package
// We assume we are running this from root, so ./npm
const swisseph = require('../npm');

console.log("Loaded swisseph-wasm from Node.js (CommonJS)", swisseph);

// Test constants
assert.strictEqual(swisseph.SE_SUN, 0);
assert.strictEqual(swisseph.SE_MOON, 1);
assert.strictEqual(swisseph.SE_GREG_CAL, 1);

console.log("Constants verified.");

// Test Julian Day
const jd = swisseph.swe_julday(2000, 1, 1, 12.0, swisseph.SE_GREG_CAL);
console.log(`JD for 2000-01-01 12:00: ${jd}`);
assert.strictEqual(jd, 2451545.0);

// Test Calculation
const sun = swisseph.swe_calc_ut(jd, swisseph.SE_SUN, swisseph.SEFLG_SWIEPH | swisseph.SEFLG_SPEED);
console.log("Sun Position:", sun);

assert.strictEqual(typeof sun.longitude, 'number');
assert.ok(sun.longitude > 275 && sun.longitude < 285, "Sun longitude sensible");

console.log("✅ Node.js Verification Passed!");
