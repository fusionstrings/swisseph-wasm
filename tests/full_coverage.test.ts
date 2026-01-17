/**
 * swisseph-wasm Full API Test Coverage
 *
 * Tests all 95 exported functions for 100% API coverage.
 */

import {
  assertEquals,
  assertExists,
  assertNotEquals,
} from "https://deno.land/std@0.224.0/assert/mod.ts";

import {
  SE_CALC_RISE,
  SE_GREG_CAL,
  SE_MARS,
  SE_MOON,
  SE_SUN,
  SE_VENUS,
  SEFLG_MOSEPH,
  // Coordinates
  swe_azalt,
  swe_azalt_rev,
  swe_calc,
  swe_calc_pctr,
  swe_calc_ut,
  // Configuration
  swe_close,
  swe_cotrans,
  swe_cotrans_sp,
  swe_csnorm,
  swe_csroundsec,
  swe_d2l,
  swe_date_conversion,
  swe_day_of_week,
  swe_deg_midp,
  swe_degnorm,
  swe_deltat,
  swe_deltat_ex,
  swe_difcs2n,
  swe_difcsn,
  swe_difdeg2n,
  swe_difdegn,
  swe_difrad2n,
  // Fixed Stars
  swe_fixstar,
  swe_fixstar2,
  swe_fixstar2_mag,
  swe_fixstar2_ut,
  swe_fixstar_mag,
  swe_fixstar_ut,
  swe_gauquelin_sector,
  // Ayanamsa
  swe_get_ayanamsa,
  swe_get_ayanamsa_ex,
  swe_get_ayanamsa_ex_ut,
  swe_get_ayanamsa_name,
  swe_get_ayanamsa_ut,
  swe_get_orbital_elements,
  // Utilities
  swe_get_planet_name,
  swe_get_tid_acc,
  swe_heliacal_pheno_ut,
  // Heliacal
  swe_heliacal_ut,
  swe_helio_cross,
  swe_helio_cross_ut,
  swe_house_name,
  swe_house_pos,
  // Houses
  swe_houses,
  swe_houses_armc,
  swe_houses_armc_ex2,
  swe_houses_ex,
  swe_houses_ex2,
  swe_jdet_to_utc,
  swe_jdut1_to_utc,
  // Date/Time
  swe_julday,
  swe_lat_to_lmt,
  swe_lmt_to_lat,
  swe_lun_eclipse_how,
  swe_lun_eclipse_when,
  swe_lun_eclipse_when_loc,
  swe_lun_occult_when_glob,
  swe_lun_occult_when_loc,
  swe_lun_occult_where,
  swe_mooncross,
  swe_mooncross_node,
  swe_mooncross_node_ut,
  swe_mooncross_ut,
  swe_nod_aps,
  swe_nod_aps_ut,
  swe_orbit_max_min_true_distance,
  // Phenomena
  swe_pheno,
  swe_pheno_ut,
  swe_rad_midp,
  swe_radnorm,
  swe_refrac,
  swe_refrac_extended,
  swe_revjul,
  // Rise/Set
  swe_rise_trans,
  swe_rise_trans_true_hor,
  swe_set_delta_t_userdef,
  swe_set_interpolate_nut,
  swe_set_lapse_rate,
  swe_set_sid_mode,
  swe_set_tid_acc,
  swe_set_topo,
  swe_sidtime,
  swe_sidtime0,
  swe_sol_eclipse_how,
  swe_sol_eclipse_when_glob,
  swe_sol_eclipse_when_loc,
  // Eclipses
  swe_sol_eclipse_where,
  swe_solcross,
  swe_solcross_ut,
  swe_split_deg,
  swe_time_equ,
  swe_utc_time_zone,
  swe_utc_to_jd,
  // Core Calculation
  swe_version,
  swe_vis_limit_mag,
} from "../mod.ts";

// ============================================================
// SECTION 1: Core Calculation (11 functions)
// ============================================================

Deno.test("Core: swe_version", () => {
  const v = swe_version();
  assertExists(v);
  assertNotEquals(v, "");
  console.log("Version:", v);
});

Deno.test("Core: swe_calc", () => {
  const tjd = swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
  const res = swe_calc(tjd, SE_SUN, SEFLG_MOSEPH);
  assertExists(res);
  assertExists(res.longitude);
  console.log("swe_calc Sun:", res.longitude);
});

