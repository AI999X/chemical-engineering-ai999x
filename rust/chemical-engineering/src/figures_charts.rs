//! Charts of Chemical Engineering's figures: phase diagrams, reactor conversion profiles, fitted data, responses,
//! maps and bars. Every curve is the model's own result at each point (this capability solved again, or a sibling
//! capability of the same model); data drawn are the inputs as read; nothing is recomputed from a formula here.

use crate::figures::{gin, nint, pfac, q, tfac, txt_of};
use crate::figures_plot::{self as pl, legend, li, Plot};
use ax_engine::figkit::{self as k, eq, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use serde_json::json;

fn fl(x: f64) -> Input {
    Input::Float(x)
}

fn ok_pts(v: &[(f64, f64)]) -> bool {
    v.len() >= 2
}

// ------------------------------------------------------------------------------------------------ phase diagrams
/// The Antoine constants and their units/base as given (shared by every Raoult capability).
fn ant_set(s: &Solved) -> Option<Vec<(&'static str, Input)>> {
    Some(vec![("ant", gin(s, "ant")?), ("units", gin(s, "units")?), ("base", gin(s, "base")?)])
}

fn pair(a: f64) -> Input {
    Input::Json(json!([a, 1.0 - a]))
}

/// One Raoult solve: the bubble or dew point of a binary at composition c1 of the given phase, at the fixed T or P.
/// Returns (the computed P or T, the other phase's mole fraction of component 1).
fn raoult(s: &Solved, bubble: bool, fixed: &str, fv: f64, c1: f64) -> Option<(f64, f64)> {
    let key = match (bubble, fixed) {
        (true, "T") => "chemeng_raoult_bubble_pressure",
        (false, "T") => "chemeng_raoult_dew_pressure",
        (true, _) => "chemeng_raoult_bubble_temperature",
        (false, _) => "chemeng_raoult_dew_temperature",
    };
    let mut set = ant_set(s)?;
    set.push((if bubble { "x" } else { "y" }, pair(c1)));
    set.push((if fixed == "T" { "T" } else { "P" }, fl(fv)));
    let a = s.solve_other(key, &set)?;
    let val = k::num_of(&a, if fixed == "T" { "P" } else { "T" })?;
    let other = k::list_of(&a, if bubble { "y" } else { "x" })?;
    Some((val, *other.first()?))
}

/// Bubble and dew lines at a fixed T or P: (x1, value, y1) on each (bubble: liquid x1 given; dew: vapour y1 given).
pub struct Vle {
    pub fixed: &'static str,
    pub fv: f64,
    pub bub: Vec<(f64, f64, f64)>,
    pub dew: Vec<(f64, f64, f64)>,
}

pub fn vle(s: &Solved, fixed: &'static str, fv: f64, n: usize) -> Option<Vle> {
    if s.mat("ant").map(|m| m.len()) != Some(2) {
        return None;
    }
    let mut bub = vec![];
    let mut dew = vec![];
    for c1 in k::span(0.0, 1.0, n, false) {
        if let Some((v, y)) = raoult(s, true, fixed, fv, c1) {
            bub.push((c1, v, y));
        }
        if let Some((v, x)) = raoult(s, false, fixed, fv, c1) {
            dew.push((x, v, c1));
        }
    }
    (bub.len() >= 5 && dew.len() >= 5).then_some(Vle { fixed, fv, bub, dew })
}

fn disp(fixed: &str, v: f64) -> f64 {
    if fixed == "T" { v / 1000.0 } else { v }
}

fn vname(fixed: &str) -> (&'static str, &'static str, &'static str) {
    // (the varying quantity, its unit as displayed, the axis title)
    if fixed == "T" { ("P", "kPa", "pressure P [kPa]") } else { ("T", "K", "temperature T [K]") }
}

fn fixed_text(d: &Vle) -> String {
    if d.fixed == "T" { format!("T = {}", q(d.fv, "K")) } else { format!("P = {}", q(d.fv, "Pa")) }
}

/// The P-x-y or T-x-y diagram with a tie line (liquid x1, vapour y1 at value v); `mark_liq`: the marked end is the
/// liquid (a bubble point) or the vapour (a dew point); `heat`: the dew value at the liquid's composition (a heating
/// path drawn up from the bubble point).
#[allow(clippy::too_many_arguments)]
pub fn xy_diagram(s: &Solved, d: &Vle, kind: &str, x1: f64, y1: f64, v: f64, label: &str, mark_liq: bool,
                  heat: Option<f64>, feed: Option<(f64, f64)>, title: &str, what: &str) -> Option<Figure> {
    let (vn, vu, vlab) = vname(d.fixed);
    let bub: Vec<(f64, f64)> = d.bub.iter().map(|p| (p.0, disp(d.fixed, p.1))).collect();
    // the dew line is the value against the VAPOUR composition
    let mut dew: Vec<(f64, f64)> = d.dew.iter().map(|p| (p.2, disp(d.fixed, p.1))).collect();
    dew.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let vd = disp(d.fixed, v);
    let (lo, hi) = pl::range(bub.iter().chain(dew.iter()).map(|p| p.1).chain([vd]).chain(heat.map(|h| disp(d.fixed, h))))?;
    let mut p = Plot::chart((0.0, 1.0), (lo, hi), (false, false));
    p.frame("x1, y1: mole fraction of component 1", vlab);
    p.area(&bub, &dew, 1);
    p.line(&bub, 0);
    p.line(&dew, 2);
    p.seg((x1, vd), (y1, vd), false);
    p.pin(x1, vd);
    p.pin(y1, vd);
    // the two ends named under the tie line, outside it
    let (lx, la, ra) = if x1 <= y1 { (-6.0, "end", "start") } else { (6.0, "start", "end") };
    p.note(x1, vd, lx, 17.0, la, &format!("x1 = {}", num(x1)));
    p.note(y1, vd, -lx, 17.0, ra, &format!("y1 = {}", num(y1)));
    if let Some(hv) = heat {
        let hd = disp(d.fixed, hv);
        p.seg((x1, vd), (x1, hd), true);
        p.pin(x1, hd);
        p.note(x1, hd, 8.0, 4.0, "start", &format!("dew {vn} = {} {vu}", num(hd)));
    }
    if let Some((z, vf)) = feed {
        p.mark(z, vd, &format!("feed z1 = {}, V/F = {}", num(z), num(vf)));
    } else if mark_liq {
        p.mark(x1, vd, label);
    } else {
        p.mark(y1, vd, label);
    }
    let mut items = vec![li("two phases", "area:1"), li("bubble curve (liquid)", "line:0"), li("dew curve (vapour)", "line:2")];
    items.push(li(if heat.is_some() { "tie line, heating path" } else { "tie line" }, "bar:3"));
    legend(&mut p.c, 90.0, 22.0, &items);
    let n = d.bub.len().max(d.dew.len());
    Some(s.chart(kind, title,
                 &format!("{what} of the binary mixture at {} (Raoult's law): bubble and dew {} from the model at up to {n} \
                           compositions each; between them liquid and vapour coexist. The tie line joins the liquid \
                           x1 = {} and the vapour y1 = {} at {vn} = {} {vu}.{}", fixed_text(d),
                          if d.fixed == "T" { "pressures" } else { "temperatures" }, num(x1), num(y1), num(vd),
                          heat.map(|h| format!(" Heated at x1 = {}, the mixture is all vapour at the dew point {vn} = {} {vu}.",
                                               num(x1), num(disp(d.fixed, h)))).unwrap_or_default()), &p.body()))
}

/// The x-y equilibrium diagram from the bubble line, with the point (x1, y1) marked.
pub fn eq_diagram(s: &Solved, d: &Vle, kind: &str, x1: f64, y1: f64) -> Option<Figure> {
    let pts: Vec<(f64, f64)> = d.bub.iter().map(|p| (p.0, p.2)).collect();
    if !ok_pts(&pts) {
        return None;
    }
    let mut p = Plot::new(400.0, (124.0, 48.0, 436.0, 340.0), (0.0, 1.0), (0.0, 1.0), (false, false), (false, false), "");
    p.frame("x1: liquid mole fraction", "y1: vapour mole fraction");
    p.dashed(&[(0.0, 0.0), (1.0, 1.0)]);
    p.line(&pts, 0);
    p.seg((x1, 0.0), (x1, y1), false);
    p.seg((0.0, y1), (x1, y1), false);
    p.mark(x1, y1, &format!("x1 = {}, y1 = {}", num(x1), num(y1)));
    legend(&mut p.c, 130.0, 22.0, &[li("y = x", "dash"), li("equilibrium curve", "line:0")]);
    Some(s.figure(kind, "x-y equilibrium diagram",
                  &format!("Vapour-liquid equilibrium (x-y) diagram of the ideal binary mixture at {} (Raoult's law): the \
                            vapour in equilibrium with each liquid, from the model's bubble point at {} compositions. \
                            Marked: x1 = {}, y1 = {}.", fixed_text(d), pts.len(), num(x1), num(y1)), &p.body(), 400.0))
}

/// Bubble and dew P against T (fixed = "T" capabilities) or T against P (fixed = "P"), at composition z1.
pub fn pt_diagram(s: &Solved, kind: &str, fixed: &'static str, fv: f64, z1: f64, v: f64, bubble_marked: bool)
                  -> Option<Figure> {
    let mut bub = vec![];
    let mut dew = vec![];
    let grid = if fixed == "T" { k::span(fv - 40.0, fv + 30.0, 36, false) } else { k::span(fv / 10.0, fv * 3.0, 31, true) };
    for g in &grid {
        if let Some((val, _)) = raoult(s, true, fixed, *g, z1) {
            bub.push((disp_x(fixed, *g), disp_y(fixed, val)));
        }
        if let Some((val, _)) = raoult(s, false, fixed, *g, z1) {
            dew.push((disp_x(fixed, *g), disp_y(fixed, val)));
        }
    }
    if bub.len() < 5 || dew.len() < 5 {
        return None;
    }
    let (xlo, xhi) = pl::range(bub.iter().chain(dew.iter()).map(|p| p.0))?;
    let (ylo, yhi) = pl::range(bub.iter().chain(dew.iter()).map(|p| p.1))?;
    let (xlab, ylab, logx, logy) = if fixed == "T" {
        ("temperature T [K]", "pressure P [kPa] (log)", false, true)
    } else {
        ("pressure P [kPa] (log)", "temperature T [K]", true, false)
    };
    let mut p = Plot::chart((xlo, xhi), (ylo, yhi), (logx, logy));
    p.frame(xlab, ylab);
    p.area(&bub, &dew, 1);
    p.line(&bub, 0);
    p.line(&dew, 2);
    let (mx, my) = (disp_x(fixed, fv), disp_y(fixed, v));
    let (vn, vu) = if fixed == "T" { ("P", "kPa") } else { ("T", "K") };
    p.mark(mx, my, &format!("{} {vn} = {} {vu}", if bubble_marked { "bubble" } else { "dew" }, num(my)));
    legend(&mut p.c, 90.0, 22.0, &[li("liquid and vapour coexist", "area:1"), li("bubble point", "line:0"),
                                    li("dew point", "line:2")]);
    let (title, what) = if fixed == "T" {
        ("Bubble and dew pressure against temperature", "Bubble and dew pressure against temperature")
    } else {
        ("Bubble and dew temperature against pressure", "Bubble and dew temperature against pressure")
    };
    Some(s.chart(kind, title,
                 &format!("{what} for the composition z1 = {} (ideal mixture, Raoult's law), each line up to {} runs of the \
                           model; between them the mixture is partly vaporised. Marked: the {} point, {vn} = {} {vu}.",
                          num(z1), grid.len(), if bubble_marked { "bubble" } else { "dew" }, num(my)), &p.body()))
}

fn disp_x(fixed: &str, g: f64) -> f64 {
    if fixed == "T" { g } else { g / 1000.0 }
}
fn disp_y(fixed: &str, v: f64) -> f64 {
    if fixed == "T" { v / 1000.0 } else { v }
}

/// Every phase figure of one Raoult capability.
pub fn raoult_phase(s: &Solved) -> Vec<Option<Figure>> {
    let r = (|| -> Option<Vec<Option<Figure>>> {
        match s.key {
            "chemeng_raoult_bubble_pressure" => {
                let (x1, t, p, y1) = (*s.vec("x")?.first()?, s.inp("T")?, s.out("P")?, *s.out_list("y")?.first()?);
                let d = vle(s, "T", t, 41)?;
                Some(vec![xy_diagram(s, &d, "phase", x1, y1, p, &format!("P = {} kPa", num(p / 1000.0)), true, None, None,
                                     "Pressure-composition (P-x-y) diagram with the tie line", "Pressure-composition diagram"),
                          eq_diagram(s, &d, "phase-2", x1, y1),
                          pt_diagram(s, "phase-3", "T", t, x1, p, true)])
            }
            "chemeng_raoult_dew_pressure" => {
                let (y1, t, p, x1) = (*s.vec("y")?.first()?, s.inp("T")?, s.out("P")?, *s.out_list("x")?.first()?);
                let d = vle(s, "T", t, 41)?;
                Some(vec![xy_diagram(s, &d, "phase", x1, y1, p, &format!("P = {} kPa", num(p / 1000.0)), false, None, None,
                                     "Pressure-composition (P-x-y) diagram with the tie line", "Pressure-composition diagram"),
                          eq_diagram(s, &d, "phase-2", x1, y1),
                          pt_diagram(s, "phase-3", "T", t, y1, p, false)])
            }
            "chemeng_raoult_bubble_temperature" => {
                let (x1, pp, t, y1) = (*s.vec("x")?.first()?, s.inp("P")?, s.out("T")?, *s.out_list("y")?.first()?);
                let d = vle(s, "P", pp, 40)?;
                let tdew = raoult(s, false, "P", pp, x1).map(|r| r.0);
                Some(vec![xy_diagram(s, &d, "phase", x1, y1, t, &format!("bubble T = {} K", num(t)), true, None, None,
                                     "Boiling-point diagram: bubble and dew curves of a binary mixture", "Boiling-point diagram"),
                          xy_diagram(s, &d, "phase-2", x1, y1, t, &format!("bubble T = {} K", num(t)), true, tdew, None,
                                     "Boiling-point (T-x-y) diagram: heating the liquid from its bubble point",
                                     "Boiling-point diagram"),
                          eq_diagram(s, &d, "phase-3", x1, y1),
                          pt_diagram(s, "phase-4", "P", pp, x1, t, true)])
            }
            "chemeng_raoult_dew_temperature" => {
                let (y1, pp, t, x1) = (*s.vec("y")?.first()?, s.inp("P")?, s.out("T")?, *s.out_list("x")?.first()?);
                let d = vle(s, "P", pp, 40)?;
                Some(vec![xy_diagram(s, &d, "phase", x1, y1, t, &format!("dew T = {} K", num(t)), false, None, None,
                                     "Boiling-point (T-x-y) diagram with the tie line", "Boiling-point diagram"),
                          eq_diagram(s, &d, "phase-2", x1, y1),
                          pt_diagram(s, "phase-3", "P", pp, y1, t, false)])
            }
            "chemeng_raoult_isothermal_flash" => {
                let (z1, t, pp, vf) = (*s.vec("z")?.first()?, s.inp("T")?, s.inp("P")?, s.out("VF")?);
                let (x1, y1) = (*s.out_list("x")?.first()?, *s.out_list("y")?.first()?);
                let d = vle(s, "P", pp, 39)?;
                Some(vec![xy_diagram(s, &d, "phase", x1, y1, t, "", true, None, Some((z1, vf)),
                                     "Flash on the T-x-y diagram: the lever rule", "Boiling-point diagram")])
            }
            _ => None,
        }
    })();
    r.unwrap_or_default()
}

// ------------------------------------------------------------------------------------------------ reactors
/// Conversion against total volume for one CSTR (this capability), three equal CSTRs in series and a plug-flow reactor
/// (the model's sibling capabilities), the visitor's volume marked.
pub fn reactor_compare(s: &Solved) -> Option<Figure> {
    let (v0, v, kk, x) = (s.inp("v0")?, s.inp("V")?, s.inp("k")?, s.out("X")?);
    if !(v > 0.0) {
        return None;
    }
    let vs = k::span(3.0 * v / 60.0, 3.0 * v, 60, false);
    let cstr = s.curve("V", &vs, "X", &[]);
    let mut series3 = vec![];
    let mut pfr = vec![];
    for vt in &vs {
        if let Some(a) = s.solve_other("chemeng_cstrs_in_series_first_order",
                                       &[("v0", fl(v0)), ("V", fl(vt / 3.0)), ("k", fl(kk)), ("N", Input::Int(3))]) {
            if let Some(xx) = k::num_of(&a, "X") {
                series3.push((*vt, xx));
            }
        }
        if let Some(a) = s.solve_other("chemeng_pfr_conversion_first_order", &[("v0", fl(v0)), ("V", fl(*vt)), ("k", fl(kk))]) {
            if let Some(xx) = k::num_of(&a, "X") {
                pfr.push((*vt, xx));
            }
        }
    }
    if !ok_pts(&cstr) {
        return None;
    }
    let mut p = Plot::chart((0.0, 3.0 * v), (0.0, 1.0), (false, false));
    p.frame("total reactor volume V [m³]", "conversion X");
    p.line(&pfr, 2);
    p.line(&series3, 1);
    p.line(&cstr, 0);
    let xp = pfr.iter().min_by(|a, b| (a.0 - v).abs().partial_cmp(&(b.0 - v).abs()).unwrap()).map(|p| p.1);
    p.mark(v, x, &format!("{} (CSTR)", num(x)));
    legend(&mut p.c, 90.0, 22.0, &[li("plug flow (PFR)", "line:2"), li("3 equal CSTRs in series", "line:1"),
                                    li("one CSTR", "line:0")]);
    Some(s.chart("reactor", "Reactor conversion: CSTR, CSTRs in series, PFR",
                 &format!("Conversion of a first-order reaction (k = {} 1/s, feed {}) against total volume for one CSTR, \
                           three equal CSTRs in series and a plug-flow reactor, each computed by the model at 60 volumes.{} \
                           Marked: V = {}, X = {} (CSTR).", num(kk), q(v0, "m³/s"),
                          xp.map(|a| format!(" Near V = {} the PFR reaches X = {}.", q(v, "m³"), num(a))).unwrap_or_default(),
                          q(v, "m³"), num(x)), &p.body()))
}

/// A plug-flow reactor drawn as a tube shaded by the conversion along it, over the conversion against the volume from
/// the inlet; `volume`: the capability returns V for a wanted X (the curve is V for each smaller X).
pub fn pfr_profile(s: &Solved, volume: bool, second: bool) -> Option<Figure> {
    let pts: Vec<(f64, f64)> = if volume {
        let (x, vv) = (s.inp("X")?, s.out("V")?);
        let mut v = vec![];
        for xx in k::span(x / 40.0, x, 40, false) {
            if let Some(vol) = s.with(&[("X", xx)]).and_then(|a| k::num_of(&a, "V")) {
                v.push((vol, xx));
            }
        }
        if (v.last()?.0 - vv).abs() > 1e-6 * vv.abs().max(1e-12) {
            return None;
        }
        v
    } else {
        let v = s.inp("V")?;
        let vs = k::span(v / 40.0, v, 40, false);
        s.curve("V", &vs, "X", &[])
    };
    if pts.len() < 10 {
        return None;
    }
    let (vend, xend) = *pts.last()?;
    let mut all = vec![(0.0, 0.0)];
    all.extend(pts.iter().copied());
    let mut p = Plot::new(360.0, (86.0, 150.0, 500.0, 296.0), (0.0, vend), (0.0, 1.0), (false, false), (false, false), "");
    p.frame("reactor volume from the inlet V [m³]", "conversion X");
    p.fill_under(&all, 0);
    p.line(&all, 0);
    p.mark(vend, xend, &format!("V = {}, X = {}", q(vend, "m³"), num(xend)));
    legend(&mut p.c, 100.0, 132.0, &[li("conversion along the reactor (model)", "line:0")]);
    // the tube: slices shaded by the conversion reached there
    let mut tube = String::new();
    let (tx0, tx1, ty0, ty1) = (p.x0, p.x1, 50.0, 82.0);
    let mut prev = 0.0;
    for (vv, xx) in &pts {
        let a = tx0 + (tx1 - tx0) * prev / vend;
        let b = tx0 + (tx1 - tx0) * vv / vend;
        tube.push_str(&format!("<rect class=\"axp-hp\" x=\"{a:.1}\" y=\"{ty0}\" width=\"{:.1}\" height=\"{}\" fill-opacity=\"{:.2}\"/>",
                               (b - a + 0.6).max(0.6), ty1 - ty0, 0.06 + 0.6 * xx.clamp(0.0, 1.0)));
        prev = *vv;
    }
    tube.push_str(&format!("<rect class=\"axp-k\" x=\"{tx0}\" y=\"{ty0}\" width=\"{:.1}\" height=\"{}\"/>", tx1 - tx0, ty1 - ty0));
    p.c.raw(&tube);
    p.c.name(tx0, 40.0, "start", "feed (shade: conversion)");
    p.c.value(tx1, 40.0, "end", &format!("X = {}", num(xend)));
    let order = if second { "second" } else { "first" };
    let caption = if volume {
        format!("A plug-flow reactor for a {order}-order reaction: the tube drawn to the length of its volume axis, over \
                 the conversion reached at each volume from the inlet (the model's volume for {} conversions up to the \
                 one wanted). Marked: V = {}, X = {}.", pts.len(), q(vend, "m³"), num(xend))
    } else {
        format!("A plug-flow reactor for a {order}-order reaction: the tube drawn to the length of its volume axis, over \
                 the conversion reached at each volume from the inlet (the model's plug-flow conversion at {} volumes). \
                 Marked: V = {}, X = {}.", pts.len(), q(vend, "m³"), num(xend))
    };
    Some(s.figure("reactor", &format!("Plug-flow reactor: conversion along its volume ({order}-order)"), &caption,
                  &p.body(), 360.0))
}

/// Conversion for N = 1 to 5 tanks in series (rows) and five tank volumes (columns), each cell the model's result.
pub fn series_table(s: &Solved) -> Option<Figure> {
    let v = s.inp("V")?;
    let n = nint(s, "N")?;
    let fs = [0.25, 0.5, 1.0, 2.0, 4.0];
    let mut m = vec![];
    for nn in 1..=5i64 {
        let mut row = vec![];
        for f in fs {
            let a = s.with_input(&[("N", Input::Int(nn)), ("V", fl(v * f))])?;
            row.push(k::num_of(&a, "X")?);
        }
        m.push(row);
    }
    let rows: Vec<String> = (1..=5).map(|i| format!("N = {i}")).collect();
    let cols: Vec<String> = fs.iter().map(|f| num(v * f)).collect();
    let body = k::heat(&m, &rows, &cols, "conversion X by N and tank V [m³]");
    Some(s.chart("heatmap", "Conversion table",
                 &format!("Conversion of a first-order reaction in N equal CSTRs in series (rows, N = 1 to 5) with each tank \
                           of volume {} (columns), every cell computed by the model. More tanks of the same size convert \
                           more. The visitor's train: N = {n}, V = {}, X = {}.", cols.join(", "), q(v, "m³"),
                          num(s.out("X")?)), &body))
}

// ------------------------------------------------------------------------------------------------ fits
/// The fitted line must pass among the data (a unit mismatch would put it far away): largest residual within a
/// quarter of the data's span.
fn fits(data: &[(f64, f64)], line: impl Fn(f64) -> f64) -> bool {
    let Some((lo, hi)) = pl::range(data.iter().map(|p| p.1)) else { return false };
    let span = (hi - lo).abs().max(hi.abs() * 0.05).max(1e-300);
    data.iter().all(|p| (line(p.0) - p.1).abs() <= 0.25 * span)
}

#[allow(clippy::too_many_arguments)]
fn fit_chart(s: &Solved, data: &[(f64, f64)], line: &[(f64, f64)], xr: (f64, f64), logx: bool, xlab: &str, ylab: &str,
             dname: &str, lname: &str, marks: &[(f64, f64, String)], title: &str, caption: &str) -> Option<Figure> {
    let (ylo, yhi) = pl::range(data.iter().chain(line.iter()).map(|p| p.1).chain(marks.iter().map(|m| m.1)))?;
    let mut p = Plot::chart(xr, (ylo, yhi), (logx, false));
    p.frame(xlab, ylab);
    p.line(line, 0);
    p.dots(data, 1, 4.5);
    for m in marks {
        p.mark(m.0, m.1, &m.2);
    }
    legend(&mut p.c, 90.0, 22.0, &[li(dname, "dot:1"), li(lname, "line:0")]);
    Some(s.chart("fit", title, caption, &p.body()))
}

pub fn filtration_fit(s: &Solved) -> Option<Figure> {
    let (t, v) = (s.vec("t")?, s.vec("V")?);
    let ft = tfac(&s.txt("t_unit")?)?;
    let vu = s.txt("v_unit")?;
    let fv = match vu.as_str() { "L" => 1e-3, "m3" => 1.0, _ => return None };
    let (slope, icpt) = (s.out("slope")?, s.out("intercept")?);
    if t.len() != v.len() || v.iter().any(|x| *x <= 0.0) {
        return None;
    }
    let data: Vec<(f64, f64)> = t.iter().zip(&v).map(|(a, b)| (*b, a * ft / b)).collect();
    let line_at = |x: f64| (slope * x * fv + icpt) * fv;
    if !fits(&data, line_at) {
        return None;
    }
    let (_, xmax) = pl::range(v.iter().copied())?;
    let xs = [0.0, xmax * 1.1];
    let line: Vec<(f64, f64)> = xs.iter().map(|x| (*x, line_at(*x))).collect();
    let unit = if vu == "L" { "L" } else { "m³" };
    fit_chart(s, &data, &line, (0.0, xmax * 1.1), false, &format!("filtrate volume V [{unit}]"),
              &format!("t/V, time per volume [s/{unit}]"), "measurements, t/V", "fitted straight line",
              &[(0.0, line_at(0.0), format!("intercept {} s/{unit}", num(line_at(0.0))))], "Filtration data on the t/V-V line",
              &format!("Constant-pressure filtration data ({} times and filtrate volumes) as t/V against V, with the straight \
                        line the model fits (slope {} s/m⁶, intercept {} s/m³, drawn per {unit}). From the line: specific cake \
                        resistance {} m/kg and medium resistance {} 1/m.", t.len(), num(slope), num(icpt),
                       num(s.out("alpha")?), num(s.out("Rm")?)))
}

pub fn step_fit(s: &Solved) -> Option<Figure> {
    let (t, y) = (s.vec("t")?, s.vec("y")?);
    let tu = s.txt("t_unit")?;
    let ft = tfac(&tu)?;
    let (m, theta) = (s.inp("M")?, s.inp("theta")?);
    let (kk, tau) = (s.out("K")?, s.out("tau")?);
    if t.len() != y.len() {
        return None;
    }
    let (_, tmax) = pl::range(t.iter().copied())?;
    let step = |ts: f64| -> Option<f64> {
        let a = s.solve_other("chemeng_fopdt_step_response", &[("K", fl(kk)), ("tau", fl(tau)), ("theta", fl(theta)),
                                                              ("M", fl(m)), ("t", fl(ts))])?;
        k::num_of(&a, "y")
    };
    let mut line = vec![];
    for td in k::span(0.0, tmax, 81, false) {
        if let Some(v) = step(td * ft) {
            line.push((td, v));
        }
    }
    if line.len() < 10 {
        return None;
    }
    let data: Vec<(f64, f64)> = t.iter().copied().zip(y.iter().copied()).collect();
    let fin = step((theta + tau) * 1000.0);
    let tm = (theta + tau) / ft;
    let ym = step(theta + tau)?;
    let (ylo, yhi) = pl::range(data.iter().chain(line.iter()).map(|p| p.1).chain(fin))?;
    let mut p = Plot::chart((0.0, tmax), (ylo.min(0.0), yhi), (false, false));
    p.frame(&format!("time t [{tu}]"), "output deviation y");
    if let Some(f) = fin {
        p.hguide(f, &format!("K·M = {}", num(f)));
    }
    p.line(&line, 0);
    p.dots(&data, 1, 4.5);
    p.mark(tm, ym, &format!("K = {}, τ = {}", num(kk), q(tau, "s")));
    legend(&mut p.c, 90.0, 22.0, &[li("measured response (input)", "dot:1"), li("fitted first-order model", "line:0")]);
    Some(s.chart("fit", "Step-test data and the fitted first-order model",
                 &format!("A measured step response ({} samples in {tu}, step size M = {}, dead time θ = {}) and the \
                           first-order model fitted by the model, K = {}, τ = {}, drawn with its own step-response capability \
                           at {} times. Marked: the model at t = θ + τ.{}", t.len(), num(m), q(theta, "s"), num(kk),
                          q(tau, "s"), line.len(), fin.map(|f| format!(" The final value K·M = {}.", num(f))).unwrap_or_default()),
                 &p.body()))
}

const SCF: f64 = 0.028316846592;
const BBL: f64 = 0.158987294928;

pub fn pz_fit(s: &Solved) -> Option<Figure> {
    let (p, z, gp) = (s.vec("p")?, s.vec("z")?, s.vec("Gp")?);
    let pu = s.txt("p_unit")?;
    let gu = s.txt("gp_unit")?;
    let fp = pfac(&pu)?;
    let fg = match gu.as_str() { "m3" => 1.0, "MMscf" => 1e6 * SCF, "Bscf" => 1e9 * SCF, _ => return None };
    let (g, pz0) = (s.out("G")? / fg, s.out("pz_initial")? / fp);
    if p.len() != z.len() || p.len() != gp.len() || z.iter().any(|x| *x == 0.0) || !(g > 0.0) {
        return None;
    }
    let data: Vec<(f64, f64)> = gp.iter().zip(p.iter().zip(&z)).map(|(a, (b, c))| (*a, b / c)).collect();
    let line_at = |x: f64| pz0 * (1.0 - x / g);
    if !fits(&data, line_at) {
        return None;
    }
    let line = vec![(0.0, pz0), (g, 0.0)];
    fit_chart(s, &data, &line, (0.0, g * 1.04), false, &format!("cumulative gas produced Gp [{gu}]"),
              &format!("p/z [{pu}]"), "p/z from the data", "fitted p/z line",
              &[(g, 0.0, format!("G = {} {gu}", num(g))), (0.0, pz0, format!("p/z initial {} {pu}", num(pz0)))],
              "Gas reservoir p/z plot and gas in place",
              &format!("Gas material balance ({} surveys): p/z of each survey (from the inputs) against cumulative \
                        production, and the straight line the model fits. Marked: initial p/z = {} {pu} and the gas in \
                        place G = {} {gu} where the line reaches p/z = 0.", p.len(), num(pz0), num(g)))
}

pub fn havlena_cap(s: &Solved) -> Option<Figure> {
    let (f, eo, eg) = (s.vec("F")?, s.vec("Eo")?, s.vec("Eg")?);
    let vu = s.txt("v_unit")?;
    let fv = match vu.as_str() { "m3" => 1.0, "bbl" => BBL, _ => return None };
    let (n, m) = (s.out("N")? / fv, s.out("m")?);
    if f.len() != eo.len() || f.len() != eg.len() || eo.iter().any(|x| *x == 0.0) {
        return None;
    }
    let data: Vec<(f64, f64)> = f.iter().zip(eo.iter().zip(&eg)).map(|(a, (b, c))| (c / b, a / b / 1e6)).collect();
    let line_at = |x: f64| (n + n * m * x) / 1e6;
    if !fits(&data, line_at) {
        return None;
    }
    let (_, xmax) = pl::range(data.iter().map(|p| p.0))?;
    let line = vec![(0.0, line_at(0.0)), (xmax * 1.1, line_at(xmax * 1.1))];
    fit_chart(s, &data, &line, (0.0, xmax * 1.1), false, "Eg/Eo", &format!("F/Eo [million {vu}]"),
              "data ratios (from the inputs)", "fitted line: intercept N, slope N·m",
              &[(0.0, line_at(0.0), format!("N = {} M{vu}", num(n / 1e6)))], "Havlena-Odeh plot with a gas cap",
              &format!("Havlena-Odeh straight line with a gas cap ({} points): F/Eo against Eg/Eo (ratios of the inputs) \
                        and the line the model fits. Marked: the intercept, oil initially in place N = {} {vu}; gas-cap ratio \
                        m = {}.", f.len(), num(n), num(m)))
}

pub fn havlena_oil(s: &Solved) -> Option<Figure> {
    let (f, et) = (s.vec("F")?, s.vec("Et")?);
    let vu = s.txt("v_unit")?;
    let fv = match vu.as_str() { "m3" => 1.0, "bbl" => BBL, _ => return None };
    let n = s.out("N")? / fv;
    if f.len() != et.len() {
        return None;
    }
    let data: Vec<(f64, f64)> = et.iter().copied().zip(f.iter().map(|x| x / 1e6)).collect();
    let line_at = |x: f64| n * x / 1e6;
    if !fits(&data, line_at) {
        return None;
    }
    let (_, xmax) = pl::range(et.iter().copied())?;
    let line = vec![(0.0, 0.0), (xmax * 1.1, line_at(xmax * 1.1))];
    let last = *data.last()?;
    fit_chart(s, &data, &line, (0.0, xmax * 1.1), false, "total expansion per unit of oil in place Et",
              &format!("withdrawal F [million {vu}]"), "data (input)", "fitted line through the origin, slope N",
              &[(last.0, line_at(last.0), format!("N = {} M{vu}", num(n / 1e6)))], "Havlena-Odeh plot: oil in place",
              &format!("Havlena-Odeh straight line for an oil reservoir without gas cap or aquifer ({} points): the model \
                        fits F = N·Et through the origin. Marked: the slope, oil initially in place N = {} {vu}.", f.len(),
                       num(n)))
}

pub fn horner_fit(s: &Solved) -> Option<Figure> {
    let (dt, pws) = (s.vec("dt")?, s.vec("pws")?);
    let ft = tfac(&s.txt("t_unit")?)?;
    let pu = s.txt("p_unit")?;
    let fp = pfac(&pu)?;
    let tp = s.inp("tp")?;
    let (m, pstar) = (s.out("m")? / fp, s.out("p_star")? / fp);
    if dt.len() != pws.len() || dt.iter().any(|x| *x <= 0.0) {
        return None;
    }
    let data: Vec<(f64, f64)> = dt.iter().zip(&pws).map(|(d, p)| ((tp + d * ft) / (d * ft), *p)).collect();
    let line_at = |x: f64| pstar - m * x.log10();
    if !fits(&data, line_at) {
        return None;
    }
    let (_, xmax) = pl::range(data.iter().map(|p| p.0))?;
    let xs = k::span(1.0, xmax * 1.3, 20, true);
    let line: Vec<(f64, f64)> = xs.iter().map(|x| (*x, line_at(*x))).collect();
    fit_chart(s, &data, &line, (1.0, xmax * 1.3), true, "Horner time ratio (tp + Δt)/Δt (log)",
              &format!("shut-in pressure pws [{pu}]"), "buildup data (ratios of the inputs)", "fitted semilog line",
              &[(1.0, pstar, format!("p* = {} {pu}", num(pstar)))], "Horner buildup plot",
              &format!("Horner plot of a pressure buildup ({} shut-in times, producing time tp = {}): pressure against the \
                        Horner ratio on a logarithmic axis, with the line the model fits: slope m = {} {pu} per log cycle, \
                        extrapolated pressure p* = {} {pu}, permeability {} m².", dt.len(), q(tp, "s"), num(m),
                       num(pstar), num(s.out("k")?)))
}

pub fn backpressure_xy(s: &Solved) -> Option<Figure> {
    let (qv, pwf) = (s.vec("q")?, s.vec("pwf")?);
    let (qu, pu) = (s.txt("q_unit")?, s.txt("p_unit")?);
    let (n, aof) = (s.out("n")?, s.out("AOF")?);
    if qv.len() != pwf.len() {
        return None;
    }
    let data: Vec<(f64, f64)> = qv.iter().copied().zip(pwf.iter().copied()).collect();
    let (xlo, xhi) = pl::range(qv.iter().copied())?;
    let (ylo, yhi) = pl::range(pwf.iter().copied())?;
    let mut p = Plot::chart((xlo, xhi), (ylo, yhi), (false, false));
    p.xa = pl::Ax::new(xlo, xhi, false, p.x0, p.x1, true);
    p.frame(&format!("test gas rate q [{}]", qu.replace('_', " ")), &format!("flowing pressure pwf [{pu}]"));
    p.dots(&data, 0, 5.0);
    p.c.value(p.x1 - 6.0, p.y0 + 18.0, "end", &format!("n = {}, AOF = {}", num(n), q(aof, "m³/s")));
    legend(&mut p.c, 90.0, 22.0, &[li("data (input)", "dot:0")]);
    Some(s.chart("xy", "Gas-well deliverability from a back-pressure test: the data",
                 &format!("The test data pwf against q as given ({} points). Returned from them: deliverability exponent \
                           n = {}, log10 C = {}, absolute open flow AOF = {}.", data.len(), num(n), num(s.out("log10_C")?),
                          q(aof, "m³/s")), &p.body()))
}

// ------------------------------------------------------------------------------------------------ bars
pub fn flue_bars(s: &Solved) -> Option<Figure> {
    let mut labels = vec![];
    let mut vals = vec![];
    for (sym, name) in [("n_CO2", "CO₂"), ("n_H2O", "H₂O"), ("n_SO2", "SO₂"), ("n_O2", "O₂"), ("n_N2", "N₂")] {
        let v = s.out(sym)?;
        if sym == "n_SO2" && v == 0.0 {
            continue;
        }
        labels.push(name.to_string());
        vals.push(v);
    }
    let hl: Vec<usize> = (0..labels.len().saturating_sub(1)).collect();
    let (co2, o2) = (s.out("y_CO2_dry")? * 100.0, s.out("y_O2_dry")? * 100.0);
    let body = k::bars(&labels, &vals, &hl, "", "moles per mole of fuel",
                       &format!("dry CO₂ {} %, dry O₂ {} %", num(co2), num(o2)));
    let fuel = s.txt("fuel").unwrap_or_default();
    Some(s.chart("bars", "Flue-gas composition per mole of fuel",
                 &format!("Flue gas from burning {fuel} with {} % excess air: moles of each product per mole of fuel, from \
                           the model ({}). Dry-basis CO₂ {} % and O₂ {} %, air {} mol per mol fuel, mass air-fuel ratio {}.",
                          num(s.inp("excess_air")? * 100.0),
                          labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}", num(*v))).collect::<Vec<_>>().join(", "),
                          num(co2), num(o2), num(s.out("air")?), num(s.out("AFR_mass")?)), &body))
}

pub fn mole_bars(s: &Solved) -> Option<Figure> {
    let (w, mw) = (s.vec("w")?, s.vec("MW")?);
    let z = s.out_list("z")?;
    let mm = s.out("MW_mean")? * 1000.0;
    if w.len() != z.len() || mw.len() != z.len() || z.len() > 8 {
        return None;
    }
    let labels: Vec<String> = mw.iter().enumerate().map(|(i, m)| format!("{}: M = {}", i + 1, num(*m))).collect();
    let groups = vec![("mass fraction w".to_string(), w.iter().map(|x| Some(*x)).collect()),
                      ("mole fraction z".to_string(), z.iter().map(|x| Some(*x)).collect())];
    let body = k::bar_groups(&labels, &groups, "", "fraction", &format!("mean molar mass {} g/mol", num(mm)));
    Some(s.chart("bars", "Mass fractions to mole fractions",
                 &format!("Mass fractions w = {} of {} components with molar masses {} g/mol converted to mole fractions. \
                           Marked: the model's z = {} and mean molar mass {} g/mol; the lighter components gain share by \
                           moles.", pl::list3(&w), z.len(), pl::list3(&mw), pl::list3(&z), num(mm)), &body))
}

/// Feed, liquid and vapour compositions of a flash, grouped by component; labels under each group: `klab`.
pub fn flash_bars(s: &Solved, z: &[f64], klab: &[String]) -> Option<Figure> {
    let vf = s.out("VF")?;
    let (x, y) = (s.out_list("x")?, s.out_list("y")?);
    if x.len() != z.len() || y.len() != z.len() || z.len() > 6 {
        return None;
    }
    let labels: Vec<String> = klab.iter().enumerate().map(|(i, kk)| format!("{}: K = {kk}", i + 1)).collect();
    let groups = vec![("feed z".to_string(), z.iter().map(|v| Some(*v)).collect()),
                      ("liquid x".to_string(), x.iter().map(|v| Some(*v)).collect()),
                      ("vapour y".to_string(), y.iter().map(|v| Some(*v)).collect())];
    let body = k::bar_groups(&labels, &groups, "component", "mole fraction",
                             &format!("V/F = {}, L/F = {}", num(vf), num(1.0 - vf)));
    Some(s.chart("bars", "Flash separation: feed, liquid and vapour compositions",
                 &format!("Flash of a {}-component feed z = {}: the model's vapour fraction V/F = {} with liquid x = {} and \
                           vapour y = {} (labels under each group: K values). Components with K > 1 concentrate in the \
                           vapour.", z.len(), pl::list3(z), num(vf), pl::list3(&x), pl::list3(&y)), &body))
}

/// Where each component of one mole of feed goes: V/F·y to the vapour, L/F·x to the liquid (from the returned values).
pub fn split_bars(s: &Solved) -> Option<Figure> {
    let z = s.vec("z")?;
    let vf = s.out("VF")?;
    let (x, y) = (s.out_list("x")?, s.out_list("y")?);
    if x.len() != z.len() || y.len() != z.len() || z.len() > 6 {
        return None;
    }
    let labels: Vec<String> = (1..=z.len()).map(|i| format!("component {i}")).collect();
    let groups = vec![("in the feed z".to_string(), z.iter().map(|v| Some(*v)).collect()),
                      ("to the vapour V/F·y".to_string(), y.iter().map(|v| Some(v * vf)).collect()),
                      ("to the liquid L/F·x".to_string(), x.iter().map(|v| Some(v * (1.0 - vf))).collect())];
    let body = k::bar_groups(&labels, &groups, "", "moles per mole of feed", &format!("V/F = {}", num(vf)));
    Some(s.figure("bars-2", "Where each component of the feed goes",
                  &format!("Each component of one mole of feed z = {} split between the phases: V/F·y to the vapour and \
                            L/F·x to the liquid, from the model's V/F = {}, y = {} and x = {}; the two parts add up to the \
                            feed.", pl::list3(&z), num(vf), pl::list3(&y), pl::list3(&x)), &body, 340.0))
}

// ------------------------------------------------------------------------------------------------ maps
/// A value over two inputs (21 x 15 runs of the model), the visitor's point marked.
#[allow(clippy::too_many_arguments)]
fn map(s: &Solved, kind: &str, out: &str, xs_sym: &str, xs: Vec<f64>, ys_sym: &str, ys: Vec<f64>, log: (bool, bool),
       xlab: &str, ylab: &str, title: &str, what: &str) -> Option<Figure> {
    let (mx, my, v) = (s.inp(xs_sym)?, s.inp(ys_sym)?, s.out(out)?);
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let r = s.with(&[(xs_sym, *x), (ys_sym, *y)]).and_then(|a| k::num_of(&a, out));
            n += r.is_some() as usize;
            col.push(r);
        }
        vals.push(col);
    }
    if n < 20 {
        return None;
    }
    let body = k::field(&xs, &ys, &vals, (mx, my), xlab, ylab, &format!("{out} = {}", num(v)), log);
    Some(s.chart(kind, title,
                 &format!("{what} over {xlab} and {ylab}: each of the {} × {} cells is one run of the model, shaded by its \
                           value (darker = larger; refused cells blank). Marked: the visitor's point, {out} = {}.",
                          xs.len(), ys.len(), num(v)), &body))
}

