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
    "chemeng_equipment_cost_scaling",
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
        "chemeng_equipment_cost_scaling" => {
            let r = f_cost_scaling(match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("I2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1697(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_ergun_pressure_drop" => {
            let r = f_ergun(match v.get("dp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("vs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1753(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_exponential_decline" => {
            let r = f_decline_exp(match v.get("qi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Di") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_fenske_minimum_stages" => {
            let r = f_fenske(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1850(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_filtration_constants_fit" => {
            let r = f_filtration_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1919(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_first_order_step_fit" => {
            let r = f_fit_first_order(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2032(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_fopdt_step_response" => {
            let r = f_fopdt_step(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2087(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_gas_pz_material_balance" => {
            let r = f_gas_pz(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gp_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2155(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_gas_well_backpressure" => {
            let r = f_backpressure(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pwf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2228(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_harmonic_decline" => {
            let r = f_decline_harm(match v.get("qi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Di") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2278(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_havlena_odeh_gas_cap" => {
            let r = f_ho_gascap(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Eo") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Eg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2344(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_havlena_odeh_oil_in_place" => {
            let r = f_ho_oil(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Et") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2402(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_horner_buildup" => {
            let r = f_horner(match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pws") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2469(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_hyperbolic_decline" => {
            let r = f_decline_hyp(match v.get("qi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Di") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2525(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_kremser_absorber" => {
            let r = f_kremser_absorber(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_in") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2583(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_kremser_stripper" => {
            let r = f_kremser_stripper(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y_in") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2641(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_lag_frequency_response" => {
            let r = f_lag_freq(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2696(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_lee_kesler_vapour_pressure" => {
            let r = f_lee_kesler(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2745(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_line_source_drawdown" => {
            let r = f_line_source(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ct") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pi") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2817(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_mass_to_mole_fractions" => {
            let r = f_mass_to_mole(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2878(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_mass_transfer_coefficient" => {
            let r = f_mass_transfer_coefficient(match v.get("geometry") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2951(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_mccabe_thiele_stages" => {
            let r = f_mccabe_thiele(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3037(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_modified_raoult_bubble_pressure" => {
            let r = f_modified_raoult_bubble(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Psat") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_multicomponent_underwood" => {
            let r = f_underwood_multi(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lk") { Some(x) => py::from_val(x), None => { return None } }, match v.get("hk") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3200(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_npsh_available" => {
            let r = f_npsh_available(match v.get("p_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h_f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("NPSHr") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3256(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_nrtl_activity_coefficients" => {
            let r = f_nrtl_gamma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3328(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_p_control_offset" => {
            let r = f_p_offset(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_peng_robinson" => {
            let r = s_3433(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3487(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_conversion_first_order" => {
            let r = f_pfr_conv_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3531(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_conversion_second_order" => {
            let r = f_pfr_conv_2(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3575(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_gas_volume_first_order" => {
            let r = f_pfr_gas_1(match v.get("FA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3637(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_gas_volume_second_order" => {
            let r = f_pfr_gas_2(match v.get("FA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3699(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_volume_first_order" => {
            let r = f_pfr_volume_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3755(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pfr_volume_second_order" => {
            let r = f_pfr_volume_2(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3811(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pi_loop_margins" => {
            let r = f_margins_pi(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tauI") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3900(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pid_controller_output" => {
            let r = f_pid_position(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tauI") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tauD") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bias") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3958(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pitzer_heat_of_vaporization" => {
            let r = f_pitzer_hvap(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4007(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_pump_head" => {
            let r = f_pump_head(match v.get("p1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dz") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("v1") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("v2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Q") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("eta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4076(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_rachford_rice_flash" => {
            let r = f_rachford_rice(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4181(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_rackett_liquid_volume" => {
            let r = f_rackett(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Zc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4230(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_radial_darcy_flow" => {
            let r = f_radial_flow(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pe") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rw") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4283(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_bubble_pressure" => {
            let r = f_bubble_P(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4359(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_bubble_temperature" => {
            let r = f_bubble_T(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4469(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_dew_pressure" => {
            let r = f_dew_P(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4545(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_dew_temperature" => {
            let r = f_dew_T(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4655(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_raoult_isothermal_flash" => {
            let r = f_raoult_flash(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("units") { Some(x) => py::from_val(x), None => { return None } }, match v.get("base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4778(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_reaction_enthalpy_at_temperature" => {
            let r = f_reaction_enthalpy_T(match v.get("nu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4835(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_recycle_purge" => {
            let r = f_recycle_purge(match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_recycle_reactor_first_order" => {
            let r = f_recycle_reactor_1(match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4942(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_reversible_first_order_batch" => {
            let r = f_reversible_batch(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CB0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4992(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_riedel_heat_of_vaporization" => {
            let r = f_riedel(match v.get("Tb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5038(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_rtd_moments" => {
            let r = f_rtd_moments(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5100(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_sensible_heat_cp_polynomial" => {
            let r = f_sensible_heat(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5150(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_series_reaction_maximum" => {
            let r = f_series_max(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CA0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5197(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_srk" => {
            let r = s_5223(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5277(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_stefan_diffusion_flux" => {
            let r = f_stefan_flux(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yA1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yA2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5335(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_uniquac_activity_coefficients" => {
            let r = f_uniquac_gamma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5424(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_unsteady_mixing_tank" => {
            let r = f_mixing_tank(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5474(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_variable_volume_tank" => {
            let r = f_variable_volume_tank(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5528(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_virial_compressibility" => {
            let r = f_abbott_virial(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5577(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_watson_heat_of_vaporization" => {
            let r = f_watson(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Hvap_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5626(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_wilson_activity_coefficients" => {
            let r = f_wilson_gamma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_wilson_k_flash" => {
            let r = f_wilson_flash(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("omega") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5810(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chemeng_ziegler_nichols_tuning" => {
            let r = f_ziegler_nichols(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("taus") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_unit") { Some(x) => py::from_val(x), None => { return None } }, match v.get("controller") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5900(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4691 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4691;
out.push({ let t4692 = py::len(&v_r.clone())?; Py::Bool({ let t4693 = Py::Int(3i64); if py::eq(&t4692, &t4693) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("give three Antoine coefficients (A, B, C) per component"))?;
    if ({ let t4694 = v_n.clone(); Py::Bool({ let t4695 = Py::None; if !py::is(&t4694, &t4695) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4696 = py::len(&v_ant.clone())?; Py::Bool({ let t4697 = v_n.clone(); if py::eq(&t4696, &t4697) { true } else { false } }) }, Py::str("give one row of Antoine coefficients per component"))?;
    }
    for t4698 in py::iter(&(v_ant.clone()))? {
        v_r = t4698;
        let _ = f__fin({ let mut v = vec![]; v.extend((v_r.clone()).items()?); Py::tuple(v) })?;
        let _ = f__need({ let t4702 = py::getitem(&v_r.clone(), &Py::Int(1i64))?; Py::Bool({ let t4703 = Py::Int(0i64); if py::gt(&t4702, &t4703)? { true } else { false } }) }, Py::str("the Antoine coefficient B must be positive"))?;
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
    let t5855 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t5855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t5855[0].clone();
    v_fb = t5855[1].clone();
    if ({ let t5856 = v_fa.clone(); Py::Bool({ let t5857 = Py::Int(0i64); if py::eq(&t5856, &t5857) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t5858 = v_fb.clone(); Py::Bool({ let t5859 = Py::Int(0i64); if py::eq(&t5858, &t5859) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t5860 = { let t5862 = v_fa.clone(); Py::Bool({ let t5863 = Py::Int(0i64); if py::gt(&t5862, &t5863)? { true } else { false } }) }; Py::Bool({ let t5861 = { let t5864 = v_fb.clone(); Py::Bool({ let t5865 = Py::Int(0i64); if py::gt(&t5864, &t5865)? { true } else { false } }) }; if py::ne(&t5860, &t5861) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t5866 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t5866;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t5871 = { let t5867 = v_m.clone(); Py::Bool({ let t5868 = v_a.clone(); if py::eq(&t5867, &t5868) { true } else { false } }) }; if t5871.truthy() { t5871 } else { { let t5869 = v_m.clone(); Py::Bool({ let t5870 = v_b.clone(); if py::eq(&t5869, &t5870) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t5872 = v_fm.clone(); Py::Bool({ let t5873 = Py::Int(0i64); if py::eq(&t5872, &t5873) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t5874 = { let t5876 = v_fm.clone(); Py::Bool({ let t5877 = Py::Int(0i64); if py::gt(&t5876, &t5877)? { true } else { false } }) }; Py::Bool({ let t5875 = { let t5878 = v_fa.clone(); Py::Bool({ let t5879 = Py::Int(0i64); if py::gt(&t5878, &t5879)? { true } else { false } }) }; if py::eq(&t5874, &t5875) { true } else { false } }) }).truthy() {
            let t5880 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t5880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5880[0].clone();
            v_fa = t5880[1].clone();
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
    let _ = f__need({ let t5724 = Py::Int(2i64); Py::Bool({ let t5725 = py::len(&v_v.clone())?; if py::le(&t5724, &t5725)? { { let t5726 = Py::Int(10i64); if py::le(&t5725, &t5726)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, ": give 2 to 10 mole fractions".to_string()].concat()))?;
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
    let t5137 = (py::slice(&py::add(&Py::list((v_co.clone()).items()?), &Py::list(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)]))?, None, Some((Py::Int(4i64)).i()?), None)?).items()?;
    if t5137.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5137[0].clone();
    v_b = t5137[1].clone();
    v_c = t5137[2].clone();
    v_d = t5137[3].clone();
    return Ok(py::add(&py::add(&py::add(&py::mul(&v_a.clone(), &py::sub(&v_T2.clone(), &v_T1.clone())?)?, &py::mul(&py::truediv(&v_b.clone(), &Py::Int(2i64))?, &py::sub(&py::pow(&v_T2.clone(), &Py::Int(2i64))?, &py::pow(&v_T1.clone(), &Py::Int(2i64))?)?)?)?, &py::mul(&py::truediv(&v_c.clone(), &Py::Int(3i64))?, &py::sub(&py::pow(&v_T2.clone(), &Py::Int(3i64))?, &py::pow(&v_T1.clone(), &Py::Int(3i64))?)?)?)?, &py::mul(&py::truediv(&v_d.clone(), &Py::Int(4i64))?, &py::sub(&py::pow(&v_T2.clone(), &Py::Int(4i64))?, &py::pow(&v_T1.clone(), &Py::Int(4i64))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cp_matrix(mut v_cp: Py, mut v_n: Py) -> R {
    let _ = f__need({ let t4809 = { let t4804 = py::len(&v_cp.clone())?; Py::Bool({ let t4805 = v_n.clone(); if py::eq(&t4804, &t4805) { true } else { false } }) }; if !t4809.truthy() { t4809 } else { { let t4806 = Py::Int(1i64); Py::Bool({ let t4807 = py::len(&py::getitem(&v_cp.clone(), &Py::Int(0i64))?)?; if py::le(&t4806, &t4807)? { { let t4808 = Py::Int(4i64); if py::le(&t4807, &t4808)? { true } else { false } } } else { false } }) } } }, Py::str("give one row of 1 to 4 cp coefficients (a, b, c, d) per species"))?;
    return Ok({ let mut out = vec![]; for t4810 in py::iter(&(v_cp.clone()))? { let mut v_r: Py; 
v_r = t4810;
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
    let _ = f__need({ let t5239 = py::abs(&v_disc.clone())?; Py::Bool({ let t5240 = py::mul(&Py::Float(1e-08), &v_scale.clone())?; if py::gt(&t5239, &t5240)? { true } else { false } }) }, Py::str("the state is too close to a double root of the cubic (near the spinodal or critical point)"))?;
    if ({ let t5241 = v_disc.clone(); Py::Bool({ let t5242 = Py::Int(0i64); if py::gt(&t5241, &t5242)? { true } else { false } }) }).truthy() {
        v_s = py::math1("sqrt", &v_disc.clone())?;
        v_u = (if ({ let t5243 = v_qq.clone(); Py::Bool({ let t5244 = Py::Int(0i64); if py::le(&t5243, &t5244)? { true } else { false } }) }).truthy() { py::add(&py::truediv(&py::neg(&v_qq.clone())?, &Py::Float(2.0))?, &v_s.clone())? } else { py::sub(&py::truediv(&py::neg(&v_qq.clone())?, &Py::Float(2.0))?, &v_s.clone())? });
        v_u = py::copysign(&py::pow(&py::abs(&v_u.clone())?, &py::truediv(&Py::Float(1.0), &Py::Float(3.0))?)?, &v_u.clone())?;
        v_t = (if ({ let t5245 = v_u.clone(); Py::Bool({ let t5246 = Py::Int(0i64); if py::ne(&t5245, &t5246) { true } else { false } }) }).truthy() { py::sub(&v_u.clone(), &py::truediv(&v_p.clone(), &py::mul(&Py::Float(3.0), &v_u.clone())?)?)? } else { Py::Float(0.0) });
        v_roots = Py::list(vec![py::sub(&v_t.clone(), &py::truediv(&v_a2.clone(), &Py::Float(3.0))?)?]);
    } else {
        v_m = py::mul(&Py::Float(2.0), &py::math1("sqrt", &py::truediv(&py::neg(&v_p.clone())?, &Py::Float(3.0))?)?)?;
        v_c = py::minmax(&[py::neg(&Py::Float(1.0))?, py::minmax(&[Py::Float(1.0), py::truediv(&py::mul(&Py::Float(3.0), &v_qq.clone())?, &py::mul(&v_p.clone(), &v_m.clone())?)?], false)?], true)?;
        v_th = py::truediv(&py::math1("acos", &v_c.clone())?, &Py::Float(3.0))?;
        v_roots = { let mut out = vec![]; for t5247 in py::range_iter(&[(Py::Int(3i64)).i()?]) { let mut v_k: Py; 
v_k = t5247;
out.push(py::sub(&py::mul(&v_m.clone(), &py::math1("cos", &py::sub(&v_th.clone(), &py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &Py::Float(3.0))?)?)?)?, &py::truediv(&v_a2.clone(), &Py::Float(3.0))?)?); } Py::list(out) };
    }
    v_out = Py::list(vec![]);
    for t5248 in py::iter(&(v_roots.clone()))? {
        v_z = t5248;
        for t5249 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
            v__ = t5249;
            v_f = py::add(&py::mul(&py::add(&py::mul(&py::add(&v_z.clone(), &v_a2.clone())?, &v_z.clone())?, &v_a1.clone())?, &v_z.clone())?, &v_a0.clone())?;
            v_d = py::add(&py::mul(&py::add(&py::mul(&Py::Float(3.0), &v_z.clone())?, &py::mul(&Py::Float(2.0), &v_a2.clone())?)?, &v_z.clone())?, &v_a1.clone())?;
            if ({ let t5250 = v_d.clone(); Py::Bool({ let t5251 = Py::Int(0i64); if py::eq(&t5250, &t5251) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t2980 = v_alpha.clone(); Py::Bool({ let t2981 = Py::Float(1.0); if py::gt(&t2980, &t2981)? { true } else { false } }) }, Py::str("the relative volatility alpha must exceed 1"))?;
    let _ = f__need({ let t2982 = Py::Int(0i64); Py::Bool({ let t2983 = v_zF.clone(); if py::lt(&t2982, &t2983)? { { let t2984 = v_xD.clone(); if py::lt(&t2983, &t2984)? { { let t2985 = Py::Int(1i64); if py::lt(&t2984, &t2985)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("the compositions must satisfy 0 < zF < xD < 1"))?;
    if ({ let t2986 = v_xB.clone(); Py::Bool({ let t2987 = Py::None; if !py::is(&t2986, &t2987) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2988 = Py::Int(0i64); Py::Bool({ let t2989 = v_xB.clone(); if py::lt(&t2988, &t2989)? { { let t2990 = v_zF.clone(); if py::lt(&t2989, &t2990)? { true } else { false } } } else { false } }) }, Py::str("the bottoms composition must satisfy 0 < xB < zF"))?;
    }
    let _ = f__need({ let t2991 = py::neg(&Py::Int(5i64))?; Py::Bool({ let t2992 = v_q.clone(); if py::le(&t2991, &t2992)? { { let t2993 = Py::Int(5i64); if py::le(&t2992, &t2993)? { true } else { false } } } else { false } }) }, Py::str("the feed quality q must lie between -5 and 5"))?;
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
    if ({ let t2791 = v_x.clone(); Py::Bool({ let t2792 = Py::Float(1.0); if py::le(&t2791, &t2792)? { true } else { false } }) }).truthy() {
        let t2793 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0), Py::Int(1i64)])).items()?;
        if t2793.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_s = t2793[0].clone();
        v_term = t2793[1].clone();
        v_n = t2793[2].clone();
        while (Py::Bool(true)).truthy() {
            v_term = py::mul(&v_term, &py::truediv(&py::neg(&v_x.clone())?, &v_n.clone())?)?;
            v_add = py::truediv(&py::neg(&v_term.clone())?, &v_n.clone())?;
            v_s = py::iadd(&v_s, &v_add.clone())?;
            if ({ let t2798 = { let t2794 = py::abs(&v_add.clone())?; Py::Bool({ let t2795 = py::mul(&Py::Float(1e-18), &py::abs(&v_s.clone())?)?; if py::le(&t2794, &t2795)? { true } else { false } }) }; if t2798.truthy() { t2798 } else { { let t2796 = v_n.clone(); Py::Bool({ let t2797 = Py::Int(200i64); if py::gt(&t2796, &t2797)? { true } else { false } }) } } }).truthy() {
                break;
            }
            v_n = py::iadd(&v_n, &Py::Int(1i64))?;
        }
        return Ok(py::add(&py::sub(&py::neg(&Py::Float(0.5772156649015329))?, &py::math1("log", &v_x.clone())?)?, &v_s.clone())?);
    }
    v_b = py::add(&v_x.clone(), &Py::Float(1.0))?;
    let t2799 = (Py::tuple(vec![Py::Float(1e+300), py::truediv(&Py::Float(1.0), &v_b.clone())?])).items()?;
    if t2799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2799[0].clone();
    v_d = t2799[1].clone();
    v_h = v_d.clone();
    for t2800 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(500i64)).i()?]) {
        v_i = t2800;
        v_an = py::mul(&py::neg(&v_i.clone())?, &v_i.clone())?;
        v_b = py::iadd(&v_b, &Py::Float(2.0))?;
        v_d = py::truediv(&Py::Float(1.0), &py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?)?;
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_de = py::mul(&v_c.clone(), &v_d.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t2801 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2802 = Py::Float(1e-16); if py::le(&t2801, &t2802)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t5233 = py::neg(&Py::Float(0.5))?; Py::Bool({ let t5234 = v_omega.clone(); if py::lt(&t5233, &t5234)? { { let t5235 = Py::Float(2.0); if py::lt(&t5234, &t5235)? { true } else { false } } } else { false } }) }, Py::str("the acentric factor must lie between -0.5 and 2"))?;
    v_RT = py::mul(&c_R_GAS()?, &v_T.clone())?;
    if ({ let t5236 = v_kind.clone(); Py::Bool({ let t5237 = Py::str("PR"); if py::eq(&t5236, &t5237) { true } else { false } }) }).truthy() {
        v_b = py::truediv(&py::mul(&py::mul(&c__PR_C2()?, &c_R_GAS()?)?, &v_Tc.clone())?, &v_Pc.clone())?;
        v_k = py::sub(&py::add(&Py::Float(0.37464), &py::mul(&Py::Float(1.54226), &v_omega.clone())?)?, &py::mul(&py::mul(&Py::Float(0.26992), &v_omega.clone())?, &v_omega.clone())?)?;
        v_a = py::mul(&py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&c__PR_C1()?, &c_R_GAS()?)?, &c_R_GAS()?)?, &v_Tc.clone())?, &v_Tc.clone())?, &v_Pc.clone())?, &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_k.clone(), &py::sub(&Py::Float(1.0), &py::math1("sqrt", &py::truediv(&v_T.clone(), &v_Tc.clone())?)?)?)?)?, &Py::Int(2i64))?)?;
        let t5238 = (Py::tuple(vec![py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &py::mul(&v_RT.clone(), &v_RT.clone())?)?, py::truediv(&py::mul(&v_b.clone(), &v_P.clone())?, &v_RT.clone())?])).items()?;
        if t5238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::cell_set(&c_A, t5238[0].clone());
        py::cell_set(&c_B, t5238[1].clone());
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
        let t5252 = (Py::tuple(vec![py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &py::mul(&v_RT.clone(), &v_RT.clone())?)?, py::truediv(&py::mul(&v_b.clone(), &v_P.clone())?, &v_RT.clone())?])).items()?;
        if t5252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::cell_set(&c_A, t5252[0].clone());
        py::cell_set(&c_B, t5252[1].clone());
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
    v_zs = { let mut out = vec![]; for t5253 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t5253;
if ({ let t5254 = v_z.clone(); Py::Bool({ let t5255 = py::cell_get(&c_B)?; if py::gt(&t5254, &t5255)? { true } else { false } }) }).truthy() { out.push(v_z.clone()); }} Py::list(out) };
    let _ = f__need(Py::Bool((v_zs.clone()).truthy()), Py::str("no physical root of the cubic"))?;
    if ({ let t5256 = py::len(&v_zs.clone())?; Py::Bool({ let t5257 = Py::Int(3i64); if py::eq(&t5256, &t5257) { true } else { false } }) }).truthy() {
        v_zs = Py::list(vec![py::getitem(&v_zs.clone(), &Py::Int(0i64))?, py::getitem(&v_zs.clone(), &Py::Int(2i64))?]);
    }
    if ({ let t5258 = py::len(&v_zs.clone())?; Py::Bool({ let t5259 = Py::Int(2i64); if py::eq(&t5258, &t5259) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t5260 = py::sub(&py::getitem(&v_zs.clone(), &Py::Int(1i64))?, &py::getitem(&v_zs.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t5261 = py::mul(&Py::Float(0.0001), &py::getitem(&v_zs.clone(), &Py::Int(1i64))?)?; if py::gt(&t5260, &t5261)? { true } else { false } }) }, Py::str("the liquid and vapour roots nearly coincide"))?;
    }
    let t5262 = (Py::tuple(vec![py::getitem(&v_zs.clone(), &Py::Int(0i64))?, py::getitem(&v_zs.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
    if t5262.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_zl = t5262[0].clone();
    v_zv = t5262[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("Z_vapour"), v_zv.clone()), (Py::str("Z_liquid"), v_zl.clone()), (Py::str("phi_vapour"), py::math1("exp", &py::call(&v_lnphi.clone(), &[v_zv.clone()])?)?), (Py::str("phi_liquid"), py::math1("exp", &py::call(&v_lnphi.clone(), &[v_zl.clone()])?)?), (Py::str("n_roots"), (if ({ let t5275 = py::len(&v_zs.clone())?; Py::Bool({ let t5276 = Py::Int(1i64); if py::eq(&t5275, &t5276) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(3i64) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eps_ok(mut v_eps: Py, mut v_X: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_eps.clone()]))?;
    let _ = f__need({ let t3686 = { let t3681 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t3682 = v_eps.clone(); if py::lt(&t3681, &t3682)? { { let t3683 = Py::Int(10i64); if py::le(&t3682, &t3683)? { true } else { false } } } else { false } }) }; if !t3686.truthy() { t3686 } else { { let t3684 = py::add(&Py::Int(1i64), &py::mul(&v_eps.clone(), &v_X.clone())?)?; Py::Bool({ let t3685 = Py::Int(0i64); if py::gt(&t3684, &t3685)? { true } else { false } }) } } }, Py::str("the volume-change factor eps must exceed -1"))?;
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
    for t5836 in py::iter(&(v_vals.clone()))? {
        v_v = t5836;
        let _ = f__need({ let t5838 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5838.truthy() { t5838 } else { { let t5837 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5837.truthy() { t5837 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str("every input must be a finite number"))?;
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
    let _ = f__need({ let t5320 = (if (v_lo_open.clone()).truthy() { { let t5312 = v_v.clone(); Py::Bool({ let t5313 = Py::Int(0i64); if py::gt(&t5312, &t5313)? { true } else { false } }) } } else { { let t5314 = v_v.clone(); Py::Bool({ let t5315 = Py::Int(0i64); if py::ge(&t5314, &t5315)? { true } else { false } }) } }); if !t5320.truthy() { t5320 } else { (if (v_hi_open.clone()).truthy() { { let t5316 = v_v.clone(); Py::Bool({ let t5317 = Py::Int(1i64); if py::lt(&t5316, &t5317)? { true } else { false } }) } } else { { let t5318 = v_v.clone(); Py::Bool({ let t5319 = Py::Int(1i64); if py::le(&t5318, &t5319)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fracs(mut v_name: Py, mut v_v: Py, mut v_tol: Py) -> R {
    let mut v_x: Py = Py::None;
    for t5727 in py::iter(&(v_v.clone()))? {
        v_x = t5727;
        let _ = f__need({ let t5731 = py::math1("isfinite", &v_x.clone())?; if !t5731.truthy() { t5731 } else { { let t5728 = py::neg(&Py::Float(1e-15))?; Py::Bool({ let t5729 = v_x.clone(); if py::le(&t5728, &t5729)? { { let t5730 = py::add(&Py::Int(1i64), &Py::Float(1e-15))?; if py::le(&t5729, &t5730)? { true } else { false } } } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, ": every fraction must lie between 0 and 1".to_string()].concat()))?;
    }
    let _ = f__need({ let t5732 = py::abs(&py::sub(&py::sum(&v_v.clone(), None)?, &Py::Float(1.0))?)?; Py::Bool({ let t5733 = v_tol.clone(); if py::le(&t5732, &t5733)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, ": the fractions must sum to 1".to_string()].concat()))?;
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
    let _ = f__need({ let t2433 = { let t2428 = v_n.clone(); Py::Bool({ let t2429 = Py::Int(2i64); if py::ge(&t2428, &t2429)? { true } else { false } }) }; if !t2433.truthy() { t2433 } else { Py::Bool(({ let mut out = vec![]; for t2430 in py::iter(&(v_vecs.clone()))? { let mut v_v: Py; 
v_v = t2430;
out.push({ let t2431 = py::len(&v_v.clone())?; Py::Bool({ let t2432 = v_n.clone(); if py::eq(&t2431, &t2432) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("the lists must have the same length (at least 2 points)"))?;
    for t2434 in py::iter(&(v_vecs.clone()))? {
        v_v = t2434;
        let _ = f__fin({ let mut v = vec![]; v.extend((v_v.clone()).items()?); Py::tuple(v) })?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__kremser_frac(mut v_A: Py, mut v_N: Py) -> R {
    if ({ let t2627 = py::abs(&py::sub(&v_A.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2628 = Py::Float(1e-12); if py::le(&t2627, &t2628)? { true } else { false } }) }).truthy() {
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
    for t3368 in py::iter(&(v_taus.clone()))? {
        v_t = t3368;
        v_p = f__poly_mul(v_p.clone(), Py::list(vec![v_t.clone(), Py::Float(1.0)]))?;
    }
    return Ok(v_p.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__liq_x(mut v_x: Py) -> R {
    let _ = f__need({ let t5652 = Py::Int(2i64); Py::Bool({ let t5653 = py::len(&v_x.clone())?; if py::le(&t5652, &t5653)? { { let t5654 = Py::Int(10i64); if py::le(&t5653, &t5654)? { true } else { false } } } else { false } }) }, Py::str("give 2 to 10 mole fractions"))?;
    let _ = f__fracs(Py::str("x"), v_x.clone(), Py::Float(1e-06))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5662 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5662;
out.push({ let t5663 = v_v.clone(); Py::Bool({ let t5664 = Py::Int(0i64); if py::gt(&t5663, &t5664)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("every mole fraction must be positive (use a tiny value for infinite dilution)"))?;
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
    let t3008 = (Py::tuple(vec![py::truediv(&v_R.clone(), &py::add(&v_R.clone(), &Py::Float(1.0))?)?, py::truediv(&v_xD.clone(), &py::add(&v_R.clone(), &Py::Float(1.0))?)?])).items()?;
    if t3008.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t3008[0].clone();
    v_b1 = t3008[1].clone();
    if ({ let t3009 = py::abs(&py::sub(&v_q.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t3010 = Py::Float(1e-15); if py::lt(&t3009, &t3010)? { true } else { false } }) }).truthy() {
        v_xp = v_zF.clone();
    } else {
        v_xp = py::truediv(&py::add(&v_b1.clone(), &py::truediv(&v_zF.clone(), &py::sub(&v_q.clone(), &Py::Float(1.0))?)?)?, &py::sub(&py::truediv(&v_q.clone(), &py::sub(&v_q.clone(), &Py::Float(1.0))?)?, &v_a1.clone())?)?;
    }
    v_yp = py::add(&py::mul(&v_a1.clone(), &v_xp.clone())?, &v_b1.clone())?;
    let _ = f__need({ let t3016 = { let t3011 = v_xB.clone(); Py::Bool({ let t3012 = v_xp.clone(); if py::lt(&t3011, &t3012)? { { let t3013 = v_xD.clone(); if py::lt(&t3012, &t3013)? { true } else { false } } } else { false } }) }; if !t3016.truthy() { t3016 } else { { let t3014 = v_yp.clone(); Py::Bool({ let t3015 = f__eq_y(v_alpha.clone(), v_xp.clone())?; if py::lt(&t3014, &t3015)? { true } else { false } }) } } }, Py::str("the operating lines must meet below the equilibrium curve (R above R_min)"))?;
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
    for t5506 in py::iter(&(v_vals.clone()))? {
        v_v = t5506;
        let _ = f__need({ let t5511 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5511.truthy() { t5511 } else { { let t5510 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5510.truthy() { t5510 } else { { let t5509 = py::math1("isfinite", &v_v.clone())?; if !t5509.truthy() { t5509 } else { { let t5507 = v_v.clone(); Py::Bool({ let t5508 = Py::Int(0i64); if py::ge(&t5507, &t5508)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5886 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5887 = (t5886).items()?;
if t5887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5887[0].clone();
v_v = t5887[1].clone();
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
            return Ok(py::truediv(&py::mul(&py::mul(&v_Kc.clone(), &v_K.clone())?, &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &py::pow(&py::mul(&v_tauI.clone(), &v_w.clone())?, &Py::Int(2i64))?)?)?)?)?, &py::prod(&{ let mut out = vec![]; for t3852 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t3852;
out.push(py::math1("sqrt", &py::add(&Py::Float(1.0), &py::pow(&py::mul(&v_w.clone(), &v_t.clone())?, &Py::Int(2i64))?)?)?); } Py::list(out) })?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_ph = { let v_K = v_K.clone(); let v_Kc = v_Kc.clone(); let v_mag = v_mag.clone(); let v_tauI = v_tauI.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::truediv(&py::neg(&Py::Float(std::f64::consts::PI))?, &Py::Int(2i64))?, &py::math1("atan", &py::mul(&v_tauI.clone(), &v_w.clone())?)?)?, &py::sum(&{ let mut out = vec![]; for t3853 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t3853;
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
    for t3369 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? {
        let t3374 = (t3369).items()?;
        if t3374.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t3374[0].clone();
        v_x = t3374[1].clone();
        for t3370 in py::iter(&(py::enumerate(&v_b.clone(), 0)?))? {
            let t3373 = (t3370).items()?;
            if t3373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t3373[0].clone();
            v_y = t3373[1].clone();
            let t3371 = v_out.clone(); let t3372 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t3371, &t3372, py::iadd(&py::getitem(&t3371, &t3372)?, &py::mul(&v_x.clone(), &v_y.clone())?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5844 in py::iter(&(v_vals.clone()))? {
        v_v = t5844;
        let _ = f__need({ let t5849 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5849.truthy() { t5849 } else { { let t5848 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5848.truthy() { t5848 } else { { let t5847 = py::math1("isfinite", &v_v.clone())?; if !t5847.truthy() { t5847 } else { { let t5845 = v_v.clone(); Py::Bool({ let t5846 = Py::Int(0i64); if py::gt(&t5845, &t5846)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
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
    let t4711 = (v_row.clone()).items()?;
    if t4711.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4711[0].clone();
    v_B = t4711[1].clone();
    v_C = t4711[2].clone();
    let t4712 = (py::getitem(&c__ANT_UNITS()?, &v_units.clone())?).items()?;
    if t4712.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fac = t4712[0].clone();
    v_off = t4712[1].clone();
    v_d = py::add(&py::sub(&v_T.clone(), &v_off.clone())?, &v_C.clone())?;
    let _ = f__need({ let t4713 = v_d.clone(); Py::Bool({ let t4714 = Py::Int(0i64); if py::gt(&t4713, &t4714)? { true } else { false } }) }, Py::str("the temperature is below the singular point of the Antoine equation (T + C <= 0)"))?;
    return Ok(py::mul(&v_fac.clone(), &py::pow(&py::getitem(&c__ANT_BASE()?, &v_base.clone())?, &py::sub(&v_A.clone(), &py::truediv(&v_B.clone(), &v_d.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t5891 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5891.truthy() { t5891 } else { { let t5890 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5890.truthy() { t5890 } else { { let t5888 = v_x.clone(); Py::Bool({ let t5889 = Py::None; if py::is(&t5888, &t5889) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Complex(..)))).truthy() {
        return Ok(py::complex(&[f__r(py::real(&v_x.clone())?)?, f__r(py::imag(&v_x.clone())?)?])?);
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5892 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5892;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5895 = { let t5893 = v_x.clone(); Py::Bool({ let t5894 = Py::Int(0i64); if py::eq(&t5893, &t5894) { true } else { false } }) }; if t5895.truthy() { t5895 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5896 = v_x.clone(); Py::Bool({ let t5897 = Py::Int(0i64); if py::eq(&t5896, &t5897) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t5906 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5906.truthy() { t5906 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5910 = py::not(&{ let t5907 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5907.truthy() { t5907 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5910.truthy() { t5910 } else { { let t5908 = py::len(&v_a.clone())?; Py::Bool({ let t5909 = py::len(&v_b.clone())?; if py::ne(&t5908, &t5909) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5911 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5912 = (t5911).items()?;
if t5912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5912[0].clone();
v_y = t5912[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t5913 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5913.truthy() { t5913 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5914 = v_a.clone(); Py::Bool({ let t5915 = v_b.clone(); if py::eq(&t5914, &t5915) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t5917: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5916 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5916[0].clone();
            v_b = t5916[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5917 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5918 = v_d.clone(); Py::Bool({ let t5919 = Py::Int(0i64); if py::eq(&t5918, &t5919) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5924 = { let t5920 = py::abs(&v_a.clone())?; Py::Bool({ let t5921 = Py::Float(1e-15); if py::lt(&t5920, &t5921)? { true } else { false } }) }; if !t5924.truthy() { t5924 } else { { let t5922 = py::abs(&v_b.clone())?; Py::Bool({ let t5923 = Py::Float(1e-15); if py::lt(&t5922, &t5923)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3378 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t3379 = Py::Int(0i64); if py::lt(&t3378, &t3379)? { true } else { false } }) }).truthy() {
        v_c = { let mut out = vec![]; for t3380 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t3380;
out.push(py::neg(&v_x.clone())?); } Py::list(out) };
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_rows = Py::list(vec![py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, py::add(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::len(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?, &py::len(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?]);
    for t3381 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t3381;
        let t3382 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t3382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3382[0].clone();
        v_b = t3382[1].clone();
        let _ = f__need({ let t3383 = py::abs(&py::getitem(&v_b.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3384 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3385 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t3385;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?)?; if py::gt(&t3383, &t3384)? { true } else { false } }) }, Py::str("the Routh array has a zero in its first column (marginal case)"))?;
        let _ = py::method(&v_rows.clone(), "append", &[py::add(&{ let mut out = vec![]; for t3386 in py::range_iter(&[(py::sub(&py::len(&v_a.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t3386;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?])?;
    }
    v_first = { let mut out = vec![]; for t3387 in py::iter(&(py::slice(&v_rows.clone(), None, Some((py::add(&v_n.clone(), &Py::Int(1i64))?).i()?), None)?))? { let mut v_r: Py; 
v_r = t3387;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3388 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t3388;
out.push({ let t3389 = py::abs(&v_x.clone())?; Py::Bool({ let t3390 = py::mul(&Py::Float(1e-12), &py::minmax(&[{ let mut out = vec![]; for t3391 in py::iter(&(v_c.clone()))? { let mut v_y: Py; 
v_y = t3391;
out.push(py::abs(&v_y.clone())?); } Py::list(out) }], true)?)?; if py::gt(&t3389, &t3390)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the Routh array has a zero in its first column (marginal case)"))?;
    return Ok(Py::Bool(({ let mut out = vec![]; for t3392 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t3392;
out.push({ let t3393 = v_x.clone(); Py::Bool({ let t3394 = Py::Int(0i64); if py::gt(&t3393, &t3394)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
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
    let _ = f__need({ let t5755 = { let t5747 = py::sum(&{ let mut out = vec![]; for t5749 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5750 = (t5749).items()?;
if t5750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5750[0].clone();
v_k = t5750[1].clone();
out.push(py::mul(&v_a.clone(), &v_k.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t5748 = Py::Float(1.0); if py::gt(&t5747, &t5748)? { true } else { false } }) }; if !t5755.truthy() { t5755 } else { { let t5751 = py::sum(&{ let mut out = vec![]; for t5753 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5754 = (t5753).items()?;
if t5754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5754[0].clone();
v_k = t5754[1].clone();
out.push(py::truediv(&v_a.clone(), &v_k.clone())?); } Py::list(out) }, None)?; Py::Bool({ let t5752 = Py::Float(1.0); if py::gt(&t5751, &t5752)? { true } else { false } }) } } }, Py::str("the mixture is not two-phase at these K-values (sum zK <= 1: all liquid; sum z/K <= 1: all vapour)"))?;
    let _ = f__need({ let t5756 = py::minmax(&[{ let mut out = vec![]; for t5758 in py::iter(&(v_K.clone()))? { let mut v_k: Py; 
v_k = t5758;
out.push(py::abs(&py::sub(&v_k.clone(), &Py::Float(1.0))?)?); } Py::list(out) }], true)?; Py::Bool({ let t5757 = Py::Float(0.01); if py::gt(&t5756, &t5757)? { true } else { false } }) }, Py::str("every K-value lies within 1 % of unity: the vapour fraction is too ill-conditioned to report"))?;
    let t5759 = (Py::tuple(vec![py::minmax(&[v_K.clone()], true)?, py::minmax(&[v_K.clone()], false)?])).items()?;
    if t5759.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kmax = t5759[0].clone();
    v_kmin = t5759[1].clone();
    v_lo = (if ({ let t5760 = v_kmax.clone(); Py::Bool({ let t5761 = Py::Int(1i64); if py::gt(&t5760, &t5761)? { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::sub(&Py::Float(1.0), &v_kmax.clone())?)? } else { py::neg(&Py::Float(f64::INFINITY))? });
    v_hi = (if ({ let t5762 = v_kmin.clone(); Py::Bool({ let t5763 = Py::Int(1i64); if py::lt(&t5762, &t5763)? { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::sub(&Py::Float(1.0), &v_kmin.clone())?)? } else { Py::Float(f64::INFINITY) });
    let t5764 = (Py::tuple(vec![py::minmax(&[v_lo.clone(), Py::Float(0.0)], true)?, py::minmax(&[v_hi.clone(), Py::Float(1.0)], false)?])).items()?;
    if t5764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t5764[0].clone();
    v_hi = t5764[1].clone();
    v_g = { let v_K = v_K.clone(); let v_hi = v_hi.clone(); let v_kmax = v_kmax.clone(); let v_kmin = v_kmin.clone(); let v_lo = v_lo.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_V: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t5765 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5766 = (t5765).items()?;
if t5766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5766[0].clone();
v_k = t5766[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &py::sub(&v_k.clone(), &Py::Float(1.0))?)?, &py::add(&Py::Float(1.0), &py::mul(&v_V.clone(), &py::sub(&v_k.clone(), &Py::Float(1.0))?)?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_V = f__bisect(v_g.clone(), v_lo.clone(), v_hi.clone(), Py::Int(500i64))?;
    v_x = { let mut out = vec![]; for t5793 in py::iter(&(py::zip(&[v_z.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_k: Py; 
let t5794 = (t5793).items()?;
if t5794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5794[0].clone();
v_k = t5794[1].clone();
out.push(py::truediv(&v_a.clone(), &py::add(&Py::Float(1.0), &py::mul(&v_V.clone(), &py::sub(&v_k.clone(), &Py::Float(1.0))?)?)?)?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_V.clone(), v_x.clone(), { let mut out = vec![]; for t5795 in py::iter(&(py::zip(&[v_K.clone(), v_x.clone()])?))? { let mut v_k: Py; let mut v_xi: Py; 
let t5796 = (t5795).items()?;
if t5796.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5796[0].clone();
v_xi = t5796[1].clone();
out.push(py::mul(&v_k.clone(), &v_xi.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sherwood(mut v_geometry: Py, mut v_Re: Py, mut v_Sc: Py) -> R {
    if ({ let t2935 = v_geometry.clone(); Py::Bool({ let t2936 = Py::str("sphere"); if py::eq(&t2935, &t2936) { true } else { false } }) }).truthy() {
        return Ok(py::add(&Py::Float(2.0), &py::mul(&py::mul(&Py::Float(0.6), &py::math1("sqrt", &v_Re.clone())?)?, &py::pow(&v_Sc.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?)?);
    }
    if ({ let t2937 = v_geometry.clone(); Py::Bool({ let t2938 = Py::str("flat_plate"); if py::eq(&t2937, &t2938) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&py::mul(&Py::Float(0.664), &py::math1("sqrt", &v_Re.clone())?)?, &py::pow(&v_Sc.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?);
    }
    return Ok(py::mul(&py::mul(&Py::Float(0.023), &py::pow(&v_Re.clone(), &Py::Float(0.83))?)?, &py::pow(&v_Sc.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__square(mut v_name: Py, mut v_M: Py, mut v_n: Py, mut v_diag: Py) -> R {
    let _ = f__need({ let t5670 = { let t5665 = py::len(&v_M.clone())?; Py::Bool({ let t5666 = v_n.clone(); if py::eq(&t5665, &t5666) { true } else { false } }) }; if !t5670.truthy() { t5670 } else { Py::Bool(({ let mut out = vec![]; for t5667 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t5667;
out.push({ let t5668 = py::len(&v_r.clone())?; Py::Bool({ let t5669 = v_n.clone(); if py::eq(&t5668, &t5669) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be an n x n matrix for n = ".to_string(), py::format(&v_n.clone(), "")?, " components".to_string()].concat()))?;
    if ({ let t5671 = v_diag.clone(); Py::Bool({ let t5672 = Py::None; if !py::is(&t5671, &t5672) { true } else { false } }) }).truthy() {
        let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5673 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5673;
out.push({ let t5674 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?, &v_diag.clone())?)?; Py::Bool({ let t5675 = Py::Float(1e-12); if py::le(&t5674, &t5675)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&["the diagonal of ".to_string(), py::format(&v_name.clone(), "")?, " must be ".to_string(), py::format(&v_diag.clone(), "g")?].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stages_check(mut v_N: Py, mut v_pos: Py) -> R {
    let _ = f__need({ let t2612 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2612.truthy() { t2612 } else { { let t2609 = Py::Int(1i64); Py::Bool({ let t2610 = v_N.clone(); if py::le(&t2609, &t2610)? { { let t2611 = Py::Int(200i64); if py::le(&t2610, &t2611)? { true } else { false } } } else { false } }) } } }, Py::str("N must be a whole number of stages from 1 to 200"))?;
    let _ = f__pos(Py::str("the factors and slopes"), { let mut v = vec![]; v.extend((v_pos.clone()).items()?); Py::tuple(v) })?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_bracket(mut v_ant: Py, mut v_units: Py, mut v_base: Py, mut v_P: Py) -> R {
    let mut v_ts: Py = Py::None;
    v_ts = { let mut out = vec![]; for t4600 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4600;
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
    let t4601 = (v_row.clone()).items()?;
    if t4601.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4601[0].clone();
    v_B = t4601[1].clone();
    v_C = t4601[2].clone();
    let t4602 = (py::getitem(&c__ANT_UNITS()?, &v_units.clone())?).items()?;
    if t4602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fac = t4602[0].clone();
    v_off = t4602[1].clone();
    v_lg = py::truediv(&py::math1("log", &py::truediv(&v_P.clone(), &v_fac.clone())?)?, &py::math1("log", &py::getitem(&c__ANT_BASE()?, &v_base.clone())?)?)?;
    let _ = f__need({ let t4603 = py::sub(&v_A.clone(), &v_lg.clone())?; Py::Bool({ let t4604 = Py::Int(0i64); if py::gt(&t4603, &t4604)? { true } else { false } }) }, Py::str("this pressure lies outside the Antoine equation's range"))?;
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
    let _ = f__need({ let t5850 = py::len(&v_ts.clone())?; Py::Bool({ let t5851 = Py::Int(3i64); if py::ge(&t5850, &t5851)? { true } else { false } }) }, Py::str("at least three lags are needed for a finite ultimate gain"))?;
    v_f = { let v_K = v_K.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sum(&{ let mut out = vec![]; for t5852 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t5852;
out.push(py::math1("atan", &py::mul(&v_w.clone(), &v_t.clone())?)?); } Py::list(out) }, None)?, &Py::Float(std::f64::consts::PI))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_hi = py::truediv(&Py::Float(1.0), &py::minmax(&[v_ts.clone()], false)?)?;
    while ({ let t5853 = py::call(&v_f.clone(), &[v_hi.clone()])?; Py::Bool({ let t5854 = Py::Int(0i64); if py::lt(&t5853, &t5854)? { true } else { false } }) }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Float(2.0))?;
    }
    v_wu = f__bisect(v_f.clone(), Py::Float(0.0), v_hi.clone(), Py::Int(500i64))?;
    v_ku = py::truediv(&py::prod(&{ let mut out = vec![]; for t5881 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t5881;
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
    let t2994 = (Py::tuple(vec![Py::Float(1.0), v_alpha.clone()])).items()?;
    if t2994.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2994[0].clone();
    v_hi = t2994[1].clone();
    for t2995 in py::range_iter(&[(Py::Int(2000i64)).i()?]) {
        v__ = t2995;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t3000 = { let t2996 = v_m.clone(); Py::Bool({ let t2997 = v_lo.clone(); if py::le(&t2996, &t2997)? { true } else { false } }) }; if t3000.truthy() { t3000 } else { { let t2998 = v_m.clone(); Py::Bool({ let t2999 = v_hi.clone(); if py::ge(&t2998, &t2999)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t3001 = py::call(&v_f.clone(), &[v_m.clone()])?; Py::Bool({ let t3002 = Py::Int(0i64); if py::gt(&t3001, &t3002)? { true } else { false } }) }).truthy() {
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
            return Ok(py::sub(&py::sum(&{ let mut out = vec![]; for t3173 in py::iter(&(py::zip(&[v_alpha.clone(), v_z.clone()])?))? { let mut v_a: Py; let mut v_zi: Py; 
let t3174 = (t3173).items()?;
if t3174.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3174[0].clone();
v_zi = t3174[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_zi.clone())?, &py::sub(&v_a.clone(), &v_th.clone())?)?); } Py::list(out) }, None)?, &py::sub(&Py::Int(1i64), &v_q.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t3175 = (Py::tuple(vec![v_a_lo.clone(), v_a_hi.clone()])).items()?;
    if t3175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3175[0].clone();
    v_hi = t3175[1].clone();
    for t3176 in py::range_iter(&[(Py::Int(3000i64)).i()?]) {
        v__ = t3176;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t3181 = { let t3177 = v_m.clone(); Py::Bool({ let t3178 = v_lo.clone(); if py::le(&t3177, &t3178)? { true } else { false } }) }; if t3181.truthy() { t3181 } else { { let t3179 = v_m.clone(); Py::Bool({ let t3180 = v_hi.clone(); if py::ge(&t3179, &t3180)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t3182 = py::call(&v_f.clone(), &[v_m.clone()])?; Py::Bool({ let t3183 = Py::Int(0i64); if py::gt(&t3182, &t3183)? { true } else { false } }) }).truthy() {
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
    if ({ let t5884 = v_controller.clone(); Py::Bool({ let t5885 = Py::str("P"); if py::eq(&t5884, &t5885) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("Ku"), v_ku.clone()), (Py::str("Pu"), v_pu.clone()), (Py::str("Kc"), py::mul(&Py::Float(0.5), &v_ku.clone())?)]))?);
    }
    if ({ let t5898 = v_controller.clone(); Py::Bool({ let t5899 = Py::str("PI"); if py::eq(&t5898, &t5899) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t5563 = v_Z.clone(); Py::Bool({ let t5564 = Py::Float(0.5); if py::gt(&t5563, &t5564)? { true } else { false } }) }, Py::str("the truncated virial equation is not valid here (Z would fall below 0.5)"))?;
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
    let _ = f__need({ let t2197 = py::len(&v_q.clone())?; Py::Bool({ let t2198 = Py::Int(2i64); if py::ge(&t2197, &t2198)? { true } else { false } }) }, Py::str("at least two test rates are needed"))?;
    v_qs = { let mut out = vec![]; for t2199 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t2199;
out.push(py::mul(&v_v.clone(), &py::getitem(&c__QU()?, &v_q_unit.clone())?)?); } Py::list(out) };
    v_ps = { let mut out = vec![]; for t2200 in py::iter(&(v_pwf.clone()))? { let mut v_v: Py; 
v_v = t2200;
out.push(py::mul(&v_v.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?); } Py::list(out) };
    let _ = f__pos(Py::str("the rates"), { let mut v = vec![]; v.extend((v_qs.clone()).items()?); Py::tuple(v) })?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2201 in py::iter(&(v_ps.clone()))? { let mut v_p: Py; 
v_p = t2201;
out.push({ let t2202 = Py::Int(0i64); Py::Bool({ let t2203 = v_p.clone(); if py::le(&t2202, &t2203)? { { let t2204 = v_pr.clone(); if py::lt(&t2203, &t2204)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("flowing pressures must lie between 0 and the reservoir pressure"))?;
    v_xs = { let mut out = vec![]; for t2205 in py::iter(&(v_ps.clone()))? { let mut v_p: Py; 
v_p = t2205;
out.push(py::math1("log10", &py::sub(&py::mul(&v_pr.clone(), &v_pr.clone())?, &py::mul(&v_p.clone(), &v_p.clone())?)?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2206 in py::iter(&(v_qs.clone()))? { let mut v_v: Py; 
v_v = t2206;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let t2207 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2207[0].clone();
    v_my = t2207[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2208 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2208;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2209 = v_sxx.clone(); Py::Bool({ let t2210 = Py::Int(0i64); if py::gt(&t2209, &t2210)? { true } else { false } }) }, Py::str("the flowing pressures must differ"))?;
    v_nn = py::truediv(&py::sum(&{ let mut out = vec![]; for t2211 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2212 = (t2211).items()?;
if t2212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2212[0].clone();
v_y = t2212[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_lc = py::sub(&v_my.clone(), &py::mul(&v_nn.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t2213 = Py::Float(0.3); Py::Bool({ let t2214 = v_nn.clone(); if py::lt(&t2213, &t2214)? { { let t2215 = Py::Float(1.5); if py::lt(&t2214, &t2215)? { true } else { false } } } else { false } }) }, Py::str("the back-pressure exponent n falls outside 0.3 to 1.5: the data do not fit the back-pressure equation"))?;
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
    v_ps = { let mut out = vec![]; for t4338 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4338;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    v_P = py::sum(&{ let mut out = vec![]; for t4343 in py::iter(&(py::zip(&[v_x.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4344 = (t4343).items()?;
if t4344.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4344[0].clone();
v_b = t4344[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("y"), { let mut out = vec![]; for t4357 in py::iter(&(py::zip(&[v_x.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4358 = (t4357).items()?;
if t4358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4358[0].clone();
v_b = t4358[1].clone();
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
    let t4419 = (f__t_bracket(v_ant.clone(), v_units.clone(), v_base.clone(), v_P.clone())?).items()?;
    if t4419.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4419[0].clone();
    v_hi = t4419[1].clone();
    v_f = { let v_P = v_P.clone(); let v_ant = v_ant.clone(); let v_base = v_base.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_units = v_units.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::truediv(&py::sum(&{ let mut out = vec![]; for t4420 in py::iter(&(py::zip(&[v_x.clone(), v_ant.clone()])?))? { let mut v_a: Py; let mut v_r: Py; 
let t4421 = (t4420).items()?;
if t4421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4421[0].clone();
v_r = t4421[1].clone();
out.push(py::mul(&v_a.clone(), &f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?)?); } Py::list(out) }, None)?, &v_P.clone())?, &Py::Float(1.0))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_T = (if ({ let t4426 = v_hi.clone(); Py::Bool({ let t4427 = v_lo.clone(); if py::eq(&t4426, &t4427) { true } else { false } }) }).truthy() { v_lo.clone() } else { f__bisect(v_f.clone(), v_lo.clone(), v_hi.clone(), Py::Int(500i64))? });
    v_ps = { let mut out = vec![]; for t4454 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4454;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("y"), { let mut out = vec![]; for t4467 in py::iter(&(py::zip(&[v_x.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4468 = (t4467).items()?;
if t4468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4468[0].clone();
v_b = t4468[1].clone();
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
fn f_cost_scaling(mut v_C1: Py, mut v_S1: Py, mut v_S2: Py, mut v_n: Py, mut v_I1: Py, mut v_I2: Py) -> R {
    let mut v_Cs: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_ir: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let _ = f__pos(Py::str("the known cost and the two capacities"), Py::tuple(vec![v_C1.clone(), v_S1.clone(), v_S2.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_n.clone()]))?;
    let _ = f__need({ let t1671 = Py::Int(0i64); Py::Bool({ let t1672 = v_n.clone(); if py::lt(&t1671, &t1672)? { { let t1673 = Py::Float(1.5); if py::le(&t1672, &t1673)? { true } else { false } } } else { false } }) }, Py::str("the capacity exponent must lie in (0, 1.5] (0.6 for the six-tenths rule)"))?;
    v_ratio = py::truediv(&v_S2.clone(), &v_S1.clone())?;
    let _ = f__need({ let t1674 = Py::Float(0.1); Py::Bool({ let t1675 = v_ratio.clone(); if py::le(&t1674, &t1675)? { { let t1676 = Py::Int(10i64); if py::le(&t1675, &t1676)? { true } else { false } } } else { false } }) }, Py::str("the two capacities must lie within a factor of 10 of each other (the rule is unreliable beyond)"))?;
    let _ = f__need({ let t1677 = { let t1679 = v_I1.clone(); Py::Bool({ let t1680 = Py::None; if py::is(&t1679, &t1680) { true } else { false } }) }; Py::Bool({ let t1678 = { let t1681 = v_I2.clone(); Py::Bool({ let t1682 = Py::None; if py::is(&t1681, &t1682) { true } else { false } }) }; if py::eq(&t1677, &t1678) { true } else { false } }) }, Py::str("give both cost indices (at the known cost's date and now) or neither"))?;
    v_f = py::pow(&v_ratio.clone(), &v_n.clone())?;
    v_Cs = py::mul(&v_C1.clone(), &v_f.clone())?;
    v_ir = Py::Float(1.0);
    if ({ let t1683 = v_I1.clone(); Py::Bool({ let t1684 = Py::None; if !py::is(&t1683, &t1684) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the cost indices"), Py::tuple(vec![v_I1.clone(), v_I2.clone()]))?;
        v_ir = py::truediv(&v_I2.clone(), &v_I1.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("capacity_ratio"), v_ratio.clone()), (Py::str("scale_factor"), v_f.clone()), (Py::str("C_scaled"), v_Cs.clone()), (Py::str("index_ratio"), v_ir.clone()), (Py::str("C2"), py::mul(&v_Cs.clone(), &v_ir.clone())?)]))?);
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
    let _ = f__need({ let t2510 = Py::Int(0i64); Py::Bool({ let t2511 = v_b.clone(); if py::lt(&t2510, &t2511)? { { let t2512 = Py::Int(1i64); if py::lt(&t2511, &t2512)? { true } else { false } } } else { false } }) }, Py::str("the hyperbolic exponent b must lie strictly between 0 and 1 (b = 0 exponential, b = 1 harmonic)"))?;
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
    v_ps = { let mut out = vec![]; for t4524 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4524;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    v_P = py::truediv(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t4529 in py::iter(&(py::zip(&[v_y.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4530 = (t4529).items()?;
if t4530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4530[0].clone();
v_b = t4530[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("x"), { let mut out = vec![]; for t4543 in py::iter(&(py::zip(&[v_y.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4544 = (t4543).items()?;
if t4544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4544[0].clone();
v_b = t4544[1].clone();
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
    let t4605 = (f__t_bracket(v_ant.clone(), v_units.clone(), v_base.clone(), v_P.clone())?).items()?;
    if t4605.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4605[0].clone();
    v_hi = t4605[1].clone();
    v_f = { let v_P = v_P.clone(); let v_ant = v_ant.clone(); let v_base = v_base.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_units = v_units.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_T: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&Py::Float(1.0), &py::sum(&{ let mut out = vec![]; for t4606 in py::iter(&(py::zip(&[v_y.clone(), v_ant.clone()])?))? { let mut v_a: Py; let mut v_r: Py; 
let t4607 = (t4606).items()?;
if t4607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4607[0].clone();
v_r = t4607[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_P.clone())?, &f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?)?); } Py::list(out) }, None)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_T = (if ({ let t4612 = v_hi.clone(); Py::Bool({ let t4613 = v_lo.clone(); if py::eq(&t4612, &t4613) { true } else { false } }) }).truthy() { v_lo.clone() } else { f__bisect(v_f.clone(), v_lo.clone(), v_hi.clone(), Py::Int(500i64))? });
    v_ps = { let mut out = vec![]; for t4640 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4640;
out.push(f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("x"), { let mut out = vec![]; for t4653 in py::iter(&(py::zip(&[v_y.clone(), v_ps.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4654 = (t4653).items()?;
if t4654.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4654[0].clone();
v_b = t4654[1].clone();
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
    let _ = f__need({ let t1832 = v_alpha.clone(); Py::Bool({ let t1833 = Py::Float(1.0); if py::gt(&t1832, &t1833)? { true } else { false } }) }, Py::str("the relative volatility alpha must exceed 1"))?;
    let _ = f__need({ let t1834 = Py::Int(0i64); Py::Bool({ let t1835 = v_xB.clone(); if py::lt(&t1834, &t1835)? { { let t1836 = v_xD.clone(); if py::lt(&t1835, &t1836)? { { let t1837 = Py::Int(1i64); if py::lt(&t1836, &t1837)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("the compositions must satisfy 0 < xB < xD < 1"))?;
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
    let _ = f__need({ let t1876 = py::len(&v_t.clone())?; Py::Bool({ let t1877 = py::len(&v_V.clone())?; if py::eq(&t1876, &t1877) { { let t1878 = Py::Int(3i64); if py::ge(&t1877, &t1878)? { true } else { false } } } else { false } }) }, Py::str("t and V need the same number of points (at least 3)"))?;
    let _ = f__pos(Py::str("A, dP, mu and c"), Py::tuple(vec![v_A.clone(), v_dP.clone(), v_mu.clone(), v_c.clone()]))?;
    v_ts = { let mut out = vec![]; for t1885 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t1885;
out.push(py::mul(&v_x.clone(), &py::getitem(&Py::dict(vec![(Py::str("s"), Py::Float(1.0)), (Py::str("min"), Py::Float(60.0)), (Py::str("h"), Py::Float(3600.0))]), &v_t_unit.clone())?)?); } Py::list(out) };
    v_vs = { let mut out = vec![]; for t1886 in py::iter(&(v_V.clone()))? { let mut v_x: Py; 
v_x = t1886;
out.push(py::mul(&v_x.clone(), &py::getitem(&Py::dict(vec![(Py::str("m3"), Py::Float(1.0)), (Py::str("L"), Py::Float(0.001))]), &v_v_unit.clone())?)?); } Py::list(out) };
    let _ = f__need({ let t1893 = Py::Bool(({ let mut out = vec![]; for t1887 in py::iter(&(v_vs.clone()))? { let mut v_v: Py; 
v_v = t1887;
out.push({ let t1888 = v_v.clone(); Py::Bool({ let t1889 = Py::Int(0i64); if py::gt(&t1888, &t1889)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t1893.truthy() { t1893 } else { Py::Bool(({ let mut out = vec![]; for t1890 in py::iter(&(v_ts.clone()))? { let mut v_x: Py; 
v_x = t1890;
out.push({ let t1891 = v_x.clone(); Py::Bool({ let t1892 = Py::Int(0i64); if py::gt(&t1891, &t1892)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("times and volumes must be positive"))?;
    v_yv = { let mut out = vec![]; for t1894 in py::iter(&(py::zip(&[v_ts.clone(), v_vs.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1895 = (t1894).items()?;
if t1895.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1895[0].clone();
v_b = t1895[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    v_n = py::len(&v_vs.clone())?;
    let t1896 = (Py::tuple(vec![py::truediv(&py::sum(&v_vs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_yv.clone(), None)?, &v_n.clone())?])).items()?;
    if t1896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t1896[0].clone();
    v_my = t1896[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t1897 in py::iter(&(v_vs.clone()))? { let mut v_x: Py; 
v_x = t1897;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t1898 = v_sxx.clone(); Py::Bool({ let t1899 = Py::Int(0i64); if py::gt(&t1898, &t1899)? { true } else { false } }) }, Py::str("the volumes must not all be equal"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t1900 in py::iter(&(py::zip(&[v_vs.clone(), v_yv.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1901 = (t1900).items()?;
if t1901.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1901[0].clone();
v_y = t1901[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_icpt = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t1906 = { let t1902 = v_slope.clone(); Py::Bool({ let t1903 = Py::Int(0i64); if py::gt(&t1902, &t1903)? { true } else { false } }) }; if !t1906.truthy() { t1906 } else { { let t1904 = v_icpt.clone(); Py::Bool({ let t1905 = Py::Int(0i64); if py::gt(&t1904, &t1905)? { true } else { false } }) } } }, Py::str("the data do not give a positive slope and intercept of t/V against V"))?;
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
    let _ = f__need({ let t1945 = py::len(&v_t.clone())?; Py::Bool({ let t1946 = py::len(&v_y.clone())?; if py::eq(&t1945, &t1946) { { let t1947 = Py::Int(4i64); if py::ge(&t1946, &t1947)? { true } else { false } } } else { false } }) }, Py::str("t and y need the same number of points (at least 4)"))?;
    let _ = f__fin({ let mut v = vec![v_M.clone()]; v.extend((v_y.clone()).items()?); Py::tuple(v) })?;
    let _ = f__need({ let t1951 = v_M.clone(); Py::Bool({ let t1952 = Py::Int(0i64); if py::ne(&t1951, &t1952) { true } else { false } }) }, Py::str("the step size M must not be zero"))?;
    let _ = f__nonneg(Py::str("theta"), Py::tuple(vec![v_theta.clone()]))?;
    v_ts = { let mut out = vec![]; for t1959 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t1959;
out.push(py::mul(&v_x.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1960 in py::iter(&(py::zip(&[v_ts.clone(), py::slice(&v_ts.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1961 = (t1960).items()?;
if t1961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1961[0].clone();
v_b = t1961[1].clone();
out.push({ let t1962 = v_b.clone(); Py::Bool({ let t1963 = v_a.clone(); if py::gt(&t1962, &t1963)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("times must increase"))?;
    v_pts = { let mut out = vec![]; for t1964 in py::iter(&(py::zip(&[v_ts.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1965 = (t1964).items()?;
if t1965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1965[0].clone();
v_b = t1965[1].clone();
if ({ let t1966 = v_a.clone(); Py::Bool({ let t1967 = v_theta.clone(); if py::gt(&t1966, &t1967)? { true } else { false } }) }).truthy() { out.push(Py::tuple(vec![py::sub(&v_a.clone(), &v_theta.clone())?, v_b.clone()])); }} Py::list(out) };
    let _ = f__need({ let t1968 = py::len(&v_pts.clone())?; Py::Bool({ let t1969 = Py::Int(3i64); if py::ge(&t1968, &t1969)? { true } else { false } }) }, Py::str("need at least three points after the dead time"))?;
    v_kfit = { let v_M = v_M.clone(); let v_kfit = v_kfit.clone(); let v_pts = v_pts.clone(); let v_t = v_t.clone(); let v_t_unit = v_t_unit.clone(); let v_theta = v_theta.clone(); let v_ts = v_ts.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_tau: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_f: Py = Py::None;
            let mut v_sff: Py = Py::None;
            v_f = { let mut out = vec![]; for t1970 in py::iter(&(v_pts.clone()))? { let mut v__: Py; let mut v_s: Py; 
let t1971 = (t1970).items()?;
if t1971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t1971[0].clone();
v__ = t1971[1].clone();
out.push(py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_s.clone())?, &v_tau.clone())?)?)?); } Py::list(out) };
            v_sff = py::sum(&{ let mut out = vec![]; for t1972 in py::iter(&(v_f.clone()))? { let mut v_v: Py; 
v_v = t1972;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) }, None)?;
            return Ok(Py::tuple(vec![py::truediv(&py::sum(&{ let mut out = vec![]; for t1973 in py::iter(&(py::zip(&[v_f.clone(), v_pts.clone()])?))? { let mut v__: Py; let mut v_v: Py; let mut v_yy: Py; 
let t1974 = (t1973).items()?;
if t1974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1974[0].clone();
let t1975 = (t1974[1].clone()).items()?;
if t1975.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t1975[0].clone();
v_yy = t1975[1].clone();
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
            let t1976 = (py::call(&v_kfit.clone(), &[v_tau.clone()])?).items()?;
            if t1976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_K = t1976[0].clone();
            v_f = t1976[1].clone();
            return Ok(py::sum(&{ let mut out = vec![]; for t1977 in py::iter(&(py::zip(&[v_f.clone(), v_pts.clone()])?))? { let mut v__: Py; let mut v_v: Py; let mut v_yy: Py; 
let t1978 = (t1977).items()?;
if t1978.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1978[0].clone();
let t1979 = (t1978[1].clone()).items()?;
if t1979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t1979[0].clone();
v_yy = t1979[1].clone();
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
            let t1980 = (py::call(&v_kfit.clone(), &[v_tau.clone()])?).items()?;
            if t1980.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_K = t1980[0].clone();
            v_f = t1980[1].clone();
            return Ok(py::sum(&{ let mut out = vec![]; for t1981 in py::iter(&(py::zip(&[v_f.clone(), v_pts.clone()])?))? { let mut v_s: Py; let mut v_v: Py; let mut v_yy: Py; 
let t1982 = (t1981).items()?;
if t1982.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t1982[0].clone();
let t1983 = (t1982[1].clone()).items()?;
if t1983.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t1983[0].clone();
v_yy = t1983[1].clone();
out.push(py::mul(&py::mul(&py::sub(&v_yy.clone(), &py::mul(&py::mul(&v_K.clone(), &v_M.clone())?, &v_v.clone())?)?, &py::truediv(&v_s.clone(), &py::pow(&v_tau.clone(), &Py::Int(2i64))?)?)?, &py::math1("exp", &py::truediv(&py::neg(&v_s.clone())?, &v_tau.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_span = py::minmax(&[{ let mut out = vec![]; for t1984 in py::iter(&(v_pts.clone()))? { let mut v__: Py; let mut v_s: Py; 
let t1985 = (t1984).items()?;
if t1985.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_s = t1985[0].clone();
v__ = t1985[1].clone();
out.push(v_s.clone()); } Py::list(out) }], true)?;
    v_grid = { let mut out = vec![]; for t1986 in py::range_iter(&[(Py::Int(401i64)).i()?]) { let mut v_i: Py; 
v_i = t1986;
out.push(py::mul(&py::mul(&v_span.clone(), &Py::Float(0.001))?, &py::pow(&Py::Float(100000.0), &py::truediv(&v_i.clone(), &Py::Float(400.0))?)?)?); } Py::list(out) };
    v_vals = { let mut out = vec![]; for t1987 in py::iter(&(v_grid.clone()))? { let mut v_g: Py; 
v_g = t1987;
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
    let _ = f__need({ let t1988 = Py::Int(0i64); Py::Bool({ let t1989 = v_i.clone(); if py::lt(&t1988, &t1989)? { { let t1990 = py::sub(&py::len(&v_grid.clone())?, &Py::Int(1i64))?; if py::lt(&t1989, &t1990)? { true } else { false } } } else { false } }) }, Py::str("the data do not determine a time constant (no interior minimum)"))?;
    v_tau = (if ({ let t1991 = py::mul(&py::call(&v_dsse.clone(), &[py::getitem(&v_grid.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?])?, &py::call(&v_dsse.clone(), &[py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])?)?; Py::Bool({ let t1992 = Py::Int(0i64); if py::lt(&t1991, &t1992)? { true } else { false } }) }).truthy() { f__bisect(v_dsse.clone(), py::getitem(&v_grid.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, Py::Int(500i64))? } else { py::getitem(&v_grid.clone(), &v_i.clone())? });
    let t2019 = (py::call(&v_kfit.clone(), &[v_tau.clone()])?).items()?;
    if t2019.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_K = t2019[0].clone();
    v__ = t2019[1].clone();
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
    v_y = (if ({ let t2073 = v_t.clone(); Py::Bool({ let t2074 = v_theta.clone(); if py::le(&t2073, &t2074)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::mul(&v_K.clone(), &v_M.clone())?, &py::neg(&py::math1("expm1", &py::truediv(&py::neg(&py::sub(&v_t.clone(), &v_theta.clone())?)?, &v_tau.clone())?)?)?)? });
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
    v_xs = { let mut out = vec![]; for t2129 in py::iter(&(v_Gp.clone()))? { let mut v_g: Py; 
v_g = t2129;
out.push(py::mul(&v_g.clone(), &py::getitem(&c__GPU()?, &v_gp_unit.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2130 in py::iter(&(py::zip(&[v_p.clone(), v_z.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2131 = (t2130).items()?;
if t2131.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2131[0].clone();
v_b = t2131[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?, &v_b.clone())?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let t2132 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2132[0].clone();
    v_my = t2132[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2133 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2133;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2134 = v_sxx.clone(); Py::Bool({ let t2135 = Py::Int(0i64); if py::gt(&t2134, &t2135)? { true } else { false } }) }, Py::str("the cumulative productions must differ"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t2136 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2137 = (t2136).items()?;
if t2137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2137[0].clone();
v_y = t2137[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_icpt = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t2142 = { let t2138 = v_slope.clone(); Py::Bool({ let t2139 = Py::Int(0i64); if py::lt(&t2138, &t2139)? { true } else { false } }) }; if !t2142.truthy() { t2142 } else { { let t2140 = v_icpt.clone(); Py::Bool({ let t2141 = Py::Int(0i64); if py::gt(&t2140, &t2141)? { true } else { false } }) } } }, Py::str("p/z must fall as gas is produced"))?;
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
    let _ = f__need({ let t2319 = Py::Bool(({ let mut out = vec![]; for t2314 in py::iter(&(v_Eo.clone()))? { let mut v_e: Py; 
v_e = t2314;
out.push({ let t2315 = v_e.clone(); Py::Bool({ let t2316 = Py::Int(0i64); if py::gt(&t2315, &t2316)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t2319.truthy() { t2319 } else { { let t2317 = py::len(&v_F.clone())?; Py::Bool({ let t2318 = Py::Int(3i64); if py::ge(&t2317, &t2318)? { true } else { false } }) } } }, Py::str("Eo must be positive and at least three surveys are needed"))?;
    v_xs = { let mut out = vec![]; for t2320 in py::iter(&(py::zip(&[v_Eg.clone(), v_Eo.clone()])?))? { let mut v_g: Py; let mut v_o: Py; 
let t2321 = (t2320).items()?;
if t2321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_g = t2321[0].clone();
v_o = t2321[1].clone();
out.push(py::truediv(&v_g.clone(), &v_o.clone())?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2322 in py::iter(&(py::zip(&[v_F.clone(), v_Eo.clone()])?))? { let mut v_f: Py; let mut v_o: Py; 
let t2323 = (t2322).items()?;
if t2323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t2323[0].clone();
v_o = t2323[1].clone();
out.push(py::truediv(&v_f.clone(), &v_o.clone())?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let t2324 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2324[0].clone();
    v_my = t2324[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2325 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2325;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2326 = v_sxx.clone(); Py::Bool({ let t2327 = Py::Int(0i64); if py::gt(&t2326, &t2327)? { true } else { false } }) }, Py::str("Eg/Eo must vary between surveys"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t2328 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2329 = (t2328).items()?;
if t2329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2329[0].clone();
v_y = t2329[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_N = py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?;
    let _ = f__need({ let t2330 = v_N.clone(); Py::Bool({ let t2331 = Py::Int(0i64); if py::gt(&t2330, &t2331)? { true } else { false } }) }, Py::str("the fitted oil in place is not positive"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), py::mul(&v_N.clone(), &py::getitem(&c__VU()?, &v_v_unit.clone())?)?), (Py::str("m"), py::truediv(&v_slope.clone(), &v_N.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ho_oil(mut v_F: Py, mut v_Et: Py, mut v_v_unit: Py) -> R {
    let mut v_N: Py = Py::None;
    let _ = f__ho_check(Py::tuple(vec![v_F.clone(), v_Et.clone()]))?;
    let _ = f__need({ let t2386 = Py::Bool(({ let mut out = vec![]; for t2380 in py::iter(&(v_Et.clone()))? { let mut v_e: Py; 
v_e = t2380;
out.push({ let t2381 = v_e.clone(); Py::Bool({ let t2382 = Py::Int(0i64); if py::gt(&t2381, &t2382)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t2386.truthy() { t2386 } else { Py::Bool(({ let mut out = vec![]; for t2383 in py::iter(&(v_F.clone()))? { let mut v_f: Py; 
v_f = t2383;
out.push({ let t2384 = v_f.clone(); Py::Bool({ let t2385 = Py::Int(0i64); if py::gt(&t2384, &t2385)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("F and Et must be positive"))?;
    v_N = py::truediv(&py::sum(&{ let mut out = vec![]; for t2387 in py::iter(&(py::zip(&[v_F.clone(), v_Et.clone()])?))? { let mut v_e: Py; let mut v_f: Py; 
let t2388 = (t2387).items()?;
if t2388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_f = t2388[0].clone();
v_e = t2388[1].clone();
out.push(py::mul(&v_f.clone(), &v_e.clone())?); } Py::list(out) }, None)?, &py::sum(&{ let mut out = vec![]; for t2389 in py::iter(&(v_Et.clone()))? { let mut v_e: Py; 
v_e = t2389;
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
    v_ds = { let mut out = vec![]; for t2444 in py::iter(&(v_dt.clone()))? { let mut v_x: Py; 
v_x = t2444;
out.push(py::mul(&v_x.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    let _ = f__pos(Py::str("the shut-in times"), { let mut v = vec![]; v.extend((v_ds.clone()).items()?); Py::tuple(v) })?;
    v_xs = { let mut out = vec![]; for t2445 in py::iter(&(v_ds.clone()))? { let mut v_d: Py; 
v_d = t2445;
out.push(py::math1("log10", &py::truediv(&py::add(&v_tp.clone(), &v_d.clone())?, &v_d.clone())?)?); } Py::list(out) };
    v_ys = { let mut out = vec![]; for t2446 in py::iter(&(v_pws.clone()))? { let mut v_v: Py; 
v_v = t2446;
out.push(py::mul(&v_v.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?); } Py::list(out) };
    v_n = py::len(&v_xs.clone())?;
    let _ = f__need({ let t2447 = v_n.clone(); Py::Bool({ let t2448 = Py::Int(3i64); if py::ge(&t2447, &t2448)? { true } else { false } }) }, Py::str("at least three build-up points are needed"))?;
    let t2449 = (Py::tuple(vec![py::truediv(&py::sum(&v_xs.clone(), None)?, &v_n.clone())?, py::truediv(&py::sum(&v_ys.clone(), None)?, &v_n.clone())?])).items()?;
    if t2449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2449[0].clone();
    v_my = t2449[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2450 in py::iter(&(v_xs.clone()))? { let mut v_x: Py; 
v_x = t2450;
out.push(py::pow(&py::sub(&v_x.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, None)?;
    let _ = f__need({ let t2451 = v_sxx.clone(); Py::Bool({ let t2452 = Py::Int(0i64); if py::gt(&t2451, &t2452)? { true } else { false } }) }, Py::str("the shut-in times must differ"))?;
    v_slope = py::truediv(&py::sum(&{ let mut out = vec![]; for t2453 in py::iter(&(py::zip(&[v_xs.clone(), v_ys.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t2454 = (t2453).items()?;
if t2454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2454[0].clone();
v_y = t2454[1].clone();
out.push(py::mul(&py::sub(&v_x.clone(), &v_mx.clone())?, &py::sub(&v_y.clone(), &v_my.clone())?)?); } Py::list(out) }, None)?, &v_sxx.clone())?;
    v_m = py::neg(&v_slope.clone())?;
    let _ = f__need({ let t2455 = v_m.clone(); Py::Bool({ let t2456 = Py::Int(0i64); if py::gt(&t2455, &t2456)? { true } else { false } }) }, Py::str("the build-up pressure must rise with shut-in time"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("k"), py::truediv(&py::mul(&py::mul(&v_q.clone(), &v_mu.clone())?, &py::math1("log", &Py::Float(10.0))?)?, &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_m.clone())?, &v_h.clone())?)?), (Py::str("p_star"), py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kremser_absorber(mut v_N: Py, mut v_A: Py, mut v_m: Py, mut v_y_in: Py, mut v_x_in: Py) -> R {
    let mut v_y_out: Py = Py::None;
    let _ = f__stages_check(v_N.clone(), Py::tuple(vec![v_A.clone(), v_m.clone()]))?;
    let _ = f__nonneg(Py::str("the compositions"), Py::tuple(vec![v_y_in.clone(), v_x_in.clone()]))?;
    let _ = f__need({ let t2567 = v_y_in.clone(); Py::Bool({ let t2568 = py::mul(&v_m.clone(), &v_x_in.clone())?; if py::gt(&t2567, &t2568)? { true } else { false } }) }, Py::str("no absorption: the entering gas is not richer than equilibrium with the entering liquid"))?;
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
    let _ = f__need({ let t2625 = v_x_in.clone(); Py::Bool({ let t2626 = py::truediv(&v_y_in.clone(), &v_m.clone())?; if py::gt(&t2625, &t2626)? { true } else { false } }) }, Py::str("no stripping: the entering liquid is not richer than equilibrium with the entering gas"))?;
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
    let _ = f__need({ let t2670 = v_K.clone(); Py::Bool({ let t2671 = Py::Int(0i64); if py::gt(&t2670, &t2671)? { true } else { false } }) }, Py::str("the process gain must be positive"))?;
    let _ = f__need({ let t2672 = Py::Int(1i64); Py::Bool({ let t2673 = py::len(&v_taus.clone())?; if py::le(&t2672, &t2673)? { { let t2674 = Py::Int(3i64); if py::le(&t2673, &t2674)? { true } else { false } } } else { false } }) }, Py::str("give 1 to 3 time constants"))?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("the time constants and the frequency")); v.extend((v_taus.clone()).items()?); v.push(v_w.clone()); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_pos() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_pos() got an unexpected keyword argument".into())) } f__pos(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    v_ts = { let mut out = vec![]; for t2681 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t2681;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    v_ar = v_K.clone();
    for t2682 in py::iter(&(v_ts.clone()))? {
        v_t = t2682;
        v_ar = py::truediv(&v_ar, &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::pow(&py::mul(&v_w.clone(), &v_t.clone())?, &Py::Int(2i64))?)?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("AR"), v_ar.clone()), (Py::str("phase_deg"), py::neg(&py::sum(&{ let mut out = vec![]; for t2695 in py::iter(&(v_ts.clone()))? { let mut v_t: Py; 
v_t = t2695;
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
    let _ = f__need({ let t2731 = v_T.clone(); Py::Bool({ let t2732 = v_Tc.clone(); if py::lt(&t2731, &t2732)? { true } else { false } }) }, Py::str("T must be below the critical temperature"))?;
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
    let _ = f__need({ let t2789 = v_x.clone(); Py::Bool({ let t2790 = Py::Float(50.0); if py::le(&t2789, &t2790)? { true } else { false } }) }, Py::str("the pressure disturbance has not reached this radius yet (Ei argument above 50)"))?;
    v_dp = py::mul(&py::truediv(&py::mul(&v_q.clone(), &v_mu.clone())?, &py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_h.clone())?)?, &f__e1(v_x.clone())?)?;
    let _ = f__need({ let t2803 = v_dp.clone(); Py::Bool({ let t2804 = v_pi.clone(); if py::lt(&t2803, &t2804)? { true } else { false } }) }, Py::str("the drawdown exceeds the initial pressure"))?;
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
    let _ = f__need({ let t3840 = py::mul(&v_Kc.clone(), &v_K.clone())?; Py::Bool({ let t3841 = Py::Int(0i64); if py::gt(&t3840, &t3841)? { true } else { false } }) }, Py::str("the loop gain must be positive"))?;
    let _ = f__need({ let t3842 = Py::Int(3i64); Py::Bool({ let t3843 = py::len(&v_taus.clone())?; if py::le(&t3842, &t3843)? { { let t3844 = Py::Int(5i64); if py::le(&t3843, &t3844)? { true } else { false } } } else { false } }) }, Py::str("give 3 to 5 time constants"))?;
    let _ = f__pos(Py::str("tauI and the time constants"), { let mut v = vec![v_tauI.clone()]; v.extend((v_taus.clone()).items()?); Py::tuple(v) })?;
    v_ts = { let mut out = vec![]; for t3851 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t3851;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) };
    v_ti = py::mul(&v_tauI.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?;
    let t3854 = (f__pi_loop(v_Kc.clone(), v_ti.clone(), v_K.clone(), v_ts.clone())?).items()?;
    if t3854.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mag = t3854[0].clone();
    v_ph = t3854[1].clone();
    let t3855 = (Py::tuple(vec![py::truediv(&Py::Float(1e-06), &py::minmax(&[py::add(&v_ts.clone(), &Py::list(vec![v_ti.clone()]))?], true)?)?, py::truediv(&Py::Float(10000.0), &py::minmax(&[v_ts.clone()], false)?)?])).items()?;
    if t3855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3855[0].clone();
    v_hi = t3855[1].clone();
    v_grid = { let mut out = vec![]; for t3856 in py::range_iter(&[(Py::Int(4001i64)).i()?]) { let mut v_i: Py; 
v_i = t3856;
out.push(py::mul(&v_lo.clone(), &py::pow(&py::truediv(&v_hi.clone(), &v_lo.clone())?, &py::truediv(&v_i.clone(), &Py::Float(4000.0))?)?)?); } Py::list(out) };
    v_cross = { let mut out = vec![]; for t3857 in py::range_iter(&[(Py::Int(4000i64)).i()?]) { let mut v_i: Py; 
v_i = t3857;
if ({ let t3858 = py::mul(&py::add(&py::call(&v_ph.clone(), &[py::getitem(&v_grid.clone(), &v_i.clone())?])?, &Py::Float(std::f64::consts::PI))?, &py::add(&py::call(&v_ph.clone(), &[py::getitem(&v_grid.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])?, &Py::Float(std::f64::consts::PI))?)?; Py::Bool({ let t3859 = Py::Int(0i64); if py::le(&t3858, &t3859)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); }} Py::list(out) };
    let _ = f__need({ let t3860 = py::len(&v_cross.clone())?; Py::Bool({ let t3861 = Py::Int(1i64); if py::eq(&t3860, &t3861) { true } else { false } }) }, Py::str("the phase crosses -180 degrees more than once (or never): margins are not unique"))?;
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
    let _ = f__need({ let t2843 = Py::Int(2i64); Py::Bool({ let t2844 = py::len(&v_w.clone())?; if py::le(&t2843, &t2844)? { { let t2845 = py::len(&v_MW.clone())?; if py::eq(&t2844, &t2845) { { let t2846 = Py::Int(20i64); if py::le(&t2845, &t2846)? { true } else { false } } } else { false } } } else { false } }) }, Py::str("give one molar mass per mass fraction (2 to 20 components)"))?;
    let _ = f__fracs(Py::str("w"), v_w.clone(), Py::Float(1e-06))?;
    let _ = f__pos(Py::str("the molar masses"), { let mut v = vec![]; v.extend((v_MW.clone()).items()?); Py::tuple(v) })?;
    v_s = py::sum(&{ let mut out = vec![]; for t2860 in py::iter(&(py::zip(&[v_w.clone(), v_MW.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2861 = (t2860).items()?;
if t2861.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2861[0].clone();
v_b = t2861[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?;
    v_z = { let mut out = vec![]; for t2862 in py::iter(&(py::zip(&[v_w.clone(), v_MW.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2863 = (t2862).items()?;
if t2863.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2863[0].clone();
v_b = t2863[1].clone();
out.push(py::truediv(&py::truediv(&v_a.clone(), &v_b.clone())?, &v_s.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("MW_mean"), py::mul(&Py::Float(0.001), &py::sum(&{ let mut out = vec![]; for t2876 in py::iter(&(py::zip(&[v_z.clone(), v_MW.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2877 = (t2876).items()?;
if t2877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2877[0].clone();
v_b = t2877[1].clone();
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
    let _ = f__need({ let t2910 = v_geometry.clone(); Py::Bool({ let t2911 = c__SH_CORR()?; if py::contains(&t2911, &t2910)? { true } else { false } }) }, Py::str("geometry must be sphere, flat_plate or pipe"))?;
    v_Re = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &v_u.clone())?, &v_L.clone())?, &v_mu.clone())?;
    v_Sc = py::truediv(&v_mu.clone(), &py::mul(&v_rho.clone(), &v_D.clone())?)?;
    if ({ let t2912 = v_geometry.clone(); Py::Bool({ let t2913 = Py::str("sphere"); if py::eq(&t2912, &t2913) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t2920 = { let t2914 = Py::Int(2i64); Py::Bool({ let t2915 = v_Re.clone(); if py::le(&t2914, &t2915)? { { let t2916 = Py::Float(200000.0); if py::le(&t2915, &t2916)? { true } else { false } } } else { false } }) }; if !t2920.truthy() { t2920 } else { { let t2917 = Py::Float(0.6); Py::Bool({ let t2918 = v_Sc.clone(); if py::le(&t2917, &t2918)? { { let t2919 = Py::Int(400i64); if py::le(&t2918, &t2919)? { true } else { false } } } else { false } }) } } }, Py::str("Ranz-Marshall is stated for 2 <= Re <= 200000 and 0.6 <= Sc <= 400"))?;
    } else {
        if ({ let t2921 = v_geometry.clone(); Py::Bool({ let t2922 = Py::str("flat_plate"); if py::eq(&t2921, &t2922) { true } else { false } }) }).truthy() {
            let _ = f__need({ let t2927 = { let t2923 = v_Re.clone(); Py::Bool({ let t2924 = Py::Float(500000.0); if py::lt(&t2923, &t2924)? { true } else { false } }) }; if !t2927.truthy() { t2927 } else { { let t2925 = v_Sc.clone(); Py::Bool({ let t2926 = Py::Float(0.6); if py::ge(&t2925, &t2926)? { true } else { false } }) } } }, Py::str("the laminar flat-plate correlation needs Re < 500000 and Sc >= 0.6"))?;
        } else {
            let _ = f__need({ let t2934 = { let t2928 = Py::Int(2000i64); Py::Bool({ let t2929 = v_Re.clone(); if py::le(&t2928, &t2929)? { { let t2930 = Py::Int(70000i64); if py::le(&t2929, &t2930)? { true } else { false } } } else { false } }) }; if !t2934.truthy() { t2934 } else { { let t2931 = Py::Float(0.6); Py::Bool({ let t2932 = v_Sc.clone(); if py::le(&t2931, &t2932)? { { let t2933 = Py::Int(3000i64); if py::le(&t2932, &t2933)? { true } else { false } } } else { false } }) } } }, Py::str("Linton-Sherwood is stated for 2000 <= Re <= 70000 and 0.6 <= Sc <= 3000"))?;
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
    let _ = f__need({ let t3007 = { let t3003 = v_R.clone(); Py::Bool({ let t3004 = py::mul(&v_Rm.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-06))?)?; if py::gt(&t3003, &t3004)? { true } else { false } }) }; if !t3007.truthy() { t3007 } else { { let t3005 = v_R.clone(); Py::Bool({ let t3006 = Py::Int(0i64); if py::gt(&t3005, &t3006)? { true } else { false } }) } } }, Py::str(&["the reflux ratio must exceed the minimum reflux ratio ".to_string(), py::format(&v_Rm.clone(), ".6g")?].concat()))?;
    let t3017 = (f__mt_lines(v_alpha.clone(), v_zF.clone(), v_xD.clone(), v_xB.clone(), v_q.clone(), v_R.clone())?).items()?;
    if t3017.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t3017[0].clone();
    v_b1 = t3017[1].clone();
    v_xp = t3017[2].clone();
    v_yp = t3017[3].clone();
    v_s = t3017[4].clone();
    let t3018 = (Py::tuple(vec![py::truediv(&v_xD.clone(), &py::sub(&v_alpha.clone(), &py::mul(&py::sub(&v_alpha.clone(), &Py::Int(1i64))?, &v_xD.clone())?)?)?, Py::Int(1i64)])).items()?;
    if t3018.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3018[0].clone();
    v_n = t3018[1].clone();
    while ({ let t3023 = v_x.clone(); Py::Bool({ let t3024 = v_xB.clone(); if py::gt(&t3023, &t3024)? { true } else { false } }) }).truthy() {
        v_y = (if ({ let t3019 = v_x.clone(); Py::Bool({ let t3020 = v_xp.clone(); if py::gt(&t3019, &t3020)? { true } else { false } }) }).truthy() { py::add(&py::mul(&v_a1.clone(), &v_x.clone())?, &v_b1.clone())? } else { py::add(&v_xB.clone(), &py::mul(&v_s.clone(), &py::sub(&v_x.clone(), &v_xB.clone())?)?)? });
        v_x = py::truediv(&v_y.clone(), &py::sub(&v_alpha.clone(), &py::mul(&py::sub(&v_alpha.clone(), &Py::Int(1i64))?, &v_y.clone())?)?)?;
        v_n = py::iadd(&v_n, &Py::Int(1i64))?;
        let _ = f__need({ let t3021 = v_n.clone(); Py::Bool({ let t3022 = Py::Int(400i64); if py::le(&t3021, &t3022)? { true } else { false } }) }, Py::str("more than 400 stages: the reflux ratio is too close to the minimum"))?;
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
    let _ = f__need({ let t3071 = { let t3063 = Py::Int(2i64); Py::Bool({ let t3064 = v_n.clone(); if py::le(&t3063, &t3064)? { { let t3065 = Py::Int(10i64); if py::le(&t3064, &t3065)? { true } else { false } } } else { false } }) }; if !t3071.truthy() { t3071 } else { { let t3070 = { let t3066 = py::len(&v_gamma.clone())?; Py::Bool({ let t3067 = v_n.clone(); if py::eq(&t3066, &t3067) { true } else { false } }) }; if !t3070.truthy() { t3070 } else { { let t3068 = py::len(&v_Psat.clone())?; Py::Bool({ let t3069 = v_n.clone(); if py::eq(&t3068, &t3069) { true } else { false } }) } } } } }, Py::str("x, gamma and Psat need one entry per component (2 to 10)"))?;
    let _ = f__fracs(Py::str("x"), v_x.clone(), Py::Float(1e-06))?;
    for t3079 in py::iter(&(v_gamma.clone()))? {
        v_g = t3079;
        let _ = f__need({ let t3083 = py::math1("isfinite", &v_g.clone())?; if !t3083.truthy() { t3083 } else { { let t3080 = Py::Int(0i64); Py::Bool({ let t3081 = v_g.clone(); if py::lt(&t3080, &t3081)? { { let t3082 = Py::Int(100i64); if py::le(&t3081, &t3082)? { true } else { false } } } else { false } }) } } }, Py::str("every activity coefficient must lie in (0, 100]"))?;
    }
    for t3084 in py::iter(&(v_Psat.clone()))? {
        v_p = t3084;
        let _ = f__need({ let t3087 = py::math1("isfinite", &v_p.clone())?; if !t3087.truthy() { t3087 } else { { let t3085 = v_p.clone(); Py::Bool({ let t3086 = Py::Int(0i64); if py::gt(&t3085, &t3086)? { true } else { false } }) } } }, Py::str("every vapour pressure must be positive"))?;
    }
    v_f = py::getitem(&c__PUNIT()?, &v_p_unit.clone())?;
    v_parts = { let mut out = vec![]; for t3088 in py::iter(&(py::zip(&[v_x.clone(), v_gamma.clone(), v_Psat.clone()])?))? { let mut v_gi: Py; let mut v_pi: Py; let mut v_xi: Py; 
let t3089 = (t3088).items()?;
if t3089.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_xi = t3089[0].clone();
v_gi = t3089[1].clone();
v_pi = t3089[2].clone();
out.push(py::mul(&py::mul(&py::mul(&v_xi.clone(), &v_gi.clone())?, &v_pi.clone())?, &v_f.clone())?); } Py::list(out) };
    v_P = py::sum(&v_parts.clone(), None)?;
    v_y = { let mut out = vec![]; for t3090 in py::iter(&(v_parts.clone()))? { let mut v_v: Py; 
v_v = t3090;
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
    let _ = f__need({ let t3232 = v_p_v.clone(); Py::Bool({ let t3233 = Py::Int(0i64); if py::ge(&t3232, &t3233)? { true } else { false } }) }, Py::str("the vapour pressure must be zero or positive"))?;
    let _ = f__fin(Py::tuple(vec![v_z_s.clone(), v_h_f.clone()]))?;
    let _ = f__need({ let t3237 = v_h_f.clone(); Py::Bool({ let t3238 = Py::Int(0i64); if py::ge(&t3237, &t3238)? { true } else { false } }) }, Py::str("the suction friction head loss must be zero or positive"))?;
    v_N = py::sub(&py::add(&py::truediv(&py::sub(&v_p_s.clone(), &v_p_v.clone())?, &py::mul(&v_rho.clone(), &c_G0()?)?)?, &v_z_s.clone())?, &v_h_f.clone())?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("NPSHa"), v_N.clone()), (Py::str("pressure_head"), py::truediv(&v_p_s.clone(), &py::mul(&v_rho.clone(), &c_G0()?)?)?), (Py::str("vapour_head"), py::truediv(&v_p_v.clone(), &py::mul(&v_rho.clone(), &c_G0()?)?)?)])?;
    if ({ let t3239 = v_NPSHr.clone(); Py::Bool({ let t3240 = Py::None; if !py::is(&t3239, &t3240) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t3241 = v_NPSHr.clone(); Py::Bool({ let t3242 = Py::Int(0i64); if py::ge(&t3241, &t3242)? { true } else { false } }) }, Py::str("the required NPSH must be zero or positive"))?;
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
    v_G = { let mut out = vec![]; for t3306 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3306;
out.push({ let mut out = vec![]; for t3307 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3307;
out.push(py::math1("exp", &py::mul(&py::neg(&py::getitem(&py::getitem(&v_alpha.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&py::getitem(&v_tau.clone(), &v_i.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_D = { let mut out = vec![]; for t3308 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3308;
out.push(py::sum(&{ let mut out = vec![]; for t3309 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3309;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_G.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_N = { let mut out = vec![]; for t3310 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3310;
out.push(py::sum(&{ let mut out = vec![]; for t3311 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3311;
out.push(py::mul(&py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_tau.clone(), &v_k.clone())?, &v_j.clone())?)?, &py::getitem(&py::getitem(&v_G.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_lg = { let mut out = vec![]; for t3312 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3312;
out.push(py::add(&py::truediv(&py::getitem(&v_N.clone(), &v_i.clone())?, &py::getitem(&v_D.clone(), &v_i.clone())?)?, &py::sum(&{ let mut out = vec![]; for t3313 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3313;
out.push(py::mul(&py::truediv(&py::mul(&py::getitem(&v_x.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_G.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_D.clone(), &v_j.clone())?)?, &py::sub(&py::getitem(&py::getitem(&v_tau.clone(), &v_i.clone())?, &v_j.clone())?, &py::truediv(&py::getitem(&v_N.clone(), &v_j.clone())?, &py::getitem(&v_D.clone(), &v_j.clone())?)?)?)?); } Py::list(out) }, None)?)?); } Py::list(out) };
    v_g = { let mut out = vec![]; for t3314 in py::iter(&(v_lg.clone()))? { let mut v_v: Py; 
v_v = t3314;
out.push(py::math1("exp", &v_v.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("gamma1"), py::getitem(&v_g.clone(), &Py::Int(0i64))?), (Py::str("gamma"), v_g.clone()), (Py::str("GE_RT"), py::sum(&{ let mut out = vec![]; for t3327 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3327;
out.push(py::truediv(&py::mul(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::getitem(&v_N.clone(), &v_i.clone())?)?, &py::getitem(&v_D.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_p_offset(mut v_Kc: Py, mut v_K: Py, mut v_taus: Py, mut v_t_unit: Py) -> R {
    let mut v_char: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_Kc.clone(), v_K.clone()]))?;
    let _ = f__need({ let t3357 = py::mul(&v_Kc.clone(), &v_K.clone())?; Py::Bool({ let t3358 = Py::Int(0i64); if py::gt(&t3357, &t3358)? { true } else { false } }) }, Py::str("the loop gain Kc K must be positive (negative feedback)"))?;
    let _ = f__need({ let t3359 = Py::Int(1i64); Py::Bool({ let t3360 = py::len(&v_taus.clone())?; if py::le(&t3359, &t3360)? { { let t3361 = Py::Int(6i64); if py::le(&t3360, &t3361)? { true } else { false } } } else { false } }) }, Py::str("give 1 to 6 time constants"))?;
    let _ = f__pos(Py::str("the time constants"), { let mut v = vec![]; v.extend((v_taus.clone()).items()?); Py::tuple(v) })?;
    v_char = f__lags_poly({ let mut out = vec![]; for t3375 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t3375;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) })?;
    let t3376 = v_char.clone(); let t3377 = py::neg(&Py::Int(1i64))?;
    py::setitem(&t3376, &t3377, py::iadd(&py::getitem(&t3376, &t3377)?, &py::mul(&v_Kc.clone(), &v_K.clone())?)?)?;
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
    let _ = f__need({ let t3941 = Py::Int(1i64); Py::Bool({ let t3942 = py::len(&v_e.clone())?; if py::le(&t3941, &t3942)? { { let t3943 = Py::Int(500i64); if py::le(&t3942, &t3943)? { true } else { false } } } else { false } }) }, Py::str("give 1 to 500 error samples"))?;
    let t3944 = (Py::tuple(vec![Py::list(vec![]), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3944.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t3944[0].clone();
    v_s = t3944[1].clone();
    v_prev = t3944[2].clone();
    for t3945 in py::iter(&(v_e.clone()))? {
        v_ek = t3945;
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
    let _ = f__need({ let t3993 = v_T.clone(); Py::Bool({ let t3994 = v_Tc.clone(); if py::lt(&t3993, &t3994)? { true } else { false } }) }, Py::str("T must be below the critical temperature"))?;
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
    let _ = f__need({ let t4043 = { let t4039 = v_p1.clone(); Py::Bool({ let t4040 = Py::Int(0i64); if py::gt(&t4039, &t4040)? { true } else { false } }) }; if !t4043.truthy() { t4043 } else { { let t4041 = v_p2.clone(); Py::Bool({ let t4042 = Py::Int(0i64); if py::gt(&t4041, &t4042)? { true } else { false } }) } } }, Py::str("pressures are absolute and must be positive"))?;
    let _ = f__fin(Py::tuple(vec![v_dz.clone(), v_v1.clone(), v_v2.clone()]))?;
    let _ = f__need({ let t4051 = { let t4047 = v_v1.clone(); Py::Bool({ let t4048 = Py::Int(0i64); if py::ge(&t4047, &t4048)? { true } else { false } }) }; if !t4051.truthy() { t4051 } else { { let t4049 = v_v2.clone(); Py::Bool({ let t4050 = Py::Int(0i64); if py::ge(&t4049, &t4050)? { true } else { false } }) } } }, Py::str("the speeds must be zero or positive"))?;
    v_H = py::add(&py::add(&py::truediv(&py::sub(&v_p2.clone(), &v_p1.clone())?, &py::mul(&v_rho.clone(), &c_G0()?)?)?, &v_dz.clone())?, &py::truediv(&py::sub(&py::mul(&v_v2.clone(), &v_v2.clone())?, &py::mul(&v_v1.clone(), &v_v1.clone())?)?, &py::mul(&Py::Int(2i64), &c_G0()?)?)?)?;
    let _ = f__need({ let t4052 = v_H.clone(); Py::Bool({ let t4053 = Py::Int(0i64); if py::gt(&t4052, &t4053)? { true } else { false } }) }, Py::str("the pump must add head (H > 0)"))?;
    v_out = py::dict_from(&Py::dict(vec![]), vec![(Py::str("H"), v_H.clone()), (Py::str("dp_equivalent"), py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_H.clone())?)])?;
    if ({ let t4054 = v_Q.clone(); Py::Bool({ let t4055 = Py::None; if !py::is(&t4054, &t4055) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("Q"), Py::tuple(vec![v_Q.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("P_hydraulic"), py::mul(&py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_Q.clone())?, &v_H.clone())?)?;
        if ({ let t4056 = v_eta.clone(); Py::Bool({ let t4057 = Py::None; if !py::is(&t4056, &t4057) { true } else { false } }) }).truthy() {
            let _ = f__need({ let t4058 = Py::Int(0i64); Py::Bool({ let t4059 = v_eta.clone(); if py::lt(&t4058, &t4059)? { { let t4060 = Py::Int(1i64); if py::le(&t4059, &t4060)? { true } else { false } } } else { false } }) }, Py::str("the pump efficiency must lie in (0, 1]"))?;
            py::setitem(&v_out.clone(), &Py::str("P_shaft"), py::truediv(&py::mul(&py::mul(&py::mul(&v_rho.clone(), &c_G0()?)?, &v_Q.clone())?, &v_H.clone())?, &v_eta.clone())?)?;
        }
    } else {
        if ({ let t4061 = v_eta.clone(); Py::Bool({ let t4062 = Py::None; if !py::is(&t4061, &t4062) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t4117 = { let t4112 = py::len(&v_K.clone())?; Py::Bool({ let t4113 = py::len(&v_z.clone())?; if py::eq(&t4112, &t4113) { true } else { false } }) }; if !t4117.truthy() { t4117 } else { Py::Bool(({ let mut out = vec![]; for t4114 in py::iter(&(v_K.clone()))? { let mut v_k: Py; 
v_k = t4114;
out.push({ let t4115 = v_k.clone(); Py::Bool({ let t4116 = Py::Int(0i64); if py::gt(&t4115, &t4116)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("give one positive K-value per component"))?;
    let t4168 = (f__rr(v_z.clone(), v_K.clone())?).items()?;
    if t4168.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t4168[0].clone();
    v_x = t4168[1].clone();
    v_y = t4168[2].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("VF"), v_V.clone()), (Py::str("x"), v_x.clone()), (Py::str("y"), v_y.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rackett(mut v_T: Py, mut v_Tc: Py, mut v_Pc: Py, mut v_Zc: Py) -> R {
    let _ = f__pos(Py::str("T, Tc, Pc and Zc"), Py::tuple(vec![v_T.clone(), v_Tc.clone(), v_Pc.clone(), v_Zc.clone()]))?;
    let _ = f__need({ let t4217 = { let t4213 = v_T.clone(); Py::Bool({ let t4214 = v_Tc.clone(); if py::lt(&t4213, &t4214)? { true } else { false } }) }; if !t4217.truthy() { t4217 } else { { let t4215 = v_Zc.clone(); Py::Bool({ let t4216 = Py::Int(1i64); if py::lt(&t4215, &t4216)? { true } else { false } }) } } }, Py::str("T must be below Tc and Zc below 1"))?;
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
    let _ = f__need({ let t4265 = v_re.clone(); Py::Bool({ let t4266 = v_rw.clone(); if py::gt(&t4265, &t4266)? { true } else { false } }) }, Py::str("the drainage radius re must exceed the well radius rw"))?;
    let _ = f__need({ let t4267 = v_pe.clone(); Py::Bool({ let t4268 = v_pw.clone(); if py::gt(&t4267, &t4268)? { true } else { false } }) }, Py::str("the boundary pressure must exceed the flowing well pressure"))?;
    v_d = py::add(&py::math1("log", &py::truediv(&v_re.clone(), &v_rw.clone())?)?, &v_S.clone())?;
    let _ = f__need({ let t4269 = v_d.clone(); Py::Bool({ let t4270 = Py::Int(0i64); if py::gt(&t4269, &t4270)? { true } else { false } }) }, Py::str("the skin is so negative that the flow resistance vanishes"))?;
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
    v_K = { let mut out = vec![]; for t4710 in py::iter(&(v_ant.clone()))? { let mut v_r: Py; 
v_r = t4710;
out.push(py::truediv(&f__psat(v_r.clone(), v_units.clone(), v_base.clone(), v_T.clone())?, &v_P.clone())?); } Py::list(out) };
    let t4765 = (f__rr(v_z.clone(), v_K.clone())?).items()?;
    if t4765.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t4765[0].clone();
    v_x = t4765[1].clone();
    v_y = t4765[2].clone();
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
    v_dco = { let mut out = vec![]; for t4820 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_j: Py; 
v_j = t4820;
out.push(py::sum(&{ let mut out = vec![]; for t4821 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4821;
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
    let t4879 = (Py::tuple(vec![py::mul(&py::sub(&Py::Float(1.0), &v_X.clone())?, &v_A_in.clone())?, v_I_in.clone()])).items()?;
    if t4879.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S_A = t4879[0].clone();
    v_S_I = t4879[1].clone();
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
    let _ = f__need({ let t4927 = Py::Int(0i64); Py::Bool({ let t4928 = v_R.clone(); if py::le(&t4927, &t4928)? { { let t4929 = Py::Float(10000.0); if py::le(&t4928, &t4929)? { true } else { false } } } else { false } }) }, Py::str("the recycle ratio R must be zero or positive"))?;
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
    let _ = f__need({ let t5024 = v_Tb.clone(); Py::Bool({ let t5025 = py::mul(&Py::Float(0.93), &v_Tc.clone())?; if py::lt(&t5024, &t5025)? { true } else { false } }) }, Py::str("the normal boiling point must lie well below Tc"))?;
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
    let _ = f__need({ let t5068 = { let t5064 = py::len(&v_t.clone())?; Py::Bool({ let t5065 = py::len(&v_C.clone())?; if py::eq(&t5064, &t5065) { true } else { false } }) }; if !t5068.truthy() { t5068 } else { { let t5066 = py::len(&v_t.clone())?; Py::Bool({ let t5067 = Py::Int(3i64); if py::ge(&t5066, &t5067)? { true } else { false } }) } } }, Py::str("t and C need the same number of points (at least 3)"))?;
    v_f = py::getitem(&Py::dict(vec![(Py::str("s"), Py::Float(1.0)), (Py::str("min"), Py::Float(60.0)), (Py::str("h"), Py::Float(3600.0))]), &v_t_unit.clone())?;
    v_ts = { let mut out = vec![]; for t5069 in py::iter(&(v_t.clone()))? { let mut v_x: Py; 
v_x = t5069;
out.push(py::mul(&v_x.clone(), &v_f.clone())?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5070 in py::iter(&(py::zip(&[v_ts.clone(), py::slice(&v_ts.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5071 = (t5070).items()?;
if t5071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5071[0].clone();
v_b = t5071[1].clone();
out.push({ let t5072 = v_b.clone(); Py::Bool({ let t5073 = v_a.clone(); if py::gt(&t5072, &t5073)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("times must increase"))?;
    let _ = f__need({ let t5080 = Py::Bool(({ let mut out = vec![]; for t5074 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t5074;
out.push({ let t5075 = v_c.clone(); Py::Bool({ let t5076 = Py::Int(0i64); if py::ge(&t5075, &t5076)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())); if !t5080.truthy() { t5080 } else { Py::Bool(({ let mut out = vec![]; for t5077 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t5077;
out.push({ let t5078 = v_c.clone(); Py::Bool({ let t5079 = Py::Int(0i64); if py::gt(&t5078, &t5079)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }, Py::str("tracer concentrations must be zero or positive, not all zero"))?;
    v_trap = { let v_C = v_C.clone(); let v_f = v_f.clone(); let v_t = v_t.clone(); let v_t_unit = v_t_unit.clone(); let v_ts = v_ts.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t5081 in py::range_iter(&[(py::sub(&py::len(&v_ts.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t5081;
out.push(py::truediv(&py::mul(&py::sub(&py::getitem(&v_ts.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_ts.clone(), &v_i.clone())?)?, &py::add(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &Py::Float(2.0))?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_area = py::call(&v_trap.clone(), &[v_C.clone()])?;
    v_tm = py::truediv(&py::call(&v_trap.clone(), &[{ let mut out = vec![]; for t5082 in py::iter(&(py::zip(&[v_ts.clone(), v_C.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5083 = (t5082).items()?;
if t5083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5083[0].clone();
v_b = t5083[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }])?, &v_area.clone())?;
    v_var = py::truediv(&py::call(&v_trap.clone(), &[{ let mut out = vec![]; for t5084 in py::iter(&(py::zip(&[v_ts.clone(), v_C.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5085 = (t5084).items()?;
if t5085.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5085[0].clone();
v_b = t5085[1].clone();
out.push(py::mul(&py::pow(&py::sub(&v_a.clone(), &v_tm.clone())?, &Py::Int(2i64))?, &v_b.clone())?); } Py::list(out) }])?, &v_area.clone())?;
    let _ = f__need({ let t5086 = v_var.clone(); Py::Bool({ let t5087 = Py::Int(0i64); if py::gt(&t5086, &t5087)? { true } else { false } }) }, Py::str("the tracer response has no spread"))?;
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
    let _ = f__need({ let t5135 = v_T1.clone(); Py::Bool({ let t5136 = v_T2.clone(); if py::ne(&t5135, &t5136) { true } else { false } }) }, Py::str("T1 and T2 must differ"))?;
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
    if ({ let t5182 = py::abs(&py::sub(&v_k1.clone(), &v_k2.clone())?)?; Py::Bool({ let t5183 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_k1.clone(), v_k2.clone()], true)?)?; if py::le(&t5182, &t5183)? { true } else { false } }) }).truthy() {
        v_k = py::mul(&Py::Float(0.5), &py::add(&v_k1.clone(), &v_k2.clone())?)?;
        let t5184 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &v_k.clone())?, py::truediv(&v_CA0.clone(), &Py::Float(std::f64::consts::E))?])).items()?;
        if t5184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t = t5184[0].clone();
        v_CB = t5184[1].clone();
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
    let _ = f__need({ let t5321 = v_yA1.clone(); Py::Bool({ let t5322 = v_yA2.clone(); if py::gt(&t5321, &t5322)? { true } else { false } }) }, Py::str("A diffuses from yA1 to yA2: yA1 must exceed yA2"))?;
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
    let _ = f__need({ let t3137 = { let t3129 = Py::Int(2i64); Py::Bool({ let t3130 = v_n.clone(); if py::le(&t3129, &t3130)? { { let t3131 = Py::Int(10i64); if py::le(&t3130, &t3131)? { true } else { false } } } else { false } }) }; if !t3137.truthy() { t3137 } else { { let t3136 = { let t3132 = py::len(&v_z.clone())?; Py::Bool({ let t3133 = v_n.clone(); if py::eq(&t3132, &t3133) { true } else { false } }) }; if !t3136.truthy() { t3136 } else { { let t3134 = py::len(&v_xD.clone())?; Py::Bool({ let t3135 = v_n.clone(); if py::eq(&t3134, &t3135) { true } else { false } }) } } } } }, Py::str("alpha, z and xD need one entry per component (2 to 10)"))?;
    let _ = f__need({ let t3149 = Py::Bool(matches!(v_lk.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3149.truthy() { t3149 } else { { let t3148 = Py::Bool(matches!(v_hk.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3148.truthy() { t3148 } else { { let t3147 = { let t3138 = Py::Int(1i64); Py::Bool({ let t3139 = v_lk.clone(); if py::le(&t3138, &t3139)? { { let t3140 = v_n.clone(); if py::le(&t3139, &t3140)? { true } else { false } } } else { false } }) }; if !t3147.truthy() { t3147 } else { { let t3146 = { let t3141 = Py::Int(1i64); Py::Bool({ let t3142 = v_hk.clone(); if py::le(&t3141, &t3142)? { { let t3143 = v_n.clone(); if py::le(&t3142, &t3143)? { true } else { false } } } else { false } }) }; if !t3146.truthy() { t3146 } else { { let t3144 = v_lk.clone(); Py::Bool({ let t3145 = v_hk.clone(); if py::ne(&t3144, &t3145) { true } else { false } }) } } } } } } } } }, Py::str("lk and hk are component numbers (1 to n)"))?;
    for t3150 in py::iter(&(v_alpha.clone()))? {
        v_a = t3150;
        let _ = f__need({ let t3153 = py::math1("isfinite", &v_a.clone())?; if !t3153.truthy() { t3153 } else { { let t3151 = v_a.clone(); Py::Bool({ let t3152 = Py::Int(0i64); if py::gt(&t3151, &t3152)? { true } else { false } }) } } }, Py::str("every relative volatility must be positive"))?;
    }
    let _ = f__fracs(Py::str("z"), v_z.clone(), Py::Float(1e-06))?;
    let _ = f__fracs(Py::str("xD"), v_xD.clone(), Py::Float(1e-06))?;
    let _ = f__need({ let t3161 = py::neg(&Py::Int(5i64))?; Py::Bool({ let t3162 = v_q.clone(); if py::le(&t3161, &t3162)? { { let t3163 = Py::Int(5i64); if py::le(&t3162, &t3163)? { true } else { false } } } else { false } }) }, Py::str("the feed quality q must lie between -5 and 5"))?;
    let t3164 = (Py::tuple(vec![py::getitem(&v_alpha.clone(), &py::sub(&v_lk.clone(), &Py::Int(1i64))?)?, py::getitem(&v_alpha.clone(), &py::sub(&v_hk.clone(), &Py::Int(1i64))?)?])).items()?;
    if t3164.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a_lk = t3164[0].clone();
    v_a_hk = t3164[1].clone();
    let _ = f__need({ let t3165 = v_a_lk.clone(); Py::Bool({ let t3166 = v_a_hk.clone(); if py::gt(&t3165, &t3166)? { true } else { false } }) }, Py::str("the light key must be more volatile than the heavy key"))?;
    let _ = f__need(py::not(&Py::Bool(({ let mut out = vec![]; for t3167 in py::iter(&(v_alpha.clone()))? { let mut v_a: Py; 
v_a = t3167;
out.push({ let t3168 = v_a_hk.clone(); Py::Bool({ let t3169 = v_a.clone(); if py::lt(&t3168, &t3169)? { { let t3170 = v_a_lk.clone(); if py::lt(&t3169, &t3170)? { true } else { false } } } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))), Py::str("the keys must be adjacent in volatility (no component between them)"))?;
    let _ = f__need({ let t3171 = py::len(&py::set(&v_alpha.clone())?)?; Py::Bool({ let t3172 = v_n.clone(); if py::eq(&t3171, &t3172) { true } else { false } }) }, Py::str("relative volatilities must be distinct"))?;
    v_th = f__underwood_theta(v_alpha.clone(), v_z.clone(), v_q.clone(), v_a_hk.clone(), v_a_lk.clone())?;
    v_Rm = py::sub(&py::sum(&{ let mut out = vec![]; for t3184 in py::iter(&(py::zip(&[v_alpha.clone(), v_xD.clone()])?))? { let mut v_a: Py; let mut v_x: Py; 
let t3185 = (t3184).items()?;
if t3185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3185[0].clone();
v_x = t3185[1].clone();
out.push(py::truediv(&py::mul(&v_a.clone(), &v_x.clone())?, &py::sub(&v_a.clone(), &v_th.clone())?)?); } Py::list(out) }, None)?, &Py::Float(1.0))?;
    let _ = f__need({ let t3186 = v_Rm.clone(); Py::Bool({ let t3187 = Py::Int(0i64); if py::gt(&t3186, &t3187)? { true } else { false } }) }, Py::str("no positive minimum reflux for this distillate"))?;
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
    let _ = f__need({ let t5374 = py::len(&v_r.clone())?; Py::Bool({ let t5375 = py::len(&v_q.clone())?; if py::eq(&t5374, &t5375) { { let t5376 = v_n.clone(); if py::eq(&t5375, &t5376) { true } else { false } } } else { false } }) }, Py::str("give r and q for every component"))?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("r and q")); v.extend((v_r.clone()).items()?); v.extend((v_q.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_pos() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_pos() got an unexpected keyword argument".into())) } f__pos(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    let _ = f__square(Py::str("tau"), v_tau.clone(), v_n.clone(), Py::Float(1.0))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5394 in py::iter(&(v_tau.clone()))? { let mut v_row: Py; 
v_row = t5394;
for t5395 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t5395;
out.push({ let t5396 = v_v.clone(); Py::Bool({ let t5397 = Py::Int(0i64); if py::gt(&t5396, &t5397)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("UNIQUAC tau must be positive"))?;
    let t5402 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t5398 in py::iter(&(py::zip(&[v_r.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5399 = (t5398).items()?;
if t5399.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5399[0].clone();
v_b = t5399[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t5400 in py::iter(&(py::zip(&[v_q.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5401 = (t5400).items()?;
if t5401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5401[0].clone();
v_b = t5401[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?])).items()?;
    if t5402.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sr = t5402[0].clone();
    v_sq = t5402[1].clone();
    v_phi = { let mut out = vec![]; for t5403 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5403;
out.push(py::truediv(&py::mul(&py::getitem(&v_r.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_sr.clone())?); } Py::list(out) };
    v_th = { let mut out = vec![]; for t5404 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5404;
out.push(py::truediv(&py::mul(&py::getitem(&v_q.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_sq.clone())?); } Py::list(out) };
    v_l = { let mut out = vec![]; for t5405 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5405;
out.push(py::sub(&py::mul(&Py::Float(5.0), &py::sub(&py::getitem(&v_r.clone(), &v_i.clone())?, &py::getitem(&v_q.clone(), &v_i.clone())?)?)?, &py::sub(&py::getitem(&v_r.clone(), &v_i.clone())?, &Py::Float(1.0))?)?); } Py::list(out) };
    v_sl = py::sum(&{ let mut out = vec![]; for t5406 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5406;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_j.clone())?, &py::getitem(&v_l.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?;
    v_S = { let mut out = vec![]; for t5407 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5407;
out.push(py::sum(&{ let mut out = vec![]; for t5408 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5408;
out.push(py::mul(&py::getitem(&v_th.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_tau.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_lg = Py::list(vec![]);
    for t5409 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5409;
        v_comb = py::sub(&py::add(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_phi.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?)?, &py::mul(&py::mul(&Py::Float(5.0), &py::getitem(&v_q.clone(), &v_i.clone())?)?, &py::math1("log", &py::truediv(&py::getitem(&v_th.clone(), &v_i.clone())?, &py::getitem(&v_phi.clone(), &v_i.clone())?)?)?)?)?, &py::getitem(&v_l.clone(), &v_i.clone())?)?, &py::mul(&py::truediv(&py::getitem(&v_phi.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &v_sl.clone())?)?;
        v_res = py::mul(&py::getitem(&v_q.clone(), &v_i.clone())?, &py::sub(&py::sub(&Py::Float(1.0), &py::math1("log", &py::getitem(&v_S.clone(), &v_i.clone())?)?)?, &py::sum(&{ let mut out = vec![]; for t5410 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5410;
out.push(py::truediv(&py::mul(&py::getitem(&v_th.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_tau.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_S.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?)?)?;
        let _ = py::method(&v_lg.clone(), "append", &[py::add(&v_comb.clone(), &v_res.clone())?])?;
    }
    v_g = { let mut out = vec![]; for t5411 in py::iter(&(v_lg.clone()))? { let mut v_v: Py; 
v_v = t5411;
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
    let _ = f__need({ let t5512 = v_V.clone(); Py::Bool({ let t5513 = Py::Int(0i64); if py::gt(&t5512, &t5513)? { true } else { false } }) }, Py::str("the tank would be empty before time t"))?;
    if ({ let t5514 = v_q_in.clone(); Py::Bool({ let t5515 = v_q_out.clone(); if py::eq(&t5514, &t5515) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t5613 = { let t5609 = v_T.clone(); Py::Bool({ let t5610 = v_Tc.clone(); if py::lt(&t5609, &t5610)? { true } else { false } }) }; if !t5613.truthy() { t5613 } else { { let t5611 = v_T_ref.clone(); Py::Bool({ let t5612 = v_Tc.clone(); if py::lt(&t5611, &t5612)? { true } else { false } }) } } }, Py::str("both temperatures must be below the critical temperature"))?;
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
    let _ = f__need({ let t5734 = py::len(&v_Tc.clone())?; Py::Bool({ let t5735 = py::len(&v_Pc.clone())?; if py::eq(&t5734, &t5735) { { let t5736 = py::len(&v_omega.clone())?; if py::eq(&t5735, &t5736) { { let t5737 = v_n.clone(); if py::eq(&t5736, &t5737) { true } else { false } } } else { false } } } else { false } }) }, Py::str("give Tc, Pc and omega for every component"))?;
    let _ = { let mut v: Vec<Py> = vec![]; v.push(Py::str("Tc and Pc")); v.extend((v_Tc.clone()).items()?); v.extend((v_Pc.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "name") { Some(x) => x, None => { return Err(PyErr::Type("_pos() missing argument name".into())) } } }; if !d.is_empty() { return Err(PyErr::Type("_pos() got an unexpected keyword argument".into())) } f__pos(a0, Py::tuple(v.get(1..).map(|s| s.to_vec()).unwrap_or_default()))? };
    let _ = f__pos(Py::str("T and P"), Py::tuple(vec![v_T.clone(), v_P.clone()]))?;
    v_pcs = { let mut out = vec![]; for t5744 in py::iter(&(v_Pc.clone()))? { let mut v_p: Py; 
v_p = t5744;
out.push(py::mul(&v_p.clone(), &py::getitem(&c__PUNIT()?, &v_p_unit.clone())?)?); } Py::list(out) };
    v_K = { let mut out = vec![]; for t5745 in py::iter(&(py::zip(&[v_Tc.clone(), v_pcs.clone(), v_omega.clone()])?))? { let mut v_pc: Py; let mut v_tc: Py; let mut v_w: Py; 
let t5746 = (t5745).items()?;
if t5746.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_tc = t5746[0].clone();
v_pc = t5746[1].clone();
v_w = t5746[2].clone();
out.push(py::mul(&py::truediv(&v_pc.clone(), &v_P.clone())?, &py::math1("exp", &py::mul(&py::mul(&Py::Float(5.37), &py::add(&Py::Float(1.0), &v_w.clone())?)?, &py::sub(&Py::Float(1.0), &py::truediv(&v_tc.clone(), &v_T.clone())?)?)?)?)?); } Py::list(out) };
    let t5797 = (f__rr(v_z.clone(), v_K.clone())?).items()?;
    if t5797.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_V = t5797[0].clone();
    v_x = t5797[1].clone();
    v_y = t5797[2].clone();
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
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5676 in py::iter(&(v_Lam.clone()))? { let mut v_r: Py; 
v_r = t5676;
for t5677 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t5677;
out.push({ let t5678 = v_v.clone(); Py::Bool({ let t5679 = Py::Int(0i64); if py::gt(&t5678, &t5679)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("Wilson parameters must be positive"))?;
    v_S = { let mut out = vec![]; for t5680 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5680;
out.push(py::sum(&{ let mut out = vec![]; for t5681 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5681;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_j.clone())?, &py::getitem(&py::getitem(&v_Lam.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_lg = { let mut out = vec![]; for t5682 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5682;
out.push(py::sub(&py::sub(&Py::Float(1.0), &py::math1("log", &py::getitem(&v_S.clone(), &v_i.clone())?)?)?, &py::sum(&{ let mut out = vec![]; for t5683 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5683;
out.push(py::truediv(&py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_Lam.clone(), &v_k.clone())?, &v_i.clone())?)?, &py::getitem(&v_S.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?); } Py::list(out) };
    v_g = { let mut out = vec![]; for t5684 in py::iter(&(v_lg.clone()))? { let mut v_v: Py; 
v_v = t5684;
out.push(py::math1("exp", &v_v.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("gamma1"), py::getitem(&v_g.clone(), &Py::Int(0i64))?), (Py::str("gamma"), v_g.clone()), (Py::str("GE_RT"), py::neg(&py::sum(&{ let mut out = vec![]; for t5697 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5697;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_i.clone())?, &py::math1("log", &py::getitem(&v_S.clone(), &v_i.clone())?)?)?); } Py::list(out) }, None)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ziegler_nichols(mut v_K: Py, mut v_taus: Py, mut v_t_unit: Py, mut v_controller: Py) -> R {
    let mut v_ku: Py = Py::None;
    let mut v_pu: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_K.clone()]))?;
    let _ = f__need({ let t5839 = v_K.clone(); Py::Bool({ let t5840 = Py::Int(0i64); if py::gt(&t5839, &t5840)? { true } else { false } }) }, Py::str("the process gain must be positive"))?;
    let _ = f__need({ let t5841 = Py::Int(3i64); Py::Bool({ let t5842 = py::len(&v_taus.clone())?; if py::le(&t5841, &t5842)? { { let t5843 = Py::Int(6i64); if py::le(&t5842, &t5843)? { true } else { false } } } else { false } }) }, Py::str("give 3 to 6 time constants"))?;
    let _ = f__pos(Py::str("the time constants"), { let mut v = vec![]; v.extend((v_taus.clone()).items()?); Py::tuple(v) })?;
    let t5883 = (f__ultimate(v_K.clone(), { let mut out = vec![]; for t5882 in py::iter(&(v_taus.clone()))? { let mut v_t: Py; 
v_t = t5882;
out.push(py::mul(&v_t.clone(), &py::getitem(&c__TU()?, &v_t_unit.clone())?)?); } Py::list(out) })?).items()?;
    if t5883.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ku = t5883[0].clone();
    v_pu = t5883[1].clone();
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
fn s_1697(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1699: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1698 = v_inputs.clone(); f_cost_scaling(py::getitem(&t1698, &Py::str("C1"))?, py::getitem(&t1698, &Py::str("S1"))?, py::getitem(&t1698, &Py::str("S2"))?, py::getitem(&t1698, &Py::str("n"))?, py::method(&t1698, "get", &[Py::str("I1"), Py::None])?, py::method(&t1698, "get", &[Py::str("I2"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1699 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1700 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1722 = (t1700).items()?;
        if t1722.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1722[0].clone();
        v_v = t1722[1].clone();
        if ({ let t1701 = v_k.clone(); Py::Bool({ let t1702 = v_outputs.clone(); if !py::contains(&t1702, &t1701)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1753(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1755: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1754 = v_inputs.clone(); f_ergun(py::getitem(&t1754, &Py::str("dp"))?, py::getitem(&t1754, &Py::str("eps"))?, py::getitem(&t1754, &Py::str("vs"))?, py::getitem(&t1754, &Py::str("rho"))?, py::getitem(&t1754, &Py::str("mu"))?, py::getitem(&t1754, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1755 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1756 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1778 = (t1756).items()?;
        if t1778.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1778[0].clone();
        v_v = t1778[1].clone();
        if ({ let t1757 = v_k.clone(); Py::Bool({ let t1758 = v_outputs.clone(); if !py::contains(&t1758, &t1757)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1805: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1804 = v_inputs.clone(); f_decline_exp(py::getitem(&t1804, &Py::str("qi"))?, py::getitem(&t1804, &Py::str("Di"))?, py::getitem(&t1804, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1805 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1806 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1828 = (t1806).items()?;
        if t1828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1828[0].clone();
        v_v = t1828[1].clone();
        if ({ let t1807 = v_k.clone(); Py::Bool({ let t1808 = v_outputs.clone(); if !py::contains(&t1808, &t1807)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1850(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1852: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1851 = v_inputs.clone(); f_fenske(py::getitem(&t1851, &Py::str("alpha"))?, py::getitem(&t1851, &Py::str("xD"))?, py::getitem(&t1851, &Py::str("xB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1852 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1853 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1875 = (t1853).items()?;
        if t1875.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1875[0].clone();
        v_v = t1875[1].clone();
        if ({ let t1854 = v_k.clone(); Py::Bool({ let t1855 = v_outputs.clone(); if !py::contains(&t1855, &t1854)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1919(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1921: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1920 = v_inputs.clone(); f_filtration_fit(py::getitem(&t1920, &Py::str("t"))?, py::getitem(&t1920, &Py::str("V"))?, py::getitem(&t1920, &Py::str("t_unit"))?, py::getitem(&t1920, &Py::str("v_unit"))?, py::getitem(&t1920, &Py::str("A"))?, py::getitem(&t1920, &Py::str("dP"))?, py::getitem(&t1920, &Py::str("mu"))?, py::getitem(&t1920, &Py::str("c"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1921 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1922 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1944 = (t1922).items()?;
        if t1944.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1944[0].clone();
        v_v = t1944[1].clone();
        if ({ let t1923 = v_k.clone(); Py::Bool({ let t1924 = v_outputs.clone(); if !py::contains(&t1924, &t1923)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2032(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2034: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2033 = v_inputs.clone(); f_fit_first_order(py::getitem(&t2033, &Py::str("t"))?, py::getitem(&t2033, &Py::str("y"))?, py::getitem(&t2033, &Py::str("t_unit"))?, py::getitem(&t2033, &Py::str("M"))?, py::getitem(&t2033, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2034 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2035 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2057 = (t2035).items()?;
        if t2057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2057[0].clone();
        v_v = t2057[1].clone();
        if ({ let t2036 = v_k.clone(); Py::Bool({ let t2037 = v_outputs.clone(); if !py::contains(&t2037, &t2036)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2087(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2089: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2088 = v_inputs.clone(); f_fopdt_step(py::getitem(&t2088, &Py::str("K"))?, py::getitem(&t2088, &Py::str("tau"))?, py::getitem(&t2088, &Py::str("theta"))?, py::getitem(&t2088, &Py::str("M"))?, py::getitem(&t2088, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2089 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2090 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2112 = (t2090).items()?;
        if t2112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2112[0].clone();
        v_v = t2112[1].clone();
        if ({ let t2091 = v_k.clone(); Py::Bool({ let t2092 = v_outputs.clone(); if !py::contains(&t2092, &t2091)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2155(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2157: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2156 = v_inputs.clone(); f_gas_pz(py::getitem(&t2156, &Py::str("p"))?, py::getitem(&t2156, &Py::str("z"))?, py::getitem(&t2156, &Py::str("Gp"))?, py::getitem(&t2156, &Py::str("p_unit"))?, py::getitem(&t2156, &Py::str("gp_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2157 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2158 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2180 = (t2158).items()?;
        if t2180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2180[0].clone();
        v_v = t2180[1].clone();
        if ({ let t2159 = v_k.clone(); Py::Bool({ let t2160 = v_outputs.clone(); if !py::contains(&t2160, &t2159)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2228(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2230: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2229 = v_inputs.clone(); f_backpressure(py::getitem(&t2229, &Py::str("q"))?, py::getitem(&t2229, &Py::str("pwf"))?, py::getitem(&t2229, &Py::str("q_unit"))?, py::getitem(&t2229, &Py::str("p_unit"))?, py::getitem(&t2229, &Py::str("pr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2230 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2231 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2253 = (t2231).items()?;
        if t2253.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2253[0].clone();
        v_v = t2253[1].clone();
        if ({ let t2232 = v_k.clone(); Py::Bool({ let t2233 = v_outputs.clone(); if !py::contains(&t2233, &t2232)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2278(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2280: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2279 = v_inputs.clone(); f_decline_harm(py::getitem(&t2279, &Py::str("qi"))?, py::getitem(&t2279, &Py::str("Di"))?, py::getitem(&t2279, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2280 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2281 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2303 = (t2281).items()?;
        if t2303.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2303[0].clone();
        v_v = t2303[1].clone();
        if ({ let t2282 = v_k.clone(); Py::Bool({ let t2283 = v_outputs.clone(); if !py::contains(&t2283, &t2282)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2344(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2346: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2345 = v_inputs.clone(); f_ho_gascap(py::getitem(&t2345, &Py::str("F"))?, py::getitem(&t2345, &Py::str("Eo"))?, py::getitem(&t2345, &Py::str("Eg"))?, py::getitem(&t2345, &Py::str("v_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2346 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2347 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2369 = (t2347).items()?;
        if t2369.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2369[0].clone();
        v_v = t2369[1].clone();
        if ({ let t2348 = v_k.clone(); Py::Bool({ let t2349 = v_outputs.clone(); if !py::contains(&t2349, &t2348)? { true } else { false } }) }).truthy() {
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
fn s_2402(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2404: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2403 = v_inputs.clone(); f_ho_oil(py::getitem(&t2403, &Py::str("F"))?, py::getitem(&t2403, &Py::str("Et"))?, py::getitem(&t2403, &Py::str("v_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2404 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2405 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2427 = (t2405).items()?;
        if t2427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2427[0].clone();
        v_v = t2427[1].clone();
        if ({ let t2406 = v_k.clone(); Py::Bool({ let t2407 = v_outputs.clone(); if !py::contains(&t2407, &t2406)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2469(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2471: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2470 = v_inputs.clone(); f_horner(py::getitem(&t2470, &Py::str("dt"))?, py::getitem(&t2470, &Py::str("pws"))?, py::getitem(&t2470, &Py::str("t_unit"))?, py::getitem(&t2470, &Py::str("p_unit"))?, py::getitem(&t2470, &Py::str("tp"))?, py::getitem(&t2470, &Py::str("q"))?, py::getitem(&t2470, &Py::str("mu"))?, py::getitem(&t2470, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2471 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2472 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2494 = (t2472).items()?;
        if t2494.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2494[0].clone();
        v_v = t2494[1].clone();
        if ({ let t2473 = v_k.clone(); Py::Bool({ let t2474 = v_outputs.clone(); if !py::contains(&t2474, &t2473)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2525(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2527: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2526 = v_inputs.clone(); f_decline_hyp(py::getitem(&t2526, &Py::str("qi"))?, py::getitem(&t2526, &Py::str("Di"))?, py::getitem(&t2526, &Py::str("b"))?, py::getitem(&t2526, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2527 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2528 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2550 = (t2528).items()?;
        if t2550.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2550[0].clone();
        v_v = t2550[1].clone();
        if ({ let t2529 = v_k.clone(); Py::Bool({ let t2530 = v_outputs.clone(); if !py::contains(&t2530, &t2529)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2583(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2585: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2584 = v_inputs.clone(); f_kremser_absorber(py::getitem(&t2584, &Py::str("N"))?, py::getitem(&t2584, &Py::str("A"))?, py::getitem(&t2584, &Py::str("m"))?, py::getitem(&t2584, &Py::str("y_in"))?, py::getitem(&t2584, &Py::str("x_in"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2585 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2586 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2608 = (t2586).items()?;
        if t2608.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2608[0].clone();
        v_v = t2608[1].clone();
        if ({ let t2587 = v_k.clone(); Py::Bool({ let t2588 = v_outputs.clone(); if !py::contains(&t2588, &t2587)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2641(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2643: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2642 = v_inputs.clone(); f_kremser_stripper(py::getitem(&t2642, &Py::str("N"))?, py::getitem(&t2642, &Py::str("S"))?, py::getitem(&t2642, &Py::str("m"))?, py::getitem(&t2642, &Py::str("x_in"))?, py::getitem(&t2642, &Py::str("y_in"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2643 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2644 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2666 = (t2644).items()?;
        if t2666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2666[0].clone();
        v_v = t2666[1].clone();
        if ({ let t2645 = v_k.clone(); Py::Bool({ let t2646 = v_outputs.clone(); if !py::contains(&t2646, &t2645)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2696(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2698: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2697 = v_inputs.clone(); f_lag_freq(py::getitem(&t2697, &Py::str("K"))?, py::getitem(&t2697, &Py::str("taus"))?, py::getitem(&t2697, &Py::str("t_unit"))?, py::getitem(&t2697, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2698 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2699 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2721 = (t2699).items()?;
        if t2721.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2721[0].clone();
        v_v = t2721[1].clone();
        if ({ let t2700 = v_k.clone(); Py::Bool({ let t2701 = v_outputs.clone(); if !py::contains(&t2701, &t2700)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2745(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2747: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2746 = v_inputs.clone(); f_lee_kesler(py::getitem(&t2746, &Py::str("T"))?, py::getitem(&t2746, &Py::str("Tc"))?, py::getitem(&t2746, &Py::str("Pc"))?, py::getitem(&t2746, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2747 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2748 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2770 = (t2748).items()?;
        if t2770.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2770[0].clone();
        v_v = t2770[1].clone();
        if ({ let t2749 = v_k.clone(); Py::Bool({ let t2750 = v_outputs.clone(); if !py::contains(&t2750, &t2749)? { true } else { false } }) }).truthy() {
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
            v_want = { let t2818 = v_inputs.clone(); f_line_source(py::getitem(&t2818, &Py::str("q"))?, py::getitem(&t2818, &Py::str("mu"))?, py::getitem(&t2818, &Py::str("k"))?, py::getitem(&t2818, &Py::str("h"))?, py::getitem(&t2818, &Py::str("phi"))?, py::getitem(&t2818, &Py::str("ct"))?, py::getitem(&t2818, &Py::str("r"))?, py::getitem(&t2818, &Py::str("t"))?, py::getitem(&t2818, &Py::str("pi"))?)? };
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
fn s_2878(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2880: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2879 = v_inputs.clone(); f_mass_to_mole(py::getitem(&t2879, &Py::str("w"))?, py::getitem(&t2879, &Py::str("MW"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2880 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2881 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2903 = (t2881).items()?;
        if t2903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2903[0].clone();
        v_v = t2903[1].clone();
        if ({ let t2882 = v_k.clone(); Py::Bool({ let t2883 = v_outputs.clone(); if !py::contains(&t2883, &t2882)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2951(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2953: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2952 = v_inputs.clone(); f_mass_transfer_coefficient(py::getitem(&t2952, &Py::str("geometry"))?, py::getitem(&t2952, &Py::str("u"))?, py::getitem(&t2952, &Py::str("L"))?, py::getitem(&t2952, &Py::str("D"))?, py::getitem(&t2952, &Py::str("rho"))?, py::getitem(&t2952, &Py::str("mu"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2953 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2954 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2976 = (t2954).items()?;
        if t2976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2976[0].clone();
        v_v = t2976[1].clone();
        if ({ let t2955 = v_k.clone(); Py::Bool({ let t2956 = v_outputs.clone(); if !py::contains(&t2956, &t2955)? { true } else { false } }) }).truthy() {
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
fn s_3037(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3039: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3038 = v_inputs.clone(); f_mccabe_thiele(py::getitem(&t3038, &Py::str("alpha"))?, py::getitem(&t3038, &Py::str("zF"))?, py::getitem(&t3038, &Py::str("xD"))?, py::getitem(&t3038, &Py::str("xB"))?, py::getitem(&t3038, &Py::str("q"))?, py::getitem(&t3038, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3039 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3040 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3062 = (t3040).items()?;
        if t3062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3062[0].clone();
        v_v = t3062[1].clone();
        if ({ let t3041 = v_k.clone(); Py::Bool({ let t3042 = v_outputs.clone(); if !py::contains(&t3042, &t3041)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3105: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3104 = v_inputs.clone(); f_modified_raoult_bubble(py::getitem(&t3104, &Py::str("x"))?, py::getitem(&t3104, &Py::str("gamma"))?, py::getitem(&t3104, &Py::str("Psat"))?, py::getitem(&t3104, &Py::str("p_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3105 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3106 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3128 = (t3106).items()?;
        if t3128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3128[0].clone();
        v_v = t3128[1].clone();
        if ({ let t3107 = v_k.clone(); Py::Bool({ let t3108 = v_outputs.clone(); if !py::contains(&t3108, &t3107)? { true } else { false } }) }).truthy() {
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
fn s_3200(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3202: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3201 = v_inputs.clone(); f_underwood_multi(py::getitem(&t3201, &Py::str("alpha"))?, py::getitem(&t3201, &Py::str("z"))?, py::getitem(&t3201, &Py::str("xD"))?, py::getitem(&t3201, &Py::str("q"))?, py::getitem(&t3201, &Py::str("lk"))?, py::getitem(&t3201, &Py::str("hk"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3202 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3203 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3225 = (t3203).items()?;
        if t3225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3225[0].clone();
        v_v = t3225[1].clone();
        if ({ let t3204 = v_k.clone(); Py::Bool({ let t3205 = v_outputs.clone(); if !py::contains(&t3205, &t3204)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3256(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3258: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3257 = v_inputs.clone(); f_npsh_available(py::getitem(&t3257, &Py::str("p_s"))?, py::getitem(&t3257, &Py::str("p_v"))?, py::getitem(&t3257, &Py::str("rho"))?, py::getitem(&t3257, &Py::str("z_s"))?, py::getitem(&t3257, &Py::str("h_f"))?, py::method(&t3257, "get", &[Py::str("NPSHr"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3258 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3259 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3281 = (t3259).items()?;
        if t3281.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3281[0].clone();
        v_v = t3281[1].clone();
        if ({ let t3260 = v_k.clone(); Py::Bool({ let t3261 = v_outputs.clone(); if !py::contains(&t3261, &t3260)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3328(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3330: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3329 = v_inputs.clone(); f_nrtl_gamma(py::getitem(&t3329, &Py::str("x"))?, py::getitem(&t3329, &Py::str("tau"))?, py::getitem(&t3329, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3330 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3331 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3353 = (t3331).items()?;
        if t3353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3353[0].clone();
        v_v = t3353[1].clone();
        if ({ let t3332 = v_k.clone(); Py::Bool({ let t3333 = v_outputs.clone(); if !py::contains(&t3333, &t3332)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3407(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3409: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3408 = v_inputs.clone(); f_p_offset(py::getitem(&t3408, &Py::str("Kc"))?, py::getitem(&t3408, &Py::str("K"))?, py::getitem(&t3408, &Py::str("taus"))?, py::getitem(&t3408, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3409 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3410 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3432 = (t3410).items()?;
        if t3432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3432[0].clone();
        v_v = t3432[1].clone();
        if ({ let t3411 = v_k.clone(); Py::Bool({ let t3412 = v_outputs.clone(); if !py::contains(&t3412, &t3411)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3433(mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py, mut v_T: Py, mut v_P: Py) -> R {
    return Ok(f__eos(Py::str("PR"), v_Tc.clone(), v_Pc.clone(), v_omega.clone(), v_T.clone(), v_P.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3487(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3489: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3488 = v_inputs.clone(); s_3433(py::getitem(&t3488, &Py::str("Tc"))?, py::getitem(&t3488, &Py::str("Pc"))?, py::getitem(&t3488, &Py::str("omega"))?, py::getitem(&t3488, &Py::str("T"))?, py::getitem(&t3488, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3489 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3490 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3512 = (t3490).items()?;
        if t3512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3512[0].clone();
        v_v = t3512[1].clone();
        if ({ let t3491 = v_k.clone(); Py::Bool({ let t3492 = v_outputs.clone(); if !py::contains(&t3492, &t3491)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3531(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3533: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3532 = v_inputs.clone(); f_pfr_conv_1(py::getitem(&t3532, &Py::str("v0"))?, py::getitem(&t3532, &Py::str("V"))?, py::getitem(&t3532, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3533 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3534 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3556 = (t3534).items()?;
        if t3556.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3556[0].clone();
        v_v = t3556[1].clone();
        if ({ let t3535 = v_k.clone(); Py::Bool({ let t3536 = v_outputs.clone(); if !py::contains(&t3536, &t3535)? { true } else { false } }) }).truthy() {
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
fn s_3575(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3577: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3576 = v_inputs.clone(); f_pfr_conv_2(py::getitem(&t3576, &Py::str("v0"))?, py::getitem(&t3576, &Py::str("V"))?, py::getitem(&t3576, &Py::str("k"))?, py::getitem(&t3576, &Py::str("CA0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3577 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3578 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3600 = (t3578).items()?;
        if t3600.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3600[0].clone();
        v_v = t3600[1].clone();
        if ({ let t3579 = v_k.clone(); Py::Bool({ let t3580 = v_outputs.clone(); if !py::contains(&t3580, &t3579)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3637(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3639: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3638 = v_inputs.clone(); f_pfr_gas_1(py::getitem(&t3638, &Py::str("FA0"))?, py::getitem(&t3638, &Py::str("CA0"))?, py::getitem(&t3638, &Py::str("k"))?, py::getitem(&t3638, &Py::str("eps"))?, py::getitem(&t3638, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3639 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3640 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3662 = (t3640).items()?;
        if t3662.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3662[0].clone();
        v_v = t3662[1].clone();
        if ({ let t3641 = v_k.clone(); Py::Bool({ let t3642 = v_outputs.clone(); if !py::contains(&t3642, &t3641)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3699(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3701: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3700 = v_inputs.clone(); f_pfr_gas_2(py::getitem(&t3700, &Py::str("FA0"))?, py::getitem(&t3700, &Py::str("CA0"))?, py::getitem(&t3700, &Py::str("k"))?, py::getitem(&t3700, &Py::str("eps"))?, py::getitem(&t3700, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3701 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3702 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3724 = (t3702).items()?;
        if t3724.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3724[0].clone();
        v_v = t3724[1].clone();
        if ({ let t3703 = v_k.clone(); Py::Bool({ let t3704 = v_outputs.clone(); if !py::contains(&t3704, &t3703)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3755(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3757: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3756 = v_inputs.clone(); f_pfr_volume_1(py::getitem(&t3756, &Py::str("v0"))?, py::getitem(&t3756, &Py::str("k"))?, py::getitem(&t3756, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3757 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3758 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3780 = (t3758).items()?;
        if t3780.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3780[0].clone();
        v_v = t3780[1].clone();
        if ({ let t3759 = v_k.clone(); Py::Bool({ let t3760 = v_outputs.clone(); if !py::contains(&t3760, &t3759)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3811(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3813: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3812 = v_inputs.clone(); f_pfr_volume_2(py::getitem(&t3812, &Py::str("v0"))?, py::getitem(&t3812, &Py::str("k"))?, py::getitem(&t3812, &Py::str("CA0"))?, py::getitem(&t3812, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3813 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3814 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3836 = (t3814).items()?;
        if t3836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3836[0].clone();
        v_v = t3836[1].clone();
        if ({ let t3815 = v_k.clone(); Py::Bool({ let t3816 = v_outputs.clone(); if !py::contains(&t3816, &t3815)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3900(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3902: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3901 = v_inputs.clone(); f_margins_pi(py::getitem(&t3901, &Py::str("Kc"))?, py::getitem(&t3901, &Py::str("tauI"))?, py::getitem(&t3901, &Py::str("K"))?, py::getitem(&t3901, &Py::str("taus"))?, py::getitem(&t3901, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3902 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3903 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3925 = (t3903).items()?;
        if t3925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3925[0].clone();
        v_v = t3925[1].clone();
        if ({ let t3904 = v_k.clone(); Py::Bool({ let t3905 = v_outputs.clone(); if !py::contains(&t3905, &t3904)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3958(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3960: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3959 = v_inputs.clone(); f_pid_position(py::getitem(&t3959, &Py::str("Kc"))?, py::getitem(&t3959, &Py::str("tauI"))?, py::getitem(&t3959, &Py::str("tauD"))?, py::getitem(&t3959, &Py::str("dt"))?, py::getitem(&t3959, &Py::str("e"))?, py::getitem(&t3959, &Py::str("bias"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3960 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3961 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3983 = (t3961).items()?;
        if t3983.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3983[0].clone();
        v_v = t3983[1].clone();
        if ({ let t3962 = v_k.clone(); Py::Bool({ let t3963 = v_outputs.clone(); if !py::contains(&t3963, &t3962)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4007(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4009: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4008 = v_inputs.clone(); f_pitzer_hvap(py::getitem(&t4008, &Py::str("T"))?, py::getitem(&t4008, &Py::str("Tc"))?, py::getitem(&t4008, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4009 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4010 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4032 = (t4010).items()?;
        if t4032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4032[0].clone();
        v_v = t4032[1].clone();
        if ({ let t4011 = v_k.clone(); Py::Bool({ let t4012 = v_outputs.clone(); if !py::contains(&t4012, &t4011)? { true } else { false } }) }).truthy() {
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
fn s_4076(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4078: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4077 = v_inputs.clone(); f_pump_head(py::getitem(&t4077, &Py::str("p1"))?, py::getitem(&t4077, &Py::str("p2"))?, py::getitem(&t4077, &Py::str("rho"))?, py::method(&t4077, "get", &[Py::str("dz"), Py::Float(0.0)])?, py::method(&t4077, "get", &[Py::str("v1"), Py::Float(0.0)])?, py::method(&t4077, "get", &[Py::str("v2"), Py::Float(0.0)])?, py::method(&t4077, "get", &[Py::str("Q"), Py::None])?, py::method(&t4077, "get", &[Py::str("eta"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4078 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4079 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4101 = (t4079).items()?;
        if t4101.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4101[0].clone();
        v_v = t4101[1].clone();
        if ({ let t4080 = v_k.clone(); Py::Bool({ let t4081 = v_outputs.clone(); if !py::contains(&t4081, &t4080)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4181(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4183: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4182 = v_inputs.clone(); f_rachford_rice(py::getitem(&t4182, &Py::str("z"))?, py::getitem(&t4182, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4183 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4184 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4206 = (t4184).items()?;
        if t4206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4206[0].clone();
        v_v = t4206[1].clone();
        if ({ let t4185 = v_k.clone(); Py::Bool({ let t4186 = v_outputs.clone(); if !py::contains(&t4186, &t4185)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4230(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4232: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4231 = v_inputs.clone(); f_rackett(py::getitem(&t4231, &Py::str("T"))?, py::getitem(&t4231, &Py::str("Tc"))?, py::getitem(&t4231, &Py::str("Pc"))?, py::getitem(&t4231, &Py::str("Zc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4232 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4233 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4255 = (t4233).items()?;
        if t4255.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4255[0].clone();
        v_v = t4255[1].clone();
        if ({ let t4234 = v_k.clone(); Py::Bool({ let t4235 = v_outputs.clone(); if !py::contains(&t4235, &t4234)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4283(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4285: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4284 = v_inputs.clone(); f_radial_flow(py::getitem(&t4284, &Py::str("k"))?, py::getitem(&t4284, &Py::str("h"))?, py::getitem(&t4284, &Py::str("mu"))?, py::getitem(&t4284, &Py::str("pe"))?, py::getitem(&t4284, &Py::str("pw"))?, py::getitem(&t4284, &Py::str("re"))?, py::getitem(&t4284, &Py::str("rw"))?, py::method(&t4284, "get", &[Py::str("S"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4285 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4286 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4308 = (t4286).items()?;
        if t4308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4308[0].clone();
        v_v = t4308[1].clone();
        if ({ let t4287 = v_k.clone(); Py::Bool({ let t4288 = v_outputs.clone(); if !py::contains(&t4288, &t4287)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4359(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4361: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4360 = v_inputs.clone(); f_bubble_P(py::getitem(&t4360, &Py::str("x"))?, py::getitem(&t4360, &Py::str("ant"))?, py::getitem(&t4360, &Py::str("units"))?, py::getitem(&t4360, &Py::str("base"))?, py::getitem(&t4360, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4361 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4362 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4384 = (t4362).items()?;
        if t4384.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4384[0].clone();
        v_v = t4384[1].clone();
        if ({ let t4363 = v_k.clone(); Py::Bool({ let t4364 = v_outputs.clone(); if !py::contains(&t4364, &t4363)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4469(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4471: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4470 = v_inputs.clone(); f_bubble_T(py::getitem(&t4470, &Py::str("x"))?, py::getitem(&t4470, &Py::str("ant"))?, py::getitem(&t4470, &Py::str("units"))?, py::getitem(&t4470, &Py::str("base"))?, py::getitem(&t4470, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4471 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4472 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4494 = (t4472).items()?;
        if t4494.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4494[0].clone();
        v_v = t4494[1].clone();
        if ({ let t4473 = v_k.clone(); Py::Bool({ let t4474 = v_outputs.clone(); if !py::contains(&t4474, &t4473)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4545(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4547: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4546 = v_inputs.clone(); f_dew_P(py::getitem(&t4546, &Py::str("y"))?, py::getitem(&t4546, &Py::str("ant"))?, py::getitem(&t4546, &Py::str("units"))?, py::getitem(&t4546, &Py::str("base"))?, py::getitem(&t4546, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4547 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4548 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4570 = (t4548).items()?;
        if t4570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4570[0].clone();
        v_v = t4570[1].clone();
        if ({ let t4549 = v_k.clone(); Py::Bool({ let t4550 = v_outputs.clone(); if !py::contains(&t4550, &t4549)? { true } else { false } }) }).truthy() {
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
fn s_4655(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4657: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4656 = v_inputs.clone(); f_dew_T(py::getitem(&t4656, &Py::str("y"))?, py::getitem(&t4656, &Py::str("ant"))?, py::getitem(&t4656, &Py::str("units"))?, py::getitem(&t4656, &Py::str("base"))?, py::getitem(&t4656, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4657 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4658 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4680 = (t4658).items()?;
        if t4680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4680[0].clone();
        v_v = t4680[1].clone();
        if ({ let t4659 = v_k.clone(); Py::Bool({ let t4660 = v_outputs.clone(); if !py::contains(&t4660, &t4659)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4778(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4780: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4779 = v_inputs.clone(); f_raoult_flash(py::getitem(&t4779, &Py::str("z"))?, py::getitem(&t4779, &Py::str("ant"))?, py::getitem(&t4779, &Py::str("units"))?, py::getitem(&t4779, &Py::str("base"))?, py::getitem(&t4779, &Py::str("T"))?, py::getitem(&t4779, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4780 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4781 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4803 = (t4781).items()?;
        if t4803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4803[0].clone();
        v_v = t4803[1].clone();
        if ({ let t4782 = v_k.clone(); Py::Bool({ let t4783 = v_outputs.clone(); if !py::contains(&t4783, &t4782)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4835(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4837: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4836 = v_inputs.clone(); f_reaction_enthalpy_T(py::getitem(&t4836, &Py::str("nu"))?, py::getitem(&t4836, &Py::str("cp"))?, py::getitem(&t4836, &Py::str("dH0"))?, py::getitem(&t4836, &Py::str("T0"))?, py::getitem(&t4836, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4837 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4838 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4860 = (t4838).items()?;
        if t4860.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4860[0].clone();
        v_v = t4860[1].clone();
        if ({ let t4839 = v_k.clone(); Py::Bool({ let t4840 = v_outputs.clone(); if !py::contains(&t4840, &t4839)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4894: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4893 = v_inputs.clone(); f_recycle_purge(py::getitem(&t4893, &Py::str("F0"))?, py::getitem(&t4893, &Py::str("zA"))?, py::getitem(&t4893, &Py::str("X"))?, py::getitem(&t4893, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4894 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4895 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4917 = (t4895).items()?;
        if t4917.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4917[0].clone();
        v_v = t4917[1].clone();
        if ({ let t4896 = v_k.clone(); Py::Bool({ let t4897 = v_outputs.clone(); if !py::contains(&t4897, &t4896)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4942(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4944: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4943 = v_inputs.clone(); f_recycle_reactor_1(py::getitem(&t4943, &Py::str("v0"))?, py::getitem(&t4943, &Py::str("V"))?, py::getitem(&t4943, &Py::str("k"))?, py::getitem(&t4943, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4944 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4945 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4967 = (t4945).items()?;
        if t4967.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4967[0].clone();
        v_v = t4967[1].clone();
        if ({ let t4946 = v_k.clone(); Py::Bool({ let t4947 = v_outputs.clone(); if !py::contains(&t4947, &t4946)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4992(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4994: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4993 = v_inputs.clone(); f_reversible_batch(py::getitem(&t4993, &Py::str("k1"))?, py::getitem(&t4993, &Py::str("k2"))?, py::getitem(&t4993, &Py::str("CA0"))?, py::getitem(&t4993, &Py::str("CB0"))?, py::getitem(&t4993, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4994 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4995 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5017 = (t4995).items()?;
        if t5017.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5017[0].clone();
        v_v = t5017[1].clone();
        if ({ let t4996 = v_k.clone(); Py::Bool({ let t4997 = v_outputs.clone(); if !py::contains(&t4997, &t4996)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5038(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5040: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5039 = v_inputs.clone(); f_riedel(py::getitem(&t5039, &Py::str("Tb"))?, py::getitem(&t5039, &Py::str("Tc"))?, py::getitem(&t5039, &Py::str("Pc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5040 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5041 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5063 = (t5041).items()?;
        if t5063.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5063[0].clone();
        v_v = t5063[1].clone();
        if ({ let t5042 = v_k.clone(); Py::Bool({ let t5043 = v_outputs.clone(); if !py::contains(&t5043, &t5042)? { true } else { false } }) }).truthy() {
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
fn s_5100(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5101 = v_inputs.clone(); f_rtd_moments(py::getitem(&t5101, &Py::str("t"))?, py::getitem(&t5101, &Py::str("C"))?, py::getitem(&t5101, &Py::str("t_unit"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5102 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5103 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5125 = (t5103).items()?;
        if t5125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5125[0].clone();
        v_v = t5125[1].clone();
        if ({ let t5104 = v_k.clone(); Py::Bool({ let t5105 = v_outputs.clone(); if !py::contains(&t5105, &t5104)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5150(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5152: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5151 = v_inputs.clone(); f_sensible_heat(py::getitem(&t5151, &Py::str("n"))?, py::getitem(&t5151, &Py::str("a"))?, py::getitem(&t5151, &Py::str("b"))?, py::getitem(&t5151, &Py::str("T1"))?, py::getitem(&t5151, &Py::str("T2"))?, py::method(&t5151, "get", &[Py::str("c"), Py::Float(0.0)])?, py::method(&t5151, "get", &[Py::str("d"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5152 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5153 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5175 = (t5153).items()?;
        if t5175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5175[0].clone();
        v_v = t5175[1].clone();
        if ({ let t5154 = v_k.clone(); Py::Bool({ let t5155 = v_outputs.clone(); if !py::contains(&t5155, &t5154)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5197(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5199: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5198 = v_inputs.clone(); f_series_max(py::getitem(&t5198, &Py::str("k1"))?, py::getitem(&t5198, &Py::str("k2"))?, py::getitem(&t5198, &Py::str("CA0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5199 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5200 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5222 = (t5200).items()?;
        if t5222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5222[0].clone();
        v_v = t5222[1].clone();
        if ({ let t5201 = v_k.clone(); Py::Bool({ let t5202 = v_outputs.clone(); if !py::contains(&t5202, &t5201)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5223(mut v_Tc: Py, mut v_Pc: Py, mut v_omega: Py, mut v_T: Py, mut v_P: Py) -> R {
    return Ok(f__eos(Py::str("SRK"), v_Tc.clone(), v_Pc.clone(), v_omega.clone(), v_T.clone(), v_P.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5277(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5279: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5278 = v_inputs.clone(); s_5223(py::getitem(&t5278, &Py::str("Tc"))?, py::getitem(&t5278, &Py::str("Pc"))?, py::getitem(&t5278, &Py::str("omega"))?, py::getitem(&t5278, &Py::str("T"))?, py::getitem(&t5278, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5279 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5280 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5302 = (t5280).items()?;
        if t5302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5302[0].clone();
        v_v = t5302[1].clone();
        if ({ let t5281 = v_k.clone(); Py::Bool({ let t5282 = v_outputs.clone(); if !py::contains(&t5282, &t5281)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5335(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5337: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5336 = v_inputs.clone(); f_stefan_flux(py::getitem(&t5336, &Py::str("D"))?, py::getitem(&t5336, &Py::str("P"))?, py::getitem(&t5336, &Py::str("T"))?, py::getitem(&t5336, &Py::str("L"))?, py::getitem(&t5336, &Py::str("yA1"))?, py::getitem(&t5336, &Py::str("yA2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5337 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5338 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5360 = (t5338).items()?;
        if t5360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5360[0].clone();
        v_v = t5360[1].clone();
        if ({ let t5339 = v_k.clone(); Py::Bool({ let t5340 = v_outputs.clone(); if !py::contains(&t5340, &t5339)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5424(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5426: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5425 = v_inputs.clone(); f_uniquac_gamma(py::getitem(&t5425, &Py::str("x"))?, py::getitem(&t5425, &Py::str("r"))?, py::getitem(&t5425, &Py::str("q"))?, py::getitem(&t5425, &Py::str("tau"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5426 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5427 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5449 = (t5427).items()?;
        if t5449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5449[0].clone();
        v_v = t5449[1].clone();
        if ({ let t5428 = v_k.clone(); Py::Bool({ let t5429 = v_outputs.clone(); if !py::contains(&t5429, &t5428)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5474(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5476: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5475 = v_inputs.clone(); f_mixing_tank(py::getitem(&t5475, &Py::str("V"))?, py::getitem(&t5475, &Py::str("q"))?, py::getitem(&t5475, &Py::str("C_in"))?, py::getitem(&t5475, &Py::str("C0"))?, py::getitem(&t5475, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5476 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5477 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5499 = (t5477).items()?;
        if t5499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5499[0].clone();
        v_v = t5499[1].clone();
        if ({ let t5478 = v_k.clone(); Py::Bool({ let t5479 = v_outputs.clone(); if !py::contains(&t5479, &t5478)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5528(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5530: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5529 = v_inputs.clone(); f_variable_volume_tank(py::getitem(&t5529, &Py::str("V0"))?, py::getitem(&t5529, &Py::str("q_in"))?, py::getitem(&t5529, &Py::str("q_out"))?, py::getitem(&t5529, &Py::str("C_in"))?, py::getitem(&t5529, &Py::str("C0"))?, py::getitem(&t5529, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5530 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5531 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5553 = (t5531).items()?;
        if t5553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5553[0].clone();
        v_v = t5553[1].clone();
        if ({ let t5532 = v_k.clone(); Py::Bool({ let t5533 = v_outputs.clone(); if !py::contains(&t5533, &t5532)? { true } else { false } }) }).truthy() {
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
fn s_5577(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5579: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5578 = v_inputs.clone(); f_abbott_virial(py::getitem(&t5578, &Py::str("T"))?, py::getitem(&t5578, &Py::str("P"))?, py::getitem(&t5578, &Py::str("Tc"))?, py::getitem(&t5578, &Py::str("Pc"))?, py::getitem(&t5578, &Py::str("omega"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5579 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5580 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5602 = (t5580).items()?;
        if t5602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5602[0].clone();
        v_v = t5602[1].clone();
        if ({ let t5581 = v_k.clone(); Py::Bool({ let t5582 = v_outputs.clone(); if !py::contains(&t5582, &t5581)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5626(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5628: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5627 = v_inputs.clone(); f_watson(py::getitem(&t5627, &Py::str("T"))?, py::getitem(&t5627, &Py::str("T_ref"))?, py::getitem(&t5627, &Py::str("Hvap_ref"))?, py::getitem(&t5627, &Py::str("Tc"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5628 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5629 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5651 = (t5629).items()?;
        if t5651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5651[0].clone();
        v_v = t5651[1].clone();
        if ({ let t5630 = v_k.clone(); Py::Bool({ let t5631 = v_outputs.clone(); if !py::contains(&t5631, &t5630)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5700: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5699 = v_inputs.clone(); f_wilson_gamma(py::getitem(&t5699, &Py::str("x"))?, py::getitem(&t5699, &Py::str("Lam"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5700 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5701 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5723 = (t5701).items()?;
        if t5723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5723[0].clone();
        v_v = t5723[1].clone();
        if ({ let t5702 = v_k.clone(); Py::Bool({ let t5703 = v_outputs.clone(); if !py::contains(&t5703, &t5702)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5810(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5812: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5811 = v_inputs.clone(); f_wilson_flash(py::getitem(&t5811, &Py::str("z"))?, py::getitem(&t5811, &Py::str("Tc"))?, py::getitem(&t5811, &Py::str("Pc"))?, py::getitem(&t5811, &Py::str("omega"))?, py::getitem(&t5811, &Py::str("p_unit"))?, py::getitem(&t5811, &Py::str("T"))?, py::getitem(&t5811, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5812 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5813 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5835 = (t5813).items()?;
        if t5835.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5835[0].clone();
        v_v = t5835[1].clone();
        if ({ let t5814 = v_k.clone(); Py::Bool({ let t5815 = v_outputs.clone(); if !py::contains(&t5815, &t5814)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5900(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5902: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5901 = v_inputs.clone(); f_ziegler_nichols(py::getitem(&t5901, &Py::str("K"))?, py::getitem(&t5901, &Py::str("taus"))?, py::getitem(&t5901, &Py::str("t_unit"))?, py::getitem(&t5901, &Py::str("controller"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5902 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5903 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5925 = (t5903).items()?;
        if t5925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5925[0].clone();
        v_v = t5925[1].clone();
        if ({ let t5904 = v_k.clone(); Py::Bool({ let t5905 = v_outputs.clone(); if !py::contains(&t5905, &t5904)? { true } else { false } }) }).truthy() {
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
