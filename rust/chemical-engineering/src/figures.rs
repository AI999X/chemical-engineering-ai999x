//! Chemical Engineering's own live figures (ax_engine::figkit): every capability's published figure kinds (process
//! diagrams, reactor profiles, phase diagrams, fitted data, responses, maps, bars), drawn for the visitor's inputs from
//! what the model computed. Written by hand for the Rust build (not generated). A figure the inputs make impossible is
//! left out, never drawn wrong (docs/FIGURE_PARITY.md).

use crate::figures_charts as ch;
use crate::figures_process as pr;
use ax_engine::figkit::Solved;
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let key = s.key.strip_prefix("chemeng_").unwrap_or(s.key);
    let v: Vec<Option<Figure>> = match key {
        "absorber_transfer_units" => vec![pr::absorber_packed(s)],
        "binary_separator_balance" => vec![pr::separator_column(s)],
        "kremser_absorber" => vec![pr::kremser(s, true)],
        "kremser_stripper" => vec![pr::kremser(s, false)],
        "blending_flows" => vec![pr::blending(s)],
        "countercurrent_extraction" => vec![pr::countercurrent(s)],
        "crosscurrent_extraction" => vec![pr::crosscurrent(s)],
        "recycle_purge" => vec![pr::recycle_purge(s)],
        "recycle_reactor_first_order" => vec![pr::recycle_reactor(s)],
        "unsteady_mixing_tank" => vec![pr::mixing_tank(s)],
        "variable_volume_tank" => vec![pr::variable_tank(s)],
        "cstr_conversion_first_order" => vec![pr::cstr(s, false, false), ch::reactor_compare(s)],
        "cstr_conversion_second_order" => vec![pr::cstr(s, true, false)],
        "cstr_volume_first_order" => vec![pr::cstr(s, false, true)],
        "cstr_volume_second_order" => vec![pr::cstr(s, true, true)],
        "cstrs_in_series_first_order" => vec![pr::cstrs_series(s, false), ch::series_table(s)],
        "cstrs_in_series_second_order" => vec![pr::cstrs_series(s, true)],
        "pfr_conversion_first_order" => vec![ch::pfr_profile(s, false, false)],
        "pfr_conversion_second_order" => vec![ch::pfr_profile(s, false, true)],
        "pfr_volume_first_order" | "pfr_gas_volume_first_order" => vec![ch::pfr_profile(s, true, false)],
        "pfr_volume_second_order" | "pfr_gas_volume_second_order" => vec![ch::pfr_profile(s, true, true)],
        "rachford_rice_flash" => {
            let kl = s.vec("K").map(|k| k.iter().map(|x| ax_engine::num::py_g(*x, 4)).collect::<Vec<_>>());
            vec![s.vec("z").zip(kl).and_then(|(z, kl)| ch::flash_bars(s, &z, &kl)), ch::split_bars(s), pr::rachford_drum(s)]
        }
        "raoult_isothermal_flash" => {
            let mut v = vec![flash_bars_ratio(s)];
            v.extend(ch::raoult_phase(s));
            v.push(pr::raoult_drum(s));
            v
        }
        "wilson_k_flash" => vec![pr::wilson_drum(s)],
        "raoult_bubble_pressure" | "raoult_dew_pressure" | "raoult_bubble_temperature" | "raoult_dew_temperature" => {
            ch::raoult_phase(s)
        }
        "antoine_vapour_pressure" | "antoine_boiling_temperature" => vec![ch::antoine(s, false), ch::antoine(s, true)],
        "adiabatic_reactor_temperature" => vec![ch::family(s, "T_ref", "T_out", "T_in", "K",
                                                           "reference temperature of dH T_ref [K]",
                                                           "outlet temperature T_out [K]",
                                                           "Adiabatic reactor outlet temperature for three feed temperatures")],
        "reversible_first_order_batch" => vec![ch::family(s, "k1", "X", "t", "s", "forward rate constant k1 [1/s]",
                                                          "conversion of A at t, X",
                                                          "Reversible first-order reaction in a batch: conversion for three times")],
        "batch_conversion_first_order" => vec![ch::batch_conv_map(s)],
        "batch_time_first_order" => vec![ch::batch_time_map(s)],
        "p_control_offset" => vec![ch::offset_map(s)],
        "closed_loop_stability" => vec![ch::stability_decision(s)],
        "combustion_flue_gas" => vec![ch::flue_bars(s)],
        "mass_to_mole_fractions" => vec![ch::mole_bars(s)],
        "equimolar_counterdiffusion" => vec![ch::diffusion_profile(s, true)],
        "stefan_diffusion_flux" => vec![ch::diffusion_profile(s, false)],
        "filtration_constants_fit" => vec![ch::filtration_fit(s)],
        "first_order_step_fit" => vec![ch::step_fit(s)],
        "gas_pz_material_balance" => vec![ch::pz_fit(s)],
        "havlena_odeh_gas_cap" => vec![ch::havlena_cap(s)],
        "havlena_odeh_oil_in_place" => vec![ch::havlena_oil(s)],
        "horner_buildup" => vec![ch::horner_fit(s)],
        "gas_well_backpressure" => vec![ch::backpressure_xy(s)],
        "fopdt_step_response" => vec![ch::fopdt_response(s)],
        "lag_frequency_response" => vec![ch::lag_bode(s)],
        "pid_controller_output" => vec![ch::pid_response(s)],
        "hyperbolic_decline" => vec![ch::decline(s)],
        "rtd_moments" => vec![ch::rtd_data(s)],
        "npsh_available" => vec![ch::npsh_diagram(s)],
        "reaction_enthalpy_at_temperature" => vec![ch::cp_matrix(s)],
        "modified_raoult_bubble_pressure" => vec![ch::raoult_vectors(s)],
        _ => vec![],
    };
    v.into_iter().flatten().take(4).collect()
}