pub fn batch_conv_map(s: &Solved) -> Option<Figure> {
    let (kk, t) = (s.inp("k")?, s.inp("t")?);
    if !(kk > 0.0 && t > 0.0) {
        return None;
    }
    map(s, "field", "X", "k", k::span(kk / 10.0, kk * 10.0, 21, true), "t", k::span(t / 10.0, t * 10.0, 15, true),
        (true, true), "rate constant k [1/s]", "reaction time t [s]", "Batch conversion (first order) over rate constant and time",
        "Conversion X of a first-order batch reaction")
}

pub fn batch_time_map(s: &Solved) -> Option<Figure> {
    let (kk, x) = (s.inp("k")?, s.inp("X")?);
    if !(kk > 0.0) || !(0.0..1.0).contains(&x) {
        return None;
    }
    map(s, "field", "t", "k", k::span(kk / 3.0, kk * 3.0, 21, true), "X", k::span(0.02, 0.98, 15, false), (true, false),
        "rate constant k [1/s]", "conversion wanted X", "Batch time (first order) over rate constant and conversion",
        "Reaction time t of a first-order batch reaction")
}

pub fn offset_map(s: &Solved) -> Option<Figure> {
    let (kc, kp) = (s.inp("Kc")?, s.inp("K")?);
    if !(kc > 0.0 && kp > 0.0) {
        return None;
    }
    map(s, "field", "offset", "Kc", k::span(kc / 10.0, kc * 10.0, 21, true), "K", k::span(kp / 10.0, kp * 10.0, 15, true),
        (true, true), "controller gain Kc", "process gain K", "Proportional control: steady-state offset over the two gains",
        "Steady-state offset per unit set-point change")
}

