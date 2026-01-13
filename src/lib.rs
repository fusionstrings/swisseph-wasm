#![no_std]
extern crate alloc;

use wasm_bindgen::prelude::*;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::CStr;
use js_sys::{Object, Reflect, Array};

// Include generated Swiss Ephemeris bindings
pub mod bindings;

// --- Constants (Exported via src/constants.ts, not wasm_bindgen) ---


#[allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case)]
pub mod swe_bindings {
    pub use crate::bindings::*;
    // Internal usage constants if needed, but we can access the pub consts above from crate root too.
    // However, swe_bindings was used for bindings::* re-export.
}


// --- Helper Functions ---

fn cstr_buf() -> [i8; 256] {
    [0i8; 256]
}

fn err_to_string(serr: &[i8]) -> String {
    unsafe { CStr::from_ptr(serr.as_ptr()).to_str().unwrap_or("Unknown error").to_string() }
}

fn string_to_c_ptr(s: &str) -> (Vec<u8>, *mut i8) {
    let mut bytes = s.as_bytes().to_vec();
    bytes.push(0);
    let ptr = bytes.as_mut_ptr() as *mut i8;
    (bytes, ptr)
}


// --- API EXPORTS ---

#[wasm_bindgen(js_name = swe_version)]
pub fn js_swe_version() -> String {
    let mut s = [0i8; 256];
    unsafe {
        swe_bindings::swe_version(s.as_mut_ptr());
        CStr::from_ptr(s.as_ptr()).to_str().unwrap_or("").to_string()
    }
}

#[wasm_bindgen(js_name = swe_close)]
pub fn js_swe_close() {
    unsafe { swe_bindings::swe_close(); }
}

// --- Calculation ---

/// Calculate planetary position (UT).
/// 
/// @param tjd_ut Julian Day (UT).
/// @param ipl Planet ID (0=Sun, 1=Moon... see constants).
/// @param iflag Calculation flags (e.g. SEFLG_SWIEPH | SEFLG_SPEED).
/// @returns Object { longitude, latitude, distance, speed_long, speed_lat, speed_dist, rc_flags }.
#[wasm_bindgen(js_name = swe_calc_ut)]
pub fn js_swe_calc_ut(tjd_ut: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_calc_ut(tjd_ut, ipl, iflag, xx.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    Reflect::set(&obj, &"longitude".into(), &xx[0].into())?;
    Reflect::set(&obj, &"latitude".into(), &xx[1].into())?;
    Reflect::set(&obj, &"distance".into(), &xx[2].into())?;
    Reflect::set(&obj, &"speed_long".into(), &xx[3].into())?;
    Reflect::set(&obj, &"speed_lat".into(), &xx[4].into())?;
    Reflect::set(&obj, &"speed_dist".into(), &xx[5].into())?;
    Reflect::set(&obj, &"rc_flags".into(), &ret.into())?;
    Ok(obj.into())
}

/// Calculate planetary position (ET).
/// 
/// @param tjd Julian Day (ET).
/// @param ipl Planet ID.
/// @param iflag Flags.
/// @returns Object { longitude, latitude, distance, ... }.
#[wasm_bindgen(js_name = swe_calc)]
pub fn js_swe_calc(tjd: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_calc(tjd, ipl, iflag, xx.as_mut_ptr(), serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    Reflect::set(&obj, &"longitude".into(), &xx[0].into())?;
    Reflect::set(&obj, &"latitude".into(), &xx[1].into())?;
    Reflect::set(&obj, &"distance".into(), &xx[2].into())?;
    Reflect::set(&obj, &"speed_long".into(), &xx[3].into())?;
    Reflect::set(&obj, &"speed_lat".into(), &xx[4].into())?;
    Reflect::set(&obj, &"speed_dist".into(), &xx[5].into())?;
    Reflect::set(&obj, &"rc_flags".into(), &ret.into())?;
    Ok(obj.into())
}

// --- Date & Time ---

/// Calculate Julian Day from calendar date.
/// 
/// @param year Year (e.g. 2000).
/// @param month Month (1-12).
/// @param day Day (1-31).
/// @param hour Hour (decimal, e.g. 12.5).
/// @param gregflag Calendar flag (0=Julian, 1=Gregorian).
/// @returns Julian Day number.
#[wasm_bindgen(js_name = swe_julday)]
pub fn js_swe_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: i32) -> f64 {
    unsafe { swe_bindings::swe_julday(year, month, day, hour, gregflag) }
}

/// Calculate Calendar Date from Julian Day.
/// 
/// @param tjd Julian Day number.
/// @param gregflag Calendar flag (0=Julian, 1=Gregorian).
/// @returns Object { year, month, day, hour }.
#[wasm_bindgen(js_name = swe_revjul)]
pub fn js_swe_revjul(tjd: f64, gregflag: i32) -> JsValue {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0.0;
    unsafe { swe_bindings::swe_revjul(tjd, gregflag, &mut year, &mut month, &mut day, &mut hour); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"year".into(), &year.into());
    let _ = Reflect::set(&obj, &"month".into(), &month.into());
    let _ = Reflect::set(&obj, &"day".into(), &day.into());
    let _ = Reflect::set(&obj, &"hour".into(), &hour.into());
    obj.into()
}

/// Calculate Sidereal Time.
///
/// @param tjd_ut Julian Day (UT).
/// @returns Sidereal Time (hours).
#[wasm_bindgen(js_name = swe_sidtime)]
pub fn js_swe_sidtime(tjd_ut: f64) -> f64 {
    unsafe { swe_bindings::swe_sidtime(tjd_ut) }
}


// --- Houses ---

/// Calculate House Cusps and Ascendant/MC.
///
/// @param tjd_ut Julian Day (UT).
/// @param geolat Geographic Latitude (negative for South).
/// @param geolon Geographic Longitude (negative for West).
/// @param hsys House System char (e.g. 'P' for Placidus).
/// @returns Object { cusps: [13], ascmc: [10], ascendant, mc, armc, vertex, eqasc, ... }.
#[wasm_bindgen(js_name = swe_houses)]
pub fn js_swe_houses(tjd_ut: f64, geolat: f64, geolon: f64, hsys: String) -> Result<JsValue, JsValue> {
    let hsys_char = hsys.chars().next().unwrap_or('P') as i32;
    let mut cusps = [0.0; 13];
    let mut ascmc = [0.0; 10];
    
    let ret = unsafe {
        swe_bindings::swe_houses(tjd_ut, geolat, geolon, hsys_char, cusps.as_mut_ptr(), ascmc.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str("swe_houses failed")); }
    
    let obj = Object::new();
    let js_cusps = Array::new();
    for i in 1..=12 { js_cusps.push(&cusps[i].into()); }
    
    let _ = Reflect::set(&obj, &"cusps".into(), &js_cusps);
    let _ = Reflect::set(&obj, &"ascendant".into(), &ascmc[0].into());
    let _ = Reflect::set(&obj, &"mc".into(), &ascmc[1].into());
    let _ = Reflect::set(&obj, &"armc".into(), &ascmc[2].into());
    let _ = Reflect::set(&obj, &"vertex".into(), &ascmc[3].into());
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_houses_ex)]
pub fn js_swe_houses_ex(tjd_ut: f64, iflag: i32, geolat: f64, geolon: f64, hsys: String) -> Result<JsValue, JsValue> {
    let hsys_char = hsys.chars().next().unwrap_or('P') as i32;
    let mut cusps = [0.0; 13];
    let mut ascmc = [0.0; 10];
    
    let ret = unsafe {
        swe_bindings::swe_houses_ex(tjd_ut, iflag, geolat, geolon, hsys_char, cusps.as_mut_ptr(), ascmc.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str("swe_houses_ex failed")); }
    
    let obj = Object::new();
    let js_cusps = Array::new();
    for i in 1..=12 { js_cusps.push(&cusps[i].into()); }
    
    let _ = Reflect::set(&obj, &"cusps".into(), &js_cusps);
    let _ = Reflect::set(&obj, &"ascendant".into(), &ascmc[0].into());
    let _ = Reflect::set(&obj, &"mc".into(), &ascmc[1].into());
    let _ = Reflect::set(&obj, &"armc".into(), &ascmc[2].into());
    let _ = Reflect::set(&obj, &"vertex".into(), &ascmc[3].into());
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_house_pos)]
pub fn js_swe_house_pos(armc: f64, geolat: f64, eps: f64, hsys: String) -> Result<f64, JsValue> {
    let hsys_char = hsys.chars().next().unwrap_or('P') as i32;
    let mut xpin = [0.0; 2];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_house_pos(armc, geolat, eps, hsys_char, xpin.as_mut_ptr(), serr.as_mut_ptr())
    };
    if ret == 0.0 && unsafe { serr[0] != 0 } {
         return Err(JsValue::from_str(&err_to_string(&serr)));
    }
    Ok(ret)
}


// --- Eclipse & Occultation ---