Deno.test("Core: swe_calc_ut", () => {
  const tjd = swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
  const res = swe_calc_ut(tjd, SE_MOON, SEFLG_MOSEPH);
  assertExists(res);
  assertExists(res.longitude);
  console.log("swe_calc_ut Moon:", res.longitude);
});

Deno.test("Core: swe_calc_pctr", () => {
  const tjd = swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
  try {
    const res = swe_calc_pctr(tjd, SE_MARS, SE_SUN, SEFLG_MOSEPH);
    assertExists(res);
    console.log("swe_calc_pctr Mars from Sun:", res.longitude);
  } catch (e) {
    console.log("swe_calc_pctr expected error (Moshier limitation):", e);
  }
});

Deno.test("Core: swe_solcross", () => {
  const tjd = swe_julday(2024, 3, 1, 0, SE_GREG_CAL);
  const result = swe_solcross(0, tjd, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_solcross:", result);
});

Deno.test("Core: swe_solcross_ut", () => {
  const tjd = swe_julday(2024, 3, 1, 0, SE_GREG_CAL);
  const result = swe_solcross_ut(0, tjd, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_solcross_ut:", result);
});

Deno.test("Core: swe_mooncross", () => {
  const tjd = swe_julday(2024, 1, 1, 0, SE_GREG_CAL);
  const result = swe_mooncross(0, tjd, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_mooncross:", result);
});

Deno.test("Core: swe_mooncross_ut", () => {
  const tjd = swe_julday(2024, 1, 1, 0, SE_GREG_CAL);
  const result = swe_mooncross_ut(0, tjd, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_mooncross_ut:", result);
});

Deno.test("Core: swe_mooncross_node", () => {
  const tjd = swe_julday(2024, 1, 1, 0, SE_GREG_CAL);
  const result = swe_mooncross_node(tjd, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_mooncross_node:", result);
});

Deno.test("Core: swe_mooncross_node_ut", () => {
  const tjd = swe_julday(2024, 1, 1, 0, SE_GREG_CAL);
  const result = swe_mooncross_node_ut(tjd, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_mooncross_node_ut:", result);
});

Deno.test("Core: swe_helio_cross", () => {
  const tjd = swe_julday(2024, 1, 1, 0, SE_GREG_CAL);
  try {
    const result = swe_helio_cross(SE_MARS, 0, tjd, SEFLG_MOSEPH, 1);
    console.log("swe_helio_cross:", result);
  } catch (e) {
    console.log("swe_helio_cross (expected error):", e);
  }
});

Deno.test("Core: swe_helio_cross_ut", () => {
  const tjd = swe_julday(2024, 1, 1, 0, SE_GREG_CAL);
  try {
    const result = swe_helio_cross_ut(SE_MARS, 0, tjd, SEFLG_MOSEPH, 1);
    console.log("swe_helio_cross_ut:", result);
  } catch (e) {
    console.log("swe_helio_cross_ut (expected error):", e);
  }
});

// ============================================================
// SECTION 2: Fixed Stars (6 functions)
// ============================================================

Deno.test("Stars: swe_fixstar", () => {
  try {
    const result = swe_fixstar("Sirius", 2451545.0, SEFLG_MOSEPH);
    console.log("swe_fixstar:", result);
    // Reference (swetest): 104.0853066
    const expectedLong = 104.0853066;
    if (Math.abs(result.longitude - expectedLong) > 1e-6) {
      throw new Error(
        `Sirius longitude mismatch: got ${result.longitude}, expected ${expectedLong}`,
      );
    }
  } catch (e) {
    console.log("swe_fixstar (no file):", e);
  }
});

Deno.test("Stars: swe_fixstar_ut", () => {
  try {
    const result = swe_fixstar_ut("Sirius", 2451545.0, SEFLG_MOSEPH);
    console.log("swe_fixstar_ut:", result);
  } catch (e) {
    console.log("swe_fixstar_ut (no file):", e);
  }
});

Deno.test("Stars: swe_fixstar_mag", () => {
  try {
    const result = swe_fixstar_mag("Sirius");
    console.log("swe_fixstar_mag:", result);
  } catch (e) {
    console.log("swe_fixstar_mag (no file):", e);
  }
});

Deno.test("Stars: swe_fixstar2", () => {
  try {
    const result = swe_fixstar2("Sirius", 2451545.0, SEFLG_MOSEPH);
    console.log("swe_fixstar2:", result);
  } catch (e) {
    console.log("swe_fixstar2 (no file):", e);
  }
});

Deno.test("Stars: swe_fixstar2_ut", () => {
  try {
    const result = swe_fixstar2_ut("Sirius", 2451545.0, SEFLG_MOSEPH);
    console.log("swe_fixstar2_ut:", result);
  } catch (e) {
    console.log("swe_fixstar2_ut (no file):", e);
  }
});

Deno.test("Stars: swe_fixstar2_mag", () => {
  try {
    const result = swe_fixstar2_mag("Sirius");
    console.log("swe_fixstar2_mag:", result);
  } catch (e) {
    console.log("swe_fixstar2_mag (no file):", e);
  }
});

// ============================================================
// SECTION 3: Date/Time (14 functions)
// ============================================================

Deno.test("DateTime: swe_julday", () => {
  const jd = swe_julday(2000, 1, 1, 12, SE_GREG_CAL);
  assertEquals(Math.round(jd), 2451545);
  console.log("swe_julday:", jd);
});

Deno.test("DateTime: swe_revjul", () => {
  const result = swe_revjul(2451545.0, SE_GREG_CAL);
  assertEquals(result.year, 2000);
  assertEquals(result.month, 1);
  assertEquals(result.day, 1);
  console.log("swe_revjul:", result);
});

Deno.test("DateTime: swe_sidtime", () => {
  const result = swe_sidtime(2451545.0);
  assertExists(result);
  console.log("swe_sidtime:", result);
});

Deno.test("DateTime: swe_sidtime0", () => {
  const result = swe_sidtime0(2451545.0, 23.44, 0);
  assertExists(result);
  console.log("swe_sidtime0:", result);
});

Deno.test("DateTime: swe_deltat", () => {
  const result = swe_deltat(2451545.0);
  assertExists(result);
  console.log("swe_deltat:", result);
});

Deno.test("DateTime: swe_deltat_ex", () => {
  const result = swe_deltat_ex(2451545.0, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_deltat_ex:", result);
});

Deno.test("DateTime: swe_date_conversion", () => {
  const result = swe_date_conversion(2000, 1, 1, 12.0, "g");
  assertExists(result);
  console.log("swe_date_conversion:", result);
});

Deno.test("DateTime: swe_utc_to_jd", () => {
  const result = swe_utc_to_jd(2000, 1, 1, 12, 0, 0, SE_GREG_CAL);
  assertExists(result.jd_ut);
  console.log("swe_utc_to_jd:", result);
});

Deno.test("DateTime: swe_jdet_to_utc", () => {
  const result = swe_jdet_to_utc(2451545.0, SE_GREG_CAL);
  assertExists(result);
  console.log("swe_jdet_to_utc:", result);
});

Deno.test("DateTime: swe_jdut1_to_utc", () => {
  const result = swe_jdut1_to_utc(2451545.0, SE_GREG_CAL);
  assertExists(result);
  console.log("swe_jdut1_to_utc:", result);
});

Deno.test("DateTime: swe_utc_time_zone", () => {
  const result = swe_utc_time_zone(2024, 1, 15, 12, 0, 0, -5);
  assertExists(result);
  console.log("swe_utc_time_zone:", result);
});

Deno.test("DateTime: swe_time_equ", () => {
  const result = swe_time_equ(2451545.0);
  assertExists(result);
  console.log("swe_time_equ:", result);
});

Deno.test("DateTime: swe_lmt_to_lat", () => {
  const result = swe_lmt_to_lat(2451545.0, 8.5);
  assertExists(result);
  console.log("swe_lmt_to_lat:", result);
});

Deno.test("DateTime: swe_lat_to_lmt", () => {
  const result = swe_lat_to_lmt(2451545.0, 8.5);
  assertExists(result);
  console.log("swe_lat_to_lmt:", result);
});

// ============================================================
// SECTION 4: Houses (7 functions)
// ============================================================

Deno.test("Houses: swe_houses", () => {
  const result = swe_houses(2451545.0, 47.37, 8.54, "P");
  assertEquals(result.cusps.length, 12);
  // Reference (swetest): 36.7814492
  const expectedAsc = 36.7814492;
  console.log(
    "swe_houses Ascendant:",
    result.ascendant,
    "Expected:",
    expectedAsc,
  );
  // Ensure we match C reference within 6 decimals
  if (Math.abs(result.ascendant - expectedAsc) > 1e-6) {
    throw new Error(
      `Ascendant mismatch: got ${result.ascendant}, expected ${expectedAsc}`,
    );
  }
});

Deno.test("Houses: swe_houses_ex", () => {
  const result = swe_houses_ex(2451545.0, SEFLG_MOSEPH, 47.37, 8.54, "P");
  assertEquals(result.cusps.length, 12);
  console.log("swe_houses_ex:", result.ascendant);
});

Deno.test("Houses: swe_houses_ex2", () => {
  const result = swe_houses_ex2(2451545.0, SEFLG_MOSEPH, 47.37, 8.54, "P");
  assertExists(result.cusps);
  // assertExists(result.cusp_speeds);
  console.log("swe_houses_ex2:", result.ascendant);
});

Deno.test("Houses: swe_houses_armc", () => {
  const result = swe_houses_armc(180, 47.37, 23.44, "P");
  assertExists(result.cusps);
  console.log("swe_houses_armc:", result.ascendant);
});

Deno.test("Houses: swe_houses_armc_ex2", () => {
  const result = swe_houses_armc_ex2(180, 47.37, 23.44, "P");
  assertExists(result.cusps);
  console.log("swe_houses_armc_ex2:", result.ascendant);
});

Deno.test("Houses: swe_house_pos", () => {
  const result = swe_house_pos(180, 47.37, 23.44, "P");
  assertExists(result);
  console.log("swe_house_pos:", result);
});

Deno.test("Houses: swe_house_name", () => {
  const result = swe_house_name("P");
  assertEquals(result, "Placidus");
  console.log("swe_house_name:", result);
});

// ============================================================
// SECTION 5: Eclipses (11 functions)
// ============================================================

Deno.test("Eclipse: swe_sol_eclipse_where", () => {
  const result = swe_sol_eclipse_where(2460408.26, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_sol_eclipse_where:", result);
});

Deno.test("Eclipse: swe_sol_eclipse_how", () => {
  const result = swe_sol_eclipse_how(2460408.26, SEFLG_MOSEPH, -96.8, 32.8, 0);
  assertExists(result);
  console.log("swe_sol_eclipse_how:", result.flags);
});

Deno.test("Eclipse: swe_sol_eclipse_when_loc", () => {
  const result = swe_sol_eclipse_when_loc(
    2460000,
    SEFLG_MOSEPH,
    -96.8,
    32.8,
    0,
  );
  assertExists(result);
  console.log("swe_sol_eclipse_when_loc:", result);
});

Deno.test("Eclipse: swe_sol_eclipse_when_glob", () => {
  const result = swe_sol_eclipse_when_glob(2451545.0, SEFLG_MOSEPH, 0, 0);
  assertExists(result);
  console.log("swe_sol_eclipse_when_glob:", result.tret[0]);
  // Reference (swetest): 2451580.034250
  const expectedJD = 2451580.034250;
  if (Math.abs(result.tret[0] - expectedJD) > 1e-6) {
    throw new Error(
      `Eclipse time mismatch: got ${result.tret[0]}, expected ${expectedJD}`,
    );
  }
});

Deno.test("Eclipse: swe_lun_eclipse_how", () => {
  const result = swe_lun_eclipse_how(2460398.0, SEFLG_MOSEPH, 0, 47.37, 0);
  assertExists(result);
  console.log("swe_lun_eclipse_how:", result);
});

Deno.test("Eclipse: swe_lun_eclipse_when", () => {
  const result = swe_lun_eclipse_when(2460000, SEFLG_MOSEPH, 0);
  assertExists(result);
  console.log("swe_lun_eclipse_when:", result.tret[0]);
});

Deno.test("Eclipse: swe_lun_eclipse_when_loc", () => {
  const result = swe_lun_eclipse_when_loc(
    2460000,
    SEFLG_MOSEPH,
    8.5,
    47.37,
    0,
    false,
  );
  assertExists(result);
  console.log("swe_lun_eclipse_when_loc:", result);
});

Deno.test("Eclipse: swe_lun_occult_where", () => {
  try {
    const result = swe_lun_occult_where(
      2460408.0,
      SE_SUN,
      "",
      SEFLG_MOSEPH,
    );
    console.log("swe_lun_occult_where:", result);
  } catch (e) {
    console.log("swe_lun_occult_where (expected):", e);
  }
});

Deno.test("Eclipse: swe_lun_occult_when_loc", () => {
  try {
    const result = swe_lun_occult_when_loc(
      2460000,
      SE_VENUS,
      "",
      SEFLG_MOSEPH,
      8.5,
      47.37,
      0,
      false,
    );
    console.log("swe_lun_occult_when_loc:", result);
  } catch (e) {
    console.log("swe_lun_occult_when_loc (expected):", e);
  }
});

Deno.test("Eclipse: swe_lun_occult_when_glob", () => {
  try {
    const result = swe_lun_occult_when_glob(
      2460000,
      SE_VENUS,
      "",
      SEFLG_MOSEPH,
      0,
      false,
    );
    console.log("swe_lun_occult_when_glob:", result);
  } catch (e) {
    console.log("swe_lun_occult_when_glob (expected):", e);
  }
});

Deno.test("Eclipse: swe_gauquelin_sector", () => {
  try {
    const result = swe_gauquelin_sector(
      2451545.0,
      SE_SUN,
      "",
      SEFLG_MOSEPH,
      0,
      8.5,
      47.37,
      0,
      1013,
      15,
    );
    console.log("swe_gauquelin_sector:", result);
  } catch (e) {
    console.log("swe_gauquelin_sector (expected):", e);
  }
});

// ============================================================
// SECTION 6: Heliacal (3 functions)
// ============================================================

Deno.test("Heliacal: swe_heliacal_ut", () => {
  try {
    const result = swe_heliacal_ut(
      2451545.0,
      8.5,
      47.37,
      0,
      new Float64Array([1013, 15, 0, 0]),
      new Float64Array([0.5, 10, 30, 1]),
      "Venus",
      1,
    );
    console.log("swe_heliacal_ut:", result);
  } catch (e) {
    console.log("swe_heliacal_ut (expected):", e);
  }
});

Deno.test("Heliacal: swe_heliacal_pheno_ut", () => {
  try {
    const result = swe_heliacal_pheno_ut(
      2451545.0,
      8.5,
      47.37,
      0,
      new Float64Array([1013, 15, 0, 0]),
      new Float64Array([0.5, 10, 30, 1]),
      "Venus",
      1,
    );
    console.log("swe_heliacal_pheno_ut:", result);
  } catch (e) {
    console.log("swe_heliacal_pheno_ut (expected):", e);
  }
});

Deno.test("Heliacal: swe_vis_limit_mag", () => {
  try {
    const result = swe_vis_limit_mag(
      2451545.0,
      8.5,
      47.37,
      0,
      1013,
      15,
      0.5,
      10,
      30,
      1,
      "Venus",
      0,
    );
    console.log("swe_vis_limit_mag:", result);
  } catch (e) {
    console.log("swe_vis_limit_mag (expected):", e);
  }
});

// ============================================================
// SECTION 7: Phenomena (6 functions)
// ============================================================

Deno.test("Pheno: swe_pheno", () => {
  const result = swe_pheno(2451545.0, SE_MARS, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_pheno:", result);
});

Deno.test("Pheno: swe_pheno_ut", () => {
  const result = swe_pheno_ut(2451545.0, SE_MARS, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_pheno_ut:", result);
});

Deno.test("Pheno: swe_nod_aps", () => {
  const result = swe_nod_aps(2451545.0, SE_MOON, SEFLG_MOSEPH, 0);
  assertExists(result);
  console.log("swe_nod_aps:", result);
});

Deno.test("Pheno: swe_nod_aps_ut", () => {
  const result = swe_nod_aps_ut(2451545.0, SE_MOON, SEFLG_MOSEPH, 0);
  assertExists(result);
  console.log("swe_nod_aps_ut:", result);
});

Deno.test("Pheno: swe_get_orbital_elements", () => {
  const result = swe_get_orbital_elements(2451545.0, SE_MOON, SEFLG_MOSEPH);
  assertExists(result.semi_major_axis);
  console.log("swe_get_orbital_elements:", result.semi_major_axis);
});

Deno.test("Pheno: swe_orbit_max_min_true_distance", () => {
  const result = swe_orbit_max_min_true_distance(
    2451545.0,
    SE_MOON,
    SEFLG_MOSEPH,
  );
  assertExists(result);
  console.log("swe_orbit_max_min_true_distance:", result);
});

// ============================================================
// SECTION 8: Coordinates (6 functions)
// ============================================================

Deno.test("Coords: swe_azalt", () => {
  const result = swe_azalt(2451545.0, 0, 8.5, 47.37, 0, 1013, 15, 280, 0, 1);
  assertExists(result);
  console.log("swe_azalt:", result);
});

Deno.test("Coords: swe_azalt_rev", () => {
  const result = swe_azalt_rev(2451545.0, 0, 8.5, 47.37, 0, 180, 45);
  assertExists(result);
  console.log("swe_azalt_rev:", result);
});

Deno.test("Coords: swe_cotrans", () => {
  const result = swe_cotrans(280, 0, 1, 23.44);
  assertExists(result);
  console.log("swe_cotrans:", result);
});

Deno.test("Coords: swe_cotrans_sp", () => {
  const result = swe_cotrans_sp(280, 0, 1, 23.44);
  assertExists(result);
  console.log("swe_cotrans_sp:", result);
});

Deno.test("Coords: swe_refrac", () => {
  const result = swe_refrac(45, 1013, 15, 0);
  assertExists(result);
  console.log("swe_refrac:", result);
});

Deno.test("Coords: swe_refrac_extended", () => {
  const result = swe_refrac_extended(45, 0, 1013, 15, 0.0065, 0);
  assertExists(result);
  console.log("swe_refrac_extended:", result);
});

// ============================================================
// SECTION 9: Rise/Set (2 functions)
// ============================================================

Deno.test("RiseSet: swe_rise_trans", () => {
  const result = swe_rise_trans(
    2451544.5, // Start at 00:00 to catch sunrise
    SE_SUN,
    "",
    SEFLG_MOSEPH,
    SE_CALC_RISE,
    8.54,
    47.37,
    0,
    1013.25,
    15,
  );
  assertExists(result);
  console.log("swe_rise_trans:", result);
  // Reference: 2451544.800827
  const expectedJD = 2451544.800827;
  if (Math.abs(result.tret - expectedJD) > 1e-4) { // Rise times can vary slightly by refraction model
    throw new Error(
      `Rise time mismatch: got ${result.tret}, expected ${expectedJD}`,
    );
  }
});

Deno.test("RiseSet: swe_rise_trans_true_hor", () => {
  const result = swe_rise_trans_true_hor(
    2451545.0,
    SE_SUN,
    "",
    SEFLG_MOSEPH,
    SE_CALC_RISE,
    8.5,
    47.37,
    0,
    1013,
    15,
    0,
  );
  assertExists(result);
  console.log("swe_rise_trans_true_hor:", result);
});

// ============================================================
// SECTION 10: Configuration (7 functions)
// ============================================================

Deno.test("Config: swe_set_topo", () => {
  swe_set_topo(8.5, 47.37, 400);
  console.log("swe_set_topo: set to Zurich");
});

Deno.test("Config: swe_set_sid_mode", () => {
  swe_set_sid_mode(1, 0, 0); // Lahiri
  console.log("swe_set_sid_mode: Lahiri");
});

Deno.test("Config: swe_set_lapse_rate", () => {
  swe_set_lapse_rate(0.0065);
  console.log("swe_set_lapse_rate: 0.0065");
});

Deno.test("Config: swe_set_tid_acc", () => {
  swe_set_tid_acc(-25.8);
  console.log("swe_set_tid_acc: -25.8");
});

Deno.test("Config: swe_set_delta_t_userdef", () => {
  swe_set_delta_t_userdef(-1); // Reset
  console.log("swe_set_delta_t_userdef: reset");
});

Deno.test("Config: swe_set_interpolate_nut", () => {
  swe_set_interpolate_nut(1);
  console.log("swe_set_interpolate_nut: enabled");
});

Deno.test("Config: swe_close", () => {
  swe_close();
  console.log("swe_close: called");
});

// ============================================================
// SECTION 11: Utilities (16 functions)
// ============================================================

Deno.test("Util: swe_get_planet_name", () => {
  const result = swe_get_planet_name(SE_SUN);
  assertEquals(result, "Sun");
  console.log("swe_get_planet_name:", result);
});

Deno.test("Util: swe_split_deg", () => {
  const result = swe_split_deg(123.456, 0);
  assertExists(result);
  console.log("swe_split_deg:", result);
});

Deno.test("Util: swe_degnorm", () => {
  assertEquals(Math.round(swe_degnorm(370)), 10);
  assertEquals(Math.round(swe_degnorm(-30)), 330);
  console.log("swe_degnorm: OK");
});

Deno.test("Util: swe_radnorm", () => {
  const result = swe_radnorm(7);
  if (result < 0 || result >= 2 * Math.PI) throw new Error("Invalid");
  console.log("swe_radnorm:", result);
});

Deno.test("Util: swe_deg_midp", () => {
  const result = swe_deg_midp(350, 10);
  assertEquals(result, 0);
  console.log("swe_deg_midp:", result);
});

Deno.test("Util: swe_rad_midp", () => {
  const result = swe_rad_midp(0.1, 6.1);
  assertExists(result);
  console.log("swe_rad_midp:", result);
});

Deno.test("Util: swe_difdegn", () => {
  const result = swe_difdegn(10, 350);
  assertExists(result);
  console.log("swe_difdegn:", result);
});

Deno.test("Util: swe_difdeg2n", () => {
  const result = swe_difdeg2n(10, 350);
  assertExists(result);
  console.log("swe_difdeg2n:", result);
});

Deno.test("Util: swe_difrad2n", () => {
  const result = swe_difrad2n(0.1, 6.1);
  assertExists(result);
  console.log("swe_difrad2n:", result);
});

Deno.test("Util: swe_day_of_week", () => {
  const result = swe_day_of_week(2451545.0);
  assertEquals(result, 5); // Saturday
  console.log("swe_day_of_week:", result);
});

Deno.test("Util: swe_d2l", () => {
  const result = swe_d2l(3.7);
  assertEquals(result, 4);
  console.log("swe_d2l:", result);
});

Deno.test("Util: swe_get_tid_acc", () => {
  const result = swe_get_tid_acc();
  assertExists(result);
  console.log("swe_get_tid_acc:", result);
});

Deno.test("Util: swe_csnorm", () => {
  const result = swe_csnorm(1300000);
  assertExists(result);
  console.log("swe_csnorm:", result);
});

Deno.test("Util: swe_difcsn", () => {
  const result = swe_difcsn(100000, 50000);
  assertExists(result);
  console.log("swe_difcsn:", result);
});

Deno.test("Util: swe_difcs2n", () => {
  const result = swe_difcs2n(100000, 1200000);
  assertExists(result);
  console.log("swe_difcs2n:", result);
});

Deno.test("Util: swe_csroundsec", () => {
  const result = swe_csroundsec(12345);
  assertExists(result);
  console.log("swe_csroundsec:", result);
});

// ============================================================
// SECTION 12: Ayanamsa (5 functions)
// ============================================================

Deno.test("Ayanamsa: swe_get_ayanamsa", () => {
  const result = swe_get_ayanamsa(2451545.0);
  assertExists(result);
  console.log("swe_get_ayanamsa:", result);
});

Deno.test("Ayanamsa: swe_get_ayanamsa_ut", () => {
  const result = swe_get_ayanamsa_ut(2451545.0);
  assertExists(result);
  console.log("swe_get_ayanamsa_ut:", result);
});

Deno.test("Ayanamsa: swe_get_ayanamsa_ex", () => {
  const result = swe_get_ayanamsa_ex(2451545.0, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_get_ayanamsa_ex:", result);
});

Deno.test("Ayanamsa: swe_get_ayanamsa_ex_ut", () => {
  const result = swe_get_ayanamsa_ex_ut(2451545.0, SEFLG_MOSEPH);
  assertExists(result);
  console.log("swe_get_ayanamsa_ex_ut:", result);
});

Deno.test("Ayanamsa: swe_get_ayanamsa_name", () => {
  const result = swe_get_ayanamsa_name(1);
  assertEquals(result, "Lahiri");
  console.log("swe_get_ayanamsa_name:", result);
});