// ------------------------------------------------------------------------------------------------ responses
pub fn fopdt_response(s: &Solved) -> Option<Figure> {
    let (kk, tau, theta, m, t, y) = (s.inp("K")?, s.inp("tau")?, s.inp("theta")?, s.inp("M")?, s.inp("t")?, s.out("y")?);
    if !(tau > 0.0) || theta < 0.0 {
        return None;
    }
    let tend = (theta + 5.0 * tau).max(t * 1.1);
    let ts = k::span(0.0, tend, 81, false);
    let pts = s.curve("t", &ts, "y", &[]);
    if pts.len() < 10 {
        return None;
    }
    let fin = s.with(&[("t", (theta + tau) * 1000.0)]).and_then(|a| k::num_of(&a, "y"));
    let (lo, hi) = pl::range(pts.iter().map(|p| p.1).chain(fin).chain([0.0]))?;
    let mut p = Plot::chart((0.0, tend), (lo, hi), (false, false));
    p.frame("time since the step t [s]", "output deviation y");
    if let Some(f) = fin {
        p.hguide(f, &format!("final value {}", num(f)));
    }
    p.vguide(theta, "θ");
    p.vguide(theta + tau, "θ + τ");
    p.line(&pts, 0);
    p.mark(t, y, &format!("y = {} at {}", num(y), q(t, "s")));
    legend(&mut p.c, 90.0, 22.0, &[li("output (model)", "line:0"), li("final value, θ and θ + τ", "dash")]);
    Some(s.chart("response", "First-order-plus-dead-time step response",
                 &format!("Step response of a first-order-plus-dead-time process (gain K = {}, time constant τ = {}, dead \
                           time θ = {}) to a step of size M = {}: the model's output at {} times.{} Marked: y = {} at t = {}.",
                          num(kk), q(tau, "s"), q(theta, "s"), num(m), pts.len(),
                          fin.map(|f| format!(" Final value (the model at a long time): {}.", num(f))).unwrap_or_default(),
                          num(y), q(t, "s")), &p.body()))
}