#[wasm_bindgen(js_name = swe_sol_eclipse_where)]
pub fn js_swe_sol_eclipse_where(tjd: f64, ifl: i32) -> Result<JsValue, JsValue> {
    let mut geopos = [0.0; 2];
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_sol_eclipse_where(tjd, ifl, geopos.as_mut_ptr(), attr.as_mut_ptr(), serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"lon".into(), &geopos[0].into());
    let _ = Reflect::set(&obj, &"lat".into(), &geopos[1].into());
    let _ = Reflect::set(&obj, &"attr".into(), &serde_to_js_array(&attr));
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_sol_eclipse_when_glob)]
pub fn js_swe_sol_eclipse_when_glob(tjd_start: f64, ifl: i32, ifltype: i32, backward: i32) -> Result<JsValue, JsValue> {
    let mut tret = [0.0; 10];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_sol_eclipse_when_glob(tjd_start, ifl, ifltype, tret.as_mut_ptr(), backward, serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"tret".into(), &serde_to_js_array(&tret));
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_sol_eclipse_when_loc)]
pub fn js_swe_sol_eclipse_when_loc(tjd_start: f64, ifl: i32, geolon: f64, geolat: f64, backward: i32) -> Result<JsValue, JsValue> {
    let mut geopos = [geolon, geolat, 0.0];
    let mut tret = [0.0; 10];
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_sol_eclipse_when_loc(tjd_start, ifl, geopos.as_mut_ptr(), tret.as_mut_ptr(), attr.as_mut_ptr(), backward, serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"tret".into(), &serde_to_js_array(&tret));
    let _ = Reflect::set(&obj, &"attr".into(), &serde_to_js_array(&attr));
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    Ok(obj.into())
}


fn serde_to_js_array(slice: &[f64]) -> Array {
    let arr = Array::new();
    for &val in slice { arr.push(&val.into()); }
    arr
}

// See strict mode implementation below...


// --- Heliacal ---

#[wasm_bindgen(js_name = swe_heliacal_ut)]
pub fn js_swe_heliacal_ut(tjdstart: f64, geo_lon: f64, geo_lat: f64, geo_alt: f64, 
                          atm_press: f64, atm_temp: f64, atm_humid: f64, atm_vis: f64,
                          obs_age: f64, obs_snellen: f64,
                          object_name: String, event_type: i32, iflag: i32) -> Result<JsValue, JsValue> {
    
    let mut dgeo = [geo_lon, geo_lat, geo_alt];
    let mut datm = [atm_press, atm_temp, atm_humid, atm_vis];
    let mut dobs = [obs_age, obs_snellen, 0.0, 0.0, 0.0, 0.0];
    let mut dret = [0.0; 50];
    let mut serr = cstr_buf();
    
    let (obj_bytes, obj_ptr) = string_to_c_ptr(&object_name);
    
    let ret = unsafe {
        swe_bindings::swe_heliacal_ut(tjdstart, dgeo.as_mut_ptr(), datm.as_mut_ptr(), dobs.as_mut_ptr(), 
                            obj_ptr, event_type, iflag, dret.as_mut_ptr(), serr.as_mut_ptr())
    };
    drop(obj_bytes);
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"start_visible".into(), &dret[0].into());
    let _ = Reflect::set(&obj, &"best_visible".into(), &dret[1].into());
    let _ = Reflect::set(&obj, &"end_visible".into(), &dret[2].into());
    let _ = Reflect::set(&obj, &"visibility_duration".into(), &((dret[2] - dret[0]) * 24.0 * 60.0).into()); // minutes
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_vis_limit_mag)]
pub fn js_swe_vis_limit_mag(tjdut: f64, geo_lon: f64, geo_lat: f64, geo_alt: f64, 
                          atm_press: f64, atm_temp: f64, atm_humid: f64, atm_vis: f64,
                          obs_age: f64, obs_snellen: f64,
                          object_name: String, helflag: i32) -> Result<f64, JsValue> {
    
    let mut dgeo = [geo_lon, geo_lat, geo_alt];
    let mut datm = [atm_press, atm_temp, atm_humid, atm_vis];
    let mut dobs = [obs_age, obs_snellen, 0.0, 0.0, 0.0, 0.0];
    let mut dret = [0.0; 50];
    let mut serr = cstr_buf();
    
    let (obj_bytes, obj_ptr) = string_to_c_ptr(&object_name);

    let ret = unsafe {
        swe_bindings::swe_vis_limit_mag(tjdut, dgeo.as_mut_ptr(), datm.as_mut_ptr(), dobs.as_mut_ptr(),
                              obj_ptr, helflag, dret.as_mut_ptr(), serr.as_mut_ptr())
    };
    drop(obj_bytes);
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    // dret[0] is limiting magnitude (vmag)
    Ok(dret[0])
}


// --- Fixed Stars ---

#[wasm_bindgen(js_name = swe_fixstar_ut)]
pub fn js_swe_fixstar_ut(star: String, tjd_ut: f64, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = cstr_buf();
    
    // Star name buffer (must be large enough for returned name)
    let mut star_bytes = alloc::vec![0u8; 256]; 
    let s_in = star.as_bytes();
    for (i, &b) in s_in.iter().enumerate().take(255) { star_bytes[i] = b; }
    
    let ret = unsafe {
        swe_bindings::swe_fixstar_ut(star_bytes.as_mut_ptr() as *mut i8, tjd_ut, iflag, xx.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let returned_name = unsafe { CStr::from_ptr(star_bytes.as_ptr() as *const i8).to_str().unwrap_or(&star).to_string() };

    let obj = Object::new();
    let _ = Reflect::set(&obj, &"name".into(), &returned_name.into());
    let _ = Reflect::set(&obj, &"longitude".into(), &xx[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xx[1].into());
    let _ = Reflect::set(&obj, &"distance".into(), &xx[2].into());
    let _ = Reflect::set(&obj, &"rc_flags".into(), &ret.into());
    Ok(obj.into())
}


// --- Phenomena ---

#[wasm_bindgen(js_name = swe_pheno_ut)]
pub fn js_swe_pheno_ut(tjd_ut: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_pheno_ut(tjd_ut, ipl, iflag, attr.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"phase_angle".into(), &attr[0].into());
    let _ = Reflect::set(&obj, &"phase".into(), &attr[1].into());
    let _ = Reflect::set(&obj, &"elongation".into(), &attr[2].into());
    let _ = Reflect::set(&obj, &"diameter".into(), &attr[3].into()); // apparent diameter
    let _ = Reflect::set(&obj, &"magnitude".into(), &attr[4].into());
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_nod_aps_ut)]
pub fn js_swe_nod_aps_ut(tjd_ut: f64, ipl: i32, iflag: i32, method: i32) -> Result<JsValue, JsValue> {
    let mut xnasc = [0.0; 6];
    let mut xndsc = [0.0; 6];
    let mut xperi = [0.0; 6];
    let mut xaphe = [0.0; 6];
    let mut serr = cstr_buf();

    let ret = unsafe {
        swe_bindings::swe_nod_aps_ut(tjd_ut, ipl, iflag, method, 
            xnasc.as_mut_ptr(), xndsc.as_mut_ptr(), xperi.as_mut_ptr(), xaphe.as_mut_ptr(),
            serr.as_mut_ptr())
    };

    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"node_ascending".into(), &serde_to_js_array(&xnasc));
    let _ = Reflect::set(&obj, &"node_descending".into(), &serde_to_js_array(&xndsc));
    let _ = Reflect::set(&obj, &"perihelion".into(), &serde_to_js_array(&xperi));
    let _ = Reflect::set(&obj, &"aphelion".into(), &serde_to_js_array(&xaphe));
    Ok(obj.into())
}


// --- Utilities ---

#[wasm_bindgen(js_name = swe_deltat)]
pub fn js_swe_deltat(tjd: f64) -> f64 {
    unsafe { swe_bindings::swe_deltat(tjd) }
}

#[wasm_bindgen(js_name = swe_split_deg)]
pub fn js_swe_split_deg(ddeg: f64, round_flag: i32) -> JsValue {
    let mut ideg = 0;
    let mut imin = 0;
    let mut isec = 0;
    let mut dsecfr = 0.0;
    let mut isgn = 0;
    
    unsafe { swe_bindings::swe_split_deg(ddeg, round_flag, &mut ideg, &mut imin, &mut isec, &mut dsecfr, &mut isgn); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"degree".into(), &ideg.into());
    let _ = Reflect::set(&obj, &"min".into(), &imin.into());
    let _ = Reflect::set(&obj, &"sec".into(), &isec.into());
    let _ = Reflect::set(&obj, &"sec_frac".into(), &dsecfr.into());
     let _ = Reflect::set(&obj, &"sign".into(), &isgn.into());
    obj.into()
}

#[wasm_bindgen(js_name = swe_get_ayanamsa_ut)]
pub fn js_swe_get_ayanamsa_ut(tjd_ut: f64) -> f64 {
    unsafe { swe_bindings::swe_get_ayanamsa_ut(tjd_ut) }
}

#[wasm_bindgen(js_name = swe_get_planet_name)]
pub fn js_swe_get_planet_name(ipl: i32) -> String {
    let mut spname = [0i8; 256];
    unsafe {
        swe_bindings::swe_get_planet_name(ipl, spname.as_mut_ptr());
        CStr::from_ptr(spname.as_ptr()).to_str().unwrap_or("Unknown").to_string()
    }
}

#[wasm_bindgen(js_name = swe_set_topo)]
pub fn js_swe_set_topo(geolon: f64, geolat: f64, geoalt: f64) {
    unsafe { swe_bindings::swe_set_topo(geolon, geolat, geoalt); }
}

#[wasm_bindgen(js_name = swe_set_sid_mode)]
pub fn js_swe_set_sid_mode(sid_mode: i32, t0: f64, ayan_t0: f64) {
    unsafe { swe_bindings::swe_set_sid_mode(sid_mode, t0, ayan_t0); }
}

// --- Extended API (Rise/Set, AzAlt, Lunar Eclipses) ---

#[wasm_bindgen(js_name = swe_azalt)]
pub fn js_swe_azalt(tjd_ut: f64, calc_flag: i32, geolon: f64, geolat: f64, geoalt: f64, atpress: f64, attemp: f64, xin_lon: f64, xin_lat: f64, xin_dist: f64) -> JsValue {
    let mut xaz = [0.0; 3];
    let mut xin = [xin_lon, xin_lat, xin_dist];
    let mut geopos = [geolon, geolat, geoalt];
    
    unsafe {
        swe_bindings::swe_azalt(tjd_ut, calc_flag, geopos.as_mut_ptr(), atpress, attemp, xin.as_mut_ptr(), xaz.as_mut_ptr())
    };
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"azimuth".into(), &xaz[0].into());
    let _ = Reflect::set(&obj, &"true_altitude".into(), &xaz[1].into());
    let _ = Reflect::set(&obj, &"apparent_altitude".into(), &xaz[2].into());
    obj.into()
}

