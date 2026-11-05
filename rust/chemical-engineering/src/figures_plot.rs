//! Drawing helpers of Chemical Engineering's figures: a chart panel placed anywhere on a canvas (axes, grid, curves,
//! a shaded region between two curves, data dots, stems, guides, a marked point, a legend) and process-equipment
//! symbols (columns, stirred tanks, flash drums, coloured streams). Only the site's figure classes (axp-*) are used, so
//! the drawings follow the page's light and dark theme.

use ax_engine::figkit::{self as k, Canvas, P};

const LINE: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
const DOT: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];
const SHADE: [&str; 4] = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];
const BAR: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];

/// One axis: values lo..hi mapped to pixels a..b, linear or logarithmic.
#[derive(Clone, Copy)]
pub struct Ax {
    pub lo: f64,
    pub hi: f64,
    pub log: bool,
    a: f64,
    b: f64,
}

impl Ax {
    /// `pad`: a little room beyond the values.
    pub fn new(lo: f64, hi: f64, log: bool, a: f64, b: f64, pad: bool) -> Ax {
        let (mut lo, mut hi) = (lo, hi);
        let log = log && lo > 0.0 && hi > 0.0;
        if log {
            if hi <= lo * 1.000001 {
                lo /= 2.0;
                hi *= 2.0;
            }
            if pad {
                let r = (hi / lo).powf(0.04);
                lo /= r;
                hi *= r;
            }
        } else {
            if !(hi > lo) {
                let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
                lo -= d;
                hi += d;
            }
            if pad {
                let d = (hi - lo) * 0.05;
                lo -= d;
                hi += d;
            }
        }
        Ax { lo, hi, log, a, b }
    }
    pub fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    fn ok(&self, v: f64) -> bool {
        v.is_finite() && (!self.log || v > 0.0)
    }
    pub fn ticks(&self) -> Vec<f64> {
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            let mut v: Vec<f64> = vec![];
            if e1 - e0 >= 1 {
                let step = (((e1 - e0 + 1) as f64) / 6.0).ceil().max(1.0) as usize;
                v = (e0..=e1).step_by(step).map(|e| 10f64.powi(e)).collect();
            }
            if v.len() < 3 {
                // less than two decades: 1, 2, 5 steps
                let mut w = vec![];
                for e in (e0 - 1)..=(e1 + 1) {
                    for m in [1.0, 2.0, 5.0] {
                        let t = m * 10f64.powi(e);
                        if t >= self.lo * 0.999 && t <= self.hi * 1.001 {
                            w.push(t);
                        }
                    }
                }
                if w.len() >= 2 {
                    v = w;
                }
            }
            return v;
        }
        let span = self.hi - self.lo;
        let raw = span / 5.0;
        let mag = 10f64.powf(raw.log10().floor());
        let norm = raw / mag;
        let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
        let mut v = vec![];
        let mut t = (self.lo / step).ceil() * step;
        while t <= self.hi + step * 1e-9 && v.len() < 12 {
            v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
            t += step;
        }
        v
    }
}

/// The smallest and largest finite value.
pub fn range(v: impl IntoIterator<Item = f64>) -> Option<(f64, f64)> {
    let mut r: Option<(f64, f64)> = None;
    for x in v.into_iter().filter(|x| x.is_finite()) {
        r = Some(match r {
            None => (x, x),
            Some((a, b)) => (a.min(x), b.max(x)),
        });
    }
    r
}

/// A chart panel in the rectangle (x0, y0)-(x1, y1) of a canvas `h` tall: clipped drawing under, labels over.
pub struct Plot {
    pub xa: Ax,
    pub ya: Ax,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    under: String,
    pub c: Canvas,
    clip: String,
    /// grid lines and numbers along x (off for a chart of named categories)
    pub xticks: bool,
}