pub fn lag_bode(s: &Solved) -> Option<Figure> {
    let (w, ar, ph) = (s.inp("w")?, s.out("AR")?, s.out("phase_deg")?);
    if !(w > 0.0) {
        return None;
    }
    let ws = k::span(w / 100.0, w * 100.0, 61, true);
    let mut a_pts = vec![];
    let mut p_pts = vec![];
    for x in &ws {
        if let Some(a) = s.with(&[("w", *x)]) {
            if let (Some(a1), Some(a2)) = (k::num_of(&a, "AR"), k::num_of(&a, "phase_deg")) {
                a_pts.push((*x, a1));
                p_pts.push((*x, a2));
            }
        }
    }
    if a_pts.len() < 10 {
        return None;
    }
    let h = 460.0;
    let (alo, ahi) = pl::range(a_pts.iter().map(|p| p.1))?;
    let mut p1 = Plot::new(h, (86.0, 40.0, 500.0, 196.0), (ws[0], ws[60]), (alo, ahi), (true, alo > 0.0), (false, true), "");
    p1.frame("", "amplitude ratio (log)");
    p1.line(&a_pts, 0);
    p1.mark(w, ar, &format!("AR = {}", num(ar)));
    legend(&mut p1.c, 100.0, 24.0, &[li("amplitude ratio", "line:0")]);
    let (plo, phi) = pl::range(p_pts.iter().map(|p| p.1))?;
    let mut p2 = Plot::new(h, (86.0, 256.0, 500.0, 400.0), (ws[0], ws[60]), (plo, phi), (true, false), (false, true), "2");
    p2.frame("frequency ω [rad/s]", "phase [°]");
    p2.line(&p_pts, 2);
    p2.mark(w, ph, &format!("{} °", num(ph)));
    legend(&mut p2.c, 100.0, 240.0, &[li("phase", "line:2")]);
    let taus = s.vec("taus").map(|v| pl::list3(&v)).unwrap_or_default();
    Some(s.figure("response", "Frequency response of first-order lags (Bode)",
                  &format!("Bode diagram of a gain K = {} with first-order lags τ = {taus} ({}): amplitude ratio (log-log, \
                            top) and phase (bottom) from the model at {} frequencies over four decades around the given one. \
                            Marked: ω = {} rad/s, AR = {}, phase {}°.", num(s.inp("K")?), s.txt("t_unit").unwrap_or_default(),
                           a_pts.len(), num(w), num(ar), num(ph)), &format!("{}{}", p1.body(), p2.body()), h))
}

