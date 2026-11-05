//! chemical_engineering: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/chemical_engineering/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "chemeng_absorber_transfer_units",
    "chemeng_adiabatic_flame_temperature",
    "chemeng_adiabatic_reactor_temperature",
    "chemeng_antoine_boiling_temperature",
    "chemeng_antoine_vapour_pressure",
    "chemeng_batch_coil_cooling",
    "chemeng_batch_conversion_first_order",
    "chemeng_batch_conversion_second_order",
    "chemeng_batch_heating_temperature",
    "chemeng_batch_heating_time",
    "chemeng_batch_time_first_order",
    "chemeng_batch_time_second_order",
    "chemeng_binary_minimum_reflux",
    "chemeng_binary_separator_balance",
    "chemeng_blending_flows",
    "chemeng_cake_filtration_volume",
    "chemeng_carman_pressure_drop",
    "chemeng_closed_loop_stability",
    "chemeng_combustion_flue_gas",
    "chemeng_countercurrent_extraction",
    "chemeng_crosscurrent_extraction",
    "chemeng_cstr_conversion_first_order",
    "chemeng_cstr_conversion_second_order",
    "chemeng_cstr_volume_first_order",
    "chemeng_cstr_volume_second_order",
    "chemeng_cstrs_in_series_first_order",
    "chemeng_cstrs_in_series_second_order",
    "chemeng_equimolar_counterdiffusion",
    "chemeng_ergun_pressure_drop",
    "chemeng_exponential_decline",
    "chemeng_fenske_minimum_stages",
    "chemeng_filtration_constants_fit",
    "chemeng_first_order_step_fit",
    "chemeng_fopdt_step_response",
    "chemeng_gas_pz_material_balance",
    "chemeng_gas_well_backpressure",
    "chemeng_harmonic_decline",
    "chemeng_havlena_odeh_gas_cap",
    "chemeng_havlena_odeh_oil_in_place",
    "chemeng_horner_buildup",
    "chemeng_hyperbolic_decline",
    "chemeng_kremser_absorber",
    "chemeng_kremser_stripper",
    "chemeng_lag_frequency_response",
    "chemeng_lee_kesler_vapour_pressure",
    "chemeng_line_source_drawdown",
    "chemeng_mass_to_mole_fractions",
    "chemeng_mass_transfer_coefficient",
    "chemeng_mccabe_thiele_stages",
    "chemeng_modified_raoult_bubble_pressure",
    "chemeng_multicomponent_underwood",
    "chemeng_npsh_available",
    "chemeng_nrtl_activity_coefficients",
    "chemeng_p_control_offset",
    "chemeng_peng_robinson",
    "chemeng_pfr_conversion_first_order",
    "chemeng_pfr_conversion_second_order",
    "chemeng_pfr_gas_volume_first_order",
    "chemeng_pfr_gas_volume_second_order",
    "chemeng_pfr_volume_first_order",
    "chemeng_pfr_volume_second_order",
    "chemeng_pi_loop_margins",
    "chemeng_pid_controller_output",
    "chemeng_pitzer_heat_of_vaporization",
    "chemeng_pump_head",
    "chemeng_rachford_rice_flash",
    "chemeng_rackett_liquid_volume",
    "chemeng_radial_darcy_flow",
    "chemeng_raoult_bubble_pressure",
    "chemeng_raoult_bubble_temperature",
    "chemeng_raoult_dew_pressure",
    "chemeng_raoult_dew_temperature",
    "chemeng_raoult_isothermal_flash",
    "chemeng_reaction_enthalpy_at_temperature",
    "chemeng_recycle_purge",
    "chemeng_recycle_reactor_first_order",
    "chemeng_reversible_first_order_batch",
    "chemeng_riedel_heat_of_vaporization",
    "chemeng_rtd_moments",
    "chemeng_sensible_heat_cp_polynomial",
    "chemeng_series_reaction_maximum",
    "chemeng_srk",
    "chemeng_stefan_diffusion_flux",
    "chemeng_uniquac_activity_coefficients",
    "chemeng_unsteady_mixing_tank",
    "chemeng_variable_volume_tank",
    "chemeng_virial_compressibility",
    "chemeng_watson_heat_of_vaporization",
    "chemeng_wilson_activity_coefficients",
    "chemeng_wilson_k_flash",
    "chemeng_ziegler_nichols_tuning",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "chemeng_absorber_transfer_units" => {
            let r = f_absorber_nog(match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HOG") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_32(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_adiabatic_flame_temperature" => {
            let r = f_flame_temperature(match v.get("fuel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("excess_air") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_in") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_117(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_adiabatic_reactor_temperature" => {
            let r = f_adiabatic_reactor_T(match v.get("n0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("key") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_235(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_antoine_boiling_temperature" => {
            let r = f_antoine_T(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_296(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_antoine_vapour_pressure" => {
            let r = f_antoine_P(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_357(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_batch_coil_cooling" => {
            let r = f_batch_coil_cooling(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("UA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_batch_conversion_first_order" => {
            let r = f_batch_conv_1(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_457(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_batch_conversion_second_order" => {
            let r = f_batch_conv_2(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_507(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_batch_heating_temperature" => {
            let r = f_batch_heating_T(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("UA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_557(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_batch_heating_time" => {
            let r = f_batch_heating_time(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("UA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_607(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_batch_time_first_order" => {
            let r = f_batch_time_1(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_663(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_batch_time_second_order" => {
            let r = f_batch_time_2(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_719(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_binary_minimum_reflux" => {
            let r = f_minimum_reflux(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_785(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_binary_separator_balance" => {
            let r = f_binary_separator(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_848(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_blending_flows" => {
            let r = f_blending(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_937(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_cake_filtration_volume" => {
            let r = f_filtration_volume(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_987(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_carman_pressure_drop" => {
            let r = f_carman(match v.get("dp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1043(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_closed_loop_stability" => {
            let r = f_closed_loop_stability(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1120(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_combustion_flue_gas" => {
            let r = f_combustion(match v.get("fuel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("excess_air") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1175(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_countercurrent_extraction" => {
            let r = f_countercurrent_extraction(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("XF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("YS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1233(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_crosscurrent_extraction" => {
            let r = f_crosscurrent_extraction(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("XF") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1281(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_cstr_conversion_first_order" => {
            let r = f_cstr_conv_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1325(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_cstr_conversion_second_order" => {
            let r = f_cstr_conv_2(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1369(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_cstr_volume_first_order" => {
            let r = f_cstr_volume_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1425(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_cstr_volume_second_order" => {
            let r = f_cstr_volume_2(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1481(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_cstrs_in_series_first_order" => {
            let r = f_cstrs_series_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1529(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_cstrs_in_series_second_order" => {
            let r = f_cstrs_series_2(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1578(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_equimolar_counterdiffusion" => {
            let r = f_equimolar_flux(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yA1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yA2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1636(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_ergun_pressure_drop" => {
            let r = f_ergun(match v.get("dp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1692(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_exponential_decline" => {
            let r = f_decline_exp(match v.get("qi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Di") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1742(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_fenske_minimum_stages" => {
            let r = f_fenske(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1789(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_filtration_constants_fit" => {
            let r = f_filtration_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1858(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_first_order_step_fit" => {
            let r = f_fit_first_order(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1971(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_fopdt_step_response" => {
            let r = f_fopdt_step(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2026(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_gas_pz_material_balance" => {
            let r = f_gas_pz(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gp_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2094(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_gas_well_backpressure" => {
            let r = f_backpressure(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pwf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2167(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_harmonic_decline" => {
            let r = f_decline_harm(match v.get("qi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Di") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2217(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_havlena_odeh_gas_cap" => {
            let r = f_ho_gascap(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Eo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Eg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2283(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_havlena_odeh_oil_in_place" => {
            let r = f_ho_oil(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Et") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2341(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_horner_buildup" => {
            let r = f_horner(match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pws") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2408(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_hyperbolic_decline" => {
            let r = f_decline_hyp(match v.get("qi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Di") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2464(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_kremser_absorber" => {
            let r = f_kremser_absorber(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_in") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2522(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_kremser_stripper" => {
            let r = f_kremser_stripper(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y_in") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2580(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_lag_frequency_response" => {
            let r = f_lag_freq(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2635(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_lee_kesler_vapour_pressure" => {
            let r = f_lee_kesler(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2684(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_line_source_drawdown" => {
            let r = f_line_source(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ct") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2756(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_mass_to_mole_fractions" => {
            let r = f_mass_to_mole(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2817(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_mass_transfer_coefficient" => {
            let r = f_mass_transfer_coefficient(match v.get("geometry") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2890(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_mccabe_thiele_stages" => {
            let r = f_mccabe_thiele(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2976(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_modified_raoult_bubble_pressure" => {
            let r = f_modified_raoult_bubble(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Psat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3042(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_multicomponent_underwood" => {
            let r = f_underwood_multi(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hk") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3139(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_npsh_available" => {
            let r = f_npsh_available(match v.get("p_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("NPSHr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3195(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_nrtl_activity_coefficients" => {
            let r = f_nrtl_gamma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3267(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_p_control_offset" => {
            let r = f_p_offset(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3346(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_peng_robinson" => {
            let r = s_3372(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_conversion_first_order" => {
            let r = f_pfr_conv_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3470(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_conversion_second_order" => {
            let r = f_pfr_conv_2(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3514(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_gas_volume_first_order" => {
            let r = f_pfr_gas_1(match v.get("FA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3576(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_gas_volume_second_order" => {
            let r = f_pfr_gas_2(match v.get("FA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3638(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_volume_first_order" => {
            let r = f_pfr_volume_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3694(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_volume_second_order" => {
            let r = f_pfr_volume_2(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3750(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pi_loop_margins" => {
            let r = f_margins_pi(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tauI") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3839(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pid_controller_output" => {
            let r = f_pid_position(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tauI") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tauD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bias") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3897(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pitzer_heat_of_vaporization" => {
            let r = f_pitzer_hvap(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3946(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pump_head" => {
            let r = f_pump_head(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dz") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("v1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("v2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Q") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("eta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4015(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_rachford_rice_flash" => {
            let r = f_rachford_rice(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4120(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_rackett_liquid_volume" => {
            let r = f_rackett(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Zc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4169(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_radial_darcy_flow" => {
            let r = f_radial_flow(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pe") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4222(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_bubble_pressure" => {
            let r = f_bubble_P(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4298(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_bubble_temperature" => {
            let r = f_bubble_T(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4408(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_dew_pressure" => {
            let r = f_dew_P(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4484(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_dew_temperature" => {
            let r = f_dew_T(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_isothermal_flash" => {
            let r = f_raoult_flash(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_reaction_enthalpy_at_temperature" => {
            let r = f_reaction_enthalpy_T(match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4774(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_recycle_purge" => {
            let r = f_recycle_purge(match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4831(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_recycle_reactor_first_order" => {
            let r = f_recycle_reactor_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4881(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_reversible_first_order_batch" => {
            let r = f_reversible_batch(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CB0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4931(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_riedel_heat_of_vaporization" => {
            let r = f_riedel(match v.get("Tb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4977(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_rtd_moments" => {
            let r = f_rtd_moments(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5039(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_sensible_heat_cp_polynomial" => {
            let r = f_sensible_heat(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5089(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_series_reaction_maximum" => {
            let r = f_series_max(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5136(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_srk" => {
            let r = s_5162(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5216(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_stefan_diffusion_flux" => {
            let r = f_stefan_flux(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yA1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yA2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5274(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_uniquac_activity_coefficients" => {
            let r = f_uniquac_gamma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5363(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_unsteady_mixing_tank" => {
            let r = f_mixing_tank(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_variable_volume_tank" => {
            let r = f_variable_volume_tank(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5467(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_virial_compressibility" => {
            let r = f_abbott_virial(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5516(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_watson_heat_of_vaporization" => {
            let r = f_watson(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hvap_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5565(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_wilson_activity_coefficients" => {
            let r = f_wilson_gamma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5637(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_wilson_k_flash" => {
            let r = f_wilson_flash(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5749(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_ziegler_nichols_tuning" => {
            let r = f_ziegler_nichols(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("controller") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5839(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_AW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("C"), Py::Float(12.0107)), (Py::str("H"), Py::Float(1.00794)), (Py::str("O"), Py::Float(15.9994)), (Py::str("N"), Py::Float(14.0067)), (Py::str("S"), Py::Float(32.065))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MW_AIR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(28.850334)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_N2_PER_O2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.761904761904762)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_GAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ANT_BASE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("log10"), Py::Float(10.0)), (Py::str("ln"), Py::Float(2.718281828459045))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ANT_UNITS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("mmHg_degC"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(273.15)])), (Py::str("kPa_degC"), Py::tuple(vec![Py::Float(1000.0), Py::Float(273.15)])), (Py::str("bar_degC"), Py::tuple(vec![Py::Float(100000.0), Py::Float(273.15)])), (Py::str("mmHg_K"), Py::tuple(vec![Py::Float(133.322387415), Py::Float(0.0)])), (Py::str("Pa_K"), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::str("kPa_K"), Py::tuple(vec![Py::Float(1000.0), Py::Float(0.0)])), (Py::str("bar_K"), Py::tuple(vec![Py::Float(100000.0), Py::Float(0.0)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FLAME_FUELS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("CH4"), Py::tuple(vec![Py::Int(1i64), Py::Int(4i64), Py::Int(0i64)])), (Py::str("C2H6"), Py::tuple(vec![Py::Int(2i64), Py::Int(6i64), Py::Int(0i64)])), (Py::str("C3H8"), Py::tuple(vec![Py::Int(3i64), Py::Int(8i64), Py::Int(0i64)])), (Py::str("H2"), Py::tuple(vec![Py::Int(0i64), Py::Int(2i64), Py::Int(0i64)])), (Py::str("CO"), Py::tuple(vec![Py::Int(1i64), Py::Int(0i64), Py::Int(1i64)])), (Py::str("C2H4"), Py::tuple(vec![Py::Int(2i64), Py::Int(4i64), Py::Int(0i64)])), (Py::str("C2H2"), Py::tuple(vec![Py::Int(2i64), Py::Int(2i64), Py::Int(0i64)])), (Py::str("CH3OH"), Py::tuple(vec![Py::Int(1i64), Py::Int(4i64), Py::Int(1i64)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GPU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("m3"), Py::Float(1.0)), (Py::str("MMscf"), Py::Float(28316.846592)), (Py::str("Bscf"), Py::Float(28316846.592))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NASA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("CH4"), Py::list(vec![Py::Float(1000.0), Py::Float(0.074851495), Py::Float(0.0133909467), Py::Float(-5.73285809e-06), Py::Float(1.22292535e-09), Py::Float(-1.0181523e-13), Py::Float(-9468.34459), Py::Float(18.437318), Py::Float(5.14987613), Py::Float(-0.0136709788), Py::Float(4.91800599e-05), Py::Float(-4.84743026e-08), Py::Float(1.66693956e-11), Py::Float(-10246.6476), Py::Float(-4.64130376)])), (Py::str("C2H6"), Py::list(vec![Py::Float(1000.0), Py::Float(1.0718815), Py::Float(0.0216852677), Py::Float(-1.00256067e-05), Py::Float(2.21412001e-09), Py::Float(-1.9000289e-13), Py::Float(-11426.3932), Py::Float(15.1156107), Py::Float(4.29142492), Py::Float(-0.0055015427), Py::Float(5.99438288e-05), Py::Float(-7.08466285e-08), Py::Float(2.68685771e-11), Py::Float(-11522.2055), Py::Float(2.66682316)])), (Py::str("C3H8"), Py::list(vec![Py::Float(1000.0), Py::Float(7.5341368), Py::Float(0.018872239), Py::Float(-6.2718491e-06), Py::Float(9.1475649e-10), Py::Float(-4.7838069e-14), Py::Float(-16467.516), Py::Float(-17.892349), Py::Float(0.93355381), Py::Float(0.026424579), Py::Float(6.1059727e-06), Py::Float(-2.1977499e-08), Py::Float(9.5149253e-12), Py::Float(-13958.52), Py::Float(19.201691)])), (Py::str("H2"), Py::list(vec![Py::Float(1000.0), Py::Float(3.3372792), Py::Float(-4.94024731e-05), Py::Float(4.99456778e-07), Py::Float(-1.79566394e-10), Py::Float(2.00255376e-14), Py::Float(-950.158922), Py::Float(-3.20502331), Py::Float(2.34433112), Py::Float(0.00798052075), Py::Float(-1.9478151e-05), Py::Float(2.01572094e-08), Py::Float(-7.37611761e-12), Py::Float(-917.935173), Py::Float(0.683010238)])), (Py::str("CO"), Py::list(vec![Py::Float(1000.0), Py::Float(2.71518561), Py::Float(0.00206252743), Py::Float(-9.98825771e-07), Py::Float(2.30053008e-10), Py::Float(-2.03647716e-14), Py::Float(-14151.8724), Py::Float(7.81868772), Py::Float(3.57953347), Py::Float(-0.00061035368), Py::Float(1.01681433e-06), Py::Float(9.07005884e-10), Py::Float(-9.04424499e-13), Py::Float(-14344.086), Py::Float(3.50840928)])), (Py::str("C2H4"), Py::list(vec![Py::Float(1000.0), Py::Float(2.03611116), Py::Float(0.0146454151), Py::Float(-6.71077915e-06), Py::Float(1.47222923e-09), Py::Float(-1.25706061e-13), Py::Float(4939.88614), Py::Float(10.3053693), Py::Float(3.95920148), Py::Float(-0.00757052247), Py::Float(5.70990292e-05), Py::Float(-6.91588753e-08), Py::Float(2.69884373e-11), Py::Float(5089.77593), Py::Float(4.09733096)])), (Py::str("C2H2"), Py::list(vec![Py::Float(1000.0), Py::Float(4.14756964), Py::Float(0.00596166664), Py::Float(-2.37294852e-06), Py::Float(4.67412171e-10), Py::Float(-3.61235213e-14), Py::Float(25935.9992), Py::Float(-1.23028121), Py::Float(0.808681094), Py::Float(0.0233615629), Py::Float(-3.55171815e-05), Py::Float(2.80152437e-08), Py::Float(-8.50072974e-12), Py::Float(26428.9807), Py::Float(13.9397051)])), (Py::str("CH3OH"), Py::list(vec![Py::Float(1000.0), Py::Float(1.78970791), Py::Float(0.0140938292), Py::Float(-6.36500835e-06), Py::Float(1.38171085e-09), Py::Float(-1.1706022e-13), Py::Float(-25374.8747), Py::Float(14.5023623), Py::Float(5.71539582), Py::Float(-0.0152309129), Py::Float(6.52441155e-05), Py::Float(-7.10806889e-08), Py::Float(2.61352698e-11), Py::Float(-25642.7656), Py::Float(-1.50409823)])), (Py::str("O2"), Py::list(vec![Py::Float(1000.0), Py::Float(3.28253784), Py::Float(0.00148308754), Py::Float(-7.57966669e-07), Py::Float(2.09470555e-10), Py::Float(-2.16717794e-14), Py::Float(-1088.45772), Py::Float(5.45323129), Py::Float(3.78245636), Py::Float(-0.00299673416), Py::Float(9.84730201e-06), Py::Float(-9.68129509e-09), Py::Float(3.24372837e-12), Py::Float(-1063.94356), Py::Float(3.65767573)])), (Py::str("N2"), Py::list(vec![Py::Float(1000.0), Py::Float(2.92664), Py::Float(0.0014879768), Py::Float(-5.68476e-07), Py::Float(1.0097038e-10), Py::Float(-6.753351e-15), Py::Float(-922.7977), Py::Float(5.980528), Py::Float(3.298677), Py::Float(0.0014082404), Py::Float(-3.963222e-06), Py::Float(5.641515e-09), Py::Float(-2.444854e-12), Py::Float(-1020.8999), Py::Float(3.950372)])), (Py::str("CO2"), Py::list(vec![Py::Float(1000.0), Py::Float(3.85746029), Py::Float(0.00441437026), Py::Float(-2.21481404e-06), Py::Float(5.23490188e-10), Py::Float(-4.72084164e-14), Py::Float(-48759.166), Py::Float(2.27163806), Py::Float(2.35677352), Py::Float(0.00898459677), Py::Float(-7.12356269e-06), Py::Float(2.45919022e-09), Py::Float(-1.43699548e-13), Py::Float(-48371.9697), Py::Float(9.90105222)])), (Py::str("H2O"), Py::list(vec![Py::Float(1000.0), Py::Float(3.03399249), Py::Float(0.00217691804), Py::Float(-1.64072518e-07), Py::Float(-9.7041987e-11), Py::Float(1.68200992e-14), Py::Float(-30004.2971), Py::Float(4.9667701), Py::Float(4.19864056), Py::Float(-0.0020364341), Py::Float(6.52040211e-06), Py::Float(-5.48797062e-09), Py::Float(1.77197817e-12), Py::Float(-30293.7267), Py::Float(-0.849032208)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PR_C1() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.4572355289213822)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PR_C2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.07779607390388846)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PUNIT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("Pa"), Py::Float(1.0)), (Py::str("kPa"), Py::Float(1000.0)), (Py::str("MPa"), Py::Float(1000000.0)), (Py::str("bar"), Py::Float(100000.0)), (Py::str("atm"), Py::Float(101325.0)), (Py::str("psi"), Py::Float(6894.757293168361))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__QU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("m3_per_day"), Py::Float(1.1574074074074073e-05)), (Py::str("Mscf_per_day"), Py::Float(0.00032774128))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SH_CORR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("sphere"), Py::str("Ranz-Marshall Sh = 2 + 0.6 Re^(1/2) Sc^(1/3)")), (Py::str("flat_plate"), Py::str("laminar flat plate (average) Sh = 0.664 Re^(1/2) Sc^(1/3)")), (Py::str("pipe"), Py::str("Linton-Sherwood turbulent pipe Sh = 0.023 Re^0.83 Sc^(1/3)"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SRK_C1() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.4274802335403414)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SRK_C2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.08664034996495772)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("s"), Py::Float(1.0)), (Py::str("min"), Py::Float(60.0)), (Py::str("h"), Py::Float(3600.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__VU() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("m3"), Py::Float(1.0)), (Py::str("bbl"), Py::Float(0.158987294928))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__antoine_rows(mut v_ant: Py, mut v_n: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4630 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4630;
out.push({ let t4631 = py::len(&v_r.clone())?; Py::Bool({ let t4632 = Py::Int(3i64); if py::eq(&t4631, &t4632) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("give three Antoine coefficients (A, B, C) per component"))?;
    if ({ let t4633 = v_n.clone(); Py::Bool({ let t4634 = Py::None; if !py::is(&t4633, &t4634) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4635 = py::len(&v_ant.clone())?; Py::Bool({ let t4636 = v_n.clone(); if py::eq(&t4635, &t4636) { true } else { false } }) }, Py::str("give one row of Antoine coefficients per component"))?;
    }
    for t4637 in py::iter(&(v_ant.clone()))? {
        v_r = t4637;
        let _ = f__fin({ let mut v = vec![]; v.extend((v_r.clone()).items()?); Py::tuple(v) })?;
        let _ = f__need({ let t4641 = py::getitem(&v_r.clone(), &Py::Int(1i64))?; Py::Bool({ let t4642 = Py::Int(0i64); if py::gt(&t4641, &t4642)? { true } else { false } }) }, Py::str("the Antoine coefficient B must be positive"))?;
    }
    return Ok(v_ant.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bed_check(mut v_dp: Py, mut v_eps: Py, mut v_vs: Py, mut v_rho: Py, mut v_mu: Py, mut v_L: Py) -> R {
    let _ = f__pos(Py::str("dp, vs, rho, mu and L"), Py::tuple(vec![v_dp.clone(), v_vs.clone(), v_rho.clone(), v_mu.clone(), v_L.clone()]))?;
    let _ = f__frac(Py::str("eps"), v_eps.clone(), Py::Bool(true), Py::Bool(true))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t5794 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t5794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t5794[0].clone();
    v_fb = t5794[1].clone();
    if ({ let t5795 = v_fa.clone(); Py::Bool({ let t5796 = Py::Int(0i64); if py::eq(&t5795, &t5796) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t5797 = v_fb.clone(); Py::Bool({ let t5798 = Py::Int(0i64); if py::eq(&t5797, &t5798) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t5799 = { let t5801 = v_fa.clone(); Py::Bool({ let t5802 = Py::Int(0i64); if py::gt(&t5801, &t5802)? { true } else { false } }) }; Py::Bool({ let t5800 = { let t5803 = v_fb.clone(); Py::Bool({ let t5804 = Py::Int(0i64); if py::gt(&t5803, &t5804)? { true } else { false } }) }; if py::ne(&t5799, &t5800) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t5805 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t5805;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t5810 = { let t5806 = v_m.clone(); Py::Bool({ let t5807 = v_a.clone(); if py::eq(&t5806, &t5807) { true } else { false } }) }; if t5810.truthy() { t5810 } else { { let t5808 = v_m.clone(); Py::Bool({ let t5809 = v_b.clone(); if py::eq(&t5808, &t5809) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t5811 = v_fm.clone(); Py::Bool({ let t5812 = Py::Int(0i64); if py::eq(&t5811, &t5812) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t5813 = { let t5815 = v_fm.clone(); Py::Bool({ let t5816 = Py::Int(0i64); if py::gt(&t5815, &t5816)? { true } else { false } }) }; Py::Bool({ let t5814 = { let t5817 = v_fa.clone(); Py::Bool({ let t5818 = Py::Int(0i64); if py::gt(&t5817, &t5818)? { true } else { false } }) }; if py::eq(&t5813, &t5814) { true } else { false } }) }).truthy() {
            let t5819 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t5819.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5819[0].clone();
            v_fa = t5819[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__comp(mut v_name: Py, mut v_v: Py) -> R {
    let _ = f__need({ let t5663 = Py::Int(2i64); Py::Bool({ let t5664 = py::len(&v_v.clone())?; if py::le(&t5663, &t5664)? { { let t5665 = Py::Int(10i64); if py::le(&t5664, &t5665)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, ": give 2 to 10 mole fractions".to_string()].concat()))?;
    let _ = f__fracs(v_name.clone(), v_v.clone(), Py::Float(1e-06))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__conv_ok(mut v_X: Py) -> R {
    let _ = f__frac(Py::str("the conversion X"), v_X.clone(), Py::Bool(false), Py::Bool(true))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cp_at(mut v_co: Py, mut v_T: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t193 = (py::slice(&py::add(&Py::list((v_co.clone()).items()?), &Py::list(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]))?, None, Some((Py::Int(4i64)).i()?), None)?).items()?;
    if t193.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t193[0].clone();
    v_b = t193[1].clone();
    v_c = t193[2].clone();
    v_d = t193[3].clone();
    return Ok(py::add(&v_a.clone(), &py::mul(&v_T.clone(), &py::add(&v_b.clone(), &py::mul(&v_T.clone(), &py::add(&v_c.clone(), &py::mul(&v_T.clone(), &v_d.clone())?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cp_int(mut v_co: Py, mut v_T1: Py, mut v_T2: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t5076 = (py::slice(&py::add(&Py::list((v_co.clone()).items()?), &Py::list(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]))?, None, Some((Py::Int(4i64)).i()?), None)?).items()?;
    if t5076.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5076[0].clone();
    v_b = t5076[1].clone();
    v_c = t5076[2].clone();
    v_d = t5076[3].clone();
    return Ok(py::add(&py::add(&py::add(&py::mul(&v_a.clone(), &py::sub(&v_T2.clone(), &v_T1.clone())?)?, &py::mul(&py::truediv(&v_b.clone(), &Py::Int(2i64))?, &py::sub(&py::pow(&v_T2.clone(), &Py::Int(2i64))?, &py::pow(&v_T1.clone(), &Py::Int(2i64))?)?)?)?, &py::mul(&py::truediv(&v_c.clone(), &Py::Int(3i64))?, &py::sub(&py::pow(&v_T2.clone(), &Py::Int(3i64))?, &py::pow(&v_T1.clone(), &Py::Int(3i64))?)?)?)?, &py::mul(&py::truediv(&v_d.clone(), &Py::Int(4i64))?, &py::sub(&py::pow(&v_T2.clone(), &Py::Int(4i64))?, &py::pow(&v_T1.clone(), &Py::Int(4i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cp_matrix(mut v_cp: Py, mut v_n: Py) -> R {
    let _ = f__need({ let t4748 = { let t4743 = py::len(&v_cp.clone())?; Py::Bool({ let t4744 = v_n.clone(); if py::eq(&t4743, &t4744) { true } else { false } }) }; if !t4748.truthy() { t4748 } else { { let t4745 = Py::Int(1i64); Py::Bool({ let t4746 = py::len(&py::getitem(&v_cp.clone(), &Py::Int(0i64))?)?; if py::le(&t4745, &t4746)? { { let t4747 = Py::Int(4i64); if py::le(&t4746, &t4747)? { true } else { false } } } else { false } }) } } }, Py::str("give one row of 1 to 4 cp coefficients (a, b, c, d) per species"))?;
    return Ok({ let mut out = vec![]; for t4749 in py::iter(&(v_cp.clone()))? { let mut v_r: Py; 
v_r = t4749;
out.push(py::add(&Py::list((v_r.clone()).items()?), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&Py::Int(4i64), &py::len(&v_r.clone())?)?)?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cubic_roots(mut v_a2: Py, mut v_a1: Py, mut v_a0: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_qq: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_p = py::sub(&v_a1.clone(), &py::truediv(&py::mul(&v_a2.clone(), &v_a2.clone())?, &Py::Float(3.0))?)?;
    v_qq = py::add(&py::sub(&py::truediv(&py::mul(&Py::Float(2.0), &py::pow(&v_a2.clone(), &Py::Int(3i64))?)?, &Py::Float(27.0))?, &py::truediv(&py::mul(&v_a2.clone(), &v_a1.clone())?, &Py::Float(3.0))?)?, &v_a0.clone())?;
    v_disc = py::add(&py::pow(&py::truediv(&v_qq.clone(), &Py::Float(2.0))?, &Py::Int(2i64))?, &py::pow(&py::truediv(&v_p.clone(), &Py::Float(3.0))?, &Py::Int(3i64))?)?;
    v_scale = py::minmax(&[py::pow(&py::abs(&py::truediv(&v_qq.clone(), &Py::Float(2.0))?)?, &Py::Int(2i64))?, py::pow(&py::abs(&py::truediv(&v_p.clone(), &Py::Float(3.0))?)?, &Py::Int(3i64))?, Py::Float(1e-300)], true)?;
    let _ = f__need({ let t5178 = py::abs(&v_disc.clone())?; Py::Bool({ let t5179 = py::mul(&Py::Float(1e-08), &v_scale.clone())?; if py::gt(&t5178, &t5179)? { true } else { false } }) }, Py::str("the state is too close to a double root of the cubic (near the spinodal or critical point)"))?;
    if ({ let t5180 = v_disc.clone(); Py::Bool({ let t5181 = Py::Int(0i64); if py::gt(&t5180, &t5181)? { true } else { false } }) }).truthy() {
        v_s = py::math1("sqrt", &v_disc.clone())?;
        v_u = (if ({ let t5182 = v_qq.clone(); Py::Bool({ let t5183 = Py::Int(0i64); if py::le(&t5182, &t5183)? { true } else { false } }) }).truthy() { py::add(&py::truediv(&py::neg(&v_qq.clone())?, &Py::Float(2.0))?, &v_s.clone())? } else { py::sub(&py::truediv(&py::neg(&v_qq.clone())?, &Py::Float(2.0))?, &v_s.clone())? });
        v_u = py::copysign(&py::pow(&py::abs(&v_u.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_u.clone())?;
        v_t = (if ({ let t5184 = v_u.clone(); Py::Bool({ let t5185 = Py::Int(0i64); if py::ne(&t5184, &t5185) { true } else { false } }) }).truthy() { py::sub(&v_u.clone(), &py::truediv(&v_p.clone(), &py::mul(&Py::Float(3.0), &v_u.clone())?)?)? } else { Py::Float(0.0) });
        v_roots = Py::list(vec![py::sub(&v_t.clone(), &py::truediv(&v_a2.clone(), &Py::Float(3.0))?)?]);
    } else {
        v_m = py::mul(&Py::Float(2.0), &py::math1("sqrt", &py::truediv(&py::neg(&v_p.clone())?, &Py::Float(3.0))?)?)?;
        v_c = py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::mul(&Py::Float(3.0), &v_qq.clone())?, &py::mul(&v_p.clone(), &v_m.clone())?)?], false)?], true)?;
        v_th = py::truediv(&py::math1("acos", &v_c.clone())?, &Py::Float(3.0))?;
        v_roots = { let mut out = vec![]; for t5186 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t5186;
out.push(py::sub(&py::mul(&v_m.clone(), &py::math1("cos", &py::sub(&v_th.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &Py::Float(3.0))?)?)?)?, &py::truediv(&v_a2.clone(), &Py::Float(3.0))?)?); } Py::list(out) };
    }
    v_out = Py::list(vec![]);
    for t5187 in py::iter(&(v_roots.clone()))? {
        v_z = t5187;
        for t5188 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
            v__ = t5188;
            v_f = py::add(&py::mul(&py::add(&py::mul(&py::add(&v_z.clone(), &v_a2.clone())?, &v_z.clone())?, &v_a1.clone())?, &v_z.clone())?, &v_a0.clone())?;
            v_d = py::add(&py::mul(&py::add(&py::mul(&Py::Float(3.0), &v_z.clone())?, &py::mul(&Py::Float(2.0), &v_a2.clone())?)?, &v_z.clone())?, &v_a1.clone())?;
            if ({ let t5189 = v_d.clone(); Py::Bool({ let t5190 = Py::Int(0i64); if py::eq(&t5189, &t5190) { true } else { false } }) }).truthy() {
                break;
            }
            v_z = py::sub(&v_z, &py::truediv(&v_f.clone(), &v_d.clone())?)?;
        }
        let _ = py::method(&v_out.clone(), "append", &[v_z.clone()])?;
    }
    return Ok(py::sorted(&v_out.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__decl_check(mut v_qi: Py, mut v_Di: Py, mut v_t: Py) -> R {
    let _ = f__pos(Py::str("qi and Di"), Py::tuple(vec![v_qi.clone(), v_Di.clone()]))?;
    let _ = f__nonneg(Py::str("t"), Py::tuple(vec![v_t.clone()]))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dist_check(mut v_alpha: Py, mut v_zF: Py, mut v_xD: Py, mut v_q: Py, mut v_xB: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_alpha.clone(), v_zF.clone(), v_xD.clone(), v_q.clone()]))?;
    let _ = f__need({ let t2919 = v_alpha.clone(); Py::Bool({ let t2920 = Py::Float(1.0); if py::gt(&t2919, &t2920)? { true } else { false } }) }, Py::str("the relative volatility alpha must exceed 1"))?;
    let _ = f__need({ let t2921 = Py::Int(0i64); Py::Bool({ let t2922 = v_zF.clone(); if py::lt(&t2921, &t2922)? { { let t2923 = v_xD.clone(); if py::lt(&t2922, &t2923)? { { let t2924 = Py::Int(1i64); if py::lt(&t2923, &t2924)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("the compositions must satisfy 0 < zF < xD < 1"))?;
    if ({ let t2925 = v_xB.clone(); Py::Bool({ let t2926 = Py::None; if !py::is(&t2925, &t2926) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2927 = Py::Int(0i64); Py::Bool({ let t2928 = v_xB.clone(); if py::lt(&t2927, &t2928)? { { let t2929 = v_zF.clone(); if py::lt(&t2928, &t2929)? { true } else { false } } } else { false } }) }, Py::str("the bottoms composition must satisfy 0 < xB < zF"))?;
    }
    let _ = f__need({ let t2930 = py::neg(&Py::Int(5i64))?; Py::Bool({ let t2931 = v_q.clone(); if py::le(&t2930, &t2931)? { { let t2932 = Py::Int(5i64); if py::le(&t2931, &t2932)? { true } else { false } } } else { false } }) }, Py::str("the feed quality q must lie between -5 and 5"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__e1(mut v_x: Py) -> R {
    let mut v_add: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    if ({ let t2730 = v_x.clone(); Py::Bool({ let t2731 = Py::Float(1.0); if py::le(&t2730, &t2731)? { true } else { false } }) }).truthy() {
        let t2732 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0), Py::Int(1i64)])).items()?;
        if t2732.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2732[0].clone();
        v_term = t2732[1].clone();
        v_n = t2732[2].clone();
        while (Py::Bool(true)).truthy() {
            v_term = py::mul(&v_term, &py::truediv(&py::neg(&v_x.clone())?, &v_n.clone())?)?;
            v_add = py::truediv(&py::neg(&v_term.clone())?, &v_n.clone())?;
            v_s = py::iadd(&v_s, &v_add.clone())?;
            if ({ let t2737 = { let t2733 = py::abs(&v_add.clone())?; Py::Bool({ let t2734 = py::mul(&Py::Float(1e-18), &py::abs(&v_s.clone())?)?; if py::le(&t2733, &t2734)? { true } else { false } }) }; if t2737.truthy() { t2737 } else { { let t2735 = v_n.clone(); Py::Bool({ let t2736 = Py::Int(200i64); if py::gt(&t2735, &t2736)? { true } else { false } }) } } }).truthy() {
                break;
            }
            v_n = py::iadd(&v_n, &Py::Int(1i64))?;
        }
        return Ok(py::add(&py::sub(&py::neg(&Py::Float(0.5772156649015329))?, &py::math1("log", &v_x.clone())?)?, &v_s.clone())?);
    }
    v_b = py::add(&v_x.clone(), &Py::Float(1.0))?;
    let t2738 = (Py::tuple(vec![Py::Float(1e+300), py::truediv(&Py::Float(1.0), &v_b.clone())?])).items()?;
    if t2738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2738[0].clone();
    v_d = t2738[1].clone();
    v_h = v_d.clone();
    for t2739 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(500i64)).i()?]) {
        v_i = t2739;
        v_an = py::mul(&py::neg(&v_i.clone())?, &v_i.clone())?;
        v_b = py::iadd(&v_b, &Py::Float(2.0))?;
        v_d = py::truediv(&Py::Float(1.0), &py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?)?;
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_de = py::mul(&v_c.clone(), &v_d.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t2740 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2741 = Py::Float(1e-16); if py::le(&t2740, &t2741)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(py::mul(&v_h.clone(), &py::math1("exp", &py::neg(&v_x.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eos(mut v_kind: Py, mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py, mut v_T: Py, mut v_P: Py) -> R {
    let mut v_RT: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lnphi: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_zl: Py = Py::None;
    let mut v_zs: Py = Py::None;
    let mut v_zv: Py = Py::None;
    let c_A = py::cell_new();
    let c_B = py::cell_new();
    let _ = f__pos(Py::str("Tc, Pc, T and P"), Py::tuple(vec![v_Tc.clone(), v_Pc.clone(), v_T.clone(), v_P.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_omega.clone()]))?;
    let _ = f__need({ let t5172 = py::neg(&Py::Float(0.5))?; Py::Bool({ let t5173 = v_omega.clone(); if py::lt(&t5172, &t5173)? { { let t5174 = Py::Float(2.0); if py::lt(&t5173, &t5174)? { true } else { false } } } else { false } }) }, Py::str("the acentric factor must lie between -0.5 and 2"))?;
    v_RT = py::mul(&c_R_GAS()?, &v_T.clone())?;
    if ({ let t5175 = v_kind.clone(); Py::Bool({ let t5176 = Py::str("PR"); if py::eq(&t5175, &t5176) { true } else { false } }) }).truthy() {
        v_b = py::truediv(&py::mul(&py::mul(&c__PR_C2()?, &c_R_GAS()?)?, &v_Tc.clone())?, &v_Pc.clone())?;
        v_k = py::sub(&py::add(&Py::Float(0.37464), &py::mul(&Py::Float(1.54226), &v_omega.clone())?)?, &py::mul(&py::mul(&Py::Float(0.26992), &v_omega.clone())?, &v_omega.clone())?)?;
        v_a = py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&c__PR_C1()?, &c_R_GAS()?)?, &c_R_GAS()?)?, &v_Tc.clone())?, &v_Tc.clone())?, &v_Pc.clone())?, &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_k.clone(), &py::sub(&Py::Float(1.0), &py::math1("sqrt", &py::truediv(&v_T.clone(), &v_Tc.clone())?)?)?)?)?, &Py::Int(2i64))?)?;
        let t5177 = (Py::tuple(vec![py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &py::mul(&v_RT.clone(), &v_RT.clone())?)?, py::truediv(&py::mul(&v_b.clone(), &v_P.clone())?, &v_RT.clone())?])).items()?;
        if t5177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::cell_set(&c_A, t5177[0].clone());
        py::cell_set(&c_B, t5177[1].clone());
        v_zs = f__cubic_roots(py::neg(&py::sub(&Py::Float(1.0), &py::cell_get(&c_B)?)?)?, py::sub(&py::sub(&py::cell_get(&c_A)?, &py::mul(&py::mul(&Py::Float(3.0), &py::cell_get(&c_B)?)?, &py::cell_get(&c_B)?)?)?, &py::mul(&Py::Float(2.0), &py::cell_get(&c_B)?)?)?, py::neg(&py::sub(&py::sub(&py::mul(&py::cell_get(&c_A)?, &py::cell_get(&c_B)?)?, &py::mul(&py::cell_get(&c_B)?, &py::cell_get(&c_B)?)?)?, &py::pow(&py::cell_get(&c_B)?, &Py::Int(3i64))?)?)?)?;
        v_s2 = py::math1("sqrt", &Py::Float(2.0))?;
        v_lnphi = { let v_P = v_P.clone(); let v_Pc = v_Pc.clone(); let v_RT = v_RT.clone(); let v_T = v_T.clone(); let v_Tc = v_Tc.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_k = v_k.clone(); let v_kind = v_kind.clone(); let v_omega = v_omega.clone(); let v_s2 = v_s2.clone(); let v_zs = v_zs.clone(); let c_A = c_A.clone(); let c_B = c_B.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_Z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::sub(&v_Z.clone(), &Py::Float(1.0))?, &py::math1("log", &py::sub(&v_Z.clone(), &py::cell_get(&c_B)?)?)?)?, &py::mul(&py::truediv(&py::cell_get(&c_A)?, &py::mul(&py::mul(&Py::Float(2.0), &v_s2.clone())?, &py::cell_get(&c_B)?)?)?, &py::math1("log", &py::truediv(&py::add(&v_Z.clone(), &py::mul(&py::add(&Py::Float(1.0), &v_s2.clone())?, &py::cell_get(&c_B)?)?)?, &py::add(&v_Z.clone(), &py::mul(&py::sub(&Py::Float(1.0), &v_s2.clone())?, &py::cell_get(&c_B)?)?)?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    } else {
        v_b = py::truediv(&py::mul(&py::mul(&c__SRK_C2()?, &c_R_GAS()?)?, &v_Tc.clone())?, &v_Pc.clone())?;
        v_m = py::sub(&py::add(&Py::Float(0.48), &py::mul(&Py::Float(1.574), &v_omega.clone())?)?, &py::mul(&py::mul(&Py::Float(0.176), &v_omega.clone())?, &v_omega.clone())?)?;
        v_a = py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&c__SRK_C1()?, &c_R_GAS()?)?, &c_R_GAS()?)?, &v_Tc.clone())?, &v_Tc.clone())?, &v_Pc.clone())?, &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_m.clone(), &py::sub(&Py::Float(1.0), &py::math1("sqrt", &py::truediv(&v_T.clone(), &v_Tc.clone())?)?)?)?)?, &Py::Int(2i64))?)?;
        let t5191 = (Py::tuple(vec![py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &py::mul(&v_RT.clone(), &v_RT.clone())?)?, py::truediv(&py::mul(&v_b.clone(), &v_P.clone())?, &v_RT.clone())?])).items()?;
        if t5191.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::cell_set(&c_A, t5191[0].clone());
        py::cell_set(&c_B, t5191[1].clone());
        v_zs = f__cubic_roots(py::neg(&Py::Float(1.0))?, py::sub(&py::sub(&py::cell_get(&c_A)?, &py::cell_get(&c_B)?)?, &py::mul(&py::cell_get(&c_B)?, &py::cell_get(&c_B)?)?)?, py::mul(&py::neg(&py::cell_get(&c_A)?)?, &py::cell_get(&c_B)?)?)?;
        v_lnphi = { let v_P = v_P.clone(); let v_Pc = v_Pc.clone(); let v_RT = v_RT.clone(); let v_T = v_T.clone(); let v_Tc = v_Tc.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_k = v_k.clone(); let v_kind = v_kind.clone(); let v_lnphi = v_lnphi.clone(); let v_m = v_m.clone(); let v_omega = v_omega.clone(); let v_s2 = v_s2.clone(); let v_zs = v_zs.clone(); let c_A = c_A.clone(); let c_B = c_B.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_Z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::sub(&v_Z.clone(), &Py::Float(1.0))?, &py::math1("log", &py::sub(&v_Z.clone(), &py::cell_get(&c_B)?)?)?)?, &py::mul(&py::truediv(&py::cell_get(&c_A)?, &py::cell_get(&c_B)?)?, &py::math1("log", &py::add(&Py::Float(1.0), &py::truediv(&py::cell_get(&c_B)?, &v_Z.clone())?)?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    }
    v_zs = { let mut out = vec![]; for t5192 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t5192;
if ({ let t5193 = v_z.clone(); Py::Bool({ let t5194 = py::cell_get(&c_B)?; if py::gt(&t5193, &t5194)? { true } else { false } }) }).truthy() { out.push(v_z.clone()); }} Py::list(out) };
    let _ = f__need(Py::Bool((v_zs.clone()).truthy()), Py::str("no physical root of the cubic"))?;
    if ({ let t5195 = py::len(&v_zs.clone())?; Py::Bool({ let t5196 = Py::Int(3i64); if py::eq(&t5195, &t5196) { true } else { false } }) }).truthy() {
        v_zs = Py::list(vec![py::getitem(&v_zs.clone(), &Py::Int(0i64))?, py::getitem(&v_zs.clone(), &Py::Int(2i64))?]);
    }
    if ({ let t5197 = py::len(&v_zs.clone())?; Py::Bool({ let t5198 = Py::Int(2i64); if py::eq(&t5197, &t5198) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5199 = py::sub(&py::getitem(&v_zs.clone(), &Py::Int(1i64))?, &py::getitem(&v_zs.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t5200 = py::mul(&Py::Float(0.0001), &py::getitem(&v_zs.clone(), &Py::Int(1i64))?)?; if py::gt(&t5199, &t5200)? { true } else { false } }) }, Py::str("the liquid and vapour roots nearly coincide"))?;
    }
    let t5201 = (Py::tuple(vec![py::getitem(&v_zs.clone(), &Py::Int(0i64))?, py::getitem(&v_zs.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
    if t5201.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_zl = t5201[0].clone();
    v_zv = t5201[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("Z_vapour"), v_zv.clone()), (Py::str("Z_liquid"), v_zl.clone()), (Py::str("phi_vapour"), py::math1("exp", &py::call(&v_lnphi.clone(), &[v_zv.clone()])?)?), (Py::str("phi_liquid"), py::math1("exp", &py::call(&v_lnphi.clone(), &[v_zl.clone()])?)?), (Py::str("n_roots"), (if ({ let t5214 = py::len(&v_zs.clone())?; Py::Bool({ let t5215 = Py::Int(1i64); if py::eq(&t5214, &t5215) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(3i64) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eps_ok(mut v_eps: Py, mut v_X: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_eps.clone()]))?;
    let _ = f__need({ let t3625 = { let t3620 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t3621 = v_eps.clone(); if py::lt(&t3620, &t3621)? { { let t3622 = Py::Int(10i64); if py::le(&t3621, &t3622)? { true } else { false } } } else { false } }) }; if !t3625.truthy() { t3625 } else { { let t3623 = py::add(&Py::Int(1i64), &py::mul(&v_eps.clone(), &v_X.clone())?)?; Py::Bool({ let t3624 = Py::Int(0i64); if py::gt(&t3623, &t3624)? { true } else { false } }) } } }, Py::str("the volume-change factor eps must exceed -1"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eq_y(mut v_alpha: Py, mut v_x: Py) -> R {
    return Ok(py::truediv(&py::mul(&v_alpha.clone(), &v_x.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::sub(&v_alpha.clone(), &Py::Int(1i64))?, &v_x.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5775 in py::iter(&(v_vals.clone()))? {
        v_v = t5775;
        let _ = f__need({ let t5777 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5777.truthy() { t5777 } else { { let t5776 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5776.truthy() { t5776 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str("every input must be a finite number"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__flame_mix(mut v_fuel: Py, mut v_excess_air: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_o2: Py = Py::None;
    let mut v_prod: Py = Py::None;
    let mut v_reac: Py = Py::None;
    let t69 = (py::getitem(&c__FLAME_FUELS()?, &v_fuel.clone())?).items()?;
    if t69.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_c = t69[0].clone();
    v_h = t69[1].clone();
    v_o = t69[2].clone();
    v_o2 = py::sub(&py::add(&v_c.clone(), &py::truediv(&v_h.clone(), &Py::Float(4.0))?)?, &py::truediv(&v_o.clone(), &Py::Float(2.0))?)?;
    v_reac = Py::dict(vec![(v_fuel.clone(), Py::Float(1.0)), (Py::str("O2"), py::mul(&py::add(&Py::Int(1i64), &v_excess_air.clone())?, &v_o2.clone())?), (Py::str("N2"), py::mul(&py::mul(&c_N2_PER_O2()?, &py::add(&Py::Int(1i64), &v_excess_air.clone())?)?, &v_o2.clone())?)]);
    v_prod = Py::dict(vec![(Py::str("CO2"), py::float(&v_c.clone())?), (Py::str("H2O"), py::truediv(&v_h.clone(), &Py::Float(2.0))?), (Py::str("O2"), py::mul(&v_excess_air.clone(), &v_o2.clone())?), (Py::str("N2"), py::mul(&py::mul(&c_N2_PER_O2()?, &py::add(&Py::Int(1i64), &v_excess_air.clone())?)?, &v_o2.clone())?)]);
    return Ok(Py::tuple(vec![v_reac.clone(), v_prod.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__formula(mut v_text: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::method(&Py::str(&py::to_str(&v_text.clone())), "strip", &[])?;
    let _ = f__need(Py::Bool((ax_engine::pyre::module_call("fullmatch", &Py::str("(?:[A-Z][a-z]?\\d*)+"), &[v_s.clone()], 0)?).truthy()), Py::str("write the fuel as a molecular formula such as CH4, C3H8 or C2H5OH"))?;
    v_counts = Py::dict(vec![]);
    for t1146 in py::iter(&(ax_engine::pyre::module_call("findall", &Py::str("([A-Z][a-z]?)(\\d*)"), &[v_s.clone()], 0)?))? {
        let t1149 = (t1146).items()?;
        if t1149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_el = t1149[0].clone();
        v_n = t1149[1].clone();
        let _ = f__need({ let t1147 = v_el.clone(); Py::Bool({ let t1148 = c_AW()?; if py::contains(&t1148, &t1147)? { true } else { false } }) }, Py::str(&["element ".to_string(), py::format(&v_el.clone(), "")?, " is not supported (fuels of C, H, O, N and S only)".to_string()].concat()))?;
        py::setitem(&v_counts.clone(), &v_el.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &(if (v_n.clone()).truthy() { py::int(&v_n.clone())? } else { Py::Int(1i64) }))?)?;
    }
    let _ = f__need({ let t1150 = py::add(&py::add(&py::method(&v_counts.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, &py::method(&v_counts.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?)?, &py::method(&v_counts.clone(), "get", &[Py::str("S"), Py::Int(0i64)])?)?; Py::Bool({ let t1151 = Py::Int(0i64); if py::gt(&t1150, &t1151)? { true } else { false } }) }, Py::str("the fuel must contain C, H or S"))?;
    return Ok({ let mut out = vec![]; for t1152 in py::iter(&(Py::tuple(vec![Py::str("C"), Py::str("H"), Py::str("O"), Py::str("N"), Py::str("S")])))? { let mut v_e: Py; 
v_e = t1152;
out.push((v_e.clone(), py::method(&v_counts.clone(), "get", &[v_e.clone(), Py::Int(0i64)])?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_v.clone()]))?;
    let _ = f__need({ let t5259 = (if (v_lo_open.clone()).truthy() { { let t5251 = v_v.clone(); Py::Bool({ let t5252 = Py::Int(0i64); if py::gt(&t5251, &t5252)? { true } else { false } }) } } else { { let t5253 = v_v.clone(); Py::Bool({ let t5254 = Py::Int(0i64); if py::ge(&t5253, &t5254)? { true } else { false } }) } }); if !t5259.truthy() { t5259 } else { (if (v_hi_open.clone()).truthy() { { let t5255 = v_v.clone(); Py::Bool({ let t5256 = Py::Int(1i64); if py::lt(&t5255, &t5256)? { true } else { false } }) } } else { { let t5257 = v_v.clone(); Py::Bool({ let t5258 = Py::Int(1i64); if py::le(&t5257, &t5258)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fracs(mut v_name: Py, mut v_v: Py, mut v_tol: Py) -> R {
    let mut v_x: Py = Py::None;
    for t5666 in py::iter(&(v_v.clone()))? {
        v_x = t5666;
        let _ = f__need({ let t5670 = py::math1("isfinite", &v_x.clone())?; if !t5670.truthy() { t5670 } else { { let t5667 = py::neg(&Py::Float(1e-15))?; Py::Bool({ let t5668 = v_x.clone(); if py::le(&t5667, &t5668)? { { let t5669 = py::add(&Py::Int(1i64), &Py::Float(1e-15))?; if py::le(&t5668, &t5669)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, ": every fraction must lie between 0 and 1".to_string()].concat()))?;
    }
    let _ = f__need({ let t5671 = py::abs(&py::sub(&py::sum(&v_v.clone(), None)?, &Py::Float(1.0))?)?; Py::Bool({ let t5672 = v_tol.clone(); if py::le(&t5671, &t5672)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, ": the fractions must sum to 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gauss(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t901 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t902 = (t901).items()?;
if t902.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t902[0].clone();
v_row = t902[1].clone();
out.push(py::add(&Py::list((v_row.clone()).items()?), &Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]))?); } Py::list(out) };
    v_scale = { let t905 = { let t = { let mut out = vec![]; for t903 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t903;
for t904 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t904;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(1.0) } else { py::minmax(&[t], true)? } }; if t905.truthy() { t905 } else { Py::Float(1.0) } };
    for t906 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t906;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        let _ = f__need({ let t907 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t908 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t907, &t908)? { true } else { false } }) }, Py::str("the balance equations are singular: they do not determine every unknown"))?;
        let t909 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t909[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t909[1].clone())?;
        for t910 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t910;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t911 = v_f.clone(); Py::Bool({ let t912 = Py::Int(0i64); if py::ne(&t911, &t912) { true } else { false } }) }).truthy() {
                for t913 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t913;
                    let t914 = py::getitem(&v_M.clone(), &v_r.clone())?; let t915 = v_k.clone();
                    py::setitem(&t914, &t915, py::sub(&py::getitem(&t914, &t915)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t916 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t916;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t917 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t917;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__h_nasa(mut v_sp: Py, mut v_T: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    v_c = py::getitem(&c__NASA()?, &v_sp.clone())?;
    v_a = (if ({ let t73 = v_T.clone(); Py::Bool({ let t74 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; if py::gt(&t73, &t74)? { true } else { false } }) }).truthy() { py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), Some((Py::Int(8i64)).i()?), None)? } else { py::slice(&v_c.clone(), Some((Py::Int(8i64)).i()?), Some((Py::Int(15i64)).i()?), None)? });
    return Ok(py::mul(&py::mul(&c_R_GAS()?, &v_T.clone())?, &py::add(&py::add(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::mul(&v_T.clone(), &py::add(&py::truediv(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?, &py::mul(&v_T.clone(), &py::add(&py::truediv(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &Py::Int(3i64))?, &py::mul(&v_T.clone(), &py::add(&py::truediv(&py::getitem(&v_a.clone(), &Py::Int(3i64))?, &Py::Int(4i64))?, &py::truediv(&py::mul(&v_T.clone(), &py::getitem(&v_a.clone(), &Py::Int(4i64))?)?, &Py::Int(5i64))?)?)?)?)?)?)?)?, &py::truediv(&py::getitem(&v_a.clone(), &Py::Int(5i64))?, &v_T.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ho_check(mut v_vecs: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&py::getitem(&v_vecs.clone(), &Py::Int(0i64))?)?;
    let _ = f__need({ let t2372 = { let t2367 = v_n.clone(); Py::Bool({ let t2368 = Py::Int(2i64); if py::ge(&t2367, &t2368)? { true } else { false } }) }; if !t2372.truthy() { t2372 } else { Py::Bool(({ let mut out = vec![]; for t2369 in py::iter(&(v_vecs.clone()))? { let mut v_v: Py; 
v_v = t2369;
out.push({ let t2370 = py::len(&v_v.clone())?; Py::Bool({ let t2371 = v_n.clone(); if py::eq(&t2370, &t2371) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the lists must have the same length (at least 2 points)"))?;
    for t2373 in py::iter(&(v_vecs.clone()))? {
        v_v = t2373;
        let _ = f__fin({ let mut v = vec![]; v.extend((v_v.clone()).items()?); Py::tuple(v) })?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kremser_frac(mut v_A: Py, mut v_N: Py) -> R {
    if ({ let t2566 = py::abs(&py::sub(&v_A.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2567 = Py::Float(1e-12); if py::le(&t2566, &t2567)? { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&v_N.clone(), &py::add(&v_N.clone(), &Py::Float(1.0))?)?);
    }
    return Ok(py::truediv(&py::sub(&py::pow(&v_A.clone(), &py::add(&v_N.clone(), &Py::Int(1i64))?)?, &v_A.clone())?, &py::sub(&py::pow(&v_A.clone(), &py::add(&v_N.clone(), &Py::Int(1i64))?)?, &Py::Float(1.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lags_poly(mut v_taus: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_p = Py::list(vec![Py::Float(1.0)]);
    for t3307 in py::iter(&(v_taus.clone()))? {
        v_t = t3307;
        v_p = f__poly_mul(v_p.clone(), Py::list(vec![v_t.clone(), Py::Float(1.0)]))?;
    }
    return Ok(v_p.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__liq_x(mut v_x: Py) -> R {
    let _ = f__need({ let t5591 = Py::Int(2i64); Py::Bool({ let t5592 = py::len(&v_x.clone())?; if py::le(&t5591, &t5592)? { { let t5593 = Py::Int(10i64); if py::le(&t5592, &t5593)? { true } else { false } } } else { false } }) }, Py::str("give 2 to 10 mole fractions"))?;
    let _ = f__fracs(Py::str("x"), v_x.clone(), Py::Float(1e-06))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5601 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5601;
out.push({ let t5602 = v_v.clone(); Py::Bool({ let t5603 = Py::Int(0i64); if py::gt(&t5602, &t5603)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("every mole fraction must be positive (use a tiny value for infinite dilution)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mt_lines(mut v_alpha: Py, mut v_zF: Py, mut v_xD: Py, mut v_xB: Py, mut v_q: Py, mut v_R: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_xp: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let t2947 = (Py::tuple(vec![py::truediv(&v_R.clone(), &py::add(&v_R.clone(), &Py::Float(1.0))?)?, py::truediv(&v_xD.clone(), &py::add(&v_R.clone(), &Py::Float(1.0))?)?])).items()?;
    if t2947.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t2947[0].clone();
    v_b1 = t2947[1].clone();
    if ({ let t2948 = py::abs(&py::sub(&v_q.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2949 = Py::Float(1e-15); if py::lt(&t2948, &t2949)? { true } else { false } }) }).truthy() {
        v_xp = v_zF.clone();
    } else {
        v_xp = py::truediv(&py::add(&v_b1.clone(), &py::truediv(&v_zF.clone(), &py::sub(&v_q.clone(), &Py::Float(1.0))?)?)?, &py::sub(&py::truediv(&v_q.clone(), &py::sub(&v_q.clone(), &Py::Float(1.0))?)?, &v_a1.clone())?)?;
    }
    v_yp = py::add(&py::mul(&v_a1.clone(), &v_xp.clone())?, &v_b1.clone())?;
    let _ = f__need({ let t2955 = { let t2950 = v_xB.clone(); Py::Bool({ let t2951 = v_xp.clone(); if py::lt(&t2950, &t2951)? { { let t2952 = v_xD.clone(); if py::lt(&t2951, &t2952)? { true } else { false } } } else { false } }) }; if !t2955.truthy() { t2955 } else { { let t2953 = v_yp.clone(); Py::Bool({ let t2954 = f__eq_y(v_alpha.clone(), v_xp.clone())?; if py::lt(&t2953, &t2954)? { true } else { false } }) } } }, Py::str("the operating lines must meet below the equilibrium curve (R above R_min)"))?;
    v_s = py::truediv(&py::sub(&v_yp.clone(), &v_xB.clone())?, &py::sub(&v_xp.clone(), &v_xB.clone())?)?;
    return Ok(Py::tuple(vec![v_a1.clone(), v_b1.clone(), v_xp.clone(), v_yp.clone(), v_s.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5445 in py::iter(&(v_vals.clone()))? {
        v_v = t5445;
        let _ = f__need({ let t5450 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5450.truthy() { t5450 } else { { let t5449 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5449.truthy() { t5449 } else { { let t5448 = py::math1("isfinite", &v_v.clone())?; if !t5448.truthy() { t5448 } else { { let t5446 = v_v.clone(); Py::Bool({ let t5447 = Py::Int(0i64); if py::ge(&t5446, &t5447)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5825 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5826 = (t5825).items()?;
if t5826.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5826[0].clone();
v_v = t5826[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pi_loop(mut v_Kc: Py, mut v_tauI: Py, mut v_K: Py, mut v_ts: Py) -> R {
    let mut v_mag: Py = Py::None;
    let mut v_ph: Py = Py::None;
    v_mag = { let v_K = v_K.clone(); let v_Kc = v_Kc.clone(); let v_tauI = v_tauI.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&py::mul(&v_Kc.clone(), &v_K.clone())?, &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &py::pow(&py::mul(&v_tauI.clone(), &v_w.clone())?, &Py::Int(2i64))?)?)?)?)?, &py::prod(&{ let mut out = vec![]; for t3791 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t3791;
out.push(py::math1("sqrt", &py::add(&Py::Float(1.0), &py::pow(&py::mul(&v_w.clone(), &v_t.clone())?, &Py::Int(2i64))?)?)?); } Py::list(out) })?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_ph = { let v_K = v_K.clone(); let v_Kc = v_Kc.clone(); let v_mag = v_mag.clone(); let v_tauI = v_tauI.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?, &py::math1("atan", &py::mul(&v_tauI.clone(), &v_w.clone())?)?)?, &py::sum(&{ let mut out = vec![]; for t3792 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t3792;
out.push(py::math1("atan", &py::mul(&v_w.clone(), &v_t.clone())?)?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::tuple(vec![v_mag.clone(), v_ph.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poly_mul(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_out = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::add(&py::len(&v_a.clone())?, &py::len(&v_b.clone())?)?, &Py::Int(1i64))?)?;
    for t3308 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? {
        let t3313 = (t3308).items()?;
        if t3313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t3313[0].clone();
        v_x = t3313[1].clone();
        for t3309 in py::iter(&(py::enumerate(&v_b.clone(), 0)?))? {
            let t3312 = (t3309).items()?;
            if t3312.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t3312[0].clone();
            v_y = t3312[1].clone();
            let t3310 = v_out.clone(); let t3311 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t3310, &t3311, py::iadd(&py::getitem(&t3310, &t3311)?, &py::mul(&v_x.clone(), &v_y.clone())?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5783 in py::iter(&(v_vals.clone()))? {
        v_v = t5783;
        let _ = f__need({ let t5788 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5788.truthy() { t5788 } else { { let t5787 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5787.truthy() { t5787 } else { { let t5786 = py::math1("isfinite", &v_v.clone())?; if !t5786.truthy() { t5786 } else { { let t5784 = v_v.clone(); Py::Bool({ let t5785 = Py::Int(0i64); if py::gt(&t5784, &t5785)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__psat(mut v_row: Py, mut v_units: Py, mut v_base: Py, mut v_T: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_fac: Py = Py::None;
    let mut v_off: Py = Py::None;
    let t4650 = (v_row.clone()).items()?;
    if t4650.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4650[0].clone();
    v_B = t4650[1].clone();
    v_C = t4650[2].clone();
    let t4651 = (py::getitem(&c__ANT_UNITS()?, &v_units.clone())?).items()?;
    if t4651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fac = t4651[0].clone();
    v_off = t4651[1].clone();
    v_d = py::add(&py::sub(&v_T.clone(), &v_off.clone())?, &v_C.clone())?;
    let _ = f__need({ let t4652 = v_d.clone(); Py::Bool({ let t4653 = Py::Int(0i64); if py::gt(&t4652, &t4653)? { true } else { false } }) }, Py::str("the temperature is below the singular point of the Antoine equation (T + C <= 0)"))?;
    return Ok(py::mul(&v_fac.clone(), &py::pow(&py::getitem(&c__ANT_BASE()?, &v_base.clone())?, &py::sub(&v_A.clone(), &py::truediv(&v_B.clone(), &v_d.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t5830 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5830.truthy() { t5830 } else { { let t5829 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5829.truthy() { t5829 } else { { let t5827 = v_x.clone(); Py::Bool({ let t5828 = Py::None; if py::is(&t5827, &t5828) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Complex(..)))).truthy() {
        return Ok(py::complex(&[f__r(py::real(&v_x.clone())?)?, f__r(py::imag(&v_x.clone())?)?])?);
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5831 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5831;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5834 = { let t5832 = v_x.clone(); Py::Bool({ let t5833 = Py::Int(0i64); if py::eq(&t5832, &t5833) { true } else { false } }) }; if t5834.truthy() { t5834 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5835 = v_x.clone(); Py::Bool({ let t5836 = Py::Int(0i64); if py::eq(&t5835, &t5836) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t5845 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5845.truthy() { t5845 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5849 = py::not(&{ let t5846 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5846.truthy() { t5846 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5849.truthy() { t5849 } else { { let t5847 = py::len(&v_a.clone())?; Py::Bool({ let t5848 = py::len(&v_b.clone())?; if py::ne(&t5847, &t5848) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5850 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5851 = (t5850).items()?;
if t5851.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5851[0].clone();
v_y = t5851[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t5852 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5852.truthy() { t5852 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5853 = v_a.clone(); Py::Bool({ let t5854 = v_b.clone(); if py::eq(&t5853, &t5854) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t5856: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5855 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5855[0].clone();
            v_b = t5855[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5856 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5857 = v_d.clone(); Py::Bool({ let t5858 = Py::Int(0i64); if py::eq(&t5857, &t5858) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5863 = { let t5859 = py::abs(&v_a.clone())?; Py::Bool({ let t5860 = Py::Float(1e-15); if py::lt(&t5859, &t5860)? { true } else { false } }) }; if !t5863.truthy() { t5863 } else { { let t5861 = py::abs(&v_b.clone())?; Py::Bool({ let t5862 = Py::Float(1e-15); if py::lt(&t5861, &t5862)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots(mut v_c: Py) -> R {
    let mut v_rs: Py = Py::None;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_rs = ax_engine::pymp::call("polyroots", &[{ let mut out = vec![]; for t1093 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t1093;
out.push(ax_engine::pymp::call("mpf", &[Py::str(&py::repr(&py::float(&v_x.clone())?))], &[])?); } Py::list(out) }], &[(String::from("maxsteps"), Py::Int(400i64)), (String::from("extraprec"), Py::Int(80i64))])?;
    }
    return Ok({ let mut out = vec![]; for t1094 in py::iter(&((if (Py::Bool(matches!(v_rs.clone(), Py::List(_)))).truthy() { v_rs.clone() } else { Py::list(vec![v_rs.clone()]) })))? { let mut v_r: Py; 
v_r = t1094;
out.push(py::complex(&[v_r.clone()])?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__routh_stable(mut v_c: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rows: Py = Py::None;
    v_c = Py::list((v_c.clone()).items()?);
    if ({ let t3317 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t3318 = Py::Int(0i64); if py::lt(&t3317, &t3318)? { true } else { false } }) }).truthy() {
        v_c = { let mut out = vec![]; for t3319 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t3319;
out.push(py::neg(&v_x.clone())?); } Py::list(out) };
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_rows = Py::list(vec![py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, py::add(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::len(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?, &py::len(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?]);
    for t3320 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t3320;
        let t3321 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t3321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3321[0].clone();
        v_b = t3321[1].clone();
        let _ = f__need({ let t3322 = py::abs(&py::getitem(&v_b.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3323 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3324 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t3324;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?)?; if py::gt(&t3322, &t3323)? { true } else { false } }) }, Py::str("the Routh array has a zero in its first column (marginal case)"))?;
        let _ = py::method(&v_rows.clone(), "append", &[py::add(&{ let mut out = vec![]; for t3325 in py::range_iter(&[(py::sub(&py::len(&v_a.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t3325;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?])?;
    }
    v_first = { let mut out = vec![]; for t3326 in py::iter(&(py::slice(&v_rows.clone(), None, Some((py::add(&v_n.clone(), &Py::Int(1i64))?).i()?), None)?))? { let mut v_r: Py; 
v_r = t3326;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3327 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t3327;
out.push({ let t3328 = py::abs(&v_x.clone())?; Py::Bool({ let t3329 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3330 in py::iter(&(v_c.clone()))? { let mut v_y: Py; 
v_y = t3330;
out.push(py::abs(&v_y.clone())?); } Py::list(out) }], true)?)?; if py::gt(&t3328, &t3329)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the Routh array has a zero in its first column (marginal case)"))?;
    return Ok(Py::Bool(({ let mut out = vec![]; for t3331 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t3331;
out.push({ let t3332 = v_x.clone(); Py::Bool({ let t3333 = Py::Int(0i64); if py::gt(&t3332, &t3333)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rr(mut v_z: Py, mut v_K: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_kmax: Py = Py::None;
    let mut v_kmin: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t5694 = { let t5686 = py::sum(&{ let mut out = vec![]; for t5688 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5689 = (t5688).items()?;
if t5689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5689[0].clone();
v_k = t5689[1].clone();
out.push(py::mul(&v_a.clone(), &v_k.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t5687 = Py::Float(1.0); if py::gt(&t5686, &t5687)? { true } else { false } }) }; if !t5694.truthy() { t5694 } else { { let t5690 = py::sum(&{ let mut out = vec![]; for t5692 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5693 = (t5692).items()?;
if t5693.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5693[0].clone();
v_k = t5693[1].clone();
out.push(py::truediv(&v_a.clone(), &v_k.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t5691 = Py::Float(1.0); if py::gt(&t5690, &t5691)? { true } else { false } }) } } }, Py::str("the mixture is not two-phase at these K-values (sum zK <= 1: all liquid; sum z/K <= 1: all vapour)"))?;
    let _ = f__need({ let t5695 = py::minmax(&[{ let mut out = vec![]; for t5697 in py::iter(&(v_K.clone()))? { let mut v_k: Py; 
v_k = t5697;
out.push(py::abs(&py::sub(&v_k.clone(), &Py::Float(1.0))?)?); } Py::list(out) }], true)?; Py::Bool({ let t5696 = Py::Float(0.01); if py::gt(&t5695, &t5696)? { true } else { false } }) }, Py::str("every K-value lies within 1 % of unity: the vapour fraction is too ill-conditioned to report"))?;
    let t5698 = (Py::tuple(vec![py::minmax(&[v_K.clone()], true)?, py::minmax(&[v_K.clone()], false)?])).items()?;
    if t5698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kmax = t5698[0].clone();
    v_kmin = t5698[1].clone();
    v_lo = (if ({ let t5699 = v_kmax.clone(); Py::Bool({ let t5700 = Py::Int(1i64); if py::gt(&t5699, &t5700)? { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::sub(&Py::Float(1.0), &v_kmax.clone())?)? } else { py::neg(&Py::Float(f64::INFINITY))? });
    v_hi = (if ({ let t5701 = v_kmin.clone(); Py::Bool({ let t5702 = Py::Int(1i64); if py::lt(&t5701, &t5702)? { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::sub(&Py::Float(1.0), &v_kmin.clone())?)? } else { Py::Float(f64::INFINITY) });
    let t5703 = (Py::tuple(vec![py::minmax(&[v_lo.clone(), Py::Float(0.0)], true)?, py::minmax(&[v_hi.clone(), Py::Float(1.0)], false)?])).items()?;
    if t5703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t5703[0].clone();
    v_hi = t5703[1].clone();
    v_g = { let v_K = v_K.clone(); let v_hi = v_hi.clone(); let v_kmax = v_kmax.clone(); let v_kmin = v_kmin.clone(); let v_lo = v_lo.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_V: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t5704 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5705 = (t5704).items()?;
if t5705.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5705[0].clone();
v_k = t5705[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &py::sub(&v_k.clone(), &Py::Float(1.0))?)?, &py::add(&Py::Float(1.0), &py::mul(&v_V.clone(), &py::sub(&v_k.clone(), &Py::Float(1.0))?)?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_V = f__bisect(v_g.clone(), v_lo.clone(), v_hi.clone(), Py::Int(500i64))?;
    v_x = { let mut out = vec![]; for t5732 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5733 = (t5732).items()?;
if t5733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5733[0].clone();
v_k = t5733[1].clone();
out.push(py::truediv(&v_a.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_V.clone(), &py::sub(&v_k.clone(), &Py::Float(1.0))?)?)?)?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_V.clone(), v_x.clone(), { let mut out = vec![]; for t5734 in py::iter(&(py::zip(&[v_K.clone(), v_x.clone()])?))? { let mut v_k: Py; let mut v_xi: Py; 
let t5735 = (t5734).items()?;
if t5735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5735[0].clone();
v_xi = t5735[1].clone();
out.push(py::mul(&v_k.clone(), &v_xi.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sherwood(mut v_geometry: Py, mut v_Re: Py, mut v_Sc: Py) -> R {
    if ({ let t2874 = v_geometry.clone(); Py::Bool({ let t2875 = Py::str("sphere"); if py::eq(&t2874, &t2875) { true } else { false } }) }).truthy() {
        return Ok(py::add(&Py::Float(2.0), &py::mul(&py::mul(&Py::Float(0.6), &py::math1("sqrt", &v_Re.clone())?)?, &py::pow(&v_Sc.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?)?);
    }
    if ({ let t2876 = v_geometry.clone(); Py::Bool({ let t2877 = Py::str("flat_plate"); if py::eq(&t2876, &t2877) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&py::mul(&Py::Float(0.664), &py::math1("sqrt", &v_Re.clone())?)?, &py::pow(&v_Sc.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?);
    }
    return Ok(py::mul(&py::mul(&Py::Float(0.023), &py::pow(&v_Re.clone(), &Py::Float(0.83))?)?, &py::pow(&v_Sc.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__square(mut v_name: Py, mut v_M: Py, mut v_n: Py, mut v_diag: Py) -> R {
    let _ = f__need({ let t5609 = { let t5604 = py::len(&v_M.clone())?; Py::Bool({ let t5605 = v_n.clone(); if py::eq(&t5604, &t5605) { true } else { false } }) }; if !t5609.truthy() { t5609 } else { Py::Bool(({ let mut out = vec![]; for t5606 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t5606;
out.push({ let t5607 = py::len(&v_r.clone())?; Py::Bool({ let t5608 = v_n.clone(); if py::eq(&t5607, &t5608) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be an n x n matrix for n = ".to_string(), py::format(&v_n.clone(), "")?, " components".to_string()].concat()))?;
    if ({ let t5610 = v_diag.clone(); Py::Bool({ let t5611 = Py::None; if !py::is(&t5610, &t5611) { true } else { false } }) }).truthy() {
        let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5612 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5612;
out.push({ let t5613 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?, &v_diag.clone())?)?; Py::Bool({ let t5614 = Py::Float(1e-12); if py::le(&t5613, &t5614)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&["the diagonal of ".to_string(), py::format(&v_name.clone(), "")?, " must be ".to_string(), py::format(&v_diag.clone(), "g")?].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stages_check(mut v_N: Py, mut v_pos: Py) -> R {
    let _ = f__need({ let t2551 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2551.truthy() { t2551 } else { { let t2548 = Py::Int(1i64); Py::Bool({ let t2549 = v_N.clone(); if py::le(&t2548, &t2549)? { { let t2550 = Py::Int(200i64); if py::le(&t2549, &t2550)? { true } else { false } } } else { false } }) } } }, Py::str("N must be a whole number of stages from 1 to 200"))?;
    let _ = f__pos(Py::str("the factors and slopes"), { let mut v = vec![]; v.extend((v_pos.clone()).items()?); Py::tuple(v) })?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_bracket(mut v_ant: Py, mut v_units: Py, mut v_base: Py, mut v_P: Py) -> R {
    let mut v_ts: Py = Py::None;
    v_ts = { let mut out = vec![]; for t4539 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4539;
out.push(f__tsat(v_r.clone(), v_units.clone(), v_base.clone(), v_P.clone())?); } Py::list(out) };
    return Ok(Py::tuple(vec![py::minmax(&[v_ts.clone()], false)?, py::minmax(&[v_ts.clone()], true)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tsat(mut v_row: Py, mut v_units: Py, mut v_base: Py, mut v_P: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_fac: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let mut v_off: Py = Py::None;
    let t4540 = (v_row.clone()).items()?;
    if t4540.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4540[0].clone();
    v_B = t4540[1].clone();
    v_C = t4540[2].clone();
    let t4541 = (py::getitem(&c__ANT_UNITS()?, &v_units.clone())?).items()?;
    if t4541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fac = t4541[0].clone();
    v_off = t4541[1].clone();
    v_lg = py::truediv(&py::math1("log", &py::truediv(&v_P.clone(), &v_fac.clone())?)?, &py::math1("log", &py::getitem(&c__ANT_BASE()?, &v_base.clone())?)?)?;
    let _ = f__need({ let t4542 = py::sub(&v_A.clone(), &v_lg.clone())?; Py::Bool({ let t4543 = Py::Int(0i64); if py::gt(&t4542, &t4543)? { true } else { false } }) }, Py::str("this pressure lies outside the Antoine equation's range"))?;
    return Ok(py::add(&py::sub(&py::truediv(&v_B.clone(), &py::sub(&v_A.clone(), &v_lg.clone())?)?, &v_C.clone())?, &v_off.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ultimate(mut v_K: Py, mut v_ts: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_ku: Py = Py::None;
    let mut v_wu: Py = Py::None;
    let _ = f__need({ let t5789 = py::len(&v_ts.clone())?; Py::Bool({ let t5790 = Py::Int(3i64); if py::ge(&t5789, &t5790)? { true } else { false } }) }, Py::str("at least three lags are needed for a finite ultimate gain"))?;
    v_f = { let v_K = v_K.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sum(&{ let mut out = vec![]; for t5791 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t5791;
out.push(py::math1("atan", &py::mul(&v_w.clone(), &v_t.clone())?)?); } Py::list(out) }, None)?, &Py::Float(std::f64::consts::PI))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_hi = py::truediv(&Py::Float(1.0), &py::minmax(&[v_ts.clone()], false)?)?;
    while ({ let t5792 = py::call(&v_f.clone(), &[v_hi.clone()])?; Py::Bool({ let t5793 = Py::Int(0i64); if py::lt(&t5792, &t5793)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Float(2.0))?;
    }
    v_wu = f__bisect(v_f.clone(), Py::Float(0.0), v_hi.clone(), Py::Int(500i64))?;
    v_ku = py::truediv(&py::prod(&{ let mut out = vec![]; for t5820 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t5820;
out.push(py::math1("sqrt", &py::add(&Py::Float(1.0), &py::pow(&py::mul(&v_wu.clone(), &v_t.clone())?, &Py::Int(2i64))?)?)?); } Py::list(out) })?, &v_K.clone())?;
    return Ok(Py::tuple(vec![v_ku.clone(), py::truediv(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_wu.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__underwood_rmin(mut v_alpha: Py, mut v_zF: Py, mut v_xD: Py, mut v_q: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_f = { let v_alpha = v_alpha.clone(); let v_q = v_q.clone(); let v_xD = v_xD.clone(); let v_zF = v_zF.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_th: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::truediv(&py::mul(&v_alpha.clone(), &v_zF.clone())?, &py::sub(&v_alpha.clone(), &v_th.clone())?)?, &py::truediv(&py::sub(&Py::Int(1i64), &v_zF.clone())?, &py::sub(&Py::Int(1i64), &v_th.clone())?)?)?, &py::sub(&Py::Int(1i64), &v_q.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2933 = (Py::tuple(vec![Py::Float(1.0), v_alpha.clone()])).items()?;
    if t2933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2933[0].clone();
    v_hi = t2933[1].clone();
    for t2934 in py::range_iter(&[(Py::Int(2000i64)).i()?]) {
        v__ = t2934;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t2939 = { let t2935 = v_m.clone(); Py::Bool({ let t2936 = v_lo.clone(); if py::le(&t2935, &t2936)? { true } else { false } }) }; if t2939.truthy() { t2939 } else { { let t2937 = v_m.clone(); Py::Bool({ let t2938 = v_hi.clone(); if py::ge(&t2937, &t2938)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t2940 = py::call(&v_f.clone(), &[v_m.clone()])?; Py::Bool({ let t2941 = Py::Int(0i64); if py::gt(&t2940, &t2941)? { true } else { false } }) }).truthy() {
            v_hi = v_m.clone();
        } else {
            v_lo = v_m.clone();
        }
    }
    v_th = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    return Ok(py::sub(&py::add(&py::truediv(&py::mul(&v_alpha.clone(), &v_xD.clone())?, &py::sub(&v_alpha.clone(), &v_th.clone())?)?, &py::truediv(&py::sub(&Py::Int(1i64), &v_xD.clone())?, &py::sub(&Py::Int(1i64), &v_th.clone())?)?)?, &Py::Float(1.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__underwood_theta(mut v_alpha: Py, mut v_z: Py, mut v_q: Py, mut v_a_lo: Py, mut v_a_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_f = { let v_a_hi = v_a_hi.clone(); let v_a_lo = v_a_lo.clone(); let v_alpha = v_alpha.clone(); let v_q = v_q.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_th: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sum(&{ let mut out = vec![]; for t3112 in py::iter(&(py::zip(&[v_alpha.clone(), v_z.clone()])?))? { let mut v_a: Py; let mut v_zi: Py; 
let t3113 = (t3112).items()?;
if t3113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3113[0].clone();
v_zi = t3113[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_zi.clone())?, &py::sub(&v_a.clone(), &v_th.clone())?)?); } Py::list(out) }, None)?, &py::sub(&Py::Int(1i64), &v_q.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t3114 = (Py::tuple(vec![v_a_lo.clone(), v_a_hi.clone()])).items()?;
    if t3114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3114[0].clone();
    v_hi = t3114[1].clone();
    for t3115 in py::range_iter(&[(Py::Int(3000i64)).i()?]) {
        v__ = t3115;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t3120 = { let t3116 = v_m.clone(); Py::Bool({ let t3117 = v_lo.clone(); if py::le(&t3116, &t3117)? { true } else { false } }) }; if t3120.truthy() { t3120 } else { { let t3118 = v_m.clone(); Py::Bool({ let t3119 = v_hi.clone(); if py::ge(&t3118, &t3119)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t3121 = py::call(&v_f.clone(), &[v_m.clone()])?; Py::Bool({ let t3122 = Py::Int(0i64); if py::gt(&t3121, &t3122)? { true } else { false } }) }).truthy() {
            v_hi = v_m.clone();
        } else {
            v_lo = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zn(mut v_ku: Py, mut v_pu: Py, mut v_controller: Py) -> R {
    if ({ let t5823 = v_controller.clone(); Py::Bool({ let t5824 = Py::str("P"); if py::eq(&t5823, &t5824) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("Ku"), v_ku.clone()), (Py::str("Pu"), v_pu.clone()), (Py::str("Kc"), py::mul(&Py::Float(0.5), &v_ku.clone())?)]))?);
    }
    if ({ let t5837 = v_controller.clone(); Py::Bool({ let t5838 = Py::str("PI"); if py::eq(&t5837, &t5838) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("Ku"), v_ku.clone()), (Py::str("Pu"), v_pu.clone()), (Py::str("Kc"), py::mul(&Py::Float(0.45), &v_ku.clone())?), (Py::str("tauI"), py::truediv(&v_pu.clone(), &Py::Float(1.2))?)]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Ku"), v_ku.clone()), (Py::str("Pu"), v_pu.clone()), (Py::str("Kc"), py::mul(&Py::Float(0.6), &v_ku.clone())?), (Py::str("tauI"), py::truediv(&v_pu.clone(), &Py::Float(2.0))?), (Py::str("tauD"), py::truediv(&v_pu.clone(), &Py::Float(8.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_abbott_virial(mut v_T: Py, mut v_P: Py, mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_Tr: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let _ = f__pos(Py::str("T, P, Tc and Pc"), Py::tuple(vec![v_T.clone(), v_P.clone(), v_Tc.clone(), v_Pc.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_omega.clone()]))?;
    v_Tr = py::truediv(&v_T.clone(), &v_Tc.clone())?;
    v_B = py::truediv(&py::mul(&py::mul(&py::add(&py::sub(&Py::Float(0.083), &py::truediv(&Py::Float(0.422), &py::pow(&v_Tr.clone(), &Py::Float(1.6))?)?)?, &py::mul(&v_omega.clone(), &py::sub(&Py::Float(0.139), &py::truediv(&Py::Float(0.172), &py::pow(&v_Tr.clone(), &Py::Float(4.2))?)?)?)?)?, &c_R_GAS()?)?, &v_Tc.clone())?, &v_Pc.clone())?;
    v_Z = py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_B.clone(), &v_P.clone())?, &py::mul(&c_R_GAS()?, &v_T.clone())?)?)?;
    let _ = f__need({ let t5502 = v_Z.clone(); Py::Bool({ let t5503 = Py::Float(0.5); if py::gt(&t5502, &t5503)? { true } else { false } }) }, Py::str("the truncated virial equation is not valid here (Z would fall below 0.5)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("B"), v_B.clone()), (Py::str("Z"), v_Z.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_absorber_nog(mut v_y1: Py, mut v_y2: Py, mut v_x2: Py, mut v_m: Py, mut v_A: Py, mut v_HOG: Py) -> R {
    let mut v_ia: Py = Py::None;
    let mut v_nog: Py = Py::None;
    let mut v_x1: Py = Py::None;
    let _ = f__pos(Py::str("m, A and HOG"), Py::tuple(vec![v_m.clone(), v_A.clone(), v_HOG.clone()]))?;
    let _ = f__nonneg(Py::str("the compositions"), Py::tuple(vec![v_y1.clone(), v_y2.clone(), v_x2.clone()]))?;
    let _ = f__need({ let t13 = v_y1.clone(); Py::Bool({ let t14 = v_y2.clone(); if py::gt(&t13, &t14)? { { let t15 = py::mul(&v_m.clone(), &v_x2.clone())?; if py::gt(&t14, &t15)? { true } else { false } } } else { false } }) }, Py::str("need y1 > y2 > m x2 (gas in richer than gas out, which is above equilibrium with the entering liquid)"))?;
    v_x1 = py::add(&v_x2.clone(), &py::truediv(&py::sub(&v_y1.clone(), &v_y2.clone())?, &py::mul(&v_A.clone(), &v_m.clone())?)?)?;
    let _ = f__need({ let t16 = v_y1.clone(); Py::Bool({ let t17 = py::mul(&v_m.clone(), &v_x1.clone())?; if py::gt(&t16, &t17)? { true } else { false } }) }, Py::str("the operating line crosses the equilibrium line: the separation is impossible at this A"))?;
    if ({ let t18 = py::abs(&py::sub(&v_A.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t19 = Py::Float(1e-12); if py::le(&t18, &t19)? { true } else { false } }) }).truthy() {
        v_nog = py::truediv(&py::sub(&v_y1.clone(), &v_y2.clone())?, &py::sub(&v_y2.clone(), &py::mul(&v_m.clone(), &v_x2.clone())?)?)?;
    } else {
        v_ia = py::truediv(&Py::Float(1.0), &v_A.clone())?;
        v_nog = py::truediv(&py::math1("log", &py::add(&py::truediv(&py::mul(&py::sub(&Py::Int(1i64), &v_ia.clone())?, &py::sub(&v_y1.clone(), &py::mul(&v_m.clone(), &v_x2.clone())?)?)?, &py::sub(&v_y2.clone(), &py::mul(&v_m.clone(), &v_x2.clone())?)?)?, &v_ia.clone())?)?, &py::sub(&Py::Int(1i64), &v_ia.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("NOG"), v_nog.clone()), (Py::str("Z"), py::mul(&v_HOG.clone(), &v_nog.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adiabatic_reactor_T(mut v_n0: Py, mut v_nu: Py, mut v_cp: Py, mut v_dH: Py, mut v_T_ref: Py, mut v_T_in: Py, mut v_key: Py, mut v_X: Py) -> R {
    let mut v_Tj: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h_feed: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nout: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_xi: Py = Py::None;
    v_n = py::len(&v_nu.clone())?;
    let _ = f__need({ let t143 = py::len(&v_n0.clone())?; Py::Bool({ let t144 = v_n.clone(); if py::eq(&t143, &t144) { true } else { false } }) }, Py::str("n0 and nu must have one entry per species"))?;
    v_rows = f__cp_matrix(v_cp.clone(), v_n.clone())?;
    let _ = f__need({ let t155 = Py::Bool(matches!(v_key.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t155.truthy() { t155 } else { { let t152 = Py::Int(1i64); Py::Bool({ let t153 = v_key.clone(); if py::le(&t152, &t153)? { { let t154 = v_n.clone(); if py::le(&t153, &t154)? { true } else { false } } } else { false } }) } } }, Py::str("key is the number (1, 2, ...) of the limiting reactant in the species list"))?;
    v_k = py::sub(&v_key.clone(), &Py::Int(1i64))?;
    let _ = f__need({ let t160 = { let t156 = py::getitem(&v_nu.clone(), &v_k.clone())?; Py::Bool({ let t157 = Py::Int(0i64); if py::lt(&t156, &t157)? { true } else { false } }) }; if !t160.truthy() { t160 } else { { let t158 = py::getitem(&v_n0.clone(), &v_k.clone())?; Py::Bool({ let t159 = Py::Int(0i64); if py::gt(&t158, &t159)? { true } else { false } }) } } }, Py::str("the key species must be a reactant present in the feed"))?;
    let _ = f__frac(Py::str("the conversion X"), v_X.clone(), Py::Bool(false), Py::Bool(false))?;
    for t173 in py::iter(&(v_n0.clone()))? {
        v_v = t173;
        let _ = f__need({ let t174 = v_v.clone(); Py::Bool({ let t175 = Py::Int(0i64); if py::ge(&t174, &t175)? { true } else { false } }) }, Py::str("feed amounts must be zero or positive"))?;
    }
    let _ = f__pos(Py::str("the temperatures"), Py::tuple(vec![v_T_ref.clone(), v_T_in.clone()]))?;
    v_xi = py::truediv(&py::mul(&v_X.clone(), &py::getitem(&v_n0.clone(), &v_k.clone())?)?, &py::neg(&py::getitem(&v_nu.clone(), &v_k.clone())?)?)?;
    v_nout = { let mut out = vec![]; for t182 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t182;
out.push(py::add(&py::getitem(&v_n0.clone(), &v_i.clone())?, &py::mul(&py::getitem(&v_nu.clone(), &v_i.clone())?, &v_xi.clone())?)?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t183 in py::iter(&(v_nout.clone()))? { let mut v_v: Py; 
v_v = t183;
out.push({ let t184 = v_v.clone(); Py::Bool({ let t185 = py::mul(&py::neg(&Py::Float(1e-12))?, &py::minmax(&[v_n0.clone()], true)?)?; if py::ge(&t184, &t185)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the conversion would consume more of a reactant than is fed"))?;
    v_h_feed = py::add(&py::sum(&{ let mut out = vec![]; for t186 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t186;
out.push(py::mul(&py::getitem(&v_n0.clone(), &v_i.clone())?, &f__cp_int(py::getitem(&v_rows.clone(), &v_i.clone())?, v_T_in.clone(), v_T_ref.clone())?)?); } Py::list(out) }, None)?, &py::mul(&v_xi.clone(), &v_dH.clone())?)?;
    v_f = { let v_T_in = v_T_in.clone(); let v_T_ref = v_T_ref.clone(); let v_X = v_X.clone(); let v_cp = v_cp.clone(); let v_dH = v_dH.clone(); let v_f = v_f.clone(); let v_h_feed = v_h_feed.clone(); let v_k = v_k.clone(); let v_key = v_key.clone(); let v_n = v_n.clone(); let v_n0 = v_n0.clone(); let v_nout = v_nout.clone(); let v_nu = v_nu.clone(); let v_rows = v_rows.clone(); let v_v = v_v.clone(); let v_xi = v_xi.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&v_h_feed.clone(), &py::sum(&{ let mut out = vec![]; for t188 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t188;
out.push(py::mul(&py::getitem(&v_nout.clone(), &v_i.clone())?, &f__cp_int(py::getitem(&v_rows.clone(), &v_i.clone())?, v_T_ref.clone(), v_T.clone())?)?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    for t189 in py::range_iter(&[(Py::Int(61i64)).i()?]) {
        v_j = t189;
        v_Tj = py::add(&Py::Float(200.0), &py::mul(&v_j.clone(), &Py::Float(50.0))?)?;
        let _ = f__need({ let t190 = py::sum(&{ let mut out = vec![]; for t192 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t192;
out.push(py::mul(&py::getitem(&v_nout.clone(), &v_i.clone())?, &f__cp_at(py::getitem(&v_rows.clone(), &v_i.clone())?, v_Tj.clone())?)?); } Py::list(out) }, None)?; Py::Bool({ let t191 = Py::Int(0i64); if py::gt(&t190, &t191)? { true } else { false } }) }, Py::str("the product heat capacity must stay positive from 200 to 3200 K"))?;
    }
    let _ = f__need({ let t194 = py::call(&v_f.clone(), &[Py::Float(200.0)])?; Py::Bool({ let t195 = Py::Int(0i64); if py::lt(&t194, &t195)? { { let t196 = py::call(&v_f.clone(), &[Py::Float(3200.0)])?; if py::lt(&t195, &t196)? { true } else { false } } } else { false } }) }, Py::str("the adiabatic outlet temperature lies outside 200 to 3200 K"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_out"), f__bisect(v_f.clone(), Py::Float(200.0), Py::Float(3200.0), Py::Int(500i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_antoine_P(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_units: Py, mut v_base: Py, mut v_T: Py) -> R {
    let _ = f__antoine_rows(Py::list(vec![Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()])]), Py::None)?;
    let _ = f__pos(Py::str("T"), Py::tuple(vec![v_T.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Psat"), f__psat(Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone()]), v_units.clone(), v_base.clone(), v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_antoine_T(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_units: Py, mut v_base: Py, mut v_P: Py) -> R {
    let _ = f__antoine_rows(Py::list(vec![Py::list(vec![v_A.clone(), v_B.clone(), v_C.clone()])]), Py::None)?;
    let _ = f__pos(Py::str("P"), Py::tuple(vec![v_P.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Tsat"), f__tsat(Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone()]), v_units.clone(), v_base.clone(), v_P.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_backpressure(mut v_q: Py, mut v_pwf: Py, mut v_q_unit: Py, mut v_p_unit: Py, mut v_pr: Py) -> R {
    let mut v_lc: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nn: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_qs: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__ho_check(Py::tuple(vec![v_q.clone(), v_pwf.clone()]))?;
    let _ = f__pos(Py::str("pr"), Py::tuple(vec![v_pr.clone()]))?;
    let _ = f__need({ let t2136 = py::len(&v_q.clone())?; Py::Bool({ let t2137 = Py::Int(2i64); if py::ge(&t2136, &t2137)? { true } else { false } }) }, Py::str("at least two test rates are needed"))?;
    v_qs = { let mut out = vec![]; for t2138 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t2138;
out.push(py::mul(&v_v.clone(), &py::getitem(&c__QU()?, &v_q_unit.clone())?)?); } Py::list(out) };
    v_ps = { let mut out = vec![]; for t2139 in py::iter(&(v_pwf.clone()))? { let mut v_v: Py; 
v_v = t2139;
out.push(py::mul(&v_v.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?); } Py::list(out) };
    let _ = f__pos(Py::str("the rates"), { let mut v = vec![]; v.extend((v_qs.clone()).items()?); Py::tuple(v) })?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2140 in py::iter(&(v_ps.clone()))? { let mut v_p: Py; 
v_p = t2140;
out.push({ let t2141 = Py::Int(0i64); Py::Bool({ let t2142 = v_p.clone(); if py::le(&t2141, &t2142)? { { let t2143 = v_pr.clone(); if py::lt(&t2142, &t2143)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("flowing pressures must lie between 0 and the reservoir pressure"))?;
    v_xs = { let mut out = vec![]; for t2144 in py::iter(&(v_ps.clone()))? { let mut v_p: Py; 
v_p = t2144;
out.push(py::math1("log10", &py::sub(&py::mul(&v_pr.clone(), &v_pr.clone())?, &py::mul(&v_p.clone(), &v_p.clone())?)?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2145 in py::iter(&(v_qs.clone()))? { let mut v_v: Py; 
v_v = t2145;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let t2146 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2146.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2146[0].clone();
    v_my = t2146[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2147 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2147;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2148 = v_sxx.clone(); Py::Bool({ let t2149 = Py::Int(0i64); if py::gt(&t2148, &t2149)? { true } else { false } }) }, Py::str("the flowing pressures must differ"))?;
    v_nn = py::truediv(&py::sum(&{ let mut out = vec![]; for t2150 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2151 = (t2150).items()?;
if t2151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2151[0].clone();
v_y = t2151[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_lc = py::sub(&v_my.clone(), &py::mul(&v_nn.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t2152 = Py::Float(0.3); Py::Bool({ let t2153 = v_nn.clone(); if py::lt(&t2152, &t2153)? { { let t2154 = Py::Float(1.5); if py::lt(&t2153, &t2154)? { true } else { false } } } else { false } }) }, Py::str("the back-pressure exponent n falls outside 0.3 to 1.5: the data do not fit the back-pressure equation"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_nn.clone()), (Py::str("log10_C"), v_lc.clone()), (Py::str("AOF"), py::pow(&Py::Float(10.0), &py::add(&v_lc.clone(), &py::mul(&v_nn.clone(), &py::math1("log10", &py::mul(&v_pr.clone(), &v_pr.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_batch_coil_cooling(mut v_M: Py, mut v_cp: Py, mut v_W: Py, mut v_cp_c: Py, mut v_t_in: Py, mut v_UA: Py, mut v_T0: Py, mut v_t: Py) -> R {
    let mut v_eff: Py = Py::None;
    let _ = f__pos(Py::str("M, cp, W, cp_c and UA"), Py::tuple(vec![v_M.clone(), v_cp.clone(), v_W.clone(), v_cp_c.clone(), v_UA.clone()]))?;
    let _ = f__pos(Py::str("the temperatures"), Py::tuple(vec![v_t_in.clone(), v_T0.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_eff = py::sub(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::neg(&v_UA.clone())?, &py::mul(&v_W.clone(), &v_cp_c.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), py::add(&v_t_in.clone(), &py::mul(&py::sub(&v_T0.clone(), &v_t_in.clone())?, &py::math1("exp", &py::truediv(&py::mul(&py::mul(&py::mul(&py::neg(&v_W.clone())?, &v_cp_c.clone())?, &v_eff.clone())?, &v_t.clone())?, &py::mul(&v_M.clone(), &v_cp.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_batch_conv_1(mut v_k: Py, mut v_t: Py) -> R {
    let _ = f__pos(Py::str("k"), Py::tuple(vec![v_k.clone()]))?;
    let _ = f__nonneg(Py::str("t"), Py::tuple(vec![v_t.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_batch_conv_2(mut v_k: Py, mut v_CA0: Py, mut v_t: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("k and CA0"), Py::tuple(vec![v_k.clone(), v_CA0.clone()]))?;
    let _ = f__nonneg(Py::str("t"), Py::tuple(vec![v_t.clone()]))?;
    v_d = py::mul(&py::mul(&v_k.clone(), &v_CA0.clone())?, &v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::truediv(&v_d.clone(), &py::add(&Py::Float(1.0), &v_d.clone())?)?), (Py::str("CA"), py::truediv(&v_CA0.clone(), &py::add(&Py::Float(1.0), &v_d.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_batch_heating_T(mut v_m: Py, mut v_cp: Py, mut v_UA: Py, mut v_T_s: Py, mut v_T0: Py, mut v_t: Py) -> R {
    let _ = f__pos(Py::str("m, cp and UA"), Py::tuple(vec![v_m.clone(), v_cp.clone(), v_UA.clone()]))?;
    let _ = f__pos(Py::str("the temperatures"), Py::tuple(vec![v_T_s.clone(), v_T0.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), py::sub(&v_T_s.clone(), &py::mul(&py::sub(&v_T_s.clone(), &v_T0.clone())?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_UA.clone())?, &v_t.clone())?, &py::mul(&v_m.clone(), &v_cp.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_batch_heating_time(mut v_m: Py, mut v_cp: Py, mut v_UA: Py, mut v_T_s: Py, mut v_T0: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::str("m, cp and UA"), Py::tuple(vec![v_m.clone(), v_cp.clone(), v_UA.clone()]))?;
    let _ = f__pos(Py::str("the temperatures"), Py::tuple(vec![v_T_s.clone(), v_T0.clone(), v_T.clone()]))?;
    let _ = f__need({ let t594 = { let t589 = py::minmax(&[v_T0.clone(), v_T_s.clone()], false)?; Py::Bool({ let t590 = v_T.clone(); if py::le(&t589, &t590)? { { let t591 = py::minmax(&[v_T0.clone(), v_T_s.clone()], true)?; if py::lt(&t590, &t591)? { true } else { false } } } else { false } }) }; if !t594.truthy() { t594 } else { { let t592 = v_T0.clone(); Py::Bool({ let t593 = v_T_s.clone(); if py::ne(&t592, &t593) { true } else { false } }) } } }, Py::str("T must lie between T0 and the medium temperature T_s"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), py::mul(&py::truediv(&py::mul(&v_m.clone(), &v_cp.clone())?, &v_UA.clone())?, &py::math1("log", &py::truediv(&py::sub(&v_T_s.clone(), &v_T0.clone())?, &py::sub(&v_T_s.clone(), &v_T.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_batch_time_1(mut v_k: Py, mut v_X: Py) -> R {
    let _ = f__pos(Py::str("k"), Py::tuple(vec![v_k.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), py::truediv(&py::neg(&py::math1("log1p", &py::neg(&v_X.clone())?)?)?, &v_k.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_batch_time_2(mut v_k: Py, mut v_CA0: Py, mut v_X: Py) -> R {
    let _ = f__pos(Py::str("k and CA0"), Py::tuple(vec![v_k.clone(), v_CA0.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), py::truediv(&v_X.clone(), &py::mul(&py::mul(&v_k.clone(), &v_CA0.clone())?, &py::sub(&Py::Float(1.0), &v_X.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_binary_separator(mut v_F: Py, mut v_zF: Py, mut v_xD: Py, mut v_xB: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the feed flow F"), Py::tuple(vec![v_F.clone()]))?;
    for t817 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("zF"), v_zF.clone()]), Py::tuple(vec![Py::str("xD"), v_xD.clone()]), Py::tuple(vec![Py::str("xB"), v_xB.clone()])])))? {
        let t830 = (t817).items()?;
        if t830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t830[0].clone();
        v_v = t830[1].clone();
        let _ = f__frac(v_nm.clone(), v_v.clone(), Py::Bool(false), Py::Bool(false))?;
    }
    let _ = f__need({ let t831 = v_xB.clone(); Py::Bool({ let t832 = v_zF.clone(); if py::lt(&t831, &t832)? { { let t833 = v_xD.clone(); if py::lt(&t832, &t833)? { true } else { false } } } else { false } }) }, Py::str("the compositions must satisfy xB < zF < xD (light component enriched in the distillate)"))?;
    v_D = py::truediv(&py::mul(&v_F.clone(), &py::sub(&v_zF.clone(), &v_xB.clone())?)?, &py::sub(&v_xD.clone(), &v_xB.clone())?)?;
    v_B = py::sub(&v_F.clone(), &v_D.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("B"), v_B.clone()), (Py::str("recovery_D"), py::truediv(&py::mul(&v_D.clone(), &v_xD.clone())?, &py::mul(&v_F.clone(), &v_zF.clone())?)?), (Py::str("recovery_B"), (if ({ let t846 = v_zF.clone(); Py::Bool({ let t847 = Py::Int(1i64); if py::lt(&t846, &t847)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_B.clone(), &py::sub(&Py::Int(1i64), &v_xB.clone())?)?, &py::mul(&v_F.clone(), &py::sub(&Py::Int(1i64), &v_zF.clone())?)?)? } else { Py::Float(1.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_blending(mut v_W: Py, mut v_p: Py, mut v_P: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_row: Py = Py::None;
    v_n = py::len(&v_W.clone())?;
    let _ = f__need({ let t883 = { let t874 = Py::Int(2i64); Py::Bool({ let t875 = v_n.clone(); if py::le(&t874, &t875)? { { let t876 = Py::Int(6i64); if py::le(&t875, &t876)? { true } else { false } } } else { false } }) }; if !t883.truthy() { t883 } else { { let t882 = Py::Bool(({ let mut out = vec![]; for t877 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t877;
out.push({ let t878 = py::len(&v_r.clone())?; Py::Bool({ let t879 = v_n.clone(); if py::eq(&t878, &t879) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t882.truthy() { t882 } else { { let t880 = py::len(&v_p.clone())?; Py::Bool({ let t881 = v_n.clone(); if py::eq(&t880, &t881) { true } else { false } }) } } } } }, Py::str("give as many feed streams as components (2 to 6): W is n x n and p has n entries"))?;
    for t884 in py::iter(&(py::enumerate(&v_W.clone(), 0)?))? {
        let t892 = (t884).items()?;
        if t892.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t892[0].clone();
        v_row = t892[1].clone();
        let _ = f__fracs(Py::str(&["feed stream ".to_string(), py::format(&py::add(&v_i.clone(), &Py::Int(1i64))?, "")?].concat()), v_row.clone(), Py::Float(1e-06))?;
    }
    let _ = f__fracs(Py::str("product composition p"), v_p.clone(), Py::Float(1e-06))?;
    let _ = f__pos(Py::str("the product flow P"), Py::tuple(vec![v_P.clone()]))?;
    v_A = { let mut out = vec![]; for t899 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t899;
out.push({ let mut out = vec![]; for t900 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t900;
out.push(py::getitem(&py::getitem(&v_W.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }); } Py::list(out) };
    v_F = f__gauss(v_A.clone(), { let mut out = vec![]; for t918 in py::iter(&(v_p.clone()))? { let mut v_x: Py; 
v_x = t918;
out.push(py::mul(&v_P.clone(), &v_x.clone())?); } Py::list(out) })?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t919 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t919;
out.push({ let t920 = v_f.clone(); Py::Bool({ let t921 = py::mul(&py::neg(&Py::Float(1e-09))?, &v_P.clone())?; if py::ge(&t920, &t921)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("this product cannot be made from these feeds (a feed flow would be negative)"))?;
    v_F = { let mut out = vec![]; for t922 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t922;
out.push((if ({ let t923 = py::abs(&v_f.clone())?; Py::Bool({ let t924 = py::mul(&Py::Float(1e-12), &v_P.clone())?; if py::le(&t923, &t924)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_f.clone() })); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("F1"), py::getitem(&v_F.clone(), &Py::Int(0i64))?), (Py::str("F"), v_F.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bubble_P(mut v_x: Py, mut v_ant: Py, mut v_units: Py, mut v_base: Py, mut v_T: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let _ = f__comp(Py::str("x"), v_x.clone())?;
    let _ = f__antoine_rows(v_ant.clone(), py::len(&v_x.clone())?)?;
    let _ = f__pos(Py::str("T"), Py::tuple(vec![v_T.clone()]))?;
    v_ps = { let mut out = vec![]; for t4277 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4277;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    v_P = py::sum(&{ let mut out = vec![]; for t4282 in py::iter(&(py::zip(&[v_x.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4283 = (t4282).items()?;
if t4283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4283[0].clone();
v_b = t4283[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("y"), { let mut out = vec![]; for t4296 in py::iter(&(py::zip(&[v_x.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4297 = (t4296).items()?;
if t4297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4297[0].clone();
v_b = t4297[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_b.clone())?, &v_P.clone())?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bubble_T(mut v_x: Py, mut v_ant: Py, mut v_units: Py, mut v_base: Py, mut v_P: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let _ = f__comp(Py::str("x"), v_x.clone())?;
    let _ = f__antoine_rows(v_ant.clone(), py::len(&v_x.clone())?)?;
    let _ = f__pos(Py::str("P"), Py::tuple(vec![v_P.clone()]))?;
    let t4358 = (f__t_bracket(v_ant.clone(), v_units.clone(), v_base.clone(), v_P.clone())?).items()?;
    if t4358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4358[0].clone();
    v_hi = t4358[1].clone();
    v_f = { let v_P = v_P.clone(); let v_ant = v_ant.clone(); let v_base = v_base.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_units = v_units.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::truediv(&py::sum(&{ let mut out = vec![]; for t4359 in py::iter(&(py::zip(&[v_x.clone(), v_ant.clone()])?))? { let mut v_a: Py; let mut v_r: Py; 
let t4360 = (t4359).items()?;
if t4360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4360[0].clone();
v_r = t4360[1].clone();
out.push(py::mul(&v_a.clone(), &f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?)?); } Py::list(out) }, None)?, &v_P.clone())?, &Py::Float(1.0))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_T = (if ({ let t4365 = v_hi.clone(); Py::Bool({ let t4366 = v_lo.clone(); if py::eq(&t4365, &t4366) { true } else { false } }) }).truthy() { v_lo.clone() } else { f__bisect(v_f.clone(), v_lo.clone(), v_hi.clone(), Py::Int(500i64))? });
    v_ps = { let mut out = vec![]; for t4393 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4393;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("y"), { let mut out = vec![]; for t4406 in py::iter(&(py::zip(&[v_x.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4407 = (t4406).items()?;
if t4407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4407[0].clone();
v_b = t4407[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_b.clone())?, &v_P.clone())?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_carman(mut v_dp: Py, mut v_eps: Py, mut v_vs: Py, mut v_rho: Py, mut v_mu: Py, mut v_L: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let _ = f__bed_check(v_dp.clone(), v_eps.clone(), v_vs.clone(), v_rho.clone(), v_mu.clone(), v_L.clone())?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_dp.clone(), &v_rho.clone())?, &v_vs.clone())?, &v_mu.clone())?;
    v_fp = py::truediv(&py::mul(&py::add(&Py::Float(180.0), &py::mul(&Py::Float(2.871), &py::pow(&py::truediv(&v_Re.clone(), &py::sub(&Py::Float(1.0), &v_eps.clone())?)?, &Py::Float(0.9))?)?)?, &py::pow(&py::sub(&Py::Float(1.0), &v_eps.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::pow(&v_eps.clone(), &Py::Int(3i64))?, &v_Re.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dP"), py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&v_fp.clone(), &v_rho.clone())?, &v_vs.clone())?, &v_vs.clone())?, &v_L.clone())?, &v_dp.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_closed_loop_stability(mut v_num: Py, mut v_den: Py, mut v_Kc: Py) -> R {
    let mut v_char: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let mut v_n_rhp: Py = Py::None;
    let mut v_rs: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(v_Kc.clone()); v.extend((v_num.clone()).items()?); v.extend((v_den.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];   if !d.is_empty() { return Err(PyErr::Type("_fin() got an unexpected keyword argument".into())) } f__fin(Py::tuple(v.get(0..).map(|s| s.to_vec()).unwrap_or_default()))? };
    while ({ let t1076 = { let t1072 = py::len(&v_den.clone())?; Py::Bool({ let t1073 = Py::Int(1i64); if py::gt(&t1072, &t1073)? { true } else { false } }) }; if !t1076.truthy() { t1076 } else { { let t1074 = py::getitem(&v_den.clone(), &Py::Int(0i64))?; Py::Bool({ let t1075 = Py::Int(0i64); if py::eq(&t1074, &t1075) { true } else { false } }) } } }).truthy() {
        v_den = py::slice(&v_den.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    while ({ let t1081 = { let t1077 = py::len(&v_num.clone())?; Py::Bool({ let t1078 = Py::Int(1i64); if py::gt(&t1077, &t1078)? { true } else { false } }) }; if !t1081.truthy() { t1081 } else { { let t1079 = py::getitem(&v_num.clone(), &Py::Int(0i64))?; Py::Bool({ let t1080 = Py::Int(0i64); if py::eq(&t1079, &t1080) { true } else { false } }) } } }).truthy() {
        v_num = py::slice(&v_num.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    let _ = f__need({ let t1082 = Py::Int(2i64); Py::Bool({ let t1083 = py::len(&v_den.clone())?; if py::le(&t1082, &t1083)? { { let t1084 = Py::Int(9i64); if py::le(&t1083, &t1084)? { true } else { false } } } else { false } }) }, Py::str("the denominator must have degree 1 to 8"))?;
    let _ = f__need({ let t1085 = py::len(&v_num.clone())?; Py::Bool({ let t1086 = py::len(&v_den.clone())?; if py::le(&t1085, &t1086)? { true } else { false } }) }, Py::str("the process must be proper (numerator degree <= denominator degree)"))?;
    v_char = Py::list((v_den.clone()).items()?);
    for t1087 in py::iter(&(py::enumerate(&v_num.clone(), 0)?))? {
        let t1090 = (t1087).items()?;
        if t1090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1090[0].clone();
        v_v = t1090[1].clone();
        let t1088 = v_char.clone(); let t1089 = py::add(&py::sub(&py::len(&v_den.clone())?, &py::len(&v_num.clone())?)?, &v_i.clone())?;
        py::setitem(&t1088, &t1089, py::iadd(&py::getitem(&t1088, &t1089)?, &py::mul(&v_Kc.clone(), &v_v.clone())?)?)?;
    }
    let _ = f__need({ let t1091 = py::getitem(&v_char.clone(), &Py::Int(0i64))?; Py::Bool({ let t1092 = Py::Int(0i64); if py::ne(&t1091, &t1092) { true } else { false } }) }, Py::str("the closed-loop characteristic polynomial loses its leading term"))?;
    v_rs = f__roots(v_char.clone())?;
    v_scale = py::minmax(&[{ let mut out = vec![]; for t1095 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t1095;
out.push(py::hypot(&[py::real(&v_r.clone())?, py::imag(&v_r.clone())?])?); } Py::list(out) }], true)?;
    for t1096 in py::range_iter(&[(py::len(&v_rs.clone())?).i()?]) {
        v_i = t1096;
        for t1097 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_rs.clone())?).i()?]) {
            v_j = t1097;
            let _ = f__need({ let t1098 = py::hypot(&[py::sub(&py::real(&py::getitem(&v_rs.clone(), &v_i.clone())?)?, &py::real(&py::getitem(&v_rs.clone(), &v_j.clone())?)?)?, py::sub(&py::imag(&py::getitem(&v_rs.clone(), &v_i.clone())?)?, &py::imag(&py::getitem(&v_rs.clone(), &v_j.clone())?)?)?])?; Py::Bool({ let t1099 = py::mul(&Py::Float(0.001), &v_scale.clone())?; if py::gt(&t1098, &t1099)? { true } else { false } }) }, Py::str("closed-loop poles nearly repeated: refused"))?;
        }
    }
    v_mr = py::minmax(&[{ let mut out = vec![]; for t1100 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t1100;
out.push(py::real(&v_r.clone())?); } Py::list(out) }], true)?;
    let _ = f__need({ let t1101 = py::abs(&v_mr.clone())?; Py::Bool({ let t1102 = py::mul(&Py::Float(1e-06), &v_scale.clone())?; if py::gt(&t1101, &t1102)? { true } else { false } }) }, Py::str("a closed-loop pole lies on the imaginary axis (marginal stability)"))?;
    v_n_rhp = py::sum(&{ let mut out = vec![]; for t1103 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t1103;
if ({ let t1104 = py::real(&v_r.clone())?; Py::Bool({ let t1105 = Py::Int(0i64); if py::gt(&t1104, &t1105)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("max_real"), v_mr.clone()), (Py::str("n_rhp"), v_n_rhp.clone()), (Py::str("stable"), (if ({ let t1118 = v_n_rhp.clone(); Py::Bool({ let t1119 = Py::Int(0i64); if py::eq(&t1118, &t1119) { true } else { false } }) }).truthy() { Py::str("stable") } else { Py::str("unstable") }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combustion(mut v_fuel: Py, mut v_excess_air: Py) -> R {
    let mut v_air: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dry: Py = Py::None;
    let mut v_mw: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o2_in: Py = Py::None;
    let mut v_o2_th: Py = Py::None;
    let mut v_wet: Py = Py::None;
    v_c = f__formula(v_fuel.clone())?;
    let _ = f__fin(Py::tuple(vec![v_excess_air.clone()]))?;
    let _ = f__need({ let t1156 = Py::Int(0i64); Py::Bool({ let t1157 = v_excess_air.clone(); if py::le(&t1156, &t1157)? { { let t1158 = Py::Int(5i64); if py::le(&t1157, &t1158)? { true } else { false } } } else { false } }) }, Py::str("the excess air must be between 0 and 500 %"))?;
    v_o2_th = py::sub(&py::add(&py::add(&py::getitem(&v_c.clone(), &Py::str("C"))?, &py::truediv(&py::getitem(&v_c.clone(), &Py::str("H"))?, &Py::Float(4.0))?)?, &py::getitem(&v_c.clone(), &Py::str("S"))?)?, &py::truediv(&py::getitem(&v_c.clone(), &Py::str("O"))?, &Py::Float(2.0))?)?;
    let _ = f__need({ let t1159 = v_o2_th.clone(); Py::Bool({ let t1160 = Py::Int(0i64); if py::gt(&t1159, &t1160)? { true } else { false } }) }, Py::str("this formula needs no oxygen to burn"))?;
    v_o2_in = py::mul(&py::add(&Py::Float(1.0), &v_excess_air.clone())?, &v_o2_th.clone())?;
    v_n = Py::dict(vec![(Py::str("CO2"), py::float(&py::getitem(&v_c.clone(), &Py::str("C"))?)?), (Py::str("H2O"), py::truediv(&py::getitem(&v_c.clone(), &Py::str("H"))?, &Py::Float(2.0))?), (Py::str("SO2"), py::float(&py::getitem(&v_c.clone(), &Py::str("S"))?)?), (Py::str("O2"), py::mul(&v_excess_air.clone(), &v_o2_th.clone())?), (Py::str("N2"), py::add(&py::mul(&c_N2_PER_O2()?, &v_o2_in.clone())?, &py::truediv(&py::getitem(&v_c.clone(), &Py::str("N"))?, &Py::Float(2.0))?)?)]);
    v_wet = py::sum(&py::method(&v_n.clone(), "values", &[])?, None)?;
    v_dry = py::sub(&v_wet.clone(), &py::getitem(&v_n.clone(), &Py::str("H2O"))?)?;
    v_mw = py::sum(&{ let mut out = vec![]; for t1161 in py::iter(&(py::method(&v_c.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_k: Py; 
let t1162 = (t1161).items()?;
if t1162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t1162[0].clone();
v_k = t1162[1].clone();
out.push(py::mul(&py::getitem(&c_AW()?, &v_e.clone())?, &v_k.clone())?); } Py::list(out) }, None)?;
    v_air = py::truediv(&v_o2_in.clone(), &Py::Float(0.21))?;
    return Ok(f__out(Py::dict(vec![(Py::str("O2_theoretical"), v_o2_th.clone()), (Py::str("air"), v_air.clone()), (Py::str("n_CO2"), py::getitem(&v_n.clone(), &Py::str("CO2"))?), (Py::str("n_H2O"), py::getitem(&v_n.clone(), &Py::str("H2O"))?), (Py::str("n_SO2"), py::getitem(&v_n.clone(), &Py::str("SO2"))?), (Py::str("n_O2"), py::getitem(&v_n.clone(), &Py::str("O2"))?), (Py::str("n_N2"), py::getitem(&v_n.clone(), &Py::str("N2"))?), (Py::str("y_CO2_dry"), py::truediv(&py::getitem(&v_n.clone(), &Py::str("CO2"))?, &v_dry.clone())?), (Py::str("y_O2_dry"), py::truediv(&py::getitem(&v_n.clone(), &Py::str("O2"))?, &v_dry.clone())?), (Py::str("y_H2O_wet"), py::truediv(&py::getitem(&v_n.clone(), &Py::str("H2O"))?, &v_wet.clone())?), (Py::str("AFR_mass"), py::truediv(&py::mul(&v_air.clone(), &c_MW_AIR()?)?, &v_mw.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_countercurrent_extraction(mut v_F: Py, mut v_S: Py, mut v_K: Py, mut v_N: Py, mut v_XF: Py, mut v_YS: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_XN: Py = Py::None;
    let _ = f__stages_check(v_N.clone(), Py::tuple(vec![v_F.clone(), v_S.clone(), v_K.clone()]))?;
    let _ = f__nonneg(Py::str("the solute ratios"), Py::tuple(vec![v_XF.clone(), v_YS.clone()]))?;
    let _ = f__need({ let t1217 = v_XF.clone(); Py::Bool({ let t1218 = py::truediv(&v_YS.clone(), &v_K.clone())?; if py::gt(&t1217, &t1218)? { true } else { false } }) }, Py::str("no extraction: the feed is not richer than equilibrium with the entering solvent"))?;
    v_E = py::truediv(&py::mul(&v_K.clone(), &v_S.clone())?, &v_F.clone())?;
    v_XN = py::sub(&v_XF.clone(), &py::mul(&f__kremser_frac(v_E.clone(), v_N.clone())?, &py::sub(&v_XF.clone(), &py::truediv(&v_YS.clone(), &v_K.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("XN"), v_XN.clone()), (Py::str("Y1"), py::add(&v_YS.clone(), &py::truediv(&py::mul(&v_F.clone(), &py::sub(&v_XF.clone(), &v_XN.clone())?)?, &v_S.clone())?)?), (Py::str("recovery"), py::truediv(&py::sub(&v_XF.clone(), &v_XN.clone())?, &v_XF.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crosscurrent_extraction(mut v_F: Py, mut v_S: Py, mut v_K: Py, mut v_N: Py, mut v_XF: Py) -> R {
    let mut v_XN: Py = Py::None;
    let _ = f__stages_check(v_N.clone(), Py::tuple(vec![v_F.clone(), v_S.clone(), v_K.clone()]))?;
    let _ = f__pos(Py::str("XF"), Py::tuple(vec![v_XF.clone()]))?;
    v_XN = py::truediv(&v_XF.clone(), &py::pow(&py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_K.clone(), &v_S.clone())?, &v_F.clone())?)?, &v_N.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("XN"), v_XN.clone()), (Py::str("recovery"), py::sub(&Py::Float(1.0), &py::truediv(&v_XN.clone(), &v_XF.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cstr_conv_1(mut v_v0: Py, mut v_V: Py, mut v_k: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("v0, V and k"), Py::tuple(vec![v_v0.clone(), v_V.clone(), v_k.clone()]))?;
    v_d = py::truediv(&py::mul(&v_k.clone(), &v_V.clone())?, &v_v0.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::truediv(&v_d.clone(), &py::add(&Py::Float(1.0), &v_d.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cstr_conv_2(mut v_v0: Py, mut v_V: Py, mut v_k: Py, mut v_CA0: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("v0, V, k and CA0"), Py::tuple(vec![v_v0.clone(), v_V.clone(), v_k.clone(), v_CA0.clone()]))?;
    v_d = py::mul(&py::truediv(&py::mul(&v_k.clone(), &v_V.clone())?, &v_v0.clone())?, &v_CA0.clone())?;
    v_c = py::truediv(&Py::Float(2.0), &py::add(&Py::Float(1.0), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::mul(&Py::Float(4.0), &v_d.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::sub(&Py::Float(1.0), &v_c.clone())?), (Py::str("CA"), py::mul(&v_CA0.clone(), &v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cstr_volume_1(mut v_v0: Py, mut v_k: Py, mut v_X: Py) -> R {
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("v0 and k"), Py::tuple(vec![v_v0.clone(), v_k.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    v_tau = py::truediv(&v_X.clone(), &py::mul(&v_k.clone(), &py::sub(&Py::Float(1.0), &v_X.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::mul(&v_v0.clone(), &v_tau.clone())?), (Py::str("tau"), v_tau.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cstr_volume_2(mut v_v0: Py, mut v_k: Py, mut v_CA0: Py, mut v_X: Py) -> R {
    let mut v_tau: Py = Py::None;
    let _ = f__pos(Py::str("v0, k and CA0"), Py::tuple(vec![v_v0.clone(), v_k.clone(), v_CA0.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    v_tau = py::truediv(&v_X.clone(), &py::mul(&py::mul(&v_k.clone(), &v_CA0.clone())?, &py::pow(&py::sub(&Py::Int(1i64), &v_X.clone())?, &Py::Int(2i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::mul(&v_v0.clone(), &v_tau.clone())?), (Py::str("tau"), v_tau.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cstrs_series_1(mut v_v0: Py, mut v_V: Py, mut v_k: Py, mut v_N: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("v0, V and k"), Py::tuple(vec![v_v0.clone(), v_V.clone(), v_k.clone()]))?;
    let _ = f__need({ let t1516 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1516.truthy() { t1516 } else { { let t1513 = Py::Int(1i64); Py::Bool({ let t1514 = v_N.clone(); if py::le(&t1513, &t1514)? { { let t1515 = Py::Int(50i64); if py::le(&t1514, &t1515)? { true } else { false } } } else { false } }) } } }, Py::str("N must be a whole number of tanks from 1 to 50"))?;
    v_d = py::truediv(&py::mul(&v_k.clone(), &v_V.clone())?, &v_v0.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_N.clone())?, &py::math1("log1p", &v_d.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cstrs_series_2(mut v_v0: Py, mut v_V: Py, mut v_k: Py, mut v_CA0: Py, mut v_N: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("v0, V, k and CA0"), Py::tuple(vec![v_v0.clone(), v_V.clone(), v_k.clone(), v_CA0.clone()]))?;
    let _ = f__need({ let t1564 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1564.truthy() { t1564 } else { { let t1561 = Py::Int(1i64); Py::Bool({ let t1562 = v_N.clone(); if py::le(&t1561, &t1562)? { { let t1563 = Py::Int(50i64); if py::le(&t1562, &t1563)? { true } else { false } } } else { false } }) } } }, Py::str("N must be a whole number of tanks from 1 to 50"))?;
    v_d = py::mul(&py::truediv(&py::mul(&v_k.clone(), &v_V.clone())?, &v_v0.clone())?, &v_CA0.clone())?;
    v_c = Py::Float(1.0);
    for t1565 in py::range_iter(&[(v_N.clone()).i()?]) {
        v__ = t1565;
        v_c = py::truediv(&py::mul(&Py::Float(2.0), &v_c.clone())?, &py::add(&Py::Float(1.0), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::mul(&py::mul(&Py::Float(4.0), &v_d.clone())?, &v_c.clone())?)?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::sub(&Py::Float(1.0), &v_c.clone())?), (Py::str("CA"), py::mul(&v_CA0.clone(), &v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decline_exp(mut v_qi: Py, mut v_Di: Py, mut v_t: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__decl_check(v_qi.clone(), v_Di.clone(), v_t.clone())?;
    v_q = py::mul(&v_qi.clone(), &py::math1("exp", &py::mul(&py::neg(&v_Di.clone())?, &v_t.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("Np"), py::truediv(&py::mul(&v_qi.clone(), &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_Di.clone())?, &v_t.clone())?)?)?)?, &v_Di.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decline_harm(mut v_qi: Py, mut v_Di: Py, mut v_t: Py) -> R {
    let _ = f__decl_check(v_qi.clone(), v_Di.clone(), v_t.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("q"), py::truediv(&v_qi.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_Di.clone(), &v_t.clone())?)?)?), (Py::str("Np"), py::mul(&py::truediv(&v_qi.clone(), &v_Di.clone())?, &py::math1("log1p", &py::mul(&v_Di.clone(), &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decline_hyp(mut v_qi: Py, mut v_Di: Py, mut v_b: Py, mut v_t: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__decl_check(v_qi.clone(), v_Di.clone(), v_t.clone())?;
    let _ = f__fin(Py::tuple(vec![v_b.clone()]))?;
    let _ = f__need({ let t2449 = Py::Int(0i64); Py::Bool({ let t2450 = v_b.clone(); if py::lt(&t2449, &t2450)? { { let t2451 = Py::Int(1i64); if py::lt(&t2450, &t2451)? { true } else { false } } } else { false } }) }, Py::str("the hyperbolic exponent b must lie strictly between 0 and 1 (b = 0 exponential, b = 1 harmonic)"))?;
    v_q = py::mul(&v_qi.clone(), &py::pow(&py::add(&Py::Float(1.0), &py::mul(&py::mul(&v_b.clone(), &v_Di.clone())?, &v_t.clone())?)?, &py::truediv(&py::neg(&Py::Float(1.0))?, &v_b.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("Np"), py::mul(&py::truediv(&py::pow(&v_qi.clone(), &v_b.clone())?, &py::mul(&py::sub(&Py::Float(1.0), &v_b.clone())?, &v_Di.clone())?)?, &py::sub(&py::pow(&v_qi.clone(), &py::sub(&Py::Float(1.0), &v_b.clone())?)?, &py::pow(&v_q.clone(), &py::sub(&Py::Float(1.0), &v_b.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dew_P(mut v_y: Py, mut v_ant: Py, mut v_units: Py, mut v_base: Py, mut v_T: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let _ = f__comp(Py::str("y"), v_y.clone())?;
    let _ = f__antoine_rows(v_ant.clone(), py::len(&v_y.clone())?)?;
    let _ = f__pos(Py::str("T"), Py::tuple(vec![v_T.clone()]))?;
    v_ps = { let mut out = vec![]; for t4463 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4463;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    v_P = py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t4468 in py::iter(&(py::zip(&[v_y.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4469 = (t4468).items()?;
if t4469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4469[0].clone();
v_b = t4469[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("x"), { let mut out = vec![]; for t4482 in py::iter(&(py::zip(&[v_y.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4483 = (t4482).items()?;
if t4483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4483[0].clone();
v_b = t4483[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &v_b.clone())?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dew_T(mut v_y: Py, mut v_ant: Py, mut v_units: Py, mut v_base: Py, mut v_P: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let _ = f__comp(Py::str("y"), v_y.clone())?;
    let _ = f__antoine_rows(v_ant.clone(), py::len(&v_y.clone())?)?;
    let _ = f__pos(Py::str("P"), Py::tuple(vec![v_P.clone()]))?;
    let t4544 = (f__t_bracket(v_ant.clone(), v_units.clone(), v_base.clone(), v_P.clone())?).items()?;
    if t4544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4544[0].clone();
    v_hi = t4544[1].clone();
    v_f = { let v_P = v_P.clone(); let v_ant = v_ant.clone(); let v_base = v_base.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_units = v_units.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t4545 in py::iter(&(py::zip(&[v_y.clone(), v_ant.clone()])?))? { let mut v_a: Py; let mut v_r: Py; 
let t4546 = (t4545).items()?;
if t4546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4546[0].clone();
v_r = t4546[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?)?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_T = (if ({ let t4551 = v_hi.clone(); Py::Bool({ let t4552 = v_lo.clone(); if py::eq(&t4551, &t4552) { true } else { false } }) }).truthy() { v_lo.clone() } else { f__bisect(v_f.clone(), v_lo.clone(), v_hi.clone(), Py::Int(500i64))? });
    v_ps = { let mut out = vec![]; for t4579 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4579;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("x"), { let mut out = vec![]; for t4592 in py::iter(&(py::zip(&[v_y.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4593 = (t4592).items()?;
if t4593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4593[0].clone();
v_b = t4593[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &v_b.clone())?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equimolar_flux(mut v_D: Py, mut v_P: Py, mut v_T: Py, mut v_L: Py, mut v_yA1: Py, mut v_yA2: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::str("D, P, T and L"), Py::tuple(vec![v_D.clone(), v_P.clone(), v_T.clone(), v_L.clone()]))?;
    let _ = f__frac(Py::str("yA1"), v_yA1.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__frac(Py::str("yA2"), v_yA2.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t1622 = v_yA1.clone(); Py::Bool({ let t1623 = v_yA2.clone(); if py::gt(&t1622, &t1623)? { true } else { false } }) }, Py::str("A diffuses from yA1 to yA2: yA1 must exceed yA2"))?;
    v_c = py::truediv(&v_P.clone(), &py::mul(&c_R_GAS()?, &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("NA"), py::truediv(&py::mul(&py::mul(&v_c.clone(), &v_D.clone())?, &py::sub(&v_yA1.clone(), &v_yA2.clone())?)?, &v_L.clone())?), (Py::str("NB"), py::truediv(&py::mul(&py::mul(&py::neg(&v_c.clone())?, &v_D.clone())?, &py::sub(&v_yA1.clone(), &v_yA2.clone())?)?, &v_L.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ergun(mut v_dp: Py, mut v_eps: Py, mut v_vs: Py, mut v_rho: Py, mut v_mu: Py, mut v_L: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let _ = f__bed_check(v_dp.clone(), v_eps.clone(), v_vs.clone(), v_rho.clone(), v_mu.clone(), v_L.clone())?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_dp.clone(), &v_rho.clone())?, &v_vs.clone())?, &v_mu.clone())?;
    v_fp = py::truediv(&py::mul(&py::add(&Py::Float(150.0), &py::truediv(&py::mul(&Py::Float(1.75), &v_Re.clone())?, &py::sub(&Py::Float(1.0), &v_eps.clone())?)?)?, &py::pow(&py::sub(&Py::Float(1.0), &v_eps.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::pow(&v_eps.clone(), &Py::Int(3i64))?, &v_Re.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dP"), py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&v_fp.clone(), &v_rho.clone())?, &v_vs.clone())?, &v_vs.clone())?, &v_L.clone())?, &v_dp.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fenske(mut v_alpha: Py, mut v_xD: Py, mut v_xB: Py) -> R {
    let mut v_N: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_alpha.clone(), v_xD.clone(), v_xB.clone()]))?;
    let _ = f__need({ let t1771 = v_alpha.clone(); Py::Bool({ let t1772 = Py::Float(1.0); if py::gt(&t1771, &t1772)? { true } else { false } }) }, Py::str("the relative volatility alpha must exceed 1"))?;
    let _ = f__need({ let t1773 = Py::Int(0i64); Py::Bool({ let t1774 = v_xB.clone(); if py::lt(&t1773, &t1774)? { { let t1775 = v_xD.clone(); if py::lt(&t1774, &t1775)? { { let t1776 = Py::Int(1i64); if py::lt(&t1775, &t1776)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("the compositions must satisfy 0 < xB < xD < 1"))?;
    v_N = py::truediv(&py::math1("log", &py::mul(&py::truediv(&v_xD.clone(), &py::sub(&Py::Int(1i64), &v_xD.clone())?)?, &py::truediv(&py::sub(&Py::Int(1i64), &v_xB.clone())?, &v_xB.clone())?)?)?, &py::math1("log", &v_alpha.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Nmin"), v_N.clone()), (Py::str("Nmin_trays"), py::sub(&v_N.clone(), &Py::Float(1.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filtration_fit(mut v_t: Py, mut v_V: Py, mut v_t_unit: Py, mut v_v_unit: Py, mut v_A: Py, mut v_dP: Py, mut v_mu: Py, mut v_c: Py) -> R {
    let mut v_icpt: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let mut v_yv: Py = Py::None;
    let _ = f__need({ let t1815 = py::len(&v_t.clone())?; Py::Bool({ let t1816 = py::len(&v_V.clone())?; if py::eq(&t1815, &t1816) { { let t1817 = Py::Int(3i64); if py::ge(&t1816, &t1817)? { true } else { false } } } else { false } }) }, Py::str("t and V need the same number of points (at least 3)"))?;
    let _ = f__pos(Py::str("A, dP, mu and c"), Py::tuple(vec![v_A.clone(), v_dP.clone(), v_mu.clone(), v_c.clone()]))?;
    v_ts = { let mut out = vec![]; for t1824 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t1824;
out.push(py::mul(&v_x.clone(), &py::getitem(&Py::dict(vec![(Py::str("s"), Py::Float(1.0)), (Py::str("min"), Py::Float(60.0)), (Py::str("h"), Py::Float(3600.0))]), &v_t_unit.clone())?)?); } Py::list(out) };
    v_vs = { let mut out = vec![]; for t1825 in py::iter(&(v_V.clone()))? { let mut v_x: Py; 
v_x = t1825;
out.push(py::mul(&v_x.clone(), &py::getitem(&Py::dict(vec![(Py::str("m3"), Py::Float(1.0)), (Py::str("L"), Py::Float(0.001))]), &v_v_unit.clone())?)?); } Py::list(out) };
    let _ = f__need({ let t1832 = Py::Bool(({ let mut out = vec![]; for t1826 in py::iter(&(v_vs.clone()))? { let mut v_v: Py; 
v_v = t1826;
out.push({ let t1827 = v_v.clone(); Py::Bool({ let t1828 = Py::Int(0i64); if py::gt(&t1827, &t1828)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1832.truthy() { t1832 } else { Py::Bool(({ let mut out = vec![]; for t1829 in py::iter(&(v_ts.clone()))? { let mut v_x: Py; 
v_x = t1829;
out.push({ let t1830 = v_x.clone(); Py::Bool({ let t1831 = Py::Int(0i64); if py::gt(&t1830, &t1831)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("times and volumes must be positive"))?;
    v_yv = { let mut out = vec![]; for t1833 in py::iter(&(py::zip(&[v_ts.clone(), v_vs.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1834 = (t1833).items()?;
if t1834.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1834[0].clone();
v_b = t1834[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    v_n = py::len(&v_vs.clone())?;
    let t1835 = (Py::tuple(vec![py::truediv(&py::sum(&v_vs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_yv.clone(), None)?, &v_n.clone())?])).items()?;
    if t1835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t1835[0].clone();
    v_my = t1835[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t1836 in py::iter(&(v_vs.clone()))? { let mut v_x: Py; 
v_x = t1836;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t1837 = v_sxx.clone(); Py::Bool({ let t1838 = Py::Int(0i64); if py::gt(&t1837, &t1838)? { true } else { false } }) }, Py::str("the volumes must not all be equal"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t1839 in py::iter(&(py::zip(&[v_vs.clone(), v_yv.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1840 = (t1839).items()?;
if t1840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1840[0].clone();
v_y = t1840[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_icpt = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t1845 = { let t1841 = v_slope.clone(); Py::Bool({ let t1842 = Py::Int(0i64); if py::gt(&t1841, &t1842)? { true } else { false } }) }; if !t1845.truthy() { t1845 } else { { let t1843 = v_icpt.clone(); Py::Bool({ let t1844 = Py::Int(0i64); if py::gt(&t1843, &t1844)? { true } else { false } }) } } }, Py::str("the data do not give a positive slope and intercept of t/V against V"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("alpha"), py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_slope.clone())?, &v_A.clone())?, &v_A.clone())?, &v_dP.clone())?, &py::mul(&v_mu.clone(), &v_c.clone())?)?), (Py::str("Rm"), py::truediv(&py::mul(&py::mul(&v_icpt.clone(), &v_A.clone())?, &v_dP.clone())?, &v_mu.clone())?), (Py::str("slope"), v_slope.clone()), (Py::str("intercept"), v_icpt.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_filtration_volume(mut v_A: Py, mut v_dP: Py, mut v_mu: Py, mut v_alpha: Py, mut v_c: Py, mut v_Rm: Py, mut v_t: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let _ = f__pos(Py::str("A, dP, mu, alpha, c and Rm"), Py::tuple(vec![v_A.clone(), v_dP.clone(), v_mu.clone(), v_alpha.clone(), v_c.clone(), v_Rm.clone()]))?;
    let _ = f__nonneg(Py::str("t"), Py::tuple(vec![v_t.clone()]))?;
    v_a = py::truediv(&py::mul(&py::mul(&v_mu.clone(), &v_alpha.clone())?, &v_c.clone())?, &py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_A.clone())?, &v_A.clone())?, &v_dP.clone())?)?;
    v_b = py::truediv(&py::mul(&v_mu.clone(), &v_Rm.clone())?, &py::mul(&v_A.clone(), &v_dP.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::truediv(&py::mul(&Py::Float(2.0), &v_t.clone())?, &py::add(&v_b.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_a.clone())?, &v_t.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fit_first_order(mut v_t: Py, mut v_y: Py, mut v_t_unit: Py, mut v_M: Py, mut v_theta: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_dsse: Py = Py::None;
    let mut v_grid: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_kfit: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_span: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_vals: Py = Py::None;
    let _ = f__need({ let t1884 = py::len(&v_t.clone())?; Py::Bool({ let t1885 = py::len(&v_y.clone())?; if py::eq(&t1884, &t1885) { { let t1886 = Py::Int(4i64); if py::ge(&t1885, &t1886)? { true } else { false } } } else { false } }) }, Py::str("t and y need the same number of points (at least 4)"))?;
    let _ = f__fin({ let mut v = vec![v_M.clone()]; v.extend((v_y.clone()).items()?); Py::tuple(v) })?;
    let _ = f__need({ let t1890 = v_M.clone(); Py::Bool({ let t1891 = Py::Int(0i64); if py::ne(&t1890, &t1891) { true } else { false } }) }, Py::str("the step size M must not be zero"))?;
    let _ = f__nonneg(Py::str("theta"), Py::tuple(vec![v_theta.clone()]))?;
    v_ts = { let mut out = vec![]; for t1898 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t1898;
out.push(py::mul(&v_x.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1899 in py::iter(&(py::zip(&[v_ts.clone(), py::slice(&v_ts.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1900 = (t1899).items()?;
if t1900.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1900[0].clone();
v_b = t1900[1].clone();
out.push({ let t1901 = v_b.clone(); Py::Bool({ let t1902 = v_a.clone(); if py::gt(&t1901, &t1902)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("times must increase"))?;
    v_pts = { let mut out = vec![]; for t1903 in py::iter(&(py::zip(&[v_ts.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1904 = (t1903).items()?;
if t1904.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1904[0].clone();
v_b = t1904[1].clone();
if ({ let t1905 = v_a.clone(); Py::Bool({ let t1906 = v_theta.clone(); if py::gt(&t1905, &t1906)? { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![py::sub(&v_a.clone(), &v_theta.clone())?, v_b.clone()])); }} Py::list(out) };
    let _ = f__need({ let t1907 = py::len(&v_pts.clone())?; Py::Bool({ let t1908 = Py::Int(3i64); if py::ge(&t1907, &t1908)? { true } else { false } }) }, Py::str("need at least three points after the dead time"))?;
    v_kfit = { let v_M = v_M.clone(); let v_kfit = v_kfit.clone(); let v_pts = v_pts.clone(); let v_t = v_t.clone(); let v_t_unit = v_t_unit.clone(); let v_theta = v_theta.clone(); let v_ts = v_ts.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_tau: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_f: Py = Py::None;
            let mut v_sff: Py = Py::None;
            v_f = { let mut out = vec![]; for t1909 in py::iter(&(v_pts.clone()))? { let mut v__: Py; let mut v_s: Py; 
let t1910 = (t1909).items()?;
if t1910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t1910[0].clone();
v__ = t1910[1].clone();
out.push(py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_s.clone())?, &v_tau.clone())?)?)?); } Py::list(out) };
            v_sff = py::sum(&{ let mut out = vec![]; for t1911 in py::iter(&(v_f.clone()))? { let mut v_v: Py; 
v_v = t1911;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) }, None)?;
            return Ok(Py::tuple(vec![py::truediv(&py::sum(&{ let mut out = vec![]; for t1912 in py::iter(&(py::zip(&[v_f.clone(), v_pts.clone()])?))? { let mut v__: Py; let mut v_v: Py; let mut v_yy: Py; 
let t1913 = (t1912).items()?;
if t1913.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1913[0].clone();
let t1914 = (t1913[1].clone()).items()?;
if t1914.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t1914[0].clone();
v_yy = t1914[1].clone();
out.push(py::mul(&v_v.clone(), &v_yy.clone())?); } Py::list(out) }, None)?, &py::mul(&v_M.clone(), &v_sff.clone())?)?, v_f.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_sse = { let v_M = v_M.clone(); let v_kfit = v_kfit.clone(); let v_pts = v_pts.clone(); let v_sse = v_sse.clone(); let v_t = v_t.clone(); let v_t_unit = v_t_unit.clone(); let v_theta = v_theta.clone(); let v_ts = v_ts.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_tau: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_K: Py = Py::None;
            let mut v_f: Py = Py::None;
            let t1915 = (py::call(&v_kfit.clone(), &[v_tau.clone()])?).items()?;
            if t1915.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_K = t1915[0].clone();
            v_f = t1915[1].clone();
            return Ok(py::sum(&{ let mut out = vec![]; for t1916 in py::iter(&(py::zip(&[v_f.clone(), v_pts.clone()])?))? { let mut v__: Py; let mut v_v: Py; let mut v_yy: Py; 
let t1917 = (t1916).items()?;
if t1917.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1917[0].clone();
let t1918 = (t1917[1].clone()).items()?;
if t1918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t1918[0].clone();
v_yy = t1918[1].clone();
out.push(py::pow(&py::sub(&v_yy.clone(), &py::mul(&py::mul(&v_K.clone(), &v_M.clone())?, &v_v.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_dsse = { let v_M = v_M.clone(); let v_dsse = v_dsse.clone(); let v_kfit = v_kfit.clone(); let v_pts = v_pts.clone(); let v_sse = v_sse.clone(); let v_t = v_t.clone(); let v_t_unit = v_t_unit.clone(); let v_theta = v_theta.clone(); let v_ts = v_ts.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_tau: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_K: Py = Py::None;
            let mut v_f: Py = Py::None;
            let t1919 = (py::call(&v_kfit.clone(), &[v_tau.clone()])?).items()?;
            if t1919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_K = t1919[0].clone();
            v_f = t1919[1].clone();
            return Ok(py::sum(&{ let mut out = vec![]; for t1920 in py::iter(&(py::zip(&[v_f.clone(), v_pts.clone()])?))? { let mut v_s: Py; let mut v_v: Py; let mut v_yy: Py; 
let t1921 = (t1920).items()?;
if t1921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1921[0].clone();
let t1922 = (t1921[1].clone()).items()?;
if t1922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t1922[0].clone();
v_yy = t1922[1].clone();
out.push(py::mul(&py::mul(&py::sub(&v_yy.clone(), &py::mul(&py::mul(&v_K.clone(), &v_M.clone())?, &v_v.clone())?)?, &py::truediv(&v_s.clone(), &py::pow(&v_tau.clone(), &Py::Int(2i64))?)?)?, &py::math1("exp", &py::truediv(&py::neg(&v_s.clone())?, &v_tau.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_span = py::minmax(&[{ let mut out = vec![]; for t1923 in py::iter(&(v_pts.clone()))? { let mut v__: Py; let mut v_s: Py; 
let t1924 = (t1923).items()?;
if t1924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t1924[0].clone();
v__ = t1924[1].clone();
out.push(v_s.clone()); } Py::list(out) }], true)?;
    v_grid = { let mut out = vec![]; for t1925 in py::range_iter(&[(Py::Int(401i64)).i()?]) { let mut v_i: Py; 
v_i = t1925;
out.push(py::mul(&py::mul(&v_span.clone(), &Py::Float(0.001))?, &py::pow(&Py::Float(100000.0), &py::truediv(&v_i.clone(), &Py::Float(400.0))?)?)?); } Py::list(out) };
    v_vals = { let mut out = vec![]; for t1926 in py::iter(&(v_grid.clone()))? { let mut v_g: Py; 
v_g = t1926;
out.push(py::call(&v_sse.clone(), &[v_g.clone()])?); } Py::list(out) };
    v_i = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_grid.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_M = v_M.clone(); let v_dsse = v_dsse.clone(); let v_grid = v_grid.clone(); let v_kfit = v_kfit.clone(); let v_pts = v_pts.clone(); let v_span = v_span.clone(); let v_sse = v_sse.clone(); let v_t = v_t.clone(); let v_t_unit = v_t_unit.clone(); let v_theta = v_theta.clone(); let v_ts = v_ts.clone(); let v_vals = v_vals.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_vals.clone(), &v_j.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
    let _ = f__need({ let t1927 = Py::Int(0i64); Py::Bool({ let t1928 = v_i.clone(); if py::lt(&t1927, &t1928)? { { let t1929 = py::sub(&py::len(&v_grid.clone())?, &Py::Int(1i64))?; if py::lt(&t1928, &t1929)? { true } else { false } } } else { false } }) }, Py::str("the data do not determine a time constant (no interior minimum)"))?;
    v_tau = (if ({ let t1930 = py::mul(&py::call(&v_dsse.clone(), &[py::getitem(&v_grid.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?])?, &py::call(&v_dsse.clone(), &[py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])?)?; Py::Bool({ let t1931 = Py::Int(0i64); if py::lt(&t1930, &t1931)? { true } else { false } }) }).truthy() { f__bisect(v_dsse.clone(), py::getitem(&v_grid.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, Py::Int(500i64))? } else { py::getitem(&v_grid.clone(), &v_i.clone())? });
    let t1958 = (py::call(&v_kfit.clone(), &[v_tau.clone()])?).items()?;
    if t1958.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_K = t1958[0].clone();
    v__ = t1958[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("tau"), v_tau.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_flame_temperature(mut v_fuel: Py, mut v_excess_air: Py, mut v_T_in: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_prod: Py = Py::None;
    let mut v_reac: Py = Py::None;
    let _ = f__need({ let t58 = v_fuel.clone(); Py::Bool({ let t59 = c__FLAME_FUELS()?; if py::contains(&t59, &t58)? { true } else { false } }) }, py::add(&Py::str("fuel must be one of "), &py::method(&Py::str(", "), "join", &[c__FLAME_FUELS()?])?)?)?;
    let _ = f__fin(Py::tuple(vec![v_excess_air.clone()]))?;
    let _ = f__need({ let t63 = Py::Int(0i64); Py::Bool({ let t64 = v_excess_air.clone(); if py::le(&t63, &t64)? { { let t65 = Py::Int(4i64); if py::le(&t64, &t65)? { true } else { false } } } else { false } }) }, Py::str("the excess air must be between 0 and 400 %"))?;
    let _ = f__need({ let t66 = Py::Int(300i64); Py::Bool({ let t67 = v_T_in.clone(); if py::le(&t66, &t67)? { { let t68 = Py::Int(1000i64); if py::le(&t67, &t68)? { true } else { false } } } else { false } }) }, Py::str("the inlet temperature must be between 300 and 1000 K (range of the data)"))?;
    let t70 = (f__flame_mix(v_fuel.clone(), v_excess_air.clone())?).items()?;
    if t70.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_reac = t70[0].clone();
    v_prod = t70[1].clone();
    v_H = py::sum(&{ let mut out = vec![]; for t71 in py::iter(&(py::method(&v_reac.clone(), "items", &[])?))? { let mut v_n: Py; let mut v_s: Py; 
let t72 = (t71).items()?;
if t72.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t72[0].clone();
v_n = t72[1].clone();
out.push(py::mul(&v_n.clone(), &f__h_nasa(v_s.clone(), v_T_in.clone())?)?); } Py::list(out) }, None)?;
    v_f = { let v_H = v_H.clone(); let v_T_in = v_T_in.clone(); let v_excess_air = v_excess_air.clone(); let v_fuel = v_fuel.clone(); let v_prod = v_prod.clone(); let v_reac = v_reac.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sum(&{ let mut out = vec![]; for t75 in py::iter(&(py::method(&v_prod.clone(), "items", &[])?))? { let mut v_n: Py; let mut v_s: Py; 
let t76 = (t75).items()?;
if t76.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t76[0].clone();
v_n = t76[1].clone();
if (v_n.clone()).truthy() { out.push(py::mul(&v_n.clone(), &f__h_nasa(v_s.clone(), v_T.clone())?)?); }} Py::list(out) }, None)?, &v_H.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let _ = f__need({ let t77 = py::call(&v_f.clone(), &[Py::Float(3500.0)])?; Py::Bool({ let t78 = Py::Int(0i64); if py::gt(&t77, &t78)? { true } else { false } }) }, Py::str("the flame temperature would exceed 3500 K, the range of the data"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("T_ad"), f__bisect(v_f.clone(), v_T_in.clone(), Py::Float(3500.0), Py::Int(500i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fopdt_step(mut v_K: Py, mut v_tau: Py, mut v_theta: Py, mut v_M: Py, mut v_t: Py) -> R {
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_K.clone(), v_M.clone()]))?;
    let _ = f__pos(Py::str("tau"), Py::tuple(vec![v_tau.clone()]))?;
    let _ = f__nonneg(Py::str("theta and t"), Py::tuple(vec![v_theta.clone(), v_t.clone()]))?;
    v_y = (if ({ let t2012 = v_t.clone(); Py::Bool({ let t2013 = v_theta.clone(); if py::le(&t2012, &t2013)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::mul(&v_K.clone(), &v_M.clone())?, &py::neg(&py::math1("expm1", &py::truediv(&py::neg(&py::sub(&v_t.clone(), &v_theta.clone())?)?, &v_tau.clone())?)?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("y"), v_y.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gas_pz(mut v_p: Py, mut v_z: Py, mut v_Gp: Py, mut v_p_unit: Py, mut v_gp_unit: Py) -> R {
    let mut v_icpt: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__ho_check(Py::tuple(vec![v_p.clone(), v_z.clone(), v_Gp.clone()]))?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("pressures and z")); v.extend((v_p.clone()).items()?); v.extend((v_z.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_pos() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_pos() got an unexpected keyword argument".into())) } f__pos(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    v_xs = { let mut out = vec![]; for t2068 in py::iter(&(v_Gp.clone()))? { let mut v_g: Py; 
v_g = t2068;
out.push(py::mul(&v_g.clone(), &py::getitem(&c__GPU()?, &v_gp_unit.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2069 in py::iter(&(py::zip(&[v_p.clone(), v_z.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2070 = (t2069).items()?;
if t2070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2070[0].clone();
v_b = t2070[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?, &v_b.clone())?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let t2071 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2071[0].clone();
    v_my = t2071[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2072 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2072;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2073 = v_sxx.clone(); Py::Bool({ let t2074 = Py::Int(0i64); if py::gt(&t2073, &t2074)? { true } else { false } }) }, Py::str("the cumulative productions must differ"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t2075 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2076 = (t2075).items()?;
if t2076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2076[0].clone();
v_y = t2076[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_icpt = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t2081 = { let t2077 = v_slope.clone(); Py::Bool({ let t2078 = Py::Int(0i64); if py::lt(&t2077, &t2078)? { true } else { false } }) }; if !t2081.truthy() { t2081 } else { { let t2079 = v_icpt.clone(); Py::Bool({ let t2080 = Py::Int(0i64); if py::gt(&t2079, &t2080)? { true } else { false } }) } } }, Py::str("p/z must fall as gas is produced"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("G"), py::truediv(&py::neg(&v_icpt.clone())?, &v_slope.clone())?), (Py::str("pz_initial"), v_icpt.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ho_gascap(mut v_F: Py, mut v_Eo: Py, mut v_Eg: Py, mut v_v_unit: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__ho_check(Py::tuple(vec![v_F.clone(), v_Eo.clone(), v_Eg.clone()]))?;
    let _ = f__need({ let t2258 = Py::Bool(({ let mut out = vec![]; for t2253 in py::iter(&(v_Eo.clone()))? { let mut v_e: Py; 
v_e = t2253;
out.push({ let t2254 = v_e.clone(); Py::Bool({ let t2255 = Py::Int(0i64); if py::gt(&t2254, &t2255)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t2258.truthy() { t2258 } else { { let t2256 = py::len(&v_F.clone())?; Py::Bool({ let t2257 = Py::Int(3i64); if py::ge(&t2256, &t2257)? { true } else { false } }) } } }, Py::str("Eo must be positive and at least three surveys are needed"))?;
    v_xs = { let mut out = vec![]; for t2259 in py::iter(&(py::zip(&[v_Eg.clone(), v_Eo.clone()])?))? { let mut v_g: Py; let mut v_o: Py; 
let t2260 = (t2259).items()?;
if t2260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t2260[0].clone();
v_o = t2260[1].clone();
out.push(py::truediv(&v_g.clone(), &v_o.clone())?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2261 in py::iter(&(py::zip(&[v_F.clone(), v_Eo.clone()])?))? { let mut v_f: Py; let mut v_o: Py; 
let t2262 = (t2261).items()?;
if t2262.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t2262[0].clone();
v_o = t2262[1].clone();
out.push(py::truediv(&v_f.clone(), &v_o.clone())?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let t2263 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2263[0].clone();
    v_my = t2263[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2264 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2264;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2265 = v_sxx.clone(); Py::Bool({ let t2266 = Py::Int(0i64); if py::gt(&t2265, &t2266)? { true } else { false } }) }, Py::str("Eg/Eo must vary between surveys"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t2267 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2268 = (t2267).items()?;
if t2268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2268[0].clone();
v_y = t2268[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_N = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t2269 = v_N.clone(); Py::Bool({ let t2270 = Py::Int(0i64); if py::gt(&t2269, &t2270)? { true } else { false } }) }, Py::str("the fitted oil in place is not positive"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), py::mul(&v_N.clone(), &py::getitem(&c__VU()?, &v_v_unit.clone())?)?), (Py::str("m"), py::truediv(&v_slope.clone(), &v_N.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ho_oil(mut v_F: Py, mut v_Et: Py, mut v_v_unit: Py) -> R {
    let mut v_N: Py = Py::None;
    let _ = f__ho_check(Py::tuple(vec![v_F.clone(), v_Et.clone()]))?;
    let _ = f__need({ let t2325 = Py::Bool(({ let mut out = vec![]; for t2319 in py::iter(&(v_Et.clone()))? { let mut v_e: Py; 
v_e = t2319;
out.push({ let t2320 = v_e.clone(); Py::Bool({ let t2321 = Py::Int(0i64); if py::gt(&t2320, &t2321)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t2325.truthy() { t2325 } else { Py::Bool(({ let mut out = vec![]; for t2322 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t2322;
out.push({ let t2323 = v_f.clone(); Py::Bool({ let t2324 = Py::Int(0i64); if py::gt(&t2323, &t2324)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("F and Et must be positive"))?;
    v_N = py::truediv(&py::sum(&{ let mut out = vec![]; for t2326 in py::iter(&(py::zip(&[v_F.clone(), v_Et.clone()])?))? { let mut v_e: Py; let mut v_f: Py; 
let t2327 = (t2326).items()?;
if t2327.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t2327[0].clone();
v_e = t2327[1].clone();
out.push(py::mul(&v_f.clone(), &v_e.clone())?); } Py::list(out) }, None)?, &py::sum(&{ let mut out = vec![]; for t2328 in py::iter(&(v_Et.clone()))? { let mut v_e: Py; 
v_e = t2328;
out.push(py::mul(&v_e.clone(), &v_e.clone())?); } Py::list(out) }, None)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), py::mul(&v_N.clone(), &py::getitem(&c__VU()?, &v_v_unit.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_horner(mut v_dt: Py, mut v_pws: Py, mut v_t_unit: Py, mut v_p_unit: Py, mut v_tp: Py, mut v_q: Py, mut v_mu: Py, mut v_h: Py) -> R {
    let mut v_ds: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__ho_check(Py::tuple(vec![v_dt.clone(), v_pws.clone()]))?;
    let _ = f__pos(Py::str("tp, q, mu and h"), Py::tuple(vec![v_tp.clone(), v_q.clone(), v_mu.clone(), v_h.clone()]))?;
    v_ds = { let mut out = vec![]; for t2383 in py::iter(&(v_dt.clone()))? { let mut v_x: Py; 
v_x = t2383;
out.push(py::mul(&v_x.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    let _ = f__pos(Py::str("the shut-in times"), { let mut v = vec![]; v.extend((v_ds.clone()).items()?); Py::tuple(v) })?;
    v_xs = { let mut out = vec![]; for t2384 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t2384;
out.push(py::math1("log10", &py::truediv(&py::add(&v_tp.clone(), &v_d.clone())?, &v_d.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2385 in py::iter(&(v_pws.clone()))? { let mut v_v: Py; 
v_v = t2385;
out.push(py::mul(&v_v.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let _ = f__need({ let t2386 = v_n.clone(); Py::Bool({ let t2387 = Py::Int(3i64); if py::ge(&t2386, &t2387)? { true } else { false } }) }, Py::str("at least three build-up points are needed"))?;
    let t2388 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2388[0].clone();
    v_my = t2388[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2389 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2389;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2390 = v_sxx.clone(); Py::Bool({ let t2391 = Py::Int(0i64); if py::gt(&t2390, &t2391)? { true } else { false } }) }, Py::str("the shut-in times must differ"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t2392 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2393 = (t2392).items()?;
if t2393.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2393[0].clone();
v_y = t2393[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_m = py::neg(&v_slope.clone())?;
    let _ = f__need({ let t2394 = v_m.clone(); Py::Bool({ let t2395 = Py::Int(0i64); if py::gt(&t2394, &t2395)? { true } else { false } }) }, Py::str("the build-up pressure must rise with shut-in time"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("k"), py::truediv(&py::mul(&py::mul(&v_q.clone(), &v_mu.clone())?, &py::math1("log", &Py::Float(10.0))?)?, &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_m.clone())?, &v_h.clone())?)?), (Py::str("p_star"), py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kremser_absorber(mut v_N: Py, mut v_A: Py, mut v_m: Py, mut v_y_in: Py, mut v_x_in: Py) -> R {
    let mut v_y_out: Py = Py::None;
    let _ = f__stages_check(v_N.clone(), Py::tuple(vec![v_A.clone(), v_m.clone()]))?;
    let _ = f__nonneg(Py::str("the compositions"), Py::tuple(vec![v_y_in.clone(), v_x_in.clone()]))?;
    let _ = f__need({ let t2506 = v_y_in.clone(); Py::Bool({ let t2507 = py::mul(&v_m.clone(), &v_x_in.clone())?; if py::gt(&t2506, &t2507)? { true } else { false } }) }, Py::str("no absorption: the entering gas is not richer than equilibrium with the entering liquid"))?;
    v_y_out = py::sub(&v_y_in.clone(), &py::mul(&f__kremser_frac(v_A.clone(), v_N.clone())?, &py::sub(&v_y_in.clone(), &py::mul(&v_m.clone(), &v_x_in.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("y_out"), v_y_out.clone()), (Py::str("recovery"), py::truediv(&py::sub(&v_y_in.clone(), &v_y_out.clone())?, &v_y_in.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kremser_stripper(mut v_N: Py, mut v_S: Py, mut v_m: Py, mut v_x_in: Py, mut v_y_in: Py) -> R {
    let mut v_x_out: Py = Py::None;
    let _ = f__stages_check(v_N.clone(), Py::tuple(vec![v_S.clone(), v_m.clone()]))?;
    let _ = f__nonneg(Py::str("the compositions"), Py::tuple(vec![v_y_in.clone(), v_x_in.clone()]))?;
    let _ = f__need({ let t2564 = v_x_in.clone(); Py::Bool({ let t2565 = py::truediv(&v_y_in.clone(), &v_m.clone())?; if py::gt(&t2564, &t2565)? { true } else { false } }) }, Py::str("no stripping: the entering liquid is not richer than equilibrium with the entering gas"))?;
    v_x_out = py::sub(&v_x_in.clone(), &py::mul(&f__kremser_frac(v_S.clone(), v_N.clone())?, &py::sub(&v_x_in.clone(), &py::truediv(&v_y_in.clone(), &v_m.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("x_out"), v_x_out.clone()), (Py::str("recovery"), py::truediv(&py::sub(&v_x_in.clone(), &v_x_out.clone())?, &v_x_in.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lag_freq(mut v_K: Py, mut v_taus: Py, mut v_t_unit: Py, mut v_w: Py) -> R {
    let mut v_ar: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_K.clone()]))?;
    let _ = f__need({ let t2609 = v_K.clone(); Py::Bool({ let t2610 = Py::Int(0i64); if py::gt(&t2609, &t2610)? { true } else { false } }) }, Py::str("the process gain must be positive"))?;
    let _ = f__need({ let t2611 = Py::Int(1i64); Py::Bool({ let t2612 = py::len(&v_taus.clone())?; if py::le(&t2611, &t2612)? { { let t2613 = Py::Int(3i64); if py::le(&t2612, &t2613)? { true } else { false } } } else { false } }) }, Py::str("give 1 to 3 time constants"))?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("the time constants and the frequency")); v.extend((v_taus.clone()).items()?); v.push(v_w.clone()); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_pos() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_pos() got an unexpected keyword argument".into())) } f__pos(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    v_ts = { let mut out = vec![]; for t2620 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t2620;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    v_ar = v_K.clone();
    for t2621 in py::iter(&(v_ts.clone()))? {
        v_t = t2621;
        v_ar = py::truediv(&v_ar, &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::pow(&py::mul(&v_w.clone(), &v_t.clone())?, &Py::Int(2i64))?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("AR"), v_ar.clone()), (Py::str("phase_deg"), py::neg(&py::sum(&{ let mut out = vec![]; for t2634 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t2634;
out.push(py::math1("degrees", &py::math1("atan", &py::mul(&v_w.clone(), &v_t.clone())?)?)?); } Py::list(out) }, None)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lee_kesler(mut v_T: Py, mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py) -> R {
    let mut v_Tr: Py = Py::None;
    let mut v_f0: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let _ = f__pos(Py::str("T, Tc and Pc"), Py::tuple(vec![v_T.clone(), v_Tc.clone(), v_Pc.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_omega.clone()]))?;
    let _ = f__need({ let t2670 = v_T.clone(); Py::Bool({ let t2671 = v_Tc.clone(); if py::lt(&t2670, &t2671)? { true } else { false } }) }, Py::str("T must be below the critical temperature"))?;
    v_Tr = py::truediv(&v_T.clone(), &v_Tc.clone())?;
    v_f0 = py::add(&py::sub(&py::sub(&Py::Float(5.92714), &py::truediv(&Py::Float(6.09648), &v_Tr.clone())?)?, &py::mul(&Py::Float(1.28862), &py::math1("log", &v_Tr.clone())?)?)?, &py::mul(&Py::Float(0.169347), &py::pow(&v_Tr.clone(), &Py::Int(6i64))?)?)?;
    v_f1 = py::add(&py::sub(&py::sub(&Py::Float(15.2518), &py::truediv(&Py::Float(15.6875), &v_Tr.clone())?)?, &py::mul(&Py::Float(13.4721), &py::math1("log", &v_Tr.clone())?)?)?, &py::mul(&Py::Float(0.43577), &py::pow(&v_Tr.clone(), &Py::Int(6i64))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Psat"), py::mul(&v_Pc.clone(), &py::math1("exp", &py::add(&v_f0.clone(), &py::mul(&v_omega.clone(), &v_f1.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_line_source(mut v_q: Py, mut v_mu: Py, mut v_k: Py, mut v_h: Py, mut v_phi: Py, mut v_ct: Py, mut v_r: Py, mut v_t: Py, mut v_pi: Py) -> R {
    let mut v_dp: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("q, mu, k, h, ct, r, t and pi"), Py::tuple(vec![v_q.clone(), v_mu.clone(), v_k.clone(), v_h.clone(), v_ct.clone(), v_r.clone(), v_t.clone(), v_pi.clone()]))?;
    let _ = f__frac(Py::str("the porosity phi"), v_phi.clone(), Py::Bool(true), Py::Bool(true))?;
    v_x = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&v_phi.clone(), &v_mu.clone())?, &v_ct.clone())?, &v_r.clone())?, &v_r.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_k.clone())?, &v_t.clone())?)?;
    let _ = f__need({ let t2728 = v_x.clone(); Py::Bool({ let t2729 = Py::Float(50.0); if py::le(&t2728, &t2729)? { true } else { false } }) }, Py::str("the pressure disturbance has not reached this radius yet (Ei argument above 50)"))?;
    v_dp = py::mul(&py::truediv(&py::mul(&v_q.clone(), &v_mu.clone())?, &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_h.clone())?)?, &f__e1(v_x.clone())?)?;
    let _ = f__need({ let t2742 = v_dp.clone(); Py::Bool({ let t2743 = v_pi.clone(); if py::lt(&t2742, &t2743)? { true } else { false } }) }, Py::str("the drawdown exceeds the initial pressure"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("dp"), v_dp.clone()), (Py::str("p"), py::sub(&v_pi.clone(), &v_dp.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_margins_pi(mut v_Kc: Py, mut v_tauI: Py, mut v_K: Py, mut v_taus: Py, mut v_t_unit: Py) -> R {
    let mut v_cross: Py = Py::None;
    let mut v_grid: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mag: Py = Py::None;
    let mut v_ph: Py = Py::None;
    let mut v_ti: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_wgc: Py = Py::None;
    let mut v_wpc: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_Kc.clone(), v_K.clone()]))?;
    let _ = f__need({ let t3779 = py::mul(&v_Kc.clone(), &v_K.clone())?; Py::Bool({ let t3780 = Py::Int(0i64); if py::gt(&t3779, &t3780)? { true } else { false } }) }, Py::str("the loop gain must be positive"))?;
    let _ = f__need({ let t3781 = Py::Int(3i64); Py::Bool({ let t3782 = py::len(&v_taus.clone())?; if py::le(&t3781, &t3782)? { { let t3783 = Py::Int(5i64); if py::le(&t3782, &t3783)? { true } else { false } } } else { false } }) }, Py::str("give 3 to 5 time constants"))?;
    let _ = f__pos(Py::str("tauI and the time constants"), { let mut v = vec![v_tauI.clone()]; v.extend((v_taus.clone()).items()?); Py::tuple(v) })?;
    v_ts = { let mut out = vec![]; for t3790 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t3790;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    v_ti = py::mul(&v_tauI.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?;
    let t3793 = (f__pi_loop(v_Kc.clone(), v_ti.clone(), v_K.clone(), v_ts.clone())?).items()?;
    if t3793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mag = t3793[0].clone();
    v_ph = t3793[1].clone();
    let t3794 = (Py::tuple(vec![py::truediv(&Py::Float(1e-06), &py::minmax(&[py::add(&v_ts.clone(), &Py::list(vec![v_ti.clone()]))?], true)?)?, py::truediv(&Py::Float(10000.0), &py::minmax(&[v_ts.clone()], false)?)?])).items()?;
    if t3794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3794[0].clone();
    v_hi = t3794[1].clone();
    v_grid = { let mut out = vec![]; for t3795 in py::range_iter(&[(Py::Int(4001i64)).i()?]) { let mut v_i: Py; 
v_i = t3795;
out.push(py::mul(&v_lo.clone(), &py::pow(&py::truediv(&v_hi.clone(), &v_lo.clone())?, &py::truediv(&v_i.clone(), &Py::Float(4000.0))?)?)?); } Py::list(out) };
    v_cross = { let mut out = vec![]; for t3796 in py::range_iter(&[(Py::Int(4000i64)).i()?]) { let mut v_i: Py; 
v_i = t3796;
if ({ let t3797 = py::mul(&py::add(&py::call(&v_ph.clone(), &[py::getitem(&v_grid.clone(), &v_i.clone())?])?, &Py::Float(std::f64::consts::PI))?, &py::add(&py::call(&v_ph.clone(), &[py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])?, &Py::Float(std::f64::consts::PI))?)?; Py::Bool({ let t3798 = Py::Int(0i64); if py::le(&t3797, &t3798)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) };
    let _ = f__need({ let t3799 = py::len(&v_cross.clone())?; Py::Bool({ let t3800 = Py::Int(1i64); if py::eq(&t3799, &t3800) { true } else { false } }) }, Py::str("the phase crosses -180 degrees more than once (or never): margins are not unique"))?;
    v_i = py::getitem(&v_cross.clone(), &Py::Int(0i64))?;
    v_wpc = f__bisect({ let v_K = v_K.clone(); let v_Kc = v_Kc.clone(); let v_cross = v_cross.clone(); let v_grid = v_grid.clone(); let v_hi = v_hi.clone(); let v_i = v_i.clone(); let v_lo = v_lo.clone(); let v_mag = v_mag.clone(); let v_ph = v_ph.clone(); let v_t_unit = v_t_unit.clone(); let v_tauI = v_tauI.clone(); let v_taus = v_taus.clone(); let v_ti = v_ti.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::call(&v_ph.clone(), &[v_w.clone()])?, &Py::Float(std::f64::consts::PI))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::getitem(&v_grid.clone(), &v_i.clone())?, py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, Py::Int(500i64))?;
    v_lg = { let v_K = v_K.clone(); let v_Kc = v_Kc.clone(); let v_cross = v_cross.clone(); let v_grid = v_grid.clone(); let v_hi = v_hi.clone(); let v_i = v_i.clone(); let v_lo = v_lo.clone(); let v_mag = v_mag.clone(); let v_ph = v_ph.clone(); let v_t_unit = v_t_unit.clone(); let v_tauI = v_tauI.clone(); let v_taus = v_taus.clone(); let v_ti = v_ti.clone(); let v_ts = v_ts.clone(); let v_wpc = v_wpc.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::math1("log", &py::call(&v_mag.clone(), &[py::math1("exp", &v_u.clone())?])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_wgc = py::math1("exp", &f__bisect(v_lg.clone(), py::math1("log", &v_lo.clone())?, py::math1("log", &v_hi.clone())?, Py::Int(500i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("GM"), py::truediv(&Py::Float(1.0), &py::call(&v_mag.clone(), &[v_wpc.clone()])?)?), (Py::str("PM_deg"), py::add(&Py::Float(180.0), &py::math1("degrees", &py::call(&v_ph.clone(), &[v_wgc.clone()])?)?)?), (Py::str("w_pc"), v_wpc.clone()), (Py::str("w_gc"), v_wgc.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_to_mole(mut v_w: Py, mut v_MW: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__need({ let t2782 = Py::Int(2i64); Py::Bool({ let t2783 = py::len(&v_w.clone())?; if py::le(&t2782, &t2783)? { { let t2784 = py::len(&v_MW.clone())?; if py::eq(&t2783, &t2784) { { let t2785 = Py::Int(20i64); if py::le(&t2784, &t2785)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("give one molar mass per mass fraction (2 to 20 components)"))?;
    let _ = f__fracs(Py::str("w"), v_w.clone(), Py::Float(1e-06))?;
    let _ = f__pos(Py::str("the molar masses"), { let mut v = vec![]; v.extend((v_MW.clone()).items()?); Py::tuple(v) })?;
    v_s = py::sum(&{ let mut out = vec![]; for t2799 in py::iter(&(py::zip(&[v_w.clone(), v_MW.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2800 = (t2799).items()?;
if t2800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2800[0].clone();
v_b = t2800[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    v_z = { let mut out = vec![]; for t2801 in py::iter(&(py::zip(&[v_w.clone(), v_MW.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2802 = (t2801).items()?;
if t2802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2802[0].clone();
v_b = t2802[1].clone();
out.push(py::truediv(&py::truediv(&v_a.clone(), &v_b.clone())?, &v_s.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("MW_mean"), py::mul(&Py::Float(0.001), &py::sum(&{ let mut out = vec![]; for t2815 in py::iter(&(py::zip(&[v_z.clone(), v_MW.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2816 = (t2815).items()?;
if t2816.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2816[0].clone();
v_b = t2816[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_transfer_coefficient(mut v_geometry: Py, mut v_u: Py, mut v_L: Py, mut v_D: Py, mut v_rho: Py, mut v_mu: Py) -> R {
    let mut v_Re: Py = Py::None;
    let mut v_Sc: Py = Py::None;
    let mut v_Sh: Py = Py::None;
    let _ = f__pos(Py::str("u, L, D, rho and mu"), Py::tuple(vec![v_u.clone(), v_L.clone(), v_D.clone(), v_rho.clone(), v_mu.clone()]))?;
    let _ = f__need({ let t2849 = v_geometry.clone(); Py::Bool({ let t2850 = c__SH_CORR()?; if py::contains(&t2850, &t2849)? { true } else { false } }) }, Py::str("geometry must be sphere, flat_plate or pipe"))?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_u.clone())?, &v_L.clone())?, &v_mu.clone())?;
    v_Sc = py::truediv(&v_mu.clone(), &py::mul(&v_rho.clone(), &v_D.clone())?)?;
    if ({ let t2851 = v_geometry.clone(); Py::Bool({ let t2852 = Py::str("sphere"); if py::eq(&t2851, &t2852) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2859 = { let t2853 = Py::Int(2i64); Py::Bool({ let t2854 = v_Re.clone(); if py::le(&t2853, &t2854)? { { let t2855 = Py::Float(200000.0); if py::le(&t2854, &t2855)? { true } else { false } } } else { false } }) }; if !t2859.truthy() { t2859 } else { { let t2856 = Py::Float(0.6); Py::Bool({ let t2857 = v_Sc.clone(); if py::le(&t2856, &t2857)? { { let t2858 = Py::Int(400i64); if py::le(&t2857, &t2858)? { true } else { false } } } else { false } }) } } }, Py::str("Ranz-Marshall is stated for 2 <= Re <= 200000 and 0.6 <= Sc <= 400"))?;
    } else {
        if ({ let t2860 = v_geometry.clone(); Py::Bool({ let t2861 = Py::str("flat_plate"); if py::eq(&t2860, &t2861) { true } else { false } }) }).truthy() {
            let _ = f__need({ let t2866 = { let t2862 = v_Re.clone(); Py::Bool({ let t2863 = Py::Float(500000.0); if py::lt(&t2862, &t2863)? { true } else { false } }) }; if !t2866.truthy() { t2866 } else { { let t2864 = v_Sc.clone(); Py::Bool({ let t2865 = Py::Float(0.6); if py::ge(&t2864, &t2865)? { true } else { false } }) } } }, Py::str("the laminar flat-plate correlation needs Re < 500000 and Sc >= 0.6"))?;
        } else {
            let _ = f__need({ let t2873 = { let t2867 = Py::Int(2000i64); Py::Bool({ let t2868 = v_Re.clone(); if py::le(&t2867, &t2868)? { { let t2869 = Py::Int(70000i64); if py::le(&t2868, &t2869)? { true } else { false } } } else { false } }) }; if !t2873.truthy() { t2873 } else { { let t2870 = Py::Float(0.6); Py::Bool({ let t2871 = v_Sc.clone(); if py::le(&t2870, &t2871)? { { let t2872 = Py::Int(3000i64); if py::le(&t2871, &t2872)? { true } else { false } } } else { false } }) } } }, Py::str("Linton-Sherwood is stated for 2000 <= Re <= 70000 and 0.6 <= Sc <= 3000"))?;
        }
    }
    v_Sh = f__sherwood(v_geometry.clone(), v_Re.clone(), v_Sc.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Re"), v_Re.clone()), (Py::str("Sc"), v_Sc.clone()), (Py::str("Sh"), v_Sh.clone()), (Py::str("kc"), py::truediv(&py::mul(&v_Sh.clone(), &v_D.clone())?, &v_L.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mccabe_thiele(mut v_alpha: Py, mut v_zF: Py, mut v_xD: Py, mut v_xB: Py, mut v_q: Py, mut v_R: Py) -> R {
    let mut v_Rm: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xp: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let _ = f__dist_check(v_alpha.clone(), v_zF.clone(), v_xD.clone(), v_q.clone(), v_xB.clone())?;
    let _ = f__fin(Py::tuple(vec![v_R.clone()]))?;
    v_Rm = f__underwood_rmin(v_alpha.clone(), v_zF.clone(), v_xD.clone(), v_q.clone())?;
    let _ = f__need({ let t2946 = { let t2942 = v_R.clone(); Py::Bool({ let t2943 = py::mul(&v_Rm.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-06))?)?; if py::gt(&t2942, &t2943)? { true } else { false } }) }; if !t2946.truthy() { t2946 } else { { let t2944 = v_R.clone(); Py::Bool({ let t2945 = Py::Int(0i64); if py::gt(&t2944, &t2945)? { true } else { false } }) } } }, Py::str(&["the reflux ratio must exceed the minimum reflux ratio ".to_string(), py::format(&v_Rm.clone(), ".6g")?].concat()))?;
    let t2956 = (f__mt_lines(v_alpha.clone(), v_zF.clone(), v_xD.clone(), v_xB.clone(), v_q.clone(), v_R.clone())?).items()?;
    if t2956.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t2956[0].clone();
    v_b1 = t2956[1].clone();
    v_xp = t2956[2].clone();
    v_yp = t2956[3].clone();
    v_s = t2956[4].clone();
    let t2957 = (Py::tuple(vec![py::truediv(&v_xD.clone(), &py::sub(&v_alpha.clone(), &py::mul(&py::sub(&v_alpha.clone(), &Py::Int(1i64))?, &v_xD.clone())?)?)?, Py::Int(1i64)])).items()?;
    if t2957.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2957[0].clone();
    v_n = t2957[1].clone();
    while ({ let t2962 = v_x.clone(); Py::Bool({ let t2963 = v_xB.clone(); if py::gt(&t2962, &t2963)? { true } else { false } }) }).truthy() {
        v_y = (if ({ let t2958 = v_x.clone(); Py::Bool({ let t2959 = v_xp.clone(); if py::gt(&t2958, &t2959)? { true } else { false } }) }).truthy() { py::add(&py::mul(&v_a1.clone(), &v_x.clone())?, &v_b1.clone())? } else { py::add(&v_xB.clone(), &py::mul(&v_s.clone(), &py::sub(&v_x.clone(), &v_xB.clone())?)?)? });
        v_x = py::truediv(&v_y.clone(), &py::sub(&v_alpha.clone(), &py::mul(&py::sub(&v_alpha.clone(), &Py::Int(1i64))?, &v_y.clone())?)?)?;
        v_n = py::iadd(&v_n, &Py::Int(1i64))?;
        let _ = f__need({ let t2960 = v_n.clone(); Py::Bool({ let t2961 = Py::Int(400i64); if py::le(&t2960, &t2961)? { true } else { false } }) }, Py::str("more than 400 stages: the reflux ratio is too close to the minimum"))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("N"), v_n.clone()), (Py::str("Rmin"), v_Rm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minimum_reflux(mut v_alpha: Py, mut v_zF: Py, mut v_xD: Py, mut v_q: Py) -> R {
    let mut v_Rm: Py = Py::None;
    let _ = f__dist_check(v_alpha.clone(), v_zF.clone(), v_xD.clone(), v_q.clone(), Py::None)?;
    v_Rm = f__underwood_rmin(v_alpha.clone(), v_zF.clone(), v_xD.clone(), v_q.clone())?;
    let _ = f__need({ let t771 = v_Rm.clone(); Py::Bool({ let t772 = Py::Int(0i64); if py::gt(&t771, &t772)? { true } else { false } }) }, Py::str("the feed is richer than the distillate at the pinch: no minimum reflux exists"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Rmin"), v_Rm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mixing_tank(mut v_V: Py, mut v_q: Py, mut v_C_in: Py, mut v_C0: Py, mut v_t: Py) -> R {
    let _ = f__pos(Py::str("the volume V and flow q"), Py::tuple(vec![v_V.clone(), v_q.clone()]))?;
    let _ = f__nonneg(Py::str("the concentrations and time"), Py::tuple(vec![v_C_in.clone(), v_C0.clone(), v_t.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), py::add(&v_C_in.clone(), &py::mul(&py::sub(&v_C0.clone(), &v_C_in.clone())?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_q.clone())?, &v_t.clone())?, &v_V.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_modified_raoult_bubble(mut v_x: Py, mut v_gamma: Py, mut v_Psat: Py, mut v_p_unit: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__need({ let t3010 = { let t3002 = Py::Int(2i64); Py::Bool({ let t3003 = v_n.clone(); if py::le(&t3002, &t3003)? { { let t3004 = Py::Int(10i64); if py::le(&t3003, &t3004)? { true } else { false } } } else { false } }) }; if !t3010.truthy() { t3010 } else { { let t3009 = { let t3005 = py::len(&v_gamma.clone())?; Py::Bool({ let t3006 = v_n.clone(); if py::eq(&t3005, &t3006) { true } else { false } }) }; if !t3009.truthy() { t3009 } else { { let t3007 = py::len(&v_Psat.clone())?; Py::Bool({ let t3008 = v_n.clone(); if py::eq(&t3007, &t3008) { true } else { false } }) } } } } }, Py::str("x, gamma and Psat need one entry per component (2 to 10)"))?;
    let _ = f__fracs(Py::str("x"), v_x.clone(), Py::Float(1e-06))?;
    for t3018 in py::iter(&(v_gamma.clone()))? {
        v_g = t3018;
        let _ = f__need({ let t3022 = py::math1("isfinite", &v_g.clone())?; if !t3022.truthy() { t3022 } else { { let t3019 = Py::Int(0i64); Py::Bool({ let t3020 = v_g.clone(); if py::lt(&t3019, &t3020)? { { let t3021 = Py::Int(100i64); if py::le(&t3020, &t3021)? { true } else { false } } } else { false } }) } } }, Py::str("every activity coefficient must lie in (0, 100]"))?;
    }
    for t3023 in py::iter(&(v_Psat.clone()))? {
        v_p = t3023;
        let _ = f__need({ let t3026 = py::math1("isfinite", &v_p.clone())?; if !t3026.truthy() { t3026 } else { { let t3024 = v_p.clone(); Py::Bool({ let t3025 = Py::Int(0i64); if py::gt(&t3024, &t3025)? { true } else { false } }) } } }, Py::str("every vapour pressure must be positive"))?;
    }
    v_f = py::getitem(&c__PUNIT()?, &v_p_unit.clone())?;
    v_parts = { let mut out = vec![]; for t3027 in py::iter(&(py::zip(&[v_x.clone(), v_gamma.clone(), v_Psat.clone()])?))? { let mut v_gi: Py; let mut v_pi: Py; let mut v_xi: Py; 
let t3028 = (t3027).items()?;
if t3028.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_xi = t3028[0].clone();
v_gi = t3028[1].clone();
v_pi = t3028[2].clone();
out.push(py::mul(&py::mul(&py::mul(&v_xi.clone(), &v_gi.clone())?, &v_pi.clone())?, &v_f.clone())?); } Py::list(out) };
    v_P = py::sum(&v_parts.clone(), None)?;
    v_y = { let mut out = vec![]; for t3029 in py::iter(&(v_parts.clone()))? { let mut v_v: Py; 
v_v = t3029;
out.push(py::truediv(&v_v.clone(), &v_P.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("y1"), py::getitem(&v_y.clone(), &Py::Int(0i64))?), (Py::str("y"), v_y.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_npsh_available(mut v_p_s: Py, mut v_p_v: Py, mut v_rho: Py, mut v_z_s: Py, mut v_h_f: Py, mut v_NPSHr: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("p_s and rho"), Py::tuple(vec![v_p_s.clone(), v_rho.clone()]))?;
    let _ = f__need({ let t3171 = v_p_v.clone(); Py::Bool({ let t3172 = Py::Int(0i64); if py::ge(&t3171, &t3172)? { true } else { false } }) }, Py::str("the vapour pressure must be zero or positive"))?;
    let _ = f__fin(Py::tuple(vec![v_z_s.clone(), v_h_f.clone()]))?;
    let _ = f__need({ let t3176 = v_h_f.clone(); Py::Bool({ let t3177 = Py::Int(0i64); if py::ge(&t3176, &t3177)? { true } else { false } }) }, Py::str("the suction friction head loss must be zero or positive"))?;
    v_N = py::sub(&py::add(&py::truediv(&py::sub(&v_p_s.clone(), &v_p_v.clone())?, &py::mul(&v_rho.clone(), &c_G0()?)?)?, &v_z_s.clone())?, &v_h_f.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("NPSHa"), v_N.clone()), (Py::str("pressure_head"), py::truediv(&v_p_s.clone(), &py::mul(&v_rho.clone(), &c_G0()?)?)?), (Py::str("vapour_head"), py::truediv(&v_p_v.clone(), &py::mul(&v_rho.clone(), &c_G0()?)?)?)])?;
    if ({ let t3178 = v_NPSHr.clone(); Py::Bool({ let t3179 = Py::None; if !py::is(&t3178, &t3179) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t3180 = v_NPSHr.clone(); Py::Bool({ let t3181 = Py::Int(0i64); if py::ge(&t3180, &t3181)? { true } else { false } }) }, Py::str("the required NPSH must be zero or positive"))?;
        py::setitem(&v_out.clone(), &Py::str("margin"), py::sub(&v_N.clone(), &v_NPSHr.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nrtl_gamma(mut v_x: Py, mut v_tau: Py, mut v_alpha: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__liq_x(v_x.clone())?;
    let _ = f__square(Py::str("tau"), v_tau.clone(), v_n.clone(), Py::Float(0.0))?;
    let _ = f__square(Py::str("alpha"), v_alpha.clone(), v_n.clone(), Py::Float(0.0))?;
    v_G = { let mut out = vec![]; for t3245 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3245;
out.push({ let mut out = vec![]; for t3246 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3246;
out.push(py::math1("exp", &py::mul(&py::neg(&py::getitem(&py::getitem(&v_alpha.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&py::getitem(&v_tau.clone(), &v_i.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_D = { let mut out = vec![]; for t3247 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3247;
out.push(py::sum(&{ let mut out = vec![]; for t3248 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3248;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_G.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_N = { let mut out = vec![]; for t3249 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3249;
out.push(py::sum(&{ let mut out = vec![]; for t3250 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3250;
out.push(py::mul(&py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_tau.clone(), &v_k.clone())?, &v_j.clone())?)?, &py::getitem(&py::getitem(&v_G.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_lg = { let mut out = vec![]; for t3251 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3251;
out.push(py::add(&py::truediv(&py::getitem(&v_N.clone(), &v_i.clone())?, &py::getitem(&v_D.clone(), &v_i.clone())?)?, &py::sum(&{ let mut out = vec![]; for t3252 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3252;
out.push(py::mul(&py::truediv(&py::mul(&py::getitem(&v_x.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_G.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_D.clone(), &v_j.clone())?)?, &py::sub(&py::getitem(&py::getitem(&v_tau.clone(), &v_i.clone())?, &v_j.clone())?, &py::truediv(&py::getitem(&v_N.clone(), &v_j.clone())?, &py::getitem(&v_D.clone(), &v_j.clone())?)?)?)?); } Py::list(out) }, None)?)?); } Py::list(out) };
    v_g = { let mut out = vec![]; for t3253 in py::iter(&(v_lg.clone()))? { let mut v_v: Py; 
v_v = t3253;
out.push(py::math1("exp", &v_v.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("gamma1"), py::getitem(&v_g.clone(), &Py::Int(0i64))?), (Py::str("gamma"), v_g.clone()), (Py::str("GE_RT"), py::sum(&{ let mut out = vec![]; for t3266 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3266;
out.push(py::truediv(&py::mul(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_N.clone(), &v_i.clone())?)?, &py::getitem(&v_D.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_p_offset(mut v_Kc: Py, mut v_K: Py, mut v_taus: Py, mut v_t_unit: Py) -> R {
    let mut v_char: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_Kc.clone(), v_K.clone()]))?;
    let _ = f__need({ let t3296 = py::mul(&v_Kc.clone(), &v_K.clone())?; Py::Bool({ let t3297 = Py::Int(0i64); if py::gt(&t3296, &t3297)? { true } else { false } }) }, Py::str("the loop gain Kc K must be positive (negative feedback)"))?;
    let _ = f__need({ let t3298 = Py::Int(1i64); Py::Bool({ let t3299 = py::len(&v_taus.clone())?; if py::le(&t3298, &t3299)? { { let t3300 = Py::Int(6i64); if py::le(&t3299, &t3300)? { true } else { false } } } else { false } }) }, Py::str("give 1 to 6 time constants"))?;
    let _ = f__pos(Py::str("the time constants"), { let mut v = vec![]; v.extend((v_taus.clone()).items()?); Py::tuple(v) })?;
    v_char = f__lags_poly({ let mut out = vec![]; for t3314 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t3314;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) })?;
    let t3315 = v_char.clone(); let t3316 = py::neg(&Py::Int(1i64))?;
    py::setitem(&t3315, &t3316, py::iadd(&py::getitem(&t3315, &t3316)?, &py::mul(&v_Kc.clone(), &v_K.clone())?)?)?;
    let _ = f__need(f__routh_stable(v_char.clone())?, Py::str("the closed loop is unstable at this controller gain: there is no steady state"))?;
    v_y = py::truediv(&py::mul(&v_Kc.clone(), &v_K.clone())?, &py::add(&Py::Float(1.0), &py::mul(&v_Kc.clone(), &v_K.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("y_ss"), v_y.clone()), (Py::str("offset"), py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::mul(&v_Kc.clone(), &v_K.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pfr_conv_1(mut v_v0: Py, mut v_V: Py, mut v_k: Py) -> R {
    let _ = f__pos(Py::str("v0, V and k"), Py::tuple(vec![v_v0.clone(), v_V.clone(), v_k.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::neg(&py::math1("expm1", &py::truediv(&py::mul(&py::neg(&v_k.clone())?, &v_V.clone())?, &v_v0.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pfr_conv_2(mut v_v0: Py, mut v_V: Py, mut v_k: Py, mut v_CA0: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("v0, V, k and CA0"), Py::tuple(vec![v_v0.clone(), v_V.clone(), v_k.clone(), v_CA0.clone()]))?;
    v_d = py::mul(&py::truediv(&py::mul(&v_k.clone(), &v_V.clone())?, &v_v0.clone())?, &v_CA0.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::truediv(&v_d.clone(), &py::add(&Py::Float(1.0), &v_d.clone())?)?), (Py::str("CA"), py::truediv(&v_CA0.clone(), &py::add(&Py::Float(1.0), &v_d.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pfr_gas_1(mut v_FA0: Py, mut v_CA0: Py, mut v_k: Py, mut v_eps: Py, mut v_X: Py) -> R {
    let _ = f__pos(Py::str("FA0, CA0 and k"), Py::tuple(vec![v_FA0.clone(), v_CA0.clone(), v_k.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    let _ = f__eps_ok(v_eps.clone(), v_X.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::mul(&py::truediv(&v_FA0.clone(), &py::mul(&v_k.clone(), &v_CA0.clone())?)?, &py::sub(&py::mul(&py::neg(&py::add(&Py::Float(1.0), &v_eps.clone())?)?, &py::math1("log1p", &py::neg(&v_X.clone())?)?)?, &py::mul(&v_eps.clone(), &v_X.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pfr_gas_2(mut v_FA0: Py, mut v_CA0: Py, mut v_k: Py, mut v_eps: Py, mut v_X: Py) -> R {
    let _ = f__pos(Py::str("FA0, CA0 and k"), Py::tuple(vec![v_FA0.clone(), v_CA0.clone(), v_k.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    let _ = f__eps_ok(v_eps.clone(), v_X.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::mul(&py::truediv(&v_FA0.clone(), &py::mul(&v_k.clone(), &py::pow(&v_CA0.clone(), &Py::Int(2i64))?)?)?, &py::add(&py::add(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_eps.clone())?, &py::add(&Py::Int(1i64), &v_eps.clone())?)?, &py::math1("log1p", &py::neg(&v_X.clone())?)?)?, &py::mul(&py::pow(&v_eps.clone(), &Py::Int(2i64))?, &v_X.clone())?)?, &py::truediv(&py::mul(&py::pow(&py::add(&Py::Int(1i64), &v_eps.clone())?, &Py::Int(2i64))?, &v_X.clone())?, &py::sub(&Py::Int(1i64), &v_X.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pfr_volume_1(mut v_v0: Py, mut v_k: Py, mut v_X: Py) -> R {
    let _ = f__pos(Py::str("v0 and k"), Py::tuple(vec![v_v0.clone(), v_k.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::truediv(&py::mul(&py::neg(&v_v0.clone())?, &py::math1("log1p", &py::neg(&v_X.clone())?)?)?, &v_k.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pfr_volume_2(mut v_v0: Py, mut v_k: Py, mut v_CA0: Py, mut v_X: Py) -> R {
    let _ = f__pos(Py::str("v0, k and CA0"), Py::tuple(vec![v_v0.clone(), v_k.clone(), v_CA0.clone()]))?;
    let _ = f__conv_ok(v_X.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::truediv(&py::mul(&v_v0.clone(), &v_X.clone())?, &py::mul(&py::mul(&v_k.clone(), &v_CA0.clone())?, &py::sub(&Py::Float(1.0), &v_X.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pid_position(mut v_Kc: Py, mut v_tauI: Py, mut v_tauD: Py, mut v_dt: Py, mut v_e: Py, mut v_bias: Py) -> R {
    let mut v_ek: Py = Py::None;
    let mut v_prev: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_Kc.clone(), v_bias.clone()]))?;
    let _ = f__pos(Py::str("tauI and dt"), Py::tuple(vec![v_tauI.clone(), v_dt.clone()]))?;
    let _ = f__nonneg(Py::str("tauD"), Py::tuple(vec![v_tauD.clone()]))?;
    let _ = f__need({ let t3880 = Py::Int(1i64); Py::Bool({ let t3881 = py::len(&v_e.clone())?; if py::le(&t3880, &t3881)? { { let t3882 = Py::Int(500i64); if py::le(&t3881, &t3882)? { true } else { false } } } else { false } }) }, Py::str("give 1 to 500 error samples"))?;
    let t3883 = (Py::tuple(vec![Py::list(vec![]), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3883.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3883[0].clone();
    v_s = t3883[1].clone();
    v_prev = t3883[2].clone();
    for t3884 in py::iter(&(v_e.clone()))? {
        v_ek = t3884;
        v_s = py::iadd(&v_s, &v_ek.clone())?;
        let _ = py::method(&v_u.clone(), "append", &[py::add(&v_bias.clone(), &py::mul(&v_Kc.clone(), &py::add(&py::add(&v_ek.clone(), &py::mul(&py::truediv(&v_dt.clone(), &v_tauI.clone())?, &v_s.clone())?)?, &py::mul(&py::truediv(&v_tauD.clone(), &v_dt.clone())?, &py::sub(&v_ek.clone(), &v_prev.clone())?)?)?)?)?])?;
        v_prev = v_ek.clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("u_last"), py::getitem(&v_u.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("u"), v_u.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pitzer_hvap(mut v_T: Py, mut v_Tc: Py, mut v_omega: Py) -> R {
    let mut v_tr: Py = Py::None;
    let _ = f__pos(Py::str("T and Tc"), Py::tuple(vec![v_T.clone(), v_Tc.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_omega.clone()]))?;
    let _ = f__need({ let t3932 = v_T.clone(); Py::Bool({ let t3933 = v_Tc.clone(); if py::lt(&t3932, &t3933)? { true } else { false } }) }, Py::str("T must be below the critical temperature"))?;
    v_tr = py::sub(&Py::Float(1.0), &py::truediv(&v_T.clone(), &v_Tc.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Hvap"), py::mul(&py::mul(&c_R_GAS()?, &v_Tc.clone())?, &py::add(&py::mul(&Py::Float(7.08), &py::pow(&v_tr.clone(), &Py::Float(0.354))?)?, &py::mul(&py::mul(&Py::Float(10.95), &v_omega.clone())?, &py::pow(&v_tr.clone(), &Py::Float(0.456))?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pump_head(mut v_p1: Py, mut v_p2: Py, mut v_rho: Py, mut v_dz: Py, mut v_v1: Py, mut v_v2: Py, mut v_Q: Py, mut v_eta: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("rho"), Py::tuple(vec![v_rho.clone()]))?;
    let _ = f__need({ let t3982 = { let t3978 = v_p1.clone(); Py::Bool({ let t3979 = Py::Int(0i64); if py::gt(&t3978, &t3979)? { true } else { false } }) }; if !t3982.truthy() { t3982 } else { { let t3980 = v_p2.clone(); Py::Bool({ let t3981 = Py::Int(0i64); if py::gt(&t3980, &t3981)? { true } else { false } }) } } }, Py::str("pressures are absolute and must be positive"))?;
    let _ = f__fin(Py::tuple(vec![v_dz.clone(), v_v1.clone(), v_v2.clone()]))?;
    let _ = f__need({ let t3990 = { let t3986 = v_v1.clone(); Py::Bool({ let t3987 = Py::Int(0i64); if py::ge(&t3986, &t3987)? { true } else { false } }) }; if !t3990.truthy() { t3990 } else { { let t3988 = v_v2.clone(); Py::Bool({ let t3989 = Py::Int(0i64); if py::ge(&t3988, &t3989)? { true } else { false } }) } } }, Py::str("the speeds must be zero or positive"))?;
    v_H = py::add(&py::add(&py::truediv(&py::sub(&v_p2.clone(), &v_p1.clone())?, &py::mul(&v_rho.clone(), &c_G0()?)?)?, &v_dz.clone())?, &py::truediv(&py::sub(&py::mul(&v_v2.clone(), &v_v2.clone())?, &py::mul(&v_v1.clone(), &v_v1.clone())?)?, &py::mul(&Py::Int(2i64), &c_G0()?)?)?)?;
    let _ = f__need({ let t3991 = v_H.clone(); Py::Bool({ let t3992 = Py::Int(0i64); if py::gt(&t3991, &t3992)? { true } else { false } }) }, Py::str("the pump must add head (H > 0)"))?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("H"), v_H.clone()), (Py::str("dp_equivalent"), py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_H.clone())?)])?;
    if ({ let t3993 = v_Q.clone(); Py::Bool({ let t3994 = Py::None; if !py::is(&t3993, &t3994) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("Q"), Py::tuple(vec![v_Q.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("P_hydraulic"), py::mul(&py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_Q.clone())?, &v_H.clone())?)?;
        if ({ let t3995 = v_eta.clone(); Py::Bool({ let t3996 = Py::None; if !py::is(&t3995, &t3996) { true } else { false } }) }).truthy() {
            let _ = f__need({ let t3997 = Py::Int(0i64); Py::Bool({ let t3998 = v_eta.clone(); if py::lt(&t3997, &t3998)? { { let t3999 = Py::Int(1i64); if py::le(&t3998, &t3999)? { true } else { false } } } else { false } }) }, Py::str("the pump efficiency must lie in (0, 1]"))?;
            py::setitem(&v_out.clone(), &Py::str("P_shaft"), py::truediv(&py::mul(&py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_Q.clone())?, &v_H.clone())?, &v_eta.clone())?)?;
        }
    } else {
        if ({ let t4000 = v_eta.clone(); Py::Bool({ let t4001 = Py::None; if !py::is(&t4000, &t4001) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the shaft power needs the flow Q as well as the efficiency"))));
        }
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rachford_rice(mut v_z: Py, mut v_K: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__comp(Py::str("z"), v_z.clone())?;
    let _ = f__need({ let t4056 = { let t4051 = py::len(&v_K.clone())?; Py::Bool({ let t4052 = py::len(&v_z.clone())?; if py::eq(&t4051, &t4052) { true } else { false } }) }; if !t4056.truthy() { t4056 } else { Py::Bool(({ let mut out = vec![]; for t4053 in py::iter(&(v_K.clone()))? { let mut v_k: Py; 
v_k = t4053;
out.push({ let t4054 = v_k.clone(); Py::Bool({ let t4055 = Py::Int(0i64); if py::gt(&t4054, &t4055)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("give one positive K-value per component"))?;
    let t4107 = (f__rr(v_z.clone(), v_K.clone())?).items()?;
    if t4107.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t4107[0].clone();
    v_x = t4107[1].clone();
    v_y = t4107[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("VF"), v_V.clone()), (Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rackett(mut v_T: Py, mut v_Tc: Py, mut v_Pc: Py, mut v_Zc: Py) -> R {
    let _ = f__pos(Py::str("T, Tc, Pc and Zc"), Py::tuple(vec![v_T.clone(), v_Tc.clone(), v_Pc.clone(), v_Zc.clone()]))?;
    let _ = f__need({ let t4156 = { let t4152 = v_T.clone(); Py::Bool({ let t4153 = v_Tc.clone(); if py::lt(&t4152, &t4153)? { true } else { false } }) }; if !t4156.truthy() { t4156 } else { { let t4154 = v_Zc.clone(); Py::Bool({ let t4155 = Py::Int(1i64); if py::lt(&t4154, &t4155)? { true } else { false } }) } } }, Py::str("T must be below Tc and Zc below 1"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Vm"), py::mul(&py::truediv(&py::mul(&c_R_GAS()?, &v_Tc.clone())?, &v_Pc.clone())?, &py::pow(&v_Zc.clone(), &py::add(&Py::Float(1.0), &py::pow(&py::sub(&Py::Float(1.0), &py::truediv(&v_T.clone(), &v_Tc.clone())?)?, &py::truediv(&Py::Float(2.0), &Py::Float(7.0))?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radial_flow(mut v_k: Py, mut v_h: Py, mut v_mu: Py, mut v_pe: Py, mut v_pw: Py, mut v_re: Py, mut v_rw: Py, mut v_S: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("k, h, mu, re and rw"), Py::tuple(vec![v_k.clone(), v_h.clone(), v_mu.clone(), v_re.clone(), v_rw.clone()]))?;
    let _ = f__pos(Py::str("the pressures"), Py::tuple(vec![v_pe.clone(), v_pw.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_S.clone()]))?;
    let _ = f__need({ let t4204 = v_re.clone(); Py::Bool({ let t4205 = v_rw.clone(); if py::gt(&t4204, &t4205)? { true } else { false } }) }, Py::str("the drainage radius re must exceed the well radius rw"))?;
    let _ = f__need({ let t4206 = v_pe.clone(); Py::Bool({ let t4207 = v_pw.clone(); if py::gt(&t4206, &t4207)? { true } else { false } }) }, Py::str("the boundary pressure must exceed the flowing well pressure"))?;
    v_d = py::add(&py::math1("log", &py::truediv(&v_re.clone(), &v_rw.clone())?)?, &v_S.clone())?;
    let _ = f__need({ let t4208 = v_d.clone(); Py::Bool({ let t4209 = Py::Int(0i64); if py::gt(&t4208, &t4209)? { true } else { false } }) }, Py::str("the skin is so negative that the flow resistance vanishes"))?;
    v_q = py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_h.clone())?, &py::sub(&v_pe.clone(), &v_pw.clone())?)?, &py::mul(&v_mu.clone(), &v_d.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("PI"), py::truediv(&v_q.clone(), &py::sub(&v_pe.clone(), &v_pw.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_raoult_flash(mut v_z: Py, mut v_ant: Py, mut v_units: Py, mut v_base: Py, mut v_T: Py, mut v_P: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__comp(Py::str("z"), v_z.clone())?;
    let _ = f__antoine_rows(v_ant.clone(), py::len(&v_z.clone())?)?;
    let _ = f__pos(Py::str("T and P"), Py::tuple(vec![v_T.clone(), v_P.clone()]))?;
    v_K = { let mut out = vec![]; for t4649 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4649;
out.push(py::truediv(&f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?, &v_P.clone())?); } Py::list(out) };
    let t4704 = (f__rr(v_z.clone(), v_K.clone())?).items()?;
    if t4704.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t4704[0].clone();
    v_x = t4704[1].clone();
    v_y = t4704[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("VF"), v_V.clone()), (Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_enthalpy_T(mut v_nu: Py, mut v_cp: Py, mut v_dH0: Py, mut v_T0: Py, mut v_T: Py) -> R {
    let mut v_dco: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rows: Py = Py::None;
    v_n = py::len(&v_nu.clone())?;
    v_rows = f__cp_matrix(v_cp.clone(), v_n.clone())?;
    let _ = f__fin({ let mut v = vec![v_dH0.clone()]; v.extend((v_nu.clone()).items()?); Py::tuple(v) })?;
    let _ = f__pos(Py::str("the temperatures"), Py::tuple(vec![v_T0.clone(), v_T.clone()]))?;
    v_dco = { let mut out = vec![]; for t4759 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_j: Py; 
v_j = t4759;
out.push(py::sum(&{ let mut out = vec![]; for t4760 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4760;
out.push(py::mul(&py::getitem(&v_nu.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_rows.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("dH"), py::add(&v_dH0.clone(), &f__cp_int(v_dco.clone(), v_T0.clone(), v_T.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_recycle_purge(mut v_F0: Py, mut v_zA: Py, mut v_X: Py, mut v_f: Py) -> R {
    let mut v_A_in: Py = Py::None;
    let mut v_I_in: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_S_A: Py = Py::None;
    let mut v_S_I: Py = Py::None;
    let _ = f__pos(Py::str("the fresh feed F0"), Py::tuple(vec![v_F0.clone()]))?;
    let _ = f__frac(Py::str("zA"), v_zA.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("the single-pass conversion X"), v_X.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__frac(Py::str("the purge fraction f"), v_f.clone(), Py::Bool(true), Py::Bool(true))?;
    v_A_in = py::truediv(&py::mul(&v_F0.clone(), &v_zA.clone())?, &py::sub(&Py::Float(1.0), &py::mul(&py::sub(&Py::Float(1.0), &v_f.clone())?, &py::sub(&Py::Float(1.0), &v_X.clone())?)?)?)?;
    v_I_in = py::truediv(&py::mul(&v_F0.clone(), &py::sub(&Py::Float(1.0), &v_zA.clone())?)?, &v_f.clone())?;
    let t4818 = (Py::tuple(vec![py::mul(&py::sub(&Py::Float(1.0), &v_X.clone())?, &v_A_in.clone())?, v_I_in.clone()])).items()?;
    if t4818.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S_A = t4818[0].clone();
    v_S_I = t4818[1].clone();
    v_S = py::add(&v_S_A.clone(), &v_S_I.clone())?;
    v_P = py::mul(&v_f.clone(), &v_S.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::mul(&py::sub(&Py::Float(1.0), &v_f.clone())?, &v_S.clone())?), (Py::str("P"), v_P.clone()), (Py::str("reactor_feed"), py::add(&v_A_in.clone(), &v_I_in.clone())?), (Py::str("X_overall"), py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&v_f.clone(), &v_S_A.clone())?, &py::mul(&v_F0.clone(), &v_zA.clone())?)?)?), (Py::str("product"), py::mul(&v_X.clone(), &v_A_in.clone())?), (Py::str("y_inert"), py::truediv(&v_S_I.clone(), &v_S.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_recycle_reactor_1(mut v_v0: Py, mut v_V: Py, mut v_k: Py, mut v_R: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::str("v0, V and k"), Py::tuple(vec![v_v0.clone(), v_V.clone(), v_k.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_R.clone()]))?;
    let _ = f__need({ let t4866 = Py::Int(0i64); Py::Bool({ let t4867 = v_R.clone(); if py::le(&t4866, &t4867)? { { let t4868 = Py::Float(10000.0); if py::le(&t4867, &t4868)? { true } else { false } } } else { false } }) }, Py::str("the recycle ratio R must be zero or positive"))?;
    v_a = py::truediv(&py::truediv(&py::mul(&v_k.clone(), &v_V.clone())?, &v_v0.clone())?, &py::add(&v_R.clone(), &Py::Float(1.0))?)?;
    v_u = py::truediv(&Py::Float(1.0), &py::sub(&py::mul(&py::add(&v_R.clone(), &Py::Float(1.0))?, &py::math1("exp", &v_a.clone())?)?, &v_R.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("X"), py::sub(&Py::Float(1.0), &v_u.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reversible_batch(mut v_k1: Py, mut v_k2: Py, mut v_CA0: Py, mut v_CB0: Py, mut v_t: Py) -> R {
    let mut v_CA: Py = Py::None;
    let mut v_CAe: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let _ = f__pos(Py::str("k1 and k2"), Py::tuple(vec![v_k1.clone(), v_k2.clone()]))?;
    let _ = f__pos(Py::str("CA0"), Py::tuple(vec![v_CA0.clone()]))?;
    let _ = f__nonneg(Py::str("CB0 and t"), Py::tuple(vec![v_CB0.clone(), v_t.clone()]))?;
    v_tot = py::add(&v_CA0.clone(), &v_CB0.clone())?;
    v_CAe = py::truediv(&py::mul(&v_k2.clone(), &v_tot.clone())?, &py::add(&v_k1.clone(), &v_k2.clone())?)?;
    v_CA = py::add(&v_CAe.clone(), &py::mul(&py::sub(&v_CA0.clone(), &v_CAe.clone())?, &py::math1("exp", &py::mul(&py::neg(&py::add(&v_k1.clone(), &v_k2.clone())?)?, &v_t.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("CA"), v_CA.clone()), (Py::str("CB"), py::sub(&v_tot.clone(), &v_CA.clone())?), (Py::str("X"), py::sub(&Py::Float(1.0), &py::truediv(&v_CA.clone(), &v_CA0.clone())?)?), (Py::str("X_eq"), py::sub(&Py::Float(1.0), &py::truediv(&v_CAe.clone(), &v_CA0.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_riedel(mut v_Tb: Py, mut v_Tc: Py, mut v_Pc: Py) -> R {
    let mut v_Tbr: Py = Py::None;
    let _ = f__pos(Py::str("Tb, Tc and Pc"), Py::tuple(vec![v_Tb.clone(), v_Tc.clone(), v_Pc.clone()]))?;
    let _ = f__need({ let t4963 = v_Tb.clone(); Py::Bool({ let t4964 = py::mul(&Py::Float(0.93), &v_Tc.clone())?; if py::lt(&t4963, &t4964)? { true } else { false } }) }, Py::str("the normal boiling point must lie well below Tc"))?;
    v_Tbr = py::truediv(&v_Tb.clone(), &v_Tc.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Hvap_b"), py::truediv(&py::mul(&py::mul(&py::mul(&Py::Float(1.093), &v_Tb.clone())?, &c_R_GAS()?)?, &py::sub(&py::math1("log", &py::truediv(&v_Pc.clone(), &Py::Float(100000.0))?)?, &Py::Float(1.013))?)?, &py::sub(&Py::Float(0.93), &v_Tbr.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rtd_moments(mut v_t: Py, mut v_C: Py, mut v_t_unit: Py) -> R {
    let mut v_area: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_tm: Py = Py::None;
    let mut v_trap: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_var: Py = Py::None;
    let _ = f__need({ let t5007 = { let t5003 = py::len(&v_t.clone())?; Py::Bool({ let t5004 = py::len(&v_C.clone())?; if py::eq(&t5003, &t5004) { true } else { false } }) }; if !t5007.truthy() { t5007 } else { { let t5005 = py::len(&v_t.clone())?; Py::Bool({ let t5006 = Py::Int(3i64); if py::ge(&t5005, &t5006)? { true } else { false } }) } } }, Py::str("t and C need the same number of points (at least 3)"))?;
    v_f = py::getitem(&Py::dict(vec![(Py::str("s"), Py::Float(1.0)), (Py::str("min"), Py::Float(60.0)), (Py::str("h"), Py::Float(3600.0))]), &v_t_unit.clone())?;
    v_ts = { let mut out = vec![]; for t5008 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t5008;
out.push(py::mul(&v_x.clone(), &v_f.clone())?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5009 in py::iter(&(py::zip(&[v_ts.clone(), py::slice(&v_ts.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5010 = (t5009).items()?;
if t5010.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5010[0].clone();
v_b = t5010[1].clone();
out.push({ let t5011 = v_b.clone(); Py::Bool({ let t5012 = v_a.clone(); if py::gt(&t5011, &t5012)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("times must increase"))?;
    let _ = f__need({ let t5019 = Py::Bool(({ let mut out = vec![]; for t5013 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t5013;
out.push({ let t5014 = v_c.clone(); Py::Bool({ let t5015 = Py::Int(0i64); if py::ge(&t5014, &t5015)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t5019.truthy() { t5019 } else { Py::Bool(({ let mut out = vec![]; for t5016 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t5016;
out.push({ let t5017 = v_c.clone(); Py::Bool({ let t5018 = Py::Int(0i64); if py::gt(&t5017, &t5018)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }, Py::str("tracer concentrations must be zero or positive, not all zero"))?;
    v_trap = { let v_C = v_C.clone(); let v_f = v_f.clone(); let v_t = v_t.clone(); let v_t_unit = v_t_unit.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t5020 in py::range_iter(&[(py::sub(&py::len(&v_ts.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t5020;
out.push(py::truediv(&py::mul(&py::sub(&py::getitem(&v_ts.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_ts.clone(), &v_i.clone())?)?, &py::add(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &Py::Float(2.0))?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_area = py::call(&v_trap.clone(), &[v_C.clone()])?;
    v_tm = py::truediv(&py::call(&v_trap.clone(), &[{ let mut out = vec![]; for t5021 in py::iter(&(py::zip(&[v_ts.clone(), v_C.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5022 = (t5021).items()?;
if t5022.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5022[0].clone();
v_b = t5022[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }])?, &v_area.clone())?;
    v_var = py::truediv(&py::call(&v_trap.clone(), &[{ let mut out = vec![]; for t5023 in py::iter(&(py::zip(&[v_ts.clone(), v_C.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5024 = (t5023).items()?;
if t5024.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5024[0].clone();
v_b = t5024[1].clone();
out.push(py::mul(&py::pow(&py::sub(&v_a.clone(), &v_tm.clone())?, &Py::Int(2i64))?, &v_b.clone())?); } Py::list(out) }])?, &v_area.clone())?;
    let _ = f__need({ let t5025 = v_var.clone(); Py::Bool({ let t5026 = Py::Int(0i64); if py::gt(&t5025, &t5026)? { true } else { false } }) }, Py::str("the tracer response has no spread"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_mean"), v_tm.clone()), (Py::str("variance"), v_var.clone()), (Py::str("N_tanks"), py::truediv(&py::mul(&v_tm.clone(), &v_tm.clone())?, &v_var.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sensible_heat(mut v_n: Py, mut v_a: Py, mut v_b: Py, mut v_T1: Py, mut v_T2: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__pos(Py::str("the amount n"), Py::tuple(vec![v_n.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()]))?;
    let _ = f__pos(Py::str("the temperatures"), Py::tuple(vec![v_T1.clone(), v_T2.clone()]))?;
    let _ = f__need({ let t5074 = v_T1.clone(); Py::Bool({ let t5075 = v_T2.clone(); if py::ne(&t5074, &t5075) { true } else { false } }) }, Py::str("T1 and T2 must differ"))?;
    v_Q = py::mul(&v_n.clone(), &f__cp_int(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()]), v_T1.clone(), v_T2.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("cp_mean"), py::truediv(&v_Q.clone(), &py::mul(&v_n.clone(), &py::sub(&v_T2.clone(), &v_T1.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_max(mut v_k1: Py, mut v_k2: Py, mut v_CA0: Py) -> R {
    let mut v_CA: Py = Py::None;
    let mut v_CB: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("k1, k2 and CA0"), Py::tuple(vec![v_k1.clone(), v_k2.clone(), v_CA0.clone()]))?;
    if ({ let t5121 = py::abs(&py::sub(&v_k1.clone(), &v_k2.clone())?)?; Py::Bool({ let t5122 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_k1.clone(), v_k2.clone()], true)?)?; if py::le(&t5121, &t5122)? { true } else { false } }) }).truthy() {
        v_k = py::mul(&Py::Float(0.5), &py::add(&v_k1.clone(), &v_k2.clone())?)?;
        let t5123 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &v_k.clone())?, py::truediv(&v_CA0.clone(), &Py::Float(std::f64::consts::E))?])).items()?;
        if t5123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t = t5123[0].clone();
        v_CB = t5123[1].clone();
    } else {
        v_t = py::truediv(&py::math1("log", &py::truediv(&v_k2.clone(), &v_k1.clone())?)?, &py::sub(&v_k2.clone(), &v_k1.clone())?)?;
        v_CB = py::mul(&v_CA0.clone(), &py::pow(&py::truediv(&v_k1.clone(), &v_k2.clone())?, &py::truediv(&v_k2.clone(), &py::sub(&v_k2.clone(), &v_k1.clone())?)?)?)?;
    }
    v_CA = py::mul(&v_CA0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k1.clone())?, &v_t.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_max"), v_t.clone()), (Py::str("CB_max"), v_CB.clone()), (Py::str("CA"), v_CA.clone()), (Py::str("CC"), py::sub(&py::sub(&v_CA0.clone(), &v_CA.clone())?, &v_CB.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stefan_flux(mut v_D: Py, mut v_P: Py, mut v_T: Py, mut v_L: Py, mut v_yA1: Py, mut v_yA2: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::str("D, P, T and L"), Py::tuple(vec![v_D.clone(), v_P.clone(), v_T.clone(), v_L.clone()]))?;
    let _ = f__frac(Py::str("yA1"), v_yA1.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__frac(Py::str("yA2"), v_yA2.clone(), Py::Bool(false), Py::Bool(true))?;
    let _ = f__need({ let t5260 = v_yA1.clone(); Py::Bool({ let t5261 = v_yA2.clone(); if py::gt(&t5260, &t5261)? { true } else { false } }) }, Py::str("A diffuses from yA1 to yA2: yA1 must exceed yA2"))?;
    v_c = py::truediv(&v_P.clone(), &py::mul(&c_R_GAS()?, &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("NA"), py::mul(&py::truediv(&py::mul(&v_c.clone(), &v_D.clone())?, &v_L.clone())?, &py::math1("log", &py::truediv(&py::sub(&Py::Float(1.0), &v_yA2.clone())?, &py::sub(&Py::Float(1.0), &v_yA1.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_underwood_multi(mut v_alpha: Py, mut v_z: Py, mut v_xD: Py, mut v_q: Py, mut v_lk: Py, mut v_hk: Py) -> R {
    let mut v_Rm: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_a_hk: Py = Py::None;
    let mut v_a_lk: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_n = py::len(&v_alpha.clone())?;
    let _ = f__need({ let t3076 = { let t3068 = Py::Int(2i64); Py::Bool({ let t3069 = v_n.clone(); if py::le(&t3068, &t3069)? { { let t3070 = Py::Int(10i64); if py::le(&t3069, &t3070)? { true } else { false } } } else { false } }) }; if !t3076.truthy() { t3076 } else { { let t3075 = { let t3071 = py::len(&v_z.clone())?; Py::Bool({ let t3072 = v_n.clone(); if py::eq(&t3071, &t3072) { true } else { false } }) }; if !t3075.truthy() { t3075 } else { { let t3073 = py::len(&v_xD.clone())?; Py::Bool({ let t3074 = v_n.clone(); if py::eq(&t3073, &t3074) { true } else { false } }) } } } } }, Py::str("alpha, z and xD need one entry per component (2 to 10)"))?;
    let _ = f__need({ let t3088 = Py::Bool(matches!(v_lk.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3088.truthy() { t3088 } else { { let t3087 = Py::Bool(matches!(v_hk.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3087.truthy() { t3087 } else { { let t3086 = { let t3077 = Py::Int(1i64); Py::Bool({ let t3078 = v_lk.clone(); if py::le(&t3077, &t3078)? { { let t3079 = v_n.clone(); if py::le(&t3078, &t3079)? { true } else { false } } } else { false } }) }; if !t3086.truthy() { t3086 } else { { let t3085 = { let t3080 = Py::Int(1i64); Py::Bool({ let t3081 = v_hk.clone(); if py::le(&t3080, &t3081)? { { let t3082 = v_n.clone(); if py::le(&t3081, &t3082)? { true } else { false } } } else { false } }) }; if !t3085.truthy() { t3085 } else { { let t3083 = v_lk.clone(); Py::Bool({ let t3084 = v_hk.clone(); if py::ne(&t3083, &t3084) { true } else { false } }) } } } } } } } } }, Py::str("lk and hk are component numbers (1 to n)"))?;
    for t3089 in py::iter(&(v_alpha.clone()))? {
        v_a = t3089;
        let _ = f__need({ let t3092 = py::math1("isfinite", &v_a.clone())?; if !t3092.truthy() { t3092 } else { { let t3090 = v_a.clone(); Py::Bool({ let t3091 = Py::Int(0i64); if py::gt(&t3090, &t3091)? { true } else { false } }) } } }, Py::str("every relative volatility must be positive"))?;
    }
    let _ = f__fracs(Py::str("z"), v_z.clone(), Py::Float(1e-06))?;
    let _ = f__fracs(Py::str("xD"), v_xD.clone(), Py::Float(1e-06))?;
    let _ = f__need({ let t3100 = py::neg(&Py::Int(5i64))?; Py::Bool({ let t3101 = v_q.clone(); if py::le(&t3100, &t3101)? { { let t3102 = Py::Int(5i64); if py::le(&t3101, &t3102)? { true } else { false } } } else { false } }) }, Py::str("the feed quality q must lie between -5 and 5"))?;
    let t3103 = (Py::tuple(vec![py::getitem(&v_alpha.clone(), &py::sub(&v_lk.clone(), &Py::Int(1i64))?)?, py::getitem(&v_alpha.clone(), &py::sub(&v_hk.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3103.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a_lk = t3103[0].clone();
    v_a_hk = t3103[1].clone();
    let _ = f__need({ let t3104 = v_a_lk.clone(); Py::Bool({ let t3105 = v_a_hk.clone(); if py::gt(&t3104, &t3105)? { true } else { false } }) }, Py::str("the light key must be more volatile than the heavy key"))?;
    let _ = f__need(py::not(&Py::Bool(({ let mut out = vec![]; for t3106 in py::iter(&(v_alpha.clone()))? { let mut v_a: Py; 
v_a = t3106;
out.push({ let t3107 = v_a_hk.clone(); Py::Bool({ let t3108 = v_a.clone(); if py::lt(&t3107, &t3108)? { { let t3109 = v_a_lk.clone(); if py::lt(&t3108, &t3109)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))), Py::str("the keys must be adjacent in volatility (no component between them)"))?;
    let _ = f__need({ let t3110 = py::len(&py::set(&v_alpha.clone())?)?; Py::Bool({ let t3111 = v_n.clone(); if py::eq(&t3110, &t3111) { true } else { false } }) }, Py::str("relative volatilities must be distinct"))?;
    v_th = f__underwood_theta(v_alpha.clone(), v_z.clone(), v_q.clone(), v_a_hk.clone(), v_a_lk.clone())?;
    v_Rm = py::sub(&py::sum(&{ let mut out = vec![]; for t3123 in py::iter(&(py::zip(&[v_alpha.clone(), v_xD.clone()])?))? { let mut v_a: Py; let mut v_x: Py; 
let t3124 = (t3123).items()?;
if t3124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3124[0].clone();
v_x = t3124[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_x.clone())?, &py::sub(&v_a.clone(), &v_th.clone())?)?); } Py::list(out) }, None)?, &Py::Float(1.0))?;
    let _ = f__need({ let t3125 = v_Rm.clone(); Py::Bool({ let t3126 = Py::Int(0i64); if py::gt(&t3125, &t3126)? { true } else { false } }) }, Py::str("no positive minimum reflux for this distillate"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta"), v_th.clone()), (Py::str("Rmin"), v_Rm.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_uniquac_gamma(mut v_x: Py, mut v_r: Py, mut v_q: Py, mut v_tau: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_comb: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let mut v_sr: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__liq_x(v_x.clone())?;
    let _ = f__need({ let t5313 = py::len(&v_r.clone())?; Py::Bool({ let t5314 = py::len(&v_q.clone())?; if py::eq(&t5313, &t5314) { { let t5315 = v_n.clone(); if py::eq(&t5314, &t5315) { true } else { false } } } else { false } }) }, Py::str("give r and q for every component"))?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("r and q")); v.extend((v_r.clone()).items()?); v.extend((v_q.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_pos() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_pos() got an unexpected keyword argument".into())) } f__pos(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    let _ = f__square(Py::str("tau"), v_tau.clone(), v_n.clone(), Py::Float(1.0))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5333 in py::iter(&(v_tau.clone()))? { let mut v_row: Py; 
v_row = t5333;
for t5334 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t5334;
out.push({ let t5335 = v_v.clone(); Py::Bool({ let t5336 = Py::Int(0i64); if py::gt(&t5335, &t5336)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("UNIQUAC tau must be positive"))?;
    let t5341 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t5337 in py::iter(&(py::zip(&[v_r.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5338 = (t5337).items()?;
if t5338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5338[0].clone();
v_b = t5338[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t5339 in py::iter(&(py::zip(&[v_q.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5340 = (t5339).items()?;
if t5340.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5340[0].clone();
v_b = t5340[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?])).items()?;
    if t5341.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sr = t5341[0].clone();
    v_sq = t5341[1].clone();
    v_phi = { let mut out = vec![]; for t5342 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5342;
out.push(py::truediv(&py::mul(&py::getitem(&v_r.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_sr.clone())?); } Py::list(out) };
    v_th = { let mut out = vec![]; for t5343 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5343;
out.push(py::truediv(&py::mul(&py::getitem(&v_q.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_sq.clone())?); } Py::list(out) };
    v_l = { let mut out = vec![]; for t5344 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5344;
out.push(py::sub(&py::mul(&Py::Float(5.0), &py::sub(&py::getitem(&v_r.clone(), &v_i.clone())?, &py::getitem(&v_q.clone(), &v_i.clone())?)?)?, &py::sub(&py::getitem(&v_r.clone(), &v_i.clone())?, &Py::Float(1.0))?)?); } Py::list(out) };
    v_sl = py::sum(&{ let mut out = vec![]; for t5345 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5345;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_j.clone())?, &py::getitem(&v_l.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?;
    v_S = { let mut out = vec![]; for t5346 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5346;
out.push(py::sum(&{ let mut out = vec![]; for t5347 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5347;
out.push(py::mul(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_tau.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_lg = Py::list(vec![]);
    for t5348 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5348;
        v_comb = py::sub(&py::add(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_phi.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?)?, &py::mul(&py::mul(&Py::Float(5.0), &py::getitem(&v_q.clone(), &v_i.clone())?)?, &py::math1("log", &py::truediv(&py::getitem(&v_th.clone(), &v_i.clone())?, &py::getitem(&v_phi.clone(), &v_i.clone())?)?)?)?)?, &py::getitem(&v_l.clone(), &v_i.clone())?)?, &py::mul(&py::truediv(&py::getitem(&v_phi.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_sl.clone())?)?;
        v_res = py::mul(&py::getitem(&v_q.clone(), &v_i.clone())?, &py::sub(&py::sub(&Py::Float(1.0), &py::math1("log", &py::getitem(&v_S.clone(), &v_i.clone())?)?)?, &py::sum(&{ let mut out = vec![]; for t5349 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5349;
out.push(py::truediv(&py::mul(&py::getitem(&v_th.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_tau.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_S.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?)?)?;
        let _ = py::method(&v_lg.clone(), "append", &[py::add(&v_comb.clone(), &v_res.clone())?])?;
    }
    v_g = { let mut out = vec![]; for t5350 in py::iter(&(v_lg.clone()))? { let mut v_v: Py; 
v_v = t5350;
out.push(py::math1("exp", &v_v.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("gamma1"), py::getitem(&v_g.clone(), &Py::Int(0i64))?), (Py::str("gamma"), v_g.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_variable_volume_tank(mut v_V0: Py, mut v_q_in: Py, mut v_q_out: Py, mut v_C_in: Py, mut v_C0: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_V: Py = Py::None;
    let _ = f__pos(Py::str("the initial volume V0 and inflow q_in"), Py::tuple(vec![v_V0.clone(), v_q_in.clone()]))?;
    let _ = f__nonneg(Py::str("the outflow, concentrations and time"), Py::tuple(vec![v_q_out.clone(), v_C_in.clone(), v_C0.clone(), v_t.clone()]))?;
    v_V = py::add(&v_V0.clone(), &py::mul(&py::sub(&v_q_in.clone(), &v_q_out.clone())?, &v_t.clone())?)?;
    let _ = f__need({ let t5451 = v_V.clone(); Py::Bool({ let t5452 = Py::Int(0i64); if py::gt(&t5451, &t5452)? { true } else { false } }) }, Py::str("the tank would be empty before time t"))?;
    if ({ let t5453 = v_q_in.clone(); Py::Bool({ let t5454 = v_q_out.clone(); if py::eq(&t5453, &t5454) { true } else { false } }) }).truthy() {
        v_C = py::add(&v_C_in.clone(), &py::mul(&py::sub(&v_C0.clone(), &v_C_in.clone())?, &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_q_in.clone())?, &v_t.clone())?, &v_V0.clone())?)?)?)?;
    } else {
        v_C = py::sub(&v_C_in.clone(), &py::mul(&py::sub(&v_C_in.clone(), &v_C0.clone())?, &py::pow(&py::truediv(&v_V0.clone(), &v_V.clone())?, &py::truediv(&v_q_in.clone(), &py::sub(&v_q_in.clone(), &v_q_out.clone())?)?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("C"), v_C.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_watson(mut v_T: Py, mut v_T_ref: Py, mut v_Hvap_ref: Py, mut v_Tc: Py) -> R {
    let _ = f__pos(Py::str("T, T_ref, Hvap_ref and Tc"), Py::tuple(vec![v_T.clone(), v_T_ref.clone(), v_Hvap_ref.clone(), v_Tc.clone()]))?;
    let _ = f__need({ let t5552 = { let t5548 = v_T.clone(); Py::Bool({ let t5549 = v_Tc.clone(); if py::lt(&t5548, &t5549)? { true } else { false } }) }; if !t5552.truthy() { t5552 } else { { let t5550 = v_T_ref.clone(); Py::Bool({ let t5551 = v_Tc.clone(); if py::lt(&t5550, &t5551)? { true } else { false } }) } } }, Py::str("both temperatures must be below the critical temperature"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Hvap"), py::mul(&v_Hvap_ref.clone(), &py::pow(&py::truediv(&py::sub(&Py::Float(1.0), &py::truediv(&v_T.clone(), &v_Tc.clone())?)?, &py::sub(&Py::Float(1.0), &py::truediv(&v_T_ref.clone(), &v_Tc.clone())?)?)?, &Py::Float(0.38))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wilson_flash(mut v_z: Py, mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py, mut v_p_unit: Py, mut v_T: Py, mut v_P: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pcs: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_n = py::len(&v_z.clone())?;
    let _ = f__comp(Py::str("z"), v_z.clone())?;
    let _ = f__need({ let t5673 = py::len(&v_Tc.clone())?; Py::Bool({ let t5674 = py::len(&v_Pc.clone())?; if py::eq(&t5673, &t5674) { { let t5675 = py::len(&v_omega.clone())?; if py::eq(&t5674, &t5675) { { let t5676 = v_n.clone(); if py::eq(&t5675, &t5676) { true } else { false } } } else { false } } } else { false } }) }, Py::str("give Tc, Pc and omega for every component"))?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("Tc and Pc")); v.extend((v_Tc.clone()).items()?); v.extend((v_Pc.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_pos() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_pos() got an unexpected keyword argument".into())) } f__pos(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    let _ = f__pos(Py::str("T and P"), Py::tuple(vec![v_T.clone(), v_P.clone()]))?;
    v_pcs = { let mut out = vec![]; for t5683 in py::iter(&(v_Pc.clone()))? { let mut v_p: Py; 
v_p = t5683;
out.push(py::mul(&v_p.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?); } Py::list(out) };
    v_K = { let mut out = vec![]; for t5684 in py::iter(&(py::zip(&[v_Tc.clone(), v_pcs.clone(), v_omega.clone()])?))? { let mut v_pc: Py; let mut v_tc: Py; let mut v_w: Py; 
let t5685 = (t5684).items()?;
if t5685.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_tc = t5685[0].clone();
v_pc = t5685[1].clone();
v_w = t5685[2].clone();
out.push(py::mul(&py::truediv(&v_pc.clone(), &v_P.clone())?, &py::math1("exp", &py::mul(&py::mul(&Py::Float(5.37), &py::add(&Py::Float(1.0), &v_w.clone())?)?, &py::sub(&Py::Float(1.0), &py::truediv(&v_tc.clone(), &v_T.clone())?)?)?)?)?); } Py::list(out) };
    let t5736 = (f__rr(v_z.clone(), v_K.clone())?).items()?;
    if t5736.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t5736[0].clone();
    v_x = t5736[1].clone();
    v_y = t5736[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("VF"), v_V.clone()), (Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone()), (Py::str("K"), v_K.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wilson_gamma(mut v_x: Py, mut v_Lam: Py) -> R {
    let mut v_S: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let _ = f__liq_x(v_x.clone())?;
    let _ = f__square(Py::str("Lam"), v_Lam.clone(), v_n.clone(), Py::Float(1.0))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5615 in py::iter(&(v_Lam.clone()))? { let mut v_r: Py; 
v_r = t5615;
for t5616 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t5616;
out.push({ let t5617 = v_v.clone(); Py::Bool({ let t5618 = Py::Int(0i64); if py::gt(&t5617, &t5618)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("Wilson parameters must be positive"))?;
    v_S = { let mut out = vec![]; for t5619 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5619;
out.push(py::sum(&{ let mut out = vec![]; for t5620 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5620;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_Lam.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_lg = { let mut out = vec![]; for t5621 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5621;
out.push(py::sub(&py::sub(&Py::Float(1.0), &py::math1("log", &py::getitem(&v_S.clone(), &v_i.clone())?)?)?, &py::sum(&{ let mut out = vec![]; for t5622 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5622;
out.push(py::truediv(&py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_Lam.clone(), &v_k.clone())?, &v_i.clone())?)?, &py::getitem(&v_S.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?); } Py::list(out) };
    v_g = { let mut out = vec![]; for t5623 in py::iter(&(v_lg.clone()))? { let mut v_v: Py; 
v_v = t5623;
out.push(py::math1("exp", &v_v.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("gamma1"), py::getitem(&v_g.clone(), &Py::Int(0i64))?), (Py::str("gamma"), v_g.clone()), (Py::str("GE_RT"), py::neg(&py::sum(&{ let mut out = vec![]; for t5636 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5636;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::math1("log", &py::getitem(&v_S.clone(), &v_i.clone())?)?)?); } Py::list(out) }, None)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ziegler_nichols(mut v_K: Py, mut v_taus: Py, mut v_t_unit: Py, mut v_controller: Py) -> R {
    let mut v_ku: Py = Py::None;
    let mut v_pu: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_K.clone()]))?;
    let _ = f__need({ let t5778 = v_K.clone(); Py::Bool({ let t5779 = Py::Int(0i64); if py::gt(&t5778, &t5779)? { true } else { false } }) }, Py::str("the process gain must be positive"))?;
    let _ = f__need({ let t5780 = Py::Int(3i64); Py::Bool({ let t5781 = py::len(&v_taus.clone())?; if py::le(&t5780, &t5781)? { { let t5782 = Py::Int(6i64); if py::le(&t5781, &t5782)? { true } else { false } } } else { false } }) }, Py::str("give 3 to 6 time constants"))?;
    let _ = f__pos(Py::str("the time constants"), { let mut v = vec![]; v.extend((v_taus.clone()).items()?); Py::tuple(v) })?;
    let t5822 = (f__ultimate(v_K.clone(), { let mut out = vec![]; for t5821 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t5821;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) })?).items()?;
    if t5822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ku = t5822[0].clone();
    v_pu = t5822[1].clone();
    return Ok(f__zn(v_ku.clone(), v_pu.clone(), v_controller.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1043(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1045: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1044 = v_inputs.clone(); f_carman(py::getitem(&t1044, &Py::str("dp"))?, py::getitem(&t1044, &Py::str("eps"))?, py::getitem(&t1044, &Py::str("vs"))?, py::getitem(&t1044, &Py::str("rho"))?, py::getitem(&t1044, &Py::str("mu"))?, py::getitem(&t1044, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1045 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1046 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1068 = (t1046).items()?;
        if t1068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1068[0].clone();
        v_v = t1068[1].clone();
        if ({ let t1047 = v_k.clone(); Py::Bool({ let t1048 = v_outputs.clone(); if !py::contains(&t1048, &t1047)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1120(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1122: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1121 = v_inputs.clone(); f_closed_loop_stability(py::getitem(&t1121, &Py::str("num"))?, py::getitem(&t1121, &Py::str("den"))?, py::getitem(&t1121, &Py::str("Kc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1122 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1123 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1145 = (t1123).items()?;
        if t1145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1145[0].clone();
        v_v = t1145[1].clone();
        if ({ let t1124 = v_k.clone(); Py::Bool({ let t1125 = v_outputs.clone(); if !py::contains(&t1125, &t1124)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_117(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t119: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t118 = v_inputs.clone(); f_flame_temperature(py::getitem(&t118, &Py::str("fuel"))?, py::getitem(&t118, &Py::str("excess_air"))?, py::getitem(&t118, &Py::str("T_in"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t119 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t120 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t142 = (t120).items()?;
        if t142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t142[0].clone();
        v_v = t142[1].clone();
        if ({ let t121 = v_k.clone(); Py::Bool({ let t122 = v_outputs.clone(); if !py::contains(&t122, &t121)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1175(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1176 = v_inputs.clone(); f_combustion(py::getitem(&t1176, &Py::str("fuel"))?, py::getitem(&t1176, &Py::str("excess_air"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1177 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1178 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1200 = (t1178).items()?;
        if t1200.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1200[0].clone();
        v_v = t1200[1].clone();
        if ({ let t1179 = v_k.clone(); Py::Bool({ let t1180 = v_outputs.clone(); if !py::contains(&t1180, &t1179)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1233(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1235: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1234 = v_inputs.clone(); f_countercurrent_extraction(py::getitem(&t1234, &Py::str("F"))?, py::getitem(&t1234, &Py::str("S"))?, py::getitem(&t1234, &Py::str("K"))?, py::getitem(&t1234, &Py::str("N"))?, py::getitem(&t1234, &Py::str("XF"))?, py::getitem(&t1234, &Py::str("YS"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1235 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1236 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1258 = (t1236).items()?;
        if t1258.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1258[0].clone();
        v_v = t1258[1].clone();
        if ({ let t1237 = v_k.clone(); Py::Bool({ let t1238 = v_outputs.clone(); if !py::contains(&t1238, &t1237)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1281(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1283: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1282 = v_inputs.clone(); f_crosscurrent_extraction(py::getitem(&t1282, &Py::str("F"))?, py::getitem(&t1282, &Py::str("S"))?, py::getitem(&t1282, &Py::str("K"))?, py::getitem(&t1282, &Py::str("N"))?, py::getitem(&t1282, &Py::str("XF"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1283 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1284 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1306 = (t1284).items()?;
        if t1306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1306[0].clone();
        v_v = t1306[1].clone();
        if ({ let t1285 = v_k.clone(); Py::Bool({ let t1286 = v_outputs.clone(); if !py::contains(&t1286, &t1285)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1325(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1327: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1326 = v_inputs.clone(); f_cstr_conv_1(py::getitem(&t1326, &Py::str("v0"))?, py::getitem(&t1326, &Py::str("V"))?, py::getitem(&t1326, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1327 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1328 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1350 = (t1328).items()?;
        if t1350.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1350[0].clone();
        v_v = t1350[1].clone();
        if ({ let t1329 = v_k.clone(); Py::Bool({ let t1330 = v_outputs.clone(); if !py::contains(&t1330, &t1329)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1369(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1371: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1370 = v_inputs.clone(); f_cstr_conv_2(py::getitem(&t1370, &Py::str("v0"))?, py::getitem(&t1370, &Py::str("V"))?, py::getitem(&t1370, &Py::str("k"))?, py::getitem(&t1370, &Py::str("CA0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1371 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1372 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1394 = (t1372).items()?;
        if t1394.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1394[0].clone();
        v_v = t1394[1].clone();
        if ({ let t1373 = v_k.clone(); Py::Bool({ let t1374 = v_outputs.clone(); if !py::contains(&t1374, &t1373)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1425(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1427: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1426 = v_inputs.clone(); f_cstr_volume_1(py::getitem(&t1426, &Py::str("v0"))?, py::getitem(&t1426, &Py::str("k"))?, py::getitem(&t1426, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1427 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1428 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1450 = (t1428).items()?;
        if t1450.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1450[0].clone();
        v_v = t1450[1].clone();
        if ({ let t1429 = v_k.clone(); Py::Bool({ let t1430 = v_outputs.clone(); if !py::contains(&t1430, &t1429)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1481(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1483: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1482 = v_inputs.clone(); f_cstr_volume_2(py::getitem(&t1482, &Py::str("v0"))?, py::getitem(&t1482, &Py::str("k"))?, py::getitem(&t1482, &Py::str("CA0"))?, py::getitem(&t1482, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1483 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1484 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1506 = (t1484).items()?;
        if t1506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1506[0].clone();
        v_v = t1506[1].clone();
        if ({ let t1485 = v_k.clone(); Py::Bool({ let t1486 = v_outputs.clone(); if !py::contains(&t1486, &t1485)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1529(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1531: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1530 = v_inputs.clone(); f_cstrs_series_1(py::getitem(&t1530, &Py::str("v0"))?, py::getitem(&t1530, &Py::str("V"))?, py::getitem(&t1530, &Py::str("k"))?, py::getitem(&t1530, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1531 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1532 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1554 = (t1532).items()?;
        if t1554.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1554[0].clone();
        v_v = t1554[1].clone();
        if ({ let t1533 = v_k.clone(); Py::Bool({ let t1534 = v_outputs.clone(); if !py::contains(&t1534, &t1533)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1578(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1580: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1579 = v_inputs.clone(); f_cstrs_series_2(py::getitem(&t1579, &Py::str("v0"))?, py::getitem(&t1579, &Py::str("V"))?, py::getitem(&t1579, &Py::str("k"))?, py::getitem(&t1579, &Py::str("CA0"))?, py::getitem(&t1579, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1580 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1581 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1603 = (t1581).items()?;
        if t1603.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1603[0].clone();
        v_v = t1603[1].clone();
        if ({ let t1582 = v_k.clone(); Py::Bool({ let t1583 = v_outputs.clone(); if !py::contains(&t1583, &t1582)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1636(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1638: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1637 = v_inputs.clone(); f_equimolar_flux(py::getitem(&t1637, &Py::str("D"))?, py::getitem(&t1637, &Py::str("P"))?, py::getitem(&t1637, &Py::str("T"))?, py::getitem(&t1637, &Py::str("L"))?, py::getitem(&t1637, &Py::str("yA1"))?, py::getitem(&t1637, &Py::str("yA2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1638 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1639 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1661 = (t1639).items()?;
        if t1661.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1661[0].clone();
        v_v = t1661[1].clone();
        if ({ let t1640 = v_k.clone(); Py::Bool({ let t1641 = v_outputs.clone(); if !py::contains(&t1641, &t1640)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1692(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1694: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1693 = v_inputs.clone(); f_ergun(py::getitem(&t1693, &Py::str("dp"))?, py::getitem(&t1693, &Py::str("eps"))?, py::getitem(&t1693, &Py::str("vs"))?, py::getitem(&t1693, &Py::str("rho"))?, py::getitem(&t1693, &Py::str("mu"))?, py::getitem(&t1693, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1694 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1695 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1717 = (t1695).items()?;
        if t1717.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1717[0].clone();
        v_v = t1717[1].clone();
        if ({ let t1696 = v_k.clone(); Py::Bool({ let t1697 = v_outputs.clone(); if !py::contains(&t1697, &t1696)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1742(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1744: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1743 = v_inputs.clone(); f_decline_exp(py::getitem(&t1743, &Py::str("qi"))?, py::getitem(&t1743, &Py::str("Di"))?, py::getitem(&t1743, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1744 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1745 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1767 = (t1745).items()?;
        if t1767.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1767[0].clone();
        v_v = t1767[1].clone();
        if ({ let t1746 = v_k.clone(); Py::Bool({ let t1747 = v_outputs.clone(); if !py::contains(&t1747, &t1746)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1789(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1791: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1790 = v_inputs.clone(); f_fenske(py::getitem(&t1790, &Py::str("alpha"))?, py::getitem(&t1790, &Py::str("xD"))?, py::getitem(&t1790, &Py::str("xB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1791 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1792 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1814 = (t1792).items()?;
        if t1814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1814[0].clone();
        v_v = t1814[1].clone();
        if ({ let t1793 = v_k.clone(); Py::Bool({ let t1794 = v_outputs.clone(); if !py::contains(&t1794, &t1793)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1860: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1859 = v_inputs.clone(); f_filtration_fit(py::getitem(&t1859, &Py::str("t"))?, py::getitem(&t1859, &Py::str("V"))?, py::getitem(&t1859, &Py::str("t_unit"))?, py::getitem(&t1859, &Py::str("v_unit"))?, py::getitem(&t1859, &Py::str("A"))?, py::getitem(&t1859, &Py::str("dP"))?, py::getitem(&t1859, &Py::str("mu"))?, py::getitem(&t1859, &Py::str("c"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1860 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1861 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1883 = (t1861).items()?;
        if t1883.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1883[0].clone();
        v_v = t1883[1].clone();
        if ({ let t1862 = v_k.clone(); Py::Bool({ let t1863 = v_outputs.clone(); if !py::contains(&t1863, &t1862)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1971(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1973: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1972 = v_inputs.clone(); f_fit_first_order(py::getitem(&t1972, &Py::str("t"))?, py::getitem(&t1972, &Py::str("y"))?, py::getitem(&t1972, &Py::str("t_unit"))?, py::getitem(&t1972, &Py::str("M"))?, py::getitem(&t1972, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1973 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1974 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1996 = (t1974).items()?;
        if t1996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1996[0].clone();
        v_v = t1996[1].clone();
        if ({ let t1975 = v_k.clone(); Py::Bool({ let t1976 = v_outputs.clone(); if !py::contains(&t1976, &t1975)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2026(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2028: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2027 = v_inputs.clone(); f_fopdt_step(py::getitem(&t2027, &Py::str("K"))?, py::getitem(&t2027, &Py::str("tau"))?, py::getitem(&t2027, &Py::str("theta"))?, py::getitem(&t2027, &Py::str("M"))?, py::getitem(&t2027, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2028 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2029 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2051 = (t2029).items()?;
        if t2051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2051[0].clone();
        v_v = t2051[1].clone();
        if ({ let t2030 = v_k.clone(); Py::Bool({ let t2031 = v_outputs.clone(); if !py::contains(&t2031, &t2030)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2094(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2096: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2095 = v_inputs.clone(); f_gas_pz(py::getitem(&t2095, &Py::str("p"))?, py::getitem(&t2095, &Py::str("z"))?, py::getitem(&t2095, &Py::str("Gp"))?, py::getitem(&t2095, &Py::str("p_unit"))?, py::getitem(&t2095, &Py::str("gp_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2096 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2097 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2119 = (t2097).items()?;
        if t2119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2119[0].clone();
        v_v = t2119[1].clone();
        if ({ let t2098 = v_k.clone(); Py::Bool({ let t2099 = v_outputs.clone(); if !py::contains(&t2099, &t2098)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2167(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2169: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2168 = v_inputs.clone(); f_backpressure(py::getitem(&t2168, &Py::str("q"))?, py::getitem(&t2168, &Py::str("pwf"))?, py::getitem(&t2168, &Py::str("q_unit"))?, py::getitem(&t2168, &Py::str("p_unit"))?, py::getitem(&t2168, &Py::str("pr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2169 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2170 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2192 = (t2170).items()?;
        if t2192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2192[0].clone();
        v_v = t2192[1].clone();
        if ({ let t2171 = v_k.clone(); Py::Bool({ let t2172 = v_outputs.clone(); if !py::contains(&t2172, &t2171)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2217(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2219: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2218 = v_inputs.clone(); f_decline_harm(py::getitem(&t2218, &Py::str("qi"))?, py::getitem(&t2218, &Py::str("Di"))?, py::getitem(&t2218, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2219 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2220 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2242 = (t2220).items()?;
        if t2242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2242[0].clone();
        v_v = t2242[1].clone();
        if ({ let t2221 = v_k.clone(); Py::Bool({ let t2222 = v_outputs.clone(); if !py::contains(&t2222, &t2221)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2283(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2285: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2284 = v_inputs.clone(); f_ho_gascap(py::getitem(&t2284, &Py::str("F"))?, py::getitem(&t2284, &Py::str("Eo"))?, py::getitem(&t2284, &Py::str("Eg"))?, py::getitem(&t2284, &Py::str("v_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2285 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2286 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2308 = (t2286).items()?;
        if t2308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2308[0].clone();
        v_v = t2308[1].clone();
        if ({ let t2287 = v_k.clone(); Py::Bool({ let t2288 = v_outputs.clone(); if !py::contains(&t2288, &t2287)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2341(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2343: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2342 = v_inputs.clone(); f_ho_oil(py::getitem(&t2342, &Py::str("F"))?, py::getitem(&t2342, &Py::str("Et"))?, py::getitem(&t2342, &Py::str("v_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2343 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2344 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2366 = (t2344).items()?;
        if t2366.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2366[0].clone();
        v_v = t2366[1].clone();
        if ({ let t2345 = v_k.clone(); Py::Bool({ let t2346 = v_outputs.clone(); if !py::contains(&t2346, &t2345)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_235(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t237: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t236 = v_inputs.clone(); f_adiabatic_reactor_T(py::getitem(&t236, &Py::str("n0"))?, py::getitem(&t236, &Py::str("nu"))?, py::getitem(&t236, &Py::str("cp"))?, py::getitem(&t236, &Py::str("dH"))?, py::getitem(&t236, &Py::str("T_ref"))?, py::getitem(&t236, &Py::str("T_in"))?, py::getitem(&t236, &Py::str("key"))?, py::getitem(&t236, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t237 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t238 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t260 = (t238).items()?;
        if t260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t260[0].clone();
        v_v = t260[1].clone();
        if ({ let t239 = v_k.clone(); Py::Bool({ let t240 = v_outputs.clone(); if !py::contains(&t240, &t239)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2408(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2410: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2409 = v_inputs.clone(); f_horner(py::getitem(&t2409, &Py::str("dt"))?, py::getitem(&t2409, &Py::str("pws"))?, py::getitem(&t2409, &Py::str("t_unit"))?, py::getitem(&t2409, &Py::str("p_unit"))?, py::getitem(&t2409, &Py::str("tp"))?, py::getitem(&t2409, &Py::str("q"))?, py::getitem(&t2409, &Py::str("mu"))?, py::getitem(&t2409, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2410 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2411 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2433 = (t2411).items()?;
        if t2433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2433[0].clone();
        v_v = t2433[1].clone();
        if ({ let t2412 = v_k.clone(); Py::Bool({ let t2413 = v_outputs.clone(); if !py::contains(&t2413, &t2412)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2464(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2466: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2465 = v_inputs.clone(); f_decline_hyp(py::getitem(&t2465, &Py::str("qi"))?, py::getitem(&t2465, &Py::str("Di"))?, py::getitem(&t2465, &Py::str("b"))?, py::getitem(&t2465, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2466 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2467 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2489 = (t2467).items()?;
        if t2489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2489[0].clone();
        v_v = t2489[1].clone();
        if ({ let t2468 = v_k.clone(); Py::Bool({ let t2469 = v_outputs.clone(); if !py::contains(&t2469, &t2468)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2522(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2524: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2523 = v_inputs.clone(); f_kremser_absorber(py::getitem(&t2523, &Py::str("N"))?, py::getitem(&t2523, &Py::str("A"))?, py::getitem(&t2523, &Py::str("m"))?, py::getitem(&t2523, &Py::str("y_in"))?, py::getitem(&t2523, &Py::str("x_in"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2524 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2525 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2547 = (t2525).items()?;
        if t2547.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2547[0].clone();
        v_v = t2547[1].clone();
        if ({ let t2526 = v_k.clone(); Py::Bool({ let t2527 = v_outputs.clone(); if !py::contains(&t2527, &t2526)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2580(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2582: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2581 = v_inputs.clone(); f_kremser_stripper(py::getitem(&t2581, &Py::str("N"))?, py::getitem(&t2581, &Py::str("S"))?, py::getitem(&t2581, &Py::str("m"))?, py::getitem(&t2581, &Py::str("x_in"))?, py::getitem(&t2581, &Py::str("y_in"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2582 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2583 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2605 = (t2583).items()?;
        if t2605.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2605[0].clone();
        v_v = t2605[1].clone();
        if ({ let t2584 = v_k.clone(); Py::Bool({ let t2585 = v_outputs.clone(); if !py::contains(&t2585, &t2584)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2635(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2637: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2636 = v_inputs.clone(); f_lag_freq(py::getitem(&t2636, &Py::str("K"))?, py::getitem(&t2636, &Py::str("taus"))?, py::getitem(&t2636, &Py::str("t_unit"))?, py::getitem(&t2636, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2637 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2638 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2660 = (t2638).items()?;
        if t2660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2660[0].clone();
        v_v = t2660[1].clone();
        if ({ let t2639 = v_k.clone(); Py::Bool({ let t2640 = v_outputs.clone(); if !py::contains(&t2640, &t2639)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2684(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2686: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2685 = v_inputs.clone(); f_lee_kesler(py::getitem(&t2685, &Py::str("T"))?, py::getitem(&t2685, &Py::str("Tc"))?, py::getitem(&t2685, &Py::str("Pc"))?, py::getitem(&t2685, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2686 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2687 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2709 = (t2687).items()?;
        if t2709.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2709[0].clone();
        v_v = t2709[1].clone();
        if ({ let t2688 = v_k.clone(); Py::Bool({ let t2689 = v_outputs.clone(); if !py::contains(&t2689, &t2688)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2756(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2758: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2757 = v_inputs.clone(); f_line_source(py::getitem(&t2757, &Py::str("q"))?, py::getitem(&t2757, &Py::str("mu"))?, py::getitem(&t2757, &Py::str("k"))?, py::getitem(&t2757, &Py::str("h"))?, py::getitem(&t2757, &Py::str("phi"))?, py::getitem(&t2757, &Py::str("ct"))?, py::getitem(&t2757, &Py::str("r"))?, py::getitem(&t2757, &Py::str("t"))?, py::getitem(&t2757, &Py::str("pi"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2758 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2759 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2781 = (t2759).items()?;
        if t2781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2781[0].clone();
        v_v = t2781[1].clone();
        if ({ let t2760 = v_k.clone(); Py::Bool({ let t2761 = v_outputs.clone(); if !py::contains(&t2761, &t2760)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2817(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2819: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2818 = v_inputs.clone(); f_mass_to_mole(py::getitem(&t2818, &Py::str("w"))?, py::getitem(&t2818, &Py::str("MW"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2819 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2820 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2842 = (t2820).items()?;
        if t2842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2842[0].clone();
        v_v = t2842[1].clone();
        if ({ let t2821 = v_k.clone(); Py::Bool({ let t2822 = v_outputs.clone(); if !py::contains(&t2822, &t2821)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2890(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2892: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2891 = v_inputs.clone(); f_mass_transfer_coefficient(py::getitem(&t2891, &Py::str("geometry"))?, py::getitem(&t2891, &Py::str("u"))?, py::getitem(&t2891, &Py::str("L"))?, py::getitem(&t2891, &Py::str("D"))?, py::getitem(&t2891, &Py::str("rho"))?, py::getitem(&t2891, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2892 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2893 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2915 = (t2893).items()?;
        if t2915.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2915[0].clone();
        v_v = t2915[1].clone();
        if ({ let t2894 = v_k.clone(); Py::Bool({ let t2895 = v_outputs.clone(); if !py::contains(&t2895, &t2894)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_296(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t298: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t297 = v_inputs.clone(); f_antoine_T(py::getitem(&t297, &Py::str("A"))?, py::getitem(&t297, &Py::str("B"))?, py::getitem(&t297, &Py::str("C"))?, py::getitem(&t297, &Py::str("units"))?, py::getitem(&t297, &Py::str("base"))?, py::getitem(&t297, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t298 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t299 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t321 = (t299).items()?;
        if t321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t321[0].clone();
        v_v = t321[1].clone();
        if ({ let t300 = v_k.clone(); Py::Bool({ let t301 = v_outputs.clone(); if !py::contains(&t301, &t300)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2976(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2978: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2977 = v_inputs.clone(); f_mccabe_thiele(py::getitem(&t2977, &Py::str("alpha"))?, py::getitem(&t2977, &Py::str("zF"))?, py::getitem(&t2977, &Py::str("xD"))?, py::getitem(&t2977, &Py::str("xB"))?, py::getitem(&t2977, &Py::str("q"))?, py::getitem(&t2977, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2978 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2979 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3001 = (t2979).items()?;
        if t3001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3001[0].clone();
        v_v = t3001[1].clone();
        if ({ let t2980 = v_k.clone(); Py::Bool({ let t2981 = v_outputs.clone(); if !py::contains(&t2981, &t2980)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3042(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3044: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3043 = v_inputs.clone(); f_modified_raoult_bubble(py::getitem(&t3043, &Py::str("x"))?, py::getitem(&t3043, &Py::str("gamma"))?, py::getitem(&t3043, &Py::str("Psat"))?, py::getitem(&t3043, &Py::str("p_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3044 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3045 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3067 = (t3045).items()?;
        if t3067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3067[0].clone();
        v_v = t3067[1].clone();
        if ({ let t3046 = v_k.clone(); Py::Bool({ let t3047 = v_outputs.clone(); if !py::contains(&t3047, &t3046)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3139(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3141: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3140 = v_inputs.clone(); f_underwood_multi(py::getitem(&t3140, &Py::str("alpha"))?, py::getitem(&t3140, &Py::str("z"))?, py::getitem(&t3140, &Py::str("xD"))?, py::getitem(&t3140, &Py::str("q"))?, py::getitem(&t3140, &Py::str("lk"))?, py::getitem(&t3140, &Py::str("hk"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3141 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3142 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3164 = (t3142).items()?;
        if t3164.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3164[0].clone();
        v_v = t3164[1].clone();
        if ({ let t3143 = v_k.clone(); Py::Bool({ let t3144 = v_outputs.clone(); if !py::contains(&t3144, &t3143)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3195(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3197: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3196 = v_inputs.clone(); f_npsh_available(py::getitem(&t3196, &Py::str("p_s"))?, py::getitem(&t3196, &Py::str("p_v"))?, py::getitem(&t3196, &Py::str("rho"))?, py::getitem(&t3196, &Py::str("z_s"))?, py::getitem(&t3196, &Py::str("h_f"))?, py::method(&t3196, "get", &[Py::str("NPSHr"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3197 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3198 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3220 = (t3198).items()?;
        if t3220.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3220[0].clone();
        v_v = t3220[1].clone();
        if ({ let t3199 = v_k.clone(); Py::Bool({ let t3200 = v_outputs.clone(); if !py::contains(&t3200, &t3199)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_32(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t34: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t33 = v_inputs.clone(); f_absorber_nog(py::getitem(&t33, &Py::str("y1"))?, py::getitem(&t33, &Py::str("y2"))?, py::getitem(&t33, &Py::str("x2"))?, py::getitem(&t33, &Py::str("m"))?, py::getitem(&t33, &Py::str("A"))?, py::getitem(&t33, &Py::str("HOG"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t34 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t35 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t57 = (t35).items()?;
        if t57.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t57[0].clone();
        v_v = t57[1].clone();
        if ({ let t36 = v_k.clone(); Py::Bool({ let t37 = v_outputs.clone(); if !py::contains(&t37, &t36)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3267(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3269: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3268 = v_inputs.clone(); f_nrtl_gamma(py::getitem(&t3268, &Py::str("x"))?, py::getitem(&t3268, &Py::str("tau"))?, py::getitem(&t3268, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3269 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3270 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3292 = (t3270).items()?;
        if t3292.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3292[0].clone();
        v_v = t3292[1].clone();
        if ({ let t3271 = v_k.clone(); Py::Bool({ let t3272 = v_outputs.clone(); if !py::contains(&t3272, &t3271)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3346(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3348: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3347 = v_inputs.clone(); f_p_offset(py::getitem(&t3347, &Py::str("Kc"))?, py::getitem(&t3347, &Py::str("K"))?, py::getitem(&t3347, &Py::str("taus"))?, py::getitem(&t3347, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3348 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3349 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3371 = (t3349).items()?;
        if t3371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3371[0].clone();
        v_v = t3371[1].clone();
        if ({ let t3350 = v_k.clone(); Py::Bool({ let t3351 = v_outputs.clone(); if !py::contains(&t3351, &t3350)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3372(mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py, mut v_T: Py, mut v_P: Py) -> R {
    return Ok(f__eos(Py::str("PR"), v_Tc.clone(), v_Pc.clone(), v_omega.clone(), v_T.clone(), v_P.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3428: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3427 = v_inputs.clone(); s_3372(py::getitem(&t3427, &Py::str("Tc"))?, py::getitem(&t3427, &Py::str("Pc"))?, py::getitem(&t3427, &Py::str("omega"))?, py::getitem(&t3427, &Py::str("T"))?, py::getitem(&t3427, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3428 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3429 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3451 = (t3429).items()?;
        if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3451[0].clone();
        v_v = t3451[1].clone();
        if ({ let t3430 = v_k.clone(); Py::Bool({ let t3431 = v_outputs.clone(); if !py::contains(&t3431, &t3430)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3470(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3472: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3471 = v_inputs.clone(); f_pfr_conv_1(py::getitem(&t3471, &Py::str("v0"))?, py::getitem(&t3471, &Py::str("V"))?, py::getitem(&t3471, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3472 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3473 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3495 = (t3473).items()?;
        if t3495.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3495[0].clone();
        v_v = t3495[1].clone();
        if ({ let t3474 = v_k.clone(); Py::Bool({ let t3475 = v_outputs.clone(); if !py::contains(&t3475, &t3474)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3514(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3516: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3515 = v_inputs.clone(); f_pfr_conv_2(py::getitem(&t3515, &Py::str("v0"))?, py::getitem(&t3515, &Py::str("V"))?, py::getitem(&t3515, &Py::str("k"))?, py::getitem(&t3515, &Py::str("CA0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3516 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3517 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3539 = (t3517).items()?;
        if t3539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3539[0].clone();
        v_v = t3539[1].clone();
        if ({ let t3518 = v_k.clone(); Py::Bool({ let t3519 = v_outputs.clone(); if !py::contains(&t3519, &t3518)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_357(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t359: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t358 = v_inputs.clone(); f_antoine_P(py::getitem(&t358, &Py::str("A"))?, py::getitem(&t358, &Py::str("B"))?, py::getitem(&t358, &Py::str("C"))?, py::getitem(&t358, &Py::str("units"))?, py::getitem(&t358, &Py::str("base"))?, py::getitem(&t358, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t359 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t360 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t382 = (t360).items()?;
        if t382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t382[0].clone();
        v_v = t382[1].clone();
        if ({ let t361 = v_k.clone(); Py::Bool({ let t362 = v_outputs.clone(); if !py::contains(&t362, &t361)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3576(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3578: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3577 = v_inputs.clone(); f_pfr_gas_1(py::getitem(&t3577, &Py::str("FA0"))?, py::getitem(&t3577, &Py::str("CA0"))?, py::getitem(&t3577, &Py::str("k"))?, py::getitem(&t3577, &Py::str("eps"))?, py::getitem(&t3577, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3578 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3579 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3601 = (t3579).items()?;
        if t3601.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3601[0].clone();
        v_v = t3601[1].clone();
        if ({ let t3580 = v_k.clone(); Py::Bool({ let t3581 = v_outputs.clone(); if !py::contains(&t3581, &t3580)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3638(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3640: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3639 = v_inputs.clone(); f_pfr_gas_2(py::getitem(&t3639, &Py::str("FA0"))?, py::getitem(&t3639, &Py::str("CA0"))?, py::getitem(&t3639, &Py::str("k"))?, py::getitem(&t3639, &Py::str("eps"))?, py::getitem(&t3639, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3640 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3641 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3663 = (t3641).items()?;
        if t3663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3663[0].clone();
        v_v = t3663[1].clone();
        if ({ let t3642 = v_k.clone(); Py::Bool({ let t3643 = v_outputs.clone(); if !py::contains(&t3643, &t3642)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3694(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3696: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3695 = v_inputs.clone(); f_pfr_volume_1(py::getitem(&t3695, &Py::str("v0"))?, py::getitem(&t3695, &Py::str("k"))?, py::getitem(&t3695, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3696 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3697 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3719 = (t3697).items()?;
        if t3719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3719[0].clone();
        v_v = t3719[1].clone();
        if ({ let t3698 = v_k.clone(); Py::Bool({ let t3699 = v_outputs.clone(); if !py::contains(&t3699, &t3698)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3750(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3752: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3751 = v_inputs.clone(); f_pfr_volume_2(py::getitem(&t3751, &Py::str("v0"))?, py::getitem(&t3751, &Py::str("k"))?, py::getitem(&t3751, &Py::str("CA0"))?, py::getitem(&t3751, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3752 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3753 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3775 = (t3753).items()?;
        if t3775.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3775[0].clone();
        v_v = t3775[1].clone();
        if ({ let t3754 = v_k.clone(); Py::Bool({ let t3755 = v_outputs.clone(); if !py::contains(&t3755, &t3754)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3839(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3841: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3840 = v_inputs.clone(); f_margins_pi(py::getitem(&t3840, &Py::str("Kc"))?, py::getitem(&t3840, &Py::str("tauI"))?, py::getitem(&t3840, &Py::str("K"))?, py::getitem(&t3840, &Py::str("taus"))?, py::getitem(&t3840, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3841 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3842 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3864 = (t3842).items()?;
        if t3864.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3864[0].clone();
        v_v = t3864[1].clone();
        if ({ let t3843 = v_k.clone(); Py::Bool({ let t3844 = v_outputs.clone(); if !py::contains(&t3844, &t3843)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3897(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3899: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3898 = v_inputs.clone(); f_pid_position(py::getitem(&t3898, &Py::str("Kc"))?, py::getitem(&t3898, &Py::str("tauI"))?, py::getitem(&t3898, &Py::str("tauD"))?, py::getitem(&t3898, &Py::str("dt"))?, py::getitem(&t3898, &Py::str("e"))?, py::getitem(&t3898, &Py::str("bias"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3899 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3900 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3922 = (t3900).items()?;
        if t3922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3922[0].clone();
        v_v = t3922[1].clone();
        if ({ let t3901 = v_k.clone(); Py::Bool({ let t3902 = v_outputs.clone(); if !py::contains(&t3902, &t3901)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3946(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3947 = v_inputs.clone(); f_pitzer_hvap(py::getitem(&t3947, &Py::str("T"))?, py::getitem(&t3947, &Py::str("Tc"))?, py::getitem(&t3947, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3949 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3971 = (t3949).items()?;
        if t3971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3971[0].clone();
        v_v = t3971[1].clone();
        if ({ let t3950 = v_k.clone(); Py::Bool({ let t3951 = v_outputs.clone(); if !py::contains(&t3951, &t3950)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4015(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4017: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4016 = v_inputs.clone(); f_pump_head(py::getitem(&t4016, &Py::str("p1"))?, py::getitem(&t4016, &Py::str("p2"))?, py::getitem(&t4016, &Py::str("rho"))?, py::method(&t4016, "get", &[Py::str("dz"), Py::Float(0.0)])?, py::method(&t4016, "get", &[Py::str("v1"), Py::Float(0.0)])?, py::method(&t4016, "get", &[Py::str("v2"), Py::Float(0.0)])?, py::method(&t4016, "get", &[Py::str("Q"), Py::None])?, py::method(&t4016, "get", &[Py::str("eta"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4017 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4018 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4040 = (t4018).items()?;
        if t4040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4040[0].clone();
        v_v = t4040[1].clone();
        if ({ let t4019 = v_k.clone(); Py::Bool({ let t4020 = v_outputs.clone(); if !py::contains(&t4020, &t4019)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_407(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t409: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t408 = v_inputs.clone(); f_batch_coil_cooling(py::getitem(&t408, &Py::str("M"))?, py::getitem(&t408, &Py::str("cp"))?, py::getitem(&t408, &Py::str("W"))?, py::getitem(&t408, &Py::str("cp_c"))?, py::getitem(&t408, &Py::str("t_in"))?, py::getitem(&t408, &Py::str("UA"))?, py::getitem(&t408, &Py::str("T0"))?, py::getitem(&t408, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t409 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t410 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t432 = (t410).items()?;
        if t432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t432[0].clone();
        v_v = t432[1].clone();
        if ({ let t411 = v_k.clone(); Py::Bool({ let t412 = v_outputs.clone(); if !py::contains(&t412, &t411)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4120(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4122: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4121 = v_inputs.clone(); f_rachford_rice(py::getitem(&t4121, &Py::str("z"))?, py::getitem(&t4121, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4122 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4123 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4145 = (t4123).items()?;
        if t4145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4145[0].clone();
        v_v = t4145[1].clone();
        if ({ let t4124 = v_k.clone(); Py::Bool({ let t4125 = v_outputs.clone(); if !py::contains(&t4125, &t4124)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4169(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4171: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4170 = v_inputs.clone(); f_rackett(py::getitem(&t4170, &Py::str("T"))?, py::getitem(&t4170, &Py::str("Tc"))?, py::getitem(&t4170, &Py::str("Pc"))?, py::getitem(&t4170, &Py::str("Zc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4171 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4172 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4194 = (t4172).items()?;
        if t4194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4194[0].clone();
        v_v = t4194[1].clone();
        if ({ let t4173 = v_k.clone(); Py::Bool({ let t4174 = v_outputs.clone(); if !py::contains(&t4174, &t4173)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4222(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4224: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4223 = v_inputs.clone(); f_radial_flow(py::getitem(&t4223, &Py::str("k"))?, py::getitem(&t4223, &Py::str("h"))?, py::getitem(&t4223, &Py::str("mu"))?, py::getitem(&t4223, &Py::str("pe"))?, py::getitem(&t4223, &Py::str("pw"))?, py::getitem(&t4223, &Py::str("re"))?, py::getitem(&t4223, &Py::str("rw"))?, py::method(&t4223, "get", &[Py::str("S"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4224 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4225 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4247 = (t4225).items()?;
        if t4247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4247[0].clone();
        v_v = t4247[1].clone();
        if ({ let t4226 = v_k.clone(); Py::Bool({ let t4227 = v_outputs.clone(); if !py::contains(&t4227, &t4226)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4298(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4300: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4299 = v_inputs.clone(); f_bubble_P(py::getitem(&t4299, &Py::str("x"))?, py::getitem(&t4299, &Py::str("ant"))?, py::getitem(&t4299, &Py::str("units"))?, py::getitem(&t4299, &Py::str("base"))?, py::getitem(&t4299, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4300 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4301 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4323 = (t4301).items()?;
        if t4323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4323[0].clone();
        v_v = t4323[1].clone();
        if ({ let t4302 = v_k.clone(); Py::Bool({ let t4303 = v_outputs.clone(); if !py::contains(&t4303, &t4302)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4408(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4410: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4409 = v_inputs.clone(); f_bubble_T(py::getitem(&t4409, &Py::str("x"))?, py::getitem(&t4409, &Py::str("ant"))?, py::getitem(&t4409, &Py::str("units"))?, py::getitem(&t4409, &Py::str("base"))?, py::getitem(&t4409, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4410 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4411 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4433 = (t4411).items()?;
        if t4433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4433[0].clone();
        v_v = t4433[1].clone();
        if ({ let t4412 = v_k.clone(); Py::Bool({ let t4413 = v_outputs.clone(); if !py::contains(&t4413, &t4412)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4484(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4486: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4485 = v_inputs.clone(); f_dew_P(py::getitem(&t4485, &Py::str("y"))?, py::getitem(&t4485, &Py::str("ant"))?, py::getitem(&t4485, &Py::str("units"))?, py::getitem(&t4485, &Py::str("base"))?, py::getitem(&t4485, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4486 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4487 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4509 = (t4487).items()?;
        if t4509.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4509[0].clone();
        v_v = t4509[1].clone();
        if ({ let t4488 = v_k.clone(); Py::Bool({ let t4489 = v_outputs.clone(); if !py::contains(&t4489, &t4488)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_457(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t459: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t458 = v_inputs.clone(); f_batch_conv_1(py::getitem(&t458, &Py::str("k"))?, py::getitem(&t458, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t459 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t460 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t482 = (t460).items()?;
        if t482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t482[0].clone();
        v_v = t482[1].clone();
        if ({ let t461 = v_k.clone(); Py::Bool({ let t462 = v_outputs.clone(); if !py::contains(&t462, &t461)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4596: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4595 = v_inputs.clone(); f_dew_T(py::getitem(&t4595, &Py::str("y"))?, py::getitem(&t4595, &Py::str("ant"))?, py::getitem(&t4595, &Py::str("units"))?, py::getitem(&t4595, &Py::str("base"))?, py::getitem(&t4595, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4596 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4597 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4619 = (t4597).items()?;
        if t4619.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4619[0].clone();
        v_v = t4619[1].clone();
        if ({ let t4598 = v_k.clone(); Py::Bool({ let t4599 = v_outputs.clone(); if !py::contains(&t4599, &t4598)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4719: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4718 = v_inputs.clone(); f_raoult_flash(py::getitem(&t4718, &Py::str("z"))?, py::getitem(&t4718, &Py::str("ant"))?, py::getitem(&t4718, &Py::str("units"))?, py::getitem(&t4718, &Py::str("base"))?, py::getitem(&t4718, &Py::str("T"))?, py::getitem(&t4718, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4719 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4720 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4742 = (t4720).items()?;
        if t4742.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4742[0].clone();
        v_v = t4742[1].clone();
        if ({ let t4721 = v_k.clone(); Py::Bool({ let t4722 = v_outputs.clone(); if !py::contains(&t4722, &t4721)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4774(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4776: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4775 = v_inputs.clone(); f_reaction_enthalpy_T(py::getitem(&t4775, &Py::str("nu"))?, py::getitem(&t4775, &Py::str("cp"))?, py::getitem(&t4775, &Py::str("dH0"))?, py::getitem(&t4775, &Py::str("T0"))?, py::getitem(&t4775, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4776 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4777 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4799 = (t4777).items()?;
        if t4799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4799[0].clone();
        v_v = t4799[1].clone();
        if ({ let t4778 = v_k.clone(); Py::Bool({ let t4779 = v_outputs.clone(); if !py::contains(&t4779, &t4778)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4831(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4833: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4832 = v_inputs.clone(); f_recycle_purge(py::getitem(&t4832, &Py::str("F0"))?, py::getitem(&t4832, &Py::str("zA"))?, py::getitem(&t4832, &Py::str("X"))?, py::getitem(&t4832, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4833 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4834 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4856 = (t4834).items()?;
        if t4856.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4856[0].clone();
        v_v = t4856[1].clone();
        if ({ let t4835 = v_k.clone(); Py::Bool({ let t4836 = v_outputs.clone(); if !py::contains(&t4836, &t4835)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4881(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4883: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4882 = v_inputs.clone(); f_recycle_reactor_1(py::getitem(&t4882, &Py::str("v0"))?, py::getitem(&t4882, &Py::str("V"))?, py::getitem(&t4882, &Py::str("k"))?, py::getitem(&t4882, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4883 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4884 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4906 = (t4884).items()?;
        if t4906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4906[0].clone();
        v_v = t4906[1].clone();
        if ({ let t4885 = v_k.clone(); Py::Bool({ let t4886 = v_outputs.clone(); if !py::contains(&t4886, &t4885)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4931(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4933: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4932 = v_inputs.clone(); f_reversible_batch(py::getitem(&t4932, &Py::str("k1"))?, py::getitem(&t4932, &Py::str("k2"))?, py::getitem(&t4932, &Py::str("CA0"))?, py::getitem(&t4932, &Py::str("CB0"))?, py::getitem(&t4932, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4933 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4934 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4956 = (t4934).items()?;
        if t4956.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4956[0].clone();
        v_v = t4956[1].clone();
        if ({ let t4935 = v_k.clone(); Py::Bool({ let t4936 = v_outputs.clone(); if !py::contains(&t4936, &t4935)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4977(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4979: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4978 = v_inputs.clone(); f_riedel(py::getitem(&t4978, &Py::str("Tb"))?, py::getitem(&t4978, &Py::str("Tc"))?, py::getitem(&t4978, &Py::str("Pc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4979 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4980 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5002 = (t4980).items()?;
        if t5002.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5002[0].clone();
        v_v = t5002[1].clone();
        if ({ let t4981 = v_k.clone(); Py::Bool({ let t4982 = v_outputs.clone(); if !py::contains(&t4982, &t4981)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5039(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5041: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5040 = v_inputs.clone(); f_rtd_moments(py::getitem(&t5040, &Py::str("t"))?, py::getitem(&t5040, &Py::str("C"))?, py::getitem(&t5040, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5041 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5042 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5064 = (t5042).items()?;
        if t5064.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5064[0].clone();
        v_v = t5064[1].clone();
        if ({ let t5043 = v_k.clone(); Py::Bool({ let t5044 = v_outputs.clone(); if !py::contains(&t5044, &t5043)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_507(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t509: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t508 = v_inputs.clone(); f_batch_conv_2(py::getitem(&t508, &Py::str("k"))?, py::getitem(&t508, &Py::str("CA0"))?, py::getitem(&t508, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t509 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t510 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t532 = (t510).items()?;
        if t532.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t532[0].clone();
        v_v = t532[1].clone();
        if ({ let t511 = v_k.clone(); Py::Bool({ let t512 = v_outputs.clone(); if !py::contains(&t512, &t511)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5089(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5091: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5090 = v_inputs.clone(); f_sensible_heat(py::getitem(&t5090, &Py::str("n"))?, py::getitem(&t5090, &Py::str("a"))?, py::getitem(&t5090, &Py::str("b"))?, py::getitem(&t5090, &Py::str("T1"))?, py::getitem(&t5090, &Py::str("T2"))?, py::method(&t5090, "get", &[Py::str("c"), Py::Float(0.0)])?, py::method(&t5090, "get", &[Py::str("d"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5091 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5092 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5114 = (t5092).items()?;
        if t5114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5114[0].clone();
        v_v = t5114[1].clone();
        if ({ let t5093 = v_k.clone(); Py::Bool({ let t5094 = v_outputs.clone(); if !py::contains(&t5094, &t5093)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5136(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5138: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5137 = v_inputs.clone(); f_series_max(py::getitem(&t5137, &Py::str("k1"))?, py::getitem(&t5137, &Py::str("k2"))?, py::getitem(&t5137, &Py::str("CA0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5138 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5139 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5161 = (t5139).items()?;
        if t5161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5161[0].clone();
        v_v = t5161[1].clone();
        if ({ let t5140 = v_k.clone(); Py::Bool({ let t5141 = v_outputs.clone(); if !py::contains(&t5141, &t5140)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5162(mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py, mut v_T: Py, mut v_P: Py) -> R {
    return Ok(f__eos(Py::str("SRK"), v_Tc.clone(), v_Pc.clone(), v_omega.clone(), v_T.clone(), v_P.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5216(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5218: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5217 = v_inputs.clone(); s_5162(py::getitem(&t5217, &Py::str("Tc"))?, py::getitem(&t5217, &Py::str("Pc"))?, py::getitem(&t5217, &Py::str("omega"))?, py::getitem(&t5217, &Py::str("T"))?, py::getitem(&t5217, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5218 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5219 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5241 = (t5219).items()?;
        if t5241.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5241[0].clone();
        v_v = t5241[1].clone();
        if ({ let t5220 = v_k.clone(); Py::Bool({ let t5221 = v_outputs.clone(); if !py::contains(&t5221, &t5220)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5274(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5276: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5275 = v_inputs.clone(); f_stefan_flux(py::getitem(&t5275, &Py::str("D"))?, py::getitem(&t5275, &Py::str("P"))?, py::getitem(&t5275, &Py::str("T"))?, py::getitem(&t5275, &Py::str("L"))?, py::getitem(&t5275, &Py::str("yA1"))?, py::getitem(&t5275, &Py::str("yA2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5276 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5277 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5299 = (t5277).items()?;
        if t5299.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5299[0].clone();
        v_v = t5299[1].clone();
        if ({ let t5278 = v_k.clone(); Py::Bool({ let t5279 = v_outputs.clone(); if !py::contains(&t5279, &t5278)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5363(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5365: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5364 = v_inputs.clone(); f_uniquac_gamma(py::getitem(&t5364, &Py::str("x"))?, py::getitem(&t5364, &Py::str("r"))?, py::getitem(&t5364, &Py::str("q"))?, py::getitem(&t5364, &Py::str("tau"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5365 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5366 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5388 = (t5366).items()?;
        if t5388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5388[0].clone();
        v_v = t5388[1].clone();
        if ({ let t5367 = v_k.clone(); Py::Bool({ let t5368 = v_outputs.clone(); if !py::contains(&t5368, &t5367)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5414 = v_inputs.clone(); f_mixing_tank(py::getitem(&t5414, &Py::str("V"))?, py::getitem(&t5414, &Py::str("q"))?, py::getitem(&t5414, &Py::str("C_in"))?, py::getitem(&t5414, &Py::str("C0"))?, py::getitem(&t5414, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5416 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5438 = (t5416).items()?;
        if t5438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5438[0].clone();
        v_v = t5438[1].clone();
        if ({ let t5417 = v_k.clone(); Py::Bool({ let t5418 = v_outputs.clone(); if !py::contains(&t5418, &t5417)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5467(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5469: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5468 = v_inputs.clone(); f_variable_volume_tank(py::getitem(&t5468, &Py::str("V0"))?, py::getitem(&t5468, &Py::str("q_in"))?, py::getitem(&t5468, &Py::str("q_out"))?, py::getitem(&t5468, &Py::str("C_in"))?, py::getitem(&t5468, &Py::str("C0"))?, py::getitem(&t5468, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5469 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5470 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5492 = (t5470).items()?;
        if t5492.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5492[0].clone();
        v_v = t5492[1].clone();
        if ({ let t5471 = v_k.clone(); Py::Bool({ let t5472 = v_outputs.clone(); if !py::contains(&t5472, &t5471)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5516(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5518: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5517 = v_inputs.clone(); f_abbott_virial(py::getitem(&t5517, &Py::str("T"))?, py::getitem(&t5517, &Py::str("P"))?, py::getitem(&t5517, &Py::str("Tc"))?, py::getitem(&t5517, &Py::str("Pc"))?, py::getitem(&t5517, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5518 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5519 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5541 = (t5519).items()?;
        if t5541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5541[0].clone();
        v_v = t5541[1].clone();
        if ({ let t5520 = v_k.clone(); Py::Bool({ let t5521 = v_outputs.clone(); if !py::contains(&t5521, &t5520)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5565(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5566 = v_inputs.clone(); f_watson(py::getitem(&t5566, &Py::str("T"))?, py::getitem(&t5566, &Py::str("T_ref"))?, py::getitem(&t5566, &Py::str("Hvap_ref"))?, py::getitem(&t5566, &Py::str("Tc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5567 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5568 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5590 = (t5568).items()?;
        if t5590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5590[0].clone();
        v_v = t5590[1].clone();
        if ({ let t5569 = v_k.clone(); Py::Bool({ let t5570 = v_outputs.clone(); if !py::contains(&t5570, &t5569)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_557(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t559: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t558 = v_inputs.clone(); f_batch_heating_T(py::getitem(&t558, &Py::str("m"))?, py::getitem(&t558, &Py::str("cp"))?, py::getitem(&t558, &Py::str("UA"))?, py::getitem(&t558, &Py::str("T_s"))?, py::getitem(&t558, &Py::str("T0"))?, py::getitem(&t558, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t559 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t560 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t582 = (t560).items()?;
        if t582.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t582[0].clone();
        v_v = t582[1].clone();
        if ({ let t561 = v_k.clone(); Py::Bool({ let t562 = v_outputs.clone(); if !py::contains(&t562, &t561)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5637(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5639: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5638 = v_inputs.clone(); f_wilson_gamma(py::getitem(&t5638, &Py::str("x"))?, py::getitem(&t5638, &Py::str("Lam"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5639 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5640 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5662 = (t5640).items()?;
        if t5662.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5662[0].clone();
        v_v = t5662[1].clone();
        if ({ let t5641 = v_k.clone(); Py::Bool({ let t5642 = v_outputs.clone(); if !py::contains(&t5642, &t5641)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5749(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5751: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5750 = v_inputs.clone(); f_wilson_flash(py::getitem(&t5750, &Py::str("z"))?, py::getitem(&t5750, &Py::str("Tc"))?, py::getitem(&t5750, &Py::str("Pc"))?, py::getitem(&t5750, &Py::str("omega"))?, py::getitem(&t5750, &Py::str("p_unit"))?, py::getitem(&t5750, &Py::str("T"))?, py::getitem(&t5750, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5751 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5752 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5774 = (t5752).items()?;
        if t5774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5774[0].clone();
        v_v = t5774[1].clone();
        if ({ let t5753 = v_k.clone(); Py::Bool({ let t5754 = v_outputs.clone(); if !py::contains(&t5754, &t5753)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5839(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5841: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5840 = v_inputs.clone(); f_ziegler_nichols(py::getitem(&t5840, &Py::str("K"))?, py::getitem(&t5840, &Py::str("taus"))?, py::getitem(&t5840, &Py::str("t_unit"))?, py::getitem(&t5840, &Py::str("controller"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5841 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5842 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5864 = (t5842).items()?;
        if t5864.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5864[0].clone();
        v_v = t5864[1].clone();
        if ({ let t5843 = v_k.clone(); Py::Bool({ let t5844 = v_outputs.clone(); if !py::contains(&t5844, &t5843)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_607(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t609: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t608 = v_inputs.clone(); f_batch_heating_time(py::getitem(&t608, &Py::str("m"))?, py::getitem(&t608, &Py::str("cp"))?, py::getitem(&t608, &Py::str("UA"))?, py::getitem(&t608, &Py::str("T_s"))?, py::getitem(&t608, &Py::str("T0"))?, py::getitem(&t608, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t609 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t610 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t632 = (t610).items()?;
        if t632.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t632[0].clone();
        v_v = t632[1].clone();
        if ({ let t611 = v_k.clone(); Py::Bool({ let t612 = v_outputs.clone(); if !py::contains(&t612, &t611)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_663(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t665: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t664 = v_inputs.clone(); f_batch_time_1(py::getitem(&t664, &Py::str("k"))?, py::getitem(&t664, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t665 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t666 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t688 = (t666).items()?;
        if t688.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t688[0].clone();
        v_v = t688[1].clone();
        if ({ let t667 = v_k.clone(); Py::Bool({ let t668 = v_outputs.clone(); if !py::contains(&t668, &t667)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_719(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t721: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t720 = v_inputs.clone(); f_batch_time_2(py::getitem(&t720, &Py::str("k"))?, py::getitem(&t720, &Py::str("CA0"))?, py::getitem(&t720, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t721 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t722 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t744 = (t722).items()?;
        if t744.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t744[0].clone();
        v_v = t744[1].clone();
        if ({ let t723 = v_k.clone(); Py::Bool({ let t724 = v_outputs.clone(); if !py::contains(&t724, &t723)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_785(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t787: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t786 = v_inputs.clone(); f_minimum_reflux(py::getitem(&t786, &Py::str("alpha"))?, py::getitem(&t786, &Py::str("zF"))?, py::getitem(&t786, &Py::str("xD"))?, py::getitem(&t786, &Py::str("q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t787 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t788 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t810 = (t788).items()?;
        if t810.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t810[0].clone();
        v_v = t810[1].clone();
        if ({ let t789 = v_k.clone(); Py::Bool({ let t790 = v_outputs.clone(); if !py::contains(&t790, &t789)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_848(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t850: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t849 = v_inputs.clone(); f_binary_separator(py::getitem(&t849, &Py::str("F"))?, py::getitem(&t849, &Py::str("zF"))?, py::getitem(&t849, &Py::str("xD"))?, py::getitem(&t849, &Py::str("xB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t850 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t851 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t873 = (t851).items()?;
        if t873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t873[0].clone();
        v_v = t873[1].clone();
        if ({ let t852 = v_k.clone(); Py::Bool({ let t853 = v_outputs.clone(); if !py::contains(&t853, &t852)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_937(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t939: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t938 = v_inputs.clone(); f_blending(py::getitem(&t938, &Py::str("W"))?, py::getitem(&t938, &Py::str("p"))?, py::getitem(&t938, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t939 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t940 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t962 = (t940).items()?;
        if t962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t962[0].clone();
        v_v = t962[1].clone();
        if ({ let t941 = v_k.clone(); Py::Bool({ let t942 = v_outputs.clone(); if !py::contains(&t942, &t941)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_987(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t989: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t988 = v_inputs.clone(); f_filtration_volume(py::getitem(&t988, &Py::str("A"))?, py::getitem(&t988, &Py::str("dP"))?, py::getitem(&t988, &Py::str("mu"))?, py::getitem(&t988, &Py::str("alpha"))?, py::getitem(&t988, &Py::str("c"))?, py::getitem(&t988, &Py::str("Rm"))?, py::getitem(&t988, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t989 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t990 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1012 = (t990).items()?;
        if t1012.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1012[0].clone();
        v_v = t1012[1].clone();
        if ({ let t991 = v_k.clone(); Py::Bool({ let t992 = v_outputs.clone(); if !py::contains(&t992, &t991)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
