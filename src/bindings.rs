use core::ffi::{c_int, c_double, c_char};

#[link(name = "swe")]
unsafe extern "C" {
    // --- Core Calculation ---
    #[link_name = "impl_swe_version"]
    pub fn swe_version(s: *mut c_char) -> *mut c_char;
    #[link_name = "impl_swe_get_library_path"]
    pub fn swe_get_library_path(path: *mut c_char) -> *mut c_char;

    #[link_name = "impl_swe_calc"]
    pub fn swe_calc(tjd: c_double, ipl: c_int, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_calc_ut"]
    pub fn swe_calc_ut(tjd_ut: c_double, ipl: c_int, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_calc_pctr"]
    pub fn swe_calc_pctr(tjd: c_double, ipl: c_int, iplctr: c_int, iflag: c_int, xxret: *mut c_double, serr: *mut c_char) -> c_int;

    #[link_name = "impl_swe_solcross"]
    pub fn swe_solcross(x2cross: c_double, jd_et: c_double, flag: c_int, serr: *mut c_char) -> c_double;
    #[link_name = "impl_swe_solcross_ut"]
    pub fn swe_solcross_ut(x2cross: c_double, jd_ut: c_double, flag: c_int, serr: *mut c_char) -> c_double;

    #[link_name = "impl_swe_mooncross"]
    pub fn swe_mooncross(x2cross: c_double, jd_et: c_double, flag: c_int, serr: *mut c_char) -> c_double;
    #[link_name = "impl_swe_mooncross_ut"]
    pub fn swe_mooncross_ut(x2cross: c_double, jd_ut: c_double, flag: c_int, serr: *mut c_char) -> c_double;
    
    #[link_name = "impl_swe_mooncross_node"]
    pub fn swe_mooncross_node(jd_et: c_double, flag: c_int, xlon: *mut c_double, xlat: *mut c_double, serr: *mut c_char) -> c_double;
    #[link_name = "impl_swe_mooncross_node_ut"]
    pub fn swe_mooncross_node_ut(jd_ut: c_double, flag: c_int, xlon: *mut c_double, xlat: *mut c_double, serr: *mut c_char) -> c_double;

    #[link_name = "impl_swe_helio_cross"]
    pub fn swe_helio_cross(ipl: c_int, x2cross: c_double, jd_et: c_double, iflag: c_int, dir: c_int, jd_cross: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_helio_cross_ut"]
    pub fn swe_helio_cross_ut(ipl: c_int, x2cross: c_double, jd_ut: c_double, iflag: c_int, dir: c_int, jd_cross: *mut c_double, serr: *mut c_char) -> c_int;

    // --- Fixed Stars ---
    #[link_name = "impl_swe_fixstar"]
    pub fn swe_fixstar(star: *mut c_char, tjd: c_double, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_fixstar_ut"]
    pub fn swe_fixstar_ut(star: *mut c_char, tjd_ut: c_double, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_fixstar_mag"]
    pub fn swe_fixstar_mag(star: *mut c_char, mag: *mut c_double, serr: *mut c_char) -> c_int;
    
    #[link_name = "impl_swe_fixstar2"]
    pub fn swe_fixstar2(star: *mut c_char, tjd: c_double, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_fixstar2_ut"]
    pub fn swe_fixstar2_ut(star: *mut c_char, tjd_ut: c_double, iflag: c_int, xx: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_fixstar2_mag"]
    pub fn swe_fixstar2_mag(star: *mut c_char, mag: *mut c_double, serr: *mut c_char) -> c_int;

    // --- Date & Time & Calendar ---
    #[link_name = "impl_swe_julday"]
    pub fn swe_julday(year: c_int, month: c_int, day: c_int, hour: c_double, gregflag: c_int) -> c_double;
    #[link_name = "impl_swe_revjul"]
    pub fn swe_revjul(tjd: c_double, gregflag: c_int, year: *mut c_int, month: *mut c_int, day: *mut c_int, hour: *mut c_double);
    #[link_name = "impl_swe_date_conversion"]
    pub fn swe_date_conversion(y: c_int, m: c_int, d: c_int, utime: c_double, c: c_char, tjd: *mut c_double) -> c_int;
    
    #[link_name = "impl_swe_utc_to_jd"]
    pub fn swe_utc_to_jd(iyear: c_int, imonth: c_int, iday: c_int, ihour: c_int, imin: c_int, dsec: c_double, gregflag: c_int, dret: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_jdet_to_utc"]
    pub fn swe_jdet_to_utc(tjd_et: c_double, gregflag: c_int, iyear: *mut c_int, imonth: *mut c_int, iday: *mut c_int, ihour: *mut c_int, imin: *mut c_int, dsec: *mut c_double);
    #[link_name = "impl_swe_jdut1_to_utc"]
    pub fn swe_jdut1_to_utc(tjd_ut: c_double, gregflag: c_int, iyear: *mut c_int, imonth: *mut c_int, iday: *mut c_int, ihour: *mut c_int, imin: *mut c_int, dsec: *mut c_double);
    #[link_name = "impl_swe_utc_time_zone"]
    pub fn swe_utc_time_zone(iyear: c_int, imonth: c_int, iday: c_int, ihour: c_int, imin: c_int, dsec: c_double, d_timezone: c_double, iyear_out: *mut c_int, imonth_out: *mut c_int, iday_out: *mut c_int, ihour_out: *mut c_int, imin_out: *mut c_int, dsec_out: *mut c_double);

    #[link_name = "impl_swe_time_equ"]
    pub fn swe_time_equ(tjd: c_double, te: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_lmt_to_lat"]
    pub fn swe_lmt_to_lat(tjd_lmt: c_double, geolon: c_double, tjd_lat: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_lat_to_lmt"]
    pub fn swe_lat_to_lmt(tjd_lat: c_double, geolon: c_double, tjd_lmt: *mut c_double, serr: *mut c_char) -> c_int;
    
    #[link_name = "impl_swe_sidtime"]
    pub fn swe_sidtime(tjd_ut: c_double) -> c_double;
    #[link_name = "impl_swe_sidtime0"]
    pub fn swe_sidtime0(tjd_ut: c_double, eps: c_double, nut: c_double) -> c_double;


    // --- Configuration ---
    #[link_name = "impl_swe_set_topo"]
    pub fn swe_set_topo(geolon: c_double, geolat: c_double, geoalt: c_double);
    #[link_name = "impl_swe_set_sid_mode"]
    pub fn swe_set_sid_mode(sid_mode: c_int, t0: c_double, ayan_t0: c_double);
    #[link_name = "impl_swe_set_ephe_path"]
    pub fn swe_set_ephe_path(path: *mut c_char);
    #[link_name = "impl_swe_set_jpl_file"]
    pub fn swe_set_jpl_file(fname: *mut c_char);
    #[link_name = "impl_swe_set_lapse_rate"]
    pub fn swe_set_lapse_rate(lapse_rate: c_double);
    #[link_name = "impl_swe_set_tid_acc"]
    pub fn swe_set_tid_acc(t_acc: c_double);
    #[link_name = "impl_swe_set_delta_t_userdef"]
    pub fn swe_set_delta_t_userdef(dt: c_double);
    #[link_name = "impl_swe_set_interpolate_nut"]
    pub fn swe_set_interpolate_nut(do_interpolate: c_int);
    #[link_name = "impl_swe_close"]
    pub fn swe_close();


    // --- Eclipses & Occultations ---
    #[link_name = "impl_swe_sol_eclipse_where"]
    pub fn swe_sol_eclipse_where(tjd: c_double, ifl: c_int, geopos: *mut c_double, attr: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_sol_eclipse_how"]
    pub fn swe_sol_eclipse_how(tjd: c_double, ifl: c_int, geopos: *mut c_double, attr: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_sol_eclipse_when_loc"]
    pub fn swe_sol_eclipse_when_loc(tjd_start: c_double, ifl: c_int, geopos: *mut c_double, tret: *mut c_double, attr: *mut c_double, backward: c_int, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_sol_eclipse_when_glob"]
    pub fn swe_sol_eclipse_when_glob(tjd_start: c_double, ifl: c_int, ifltype: c_int, tret: *mut c_double, backward: c_int, serr: *mut c_char) -> c_int;

    #[link_name = "impl_swe_lun_eclipse_how"]
    pub fn swe_lun_eclipse_how(tjd_ut: c_double, ifl: c_int, geopos: *mut c_double, attr: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_lun_eclipse_when"]
    pub fn swe_lun_eclipse_when(tjd_start: c_double, ifl: c_int, ifltype: c_int, tret: *mut c_double, backward: c_int, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_lun_eclipse_when_loc"]
    pub fn swe_lun_eclipse_when_loc(tjd_start: c_double, ifl: c_int, geopos: *mut c_double, tret: *mut c_double, attr: *mut c_double, backward: c_int, serr: *mut c_char) -> c_int;
    
    #[link_name = "impl_swe_lun_occult_where"]
    pub fn swe_lun_occult_where(tjd: c_double, ipl: c_int, starname: *mut c_char, ifl: c_int, geopos: *mut c_double, attr: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_lun_occult_when_loc"]
    pub fn swe_lun_occult_when_loc(tjd_start: c_double, ipl: c_int, starname: *mut c_char, ifl: c_int, geopos: *mut c_double, tret: *mut c_double, attr: *mut c_double, backward: c_int, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_lun_occult_when_glob"]
    pub fn swe_lun_occult_when_glob(tjd_start: c_double, ipl: c_int, starname: *mut c_char, ifl: c_int, ifltype: c_int, tret: *mut c_double, backward: c_int, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_gauquelin_sector"]
    pub fn swe_gauquelin_sector(t_ut: c_double, ipl: c_int, starname: *mut c_char, iflag: c_int, imeth: c_int, geopos: *mut c_double, atpress: c_double, attemp: c_double, dgsect: *mut c_double, serr: *mut c_char) -> c_int;


    // --- Houses ---
    #[link_name = "impl_swe_houses"]
    pub fn swe_houses(tjd_ut: c_double, geolat: c_double, geolon: c_double, hsys: c_int, cusps: *mut c_double, ascmc: *mut c_double) -> c_int;
    #[link_name = "impl_swe_houses_ex"]
    pub fn swe_houses_ex(tjd_ut: c_double, iflag: c_int, geolat: c_double, geolon: c_double, hsys: c_int, cusps: *mut c_double, ascmc: *mut c_double) -> c_int;
    #[link_name = "impl_swe_houses_ex2"]
    pub fn swe_houses_ex2(tjd_ut: c_double, iflag: c_int, geolat: c_double, geolon: c_double, hsys: c_int, cusps: *mut c_double, ascmc: *mut c_double, cusp_speed: *mut c_double, ascmc_speed: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_houses_armc"]
    pub fn swe_houses_armc(armc: c_double, geolat: c_double, eps: c_double, hsys: c_int, cusps: *mut c_double, ascmc: *mut c_double) -> c_int;
    #[link_name = "impl_swe_houses_armc_ex2"]
    pub fn swe_houses_armc_ex2(armc: c_double, geolat: c_double, eps: c_double, hsys: c_int, cusps: *mut c_double, ascmc: *mut c_double, cusp_speed: *mut c_double, ascmc_speed: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_house_pos"]
    pub fn swe_house_pos(armc: c_double, geolat: c_double, eps: c_double, hsys: c_int, xpin: *mut c_double, serr: *mut c_char) -> c_double;
    #[link_name = "impl_swe_house_name"]
    pub fn swe_house_name(hsys: c_int) -> *mut c_char;


    // --- Phenomena ---
    #[link_name = "impl_swe_pheno"]
    pub fn swe_pheno(tjd: c_double, ipl: c_int, iflag: c_int, attr: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_pheno_ut"]
    pub fn swe_pheno_ut(tjd_ut: c_double, ipl: c_int, iflag: c_int, attr: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_get_planet_name"]
    pub fn swe_get_planet_name(ipl: c_int, spname: *mut c_char) -> *mut c_char;

    #[link_name = "impl_swe_nod_aps"]
    pub fn swe_nod_aps(tjd_et: c_double, ipl: c_int, iflag: c_int, method: c_int, xnasc: *mut c_double, xndsc: *mut c_double, xperi: *mut c_double, xaphe: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_nod_aps_ut"]
    pub fn swe_nod_aps_ut(tjd_ut: c_double, ipl: c_int, iflag: c_int, method: c_int, xnasc: *mut c_double, xndsc: *mut c_double, xperi: *mut c_double, xaphe: *mut c_double, serr: *mut c_char) -> c_int;

    #[link_name = "impl_swe_get_orbital_elements"]
    pub fn swe_get_orbital_elements(tjd_et: c_double, ipl: c_int, iflag: c_int, dret: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_orbit_max_min_true_distance"]
    pub fn swe_orbit_max_min_true_distance(tjd_et: c_double, ipl: c_int, iflag: c_int, dmax: *mut c_double, dmin: *mut c_double, dtrue: *mut c_double, serr: *mut c_char) -> c_int;

    #[link_name = "impl_swe_azalt"]
    pub fn swe_azalt(tjd_ut: c_double, calc_flag: c_int, geopos: *mut c_double, atpress: c_double, attemp: c_double, xin: *mut c_double, xaz: *mut c_double);
    #[link_name = "impl_swe_azalt_rev"]
    pub fn swe_azalt_rev(tjd_ut: c_double, calc_flag: c_int, geopos: *mut c_double, xin: *mut c_double, xout: *mut c_double);
    #[link_name = "impl_swe_rise_trans"]
    pub fn swe_rise_trans(tjd_ut: c_double, ipl: c_int, starname: *mut c_char, epheflag: c_int, rsmi: c_int, geopos: *mut c_double, atpress: c_double, attemp: c_double, tret: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_rise_trans_true_hor"]
    pub fn swe_rise_trans_true_hor(tjd_ut: c_double, ipl: c_int, starname: *mut c_char, epheflag: c_int, rsmi: c_int, geopos: *mut c_double, atpress: c_double, attemp: c_double, horhgt: c_double, tret: *mut c_double, serr: *mut c_char) -> c_int;

    #[link_name = "impl_swe_refrac"]
    pub fn swe_refrac(inalt: c_double, atpress: c_double, attemp: c_double, calc_flag: c_int) -> c_double;
    #[link_name = "impl_swe_refrac_extended"]
    pub fn swe_refrac_extended(inalt: c_double, geoalt: c_double, atpress: c_double, attemp: c_double, lapse_rate: c_double, calc_flag: c_int, dret: *mut c_double) -> c_double;


    // --- Heliacal ---
    #[link_name = "impl_swe_heliacal_ut"]
    pub fn swe_heliacal_ut(tjdstart: c_double, geopos: *mut c_double, datm: *mut c_double, dobs: *mut c_double, objectname: *mut c_char, event_type: c_int, iflag: c_int, dret: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_heliacal_pheno_ut"]
    pub fn swe_heliacal_pheno_ut(tjd: c_double, geopos: *mut c_double, datm: *mut c_double, dobs: *mut c_double, objectname: *mut c_char, event_type: c_int, helflag: c_int, darr: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_vis_limit_mag"]
    pub fn swe_vis_limit_mag(tjdut: c_double, geopos: *mut c_double, datm: *mut c_double, dobs: *mut c_double, objectname: *mut c_char, helflag: c_int, dret: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_heliacal_angle"]
    pub fn swe_heliacal_angle(tjdut: c_double, dgeo: *mut c_double, datm: *mut c_double, dobs: *mut c_double, helflag: c_int, mag: c_double, azi_obj: c_double, azi_sun: c_double, azi_moon: c_double, alt_moon: c_double, dret: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_topo_arcus_visionis"]
    pub fn swe_topo_arcus_visionis(tjdut: c_double, dgeo: *mut c_double, datm: *mut c_double, dobs: *mut c_double, helflag: c_int, mag: c_double, azi_obj: c_double, alt_obj: c_double, azi_sun: c_double, azi_moon: c_double, alt_moon: c_double, dret: *mut c_double, serr: *mut c_char) -> c_int;

    
    // --- Utilities & Math ---
    #[link_name = "impl_swe_deltat"]
    pub fn swe_deltat(tjd: c_double) -> c_double;
    #[link_name = "impl_swe_deltat_ex"]
    pub fn swe_deltat_ex(tjd: c_double, iflag: c_int, serr: *mut c_char) -> c_double;
    #[link_name = "impl_swe_cotrans"]
    pub fn swe_cotrans(xpo: *mut c_double, xpn: *mut c_double, eps: c_double);
    #[link_name = "impl_swe_cotrans_sp"]
    pub fn swe_cotrans_sp(xpo: *mut c_double, xpn: *mut c_double, eps: c_double);
    #[link_name = "impl_swe_get_tid_acc"]
    pub fn swe_get_tid_acc() -> c_double;
    
    #[link_name = "impl_swe_degnorm"]
    pub fn swe_degnorm(x: c_double) -> c_double;
    #[link_name = "impl_swe_radnorm"]
    pub fn swe_radnorm(x: c_double) -> c_double;
    #[link_name = "impl_swe_rad_midp"]
    pub fn swe_rad_midp(x1: c_double, x0: c_double) -> c_double;
    #[link_name = "impl_swe_deg_midp"]
    pub fn swe_deg_midp(x1: c_double, x0: c_double) -> c_double;
    
    #[link_name = "impl_swe_split_deg"]
    pub fn swe_split_deg(ddeg: c_double, roundflag: c_int, ideg: *mut c_int, imin: *mut c_int, isec: *mut c_int, dsecfr: *mut c_double, isgn: *mut c_int);
    
    #[link_name = "impl_swe_csnorm"]
    pub fn swe_csnorm(p: c_int) -> c_int;
    #[link_name = "impl_swe_difcsn"]
    pub fn swe_difcsn(p1: c_int, p2: c_int) -> c_int;
    #[link_name = "impl_swe_difdegn"]
    pub fn swe_difdegn(p1: c_double, p2: c_double) -> c_double;
    #[link_name = "impl_swe_difcs2n"]
    pub fn swe_difcs2n(p1: c_int, p2: c_int) -> c_int;
    #[link_name = "impl_swe_difdeg2n"]
    pub fn swe_difdeg2n(p1: c_double, p2: c_double) -> c_double;
    #[link_name = "impl_swe_difrad2n"]
    pub fn swe_difrad2n(p1: c_double, p2: c_double) -> c_double;
    #[link_name = "impl_swe_csroundsec"]
    pub fn swe_csroundsec(x: c_int) -> c_int;
    #[link_name = "impl_swe_d2l"]
    pub fn swe_d2l(x: c_double) -> c_int;
    #[link_name = "impl_swe_day_of_week"]
    pub fn swe_day_of_week(jd: c_double) -> c_int;
    
    #[link_name = "impl_swe_cs2timestr"]
    pub fn swe_cs2timestr(t: c_int, sep: c_int, suppress_zero: c_int, a: *mut c_char) -> *mut c_char;
    #[link_name = "impl_swe_cs2lonlatstr"]
    pub fn swe_cs2lonlatstr(t: c_int, pchar: c_char, mchar: c_char, s: *mut c_char) -> *mut c_char;
    #[link_name = "impl_swe_cs2degstr"]
    pub fn swe_cs2degstr(t: c_int, a: *mut c_char) -> *mut c_char;


    // --- Info ---
    #[link_name = "impl_swe_get_ayanamsa"]
    pub fn swe_get_ayanamsa(tjd_et: c_double) -> c_double;
    #[link_name = "impl_swe_get_ayanamsa_ut"]
    pub fn swe_get_ayanamsa_ut(tjd_ut: c_double) -> c_double;
    #[link_name = "impl_swe_get_ayanamsa_ex"]
    pub fn swe_get_ayanamsa_ex(tjd_et: c_double, iflag: c_int, daya: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_get_ayanamsa_ex_ut"]
    pub fn swe_get_ayanamsa_ex_ut(tjd_ut: c_double, iflag: c_int, daya: *mut c_double, serr: *mut c_char) -> c_int;
    #[link_name = "impl_swe_get_ayanamsa_name"]
    pub fn swe_get_ayanamsa_name(isidmode: c_int) -> *const c_char;
    #[link_name = "impl_swe_get_current_file_data"]
    pub fn swe_get_current_file_data(ifno: c_int, tfstart: *mut c_double, tfend: *mut c_double, denum: *mut c_int) -> *const c_char;
}