pub fn pid_response(s: &Solved) -> Option<Figure> {
    let e = s.vec("e")?;
    let u = s.out_list("u")?;
    let (dt, bias) = (s.inp("dt")?, s.inp("bias")?);
    if e.len() != u.len() || e.is_empty() || !(dt > 0.0) || e.len() > 60 {
        return None;
    }
    let n = e.len();
    let tend = dt * (n as f64 + 1.0);
    let ep: Vec<(f64, f64)> = e.iter().enumerate().map(|(i, v)| (dt * (i + 1) as f64, *v)).collect();
    let up: Vec<(f64, f64)> = u.iter().enumerate().map(|(i, v)| (dt * (i + 1) as f64, *v)).collect();
    let h = 404.0;
    let (elo, ehi) = pl::range(e.iter().copied().chain([0.0]))?;
    let mut p1 = Plot::new(h, (86.0, 40.0, 500.0, 130.0), (0.0, tend), (elo, ehi), (false, false), (false, true), "");
    p1.frame("", "error e");
    p1.stems(&ep, 2);
    legend(&mut p1.c, 100.0, 24.0, &[li("error samples (input)", "dot:2")]);
    let (ulo, uhi) = pl::range(u.iter().copied().chain([bias]))?;
    let mut p2 = Plot::new(h, (86.0, 196.0, 500.0, 346.0), (0.0, tend), (ulo, uhi), (false, false), (false, true), "2");
    p2.frame("time t [s]", "controller output u");
    p2.hguide(bias, &format!("bias {}", num(bias)));
    p2.steps(&up, tend, 0);
    p2.dots(&up, 0, 3.5);
    if n <= 10 {
        for (i, (x, y)) in up.iter().enumerate().take(n - 1) {
            let dy = if i % 2 == 0 { -8.0 } else { 16.0 };
            p2.note(*x, *y, 4.0, dy, "start", &num(*y));
        }
    }
    p2.mark(up[n - 1].0, up[n - 1].1, &format!("last {}", num(up[n - 1].1)));
    legend(&mut p2.c, 100.0, 180.0, &[li("output, held each step (model)", "line:0")]);
    Some(s.figure("response", "PID controller: error samples and output",
                  &format!("A discrete PID controller (Kc = {}, τI = {}, τD = {}, sample time {}, bias {}) fed the error \
                            samples {} (top). Below: the model's outputs u = {}, last {}.", num(s.inp("Kc")?),
                           q(s.inp("tauI")?, "s"), q(s.inp("tauD")?, "s"), q(dt, "s"), num(bias), pl::list3(&e),
                           pl::list3(&u), num(u[n - 1])), &format!("{}{}", p1.body(), p2.body()), h))
}

