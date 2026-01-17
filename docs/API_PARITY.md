# swisseph-wasm API Parity Report

**Last Updated**: January 13, 2026\
**Library Version**: 0.1.5\
**Swiss Ephemeris Version**: 2.10.03

---

## Summary

| Metric                   | Count | Percentage |
| ------------------------ | ----- | ---------- |
| **Source C APIs**        | 95    | 100%       |
| **FFI Bindings**         | 90    | 95%        |
| **WASM Exports**         | 95    | 100%       |
| **Not Applicable (VFS)** | 5     | 5%         |

> ✅ **100% API coverage achieved!** All applicable Swiss Ephemeris functions
> are now available in JavaScript/TypeScript.

---

## 1. Core Planetary Calculation

| Function                | Status      | Notes                             |
| ----------------------- | ----------- | --------------------------------- |
| `swe_calc_ut`           | ✅ Exported | Primary calculation function (UT) |
| `swe_calc`              | ✅ Exported | ET version                        |
| `swe_calc_pctr`         | ✅ Exported | Planet-centric coordinates        |
| `swe_solcross`          | ✅ Exported | Sun longitude crossing (ET)       |
| `swe_solcross_ut`       | ✅ Exported | Sun longitude crossing (UT)       |
| `swe_mooncross`         | ✅ Exported | Moon longitude crossing (ET)      |
| `swe_mooncross_ut`      | ✅ Exported | Moon longitude crossing (UT)      |
| `swe_mooncross_node`    | ✅ Exported | Moon node crossing (ET)           |
| `swe_mooncross_node_ut` | ✅ Exported | Moon node crossing (UT)           |
| `swe_helio_cross`       | ✅ Exported | Heliocentric crossing (ET)        |
| `swe_helio_cross_ut`    | ✅ Exported | Heliocentric crossing (UT)        |

---

## 2. Fixed Stars

| Function           | Status      | Notes                          |
| ------------------ | ----------- | ------------------------------ |
| `swe_fixstar_ut`   | ✅ Exported | Star position calculation (UT) |
| `swe_fixstar`      | ✅ Exported | ET version                     |
| `swe_fixstar_mag`  | ✅ Exported | Star magnitude                 |
| `swe_fixstar2`     | ✅ Exported | Alternative lookup (ET)        |
| `swe_fixstar2_ut`  | ✅ Exported | Alternative lookup (UT)        |
| `swe_fixstar2_mag` | ✅ Exported | Alternative magnitude          |

---

## 3. Date, Time & Calendar

| Function              | Status      | Notes                      |
| --------------------- | ----------- | -------------------------- |
| `swe_julday`          | ✅ Exported | Calendar → Julian Day      |
| `swe_revjul`          | ✅ Exported | Julian Day → Calendar      |
| `swe_sidtime`         | ✅ Exported | Sidereal time              |
| `swe_deltat`          | ✅ Exported | Delta T (TT-UT)            |
| `swe_date_conversion` | ✅ Exported | Date validation            |
| `swe_utc_to_jd`       | ✅ Exported | UTC → Julian Day           |
| `swe_jdet_to_utc`     | ✅ Exported | JD (ET) → UTC              |
| `swe_jdut1_to_utc`    | ✅ Exported | JD (UT1) → UTC             |
| `swe_utc_time_zone`   | ✅ Exported | Time zone conversion       |
| `swe_time_equ`        | ✅ Exported | Equation of time           |
| `swe_lmt_to_lat`      | ✅ Exported | LMT → LAT                  |
| `swe_lat_to_lmt`      | ✅ Exported | LAT → LMT                  |
| `swe_sidtime0`        | ✅ Exported | Sidereal time (custom eps) |
| `swe_deltat_ex`       | ✅ Exported | Delta T with flags         |

---

## 4. Houses

| Function              | Status      | Notes                   |
| --------------------- | ----------- | ----------------------- |
| `swe_houses`          | ✅ Exported | House cusps + Asc/MC    |
| `swe_houses_ex`       | ✅ Exported | Extended with flags     |
| `swe_house_pos`       | ✅ Exported | Planet's house position |
| `swe_houses_ex2`      | ✅ Exported | With cusp speeds        |
| `swe_houses_armc`     | ✅ Exported | From ARMC               |
| `swe_houses_armc_ex2` | ✅ Exported | ARMC with speeds        |
| `swe_house_name`      | ✅ Exported | House system name       |

---

## 5. Eclipses & Occultations

| Function                    | Status      | Notes                     |
| --------------------------- | ----------- | ------------------------- |
| `swe_sol_eclipse_where`     | ✅ Exported | Eclipse max location      |
| `swe_sol_eclipse_when_glob` | ✅ Exported | Next global eclipse       |
| `swe_sol_eclipse_when_loc`  | ✅ Exported | Next local eclipse        |
| `swe_sol_eclipse_how`       | ✅ Exported | Solar eclipse at location |
| `swe_lun_eclipse_how`       | ✅ Exported | Lunar eclipse details     |
| `swe_lun_eclipse_when`      | ✅ Exported | Next lunar eclipse        |
| `swe_lun_eclipse_when_loc`  | ✅ Exported | Local lunar eclipse       |
| `swe_lun_occult_where`      | ✅ Exported | Occultation location      |
| `swe_lun_occult_when_loc`   | ✅ Exported | Local occultation         |
| `swe_lun_occult_when_glob`  | ✅ Exported | Global occultation        |
| `swe_gauquelin_sector`      | ✅ Exported | Gauquelin sectors         |