impl Plot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(h: f64, rect: (f64, f64, f64, f64), xr: (f64, f64), yr: (f64, f64), log: (bool, bool), pad: (bool, bool),
               clip: &str) -> Plot {
        let (x0, y0, x1, y1) = rect;
        Plot { xa: Ax::new(xr.0, xr.1, log.0, x0, x1, pad.0), ya: Ax::new(yr.0, yr.1, log.1, y1, y0, pad.1), x0, x1, y0,
               y1, under: String::new(), c: Canvas::new(h), clip: format!("CLIPID{clip}"), xticks: true }
    }
    /// The standard chart of a 340-tall figure (room for a two-row legend above).
    pub fn chart(xr: (f64, f64), yr: (f64, f64), log: (bool, bool)) -> Plot {
        Plot::new(340.0, (86.0, 56.0, 500.0, 274.0), xr, yr, log, (false, true), "")
    }
    pub fn p(&self, x: f64, y: f64) -> P {
        (self.xa.px(x), self.ya.px(y))
    }
    fn ok(&self, x: f64, y: f64) -> bool {
        self.xa.ok(x) && self.ya.ok(y)
    }
    fn path(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for &(x, y) in pts {
            if self.ok(x, y) {
                let (px, py) = self.p(x, y);
                d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
                pen = true;
            } else {
                pen = false;
            }
        }
        d.trim_end().to_string()
    }
    /// A curve in colour `k` (0 accent, 1 green, 2 violet, 3 grey); a non-finite point breaks it.
    pub fn line(&mut self, pts: &[(f64, f64)], k: usize) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.under.push_str(&format!("<path class=\"{}\" d=\"{d}\"/>", LINE[k % 4]));
        }
    }
    /// A dashed curve (a guide: y = x, a final value).
    pub fn dashed(&mut self, pts: &[(f64, f64)]) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.under.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"6 4\" d=\"{d}\"/>"));
        }
    }
    /// A step line (a value held until the next sample).
    pub fn steps(&mut self, pts: &[(f64, f64)], x_end: f64, k: usize) {
        let mut v = vec![];
        for (i, &(x, y)) in pts.iter().enumerate() {
            v.push((x, y));
            let nx = pts.get(i + 1).map(|p| p.0).unwrap_or(x_end);
            v.push((nx, y));
        }
        self.line(&v, k);
    }
    /// The region between two curves (each a list of points; the second is walked back), shaded in colour `k`.
    pub fn area(&mut self, a: &[(f64, f64)], b: &[(f64, f64)], k: usize) {
        let pts: Vec<(f64, f64)> = a.iter().chain(b.iter().rev()).copied().filter(|p| self.ok(p.0, p.1)).collect();
        if pts.len() < 3 {
            return;
        }
        let d = self.path(&pts);
        self.under.push_str(&format!("<path class=\"{}\" d=\"{d} Z\"/>", SHADE[k % 4]));
    }
    /// The region under a curve down to the axis.
    pub fn fill_under(&mut self, pts: &[(f64, f64)], k: usize) {
        let base = if self.ya.log { self.ya.lo } else { self.ya.lo.max(0.0).min(self.ya.hi) };
        let b: Vec<(f64, f64)> = pts.iter().map(|p| (p.0, base)).collect();
        self.area(pts, &b, k);
    }
    /// A vertical band between two x values in colour `k`.
    pub fn vband(&mut self, x0: f64, x1: f64, k: usize) {
        let (a, b) = (self.xa.px(x0).max(self.x0), self.xa.px(x1).min(self.x1));
        if b > a {
            self.under.push_str(&format!("<rect class=\"{}\" x=\"{a:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                         SHADE[k % 4], self.y0, b - a, self.y1 - self.y0));
        }
    }
    /// Dots in colour `k`.
    pub fn dots(&mut self, pts: &[(f64, f64)], k: usize, r: f64) {
        for &(x, y) in pts {
            if self.ok(x, y) {
                let (px, py) = self.p(x, y);
                self.under.push_str(&format!("<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"{r}\"/>", DOT[k % 4]));
            }
        }
    }
    /// Stems from the axis (or from y = 0 when it is in range) to each point, with a dot.
    pub fn stems(&mut self, pts: &[(f64, f64)], k: usize) {
        let base = if !self.ya.log && self.ya.lo <= 0.0 && self.ya.hi >= 0.0 { self.ya.px(0.0) } else { self.y1 };
        for &(x, y) in pts {
            if self.ok(x, y) {
                let (px, py) = self.p(x, y);
                self.under.push_str(&format!("<path class=\"{}\" d=\"M{px:.1},{base:.1} L{px:.1},{py:.1}\"/>\
                                              <circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.5\"/>",
                                             LINE[k % 4], DOT[k % 4]));
            }
        }
    }
    /// A horizontal dashed guide at y with a label at its right end (inside the plot).
    pub fn hguide(&mut self, y: f64, label: &str) {
        if !self.ya.ok(y) {
            return;
        }
        let py = self.ya.px(y);
        if py < self.y0 - 0.5 || py > self.y1 + 0.5 {
            return;
        }
        self.under.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"6 4\" d=\"M{:.1},{py:.1} L{:.1},{py:.1}\"/>",
                                     self.x0, self.x1));
        let ly = if py - 5.0 < self.y0 + 12.0 { py + 15.0 } else { py - 5.0 };
        self.c.name(self.x1 - 4.0, ly, "end", label);
    }
    /// A vertical dashed guide at x with a label beside its foot.
    pub fn vguide(&mut self, x: f64, label: &str) {
        self.vguide_at(x, label, false);
    }
    /// A vertical dashed guide with its label at the top of the plot.
    pub fn vguide_top(&mut self, x: f64, label: &str) {
        self.vguide_at(x, label, true);
    }
    fn vguide_at(&mut self, x: f64, label: &str, top: bool) {
        if !self.xa.ok(x) {
            return;
        }
        let px = self.xa.px(x);
        if px < self.x0 - 0.5 || px > self.x1 + 0.5 {
            return;
        }
        self.under.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"6 4\" d=\"M{px:.1},{:.1} L{px:.1},{:.1}\"/>",
                                     self.y0, self.y1));
        let right = px + 6.0 + k::text_w(label, false) <= self.x1;
        let y = if top { self.y0 + 15.0 } else { self.y1 - 8.0 };
        self.c.name(if right { px + 5.0 } else { px - 5.0 }, y, if right { "start" } else { "end" }, label);
    }
    /// A grey segment between two points (a tie line, a load line).
    pub fn seg(&mut self, a: (f64, f64), b: (f64, f64), dashed: bool) {
        if !self.ok(a.0, a.1) || !self.ok(b.0, b.1) {
            return;
        }
        let (p, q) = (self.p(a.0, a.1), self.p(b.0, b.1));
        self.under.push_str(&format!("<path class=\"axp-k\"{} d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>",
                                     if dashed { " stroke-dasharray=\"5 4\"" } else { "" }, p.0, p.1, q.0, q.1));
    }
    /// The marked point with its label (kept inside the plot).
    pub fn mark(&mut self, x: f64, y: f64, label: &str) {
        if !self.ok(x, y) {
            return;
        }
        let (px, py) = self.p(x, y);
        if px < self.x0 - 1.0 || px > self.x1 + 1.0 || py < self.y0 - 1.0 || py > self.y1 + 1.0 {
            return;
        }
        let w = k::text_w(label, true) * 1.15;
        let (tx, anchor) = if px + 10.0 + w <= self.x1 { (px + 10.0, "start") } else if px - 10.0 - w >= self.x0 {
            (px - 10.0, "end")
        } else {
            ((self.x0 + self.x1) / 2.0, "middle")
        };
        let ty = if py < self.y0 + 26.0 { py + 22.0 } else { py - 10.0 };
        self.c.raw(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>"));
        self.c.text(tx, ty, anchor, "axp-mt", label);
    }
    /// A small grey dot at a point (an end of a tie line).
    pub fn pin(&mut self, x: f64, y: f64) {
        if self.ok(x, y) {
            let (px, py) = self.p(x, y);
            self.c.raw(&format!("<circle class=\"axp-kd\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.5\"/>"));
        }
    }
    /// A grey note beside a point of the plot.
    pub fn note(&mut self, x: f64, y: f64, dx: f64, dy: f64, anchor: &str, t: &str) {
        if self.ok(x, y) {
            let (px, py) = self.p(x, y);
            self.c.name(px + dx, py + dy, anchor, t);
        }
    }
    /// Grid, ticks, frame and axis titles (`xlab` under the ticks, `ylab` turned along the left edge).
    pub fn frame(&mut self, xlab: &str, ylab: &str) {
        let mut g = String::new();
        let xt = if self.xticks { self.xa.ticks() } else { vec![] };
        let every = if xt.len() > 7 { 2 } else { 1 };
        for (i, t) in xt.iter().enumerate() {
            let x = self.xa.px(*t);
            if x < self.x0 - 0.5 || x > self.x1 + 0.5 {
                continue;
            }
            g.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>", self.y0, self.y1));
            if i % every == 0 {
                g.push_str(&format!("<text class=\"axp-t\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                    self.y1 + 18.0, k::num(*t)));
            }
        }
        for t in self.ya.ticks() {
            let y = self.ya.px(t);
            if y < self.y0 - 0.5 || y > self.y1 + 0.5 {
                continue;
            }
            g.push_str(&format!("<line class=\"axp-g\" x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\"/>\
                                 <text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                self.x0, self.x1, self.x0 - 6.0, y + 5.0, k::num(t)));
        }
        g.push_str(&format!("<rect class=\"axp-f\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", self.x0, self.y0,
                            self.x1 - self.x0, self.y1 - self.y0));
        if !xlab.is_empty() {
            g.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                (self.x0 + self.x1) / 2.0, self.y1 + 42.0, esc(xlab)));
        }
        if !ylab.is_empty() {
            let n = ((self.y1 - self.y0 + 40.0) / 8.5) as usize;
            g.push_str(&format!("<text class=\"axp-l\" transform=\"translate(18,{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                                (self.y0 + self.y1) / 2.0, esc(&clipped(ylab, n.max(8)))));
        }
        self.c.s.insert_str(0, &g);
    }
    /// The panel's drawing: frame first, then the clipped curves, then marks and labels.
    pub fn body(&self) -> String {
        let clip = format!("<clipPath id=\"{}\"><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/></clipPath>\
                            <g clip-path=\"url(#{})\">{}</g>", self.clip, self.x0, self.y0 - 1.0, self.x1 - self.x0,
                           self.y1 - self.y0 + 2.0, self.clip, self.under);
        // the frame (start of c.s) under the clipped curves, the labels over them
        let s = &self.c.s;
        let cut = s.find("<rect class=\"axp-f\"").and_then(|i| s[i..].find("/>").map(|j| i + j + 2)).unwrap_or(0);
        format!("{}{}{}", &s[..cut], clip, &s[cut..])
    }
}

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub fn clipped(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("{}...", c[..n.saturating_sub(3)].iter().collect::<String>().trim_end()) }
}

/// A legend in rows from (x, y): (name, style) with style "line:k", "dash", "dot:k", "area:k", "bar:k".
pub fn legend(c: &mut Canvas, x: f64, y: f64, items: &[(String, String)]) -> f64 {
    let mut cx = x;
    let mut cy = y;
    for (name, style) in items {
        let w = 26.0 + k::text_w(name, false) + 18.0;
        if cx + w > 512.0 && cx > x {
            cx = x;
            cy += 19.0;
        }
        let (kind, n) = style.split_once(':').unwrap_or((style.as_str(), "0"));
        let n: usize = n.parse().unwrap_or(0);
        match kind {
            "line" => c.raw(&format!("<path class=\"{}\" d=\"M{cx:.1},{:.1} L{:.1},{:.1}\"/>", LINE[n % 4], cy - 4.0, cx + 18.0, cy - 4.0)),
            "dash" => c.raw(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 3\" d=\"M{cx:.1},{:.1} L{:.1},{:.1}\"/>", cy - 4.0,
                                     cx + 18.0, cy - 4.0)),
            "dot" => c.raw(&format!("<circle class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\"/>", DOT[n % 4], cx + 9.0, cy - 4.0)),
            "area" => c.raw(&format!("<rect class=\"{}\" x=\"{cx:.1}\" y=\"{:.1}\" width=\"18\" height=\"11\"/>", SHADE[n % 4],
                                     cy - 10.0)),
            _ => c.raw(&format!("<rect class=\"{}\" x=\"{cx:.1}\" y=\"{:.1}\" width=\"18\" height=\"11\"/>", BAR[n % 4], cy - 10.0)),
        }
        c.name(cx + 25.0, cy, "start", name);
        cx += w;
    }
    cy
}

/// Legend item helper.
pub fn li(name: &str, style: &str) -> (String, String) {
    (name.to_string(), style.to_string())
}

// ------------------------------------------------------------------------------------------------ process symbols
/// A stream: an arrow in colour `k` (0 accent: gas or vapour, 2 violet: liquid) through the points.
pub fn stream(c: &mut Canvas, pts: &[P], k: usize) {
    if pts.len() < 2 {
        return;
    }
    let n = pts.len();
    let (a, b) = (pts[n - 2], pts[n - 1]);
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt().max(1e-9);
    let u = (dx / d, dy / d);
    let nn = (-u.1, u.0);
    let mut d = String::new();
    for (i, p) in pts.iter().enumerate() {
        let p = if i == n - 1 { (p.0 - u.0 * 9.0, p.1 - u.1 * 9.0) } else { *p };
        d.push_str(&format!("{}{:.1},{:.1} ", if i == 0 { "M" } else { "L" }, p.0, p.1));
    }
    c.raw(&format!("<path class=\"{}\" d=\"{}\"/>", LINE[k % 4], d.trim_end()));
    let base = (b.0 - u.0 * 11.0, b.1 - u.1 * 11.0);
    c.raw(&format!("<path class=\"{}\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", DOT[k % 4], b.0, b.1,
                   base.0 + nn.0 * 5.0, base.1 + nn.1 * 5.0, base.0 - nn.0 * 5.0, base.1 - nn.1 * 5.0));
}

/// A vertical column (rounded ends) from (x, top) to (x + w, bottom); `trays`: horizontal trays, else packing hatched.
pub fn column(c: &mut Canvas, x: f64, top: f64, w: f64, bottom: f64, trays: usize, packed: bool) {
    let r = w / 2.0;
    c.raw(&format!("<path class=\"axp-ka\" d=\"M{x:.1},{:.1} A{r:.1},{:.1} 0 0 1 {:.1},{:.1} L{:.1},{:.1} A{r:.1},{:.1} 0 0 1 {x:.1},{:.1} Z\"/>\
                    <path class=\"axp-k\" d=\"M{x:.1},{:.1} A{r:.1},{:.1} 0 0 1 {:.1},{:.1} L{:.1},{:.1} A{r:.1},{:.1} 0 0 1 {x:.1},{:.1} Z\"/>",
                   top + 12.0, 12.0, x + w, top + 12.0, x + w, bottom - 12.0, 12.0, bottom - 12.0,
                   top + 12.0, 12.0, x + w, top + 12.0, x + w, bottom - 12.0, 12.0, bottom - 12.0));
    if packed {
        let (a, b) = (top + 34.0, bottom - 34.0);
        c.raw(&format!("<rect class=\"axp-k\" x=\"{x:.1}\" y=\"{a:.1}\" width=\"{w:.1}\" height=\"{:.1}\"/>", b - a));
        let mut t = a - w;
        while t < b {
            let (y0, y1) = (t.max(a), (t + w).min(b));
            let (xa, xb) = (x + (y0 - t), x + (y1 - t));
            if y1 > y0 {
                c.raw(&format!("<path class=\"axp-c4\" d=\"M{:.1},{y0:.1} L{:.1},{y1:.1}\"/>", xa.min(x + w), xb.min(x + w)));
            }
            t += 12.0;
        }
    } else if trays > 0 {
        let n = trays.min(12);
        for i in 0..n {
            let y = top + 22.0 + (bottom - top - 44.0) * (i as f64 + 0.5) / n as f64;
            let (l, r2) = if i % 2 == 0 { (x, x + w * 0.75) } else { (x + w * 0.25, x + w) };
            c.raw(&format!("<path class=\"axp-c4\" d=\"M{l:.1},{y:.1} L{r2:.1},{y:.1}\"/>"));
        }
    }
}

/// A stirred tank: open top at y = top, liquid filled to `fill` (0..1), bottom rounded; width w, centre x.
pub fn tank(c: &mut Canvas, cx: f64, top: f64, w: f64, h: f64, fill: f64) {
    let (l, r) = (cx - w / 2.0, cx + w / 2.0);
    let b = top + h;
    let rr = w / 2.0;
    let ry = (h * 0.22).min(26.0);
    let ly = top + h * (1.0 - fill.clamp(0.05, 0.95)) * 0.85 + 12.0;
    c.raw(&format!("<path class=\"axp-s3\" d=\"M{l:.1},{ly:.1} L{l:.1},{:.1} A{rr:.1},{ry:.1} 0 0 0 {r:.1},{:.1} L{r:.1},{ly:.1} Z\"/>",
                   b - ry, b - ry));
    c.raw(&format!("<path class=\"axp-k\" d=\"M{l:.1},{top:.1} L{l:.1},{:.1} A{rr:.1},{ry:.1} 0 0 0 {r:.1},{:.1} L{r:.1},{top:.1}\"/>",
                   b - ry, b - ry));
    // the stirrer: motor box, shaft and blade
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"12\"/>\
                    <rect class=\"axp-k\" x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"12\"/>\
                    <path class=\"axp-k\" d=\"M{cx:.1},{:.1} L{cx:.1},{:.1}\"/>\
                    <path class=\"axp-k\" style=\"stroke-width:4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>",
                   cx - 7.0, top - 22.0, cx - 7.0, top - 22.0, top - 10.0, b - ry - 6.0, cx - w * 0.22, b - ry - 6.0,
                   cx + w * 0.22, b - ry - 6.0));
}