#[wasm_bindgen(js_name = swe_azalt_rev)]
pub fn js_swe_azalt_rev(tjd_ut: f64, calc_flag: i32, geolon: f64, geolat: f64, geoalt: f64, xin_az: f64, xin_alt: f64) -> JsValue {
    let mut xout = [0.0; 2];
    let mut xin = [xin_az, xin_alt];
    let mut geopos = [geolon, geolat, geoalt];
    
    unsafe {
        swe_bindings::swe_azalt_rev(tjd_ut, calc_flag, geopos.as_mut_ptr(), xin.as_mut_ptr(), xout.as_mut_ptr())
    };
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"longitude".into(), &xout[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xout[1].into());
    obj.into()
}

#[wasm_bindgen(js_name = swe_rise_trans)]
pub fn js_swe_rise_trans(tjd_ut: f64, ipl: i32, starname: Option<String>, epheflag: i32, rsmi: i32, geolon: f64, geolat: f64, geoalt: f64, atpress: f64, attemp: f64) -> Result<JsValue, JsValue> {
    let mut tret = 0.0;
    let mut serr = cstr_buf();
    let mut geopos = [geolon, geolat, geoalt];
    let (_star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
    let ret = unsafe {
        swe_bindings::swe_rise_trans(tjd_ut, ipl, star_ptr, epheflag, rsmi, geopos.as_mut_ptr(), atpress, attemp, &mut tret, serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    // Pass ret (flag) and tret (time)
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"tret".into(), &tret.into());
    let _ = Reflect::set(&obj, &"status".into(), &ret.into());
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_rise_trans_true_hor)]
pub fn js_swe_rise_trans_true_hor(tjd_ut: f64, ipl: i32, starname: Option<String>, epheflag: i32, rsmi: i32, geolon: f64, geolat: f64, geoalt: f64, atpress: f64, attemp: f64, horhgt: f64) -> Result<JsValue, JsValue> {
    let mut tret = 0.0;
    let mut serr = cstr_buf();
    let mut geopos = [geolon, geolat, geoalt];
    let (_star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
    let ret = unsafe {
        swe_bindings::swe_rise_trans_true_hor(tjd_ut, ipl, star_ptr, epheflag, rsmi, geopos.as_mut_ptr(), atpress, attemp, horhgt, &mut tret, serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"tret".into(), &tret.into());
    let _ = Reflect::set(&obj, &"status".into(), &ret.into());
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_lun_eclipse_when)]
pub fn js_swe_lun_eclipse_when(tjd_start: f64, iflag: i32, ifltype: i32) -> Result<JsValue, JsValue> {
    let mut tret = [0.0; 10];
    let backward = 0;
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_lun_eclipse_when(tjd_start, iflag, ifltype, tret.as_mut_ptr(), backward, serr.as_mut_ptr())

    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"ret".into(), &ret.into());
     let _ = Reflect::set(&obj, &"tret".into(), &serde_to_js_array(&tret));
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_lun_eclipse_how)]
pub fn js_swe_lun_eclipse_how(tjd_ut: f64, iflag: i32, geolon: f64, geolat: f64, geoalt: f64) -> Result<JsValue, JsValue> {
     let mut attr = [0.0; 20];
     let mut serr = cstr_buf();
     let mut geopos = [geolon, geolat, geoalt];
     
     let ret = unsafe {
         swe_bindings::swe_lun_eclipse_how(tjd_ut, iflag, geopos.as_mut_ptr(), attr.as_mut_ptr(), serr.as_mut_ptr())
     };
     if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
     
     // Mirroring swe_sol_eclipse_how output structure roughly
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"magnitude".into(), &attr[0].into()); // or similar
    let _ = Reflect::set(&obj, &"attributes".into(), &serde_to_js_array(&attr));
    Ok(obj.into())
}

