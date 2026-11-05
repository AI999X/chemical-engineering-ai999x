//! Process diagrams of Chemical Engineering's figures: columns, stirred tanks, flash drums, extraction cascades and
//! flowsheets, each labelled with the inputs as read and the values the model returned (stage-by-stage values are the
//! model's own results for that many stages). A diagram the inputs make impossible is left out.

use crate::figures::{nint, q};
use crate::figures_plot::{self as pl, column, drum, legend, li, list3, stream, tank, Plot};
use ax_engine::figkit::{self as k, num, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

const M3S: &str = "m³/s";
const CONC: &str = "mol/m³";

fn n4(x: f64) -> String {
    num(x)
}

// ------------------------------------------------------------------------------------------------ columns
pub fn absorber_packed(s: &Solved) -> Option<Figure> {
    let (y1, y2, x2, m, a, hog) = (s.inp("y1")?, s.inp("y2")?, s.inp("x2")?, s.inp("m")?, s.inp("A")?, s.inp("HOG")?);
    let (nog, z) = (s.out("NOG")?, s.out("Z")?);
    let mut c = Canvas::new(300.0);
    column(&mut c, 235.0, 56.0, 52.0, 270.0, 0, true);
    stream(&mut c, &[(95.0, 78.0), (233.0, 78.0)], 2);
    stream(&mut c, &[(289.0, 78.0), (460.0, 78.0)], 0);
    stream(&mut c, &[(462.0, 246.0), (291.0, 246.0)], 0);
    stream(&mut c, &[(233.0, 246.0), (98.0, 246.0)], 2);
    c.name(140.0, 36.0, "middle", "liquid in");
    c.name(140.0, 52.0, "middle", &format!("x2 = {}", n4(x2)));
    c.name(415.0, 36.0, "middle", "gas out");
    c.name(415.0, 52.0, "middle", &format!("y2 = {}", n4(y2)));
    c.name(415.0, 264.0, "middle", "gas in");
    c.name(415.0, 280.0, "middle", &format!("y1 = {}", n4(y1)));
    c.name(140.0, 264.0, "middle", "liquid out");
    c.line((312.0, 94.0), (312.0, 230.0));
    c.line((306.0, 94.0), (318.0, 94.0));
    c.line((306.0, 230.0), (318.0, 230.0));
    c.value(324.0, 150.0, "start", &format!("Z = {}", q(z, "m")));
    c.value(324.0, 170.0, "start", &format!("NOG = {} × HOG {}", n4(nog), q(hog, "m")));
    c.name(260.0, 294.0, "middle", &format!("m = {}, A = {}", n4(m), n4(a)));
    Some(s.figure("process", "Packed absorber: transfer units and packed height",
                  &format!("A packed absorber: gas enters at the bottom with y1 = {} and leaves at the top with y2 = {}; \
                            solvent enters at the top with x2 = {}; equilibrium slope m = {}, absorption factor A = {}, \
                            HOG = {}. Marked: the model's number of transfer units NOG = {} and packed height Z = {}.",
                           n4(y1), n4(y2), n4(x2), n4(m), n4(a), q(hog, "m"), n4(nog), q(z, "m")), c.body(), c.h))
}

pub fn separator_column(s: &Solved) -> Option<Figure> {
    let (f, zf, xd, xb) = (s.inp("F")?, s.inp("zF")?, s.inp("xD")?, s.inp("xB")?);
    let (d, b, rd, rb) = (s.out("D")?, s.out("B")?, s.out("recovery_D")?, s.out("recovery_B")?);
    let mut c = Canvas::new(300.0);
    column(&mut c, 235.0, 56.0, 52.0, 270.0, 10, false);
    stream(&mut c, &[(55.0, 162.0), (233.0, 162.0)], 2);
    stream(&mut c, &[(261.0, 56.0), (261.0, 34.0), (475.0, 34.0)], 0);
    stream(&mut c, &[(261.0, 270.0), (261.0, 291.0), (475.0, 291.0)], 2);
    c.bold(135.0, 120.0, "middle", "feed");
    c.name(135.0, 136.0, "middle", &format!("F = {}", q(f, "mol/s")));
    c.name(135.0, 152.0, "middle", &format!("zF = {}", n4(zf)));
    c.bold(400.0, 56.0, "middle", "distillate");
    c.value(400.0, 72.0, "middle", &format!("D = {}", q(d, "mol/s")));
    c.name(400.0, 88.0, "middle", &format!("xD = {}", n4(xd)));
    c.value(400.0, 104.0, "middle", &format!("recovery {}", n4(rd)));
    c.bold(400.0, 190.0, "middle", "bottoms");
    c.value(400.0, 206.0, "middle", &format!("B = {}", q(b, "mol/s")));
    c.name(400.0, 222.0, "middle", &format!("xB = {}", n4(xb)));
    c.value(400.0, 238.0, "middle", &format!("recovery {}", n4(rb)));
    Some(s.figure("process", "Binary separation: distillate and bottoms from the balances",
                  &format!("A binary separation (a column) balanced on total flow and the light component: feed F = {} at \
                            zF = {}, distillate at xD = {}, bottoms at xB = {}. Marked, from the model: D = {}, B = {}, \
                            recovery of the light component in the distillate {}, of the heavy one in the bottoms {}.",
                           q(f, "mol/s"), n4(zf), n4(xd), n4(xb), q(d, "mol/s"), q(b, "mol/s"), n4(rd), n4(rb)),
                  c.body(), c.h))
}

/// Kremser absorber (true) or stripper (false): a tray column with its four streams.
pub fn kremser(s: &Solved, absorber: bool) -> Option<Figure> {
    let n = nint(s, "N")?;
    let m = s.inp("m")?;
    let (xin, yin) = (s.inp("x_in")?, s.inp("y_in")?);
    let rec = s.out("recovery")?;
    let factor = if absorber { s.inp("A")? } else { s.inp("S")? };
    let out = if absorber { s.out("y_out")? } else { s.out("x_out")? };
    if n < 1 {
        return None;
    }
    let mut c = Canvas::new(300.0);
    column(&mut c, 235.0, 56.0, 52.0, 270.0, (n as usize).min(10), false);
    stream(&mut c, &[(50.0, 78.0), (233.0, 78.0)], 2);
    stream(&mut c, &[(289.0, 78.0), (470.0, 78.0)], 0);
    stream(&mut c, &[(472.0, 246.0), (291.0, 246.0)], 0);
    stream(&mut c, &[(233.0, 246.0), (52.0, 246.0)], 2);
    if absorber {
        c.name(125.0, 36.0, "middle", "solvent in");
        c.name(125.0, 52.0, "middle", &format!("x = {}", n4(xin)));
        c.name(396.0, 36.0, "middle", "gas out");
        c.value(396.0, 52.0, "middle", &format!("y = {}", n4(out)));
        c.name(125.0, 200.0, "middle", "rich liquid out");
        c.value(125.0, 216.0, "middle", &format!("recovery {}", n4(rec)));
        c.name(396.0, 200.0, "middle", "gas in");
        c.name(396.0, 216.0, "middle", &format!("y = {}", n4(yin)));
    } else {
        c.name(125.0, 36.0, "middle", "liquid in");
        c.name(125.0, 52.0, "middle", &format!("x = {}", n4(xin)));
        c.name(396.0, 36.0, "middle", "gas out");
        c.value(396.0, 52.0, "middle", &format!("recovery {}", n4(rec)));
        c.name(125.0, 200.0, "middle", "liquid out");
        c.value(125.0, 216.0, "middle", &format!("x = {}", n4(out)));
        c.name(396.0, 200.0, "middle", "stripping gas in");
        c.name(396.0, 216.0, "middle", &format!("y = {}", n4(yin)));
    }
    let f = if absorber { "A" } else { "S" };
    c.name(260.0, 294.0, "middle", &format!("{n} ideal stages, {f} = {}, m = {}", n4(factor), n4(m)));
    let (what, title) = if absorber {
        ("absorber (gas up, solvent down)", "Kremser absorber: inlet and outlet streams")
    } else {
        ("stripper (liquid down, stripping gas up)", "Kremser stripper: inlet and outlet streams")
    };
    Some(s.figure("process", title,
                  &format!("A countercurrent {what} of {n} ideal stages with equilibrium y = m·x, m = {}, and {} factor \
                            {f} = {}. Marked, from the model's Kremser equation: leaving {} = {}, fraction of the solute \
                            transferred {}.", n4(m), if absorber { "absorption" } else { "stripping" }, n4(factor),
                           if absorber { "gas y" } else { "liquid x" }, n4(out), n4(rec)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ flash drums
/// A flash drum: feed in, vapour (y) out of the top, liquid (x) out of the bottom; `extra`: feed lines; `mid`: a line
/// beside the drum (returned K values).
fn flash_drum(s: &Solved, title: &str, extra: &[String], mid: Option<String>, caption: &str) -> Option<Figure> {
    let vf = s.out("VF")?;
    let (x, y) = (s.out_list("x")?, s.out_list("y")?);
    let mut c = Canvas::new(300.0);
    drum(&mut c, 207.0, 74.0, 70.0, 250.0, 1.0 - vf);
    stream(&mut c, &[(45.0, 162.0), (205.0, 162.0)], 2);
    stream(&mut c, &[(242.0, 74.0), (242.0, 38.0), (475.0, 38.0)], 0);
    stream(&mut c, &[(242.0, 250.0), (242.0, 288.0), (475.0, 288.0)], 2);
    c.bold(125.0, 88.0, "middle", "feed");
    for (i, t) in extra.iter().enumerate() {
        c.name(125.0, 104.0 + 16.0 * i as f64, "middle", t);
    }
    c.bold(390.0, 62.0, "middle", &format!("vapour V/F = {}", n4(vf)));
    c.value(390.0, 78.0, "middle", &format!("y = {}", list3(&y)));
    c.bold(390.0, 226.0, "middle", &format!("liquid L/F = {}", n4(1.0 - vf)));
    c.value(390.0, 242.0, "middle", &format!("x = {}", list3(&x)));
    if let Some(m) = mid {
        c.value(390.0, 160.0, "middle", &m);
    }
    Some(s.figure("process", title, caption, c.body(), c.h))
}

pub fn rachford_drum(s: &Solved) -> Option<Figure> {
    let (z, kk) = (s.vec("z")?, s.vec("K")?);
    let vf = s.out("VF")?;
    let (x, y) = (s.out_list("x")?, s.out_list("y")?);
    flash_drum(s, "Flash drum: vapour and liquid from Rachford-Rice",
               &[format!("z = {}", list3(&z)), format!("K = {}", list3(&kk))], None,
               &format!("Isothermal flash of a feed z = {} with equilibrium ratios K = {}. Marked: the model's vapour \
                         fraction V/F = {} from the Rachford-Rice equation, vapour y = {} and liquid x = {} \
                         (L/F = 1 - V/F).", list3(&z), list3(&kk), n4(vf), list3(&y), list3(&x)))
}

pub fn raoult_drum(s: &Solved) -> Option<Figure> {
    let z = s.vec("z")?;
    let (t, p) = (s.inp("T")?, s.inp("P")?);
    let vf = s.out("VF")?;
    let (x, y) = (s.out_list("x")?, s.out_list("y")?);
    flash_drum(s, "Flash drum at fixed temperature and pressure",
               &[format!("z = {}", list3(&z)), format!("T = {}", q(t, "K")), format!("P = {}", q(p, "Pa"))], None,
               &format!("Isothermal flash of an ideal mixture (Raoult's law) fed at z = {} into a drum at T = {} and \
                         P = {}. Marked: the model's V/F = {}, vapour y = {}, liquid x = {}.", list3(&z), q(t, "K"),
                        q(p, "Pa"), n4(vf), list3(&y), list3(&x)))
}

pub fn wilson_drum(s: &Solved) -> Option<Figure> {
    let z = s.vec("z")?;
    let (t, p) = (s.inp("T")?, s.inp("P")?);
    let vf = s.out("VF")?;
    let (x, y, kk) = (s.out_list("x")?, s.out_list("y")?, s.out_list("K")?);
    flash_drum(s, "Flash drum with Wilson K values",
               &[format!("z = {}", list3(&z)), format!("T = {}", q(t, "K")), format!("P = {}", q(p, "Pa"))],
               Some(format!("K = {}", list3(&kk))),
               &format!("Flash of a feed z = {} at T = {}, P = {}. Marked: the model's Wilson K values {}, V/F = {}, \
                         vapour y = {}, liquid x = {}.", list3(&z), q(t, "K"), q(p, "Pa"), list3(&kk), n4(vf),
                        list3(&y), list3(&x)))
}

// ------------------------------------------------------------------------------------------------ stirred tanks
/// One CSTR with feed, product and the values beneath; `second`: a second-order reaction (CA0 and CA shown);
/// `volume`: the volume and residence time are the answer.
pub fn cstr(s: &Solved, second: bool, volume: bool) -> Option<Figure> {
    let v0 = s.inp("v0")?;
    let kk = s.inp("k")?;
    let ku = if second { "m³/(mol·s)" } else { "1/s" };
    let x = if volume { s.inp("X")? } else { s.out("X")? };
    let mut c = Canvas::new(300.0);
    tank(&mut c, 260.0, 96.0, 112.0, 140.0, 0.8);
    stream(&mut c, &[(40.0, 122.0), (202.0, 122.0)], 2);
    stream(&mut c, &[(318.0, 205.0), (480.0, 205.0)], 2);
    c.bold(116.0, 58.0, "middle", "feed");
    c.name(116.0, 74.0, "middle", &format!("v0 = {}", q(v0, M3S)));
    if second {
        c.name(116.0, 90.0, "middle", &format!("CA0 = {}", q(s.inp("CA0")?, CONC)));
    }
    c.bold(419.0, 130.0, "middle", "product");
    let xs = format!("X = {}", n4(x));
    if volume { c.name(419.0, 146.0, "middle", &xs) } else { c.value(419.0, 146.0, "middle", &xs) }
    if second && !volume {
        c.value(419.0, 162.0, "middle", &format!("CA = {}", q(s.out("CA")?, CONC)));
    }
    let (vtext, extra) = if volume {
        let (v, tau) = (s.out("V")?, s.out("tau")?);
        c.value(260.0, 256.0, "middle", &format!("V = {}", q(v, "m³")));
        c.value(260.0, 273.0, "middle", &format!("τ = {}", q(tau, "s")));
        (q(v, "m³"), format!(" and τ = {}", q(tau, "s")))
    } else {
        let v = s.inp("V")?;
        c.name(260.0, 256.0, "middle", &format!("V = {}", q(v, "m³")));
        (q(v, "m³"), String::new())
    };
    c.name(260.0, 290.0, "middle", &format!("k = {} {ku}", num(kk)));
    let order = if second { "second" } else { "first" };
    let rate = if second { "-rA = k·CA²" } else { "-rA = k·CA" };
    let title = format!("CSTR: {} for a {order}-order reaction", if volume { "volume" } else { "conversion" });
    let caption = if volume {
        format!("A continuous stirred-tank reactor for a {order}-order ({rate}) reaction: feed {}, k = {} {ku}, \
                 conversion X = {} wanted. Marked: the model's V = {vtext}{extra}.", q(v0, M3S), num(kk), n4(x))
    } else {
        format!("A continuous stirred-tank reactor for a {order}-order ({rate}) reaction: feed {}, k = {} {ku}, \
                 V = {vtext}. Marked: the model's X = {}{}.", q(v0, M3S), num(kk), n4(x),
                if second { format!(", CA = {}", q(s.out("CA")?, CONC)) } else { String::new() })
    };
    Some(s.figure("process", &title, &caption, c.body(), c.h))
}

/// The stage numbers drawn for a cascade of n: all when few, else the first two and the last two.
fn shown(n: i64) -> Vec<i64> {
    if n <= 4 { (1..=n).collect() } else { vec![1, 2, n - 1, n] }
}

/// CSTRs in series: each tank labelled with the model's conversion for a train of that many tanks.
pub fn cstrs_series(s: &Solved, second: bool) -> Option<Figure> {
    let n = nint(s, "N")?;
    if n < 1 {
        return None;
    }
    let (v0, v, kk) = (s.inp("v0")?, s.inp("V")?, s.inp("k")?);
    let idx = shown(n);
    let mut res: Vec<(i64, f64, Option<f64>)> = vec![];
    for &i in &idx {
        let a = if i == n { None } else { s.with_input(&[("N", Input::Int(i))]) };
        let (x, ca) = match a {
            Some(a) => (k::num_of(&a, "X")?, k::num_of(&a, "CA")),
            None => (s.out("X")?, s.out("CA")),
        };
        res.push((i, x, ca));
    }
    let mut c = Canvas::new(270.0);
    let m = idx.len() as f64;
    let w = 76.0;
    let gap = (420.0 - m * w) / (m + if n > 4 { 1.0 } else { 0.0 });
    let mut x = 70.0 + gap / 2.0;
    let mut centres = vec![];
    for (j, &i) in idx.iter().enumerate() {
        if n > 4 && j == 2 {
            c.bold(x + gap / 2.0 - 14.0, 160.0, "middle", "...");
            x += gap;
        }
        centres.push((i, x + w / 2.0));
        x += w + gap;
    }
    stream(&mut c, &[(30.0, 120.0), (centres[0].1 - w / 2.0 - 2.0, 120.0)], 2);
    for (j, (_, cx)) in centres.iter().enumerate() {
        tank(&mut c, *cx, 106.0, w, 92.0, 0.75);
        let out = (cx + w / 2.0, 180.0);
        if j + 1 < centres.len() {
            let nx = centres[j + 1].1 - w / 2.0 - 2.0;
            let gapped = n > 4 && j == 1;
            if gapped {
                stream(&mut c, &[out, (out.0 + 10.0, 180.0)], 2);
                stream(&mut c, &[(nx - 12.0, 120.0), (nx, 120.0)], 2);
            } else {
                let mx = (out.0 + nx) / 2.0;
                stream(&mut c, &[out, (mx, 180.0), (mx, 120.0), (nx, 120.0)], 2);
            }
        } else {
            stream(&mut c, &[out, (500.0, 180.0)], 2);
        }
    }
    for (j, (i, x, ca)) in res.iter().enumerate() {
        let cx = centres[j].1;
        let y = 222.0 + if j % 2 == 1 { 16.0 } else { 0.0 };
        let t = format!("after {i}: X = {}", n4(*x));
        if *i == n { c.value(cx, y, "middle", &t) } else { c.name(cx, y, "middle", &t) }
        if second {
            if let Some(ca) = ca {
                c.name(cx, y + 16.0, "middle", &format!("CA = {}", n4(*ca)));
            }
        }
    }
    let ku = if second { "m³/(mol·s)" } else { "1/s" };
    let feed = if second { format!("feed v0 = {}, CA0 = {}", q(v0, M3S), q(s.inp("CA0")?, CONC)) } else {
        format!("feed v0 = {}", q(v0, M3S))
    };
    c.name(30.0, 30.0, "start", &feed);
    c.name(30.0, 50.0, "start", &format!("each tank V = {}, k = {} {ku}", q(v, "m³"), num(kk)));
    let list = res.iter().map(|r| n4(r.1)).collect::<Vec<_>>().join(", ");
    Some(s.figure("process", "CSTRs in series: conversion tank by tank",
                  &format!("{n} equal CSTRs in series (each V = {}, feed {}, k = {} {ku}). The conversion after each tank \
                            drawn is the model's result for a train of that many tanks: {list}; marked: X = {} leaving \
                            the train.", q(v, "m³"), q(v0, M3S), num(kk), n4(s.out("X")?)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ extraction
pub fn countercurrent(s: &Solved) -> Option<Figure> {
    let n = nint(s, "N")?;
    if n < 1 {
        return None;
    }
    let (f, sv, kk, xf, ys) = (s.inp("F")?, s.inp("S")?, s.inp("K")?, s.inp("XF")?, s.inp("YS")?);
    let (xn, y1, rec) = (s.out("XN")?, s.out("Y1")?, s.out("recovery")?);
    let mut c = Canvas::new(260.0);
    let idx = shown(n);
    let m = idx.len() as f64 + if n > 4 { 0.6 } else { 0.0 };
    let w = 52.0;
    let step = 340.0 / m;
    let mut xs = vec![];
    let mut x = 110.0;
    for (j, &i) in idx.iter().enumerate() {
        if n > 4 && j == 2 {
            c.bold(x + step * 0.3 - 26.0, 136.0, "middle", "...");
            x += step * 0.6;
        }
        xs.push((i, x));
        x += step;
    }
    for (_, x) in &xs {
        pl::unit(&mut c, *x, 110.0, w, 62.0, "");
    }
    for (i, x) in &xs {
        c.bold(x + w / 2.0, 146.0, "middle", &i.to_string());
    }
    // raffinate left to right (upper), extract right to left (lower)
    stream(&mut c, &[(30.0, 126.0), (xs[0].1 - 2.0, 126.0)], 2);
    stream(&mut c, &[(xs[0].1 - 2.0, 156.0), (30.0, 156.0)], 0);
    for j in 0..xs.len() {
        let r = xs[j].1 + w + 2.0;
        if j + 1 < xs.len() {
            let l = xs[j + 1].1 - 2.0;
            stream(&mut c, &[(r, 126.0), (l, 126.0)], 2);
            stream(&mut c, &[(l, 156.0), (r, 156.0)], 0);
        } else {
            stream(&mut c, &[(r, 126.0), (492.0, 126.0)], 2);
            stream(&mut c, &[(492.0, 156.0), (r, 156.0)], 0);
        }
    }
    c.name(58.0, 64.0, "middle", "feed");
    c.name(58.0, 80.0, "middle", &format!("F = {}", q(f, "kg/s")));
    c.name(58.0, 96.0, "middle", &format!("XF = {}", n4(xf)));
    c.name(462.0, 64.0, "middle", "raffinate");
    c.value(462.0, 80.0, "middle", &format!("XN = {}", n4(xn)));
    c.name(58.0, 204.0, "middle", "extract");
    c.value(58.0, 220.0, "middle", &format!("Y1 = {}", n4(y1)));
    c.name(462.0, 204.0, "middle", "solvent");
    c.name(462.0, 220.0, "middle", &format!("S = {}", q(sv, "kg/s")));
    c.name(462.0, 236.0, "middle", &format!("YS = {}", n4(ys)));
    c.value(260.0, 40.0, "middle", &format!("{n} stages, K = {}, recovery {}", n4(kk), n4(rec)));
    Some(s.figure("process", "Countercurrent extraction cascade",
                  &format!("Countercurrent extraction in {n} ideal stages: feed F = {} (solute-free carrier) at XF = {} \
                            enters stage 1, solvent S = {} at YS = {} enters stage {n}, distribution Y = K·X with K = {}. \
                            Marked, from the model: raffinate XN = {}, extract Y1 = {}, solute recovered {}.",
                           q(f, "kg/s"), n4(xf), q(sv, "kg/s"), n4(ys), n4(kk), n4(xn), n4(y1), n4(rec)), c.body(), c.h))
}

pub fn crosscurrent(s: &Solved) -> Option<Figure> {
    let n = nint(s, "N")?;
    if n < 1 {
        return None;
    }
    let (sv, kk, xf) = (s.inp("S")?, s.inp("K")?, s.inp("XF")?);
    let rec = s.out("recovery")?;
    let idx = shown(n);
    let mut xn = vec![];
    for &i in &idx {
        let x = if i == n { s.out("XN")? } else { k::num_of(&s.with_input(&[("N", Input::Int(i))])?, "XN")? };
        xn.push(x);
    }
    let mut c = Canvas::new(260.0);
    let m = idx.len() as f64 + if n > 4 { 0.6 } else { 0.0 };
    let w = 52.0;
    let step = 340.0 / m;
    let mut xs = vec![];
    let mut x = 120.0;
    for (j, &i) in idx.iter().enumerate() {
        if n > 4 && j == 2 {
            c.bold(x + step * 0.3 - 26.0, 130.0, "middle", "...");
            x += step * 0.6;
        }
        xs.push((i, x));
        x += step;
    }
    stream(&mut c, &[(40.0, 122.0), (xs[0].1 - 2.0, 122.0)], 2);
    for (j, (i, x)) in xs.iter().enumerate() {
        pl::unit(&mut c, *x, 96.0, w, 52.0, &i.to_string());
        stream(&mut c, &[(x + w / 2.0, 52.0), (x + w / 2.0, 94.0)], 0);
        stream(&mut c, &[(x + w / 2.0, 150.0), (x + w / 2.0, 190.0)], 0);
        let t = format!("X = {}", n4(xn[j]));
        if *i == n { c.value(x + w / 2.0, 210.0, "middle", &t) } else { c.name(x + w / 2.0, 210.0, "middle", &t) }
        let r = x + w + 2.0;
        let end = if j + 1 < xs.len() { xs[j + 1].1 - 2.0 } else { 500.0 };
        if !(n > 4 && j == 1) {
            stream(&mut c, &[(r, 122.0), (end, 122.0)], 2);
        } else {
            stream(&mut c, &[(r, 122.0), (r + 12.0, 122.0)], 2);
        }
    }
    c.name(70.0, 112.0, "middle", &format!("XF = {}", n4(xf)));
    c.name(260.0, 30.0, "middle", &format!("fresh solvent S = {} to each stage", q(sv, "kg/s")));
    c.value(260.0, 234.0, "middle", &format!("recovery {}", n4(rec)));
    c.name(260.0, 254.0, "middle", &format!("K = {}, extract leaves each stage", n4(kk)));
    let list = xn.iter().map(|x| n4(*x)).collect::<Vec<_>>().join(", ");
    Some(s.figure("process", "Crosscurrent extraction stage by stage",
                  &format!("Crosscurrent extraction (feed carrier at XF = {}, fresh solvent S = {} to each of {n} stages, \
                            K = {}). The raffinate ratio after each stage drawn is the model's result for that many \
                            stages: {list}; marked: XN = {}, recovery {}.", n4(xf), q(sv, "kg/s"), n4(kk),
                           n4(*xn.last()?), n4(rec)), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ flowsheets
pub fn blending(s: &Solved) -> Option<Figure> {
    let w = s.mat("W")?;
    let p = s.vec("p")?;
    let pp = s.inp("P")?;
    let f = s.out_list("F")?;
    let n = w.len();
    if n < 2 || n > 5 || f.len() != n {
        return None;
    }
    let mut c = Canvas::new(270.0);
    let j = (330.0, 140.0);
    let span = 180.0;
    for (i, row) in w.iter().enumerate() {
        let y = 50.0 + span * i as f64 / (n - 1) as f64;
        stream(&mut c, &[(30.0, y), (220.0, y), (j.0 - 3.0, j.1 + (y - j.1) * 0.04)], 2);
        c.name(30.0, y - 8.0, "start", &format!("feed {}: w = {}", i + 1, list3(row)));
        c.value(30.0, y + 17.0, "start", &format!("F{} = {}", i + 1, q(f[i], "kg/s")));
    }
    c.dot(j);
    stream(&mut c, &[j, (500.0, j.1)], 2);
    c.name(498.0, j.1 - 10.0, "end", &format!("product P = {}", q(pp, "kg/s")));
    c.name(498.0, j.1 + 18.0, "end", &format!("w = {}", list3(&p)));
    let fl = f.iter().enumerate().map(|(i, x)| format!("F{} = {}", i + 1, q(*x, "kg/s"))).collect::<Vec<_>>().join(", ");
    Some(s.figure("process", "Blending: the feed flows that make the product",
                  &format!("Blending {n} feeds of mass-fraction compositions {} (rows) into P = {} of product with \
                            composition {}. Marked: the model's feed flows {fl}.",
                           w.iter().map(|r| list3(r)).collect::<Vec<_>>().join(", "), q(pp, "kg/s"), list3(&p)),
                  c.body(), c.h))
}

pub fn recycle_purge(s: &Solved) -> Option<Figure> {
    let (f0, za, x, f) = (s.inp("F0")?, s.inp("zA")?, s.inp("X")?, s.inp("f")?);
    let (r, p, feed, xo, prod, yi) = (s.out("R")?, s.out("P")?, s.out("reactor_feed")?, s.out("X_overall")?,
                                      s.out("product")?, s.out("y_inert")?);
    let mut c = Canvas::new(290.0);
    let mix = (112.0, 196.0);
    let split = (328.0, 100.0);
    stream(&mut c, &[(28.0, mix.1), (mix.0 - 4.0, mix.1)], 2);
    c.dot(mix);
    stream(&mut c, &[mix, (155.0, mix.1)], 2);
    pl::unit(&mut c, 157.0, 172.0, 82.0, 48.0, "reactor");
    stream(&mut c, &[(241.0, mix.1), (285.0, mix.1)], 2);
    pl::unit(&mut c, 287.0, 172.0, 82.0, 48.0, "separator");
    stream(&mut c, &[(371.0, mix.1), (492.0, mix.1)], 2);
    stream(&mut c, &[(split.0, 170.0), (split.0, split.1 + 4.0)], 2);
    c.dot(split);
    stream(&mut c, &[split, (492.0, split.1)], 2);
    stream(&mut c, &[split, (mix.0, split.1), (mix.0, mix.1 - 4.0)], 2);
    c.name(46.0, 186.0, "middle", &format!("F0 = {}", q(f0, "mol/s")));
    c.name(46.0, 216.0, "middle", &format!("zA = {}", n4(za)));
    c.value(198.0, 236.0, "middle", &format!("X = {}", n4(x)));
    c.value(118.0, 160.0, "start", &q(feed, "mol/s"));
    c.value(220.0, 90.0, "middle", &format!("recycle R = {}", q(r, "mol/s")));
    c.value(432.0, 90.0, "middle", &format!("purge P = {}", q(p, "mol/s")));
    c.name(432.0, 120.0, "middle", &format!("fraction {}", n4(f)));
    c.value(436.0, 184.0, "middle", &format!("product {}", q(prod, "mol/s")));
    c.value(260.0, 272.0, "middle", &format!("overall conversion {}, inert in the recycle {}", n4(xo), n4(yi)));
    Some(s.figure("process", "Recycle and purge: the solved flowsheet",
                  &format!("Recycle with purge: fresh feed F0 = {} with A fraction {}, single-pass conversion {}, purge \
                            fraction {} of the separator overhead. Marked, all from the model: reactor feed {}, recycle {}, \
                            purge {}, product {}, overall conversion {}, inert fraction in the recycle {}.",
                           q(f0, "mol/s"), n4(za), n4(x), n4(f), q(feed, "mol/s"), q(r, "mol/s"), q(p, "mol/s"),
                           q(prod, "mol/s"), n4(xo), n4(yi)), c.body(), c.h))
}

pub fn recycle_reactor(s: &Solved) -> Option<Figure> {
    let (v0, v, kk, r) = (s.inp("v0")?, s.inp("V")?, s.inp("k")?, s.inp("R")?);
    let x = s.out("X")?;
    let x0 = if r == 0.0 { Some(x) } else { s.with(&[("R", 0.0)]).and_then(|a| k::num_of(&a, "X")) };
    let mut c = Canvas::new(270.0);
    let mix = (172.0, 150.0);
    let split = (446.0, 150.0);
    stream(&mut c, &[(30.0, mix.1), (mix.0 - 4.0, mix.1)], 2);
    c.dot(mix);
    stream(&mut c, &[mix, (208.0, mix.1)], 2);
    pl::unit(&mut c, 210.0, 132.0, 192.0, 36.0, &format!("V = {}, k = {} 1/s", q(v, "m³"), num(kk)));
    stream(&mut c, &[(404.0, mix.1), (split.0 - 4.0, mix.1)], 2);
    c.dot(split);
    stream(&mut c, &[split, (505.0, split.1)], 2);
    stream(&mut c, &[split, (split.0, 70.0), (mix.0, 70.0), (mix.0, mix.1 - 4.0)], 2);
    c.name(309.0, 60.0, "middle", &format!("recycle ratio R = {}", n4(r)));
    c.name(85.0, 140.0, "middle", &format!("feed v0 = {}", q(v0, M3S)));
    c.value(500.0, 174.0, "end", &format!("X = {}", n4(x)));
    if let Some(x0) = x0 {
        c.name(309.0, 220.0, "middle", &format!("without recycle (R = 0) the same tube gives X = {}", n4(x0)));
    }
    Some(s.figure("process", "Recycle reactor: conversion with and without recycle",
                  &format!("A plug-flow reactor with recycle: feed {}, V = {}, first-order k = {} 1/s, recycle ratio \
                            R = {}; part of the outlet returns to the inlet. Marked: the model's overall conversion X = {}{}.",
                           q(v0, M3S), q(v, "m³"), num(kk), n4(r), n4(x),
                           x0.map(|a| format!("; with R = 0 it returns {}", n4(a))).unwrap_or_default()), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ tanks over time
/// A time range for a tank approaching its inlet value: doubled from the given time until the model's value is within
/// 2 % of the way to the end value `end` (at most 6 doublings).
fn approach_range(s: &Solved, t: f64, out: &str, end: f64, start: f64) -> f64 {
    let mut te = t.max(1e-9);
    for _ in 0..6 {
        match s.with(&[("t", te)]).and_then(|a| k::num_of(&a, out)) {
            Some(v) if (v - end).abs() <= 0.02 * (end - start).abs() => break,
            Some(_) => te *= 2.0,
            None => break,
        }
    }
    te.max(t * 1.25)
}

pub fn mixing_tank(s: &Solved) -> Option<Figure> {
    let (v, qf, cin, c0, t) = (s.inp("V")?, s.inp("q")?, s.inp("C_in")?, s.inp("C0")?, s.inp("t")?);
    let cc = s.out("C")?;
    if !(t > 0.0) {
        return None;
    }
    let te = approach_range(s, t, "C", cin, c0);
    let ts = k::span(0.0, te, 50, false);
    let pts = s.curve("t", &ts, "C", &[]);
    if pts.len() < 10 {
        return None;
    }
    let mut c = Canvas::new(414.0);
    tank(&mut c, 290.0, 60.0, 70.0, 66.0, 0.7);
    stream(&mut c, &[(110.0, 72.0), (253.0, 72.0)], 2);
    stream(&mut c, &[(327.0, 110.0), (440.0, 110.0)], 2);
    c.name(110.0, 62.0, "start", &format!("q = {}, C_in = {}", q(qf, M3S), num(cin)));
    c.value(400.0, 100.0, "middle", "C(t)");
    c.name(290.0, 148.0, "middle", &format!("V = {}, C(0) = {}", q(v, "m³"), num(c0)));
    let (lo, hi) = pl::range(pts.iter().map(|p| p.1).chain([cin, c0, cc]))?;
    let mut pt = Plot::new(414.0, (86.0, 214.0, 500.0, 360.0), (0.0, te), (lo, hi), (false, false), (false, true), "");
    pt.frame("time t [s]", "concentration C");
    pt.line(&pts, 0);
    pt.hguide(cin, &format!("inlet {}", num(cin)));
    pt.mark(t, cc, &format!("{} at {}", num(cc), q(t, "s")));
    legend(&mut pt.c, 100.0, 196.0, &[li("tank concentration (model)", "line:0")]);
    let body = format!("{}{}", c.body(), pt.body());
    Some(s.figure("process", "Mixing tank: concentration moving toward the inlet value",
                  &format!("A well-mixed tank (V = {}) fed at q = {} with concentration {}, starting at {}, over the \
                            concentration the model computes at 50 times up to {}. Marked: C = {} at t = {}.",
                           q(v, "m³"), q(qf, M3S), num(cin), num(c0), q(te, "s"), num(cc), q(t, "s")), &body, 414.0))
}

pub fn variable_tank(s: &Solved) -> Option<Figure> {
    let (v0, qi, qo, cin, c0, t) = (s.inp("V0")?, s.inp("q_in")?, s.inp("q_out")?, s.inp("C_in")?, s.inp("C0")?,
                                    s.inp("t")?);
    let (vv, cc) = (s.out("V")?, s.out("C")?);
    if !(t > 0.0) {
        return None;
    }
    let te = 2.0 * t;
    let mut vp = vec![];
    let mut cp = vec![];
    for x in k::span(0.0, te, 40, false) {
        if let Some(a) = s.with(&[("t", x)]) {
            if let (Some(a1), Some(a2)) = (k::num_of(&a, "V"), k::num_of(&a, "C")) {
                vp.push((x, a1));
                cp.push((x, a2));
            }
        }
    }
    if vp.len() < 10 {
        return None;
    }
    let mut c = Canvas::new(568.0);
    tank(&mut c, 330.0, 44.0, 64.0, 66.0, 0.6);
    stream(&mut c, &[(70.0, 60.0), (296.0, 60.0)], 2);
    stream(&mut c, &[(364.0, 100.0), (485.0, 100.0)], 2);
    c.name(70.0, 50.0, "start", &format!("q_in = {}, C_in = {}", q(qi, M3S), num(cin)));
    c.name(370.0, 90.0, "start", &format!("q_out = {}", q(qo, M3S)));
    c.name(330.0, 134.0, "middle", &format!("V(0) = {}, C(0) = {}", q(v0, "m³"), num(c0)));
    let (vlo, vhi) = pl::range(vp.iter().map(|p| p.1))?;
    let mut p1 = Plot::new(568.0, (86.0, 190.0, 500.0, 290.0), (0.0, te), (vlo, vhi), (false, false), (false, true), "");
    p1.frame("", "volume V [m³]");
    p1.line(&vp, 2);
    p1.mark(t, vv, &format!("V = {}", q(vv, "m³")));
    legend(&mut p1.c, 100.0, 176.0, &[li("volume in the tank", "line:2")]);
    let (clo, chi) = pl::range(cp.iter().map(|p| p.1))?;
    let mut p2 = Plot::new(568.0, (86.0, 384.0, 500.0, 510.0), (0.0, te), (clo, chi), (false, false), (false, true), "2");
    p2.frame("time t [s]", "concentration C");
    p2.line(&cp, 0);
    p2.mark(t, cc, &format!("C = {}", num(cc)));
    legend(&mut p2.c, 100.0, 370.0, &[li("tank concentration", "line:0")]);
    let body = format!("{}{}{}", c.body(), p1.body(), p2.body());
    Some(s.figure("process", "Variable-volume tank: volume and concentration",
                  &format!("A tank filling while it drains (inflow {} at concentration {}, outflow {}, starting with {} at \
                            {}): volume and concentration from the model at 40 times to {}. Marked: at t = {}, V = {} and \
                            C = {}.", q(qi, M3S), num(cin), q(qo, M3S), q(v0, "m³"), num(c0), q(te, "s"), q(t, "s"),
                           q(vv, "m³"), num(cc)), &body, 568.0))
}
