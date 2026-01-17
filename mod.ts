import { SwissEph } from "./src/typescript/wasm.ts";
export * from "./src/typescript/constants.ts";

// Load Wasm
const wasmUrl = new URL("./lib/swisseph_wasm.wasm", import.meta.url);
const wasmBinary = await Deno.readFile(wasmUrl);
const inst = await SwissEph.init(wasmBinary);

// Bind exports
export const swe_version = inst.swe_version.bind(inst);
export const swe_get_library_path = inst.swe_get_library_path.bind(inst);
export const swe_deltat = inst.swe_deltat.bind(inst);
export const swe_sidtime = inst.swe_sidtime.bind(inst);
export const swe_julday = inst.swe_julday.bind(inst);
export const swe_revjul = inst.swe_revjul.bind(inst);
export const swe_utc_to_jd = inst.swe_utc_to_jd.bind(inst);
export const swe_jdet_to_utc = inst.swe_jdet_to_utc.bind(inst);
export const swe_jdut1_to_utc = inst.swe_jdut1_to_utc.bind(inst);
export const swe_calc_ut = inst.swe_calc_ut.bind(inst);
export const swe_calc = inst.swe_calc.bind(inst);
export const swe_houses = inst.swe_houses.bind(inst);
export const swe_houses_ex = inst.swe_houses_ex.bind(inst);
export const swe_house_pos = inst.swe_house_pos.bind(inst);
export const swe_house_name = inst.swe_house_name.bind(inst);
export const swe_pheno_ut = inst.swe_pheno_ut.bind(inst);
export const swe_get_planet_name = inst.swe_get_planet_name.bind(inst);
export const swe_rise_trans = inst.swe_rise_trans.bind(inst);
export const swe_sol_eclipse_where = inst.swe_sol_eclipse_where.bind(inst);
export const swe_sol_eclipse_how = inst.swe_sol_eclipse_how.bind(inst);
export const swe_sol_eclipse_when_loc = inst.swe_sol_eclipse_when_loc.bind(
  inst,
);
export const swe_sol_eclipse_when_glob = inst.swe_sol_eclipse_when_glob.bind(
  inst,
);
export const swe_lun_eclipse_when = inst.swe_lun_eclipse_when.bind(inst);
export const swe_lun_eclipse_how = inst.swe_lun_eclipse_how.bind(inst);
export const swe_get_ayanamsa_ut = inst.swe_get_ayanamsa_ut.bind(inst);
export const swe_get_ayanamsa_name = inst.swe_get_ayanamsa_name.bind(inst);
export const swe_degnorm = inst.swe_degnorm.bind(inst);
export const swe_radnorm = inst.swe_radnorm.bind(inst);
export const swe_day_of_week = inst.swe_day_of_week.bind(inst);
export const swe_set_ephe_path = inst.swe_set_ephe_path.bind(inst);
export const swe_set_jpl_file = inst.swe_set_jpl_file.bind(inst);
export const swe_set_topo = inst.swe_set_topo.bind(inst);
export const swe_set_sid_mode = inst.swe_set_sid_mode.bind(inst);
export const swe_close = inst.swe_close.bind(inst);
export const swe_fixstar_mag = inst.swe_fixstar_mag.bind(inst);
export const swe_fixstar = inst.swe_fixstar.bind(inst);
export const swe_fixstar_ut = inst.swe_fixstar_ut.bind(inst);
export const swe_get_ayanamsa = inst.swe_get_ayanamsa.bind(inst);
export const swe_get_ayanamsa_ex = inst.swe_get_ayanamsa_ex.bind(inst);
export const swe_get_ayanamsa_ex_ut = inst.swe_get_ayanamsa_ex_ut.bind(inst);
export const swe_heliacal_ut = inst.swe_heliacal_ut.bind(inst);
export const swe_heliacal_pheno_ut = inst.swe_heliacal_pheno_ut.bind(inst);
export const swe_nod_aps_ut = inst.swe_nod_aps_ut.bind(inst);
export const swe_nod_aps = inst.swe_nod_aps.bind(inst);
export const swe_azalt = inst.swe_azalt.bind(inst);
export const swe_azalt_rev = inst.swe_azalt_rev.bind(inst);
export const swe_refrac = inst.swe_refrac.bind(inst);
export const swe_refrac_extended = inst.swe_refrac_extended.bind(inst);
export const swe_sidtime0 = inst.swe_sidtime0.bind(inst);
export const swe_set_delta_t_userdef = inst.swe_set_delta_t_userdef.bind(inst);
export const swe_set_interpolate_nut = inst.swe_set_interpolate_nut.bind(inst);
export const swe_set_lapse_rate = inst.swe_set_lapse_rate.bind(inst);
export const swe_set_tid_acc = inst.swe_set_tid_acc.bind(inst);
export const swe_rise_trans_true_hor = inst.swe_rise_trans_true_hor.bind(inst);
export const swe_solcross = inst.swe_solcross.bind(inst);
export const swe_solcross_ut = inst.swe_solcross_ut.bind(inst);
export const swe_mooncross = inst.swe_mooncross.bind(inst);
export const swe_mooncross_ut = inst.swe_mooncross_ut.bind(inst);
export const swe_mooncross_node = inst.swe_mooncross_node.bind(inst);
export const swe_mooncross_node_ut = inst.swe_mooncross_node_ut.bind(inst);
export const swe_helio_cross = inst.swe_helio_cross.bind(inst);
export const swe_helio_cross_ut = inst.swe_helio_cross_ut.bind(inst);
export const swe_vis_limit_mag = inst.swe_vis_limit_mag.bind(inst);
export const swe_time_equ = inst.swe_time_equ.bind(inst);
export const swe_utc_time_zone = inst.swe_utc_time_zone.bind(inst);
export const swe_rad_midp = inst.swe_rad_midp.bind(inst);
export const swe_deg_midp = inst.swe_deg_midp.bind(inst);
export const swe_cotrans = inst.swe_cotrans.bind(inst);
export const swe_cotrans_sp = inst.swe_cotrans_sp.bind(inst);
export const swe_split_deg = inst.swe_split_deg.bind(inst);
export const swe_d2l = inst.swe_d2l.bind(inst);
export const swe_pheno = inst.swe_pheno.bind(inst);
export const swe_get_orbital_elements = inst.swe_get_orbital_elements.bind(
  inst,
);
export const swe_orbit_max_min_true_distance = inst
  .swe_orbit_max_min_true_distance.bind(inst);
