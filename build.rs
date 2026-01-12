use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    
    // Auto-detect Homebrew LLVM on macOS for WASM builds if CC is not set
    if target.contains("wasm32") && host.contains("apple") && env::var("CC").is_err() {
        let brew_llvm = PathBuf::from("/opt/homebrew/opt/llvm/bin/clang");
        if brew_llvm.exists() {
            println!("cargo:warning=Auto-detected Homebrew LLVM for WASM build: {:?}", brew_llvm);
            unsafe {
                env::set_var("CC", brew_llvm.to_str().unwrap());
                env::set_var("AR", "/opt/homebrew/opt/llvm/bin/llvm-ar");
            }
        } else {
             println!("cargo:warning=WebAssembly build on macOS requires LLVM/Clang with wasm32 support.");
             println!("cargo:warning=Please install with: brew install llvm");
             println!("cargo:warning=Or set CC environment variable to a wasm32-compatible clang.");
        }
    }

    let mut build = cc::Build::new();
    
    // Source Directories
    let swe_src = "vendor/swisseph";
    let wrapper_src = "src/wrapper";

    // Swiss Ephemeris Source Files (Upstream)
    let c_files = [
        "swecl.c", "swedate.c", "swehel.c", "swehouse.c", "swejpl.c",
        "swemmoon.c", "swemplan.c", "sweph.c", "swephlib.c",
    ];

    for file in &c_files {
        build.file(format!("{}/{}", swe_src, file));
    }
    
    // Include paths
    build.include(swe_src);
    build.include(wrapper_src); // needed for wrapper header

    // Define macros
    build.define("NO_MOSHIER", None); // Use JPL ephemeris only

    // Rename colliding symbols
    // This list must match the functions we intend to wrap in bindings.rs
    // The pattern is: (Original Name, Renamed Internal Name)
    let renaming = [
        // Core Calculation
        ("swe_calc_ut", "impl_swe_calc_ut"),
        ("swe_calc", "impl_swe_calc"),
        ("swe_calc_pctr", "impl_swe_calc_pctr"),
        ("swe_solcross", "impl_swe_solcross"),
        ("swe_solcross_ut", "impl_swe_solcross_ut"),
        ("swe_mooncross", "impl_swe_mooncross"),
        ("swe_mooncross_ut", "impl_swe_mooncross_ut"),
        ("swe_mooncross_node", "impl_swe_mooncross_node"),
        ("swe_mooncross_node_ut", "impl_swe_mooncross_node_ut"),
        ("swe_helio_cross", "impl_swe_helio_cross"),
        ("swe_helio_cross_ut", "impl_swe_helio_cross_ut"),
        
        // Fixed Stars
        ("swe_fixstar_ut", "impl_swe_fixstar_ut"),
        ("swe_fixstar", "impl_swe_fixstar"),
        ("swe_fixstar_mag", "impl_swe_fixstar_mag"),
        ("swe_fixstar2", "impl_swe_fixstar2"),
        ("swe_fixstar2_ut", "impl_swe_fixstar2_ut"),
        ("swe_fixstar2_mag", "impl_swe_fixstar2_mag"),
        
        // Date & Time
        ("swe_julday", "impl_swe_julday"),
        ("swe_revjul", "impl_swe_revjul"),
        ("swe_date_conversion", "impl_swe_date_conversion"),
        ("swe_utc_to_jd", "impl_swe_utc_to_jd"),
        ("swe_jdet_to_utc", "impl_swe_jdet_to_utc"),
        ("swe_jdut1_to_utc", "impl_swe_jdut1_to_utc"),
        ("swe_utc_time_zone", "impl_swe_utc_time_zone"),
        ("swe_time_equ", "impl_swe_time_equ"),
        ("swe_lmt_to_lat", "impl_swe_lmt_to_lat"),
        ("swe_lat_to_lmt", "impl_swe_lat_to_lmt"),
        ("swe_sidtime", "impl_swe_sidtime"),
        ("swe_sidtime0", "impl_swe_sidtime0"),
        
        // Eclipses
        ("swe_sol_eclipse_where", "impl_swe_sol_eclipse_where"),
        ("swe_sol_eclipse_how", "impl_swe_sol_eclipse_how"),
        ("swe_sol_eclipse_when_loc", "impl_swe_sol_eclipse_when_loc"),
        ("swe_sol_eclipse_when_glob", "impl_swe_sol_eclipse_when_glob"),
        ("swe_lun_eclipse_how", "impl_swe_lun_eclipse_how"),
        ("swe_lun_eclipse_when", "impl_swe_lun_eclipse_when"),
        ("swe_lun_eclipse_when_loc", "impl_swe_lun_eclipse_when_loc"),
        ("swe_lun_occult_where", "impl_swe_lun_occult_where"),
        ("swe_lun_occult_when_loc", "impl_swe_lun_occult_when_loc"),
        ("swe_lun_occult_when_glob", "impl_swe_lun_occult_when_glob"),
        ("swe_gauquelin_sector", "impl_swe_gauquelin_sector"),
        
        // Houses
        ("swe_houses", "impl_swe_houses"),
        ("swe_houses_ex", "impl_swe_houses_ex"),
        ("swe_houses_ex2", "impl_swe_houses_ex2"),
        ("swe_houses_armc", "impl_swe_houses_armc"),
        ("swe_houses_armc_ex2", "impl_swe_houses_armc_ex2"),
        ("swe_house_pos", "impl_swe_house_pos"),
        ("swe_house_name", "impl_swe_house_name"),
        
        // Configuration
        ("swe_set_topo", "impl_swe_set_topo"),
        ("swe_set_sid_mode", "impl_swe_set_sid_mode"),
        ("swe_set_ephe_path", "impl_swe_set_ephe_path"),
        ("swe_set_jpl_file", "impl_swe_set_jpl_file"),
        ("swe_set_lapse_rate", "impl_swe_set_lapse_rate"),
        ("swe_set_tid_acc", "impl_swe_set_tid_acc"),
        ("swe_set_delta_t_userdef", "impl_swe_set_delta_t_userdef"),
        ("swe_set_interpolate_nut", "impl_swe_set_interpolate_nut"),
        ("swe_close", "impl_swe_close"),
        ("swe_version", "impl_swe_version"),
        ("swe_get_library_path", "impl_swe_get_library_path"),
        
        // Phenomena
        ("swe_pheno", "impl_swe_pheno"),
        ("swe_pheno_ut", "impl_swe_pheno_ut"),
        ("swe_get_planet_name", "impl_swe_get_planet_name"),
        ("swe_nod_aps", "impl_swe_nod_aps"),
        ("swe_nod_aps_ut", "impl_swe_nod_aps_ut"),
        ("swe_get_orbital_elements", "impl_swe_get_orbital_elements"),
        ("swe_orbit_max_min_true_distance", "impl_swe_orbit_max_min_true_distance"),
        ("swe_azalt", "impl_swe_azalt"),
        ("swe_azalt_rev", "impl_swe_azalt_rev"),
        ("swe_rise_trans", "impl_swe_rise_trans"),
        ("swe_rise_trans_true_hor", "impl_swe_rise_trans_true_hor"),
        ("swe_refrac", "impl_swe_refrac"),
        ("swe_refrac_extended", "impl_swe_refrac_extended"),
        
        // Heliacal
        ("swe_heliacal_ut", "impl_swe_heliacal_ut"),
        ("swe_heliacal_pheno_ut", "impl_swe_heliacal_pheno_ut"),
        ("swe_vis_limit_mag", "impl_swe_vis_limit_mag"),
        // Internal/Private functions (often used by experts)
        ("swe_heliacal_angle", "impl_swe_heliacal_angle"),
        ("swe_topo_arcus_visionis", "impl_swe_topo_arcus_visionis"),
        
        // Utilities & Math
        ("swe_deltat", "impl_swe_deltat"),
        ("swe_deltat_ex", "impl_swe_deltat_ex"),
        ("swe_cotrans", "impl_swe_cotrans"),
        ("swe_cotrans_sp", "impl_swe_cotrans_sp"),
        ("swe_get_tid_acc", "impl_swe_get_tid_acc"),
        ("swe_degnorm", "impl_swe_degnorm"),
        ("swe_radnorm", "impl_swe_radnorm"),
        ("swe_rad_midp", "impl_swe_rad_midp"),
        ("swe_deg_midp", "impl_swe_deg_midp"),
        ("swe_split_deg", "impl_swe_split_deg"),
        ("swe_csnorm", "impl_swe_csnorm"),
        ("swe_difcsn", "impl_swe_difcsn"),
        ("swe_difdegn", "impl_swe_difdegn"),
        ("swe_difcs2n", "impl_swe_difcs2n"),
        ("swe_difdeg2n", "impl_swe_difdeg2n"),
        ("swe_difrad2n", "impl_swe_difrad2n"),
        ("swe_csroundsec", "impl_swe_csroundsec"),
        ("swe_d2l", "impl_swe_d2l"),
        ("swe_day_of_week", "impl_swe_day_of_week"),
        ("swe_cs2timestr", "impl_swe_cs2timestr"),
        ("swe_cs2lonlatstr", "impl_swe_cs2lonlatstr"),
        ("swe_cs2degstr", "impl_swe_cs2degstr"),
        
        // Info
        ("swe_get_ayanamsa", "impl_swe_get_ayanamsa"),
        ("swe_get_ayanamsa_ut", "impl_swe_get_ayanamsa_ut"),
        ("swe_get_ayanamsa_ex", "impl_swe_get_ayanamsa_ex"),
        ("swe_get_ayanamsa_ex_ut", "impl_swe_get_ayanamsa_ex_ut"),
        ("swe_get_ayanamsa_name", "impl_swe_get_ayanamsa_name"),
        ("swe_get_current_file_data", "impl_swe_get_current_file_data"),
    ];
    
    for (orig, renamed) in &renaming {
        build.define(orig, Some(*renamed));
    }

    // Target specific configuration
    if target.contains("wasm32") {
        // Wrapper Code (Custom) - Only for Wasm
        build.file(format!("{}/stub.c", wrapper_src));

        // Use our custom shims
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_includes = PathBuf::from(manifest_dir).join("wasm-includes");
        build.include(&wasm_includes);
        
        // Strict Warnings & Quality Control
        build
             .flag("-Wall")
             .flag("-Wextra")
             // Suppress inevitable warnings from legacy C code
             .flag("-Wno-implicit-function-declaration")
             .flag("-Wno-int-conversion")
             .flag("-Wno-unused-variable")
             .flag("-Wno-unused-parameter")
             .flag("-Wno-sign-compare")
             .flag("-Wno-missing-braces")
             .flag("-Wno-parentheses")
             .flag("-Wno-misleading-indentation")
             .flag("-Wno-empty-body")
             .flag("-Wno-unknown-pragmas")
             // Ensure we are compiling for the right target
             .flag("-target").flag("wasm32-unknown-unknown");
    }

    build.compile("swe");
}