/// A closed vertical drum (a flash vessel) with liquid in its lower part.
pub fn drum(c: &mut Canvas, x: f64, top: f64, w: f64, bottom: f64, fill: f64) {
    let r = w / 2.0;
    let ly = bottom - 14.0 - (bottom - top - 28.0) * fill.clamp(0.05, 0.9);
    c.raw(&format!("<path class=\"axp-s3\" d=\"M{x:.1},{ly:.1} L{x:.1},{:.1} A{r:.1},14 0 0 0 {:.1},{:.1} L{:.1},{ly:.1} Z\"/>",
                   bottom - 14.0, x + w, bottom - 14.0, x + w));
    c.raw(&format!("<path class=\"axp-k\" d=\"M{x:.1},{:.1} A{r:.1},14 0 0 1 {:.1},{:.1} L{:.1},{:.1} A{r:.1},14 0 0 1 {x:.1},{:.1} Z\"/>",
                   top + 14.0, x + w, top + 14.0, x + w, bottom - 14.0, bottom - 14.0));
}

/// A grey box with a bold label (a stage, a unit).
pub fn unit(c: &mut Canvas, x: f64, y: f64, w: f64, h: f64, label: &str) {
    c.block(x, y, w, h, label);
}

/// "[a, b, c]" of numbers at 3 significant digits.
pub fn list3(v: &[f64]) -> String {
    format!("[{}]", v.iter().map(|x| ax_engine::num::py_g(*x, 3)).collect::<Vec<_>>().join(", "))
}