#[wasm_bindgen(js_name = swe_cotrans)]
pub fn js_swe_cotrans(xin_lon: f64, xin_lat: f64, xin_dist: f64, eps: f64) -> JsValue {
    let mut xout = [0.0; 3];
    let mut xin = [xin_lon, xin_lat, xin_dist];

    unsafe { swe_bindings::swe_cotrans(xin.as_mut_ptr(), xout.as_mut_ptr(), eps); }

    let obj = Object::new();
    let _ = Reflect::set(&obj, &"longitude".into(), &xout[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xout[1].into());
    let _ = Reflect::set(&obj, &"distance".into(), &xout[2].into());
    obj.into()
}

#[wasm_bindgen(js_name = swe_cotrans_sp)]
pub fn js_swe_cotrans_sp(xin_lon: f64, xin_lat: f64, xin_dist: f64, eps: f64) -> JsValue {
    // Check signature: cotrans_sp usually similar
     let mut xout = [0.0; 3];
    let mut xin = [xin_lon, xin_lat, xin_dist];

    unsafe { swe_bindings::swe_cotrans_sp(xin.as_mut_ptr(), xout.as_mut_ptr(), eps); }

    let obj = Object::new();
    let _ = Reflect::set(&obj, &"longitude".into(), &xout[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xout[1].into());
    let _ = Reflect::set(&obj, &"distance".into(), &xout[2].into());
    obj.into()
}

#[wasm_bindgen(js_name = swe_refrac_extended)]
pub fn js_swe_refrac_extended(inalt: f64, geoalt: f64, atpress: f64, attemp: f64, lapse: f64, calc_flag: i32) -> f64 {
    let mut dret = [0.0; 4];
    unsafe { swe_bindings::swe_refrac_extended(inalt, geoalt, atpress, attemp, lapse, calc_flag, dret.as_mut_ptr()) }
}

#[wasm_bindgen(js_name = swe_refrac)]
pub fn js_swe_refrac(inalt: f64, atpress: f64, attemp: f64, calc_flag: i32) -> f64 {
    unsafe { swe_bindings::swe_refrac(inalt, atpress, attemp, calc_flag) }
}


// --- Tier 1: Critical Missing APIs ---

/// Normalize degrees to range [0, 360).
///
/// @param x Degrees to normalize.
/// @returns Normalized degrees in [0, 360).
#[wasm_bindgen(js_name = swe_degnorm)]
pub fn js_swe_degnorm(x: f64) -> f64 {
    unsafe { swe_bindings::swe_degnorm(x) }
}

/// Normalize radians to range [0, 2π).
///
/// @param x Radians to normalize.
/// @returns Normalized radians in [0, 2π).
#[wasm_bindgen(js_name = swe_radnorm)]
pub fn js_swe_radnorm(x: f64) -> f64 {
    unsafe { swe_bindings::swe_radnorm(x) }
}

/// Get day of week from Julian Day.
///
/// @param jd Julian Day number.
/// @returns Day of week (0=Monday, 1=Tuesday, ..., 6=Sunday).
#[wasm_bindgen(js_name = swe_day_of_week)]
pub fn js_swe_day_of_week(jd: f64) -> i32 {
    unsafe { swe_bindings::swe_day_of_week(jd) }
}

/// Get house system name.
///
/// @param hsys House system code (e.g. 'P' for Placidus, 'K' for Koch).
/// @returns House system name string.
#[wasm_bindgen(js_name = swe_house_name)]
pub fn js_swe_house_name(hsys: String) -> String {
    let hsys_char = hsys.chars().next().unwrap_or('P') as i32;
    unsafe {
        let ptr = swe_bindings::swe_house_name(hsys_char);
        if ptr.is_null() {
            return "Unknown".to_string();
        }
        CStr::from_ptr(ptr).to_str().unwrap_or("Unknown").to_string()
    }
}

/// Get ayanamsa (sidereal mode) name.
///
/// @param isidmode Sidereal mode ID (e.g. 0=Fagan-Bradley, 1=Lahiri).
/// @returns Ayanamsa system name string.
#[wasm_bindgen(js_name = swe_get_ayanamsa_name)]
pub fn js_swe_get_ayanamsa_name(isidmode: i32) -> String {
    unsafe {
        let ptr = swe_bindings::swe_get_ayanamsa_name(isidmode);
        if ptr.is_null() {
            return "Unknown".to_string();
        }
        CStr::from_ptr(ptr).to_str().unwrap_or("Unknown").to_string()
    }
}

/// Convert UTC to Julian Day.
///
/// @param year Year.
/// @param month Month (1-12).
/// @param day Day (1-31).
/// @param hour Hour (0-23).
/// @param min Minute (0-59).
/// @param sec Second (0-59.999...).
/// @param gregflag Calendar (0=Julian, 1=Gregorian).
/// @returns Object { jd_et, jd_ut } or error.
#[wasm_bindgen(js_name = swe_utc_to_jd)]
pub fn js_swe_utc_to_jd(year: i32, month: i32, day: i32, hour: i32, min: i32, sec: f64, gregflag: i32) -> Result<JsValue, JsValue> {
    let mut dret = [0.0; 2];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_utc_to_jd(year, month, day, hour, min, sec, gregflag, dret.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"jd_et".into(), &dret[0].into());
    let _ = Reflect::set(&obj, &"jd_ut".into(), &dret[1].into());
    Ok(obj.into())
}

/// Convert Julian Day (ET) to UTC.
///
/// @param tjd_et Julian Day in Ephemeris Time.
/// @param gregflag Calendar (0=Julian, 1=Gregorian).
/// @returns Object { year, month, day, hour, min, sec }.
#[wasm_bindgen(js_name = swe_jdet_to_utc)]
pub fn js_swe_jdet_to_utc(tjd_et: f64, gregflag: i32) -> JsValue {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0;
    let mut min = 0;
    let mut sec = 0.0;
    
    unsafe {
        swe_bindings::swe_jdet_to_utc(tjd_et, gregflag, &mut year, &mut month, &mut day, &mut hour, &mut min, &mut sec);
    }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"year".into(), &year.into());
    let _ = Reflect::set(&obj, &"month".into(), &month.into());
    let _ = Reflect::set(&obj, &"day".into(), &day.into());
    let _ = Reflect::set(&obj, &"hour".into(), &hour.into());
    let _ = Reflect::set(&obj, &"min".into(), &min.into());
    let _ = Reflect::set(&obj, &"sec".into(), &sec.into());
    obj.into()
}

/// Convert Julian Day (UT1) to UTC.
///
/// @param tjd_ut Julian Day in Universal Time.
/// @param gregflag Calendar (0=Julian, 1=Gregorian).
/// @returns Object { year, month, day, hour, min, sec }.
#[wasm_bindgen(js_name = swe_jdut1_to_utc)]
pub fn js_swe_jdut1_to_utc(tjd_ut: f64, gregflag: i32) -> JsValue {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0;
    let mut min = 0;
    let mut sec = 0.0;
    
    unsafe {
        swe_bindings::swe_jdut1_to_utc(tjd_ut, gregflag, &mut year, &mut month, &mut day, &mut hour, &mut min, &mut sec);
    }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"year".into(), &year.into());
    let _ = Reflect::set(&obj, &"month".into(), &month.into());
    let _ = Reflect::set(&obj, &"day".into(), &day.into());
    let _ = Reflect::set(&obj, &"hour".into(), &hour.into());
    let _ = Reflect::set(&obj, &"min".into(), &min.into());
    let _ = Reflect::set(&obj, &"sec".into(), &sec.into());
    obj.into()
}

/// Calculate solar eclipse attributes at a geographic location.
///
/// @param tjd Julian Day (UT).
/// @param ifl Ephemeris flags.
/// @param geolon Geographic longitude.
/// @param geolat Geographic latitude.
/// @param geoalt Geographic altitude (meters).
/// @returns Object { flags, magnitude, fraction_covered, ... } or error.
#[wasm_bindgen(js_name = swe_sol_eclipse_how)]
pub fn js_swe_sol_eclipse_how(tjd: f64, ifl: i32, geolon: f64, geolat: f64, geoalt: f64) -> Result<JsValue, JsValue> {
    let mut geopos = [geolon, geolat, geoalt];
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_sol_eclipse_how(tjd, ifl, geopos.as_mut_ptr(), attr.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    let _ = Reflect::set(&obj, &"eclipse_fraction".into(), &attr[0].into());
    let _ = Reflect::set(&obj, &"lunar_diameter_ratio".into(), &attr[1].into());
    let _ = Reflect::set(&obj, &"solar_diameter".into(), &attr[2].into());
    let _ = Reflect::set(&obj, &"attr".into(), &serde_to_js_array(&attr));
    Ok(obj.into())
}


// --- Tier 2: Important Missing APIs ---

/// Find time when Sun crosses a specific longitude (UT).
///
/// @param x2cross Longitude to cross (degrees).
/// @param jd_ut Start Julian Day (UT).
/// @param flag Ephemeris flags.
/// @returns Julian Day of crossing, or error.
#[wasm_bindgen(js_name = swe_solcross_ut)]
pub fn js_swe_solcross_ut(x2cross: f64, jd_ut: f64, flag: i32) -> Result<f64, JsValue> {
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_solcross_ut(x2cross, jd_ut, flag, serr.as_mut_ptr())
    };
    
    if ret < 0.0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(ret)
}

/// Find time when Moon crosses a specific longitude (UT).
///
/// @param x2cross Longitude to cross (degrees).
/// @param jd_ut Start Julian Day (UT).
/// @param flag Ephemeris flags.
/// @returns Julian Day of crossing, or error.
#[wasm_bindgen(js_name = swe_mooncross_ut)]
pub fn js_swe_mooncross_ut(x2cross: f64, jd_ut: f64, flag: i32) -> Result<f64, JsValue> {
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_mooncross_ut(x2cross, jd_ut, flag, serr.as_mut_ptr())
    };
    
    if ret < 0.0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(ret)
}

/// Find time when Moon crosses its ascending or descending node (UT).
///
/// @param jd_ut Start Julian Day (UT).
/// @param flag Ephemeris flags.
/// @returns Object { jd, xlon, xlat } or error.
#[wasm_bindgen(js_name = swe_mooncross_node_ut)]
pub fn js_swe_mooncross_node_ut(jd_ut: f64, flag: i32) -> Result<JsValue, JsValue> {
    let mut xlon = 0.0;
    let mut xlat = 0.0;
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_mooncross_node_ut(jd_ut, flag, &mut xlon, &mut xlat, serr.as_mut_ptr())
    };
    
    if ret < 0.0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"jd".into(), &ret.into());
    let _ = Reflect::set(&obj, &"xlon".into(), &xlon.into());
    let _ = Reflect::set(&obj, &"xlat".into(), &xlat.into());
    Ok(obj.into())
}

/// Get Keplerian orbital elements.
///
/// @param tjd_et Julian Day (ET).
/// @param ipl Planet ID.
/// @param iflag Calculation flags.
/// @returns Object with orbital elements array, or error.
#[wasm_bindgen(js_name = swe_get_orbital_elements)]
pub fn js_swe_get_orbital_elements(tjd_et: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut dret = [0.0; 50];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_get_orbital_elements(tjd_et, ipl, iflag, dret.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"semi_major_axis".into(), &dret[0].into());
    let _ = Reflect::set(&obj, &"eccentricity".into(), &dret[1].into());
    let _ = Reflect::set(&obj, &"inclination".into(), &dret[2].into());
    let _ = Reflect::set(&obj, &"ascending_node".into(), &dret[3].into());
    let _ = Reflect::set(&obj, &"perihelion".into(), &dret[4].into());
    let _ = Reflect::set(&obj, &"mean_anomaly".into(), &dret[5].into());
    let _ = Reflect::set(&obj, &"elements".into(), &serde_to_js_array(&dret));
    Ok(obj.into())
}

