//! Every Chemical Engineering capability's own live figures (src/figures*.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas. FIG_DUMP=<dir> writes every worked example's figures there as SVG files (for looking at them).

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use chemical_engineering_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/chemical-engineering-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("chemeng_absorber_transfer_units", &["process"]),
    ("chemeng_adiabatic_reactor_temperature", &["curve-2"]),
    ("chemeng_antoine_boiling_temperature", &["chart", "chart-2"]),
    ("chemeng_antoine_vapour_pressure", &["chart", "chart-2"]),
    ("chemeng_batch_conversion_first_order", &["field"]),
    ("chemeng_batch_time_first_order", &["field"]),
    ("chemeng_binary_separator_balance", &["process"]),
    ("chemeng_blending_flows", &["process"]),
    ("chemeng_closed_loop_stability", &["decision"]),
    ("chemeng_combustion_flue_gas", &["bars"]),
    ("chemeng_countercurrent_extraction", &["process"]),
    ("chemeng_crosscurrent_extraction", &["process"]),
    ("chemeng_cstr_conversion_first_order", &["process", "reactor"]),
    ("chemeng_cstr_conversion_second_order", &["process"]),
    ("chemeng_cstr_volume_first_order", &["process"]),
    ("chemeng_cstr_volume_second_order", &["process"]),
    ("chemeng_cstrs_in_series_first_order", &["heatmap", "process"]),
    ("chemeng_cstrs_in_series_second_order", &["process"]),
    ("chemeng_equimolar_counterdiffusion", &["profile"]),
    ("chemeng_filtration_constants_fit", &["fit"]),
    ("chemeng_first_order_step_fit", &["fit"]),
    ("chemeng_fopdt_step_response", &["response"]),
    ("chemeng_gas_pz_material_balance", &["fit"]),
    ("chemeng_gas_well_backpressure", &["xy"]),
    ("chemeng_havlena_odeh_gas_cap", &["fit"]),
    ("chemeng_havlena_odeh_oil_in_place", &["fit"]),
    ("chemeng_horner_buildup", &["fit"]),
    ("chemeng_hyperbolic_decline", &["decline"]),
    ("chemeng_kremser_absorber", &["process"]),
    ("chemeng_kremser_stripper", &["process"]),
    ("chemeng_lag_frequency_response", &["response"]),
    ("chemeng_mass_to_mole_fractions", &["bars"]),
    ("chemeng_modified_raoult_bubble_pressure", &["vectors"]),
    ("chemeng_npsh_available", &["diagram"]),
    ("chemeng_p_control_offset", &["field"]),
    ("chemeng_pfr_conversion_first_order", &["reactor"]),
    ("chemeng_pfr_conversion_second_order", &["reactor"]),
    ("chemeng_pfr_gas_volume_first_order", &["reactor"]),
    ("chemeng_pfr_gas_volume_second_order", &["reactor"]),
    ("chemeng_pfr_volume_first_order", &["reactor"]),
    ("chemeng_pfr_volume_second_order", &["reactor"]),
    ("chemeng_pid_controller_output", &["response"]),
    ("chemeng_rachford_rice_flash", &["bars", "bars-2", "process"]),
    ("chemeng_raoult_bubble_pressure", &["phase", "phase-2", "phase-3"]),
    ("chemeng_raoult_bubble_temperature", &["phase", "phase-2", "phase-3", "phase-4"]),
    ("chemeng_raoult_dew_pressure", &["phase", "phase-2", "phase-3"]),
    ("chemeng_raoult_dew_temperature", &["phase", "phase-2", "phase-3"]),
    ("chemeng_raoult_isothermal_flash", &["bars", "phase", "process"]),
    ("chemeng_reaction_enthalpy_at_temperature", &["matrix"]),
    ("chemeng_recycle_purge", &["process"]),
    ("chemeng_recycle_reactor_first_order", &["process"]),
    ("chemeng_reversible_first_order_batch", &["curve-2"]),
    ("chemeng_rtd_moments", &["data"]),
    ("chemeng_stefan_diffusion_flux", &["profile"]),
    ("chemeng_unsteady_mixing_tank", &["process"]),
    ("chemeng_variable_volume_tank", &["process"]),
    ("chemeng_wilson_k_flash", &["bars", "process"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("chemeng_combustion_flue_gas", "fuel", "C3H8"),
    ("chemeng_rachford_rice_flash", "K", "[2.1, 0.9, 0.4]"),
    ("chemeng_wilson_k_flash", "T", "320 K"),
    ("chemeng_raoult_isothermal_flash", "T", "370 K"),
    ("chemeng_raoult_bubble_pressure", "x", "[0.6, 0.4]"),
    ("chemeng_raoult_dew_pressure", "y", "[0.6, 0.4]"),
    ("chemeng_raoult_bubble_temperature", "x", "[0.6, 0.4]"),
    ("chemeng_raoult_dew_temperature", "y", "[0.6, 0.4]"),
    ("chemeng_mass_to_mole_fractions", "w", "[0.3, 0.7]"),
    ("chemeng_modified_raoult_bubble_pressure", "x", "[0.7, 0.3]"),
    ("chemeng_blending_flows", "p", "[0.6, 0.4]"),
    ("chemeng_closed_loop_stability", "Kc", "5"),
    ("chemeng_countercurrent_extraction", "N", "6"),
    ("chemeng_crosscurrent_extraction", "N", "6"),
    ("chemeng_cstrs_in_series_first_order", "N", "5"),
    ("chemeng_cstrs_in_series_second_order", "N", "6"),
    ("chemeng_kremser_absorber", "N", "8"),
    ("chemeng_kremser_stripper", "N", "4"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c=[1, 2], d=[[1, 2], [3, 4]]).
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            let mut depth = 0;
            while i < cs.len() && !(depth == 0 && (cs[i] == ',' || cs[i] == ')')) {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                }
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let dump = std::env::var("FIG_DUMP").ok();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        if let Some(d) = &dump {
            std::fs::create_dir_all(d).unwrap();
            for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                std::fs::write(format!("{d}/{}.svg", f.slug), &f.svg).unwrap();
            }
        }
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert!(f.slug == format!("{}-{kind}", key.replace('_', "-")) || f.slug.contains("-live-"),
                            "{key}: slug {}", f.slug);
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf\""), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
        for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
            check_texts(f);
            for bad in [" we ", " our ", " us ", "—", "package"] {
                assert!(!f.caption.to_lowercase().contains(bad) && !f.title.to_lowercase().contains(bad),
                        "{key}: public wording ({bad}) in {}", f.slug);
            }
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f: every number of a list ("[3, 6]" -> "[4.11, 8.22]"), else only the leading
/// value, never a unit's exponent ("0.01 m^3/s" -> "0.0137 m^3/s").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let list = t.trim_start().starts_with('[');
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 && (list || !*changed) && !out.ends_with('^') => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let own: Vec<&str> = kinds.iter().copied().filter(|k| !(*key == "chemeng_wilson_k_flash" && *k == "bars")).collect();
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = own.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a conversion above 1 cannot be reached: the model refuses and no figure is drawn
    let p = pack();
    for (key, sym) in [("chemeng_pfr_volume_first_order", "X"), ("chemeng_cstr_volume_first_order", "X")] {
        let mut given = example(key);
        for e in given.iter_mut() {
            if e.0 == sym {
                e.1 = Input::Text("1.5".into());
            }
        }
        let a = ax_engine::solve(p, key, &given, None);
        let figs = figures(p, key, &given, None, None);
        if !a.solved() {
            assert!(figs.is_empty(), "{key}: a figure for a refused input");
        }
    }
    // a three-component mixture has no binary phase diagram: those figures are left out, the others stay right
    let key = "chemeng_raoult_bubble_pressure";
    let mut given = example(key);
    for e in given.iter_mut() {
        if e.0 == "x" {
            e.1 = Input::Text("[0.2, 0.3, 0.5]".into());
        }
        if e.0 == "ant" {
            e.1 = Input::Text("[[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482], [6.95464, 1344.8, 219.482]]".into());
        }
    }
    let figs = figures(p, key, &given, None, None);
    assert!(figs.iter().all(|f| !f.kind.starts_with("phase")), "a binary diagram for three components");
}
