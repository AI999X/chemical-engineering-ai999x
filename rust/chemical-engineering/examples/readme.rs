//! Chemical Engineering: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p chemical-engineering-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&chemical_engineering_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "chemeng_adiabatic_flame_temperature", "inputs": {"fuel": "CH4", "excess_air": "0", "T_in": "300 K"}}));
    println!();
    show(json!({"key": "chemeng_npsh_available", "inputs": {"p_s": "101325 Pa", "p_v": "2339 Pa", "rho": "998 kg/m^3", "z_s": "2 m", "h_f": "0.5 m"}}));
    println!();
    show(json!({"key": "chemeng_blending_flows", "inputs": {"W": [[0.9, 0.1], [0.2, 0.8]], "p": [0.5, 0.5], "P": "100 kg/s"}}));
    println!();
    show(json!({"key": "chemeng_exponential_decline", "inputs": {"qi": "0.001840130728 m^3/s", "Di": "9.506426344e-09 1/s", "t": "63115200 s"}}));
    println!();
    show(json!({"key": "chemeng_closed_loop_stability", "inputs": {"num": [1], "den": [1, 3, 3, 1], "Kc": "10"}}));
    println!();
    show(json!({"key": "chemeng_equipment_cost_scaling", "inputs": {"C1": "100000", "S1": "10", "S2": "20", "n": "0.6", "I1": "500", "I2": "800"}}));
    println!();
    show(json!({"key": "chemeng_virial_compressibility", "inputs": {"T": "510 K", "P": "2500000 Pa", "Tc": "425.1 K", "Pc": "3796000 Pa", "omega": "0.2"}}));
    println!();
    show(json!({"key": "chemeng_batch_conversion_first_order", "inputs": {"k": "0.001 1/s", "t": "1000 s"}}));
    println!();
    show(json!({"key": "chemeng_gas_pz_material_balance", "inputs": {"p": [3200, 2925, 2525, 2125], "z": [0.85, 0.84, 0.83, 0.82], "Gp": [0, 15, 35, 55], "p_unit": "psi", "gp_unit": "Bscf"}}));
    println!();
    show(json!({"key": "chemeng_kremser_absorber", "inputs": {"N": "5", "A": "1.4", "m": "2", "y_in": "0.05", "x_in": "0"}}));
    println!();
    show(json!({"key": "chemeng_cake_filtration_volume", "inputs": {"A": "1 m^2", "dP": "100000 Pa", "mu": "0.001 Pa*s", "alpha": "1e+11 m/kg", "c": "20 kg/m^3", "Rm": "1e+10 1/m", "t": "600 s"}}));
    println!();
    show(json!({"key": "chemeng_adiabatic_reactor_temperature", "inputs": {"n0": [1, 2, 0], "nu": [-1, -1, 1], "cp": [[30, 0.01], [29, 0.004], [45, 0.02]], "dH": "-60000 J/mol", "T_ref": "298.15 K", "T_in": "400 K", "key": "1", "X": "0.8"}}));
    println!();
    show(json!({"key": "chemeng_pump_head", "inputs": {"p1": "100000 Pa", "p2": "400000 Pa", "rho": "1000 kg/m^3"}}));
    println!();
    show(json!({"key": "chemeng_combustion_flue_gas", "inputs": {"fuel": "CH4", "excess_air": "0.2"}}));
    println!();
    show(json!({"key": "chemeng_gas_well_backpressure", "inputs": {"q": [4288, 9265, 15552, 20177], "pwf": [403.1, 394, 378.5, 362.6], "q_unit": "Mscf_per_day", "p_unit": "psi", "pr": "2814439.927 Pa"}}));
}