---

## 6. Heliacal Events

| Function                  | Status      | Notes              |
| ------------------------- | ----------- | ------------------ |
| `swe_heliacal_ut`         | ✅ Exported | Heliacal rise/set  |
| `swe_vis_limit_mag`       | ✅ Exported | Limiting magnitude |
| `swe_heliacal_pheno_ut`   | ✅ Exported | Heliacal details   |
| `swe_heliacal_angle`      | ❌ N/A      | Internal/expert    |
| `swe_topo_arcus_visionis` | ❌ N/A      | Internal/expert    |

---

## 7. Phenomena & Orbital

| Function                          | Status      | Notes                  |
| --------------------------------- | ----------- | ---------------------- |
| `swe_pheno_ut`                    | ✅ Exported | Phase, elongation, mag |
| `swe_pheno`                       | ✅ Exported | ET version             |
| `swe_nod_aps_ut`                  | ✅ Exported | Nodes & apsides (UT)   |
| `swe_nod_aps`                     | ✅ Exported | ET version             |
| `swe_get_orbital_elements`        | ✅ Exported | Keplerian elements     |
| `swe_orbit_max_min_true_distance` | ✅ Exported | Orbital extremes       |

---

## 8. Coordinate Transforms

| Function              | Status      | Notes                |
| --------------------- | ----------- | -------------------- |
| `swe_azalt`           | ✅ Exported | To horizon coords    |
| `swe_azalt_rev`       | ✅ Exported | From horizon coords  |
| `swe_cotrans`         | ✅ Exported | Coordinate transform |
| `swe_cotrans_sp`      | ✅ Exported | With speed           |
| `swe_refrac`          | ✅ Exported | Refraction           |
| `swe_refrac_extended` | ✅ Exported | Extended refraction  |

---

## 9. Rise, Set & Transit

| Function                  | Status      | Notes                  |
| ------------------------- | ----------- | ---------------------- |
| `swe_rise_trans`          | ✅ Exported | Rise/set/transit times |
| `swe_rise_trans_true_hor` | ✅ Exported | True horizon version   |

---

## 10. Configuration

| Function                  | Status      | Notes                  |
| ------------------------- | ----------- | ---------------------- |
| `swe_close`               | ✅ Exported | Cleanup                |
| `swe_set_topo`            | ✅ Exported | Topocentric location   |
| `swe_set_sid_mode`        | ✅ Exported | Sidereal mode          |
| `swe_set_lapse_rate`      | ✅ Exported | Atmospheric lapse      |
| `swe_set_tid_acc`         | ✅ Exported | Tidal acceleration     |
| `swe_set_delta_t_userdef` | ✅ Exported | Custom Delta T         |
| `swe_set_interpolate_nut` | ✅ Exported | Nutation interpolation |
| `swe_set_ephe_path`       | ❌ N/A      | No filesystem in WASM  |
| `swe_set_jpl_file`        | ❌ N/A      | No filesystem in WASM  |

---

## 11. Utilities & Math

| Function              | Status      | Notes              |
| --------------------- | ----------- | ------------------ |
| `swe_version`         | ✅ Exported | Library version    |
| `swe_get_planet_name` | ✅ Exported | Planet name string |
| `swe_split_deg`       | ✅ Exported | Degrees → DMS      |
| `swe_degnorm`         | ✅ Exported | Normalize degrees  |
| `swe_radnorm`         | ✅ Exported | Normalize radians  |
| `swe_deg_midp`        | ✅ Exported | Degree midpoint    |
| `swe_rad_midp`        | ✅ Exported | Radian midpoint    |
| `swe_difdegn`         | ✅ Exported | Degree difference  |
| `swe_difdeg2n`        | ✅ Exported | Signed degree diff |
| `swe_difrad2n`        | ✅ Exported | Signed radian diff |
| `swe_day_of_week`     | ✅ Exported | Weekday from JD    |
| `swe_d2l`             | ✅ Exported | Double to long     |
| `swe_get_tid_acc`     | ✅ Exported | Get tidal acc      |
| `swe_csnorm`          | ✅ Exported | Centisec normalize |
| `swe_difcsn`          | ✅ Exported | Centisec diff      |
| `swe_difcs2n`         | ✅ Exported | Signed cs diff     |
| `swe_csroundsec`      | ✅ Exported | Round centisec     |

---

## 12. Ayanamsa

| Function                 | Status      | Notes         |
| ------------------------ | ----------- | ------------- |
| `swe_get_ayanamsa_ut`    | ✅ Exported | Ayanamsa (UT) |
| `swe_get_ayanamsa`       | ✅ Exported | Ayanamsa (ET) |
| `swe_get_ayanamsa_ex`    | ✅ Exported | Extended (ET) |
| `swe_get_ayanamsa_ex_ut` | ✅ Exported | Extended (UT) |
| `swe_get_ayanamsa_name`  | ✅ Exported | Ayanamsa name |

---

## 13. Info & File Data

| Function                    | Status | Notes                  |
| --------------------------- | ------ | ---------------------- |
| `swe_get_library_path`      | ❌ N/A | Not meaningful in WASM |
| `swe_get_current_file_data` | ❌ N/A | No filesystem in WASM  |

---

## Legend

| Status      | Meaning                            |
| ----------- | ---------------------------------- |
| ✅ Exported | Available in JavaScript/TypeScript |
| ❌ N/A      | Not applicable to WASM environment |