/// The Raoult flash's bars, each group labelled with K = y/x of the returned compositions.
fn flash_bars_ratio(s: &Solved) -> Option<Figure> {
    let z = s.vec("z")?;
    let (x, y) = (s.out_list("x")?, s.out_list("y")?);
    if x.len() != y.len() || x.iter().any(|v| *v == 0.0) {
        return None;
    }
    let kl: Vec<String> = x.iter().zip(&y).map(|(a, b)| ax_engine::num::py_g(b / a, 3)).collect();
    ch::flash_bars(s, &z, &kl)
}

/// A quantity as text (ax_engine::figkit::q), times in plain seconds ("9000 s", not "9 ks").
pub fn q(x: f64, unit: &str) -> String {
    if unit == "s" { format!("{} s", ax_engine::figkit::num(x)) } else { ax_engine::figkit::q(x, unit) }
}

/// An input exactly as given (to pass to a sibling capability unchanged).
pub fn gin(s: &Solved, sym: &str) -> Option<Input> {
    s.given.iter().find(|g| g.0 == sym).map(|g| g.1.clone())
}

/// A whole-number input.
pub fn nint(s: &Solved, sym: &str) -> Option<i64> {
    match s.val(sym)? {
        ax_engine::Val::Int(i) => Some(i),
        ax_engine::Val::Num(x) if x.fract() == 0.0 && x.is_finite() => Some(x as i64),
        _ => None,
    }
}

/// Seconds in a time unit of a choice input.
pub fn tfac(u: &str) -> Option<f64> {
    match u {
        "s" => Some(1.0),
        "min" => Some(60.0),
        "h" => Some(3600.0),
        _ => None,
    }
}

/// Pascals in a pressure unit of a choice input.
pub fn pfac(u: &str) -> Option<f64> {
    match u {
        "Pa" => Some(1.0),
        "kPa" => Some(1e3),
        "MPa" => Some(1e6),
        "bar" => Some(1e5),
        "atm" => Some(101_325.0),
        "psi" => Some(6_894.757_293_168),
        _ => None,
    }
}

/// A text output of an answer (a verdict).
pub fn txt_of(a: &Answer, sym: &str) -> Option<String> {
    a.values.iter().find(|v| v.0 == sym).and_then(|v| v.1.as_str().map(|t| t.to_string()))
}