// ------------------------------------------------------------------------------------------------ vapour pressure
/// The Antoine vapour pressure from the model at temperature t (K): this capability or its sibling.
fn antoine_p(s: &Solved, t: f64) -> Option<f64> {
    let set = vec![("A", gin(s, "A")?), ("B", gin(s, "B")?), ("C", gin(s, "C")?), ("units", gin(s, "units")?),
                   ("base", gin(s, "base")?), ("T", fl(t))];
    k::num_of(&s.solve_other("chemeng_antoine_vapour_pressure", &set)?, "Psat")
}

/// The vapour-pressure curve (chart) or its Clausius-Clapeyron form (chart-2); the marked point (T, P).
pub fn antoine(s: &Solved, clapeyron: bool) -> Option<Figure> {
    let (t, p) = if s.key == "chemeng_antoine_vapour_pressure" { (s.inp("T")?, s.out("Psat")?) } else {
        (s.out("Tsat")?, s.inp("P")?)
    };
    let n = if clapeyron { 41 } else { 56 };
    let mut pts = vec![];
    for tt in k::span(t - 70.0, t + 40.0, n, false) {
        if tt <= 0.0 {
            continue;
        }
        if let Some(pp) = antoine_p(s, tt) {
            if pp > 0.0 {
                pts.push((tt, pp / 1000.0));
            }
        }
    }
    if pts.len() < 10 {
        return None;
    }
    let what = if s.key == "chemeng_antoine_vapour_pressure" {
        format!("Psat = {} kPa at {} K", num(p / 1000.0), num(t))
    } else {
        format!("Tsat = {} K at {} kPa", num(t), num(p / 1000.0))
    };
    if clapeyron {
        let cp: Vec<(f64, f64)> = pts.iter().map(|(tt, pp)| (1000.0 / tt, *pp)).collect();
        let (xlo, xhi) = pl::range(cp.iter().map(|p| p.0))?;
        let (ylo, yhi) = pl::range(cp.iter().map(|p| p.1))?;
        let mut pl_ = Plot::chart((xlo, xhi), (ylo, yhi), (false, true));
        pl_.frame("reciprocal temperature 1000/T [1/K]", "Psat [kPa] (log)");
        pl_.line(&cp, 2);
        pl_.mark(1000.0 / t, p / 1000.0, &format!("{} kPa at {} K", num(p / 1000.0), num(t)));
        legend(&mut pl_.c, 90.0, 22.0, &[li("vapour pressure (model)", "line:2")]);
        return Some(s.chart("chart-2", "Vapour pressure on Clausius-Clapeyron axes",
                            &format!("The model's Antoine vapour pressures (A = {}, B = {}, C = {}) on Clausius-Clapeyron \
                                      axes, log pressure against 1000/T: nearly a straight line whose slope reflects the heat \
                                      of vaporisation; {} runs of the model. Marked: {what}.", num(s.inp("A")?),
                                     num(s.inp("B")?), num(s.inp("C")?), cp.len()), &pl_.body()));
    }
    let (xlo, xhi) = pl::range(pts.iter().map(|p| p.0))?;
    let (ylo, yhi) = pl::range(pts.iter().map(|p| p.1))?;
    let mut pl_ = Plot::chart((xlo, xhi), (ylo, yhi), (false, true));
    pl_.frame("temperature T [K]", "vapour pressure [kPa] (log)");
    pl_.hguide(p / 1000.0, &format!("{} kPa", num(p / 1000.0)));
    pl_.line(&pts, 0);
    pl_.mark(t, p / 1000.0, &what);
    legend(&mut pl_.c, 90.0, 22.0, &[li("vapour pressure (Antoine, model)", "line:0")]);
    Some(s.chart("chart", "Antoine vapour-pressure curve",
                 &format!("Vapour-pressure curve from the Antoine constants A = {}, B = {}, C = {} ({}, {}), {} temperatures \
                           from the model, logarithmic pressure axis. Marked: {what}.", num(s.inp("A")?), num(s.inp("B")?),
                          num(s.inp("C")?), s.txt("units").unwrap_or_default(), s.txt("base").unwrap_or_default(), pts.len()),
                 &pl_.body()))
}