export const swe_gauquelin_sector = inst.swe_gauquelin_sector.bind(inst);
export const swe_houses_armc = inst.swe_houses_armc.bind(inst);
export const swe_lmt_to_lat = inst.swe_lmt_to_lat.bind(inst);
export const swe_lat_to_lmt = inst.swe_lat_to_lmt.bind(inst);
export const swe_lun_occult_where = inst.swe_lun_occult_where.bind(inst);
export const swe_lun_occult_when_loc = inst.swe_lun_occult_when_loc.bind(inst);
export const swe_lun_occult_when_glob = inst.swe_lun_occult_when_glob.bind(
  inst,
);
export const swe_get_tid_acc = inst.swe_get_tid_acc.bind(inst);
export const swe_difdeg2n = inst.swe_difdeg2n.bind(inst);
export const swe_difdegn = inst.swe_difdegn.bind(inst);
export const swe_difrad2n = inst.swe_difrad2n.bind(inst);
export const swe_csroundsec = inst.swe_csroundsec.bind(inst);
export const swe_difcs2n = inst.swe_difcs2n.bind(inst);
export const swe_difcsn = inst.swe_difcsn.bind(inst);
export const swe_deltat_ex = inst.swe_deltat_ex.bind(inst);
export const swe_date_conversion = inst.swe_date_conversion.bind(inst);
export const swe_fixstar2 = inst.swe_fixstar2.bind(inst);
export const swe_fixstar2_mag = inst.swe_fixstar2_mag.bind(inst);
export const swe_fixstar2_ut = inst.swe_fixstar2_ut.bind(inst);
export const swe_houses_ex2 = inst.swe_houses_ex2.bind(inst);
export const swe_houses_armc_ex2 = inst.swe_houses_armc_ex2.bind(inst);
export const swe_calc_pctr = inst.swe_calc_pctr.bind(inst);
export const swe_csnorm = inst.swe_csnorm.bind(inst);
export const swe_lun_eclipse_when_loc = inst.swe_lun_eclipse_when_loc.bind(
  inst,
);
// Math exports (verification)
export const sin = inst.sin.bind(inst);
export const cos = inst.cos.bind(inst);
export const tan = inst.tan.bind(inst);
export const asin = inst.asin.bind(inst);
export const acos = inst.acos.bind(inst);
export const atan = inst.atan.bind(inst);
export const atan2 = inst.atan2.bind(inst);
export const sqrt = inst.sqrt.bind(inst);
export const exp = inst.exp.bind(inst);
export const log = inst.log.bind(inst);
export const pow = inst.pow.bind(inst);
// VFS
export const injectEphemerisFile = inst.injectEphemerisFile.bind(inst);