/// Get fixed star magnitude.
///
/// @param star Star name or designation.
/// @returns Magnitude value, or error.
#[wasm_bindgen(js_name = swe_fixstar_mag)]
pub fn js_swe_fixstar_mag(star: String) -> Result<f64, JsValue> {
    let mut mag = 0.0;
    let mut serr = cstr_buf();
    
    let mut star_bytes = alloc::vec![0u8; 256]; 
    let s_in = star.as_bytes();
    for (i, &b) in s_in.iter().enumerate().take(255) { star_bytes[i] = b; }
    
    let ret = unsafe {
        swe_bindings::swe_fixstar_mag(star_bytes.as_mut_ptr() as *mut i8, &mut mag, serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(mag)
}

/// Calculate midpoint of two degree values (shortest arc).
///
/// @param x1 First degree value.
/// @param x0 Second degree value.
/// @returns Midpoint in degrees.
#[wasm_bindgen(js_name = swe_deg_midp)]
pub fn js_swe_deg_midp(x1: f64, x0: f64) -> f64 {
    unsafe { swe_bindings::swe_deg_midp(x1, x0) }
}


// ============================================================
// BATCH 1: Core Calculation Functions
// ============================================================

/// Calculate planet-centric position.
///
/// @param tjd Julian Day (ET).
/// @param ipl Planet ID.
/// @param iplctr Center planet ID.
/// @param iflag Calculation flags.
/// @returns Position object or error.
#[wasm_bindgen(js_name = swe_calc_pctr)]
pub fn js_swe_calc_pctr(tjd: f64, ipl: i32, iplctr: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_calc_pctr(tjd, ipl, iplctr, iflag, xx.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"longitude".into(), &xx[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xx[1].into());
    let _ = Reflect::set(&obj, &"distance".into(), &xx[2].into());
    let _ = Reflect::set(&obj, &"speed_long".into(), &xx[3].into());
    let _ = Reflect::set(&obj, &"speed_lat".into(), &xx[4].into());
    let _ = Reflect::set(&obj, &"speed_dist".into(), &xx[5].into());
    let _ = Reflect::set(&obj, &"rc_flags".into(), &ret.into());
    Ok(obj.into())
}

/// Find time when Sun crosses a specific longitude (ET).
#[wasm_bindgen(js_name = swe_solcross)]
pub fn js_swe_solcross(x2cross: f64, jd_et: f64, flag: i32) -> Result<f64, JsValue> {
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_solcross(x2cross, jd_et, flag, serr.as_mut_ptr()) };
    if ret < 0.0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(ret)
}

/// Find time when Moon crosses a specific longitude (ET).
#[wasm_bindgen(js_name = swe_mooncross)]
pub fn js_swe_mooncross(x2cross: f64, jd_et: f64, flag: i32) -> Result<f64, JsValue> {
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_mooncross(x2cross, jd_et, flag, serr.as_mut_ptr()) };
    if ret < 0.0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(ret)
}

/// Find time when Moon crosses its node (ET).
#[wasm_bindgen(js_name = swe_mooncross_node)]
pub fn js_swe_mooncross_node(jd_et: f64, flag: i32) -> Result<JsValue, JsValue> {
    let mut xlon = 0.0;
    let mut xlat = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_mooncross_node(jd_et, flag, &mut xlon, &mut xlat, serr.as_mut_ptr()) };
    if ret < 0.0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"jd".into(), &ret.into());
    let _ = Reflect::set(&obj, &"xlon".into(), &xlon.into());
    let _ = Reflect::set(&obj, &"xlat".into(), &xlat.into());
    Ok(obj.into())
}