// ------------------------------------------------------------------------------------------------ curve families
pub fn family(s: &Solved, x: &str, y: &str, param: &str, unit: &str, xlab: &str, ylab: &str, title: &str) -> Option<Figure> {
    let (xv, yv, pv) = (s.inp(x)?, s.out(y)?, s.inp(param)?);
    if !(xv > 0.0 && pv > 0.0) {
        return None;
    }
    let xs = k::span(xv / 10.0, xv * 10.0, 41, true);
    let fam = s.family(x, &xs, y, param, &[pv, pv / 2.0, pv * 2.0], unit);
    if fam.is_empty() {
        return None;
    }
    let body = k::lines(&fam, xlab, ylab, &eq(y, yv, unit_of(y)), Some((xv, yv)), &num(yv), true);
    Some(s.chart("curve-2", title,
                 &format!("{y} against {x} from {} to {} on a logarithmic axis, one curve for each {param}: {} (the given \
                           value, half and twice it), 41 runs of the model each. Marked: {x} = {}, {y} = {}.",
                          num(xs[0]), num(xs[40]), fam.iter().map(|f| f.0.clone()).collect::<Vec<_>>().join(", "), num(xv),
                          num(yv)), &body))
}

fn unit_of(sym: &str) -> &'static str {
    match sym {
        "T_out" => "K",
        _ => "",
    }
}

// ------------------------------------------------------------------------------------------------ diffusion profiles
/// The mole fraction of A at depth z in the film: the yA2' for which the model's flux across a film of thickness z
/// equals the computed flux (regula falsi on the model, between the two end values).
fn depth_value(s: &Solved, z: f64, na: f64, ya1: f64, ya2: f64) -> Option<f64> {
    let f = |y: f64| -> Option<f64> { Some(k::num_of(&s.with(&[("L", z), ("yA2", y)])?, "NA")? - na) };
    // the end at yA1 itself is a film with no driving force (no flux): start just inside it
    let near = ya1 - (ya1 - ya2) * 1e-6;
    let (mut a, mut b) = (near.min(ya2), near.max(ya2));
    let (mut fa, mut fb) = (f(a)?, f(b)?);
    if fa * fb > 0.0 {
        return None;
    }
    let mut side = 0;
    for _ in 0..30 {
        let c = (a * fb - b * fa) / (fb - fa);
        let fc = f(c)?;
        if fc.abs() <= 1e-9 * na.abs() || (b - a).abs() < 1e-12 {
            return Some(c);
        }
        if fc * fb > 0.0 {
            b = c;
            fb = fc;
            if side == -1 {
                fa /= 2.0;
            }
            side = -1;
        } else {
            a = c;
            fa = fc;
            if side == 1 {
                fb /= 2.0;
            }
            side = 1;
        }
    }
    Some((a + b) / 2.0)
}

