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

#[wasm_bindgen(js_name = swe_julday)]
pub fn js_swe_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: i32) -> f64 {
    unsafe { swe_bindings::swe_julday(year, month, day, hour, gregflag) }
}

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

#[wasm_bindgen(js_name = swe_sidtime)]
pub fn js_swe_sidtime(tjd_ut: f64) -> f64 {
    unsafe { swe_bindings::swe_sidtime(tjd_ut) }
}


// --- Houses ---

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
    let (star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
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
    let (star_bytes, star_ptr) = if let Some(s) = starname { string_to_c_ptr(&s) } else { (Vec::new(), core::ptr::null_mut()) };
    
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
    let mut backward = 0;
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