/// Find heliocentric crossing time (ET).
#[wasm_bindgen(js_name = swe_helio_cross)]
pub fn js_swe_helio_cross(ipl: i32, x2cross: f64, jd_et: f64, iflag: i32, dir: i32) -> Result<f64, JsValue> {
    let mut jd_cross = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_helio_cross(ipl, x2cross, jd_et, iflag, dir, &mut jd_cross, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(jd_cross)
}

/// Find heliocentric crossing time (UT).
#[wasm_bindgen(js_name = swe_helio_cross_ut)]
pub fn js_swe_helio_cross_ut(ipl: i32, x2cross: f64, jd_ut: f64, iflag: i32, dir: i32) -> Result<f64, JsValue> {
    let mut jd_cross = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_helio_cross_ut(ipl, x2cross, jd_ut, iflag, dir, &mut jd_cross, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(jd_cross)
}


// ============================================================
// BATCH 2: Fixed Stars Functions
// ============================================================

/// Calculate fixed star position (ET).
#[wasm_bindgen(js_name = swe_fixstar)]
pub fn js_swe_fixstar(star: String, tjd: f64, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = cstr_buf();
    let mut star_bytes = alloc::vec![0u8; 256];
    let s_in = star.as_bytes();
    for (i, &b) in s_in.iter().enumerate().take(255) { star_bytes[i] = b; }
    
    let ret = unsafe { swe_bindings::swe_fixstar(star_bytes.as_mut_ptr() as *mut i8, tjd, iflag, xx.as_mut_ptr(), serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let returned_name = unsafe { CStr::from_ptr(star_bytes.as_ptr() as *const i8).to_str().unwrap_or(&star).to_string() };
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"name".into(), &returned_name.into());
    let _ = Reflect::set(&obj, &"longitude".into(), &xx[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xx[1].into());
    let _ = Reflect::set(&obj, &"distance".into(), &xx[2].into());
    let _ = Reflect::set(&obj, &"rc_flags".into(), &ret.into());
    Ok(obj.into())
}

/// Calculate fixed star position using alternative catalog (ET).
#[wasm_bindgen(js_name = swe_fixstar2)]
pub fn js_swe_fixstar2(star: String, tjd: f64, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = cstr_buf();
    let mut star_bytes = alloc::vec![0u8; 256];
    let s_in = star.as_bytes();
    for (i, &b) in s_in.iter().enumerate().take(255) { star_bytes[i] = b; }
    
    let ret = unsafe { swe_bindings::swe_fixstar2(star_bytes.as_mut_ptr() as *mut i8, tjd, iflag, xx.as_mut_ptr(), serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"longitude".into(), &xx[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xx[1].into());
    let _ = Reflect::set(&obj, &"distance".into(), &xx[2].into());
    let _ = Reflect::set(&obj, &"rc_flags".into(), &ret.into());
    Ok(obj.into())
}

/// Calculate fixed star position using alternative catalog (UT).
#[wasm_bindgen(js_name = swe_fixstar2_ut)]
pub fn js_swe_fixstar2_ut(star: String, tjd_ut: f64, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = cstr_buf();
    let mut star_bytes = alloc::vec![0u8; 256];
    let s_in = star.as_bytes();
    for (i, &b) in s_in.iter().enumerate().take(255) { star_bytes[i] = b; }
    
    let ret = unsafe { swe_bindings::swe_fixstar2_ut(star_bytes.as_mut_ptr() as *mut i8, tjd_ut, iflag, xx.as_mut_ptr(), serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"longitude".into(), &xx[0].into());
    let _ = Reflect::set(&obj, &"latitude".into(), &xx[1].into());
    let _ = Reflect::set(&obj, &"distance".into(), &xx[2].into());
    let _ = Reflect::set(&obj, &"rc_flags".into(), &ret.into());
    Ok(obj.into())
}

/// Get fixed star magnitude using alternative catalog.
#[wasm_bindgen(js_name = swe_fixstar2_mag)]
pub fn js_swe_fixstar2_mag(star: String) -> Result<f64, JsValue> {
    let mut mag = 0.0;
    let mut serr = cstr_buf();
    let mut star_bytes = alloc::vec![0u8; 256];
    let s_in = star.as_bytes();
    for (i, &b) in s_in.iter().enumerate().take(255) { star_bytes[i] = b; }
    
    let ret = unsafe { swe_bindings::swe_fixstar2_mag(star_bytes.as_mut_ptr() as *mut i8, &mut mag, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(mag)
}


// ============================================================
// BATCH 3: Date/Time Functions
// ============================================================

/// Validate and convert date.
#[wasm_bindgen(js_name = swe_date_conversion)]
pub fn js_swe_date_conversion(year: i32, month: i32, day: i32, utime: f64, calendar: String) -> Result<f64, JsValue> {
    let mut tjd = 0.0;
    let c = calendar.chars().next().unwrap_or('g') as i8;
    let ret = unsafe { swe_bindings::swe_date_conversion(year, month, day, utime, c, &mut tjd) };
    if ret < 0 { return Err(JsValue::from_str("Invalid date")); }
    Ok(tjd)
}

/// Convert UTC time with timezone offset.
#[wasm_bindgen(js_name = swe_utc_time_zone)]
pub fn js_swe_utc_time_zone(year: i32, month: i32, day: i32, hour: i32, min: i32, sec: f64, d_timezone: f64) -> JsValue {
    let mut year_out = 0;
    let mut month_out = 0;
    let mut day_out = 0;
    let mut hour_out = 0;
    let mut min_out = 0;
    let mut sec_out = 0.0;
    
    unsafe {
        swe_bindings::swe_utc_time_zone(year, month, day, hour, min, sec, d_timezone,
            &mut year_out, &mut month_out, &mut day_out, &mut hour_out, &mut min_out, &mut sec_out);
    }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"year".into(), &year_out.into());
    let _ = Reflect::set(&obj, &"month".into(), &month_out.into());
    let _ = Reflect::set(&obj, &"day".into(), &day_out.into());
    let _ = Reflect::set(&obj, &"hour".into(), &hour_out.into());
    let _ = Reflect::set(&obj, &"min".into(), &min_out.into());
    let _ = Reflect::set(&obj, &"sec".into(), &sec_out.into());
    obj.into()
}

/// Calculate equation of time.
#[wasm_bindgen(js_name = swe_time_equ)]
pub fn js_swe_time_equ(tjd: f64) -> Result<f64, JsValue> {
    let mut te = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_time_equ(tjd, &mut te, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(te)
}

/// Convert Local Mean Time to Local Apparent Time.
#[wasm_bindgen(js_name = swe_lmt_to_lat)]
pub fn js_swe_lmt_to_lat(tjd_lmt: f64, geolon: f64) -> Result<f64, JsValue> {
    let mut tjd_lat = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_lmt_to_lat(tjd_lmt, geolon, &mut tjd_lat, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(tjd_lat)
}

/// Convert Local Apparent Time to Local Mean Time.
#[wasm_bindgen(js_name = swe_lat_to_lmt)]
pub fn js_swe_lat_to_lmt(tjd_lat: f64, geolon: f64) -> Result<f64, JsValue> {
    let mut tjd_lmt = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_lat_to_lmt(tjd_lat, geolon, &mut tjd_lmt, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(tjd_lmt)
}

/// Calculate sidereal time with custom obliquity and nutation.
#[wasm_bindgen(js_name = swe_sidtime0)]
pub fn js_swe_sidtime0(tjd_ut: f64, eps: f64, nut: f64) -> f64 {
    unsafe { swe_bindings::swe_sidtime0(tjd_ut, eps, nut) }
}


// ============================================================
// BATCH 4: Houses Functions
// ============================================================

/// Calculate house cusps with speeds.
#[wasm_bindgen(js_name = swe_houses_ex2)]
pub fn js_swe_houses_ex2(tjd_ut: f64, iflag: i32, geolat: f64, geolon: f64, hsys: String) -> Result<JsValue, JsValue> {
    let hsys_char = hsys.chars().next().unwrap_or('P') as i32;
    let mut cusps = [0.0; 37]; // For Gauquelin houses
    let mut ascmc = [0.0; 10];
    let mut cusp_speed = [0.0; 37];
    let mut ascmc_speed = [0.0; 10];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_houses_ex2(tjd_ut, iflag, geolat, geolon, hsys_char, 
            cusps.as_mut_ptr(), ascmc.as_mut_ptr(), cusp_speed.as_mut_ptr(), ascmc_speed.as_mut_ptr(), serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let js_cusps = Array::new();
    for i in 1..=12 { js_cusps.push(&cusps[i].into()); }
    let _ = Reflect::set(&obj, &"cusps".into(), &js_cusps);
    let _ = Reflect::set(&obj, &"ascendant".into(), &ascmc[0].into());
    let _ = Reflect::set(&obj, &"mc".into(), &ascmc[1].into());
    let _ = Reflect::set(&obj, &"armc".into(), &ascmc[2].into());
    let _ = Reflect::set(&obj, &"vertex".into(), &ascmc[3].into());
    let js_speeds = Array::new();
    for i in 1..=12 { js_speeds.push(&cusp_speed[i].into()); }
    let _ = Reflect::set(&obj, &"cusp_speeds".into(), &js_speeds);
    Ok(obj.into())
}

/// Calculate houses from ARMC.
#[wasm_bindgen(js_name = swe_houses_armc)]
pub fn js_swe_houses_armc(armc: f64, geolat: f64, eps: f64, hsys: String) -> Result<JsValue, JsValue> {
    let hsys_char = hsys.chars().next().unwrap_or('P') as i32;
    let mut cusps = [0.0; 13];
    let mut ascmc = [0.0; 10];
    
    let ret = unsafe { swe_bindings::swe_houses_armc(armc, geolat, eps, hsys_char, cusps.as_mut_ptr(), ascmc.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str("swe_houses_armc failed")); }
    
    let obj = Object::new();
    let js_cusps = Array::new();
    for i in 1..=12 { js_cusps.push(&cusps[i].into()); }
    let _ = Reflect::set(&obj, &"cusps".into(), &js_cusps);
    let _ = Reflect::set(&obj, &"ascendant".into(), &ascmc[0].into());
    let _ = Reflect::set(&obj, &"mc".into(), &ascmc[1].into());
    Ok(obj.into())
}

/// Calculate houses from ARMC with speeds.
#[wasm_bindgen(js_name = swe_houses_armc_ex2)]
pub fn js_swe_houses_armc_ex2(armc: f64, geolat: f64, eps: f64, hsys: String) -> Result<JsValue, JsValue> {
    let hsys_char = hsys.chars().next().unwrap_or('P') as i32;
    let mut cusps = [0.0; 37];
    let mut ascmc = [0.0; 10];
    let mut cusp_speed = [0.0; 37];
    let mut ascmc_speed = [0.0; 10];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_houses_armc_ex2(armc, geolat, eps, hsys_char, 
            cusps.as_mut_ptr(), ascmc.as_mut_ptr(), cusp_speed.as_mut_ptr(), ascmc_speed.as_mut_ptr(), serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let js_cusps = Array::new();
    for i in 1..=12 { js_cusps.push(&cusps[i].into()); }
    let _ = Reflect::set(&obj, &"cusps".into(), &js_cusps);
    let _ = Reflect::set(&obj, &"ascendant".into(), &ascmc[0].into());
    let _ = Reflect::set(&obj, &"mc".into(), &ascmc[1].into());
    Ok(obj.into())
}


// ============================================================
// BATCH 5: Eclipses & Occultations Functions
// ============================================================

/// Find next local lunar eclipse.
#[wasm_bindgen(js_name = swe_lun_eclipse_when_loc)]
pub fn js_swe_lun_eclipse_when_loc(tjd_start: f64, ifl: i32, geolon: f64, geolat: f64, geoalt: f64, backward: i32) -> Result<JsValue, JsValue> {
    let mut geopos = [geolon, geolat, geoalt];
    let mut tret = [0.0; 10];
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_lun_eclipse_when_loc(tjd_start, ifl, geopos.as_mut_ptr(), tret.as_mut_ptr(), attr.as_mut_ptr(), backward, serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    let _ = Reflect::set(&obj, &"tret".into(), &serde_to_js_array(&tret));
    let _ = Reflect::set(&obj, &"attr".into(), &serde_to_js_array(&attr));
    Ok(obj.into())
}

/// Find occultation location.
#[wasm_bindgen(js_name = swe_lun_occult_where)]
pub fn js_swe_lun_occult_where(tjd: f64, ipl: i32, starname: Option<String>, ifl: i32) -> Result<JsValue, JsValue> {
    let mut geopos = [0.0; 2];
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    let (_star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
    let ret = unsafe {
        swe_bindings::swe_lun_occult_where(tjd, ipl, star_ptr, ifl, geopos.as_mut_ptr(), attr.as_mut_ptr(), serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    let _ = Reflect::set(&obj, &"lon".into(), &geopos[0].into());
    let _ = Reflect::set(&obj, &"lat".into(), &geopos[1].into());
    let _ = Reflect::set(&obj, &"attr".into(), &serde_to_js_array(&attr));
    Ok(obj.into())
}

/// Find local occultation.
#[wasm_bindgen(js_name = swe_lun_occult_when_loc)]
pub fn js_swe_lun_occult_when_loc(tjd_start: f64, ipl: i32, starname: Option<String>, ifl: i32, geolon: f64, geolat: f64, geoalt: f64, backward: i32) -> Result<JsValue, JsValue> {
    let mut geopos = [geolon, geolat, geoalt];
    let mut tret = [0.0; 10];
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    let (_star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
    let ret = unsafe {
        swe_bindings::swe_lun_occult_when_loc(tjd_start, ipl, star_ptr, ifl, geopos.as_mut_ptr(), tret.as_mut_ptr(), attr.as_mut_ptr(), backward, serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    let _ = Reflect::set(&obj, &"tret".into(), &serde_to_js_array(&tret));
    let _ = Reflect::set(&obj, &"attr".into(), &serde_to_js_array(&attr));
    Ok(obj.into())
}

/// Find global occultation.
#[wasm_bindgen(js_name = swe_lun_occult_when_glob)]
pub fn js_swe_lun_occult_when_glob(tjd_start: f64, ipl: i32, starname: Option<String>, ifl: i32, ifltype: i32, backward: i32) -> Result<JsValue, JsValue> {
    let mut tret = [0.0; 10];
    let mut serr = cstr_buf();
    let (_star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
    let ret = unsafe {
        swe_bindings::swe_lun_occult_when_glob(tjd_start, ipl, star_ptr, ifl, ifltype, tret.as_mut_ptr(), backward, serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"flags".into(), &ret.into());
    let _ = Reflect::set(&obj, &"tret".into(), &serde_to_js_array(&tret));
    Ok(obj.into())
}

/// Calculate Gauquelin sector position.
#[wasm_bindgen(js_name = swe_gauquelin_sector)]
pub fn js_swe_gauquelin_sector(t_ut: f64, ipl: i32, starname: Option<String>, iflag: i32, imeth: i32, geolon: f64, geolat: f64, geoalt: f64, atpress: f64, attemp: f64) -> Result<f64, JsValue> {
    let mut geopos = [geolon, geolat, geoalt];
    let mut dgsect = 0.0;
    let mut serr = cstr_buf();
    let (_star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
    let ret = unsafe {
        swe_bindings::swe_gauquelin_sector(t_ut, ipl, star_ptr, iflag, imeth, geopos.as_mut_ptr(), atpress, attemp, &mut dgsect, serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(dgsect)
}


// ============================================================
// BATCH 6: Phenomena Functions
// ============================================================

/// Calculate planetary phenomena (ET).
#[wasm_bindgen(js_name = swe_pheno)]
pub fn js_swe_pheno(tjd: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut attr = [0.0; 20];
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_pheno(tjd, ipl, iflag, attr.as_mut_ptr(), serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"phase_angle".into(), &attr[0].into());
    let _ = Reflect::set(&obj, &"phase".into(), &attr[1].into());
    let _ = Reflect::set(&obj, &"elongation".into(), &attr[2].into());
    let _ = Reflect::set(&obj, &"diameter".into(), &attr[3].into());
    let _ = Reflect::set(&obj, &"magnitude".into(), &attr[4].into());
    Ok(obj.into())
}

/// Calculate nodes and apsides (ET).
#[wasm_bindgen(js_name = swe_nod_aps)]
pub fn js_swe_nod_aps(tjd_et: f64, ipl: i32, iflag: i32, method: i32) -> Result<JsValue, JsValue> {
    let mut xnasc = [0.0; 6];
    let mut xndsc = [0.0; 6];
    let mut xperi = [0.0; 6];
    let mut xaphe = [0.0; 6];
    let mut serr = cstr_buf();

    let ret = unsafe {
        swe_bindings::swe_nod_aps(tjd_et, ipl, iflag, method, 
            xnasc.as_mut_ptr(), xndsc.as_mut_ptr(), xperi.as_mut_ptr(), xaphe.as_mut_ptr(), serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"node_ascending".into(), &serde_to_js_array(&xnasc));
    let _ = Reflect::set(&obj, &"node_descending".into(), &serde_to_js_array(&xndsc));
    let _ = Reflect::set(&obj, &"perihelion".into(), &serde_to_js_array(&xperi));
    let _ = Reflect::set(&obj, &"aphelion".into(), &serde_to_js_array(&xaphe));
    Ok(obj.into())
}

/// Get orbital distance extremes.
#[wasm_bindgen(js_name = swe_orbit_max_min_true_distance)]
pub fn js_swe_orbit_max_min_true_distance(tjd_et: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut dmax = 0.0;
    let mut dmin = 0.0;
    let mut dtrue = 0.0;
    let mut serr = cstr_buf();
    
    let ret = unsafe {
        swe_bindings::swe_orbit_max_min_true_distance(tjd_et, ipl, iflag, &mut dmax, &mut dmin, &mut dtrue, serr.as_mut_ptr())
    };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"max_distance".into(), &dmax.into());
    let _ = Reflect::set(&obj, &"min_distance".into(), &dmin.into());
    let _ = Reflect::set(&obj, &"true_distance".into(), &dtrue.into());
    Ok(obj.into())
}


// ============================================================
// BATCH 7: Heliacal Functions
// ============================================================

/// Get heliacal phenomena details.
#[wasm_bindgen(js_name = swe_heliacal_pheno_ut)]
pub fn js_swe_heliacal_pheno_ut(tjd: f64, geo_lon: f64, geo_lat: f64, geo_alt: f64,
                                 atm_press: f64, atm_temp: f64, atm_humid: f64, atm_vis: f64,
                                 obs_age: f64, obs_snellen: f64,
                                 object_name: String, event_type: i32, helflag: i32) -> Result<JsValue, JsValue> {
    let mut dgeo = [geo_lon, geo_lat, geo_alt];
    let mut datm = [atm_press, atm_temp, atm_humid, atm_vis];
    let mut dobs = [obs_age, obs_snellen, 0.0, 0.0, 0.0, 0.0];
    let mut darr = [0.0; 50];
    let mut serr = cstr_buf();
    let (obj_bytes, obj_ptr) = string_to_c_ptr(&object_name);
    
    let ret = unsafe {
        swe_bindings::swe_heliacal_pheno_ut(tjd, dgeo.as_mut_ptr(), datm.as_mut_ptr(), dobs.as_mut_ptr(),
            obj_ptr, event_type, helflag, darr.as_mut_ptr(), serr.as_mut_ptr())
    };
    drop(obj_bytes);
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    
    let obj = Object::new();
    let _ = Reflect::set(&obj, &"tcrit".into(), &darr[0].into());
    let _ = Reflect::set(&obj, &"result".into(), &serde_to_js_array(&darr));
    Ok(obj.into())
}


// ============================================================
// BATCH 8: Utilities Functions
// ============================================================

/// Get Delta T with ephemeris flag.
#[wasm_bindgen(js_name = swe_deltat_ex)]
pub fn js_swe_deltat_ex(tjd: f64, iflag: i32) -> Result<f64, JsValue> {
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_deltat_ex(tjd, iflag, serr.as_mut_ptr()) };
    Ok(ret)
}

/// Get current tidal acceleration.
#[wasm_bindgen(js_name = swe_get_tid_acc)]
pub fn js_swe_get_tid_acc() -> f64 {
    unsafe { swe_bindings::swe_get_tid_acc() }
}

/// Calculate radian midpoint.
#[wasm_bindgen(js_name = swe_rad_midp)]
pub fn js_swe_rad_midp(x1: f64, x0: f64) -> f64 {
    unsafe { swe_bindings::swe_rad_midp(x1, x0) }
}

/// Normalize centiseconds.
#[wasm_bindgen(js_name = swe_csnorm)]
pub fn js_swe_csnorm(p: i32) -> i32 {
    unsafe { swe_bindings::swe_csnorm(p) }
}

/// Centisecond difference (normalized to positive).
#[wasm_bindgen(js_name = swe_difcsn)]
pub fn js_swe_difcsn(p1: i32, p2: i32) -> i32 {
    unsafe { swe_bindings::swe_difcsn(p1, p2) }
}

/// Degree difference (normalized to positive).
#[wasm_bindgen(js_name = swe_difdegn)]
pub fn js_swe_difdegn(p1: f64, p2: f64) -> f64 {
    unsafe { swe_bindings::swe_difdegn(p1, p2) }
}

/// Centisecond difference (signed).
#[wasm_bindgen(js_name = swe_difcs2n)]
pub fn js_swe_difcs2n(p1: i32, p2: i32) -> i32 {
    unsafe { swe_bindings::swe_difcs2n(p1, p2) }
}

/// Degree difference (signed).
#[wasm_bindgen(js_name = swe_difdeg2n)]
pub fn js_swe_difdeg2n(p1: f64, p2: f64) -> f64 {
    unsafe { swe_bindings::swe_difdeg2n(p1, p2) }
}

/// Radian difference (signed).
#[wasm_bindgen(js_name = swe_difrad2n)]
pub fn js_swe_difrad2n(p1: f64, p2: f64) -> f64 {
    unsafe { swe_bindings::swe_difrad2n(p1, p2) }
}

/// Round centiseconds.
#[wasm_bindgen(js_name = swe_csroundsec)]
pub fn js_swe_csroundsec(x: i32) -> i32 {
    unsafe { swe_bindings::swe_csroundsec(x) }
}

/// Double to long with rounding.
#[wasm_bindgen(js_name = swe_d2l)]
pub fn js_swe_d2l(x: f64) -> i32 {
    unsafe { swe_bindings::swe_d2l(x) }
}


// ============================================================
// BATCH 9: Configuration Functions
// ============================================================

/// Set atmospheric lapse rate.
#[wasm_bindgen(js_name = swe_set_lapse_rate)]
pub fn js_swe_set_lapse_rate(lapse_rate: f64) {
    unsafe { swe_bindings::swe_set_lapse_rate(lapse_rate); }
}

/// Set tidal acceleration.
#[wasm_bindgen(js_name = swe_set_tid_acc)]
pub fn js_swe_set_tid_acc(t_acc: f64) {
    unsafe { swe_bindings::swe_set_tid_acc(t_acc); }
}

/// Set user-defined Delta T.
#[wasm_bindgen(js_name = swe_set_delta_t_userdef)]
pub fn js_swe_set_delta_t_userdef(dt: f64) {
    unsafe { swe_bindings::swe_set_delta_t_userdef(dt); }
}

/// Enable/disable nutation interpolation.
#[wasm_bindgen(js_name = swe_set_interpolate_nut)]
pub fn js_swe_set_interpolate_nut(do_interpolate: i32) {
    unsafe { swe_bindings::swe_set_interpolate_nut(do_interpolate); }
}


// ============================================================
// BATCH 10: Ayanamsa Functions
// ============================================================

/// Get ayanamsa (ET).
#[wasm_bindgen(js_name = swe_get_ayanamsa)]
pub fn js_swe_get_ayanamsa(tjd_et: f64) -> f64 {
    unsafe { swe_bindings::swe_get_ayanamsa(tjd_et) }
}

/// Get ayanamsa extended (ET).
#[wasm_bindgen(js_name = swe_get_ayanamsa_ex)]
pub fn js_swe_get_ayanamsa_ex(tjd_et: f64, iflag: i32) -> Result<f64, JsValue> {
    let mut daya = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_get_ayanamsa_ex(tjd_et, iflag, &mut daya, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(daya)
}

/// Get ayanamsa extended (UT).
#[wasm_bindgen(js_name = swe_get_ayanamsa_ex_ut)]
pub fn js_swe_get_ayanamsa_ex_ut(tjd_ut: f64, iflag: i32) -> Result<f64, JsValue> {
    let mut daya = 0.0;
    let mut serr = cstr_buf();
    let ret = unsafe { swe_bindings::swe_get_ayanamsa_ex_ut(tjd_ut, iflag, &mut daya, serr.as_mut_ptr()) };
    if ret < 0 { return Err(JsValue::from_str(&err_to_string(&serr))); }
    Ok(daya)
}


// --- REQUIRED C SHIMS FOR WASM ---
// These are needed because compiling C code often requires stdlib functions not present in pure Wasm environment.
#[cfg(target_arch = "wasm32")]
mod shims {
    use super::*;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn sin(x: f64) -> f64 { libm::sin(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn cos(x: f64) -> f64 { libm::cos(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn tan(x: f64) -> f64 { libm::tan(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn asin(x: f64) -> f64 { libm::asin(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn acos(x: f64) -> f64 { libm::acos(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn atan(x: f64) -> f64 { libm::atan(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn atan2(y: f64, x: f64) -> f64 { libm::atan2(y, x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn sqrt(x: f64) -> f64 { libm::sqrt(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn log(x: f64) -> f64 { libm::log(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn exp(x: f64) -> f64 { libm::exp(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn pow(x: f64, y: f64) -> f64 { libm::pow(x, y) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn fabs(x: f64) -> f64 { libm::fabs(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ceil(x: f64) -> f64 { libm::ceil(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn floor(x: f64) -> f64 { libm::floor(x) }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn log10(x: f64) -> f64 { libm::log10(x) }
    
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_fmod(x: f64, y: f64) -> f64 { libm::fmod(x, y) }
    
    
    // Memory allocation shims
    const MAGIC: usize = 0xDEADBEEF;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_malloc(size: usize) -> *mut u8 {
        unsafe {
            let header_size = 8;
            let total_size = size + header_size;
            let layout = alloc::alloc::Layout::from_size_align_unchecked(total_size, 8);
            let ptr = alloc::alloc::alloc(layout);
            if ptr.is_null() { return core::ptr::null_mut(); }
            let header_ptr = ptr as *mut usize;
            *header_ptr = MAGIC;
            *header_ptr.add(1) = total_size;
            ptr.add(header_size)
        }
    }
    
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_free(ptr: *mut u8) {
        unsafe {
            if ptr.is_null() { return; }
            let header_size = 8;
            let real_ptr = ptr.sub(header_size);
            let header_ptr = real_ptr as *mut usize;
            if *header_ptr != MAGIC { return; }
            let total_size = *header_ptr.add(1);
            let layout = alloc::alloc::Layout::from_size_align_unchecked(total_size, 8);
            alloc::alloc::dealloc(real_ptr, layout);
        }
    }
    
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_calloc(nmemb: usize, size: usize) -> *mut u8 {
        unsafe {
            let total_size = nmemb * size;
            let ptr = rust_malloc(total_size);
            if !ptr.is_null() { core::ptr::write_bytes(ptr, 0, total_size); }
            ptr
        }
    }
    
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
         unsafe {
            if ptr.is_null() { return rust_malloc(new_size); }
            if new_size == 0 { rust_free(ptr); return core::ptr::null_mut(); }
            let header_size = 8;
            let real_ptr = ptr.sub(header_size);
            let header_ptr = real_ptr as *mut usize;
            if *header_ptr != MAGIC { return core::ptr::null_mut(); }
            let old_total_size = *header_ptr.add(1);
            let old_user_size = old_total_size - header_size;
            let new_ptr = rust_malloc(new_size);
            if !new_ptr.is_null() {
                let copy_size = if old_user_size < new_size { old_user_size } else { new_size };
                core::ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
                rust_free(ptr);
            }
            new_ptr
        }
    }
    
    // --- String & Mem Shims (Not in stub.c) ---
    
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        unsafe { core::ptr::copy_nonoverlapping(src, dest, n); dest }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
        unsafe { core::ptr::write_bytes(s, c as u8, n); s }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        unsafe { core::ptr::copy(src, dest, n); dest }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
        unsafe {
            let mut i = 0;
            loop {
                let c = *src.add(i);
                *dest.add(i) = c;
                if c == 0 { break; }
                i += 1;
            }
            dest
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
        unsafe {
            let mut i = 0;
            while i < n {
                let c = *src.add(i);
                *dest.add(i) = c;
                if c == 0 { while i < n { *dest.add(i) = 0; i += 1;} break; }
                i += 1;
            }
            dest
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strlen(s: *const i8) -> usize {
        unsafe {
            let mut len = 0;
            while *s.add(len) != 0 { len += 1; }
            len
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strcmp(s1: *const i8, s2: *const i8) -> i32 {
        unsafe {
            let mut i = 0;
            loop {
                let c1 = *s1.add(i);
                let c2 = *s2.add(i);
                if c1 != c2 { return (c1 - c2) as i32; }
                if c1 == 0 { return 0; }
                i += 1;
            }
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32 {
         unsafe {
            let mut i = 0;
            while i < n {
                let c1 = *s1.add(i);
                let c2 = *s2.add(i);
                if c1 != c2 { return (c1 - c2) as i32; }
                if c1 == 0 { return 0; }
                i += 1;
            }
            0
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strstr(haystack: *const i8, needle: *const i8) -> *const i8 {
        unsafe {
            let needle_len = strlen(needle);
            if needle_len == 0 { return haystack; }
            let mut h = haystack;
            while *h != 0 {
                if strncmp(h, needle, needle_len) == 0 { return h; }
                h = h.add(1);
            }
            core::ptr::null()
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn abs(j: i32) -> i32 { j.abs() }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn labs(j: i64) -> i64 { j.abs() }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn atof(_str: *const i8) -> f64 { 0.0 }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn atoi(_str: *const i8) -> i32 { 0 }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn atol(_str: *const i8) -> i64 { 0 }
    
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strpbrk(s: *const i8, accept: *const i8) -> *mut i8 {
        unsafe {
            let mut s_ptr = s;
            while *s_ptr != 0 {
                let mut a_ptr = accept;
                while *a_ptr != 0 { if *s_ptr == *a_ptr { return s_ptr as *mut i8; } a_ptr = a_ptr.add(1); }
                s_ptr = s_ptr.add(1);
            }
            core::ptr::null_mut()
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn memchr(s: *const u8, c: i32, n: usize) -> *mut u8 {
        unsafe {
            let mut i = 0;
            while i < n { if *s.add(i) == c as u8 { return s.add(i) as *mut u8; } i += 1; }
            core::ptr::null_mut()
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strdup(s: *const i8) -> *mut i8 {
        unsafe {
            let len = strlen(s);
            let ptr = rust_malloc(len + 1) as *mut i8;
            if !ptr.is_null() { strcpy(ptr as *mut u8, s as *const u8); }
            ptr
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strcat(dest: *mut i8, src: *const i8) -> *mut i8 {
        unsafe {
            let len = strlen(dest);
            strcpy(dest.add(len) as *mut u8, src as *const u8);
            dest
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strchr(s: *const i8, c: i32) -> *mut i8 {
        unsafe {
            let mut s_ptr = s;
            loop {
                if *s_ptr == c as i8 { return s_ptr as *mut i8; }
                if *s_ptr == 0 { return core::ptr::null_mut(); }
                s_ptr = s_ptr.add(1);
            }
        }
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn strrchr(s: *const i8, c: i32) -> *mut i8 {
        unsafe {
            let mut last = core::ptr::null_mut();
            let mut s_ptr = s;
            loop {
                if *s_ptr == c as i8 { last = s_ptr as *mut i8; }
                if *s_ptr == 0 { return last; }
                s_ptr = s_ptr.add(1);
            }
        }
    }
    // tolower is in stub.c

    
    // dlfcn stubs
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn dlopen(_filename: *const i8, _flag: i32) -> *mut u8 { core::ptr::null_mut() }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn dlerror() -> *mut i8 { core::ptr::null_mut() }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn dlsym(_handle: *mut u8, _symbol: *const i8) -> *mut u8 { core::ptr::null_mut() }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn dlclose(_handle: *mut u8) -> i32 { 0 }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn dladdr(_addr: *const u8, _info: *mut u8) -> i32 { 0 }
    
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn qsort(_base: *mut u8, _nmemb: usize, _size: usize, _compar: *const u8) {}
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn bsearch(_key: *const u8, _base: *const u8, _nmemb: usize, _size: usize, _compar: *const u8) -> *mut u8 { core::ptr::null_mut() }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn readlink(_path: *const i8, _buf: *mut i8, _bufsiz: usize) -> isize { -1 }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn stat(_path: *const i8, _buf: *mut u8) -> i32 { -1 }


}