pub fn diffusion_profile(s: &Solved, equimolar: bool) -> Option<Figure> {
    let (l, ya1, ya2, na) = (s.inp("L")?, s.inp("yA1")?, s.inp("yA2")?, s.out("NA")?);
    if !(l > 0.0) || na == 0.0 || ya1 == ya2 {
        return None;
    }
    let mut pts = vec![(0.0, ya1)];
    for z in k::span(l / 12.0, l * 11.0 / 12.0, 11, false) {
        if let Some(y) = depth_value(s, z, na, ya1, ya2) {
            pts.push((z * 100.0, y));
        }
    }
    pts.push((l * 100.0, ya2));
    if pts.len() < 8 {
        return None;
    }
    let b: Vec<(f64, f64)> = pts.iter().map(|p| (p.0, 1.0 - p.1)).collect();
    let yr = if equimolar { (0.0, 1.0) } else { pl::range(pts.iter().map(|p| p.1).chain([0.0]))? };
    let mut p = Plot::chart((0.0, l * 100.0), yr, (false, false));
    p.frame("position across the film z [cm]", if equimolar { "mole fraction" } else { "mole fraction of A" });
    p.line(&pts, 0);
    p.dots(&[pts[0], *pts.last()?], 0, 4.0);
    if equimolar {
        p.line(&b, 2);
    }
    // the flux arrows in the middle of the plot
    // equimolar: in the middle between the A and B lines; through stagnant B: in the empty upper right
    let (mx, my) = if equimolar { ((p.x0 + p.x1) / 2.0, (p.y0 + p.y1) / 2.0) } else { (p.x1 - 120.0, p.y0 + 46.0) };
    let arr = |c: &mut ax_engine::figkit::Canvas, y: f64, right: bool, k_: usize| {
        let (a, b) = if right { (mx - 60.0, mx + 60.0) } else { (mx + 60.0, mx - 60.0) };
        pl::stream(c, &[(a, y), (b, y)], k_);
    };
    arr(&mut p.c, my - 10.0, na > 0.0, 0);
    p.c.value(mx, my - 18.0, "middle", &format!("N_A = {}", num(na)));
    let mut cap_b = String::new();
    if equimolar {
        let nb = s.out("NB")?;
        arr(&mut p.c, my + 26.0, nb > 0.0, 2);
        p.c.value(mx, my + 46.0, "middle", &format!("N_B = {}", num(nb)));
        cap_b = format!(" and N_B = {} mol/(m²·s)", num(nb));
    }
    let mut items = vec![li("A", "line:0")];
    if equimolar {
        items.push(li("B", "line:2"));
    }
    legend(&mut p.c, 90.0, 22.0, &items);
    let (title, what) = if equimolar {
        ("Equimolar counter-diffusion across a film", "equimolar counter-diffusion")
    } else {
        ("Diffusion of A through stagnant B", "A through stagnant B")
    };
    Some(s.chart("profile", title,
                 &format!("Gas-phase diffusion ({what}) across a film of thickness L = {} (mole fraction of A {} at z = 0 \
                           and {} at z = L). Each inner point is the composition at which the model's flux across that \
                           depth equals the flux of the whole film, {} mol/(m²·s){cap_b}.", q(l, "m"), num(ya1), num(ya2),
                          num(na)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ single kinds
pub fn stability_decision(s: &Solved) -> Option<Figure> {
    let kc = s.inp("Kc")?;
    let mr = s.out("max_real")?;
    let verdict = s.out_text("stable")?;
    if !(kc > 0.0) {
        return None;
    }
    let ks = k::span(kc / 10.0, kc * 10.0, 81, true);
    let mut pts = vec![];
    let mut v: Vec<(f64, String)> = vec![];
    for x in &ks {
        if let Some(a) = s.with(&[("Kc", *x)]) {
            if let (Some(m), Some(t)) = (k::num_of(&a, "max_real"), txt_of(&a, "stable")) {
                pts.push((*x, m));
                v.push((*x, t));
            }
        }
    }
    if pts.len() < 10 {
        return None;
    }
    let mut names: Vec<String> = vec![];
    for (_, t) in &v {
        if !names.contains(t) {
            names.push(t.clone());
        }
    }
    let (lo, hi) = pl::range(pts.iter().map(|p| p.1).chain([0.0]))?;
    let mut p = Plot::chart((ks[0], ks[80]), (lo, hi), (true, false));
    p.frame("controller gain Kc", "largest real part of the poles");
    let mut i = 0;
    let mut change = None;
    while i < v.len() {
        let mut j = i;
        while j + 1 < v.len() && v[j + 1].1 == v[i].1 {
            j += 1;
        }
        let l = if i == 0 { v[0].0 } else { (v[i - 1].0 * v[i].0).sqrt() };
        let r = if j + 1 >= v.len() { v[j].0 } else { (v[j].0 * v[j + 1].0).sqrt() };
        let cls = names.iter().position(|n| n == &v[i].1).unwrap_or(0);
        p.vband(l, r, if cls == 0 { 0 } else { 1 });
        if i > 0 && change.is_none() {
            change = Some(l);
        }
        i = j + 1;
    }
    p.hguide(0.0, "0");
    p.line(&pts, 0);
    p.mark(kc, mr, &verdict);
    let items: Vec<(String, String)> = names.iter().enumerate()
        .map(|(i, n)| li(n, &format!("area:{}", if i == 0 { 0 } else { 1 }))).collect();
    legend(&mut p.c, 90.0, 22.0, &items);
    Some(s.chart("decision", "Closed-loop stability along the controller gain",
                 &format!("Stable or unstable along Kc from {} to {} (logarithmic axis): {} runs of the model, each band \
                           coloured by the verdict returned ({}){}; the line is the largest real part of the closed-loop \
                           poles. Marked: Kc = {}, {verdict}.", num(ks[0]), num(ks[80]), pts.len(), names.join(", "),
                          change.map(|c| format!("; it changes near Kc = {}", num(c))).unwrap_or_default(), num(kc)),
                 &p.body()))
}

pub fn decline(s: &Solved) -> Option<Figure> {
    let (qi, di, b, t, qq) = (s.inp("qi")?, s.inp("Di")?, s.inp("b")?, s.inp("t")?, s.out("q")?);
    if !(t > 0.0) {
        return None;
    }
    let year = 31_557_600.0;
    let (tf, tu) = if t >= 0.5 * year { (year, "years") } else { (86_400.0, "days") };
    let tend = 3.0 * t;
    let ts = k::span(0.0, tend, 81, false);
    let mut series = vec![];
    // the given exponent, and the exponential (b = 0) and harmonic (b = 1) laws where the model takes them, else
    // b = 0.1 and b = 0.9 (exponents the model does take)
    let curve = |bv: f64| -> Vec<(f64, f64)> {
        ts.iter().filter_map(|x| s.with(&[("b", bv), ("t", *x)]).and_then(|a| k::num_of(&a, "q"))
            .map(|v| (x / tf, v * 86_400.0))).collect()
    };
    let mut done: Vec<f64> = vec![];
    for (bv, alt) in [(b, None), (0.0, Some(0.1)), (1.0, Some(0.9))] {
        let mut tries = vec![bv];
        tries.extend(alt);
        for bb in tries {
            if done.iter().any(|d| (d - bb).abs() < 1e-12) {
                break;
            }
            let pts = curve(bb);
            if pts.len() >= 10 {
                let name = match bb {
                    x if x == 0.0 => "exponential, b = 0".to_string(),
                    x if x == 1.0 => "harmonic, b = 1".to_string(),
                    x => format!("b = {}", num(x)),
                };
                series.push((name, pts, bb == b));
                done.push(bb);
                break;
            }
        }
    }
    if series.is_empty() {
        return None;
    }
    let (lo, hi) = pl::range(series.iter().flat_map(|s| s.1.iter().map(|p| p.1)).chain([0.0]))?;
    let mut p = Plot::chart((0.0, tend / tf), (lo, hi), (false, false));
    p.frame(&format!("time t [{tu}]"), "rate q [m³/day]");
    let mut items = vec![];
    for (i, (name, pts, _)) in series.iter().enumerate() {
        p.line(pts, i);
        items.push(li(name, &format!("line:{i}")));
    }
    p.mark(t / tf, qq * 86_400.0, &format!("{} m³/day", num(qq * 86_400.0)));
    legend(&mut p.c, 90.0, 22.0, &items);
    Some(s.chart("decline", "Production decline for three decline exponents",
                 &format!("Well production rate against time for the decline laws from the same initial rate qi = {} m³/day \
                           and decline Di = {} per year, each computed by the model at 81 times over {} {tu}. Marked: the \
                           given b = {} at t = {} {tu}, q = {} m³/day.", num(qi * 86_400.0), num(di * year), num(tend / tf),
                          num(b), num(t / tf), num(qq * 86_400.0)), &p.body()))
}

pub fn rtd_data(s: &Solved) -> Option<Figure> {
    let (t, cc) = (s.vec("t")?, s.vec("C")?);
    let tu = s.txt("t_unit")?;
    let ft = tfac(&tu)?;
    let (tm, var, nt) = (s.out("t_mean")? / ft, s.out("variance")?, s.out("N_tanks")?);
    if t.len() != cc.len() || var < 0.0 {
        return None;
    }
    let sd = var.sqrt() / ft;
    let data: Vec<(f64, f64)> = t.iter().copied().zip(cc.iter().copied()).collect();
    let (xlo, xhi) = pl::range(t.iter().copied())?;
    let (_, yhi) = pl::range(cc.iter().copied())?;
    let mut p = Plot::chart((xlo, xhi), (0.0, yhi), (false, false));
    p.xa = pl::Ax::new(xlo, xhi, false, p.x0, p.x1, true);
    p.frame(&format!("time after the pulse t [{tu}]"), "outlet tracer concentration C");
    p.vband(tm - sd, tm + sd, 0);
    p.stems(&data, 0);
    p.vguide_top(tm, &format!("mean {} {tu}", num(tm)));
    legend(&mut p.c, 90.0, 22.0, &[li("tracer samples (input)", "dot:0"), li("mean ± one standard deviation", "area:0")]);
    Some(s.chart("data", "Residence-time distribution and its mean",
                 &format!("Tracer response to a pulse ({} samples in {tu}) with the moments the model computes: mean residence \
                           time {} ({} {tu}), variance {} s² (the shaded band is the mean ± one standard deviation, its \
                           square root), equivalent tanks in series {}.", t.len(), q(tm * ft, "s"), num(tm), num(var), num(nt)),
                 &p.body()))
}

pub fn npsh_diagram(s: &Solved) -> Option<Figure> {
    let (zs, hf) = (s.inp("z_s")?, s.inp("h_f")?);
    let (npsha, ph, vh) = (s.out("NPSHa")?, s.out("pressure_head")?, s.out("vapour_head")?);
    // the budget must add up to the model's NPSHa (else the drawing would contradict it)
    if (ph + zs - hf - vh - npsha).abs() > 1e-6 * npsha.abs().max(1.0) {
        return None;
    }
    let steps = [("pressure", 0.0, ph), ("static", ph, ph + zs), ("friction", ph + zs, ph + zs - hf),
                 ("vapour", ph + zs - hf, npsha), ("NPSHa", 0.0, npsha)];
    let (lo, hi) = pl::range(steps.iter().flat_map(|s| [s.1, s.2]).chain([0.0]))?;
    let npshr = s.inp("NPSHr");
    let hi = npshr.map(|r| hi.max(r)).unwrap_or(hi);
    let hi = hi + (hi - lo) * 0.12;
    let mut p = Plot::chart((0.0, 5.0), (lo, hi), (false, false));
    p.xa = pl::Ax::new(0.0, 5.0, false, p.x0, p.x1, false);
    p.xticks = false;
    p.frame("", "head [m of liquid]");
    let slot = (p.x1 - p.x0) / 5.0;
    let mut bars = String::new();
    for (i, (name, a, b)) in steps.iter().enumerate() {
        let x = p.x0 + slot * (i as f64 + 0.18);
        let (ya, yb) = (p.ya.px(*a), p.ya.px(*b));
        let cls = if i == 4 { "axp-b2" } else if b >= a { "axp-b" } else { "axp-b3" };
        bars.push_str(&format!("<rect class=\"{cls}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                               ya.min(yb), slot * 0.64, (ya - yb).abs().max(1.5)));
        p.c.text(x + slot * 0.32, p.y1 + 18.0, "middle", "axp-t", name);
        let d = b - a;
        let label = if i == 4 { format!("{} m", num(npsha)) } else { format!("{}{} m", if d >= 0.0 { "+" } else { "" }, num(d)) };
        p.c.value(x + slot * 0.32, ya.min(yb) - 6.0, "middle", &label);
    }
    p.c.raw(&bars);
    if let Some(r) = npshr {
        p.hguide(r, &format!("NPSHr {} m", num(r)));
    }
    Some(s.chart("diagram", "NPSH available: the suction head budget",
                 &format!("Net positive suction head available built up as a waterfall: pressure head {} m, static head {} m, \
                           friction loss {} m, vapour-pressure head {} m (the model's heads; the inputs for the static and \
                           friction terms). Marked: NPSHa = {} m{}.", num(ph), num(zs), num(hf), num(vh), num(npsha),
                          match (npshr, s.out("margin")) {
                              (Some(r), Some(m)) => format!(", required {} m, margin {} m", num(r), num(m)),
                              _ => String::new(),
                          }), &p.body()))
}

pub fn cp_matrix(s: &Solved) -> Option<Figure> {
    let cp = s.mat("cp")?;
    let dh = s.out("dH")?;
    if cp.len() > 8 || cp[0].len() > 6 {
        return None;
    }
    let rows: Vec<String> = (1..=cp.len()).map(|i| format!("species {i}")).collect();
    let cols: Vec<String> = (0..cp[0].len()).map(|j| format!("c{j}")).collect();
    let body = k::heat(&cp, &rows, &cols, &format!("cp coefficients (input); dH = {} J/mol", num(dh)));
    let nu = s.vec("nu").map(|v| pl::list3(&v)).unwrap_or_default();
    Some(s.chart("matrix", "Heat of reaction at another temperature (Kirchhoff): the heat-capacity matrix",
                 &format!("The heat-capacity coefficients cp (input; one row per species, one column per power of T) as a \
                           heat map, each cell shaded by its magnitude. With the stoichiometric coefficients {nu} the model \
                           returns dH = {} J/mol at T = {} (from {} J/mol at {}).", num(dh), q(s.inp("T")?, "K"),
                          num(s.inp("dH0")?), q(s.inp("T0")?, "K")), &body))
}

pub fn raoult_vectors(s: &Solved) -> Option<Figure> {
    let (x, g) = (s.vec("x")?, s.vec("gamma")?);
    let y = s.out_list("y")?;
    if x.len() != 2 || g.len() != 2 || y.len() != 2 {
        return None;
    }
    let v = vec![("x (input)".to_string(), (x[0], x[1]), false),
                 ("gamma (input)".to_string(), (g[0], g[1]), false),
                 ("y (returned)".to_string(), (y[0], y[1]), true)];
    let body = k::arrows(&v, &format!("P = {} kPa", num(s.out("P")? / 1000.0)));
    Some(s.chart("vectors", "Bubble-point pressure by the modified Raoult's law: the vectors",
                 &format!("The plane vectors of the request as arrows from the origin: input x = ({}, {}), gamma = ({}, {}); \
                           returned by the model: y = ({}, {}) at P = {} kPa. Marked: the tip of the returned vector.",
                          num(x[0]), num(x[1]), num(g[0]), num(g[1]), num(y[0]), num(y[1]), num(s.out("P")? / 1000.0)),
                 &body))
}
