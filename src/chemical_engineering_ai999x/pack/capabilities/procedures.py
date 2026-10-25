"""Chemical engineering procedures: deterministic capabilities with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics (the process-engineering view): material balances (blending, separators, recycle with purge, unsteady tanks,
combustion flue gas), energy balances (sensible heat and heats of reaction with polynomial heat capacities, adiabatic
reactor and flame temperatures, batch heating and cooling), reactor design (batch, CSTR, PFR, gas-phase PFR with
volume change, CSTRs in series, recycle reactor, reversible and series reactions, residence-time distribution),
separations (Antoine vapour pressure, Raoult bubble and dew points, flash calculations, minimum reflux, McCabe-Thiele,
Kremser absorbers and strippers, transfer units, liquid extraction, activity-coefficient models), property estimation
(cubic equations of state, corresponding-states correlations), transport and particle beds (diffusion through a
stagnant film, packed-bed pressure drop, cake filtration), process control (first-order-plus-dead-time response,
P-only offset, frequency response, discrete PID, Ziegler-Nichols tuning, closed-loop stability, margins, step-test
fits) and reservoir and petroleum engineering (radial Darcy flow, Arps decline curves, material balances, Horner
build-up analysis, the line-source solution, gas-well back-pressure tests).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re, fractions) plus mpmath where stated.

How the references differ from the shipped code:
- balances: numpy linear solves of the balance equations, sequential-modular (tear-stream) iteration of a recycle
  loop, element balances solved with numpy (atom counts from chemicals' formula parser), ODE integration (scipy DOP853)
  of unsteady tanks;
- energy: scipy quadrature of the heat-capacity polynomials, a different Hess path for the adiabatic reactor (reaction
  at the inlet temperature, then heating), cantera (GRI-Mech 3.0 NASA polynomials) for the adiabatic flame temperature;
- reactors: quadrature of the design-equation integrals, ODE integration of batch reactors, plug-flow reactors, the
  start-up of CSTRs to steady state, the segregation (RTD) integral for a first-order CSTR, ODE integration plus root
  finding for maximum yields and recycle reactors;
- separations: chemicals (Antoine, ideal flash, Rachford-Rice, Wilson K-value flash), thermo (Wilson, NRTL and UNIQUAC
  activity coefficients; Peng-Robinson and SRK equations of state), numpy linear solves of stage-by-stage balances
  (Kremser, extraction), quadrature of the transfer-unit integral, the McCabe-Thiele pinch geometry (minimum reflux)
  and stage-to-stage calculation from the reboiler upward (stage count);
- properties: chemicals' own correlations (Lee-Kesler, Watson, Riedel, Pitzer, Rackett, Abbott virial, mass to mole
  fractions);
- transport: ODE shooting for the stagnant-film flux, fluids (Ergun, Carman), ODE integration of the filtration rate,
  scipy linear regression for the filtration constants;
- control: python-control (step and frequency responses, closed-loop DC gain and poles, stability margins, discrete
  PID as a transfer function in z), scipy curve_fit for step-test fits;
- petroleum: quadrature of the radial Darcy resistance, ODE integration of the Arps loss-ratio equation and quadrature
  of the rate, numpy/scipy regressions (Havlena-Odeh, p/z, Horner, back-pressure), the line-source solution as the
  time integral of the instantaneous line source (scipy quadrature).

Conventions (fixed, so run and reference agree exactly):
- SI internally; temperatures are read in K or degC (degF is converted here) and reported in K. Pressures are absolute;
  a gauge pressure written "bar g", "barg", "psig" or "kPa g" is converted here by adding 101325 Pa.
- Gas constant R = 8.31446261815324 J/(mol K); standard gravity 9.80665 m/s^2.
- Dry air is 21 mol% O2 and 79 mol% N2 (3.7619 mol N2 per mol O2). Combustion is complete (CO2, H2O, SO2), fuel
  nitrogen leaves as N2, water in the flue gas is vapour. Atomic weights are those of the chemicals library (C 12.0107,
  H 1.00794, O 15.9994, N 14.0067, S 32.065).
- Heat-capacity polynomials are cp = a + b T + c T^2 + d T^3 in J/(mol K) with T in K; heats of reaction are per mole
  of extent, negative when exothermic; stoichiometric coefficients are positive for products.
- Reactor kinetics: -r_A = k C_A (first order) or k C_A^2 (second order), isothermal; liquid-phase reactors have
  constant density; conversions are fractions of the key reactant A.
- Antoine coefficients are read with a REQUIRED stated convention: the logarithm (log10 or ln) and the units of P and T
  (mmHg_degC, kPa_degC, bar_degC, Pa_K, kPa_K, bar_K, mmHg_K). Nothing is assumed.
- Lists carry no unit: a list of times or pressures has a required unit input (t_unit, p_unit); critical
  temperatures in lists are in K; other lists are dimensionless or in the unit stated in the input's meaning.
- Distillation: binary, constant relative volatility, constant molar overflow, total condenser, partial reboiler
  counted as a stage. Absorption and stripping: dilute, linear equilibrium y = m x, constant L and V.
- Control: deviation variables; a P or PI controller acts on the error e = setpoint - y; discrete PID is the position
  form with rectangular integration, e_k included in the sum, and e_0 = 0 before the first sample; Ziegler-Nichols
  closed-loop settings P: 0.5 Ku; PI: 0.45 Ku, Pu/1.2; PID: 0.6 Ku, Pu/2, Pu/8. Frequencies in rad/s, phases in degrees.
- Petroleum: SI throughout (field units are converted here: psi, ft, md, cP, bbl, STB, acre); rates in the radial
  and line-source formulas are reservoir rates; the Arps decline exponent b lies between 0 and 1.
- Results built from exp, log or roots are rounded to 12 significant digits (identical on every platform) and values
  that are zero by construction are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`First-order CSTR conversion: v0 = 0.01 m^3/s, V = 2 m^3, k = 0.005 1/s`). Physical values carry a unit; fractions
may be written with %; lists are written in brackets; choices are words. Units the shared unit reader lacks are
converted here and the conversion is stated: psi, psia, psig, bar g, kPa g, degF, BTU, lb, lbmol, bbl, STB, md, cP,
acre, inch, gal. Any other number, an unknown name or a name given twice is refused, never guessed.
"""
from __future__ import annotations

import math
import re

# ================================================================================================ constants
R_GAS = 8.31446261815324          # J/(mol K)
G0 = 9.80665                      # m/s^2
P_ATM = 101325.0                  # Pa
N2_PER_O2 = 79.0 / 21.0           # dry air
AW = {"C": 12.0107, "H": 1.00794, "O": 15.9994, "N": 14.0067, "S": 32.065}   # chemicals' atomic weights, g/mol
MW_AIR = (0.21 * 2 * AW["O"] + 0.79 * 2 * AW["N"])                           # g/mol


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, complex):
        return complex(_r(x.real), _r(x.imag))
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(*vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), "every input must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v > 0,
              f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v >= 0,
              f"{name} must be zero or positive")


def _frac(name, v, lo_open=False, hi_open=False):
    _fin(v)
    _need((v > 0 if lo_open else v >= 0) and (v < 1 if hi_open else v <= 1),
          f"{name} must be a fraction between 0 and 1")


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


def _bisect(f, a, b, n=500):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def _gauss(A, b):
    """Solve A x = b by Gaussian elimination with partial pivoting; singular -> ValueError."""
    n = len(A)
    M = [list(row) + [b[i]] for i, row in enumerate(A)]
    scale = max((abs(x) for row in A for x in row), default=1.0) or 1.0
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        _need(abs(M[p][c]) > 1e-12 * scale, "the balance equations are singular: they do not determine every unknown")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f != 0:
                for k in range(c, n + 1):
                    M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for r in reversed(range(n)):
        x[r] = (M[r][n] - sum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


def _ivp(fun, t_end, y0, rtol=1e-13, atol=None):
    """Reference helper: scipy DOP853 integration from 0 to t_end; returns the final state."""
    from scipy.integrate import solve_ivp
    if atol is None:
        atol = [1e-16 * max(1.0, abs(v)) for v in y0]
    sol = solve_ivp(fun, (0.0, t_end), list(y0), method="DOP853", rtol=rtol, atol=atol)
    if not sol.success:
        raise RuntimeError(sol.message)
    return [float(v) for v in sol.y[:, -1]]


def _quad(f, a, b):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=400)
    return float(v)


def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


# ================================================================================================ units
_KU = {"mass": "kg", "mass_flow_rate": "kg/s", "molar_flow_rate": "mol/s", "amount": "mol", "temperature": "K",
       "temperature_difference": "K", "pressure": "Pa", "volume": "m^3", "flow_rate": "m^3/s", "time": "s",
       "molar_concentration": "mol/m^3", "first_order_rate_constant": "1/s", "second_order_rate_constant": "m^3/(mol*s)",
       "molar_energy": "J/mol", "molar_heat_capacity": "J/(mol*K)", "energy": "J", "power": "W",
       "specific_heat": "J/(kg*K)", "thermal_conductance": "W/K", "length": "m", "area": "m^2", "speed": "m/s",
       "density": "kg/m^3", "dynamic_viscosity": "Pa*s", "diffusion_coefficient": "m^2/s", "molar_flux": "mol/(m^2*s)",
       "molar_volume": "m^3/mol", "rock_permeability": "m^2", "productivity_index": "m^3/(s*Pa)",
       "decline_rate": "1/s", "compressibility": "1/Pa", "specific_cake_resistance": "m/kg",
       "filter_medium_resistance": "1/m", "mass_concentration": "kg/m^3", "angular_frequency": "rad/s",
       "time_squared": "s^2", "molar_mass": "kg/mol", "number": ""}

# spellings the shared unit reader lacks -> (factor to the SI atom, SI atom); written forms are converted here
_UCONV = {"psi": (6894.757293168361, "Pa"), "psia": (6894.757293168361, "Pa"), "BTU": (1055.05585262, "J"),
          "Btu": (1055.05585262, "J"), "btu": (1055.05585262, "J"), "lb": (0.45359237, "kg"), "lbm": (0.45359237, "kg"),
          "lbmol": (453.59237, "mol"), "bbl": (0.158987294928, "m^3"), "STB": (0.158987294928, "m^3"),
          "stb": (0.158987294928, "m^3"), "rb": (0.158987294928, "m^3"), "RB": (0.158987294928, "m^3"),
          "md": (9.869233e-16, "m^2"), "mD": (9.869233e-16, "m^2"), "cP": (1e-3, "(Pa*s)"), "cp": None,
          "acre": (4046.8564224, "m^2"), "acres": (4046.8564224, "m^2"), "in": (0.0254, "m"),
          "inch": (0.0254, "m"), "gal": (0.003785411784, "m^3"), "hPa": (100.0, "Pa"), "mbar": (100.0, "Pa")}
_GAUGE = {"psig": 6894.757293168361, "barg": 1e5, "bar g": 1e5, "bar(g)": 1e5, "kPa g": 1e3, "kPag": 1e3,
          "kPa(g)": 1e3, "MPa g": 1e6, "MPag": 1e6, "psi g": 6894.757293168361}
_FAHRENHEIT = ("°F", "degF", "deg F", "F°")
_CELSIUS = ("°C", "degC", "deg C", "C°", "celsius")


def _written_unit(num: str, unit: str, kind: str):
    """'value unit' for the runtime, with units the shared reader lacks converted here; None when it cannot be read."""
    v = float(num)
    u = unit.strip()
    if kind in ("temperature", "temperature_difference"):
        if u in _FAHRENHEIT:
            k = (v - 32.0) * 5.0 / 9.0 + 273.15 if kind == "temperature" else v * 5.0 / 9.0
            return f"{k!r} K"
        if kind == "temperature_difference" and u in _CELSIUS:
            return f"{v!r} K"                     # a temperature difference: 1 degC is 1 K
        if u == "R" or u == "°R":
            return None                           # Rankine: not read (and R is ambiguous)
    if kind == "pressure" and u in _GAUGE:
        return f"{v * _GAUGE[u] + P_ATM!r} Pa"
    atoms = re.findall(r"[A-Za-z°µμ]+", u)
    if not any(a in _UCONV for a in atoms):
        return f"{num} {u}"
    if any(_UCONV.get(a, 0) is None for a in atoms):
        return None
    if "(" in u or u.count("/") > 1:
        return None
    num_part, _, den_part = u.partition("/")
    factor, rebuilt = 1.0, []
    for part, sign in ((num_part, 1), (den_part, -1)):
        pieces = []
        for tok in [t for t in part.split("*") if t.strip()]:
            m = re.fullmatch(r"\s*([A-Za-z°µμ]+|1)\s*(?:\^\s*(-?\d+))?\s*", tok)
            if not m:
                return None
            atom, e = m.group(1), int(m.group(2) or 1)
            if atom in _UCONV:
                f, si = _UCONV[atom]
                factor *= f ** (sign * e)
                pieces.append(si + (f"^{e}" if e != 1 else ""))
            else:
                pieces.append(atom + (f"^{e}" if e != 1 else ""))
        rebuilt.append("*".join(pieces))
    out_u = rebuilt[0] if rebuilt[0] else "1"
    if den_part:
        out_u += "/" + (rebuilt[1] if "*" not in rebuilt[1] else "(" + rebuilt[1] + ")")
    return f"{v * factor!r} {out_u}"


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\](?:\s*;\s*\[[^\]]*\])*|\[\[.*\]\])\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard (tools/ask_sweep.py, 2026-10-23: question-reader errors in finished models) is built in:
      1. every stated number is bound: the question is split into 'name = value' pairs and any other text or number
         refuses the whole question (a stray number can never be ignored);
      2. the capability phrase must open the question and be followed by the colon, so wording that names a different
         capability (CSTR vs PFR, bubble vs dew point, exponential vs hyperbolic decline, first vs second order) never
         reaches this reader; no name is shared by two meanings within one procedure;
      3. a convention is an input: Antoine units, the list units, the controller type ... are required choices, never
         assumed; a value without its unit (a physical quantity) or with a unit on a pure number is refused;
      4. physical values are passed to the runtime with their units (converted here only when the shared reader lacks
         the unit), so a value is converted exactly once.
    """
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names = {}
    by = {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    for i in inputs:                       # a lower-case spelling only where it names no other input (P and p differ)
        names.setdefault(i["symbol"].lower(), i["symbol"])
    for a, s in (aliases or {}).items():
        names[a] = s
        names[a.lower()] = s

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") not in ("vector", "matrix"):
                    return None
                text = mv.group(2).strip()
                if by[key]["type"] == "matrix":
                    if text.startswith("[["):
                        text = text[1:-1]
                    rows = re.findall(r"\[([^\]]*)\]", text)
                    text = "; ".join(r.strip() for r in rows)
                out[key] = text
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is not None and by[key].get("type") in ("choice", "boolean"):
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip().lower().replace(" ", "_").replace("-", "_")
                    continue
                if key is not None and by[key].get("type") == "text":
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip()
                    continue
                return None
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "matrix", "choice", "boolean", "text"):
                return None
            kind = spec.get("kind", "number")
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            if kind != "number":
                if not unit:
                    return None
                w = _written_unit(num, unit, kind)
                if w is None:
                    return None
                out[key] = w
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            out[key] = float(num)
        return out
    return parse


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(repr(float(x)) for x in v) + "]"
    if t == "matrix":
        return "[" + ", ".join("[" + ", ".join(repr(float(x)) for x in row) + "]" for row in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        return f"{float(v)!r} {_KU[kind]}"
    return repr(float(v))


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, **extra):
    _KU[kind]                                   # every physical kind has its SI unit here
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required, **extra}


def _numin(sym, meaning, required=True, pct=False, **extra):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}
    if pct:
        d["pct"] = True
    return d


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=50, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _matin(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": required}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": list(examples), "questions": questions}
    LEVELS[key] = level


def _ex(inputs, outputs=None):
    return {"inputs": inputs, "outputs": outputs or {}}


def _textin(sym, meaning, required=True):
    return {"symbol": sym, "type": "text", "meaning": meaning, "required": required}


def _fracs(name, v, tol=1e-6):
    for x in v:
        _need(math.isfinite(x) and -1e-15 <= x <= 1 + 1e-15, f"{name}: every fraction must lie between 0 and 1")
    _need(abs(sum(v) - 1.0) <= tol, f"{name}: the fractions must sum to 1")


def _dirichlet(rng, n, dominant=None, lo=0.6, hi=0.95):
    if dominant is None:
        w = [rng.uniform(0.05, 1.0) for _ in range(n)]
        s = sum(w)
        return [x / s for x in w]
    main = rng.uniform(lo, hi)
    rest = [rng.uniform(0.05, 1.0) for _ in range(n - 1)]
    s = sum(rest)
    rest = [x / s * (1 - main) for x in rest]
    rest.insert(dominant, main)
    return rest


# ================================================================================================ MATERIAL BALANCES
def blending(W, p, P):
    n = len(W)
    _need(2 <= n <= 6 and all(len(r) == n for r in W) and len(p) == n,
          "give as many feed streams as components (2 to 6): W is n x n and p has n entries")
    for i, row in enumerate(W):
        _fracs(f"feed stream {i + 1}", row)
    _fracs("product composition p", p)
    _pos("the product flow P", P)
    A = [[W[i][j] for i in range(n)] for j in range(n)]
    F = _gauss(A, [P * x for x in p])
    _need(all(f >= -1e-9 * P for f in F), "this product cannot be made from these feeds (a feed flow would be negative)")
    F = [0.0 if abs(f) <= 1e-12 * P else f for f in F]
    return _out(F1=F[0], F=F)


def _ref_blending(W, p, P):
    import numpy as np
    F = np.linalg.solve(np.array(W, dtype=float).T, P * np.array(p, dtype=float))
    F = [0.0 if abs(f) <= 1e-12 * P else float(f) for f in F]
    return {"F1": F[0], "F": F}


def _wrong_blending(W, p, P):
    n = len(W)
    F = _gauss([list(r) for r in W], [P * x for x in p])        # the composition matrix not transposed
    return _out(F1=F[0], F=F)


def _rand_blending(rng):
    n = rng.choice([2, 3, 3, 4, 5])
    W = [_dirichlet(rng, n, dominant=i) for i in range(n)]
    F = [rng.uniform(0.5, 20.0) for _ in range(n)]
    P = sum(F)
    p = [sum(F[i] * W[i][j] for i in range(n)) / P for j in range(n)]
    return {"W": W, "p": p, "P": P}


_add("chemeng_blending_flows",
     name="Blending: feed flows that make a product of given composition", topic="material balances / mixing and blending",
     purpose="The mass flow of each feed stream needed to make a product of stated flow and composition, from the component balances (as many feeds as components).",
     inputs=[_matin("W", "mass fractions of the feed streams: one row per feed stream, one column per component (rows sum to 1)"),
             _vecin("p", "mass fractions of the product, one per component (sum to 1)", 2, 6),
             _phys("P", "mass_flow_rate", "product mass flow")],
     outputs=[_o("F1", "mass_flow_rate", "flow of the first feed stream"),
              _o("F", "number", "flow of every feed stream, in kg/s, in the order of the rows of W")],
     primary="F1", conditions=["as many feed streams as components (2 to 6)", "each composition sums to 1",
                               "the product lies within the reach of the feeds (no negative flow)"],
     method="component balances sum_i F_i w_ij = P p_j for every component j, solved by Gaussian elimination with partial pivoting",
     latex=r"\sum_i F_i\,w_{ij} = P\,p_j \quad (j = 1..n)",
     run=blending, reference=_ref_blending, wrong=_wrong_blending, rand=_rand_blending,
     phrases=("Blending feed flows", "Blending balance", "Blending"),
     examples=[_ex({"W": [[0.9, 0.1], [0.2, 0.8]], "p": [0.5, 0.5], "P": 100.0}),
               _ex({"W": [[0.8, 0.1, 0.1], [0.1, 0.7, 0.2], [0.05, 0.15, 0.8]], "p": [0.4, 0.3, 0.3], "P": 10.0})],
     level="intermediate",
     extra_q=[{"q": "Blending: W = [[0.9, 0.1], [0.2, 0.8]], p = [0.95, 0.05], P = 100 kg/s", "outcome": "INVALID_INPUT"},
              {"q": "Blending: W = [[0.9, 0.1], [0.2, 0.8]], p = [0.5, 0.5], P = 360000 kg/h", "like": 0}])


def binary_separator(F, zF, xD, xB):
    _pos("the feed flow F", F)
    for nm, v in (("zF", zF), ("xD", xD), ("xB", xB)):
        _frac(nm, v)
    _need(xB < zF < xD, "the compositions must satisfy xB < zF < xD (light component enriched in the distillate)")
    D = F * (zF - xB) / (xD - xB)
    B = F - D
    return _out(D=D, B=B, recovery_D=D * xD / (F * zF), recovery_B=B * (1 - xB) / (F * (1 - zF)) if zF < 1 else 1.0)


def _ref_binary_separator(F, zF, xD, xB):
    import numpy as np
    D, B = np.linalg.solve(np.array([[1.0, 1.0], [xD, xB]]), np.array([F, F * zF]))
    D, B = float(D), float(B)
    return {"D": D, "B": B, "recovery_D": D * xD / (F * zF), "recovery_B": B * (1 - xB) / (F * (1 - zF))}


_add("chemeng_binary_separator_balance",
     name="Two-product separator balance (binary distillation column)", topic="material balances / separators",
     purpose="Distillate and bottoms flows of a binary separator from the feed flow and the three compositions, with the recoveries of the light component in the distillate and of the heavy component in the bottoms.",
     inputs=[_phys("F", "molar_flow_rate", "feed molar flow"),
             _numin("zF", "mole fraction of the light component in the feed", pct=True),
             _numin("xD", "mole fraction of the light component in the distillate (top product)", pct=True),
             _numin("xB", "mole fraction of the light component in the bottoms", pct=True)],
     outputs=[_o("D", "molar_flow_rate", "distillate flow"), _o("B", "molar_flow_rate", "bottoms flow"),
              _o("recovery_D", "number", "fraction of the light component recovered in the distillate"),
              _o("recovery_B", "number", "fraction of the heavy component recovered in the bottoms")],
     primary="D", conditions=["xB < zF < xD", "steady state, no reaction"],
     method="total balance F = D + B and light-component balance F zF = D xD + B xB",
     latex=r"D = F\,\frac{z_F - x_B}{x_D - x_B},\quad B = F - D",
     run=binary_separator, reference=_ref_binary_separator,
     wrong=lambda F, zF, xD, xB: _out(D=F * (xD - zF) / (xD - xB), B=F * (zF - xB) / (xD - xB), recovery_D=0.0, recovery_B=0.0),
     rand=lambda rng: (lambda xB, xD: {"F": _lu(rng, 1, 1000), "xB": xB, "xD": xD, "zF": rng.uniform(xB + 0.05, xD - 0.05)})(
         rng.uniform(0.001, 0.2), rng.uniform(0.6, 0.999)),
     phrases=("Two-product separator balance", "Binary distillation balance", "Distillation column material balance"),
     examples=[_ex({"F": 100.0, "zF": 0.4, "xD": 0.95, "xB": 0.05}), _ex({"F": 2.5, "zF": 0.3, "xD": 0.9, "xB": 0.02})],
     level="beginner",
     extra_q=[{"q": "Binary distillation balance: F = 360 kmol/h, zF = 40%, xD = 95%, xB = 5%", "like": 0},
              {"q": "Two-product separator balance: F = 100 mol/s, zF = 0.4, xD = 0.3, xB = 0.05", "outcome": "INVALID_INPUT"},
              {"q": "Two-product separator balance: F = 100, zF = 0.4, xD = 0.95, xB = 0.05", "outcome": "UNRECOGNIZED_FORMAT"}])


def recycle_purge(F0, zA, X, f):
    _pos("the fresh feed F0", F0)
    _frac("zA", zA, lo_open=True)
    _frac("the single-pass conversion X", X, lo_open=True, hi_open=True)
    _frac("the purge fraction f", f, lo_open=True, hi_open=True)
    A_in = F0 * zA / (1.0 - (1.0 - f) * (1.0 - X))
    I_in = F0 * (1.0 - zA) / f
    S_A, S_I = (1.0 - X) * A_in, I_in
    S = S_A + S_I
    P = f * S
    return _out(R=(1.0 - f) * S, P=P, reactor_feed=A_in + I_in, X_overall=1.0 - f * S_A / (F0 * zA),
                product=X * A_in, y_inert=S_I / S)


def _ref_recycle_purge(F0, zA, X, f):
    """Sequential-modular simulation: tear the recycle stream and iterate the loop (mixer, reactor, separator, purge
    splitter) by direct substitution until the recycle stream stops changing."""
    RA = RI = 0.0
    for _ in range(200000):
        A_in, I_in = F0 * zA + RA, F0 * (1 - zA) + RI
        SA, SI = A_in * (1 - X), I_in                # reactor, then the separator removes all product B
        nRA, nRI = SA * (1 - f), SI * (1 - f)        # splitter: the purge takes f of the separator overhead
        if abs(nRA - RA) <= 1e-17 * max(1.0, nRA) and abs(nRI - RI) <= 1e-17 * max(1.0, nRI):
            RA, RI = nRA, nRI
            break
        RA, RI = nRA, nRI
    A_in, I_in = F0 * zA + RA, F0 * (1 - zA) + RI
    SA, SI = A_in * (1 - X), I_in
    S = SA + SI
    return {"R": (1 - f) * S, "P": f * S, "reactor_feed": A_in + I_in, "X_overall": (F0 * zA - f * SA) / (F0 * zA),
            "product": F0 * zA - f * SA, "y_inert": SI / S}


_add("chemeng_recycle_purge",
     name="Reactor with recycle and purge", topic="material balances / recycle and purge",
     purpose="Steady-state flows of a reactor A -> B with complete product separation, recycle of unreacted A and inert, and a purge that removes a fraction of the recycle stream to stop the inert accumulating.",
     inputs=[_phys("F0", "molar_flow_rate", "fresh feed flow (reactant A plus inert)"),
             _numin("zA", "mole fraction of A in the fresh feed (the rest is inert)", pct=True),
             _numin("X", "single-pass conversion of A in the reactor", pct=True),
             _numin("f", "purge fraction: the fraction of the separator overhead (unreacted A + inert) that is purged", pct=True)],
     outputs=[_o("R", "molar_flow_rate", "recycle flow"), _o("P", "molar_flow_rate", "purge flow"),
              _o("reactor_feed", "molar_flow_rate", "combined reactor feed flow"),
              _o("X_overall", "number", "overall conversion of A (fresh feed to product)"),
              _o("product", "molar_flow_rate", "production rate of B"),
              _o("y_inert", "number", "mole fraction of inert in the recycle and purge")],
     primary="R", conditions=["0 < zA <= 1", "0 < X < 1", "0 < f < 1", "A -> B one to one; the separator removes all B"],
     method="balances on A and inert around the mixer, reactor, separator and splitter: A_in = F0 zA/(1 - (1 - f)(1 - X)), inert_in = F0 (1 - zA)/f",
     latex=r"A_{in} = \frac{F_0 z_A}{1 - (1-f)(1-X)},\quad I_{in} = \frac{F_0(1-z_A)}{f}",
     run=recycle_purge, reference=_ref_recycle_purge,
     wrong=lambda F0, zA, X, f: recycle_purge(F0, zA, X, f / 2.0),
     rand=lambda rng: {"F0": _lu(rng, 1, 500), "zA": rng.choice([1.0, rng.uniform(0.6, 0.99)]),
                       "X": rng.uniform(0.1, 0.9), "f": rng.uniform(0.02, 0.5)},
     phrases=("Recycle and purge balance", "Recycle with purge"),
     examples=[_ex({"F0": 100.0, "zA": 0.98, "X": 0.25, "f": 0.1}), _ex({"F0": 50.0, "zA": 0.9, "X": 0.6, "f": 0.05})],
     level="advanced",
     extra_q=[{"q": "Recycle and purge balance: F0 = 100 mol/s, zA = 98%, X = 25%, f = 0", "outcome": "INVALID_INPUT"}])


def mixing_tank(V, q, C_in, C0, t):
    _pos("the volume V and flow q", V, q)
    _nonneg("the concentrations and time", C_in, C0, t)
    return _out(C=C_in + (C0 - C_in) * math.exp(-q * t / V))


_add("chemeng_unsteady_mixing_tank",
     name="Unsteady well-mixed tank (concentration change)", topic="material balances / unsteady state",
     purpose="Concentration in a constant-volume well-mixed tank at time t after the inlet concentration changes (inflow equals outflow).",
     inputs=[_phys("V", "volume", "liquid volume of the tank"), _phys("q", "flow_rate", "volumetric flow in and out"),
             _phys("C_in", "molar_concentration", "inlet concentration"),
             _phys("C0", "molar_concentration", "concentration in the tank at t = 0"), _phys("t", "time", "time")],
     outputs=[_o("C", "molar_concentration", "concentration in the tank (and outlet) at time t")],
     primary="C", conditions=["V > 0, q > 0", "perfect mixing", "constant volume (inflow = outflow)"],
     method="solute balance V dC/dt = q (C_in - C), integrated: C = C_in + (C0 - C_in) exp(-q t/V)",
     latex=r"C(t) = C_{in} + (C_0 - C_{in})\,e^{-q t/V}",
     run=mixing_tank, reference=lambda V, q, C_in, C0, t: {"C": _ivp(lambda s, y: [q / V * (C_in - y[0])], t, [C0])[0]},
     wrong=lambda V, q, C_in, C0, t: _out(C=C0 * math.exp(-q * t / V)),
     rand=lambda rng: {"V": _lu(rng, 0.1, 50), "q": _lu(rng, 1e-4, 0.1), "C_in": rng.uniform(5, 2000),
                       "C0": rng.uniform(0, 2000), "t": _lu(rng, 10, 1e5)},
     phrases=("Unsteady mixing tank", "Well-mixed tank concentration"),
     examples=[_ex({"V": 10.0, "q": 0.01, "C_in": 500.0, "C0": 0.0, "t": 1000.0}),
               _ex({"V": 2.0, "q": 0.001, "C_in": 0.0, "C0": 800.0, "t": 3600.0})],
     level="beginner",
     extra_q=[{"q": "Unsteady mixing tank: V = 10 m^3, q = 36 m^3/h, C_in = 0.5 mol/L, C0 = 0 mol/L, t = 1000 s", "like": 0}])


def variable_volume_tank(V0, q_in, q_out, C_in, C0, t):
    _pos("the initial volume V0 and inflow q_in", V0, q_in)
    _nonneg("the outflow, concentrations and time", q_out, C_in, C0, t)
    V = V0 + (q_in - q_out) * t
    _need(V > 0, "the tank would be empty before time t")
    if q_in == q_out:
        C = C_in + (C0 - C_in) * math.exp(-q_in * t / V0)
    else:
        C = C_in - (C_in - C0) * (V0 / V) ** (q_in / (q_in - q_out))
    return _out(V=V, C=C)


def _ref_variable_volume_tank(V0, q_in, q_out, C_in, C0, t):
    V, n = _ivp(lambda s, y: [q_in - q_out, q_in * C_in - q_out * y[1] / y[0]], t, [V0, V0 * C0])
    return {"V": V, "C": n / V}


_add("chemeng_variable_volume_tank",
     name="Filling or draining well-mixed tank (variable volume)", topic="material balances / unsteady state",
     purpose="Volume and concentration of a well-mixed tank whose inflow and outflow differ, at time t.",
     inputs=[_phys("V0", "volume", "liquid volume at t = 0"), _phys("q_in", "flow_rate", "volumetric inflow"),
             _phys("q_out", "flow_rate", "volumetric outflow"), _phys("C_in", "molar_concentration", "inlet concentration"),
             _phys("C0", "molar_concentration", "concentration in the tank at t = 0"), _phys("t", "time", "time")],
     outputs=[_o("V", "volume", "liquid volume at time t"), _o("C", "molar_concentration", "concentration at time t")],
     primary="C", conditions=["V0 > 0, q_in > 0", "the tank does not empty before t", "perfect mixing"],
     method="total balance dV/dt = q_in - q_out and solute balance d(VC)/dt = q_in C_in - q_out C, integrated: C = C_in - (C_in - C0)(V0/V)^(q_in/(q_in - q_out))",
     latex=r"C = C_{in} - (C_{in} - C_0)\left(\frac{V_0}{V}\right)^{q_{in}/(q_{in}-q_{out})},\ V = V_0 + (q_{in}-q_{out})t",
     run=variable_volume_tank, reference=_ref_variable_volume_tank,
     wrong=lambda V0, q_in, q_out, C_in, C0, t: (lambda V: _out(V=V, C=C_in - (C_in - C0) * (V0 / V) ** (q_out / (q_in - q_out))))(V0 + (q_in - q_out) * t),
     rand=lambda rng: (lambda V0, qi, qo: {"V0": V0, "q_in": qi, "q_out": qo, "C_in": rng.uniform(10, 2000),
                                            "C0": rng.uniform(0, 2000), "t": rng.uniform(0.05, 0.8) * (V0 / max(qo - qi, 1e-9) if qo > qi else V0 / qi * 3)})(
         _lu(rng, 0.5, 20), _lu(rng, 1e-4, 0.02), _lu(rng, 1e-4, 0.02)),
     phrases=("Variable-volume tank", "Filling tank concentration", "Draining tank concentration"),
     examples=[_ex({"V0": 1.0, "q_in": 0.002, "q_out": 0.001, "C_in": 1000.0, "C0": 0.0, "t": 1000.0}),
               _ex({"V0": 5.0, "q_in": 0.001, "q_out": 0.003, "C_in": 0.0, "C0": 500.0, "t": 1000.0})],
     level="advanced",
     extra_q=[{"q": "Variable-volume tank: V0 = 1 m^3, q_in = 0.001 m^3/s, q_out = 0.003 m^3/s, C_in = 0 mol/m^3, C0 = 500 mol/m^3, t = 1000 s", "outcome": "INVALID_INPUT"}])


# --- combustion: complete combustion of a fuel CxHyOzNwSv with excess dry air -------------------------------------
def _formula(text):
    s = str(text).strip()
    _need(bool(re.fullmatch(r"(?:[A-Z][a-z]?\d*)+", s)), "write the fuel as a molecular formula such as CH4, C3H8 or C2H5OH")
    counts = {}
    for el, n in re.findall(r"([A-Z][a-z]?)(\d*)", s):
        _need(el in AW, f"element {el} is not supported (fuels of C, H, O, N and S only)")
        counts[el] = counts.get(el, 0) + (int(n) if n else 1)
    _need(counts.get("C", 0) + counts.get("H", 0) + counts.get("S", 0) > 0, "the fuel must contain C, H or S")
    return {e: counts.get(e, 0) for e in ("C", "H", "O", "N", "S")}


def combustion(fuel, excess_air):
    c = _formula(fuel)
    _fin(excess_air)
    _need(0 <= excess_air <= 5, "the excess air must be between 0 and 500 %")
    o2_th = c["C"] + c["H"] / 4.0 + c["S"] - c["O"] / 2.0
    _need(o2_th > 0, "this formula needs no oxygen to burn")
    o2_in = (1.0 + excess_air) * o2_th
    n = {"CO2": float(c["C"]), "H2O": c["H"] / 2.0, "SO2": float(c["S"]), "O2": excess_air * o2_th,
         "N2": N2_PER_O2 * o2_in + c["N"] / 2.0}
    wet = sum(n.values())
    dry = wet - n["H2O"]
    mw = sum(AW[e] * k for e, k in c.items())
    air = o2_in / 0.21
    return _out(O2_theoretical=o2_th, air=air, n_CO2=n["CO2"], n_H2O=n["H2O"], n_SO2=n["SO2"], n_O2=n["O2"], n_N2=n["N2"],
                y_CO2_dry=n["CO2"] / dry, y_O2_dry=n["O2"] / dry, y_H2O_wet=n["H2O"] / wet, AFR_mass=air * MW_AIR / mw)


def _ref_combustion(fuel, excess_air):
    """Element balances (C, H, S, O, N) solved with numpy; atom counts and molar masses from chemicals."""
    import numpy as np
    from chemicals.elements import simple_formula_parser, molecular_weight, periodic_table
    at = simple_formula_parser(fuel)
    c, h, o, nn, s = (float(at.get(e, 0)) for e in ("C", "H", "O", "N", "S"))
    k = N2_PER_O2
    # theoretical air: unknowns [CO2, H2O, SO2, N2, a], no O2 left
    A = np.array([[1, 0, 0, 0, 0], [0, 2, 0, 0, 0], [0, 0, 1, 0, 0], [2, 1, 2, 0, -2], [0, 0, 0, 2, -2 * k]], float)
    a = float(np.linalg.solve(A, np.array([c, h, s, o, nn]))[4])
    o2_in = (1 + excess_air) * a
    # actual: unknowns [CO2, H2O, SO2, O2, N2]
    A = np.array([[1, 0, 0, 0, 0], [0, 2, 0, 0, 0], [0, 0, 1, 0, 0], [2, 1, 2, 2, 0], [0, 0, 0, 0, 2]], float)
    x = [float(v) for v in np.linalg.solve(A, np.array([c, h, s, o + 2 * o2_in, nn + 2 * k * o2_in]))]
    x = [0.0 if abs(v) < 1e-13 else v for v in x]
    wet = sum(x)
    dry = wet - x[1]
    mw_air = 0.21 * 2 * periodic_table.O.MW + 0.79 * 2 * periodic_table.N.MW
    air = o2_in / 0.21
    return {"O2_theoretical": a, "air": air, "n_CO2": x[0], "n_H2O": x[1], "n_SO2": x[2], "n_O2": x[3], "n_N2": x[4],
            "y_CO2_dry": x[0] / dry, "y_O2_dry": x[3] / dry, "y_H2O_wet": x[1] / wet,
            "AFR_mass": air * mw_air / molecular_weight(at)}


_FUELS = ["CH4", "C2H6", "C3H8", "C4H10", "C8H18", "C12H26", "CH3OH", "C2H5OH", "H2", "CO", "C2H2", "C6H6", "CH3SH",
          "C2H5SH", "CH3NH2", "C7H16", "C10H8", "C3H6O"]

_add("chemeng_combustion_flue_gas",
     name="Combustion with excess air: air requirement and flue-gas composition", topic="material balances / combustion",
     purpose="Theoretical oxygen, air supplied, flue-gas moles and wet and dry compositions, and the air-fuel mass ratio for complete combustion of a fuel with excess dry air (21 % O2, 79 % N2).",
     inputs=[_textin("fuel", "molecular formula of the fuel (C, H, O, N and S), e.g. CH4, C3H8, C2H5OH"),
             _numin("excess_air", "excess air as a fraction of the theoretical air (20% = 0.2)", pct=True)],
     outputs=[_o("O2_theoretical", "number", "theoretical O2, mol per mol fuel"), _o("air", "number", "air supplied, mol per mol fuel"),
              _o("n_CO2", "number", "CO2 in the flue gas, mol per mol fuel"), _o("n_H2O", "number", "H2O, mol per mol fuel"),
              _o("n_SO2", "number", "SO2, mol per mol fuel"), _o("n_O2", "number", "O2, mol per mol fuel"),
              _o("n_N2", "number", "N2, mol per mol fuel"), _o("y_CO2_dry", "number", "CO2 mole fraction, dry basis"),
              _o("y_O2_dry", "number", "O2 mole fraction, dry basis"), _o("y_H2O_wet", "number", "H2O mole fraction, wet basis"),
              _o("AFR_mass", "number", "air-fuel ratio, kg air per kg fuel")],
     primary="y_CO2_dry", conditions=["complete combustion to CO2, H2O and SO2; fuel N leaves as N2", "dry air 21 % O2 / 79 % N2", "0 <= excess air <= 500 %"],
     method="O2 theoretical = C + H/4 + S - O/2 per mol fuel; air = (1 + e) O2/0.21; products by element balance",
     latex=r"n_{O_2,th} = x + \tfrac{y}{4} + v - \tfrac{z}{2},\quad n_{air} = \frac{(1+e)\,n_{O_2,th}}{0.21}",
     run=combustion, reference=_ref_combustion,
     wrong=lambda fuel, excess_air: (lambda c: combustion(fuel, excess_air) | _out(
         y_CO2_dry=c["C"] / (c["C"] + excess_air * (c["C"] + c["H"] / 2.0) + N2_PER_O2 * (1 + excess_air) * (c["C"] + c["H"] / 2.0))))(_formula(fuel)),
     rand=lambda rng: {"fuel": rng.choice(_FUELS), "excess_air": rng.choice([0.0, rng.uniform(0.02, 1.0)])},
     phrases=("Combustion flue gas", "Flue gas analysis", "Combustion with excess air"),
     examples=[_ex({"fuel": "CH4", "excess_air": 0.2}), _ex({"fuel": "C3H8", "excess_air": 0.0}),
               _ex({"fuel": "C2H5OH", "excess_air": 0.5})],
     level="intermediate",
     extra_q=[{"q": "Combustion flue gas: fuel = CH4, excess_air = 20%", "like": 0},
              {"q": "Combustion flue gas: fuel = CH4, excess_air = 20", "outcome": "UNRECOGNIZED_FORMAT"},
              {"q": "Combustion flue gas: fuel = CH4Cl, excess_air = 0.2", "outcome": "INVALID_INPUT"}])


# ================================================================================================ ENERGY BALANCES
def _cp_int(co, T1, T2):
    """Integral of a + bT + cT^2 + dT^3 from T1 to T2 (J/mol)."""
    a, b, c, d = (list(co) + [0.0, 0.0, 0.0])[:4]
    return a * (T2 - T1) + b / 2 * (T2 ** 2 - T1 ** 2) + c / 3 * (T2 ** 3 - T1 ** 3) + d / 4 * (T2 ** 4 - T1 ** 4)


def _cp_at(co, T):
    a, b, c, d = (list(co) + [0.0, 0.0, 0.0])[:4]
    return a + T * (b + T * (c + T * d))


def sensible_heat(n, a, b, T1, T2, c=0.0, d=0.0):
    _pos("the amount n", n)
    _fin(a, b, c, d)
    _pos("the temperatures", T1, T2)
    _need(T1 != T2, "T1 and T2 must differ")
    Q = n * _cp_int((a, b, c, d), T1, T2)
    return _out(Q=Q, cp_mean=Q / (n * (T2 - T1)))


def _ref_sensible_heat(n, a, b, T1, T2, c=0.0, d=0.0):
    Q = n * _quad(lambda T: a + b * T + c * T * T + d * T ** 3, T1, T2)
    return {"Q": Q, "cp_mean": Q / (n * (T2 - T1))}


def _rand_cp(rng):
    return {"a": rng.uniform(20, 60), "b": rng.uniform(-0.01, 0.08), "c": rng.uniform(-3e-5, 3e-5), "d": rng.uniform(-5e-9, 5e-9)}


_add("chemeng_sensible_heat_cp_polynomial",
     name="Sensible heat with a polynomial heat capacity", topic="energy balances / sensible heat",
     purpose="Heat needed to take n moles from T1 to T2 when cp = a + bT + cT^2 + dT^3 (J/(mol K), T in K), and the mean heat capacity over the range.",
     inputs=[_phys("n", "amount", "amount of substance"), _numin("a", "cp coefficient a, J/(mol K)"),
             _numin("b", "cp coefficient b, J/(mol K^2)"), _numin("c", "cp coefficient c, J/(mol K^3) (default 0)", required=False),
             _numin("d", "cp coefficient d, J/(mol K^4) (default 0)", required=False),
             _phys("T1", "temperature", "initial temperature"), _phys("T2", "temperature", "final temperature")],
     outputs=[_o("Q", "energy", "heat added (negative when cooled)"), _o("cp_mean", "molar_heat_capacity", "mean heat capacity between T1 and T2")],
     primary="Q", conditions=["no phase change between T1 and T2", "T in kelvin inside the polynomial"],
     method="Q = n * integral of cp dT, integrated term by term",
     latex=r"Q = n\int_{T_1}^{T_2}(a + bT + cT^2 + dT^3)\,dT",
     run=sensible_heat, reference=_ref_sensible_heat,
     wrong=lambda n, a, b, T1, T2, c=0.0, d=0.0: (lambda Q: _out(Q=Q, cp_mean=Q / (n * (T2 - T1))))(
         n * (a * (T2 - T1) + b * (T2 ** 2 - T1 ** 2) + c * (T2 ** 3 - T1 ** 3) + d * (T2 ** 4 - T1 ** 4))),
     rand=lambda rng: dict(_rand_cp(rng), n=_lu(rng, 0.1, 1000), T1=rng.uniform(250, 600), T2=rng.uniform(650, 1500)),
     phrases=("Sensible heat with polynomial cp", "Sensible heat", "Heat to warm a gas"),
     examples=[_ex({"n": 10.0, "a": 29.0, "b": 0.002, "c": 0.0, "d": 0.0, "T1": 300.0, "T2": 500.0}),
               _ex({"n": 1.0, "a": 19.89, "b": 0.05024, "c": 1.269e-05, "d": -1.101e-08, "T1": 298.15, "T2": 1000.0})],
     level="beginner",
     extra_q=[{"q": "Sensible heat: n = 10 mol, a = 29, b = 0.002, T1 = 26.85 degC, T2 = 226.85 degC", "like": 0}])


def _cp_matrix(cp, n):
    _need(len(cp) == n and 1 <= len(cp[0]) <= 4, "give one row of 1 to 4 cp coefficients (a, b, c, d) per species")
    return [list(r) + [0.0] * (4 - len(r)) for r in cp]


def reaction_enthalpy_T(nu, cp, dH0, T0, T):
    n = len(nu)
    rows = _cp_matrix(cp, n)
    _fin(dH0, *nu)
    _pos("the temperatures", T0, T)
    dco = [sum(nu[i] * rows[i][j] for i in range(n)) for j in range(4)]
    return _out(dH=dH0 + _cp_int(dco, T0, T))


def _ref_reaction_enthalpy_T(nu, cp, dH0, T0, T):
    rows = _cp_matrix(cp, len(nu))
    f = lambda t: sum(v * (r[0] + r[1] * t + r[2] * t * t + r[3] * t ** 3) for v, r in zip(nu, rows))
    return {"dH": dH0 + _quad(f, T0, T)}


def _rand_reaction(rng, nsp=None):
    nsp = nsp or rng.choice([2, 3, 4])
    nu = [-1.0] + [rng.choice([-2.0, -1.0, -0.5, 0.5, 1.0, 2.0]) for _ in range(nsp - 2)] + [rng.choice([1.0, 2.0])]
    cp = [[rng.uniform(20, 60), rng.uniform(0.0, 0.06), rng.uniform(-2e-5, 1e-5), rng.uniform(-2e-9, 2e-9)] for _ in range(nsp)]
    return nu, cp


_add("chemeng_reaction_enthalpy_at_temperature",
     name="Heat of reaction at another temperature (Kirchhoff)", topic="energy balances / heat of reaction",
     purpose="The heat of reaction at T from its value at T0 and the polynomial heat capacities of the species (Kirchhoff's law).",
     inputs=[_vecin("nu", "stoichiometric coefficients, negative for reactants and positive for products", 2, 8),
             _matin("cp", "heat-capacity coefficients, one row (a, b, c, d) per species in the order of nu; cp = a + bT + cT^2 + dT^3 in J/(mol K)"),
             _phys("dH0", "molar_energy", "heat of reaction at T0, per mole of extent"),
             _phys("T0", "temperature", "temperature of the known heat of reaction"), _phys("T", "temperature", "temperature wanted")],
     outputs=[_o("dH", "molar_energy", "heat of reaction at T, per mole of extent")],
     primary="dH", conditions=["no phase change of any species between T0 and T"],
     method="dH(T) = dH(T0) + integral from T0 to T of sum_i nu_i cp_i dT",
     latex=r"\Delta H_r(T) = \Delta H_r(T_0) + \int_{T_0}^{T}\sum_i \nu_i c_{p,i}\,dT",
     run=reaction_enthalpy_T, reference=_ref_reaction_enthalpy_T,
     wrong=lambda nu, cp, dH0, T0, T: reaction_enthalpy_T(nu, cp, dH0, T, T0) | _out(dH=2 * dH0 - reaction_enthalpy_T(nu, cp, dH0, T0, T)["dH"]),
     rand=lambda rng: (lambda nc: {"nu": nc[0], "cp": nc[1], "dH0": -rng.uniform(2e4, 3e5), "T0": 298.15, "T": rng.uniform(350, 1200)})(_rand_reaction(rng)),
     phrases=("Heat of reaction at temperature", "Kirchhoff heat of reaction", "Reaction enthalpy at temperature"),
     examples=[_ex({"nu": [-1.0, -3.0, 2.0], "cp": [[27.27, 0.004927, 0.0, 0.0], [27.01, 0.003509, 0.0, 0.0], [24.62, 0.03753, -7.1e-06, 0.0]],
                    "dH0": -92220.0, "T0": 298.15, "T": 700.0}),
               _ex({"nu": [-1.0, -0.5, 1.0], "cp": [[29.0, 0.0], [29.4, 0.004], [30.0, 0.01]], "dH0": -241826.0, "T0": 298.15, "T": 1000.0})],
     level="intermediate")


def adiabatic_reactor_T(n0, nu, cp, dH, T_ref, T_in, key, X):
    n = len(nu)
    _need(len(n0) == n, "n0 and nu must have one entry per species")
    rows = _cp_matrix(cp, n)
    _need(isinstance(key, int) and 1 <= key <= n, "key is the number (1, 2, ...) of the limiting reactant in the species list")
    k = key - 1
    _need(nu[k] < 0 and n0[k] > 0, "the key species must be a reactant present in the feed")
    _frac("the conversion X", X)
    for v in n0:
        _need(v >= 0, "feed amounts must be zero or positive")
    _pos("the temperatures", T_ref, T_in)
    xi = X * n0[k] / (-nu[k])
    nout = [n0[i] + nu[i] * xi for i in range(n)]
    _need(all(v >= -1e-12 * max(n0) for v in nout), "the conversion would consume more of a reactant than is fed")
    h_feed = sum(n0[i] * _cp_int(rows[i], T_in, T_ref) for i in range(n)) + xi * dH

    def f(T):
        return h_feed + sum(nout[i] * _cp_int(rows[i], T_ref, T) for i in range(n))
    for j in range(61):
        Tj = 200.0 + j * 50.0
        _need(sum(nout[i] * _cp_at(rows[i], Tj) for i in range(n)) > 0, "the product heat capacity must stay positive from 200 to 3200 K")
    _need(f(200.0) < 0 < f(3200.0), "the adiabatic outlet temperature lies outside 200 to 3200 K")
    return _out(T_out=_bisect(f, 200.0, 3200.0))


def _ref_adiabatic_reactor_T(n0, nu, cp, dH, T_ref, T_in, key, X):
    """A different Hess path: react at the INLET temperature (dH carried from T_ref to T_in by quadrature of the
    heat-capacity difference), then heat the products from T_in to T_out; brentq on the energy balance."""
    rows = _cp_matrix(cp, len(nu))
    k = key - 1
    xi = X * n0[k] / (-nu[k])
    nout = [a + v * xi for a, v in zip(n0, nu)]
    cpf = lambda i: (lambda t: rows[i][0] + rows[i][1] * t + rows[i][2] * t * t + rows[i][3] * t ** 3)
    dH_in = dH + _quad(lambda t: sum(nu[i] * cpf(i)(t) for i in range(len(nu))), T_ref, T_in)
    g = lambda T: xi * dH_in + _quad(lambda t: sum(nout[i] * cpf(i)(t) for i in range(len(nu))), T_in, T)
    return {"T_out": _brentq(g, 200.0, 3200.0)}


def _rand_adiabatic(rng):
    for _ in range(100):
        nu, cp = _rand_reaction(rng, rng.choice([2, 3, 3, 4]))
        n0 = [rng.uniform(0.5, 5.0) if v < 0 else rng.choice([0.0, rng.uniform(0.0, 2.0)]) for v in nu]
        x = {"n0": n0, "nu": nu, "cp": cp, "dH": rng.choice([-1, 1]) * rng.uniform(5e3, 8e4), "T_ref": 298.15,
             "T_in": rng.uniform(300, 700), "key": 1, "X": rng.uniform(0.1, 0.9)}
        try:
            adiabatic_reactor_T(**x)
            return x
        except ValueError:
            continue
    return x


_add("chemeng_adiabatic_reactor_temperature",
     name="Adiabatic reactor outlet temperature", topic="energy balances / adiabatic reactors",
     purpose="Outlet temperature of an adiabatic reactor from the feed, the conversion of the limiting reactant, the heat of reaction at a reference temperature and polynomial heat capacities.",
     inputs=[_vecin("n0", "feed amounts of each species, mol (or mol/s) on one basis", 2, 8),
             _vecin("nu", "stoichiometric coefficients, negative for reactants", 2, 8),
             _matin("cp", "heat-capacity coefficients, one row (a, b, c, d) per species; cp = a + bT + cT^2 + dT^3 in J/(mol K)"),
             _phys("dH", "molar_energy", "heat of reaction at T_ref, per mole of extent"),
             _phys("T_ref", "temperature", "reference temperature of dH"), _phys("T_in", "temperature", "feed temperature"),
             _intin("key", "number of the limiting reactant in the species list (1 = first)", 1, 8),
             _numin("X", "conversion of the limiting reactant", pct=True)],
     outputs=[_o("T_out", "temperature", "adiabatic outlet temperature")],
     primary="T_out", conditions=["adiabatic, no shaft work, no phase change", "outlet between 200 and 3200 K"],
     method="energy balance along the path: cool the feed to T_ref, react at T_ref, heat the products to T_out; T_out by bisection",
     latex=r"\sum_i n_{i0}\!\int_{T_{in}}^{T_{ref}}\!c_{p,i}dT + \xi\,\Delta H_r(T_{ref}) + \sum_i n_i\!\int_{T_{ref}}^{T_{out}}\!c_{p,i}dT = 0",
     run=adiabatic_reactor_T, reference=_ref_adiabatic_reactor_T,
     wrong=lambda n0, nu, cp, dH, T_ref, T_in, key, X: (lambda rows, xi: (lambda nout: _out(T_out=_bisect(
         lambda T: xi * dH + sum(nout[i] * _cp_int(rows[i], T_in, T) for i in range(len(nu))), 200.0, 3200.0)))(
         [a + v * xi for a, v in zip(n0, nu)]))(_cp_matrix(cp, len(nu)), X * n0[key - 1] / (-nu[key - 1])),
     rand=_rand_adiabatic,
     phrases=("Adiabatic reactor temperature", "Adiabatic outlet temperature"),
     examples=[_ex({"n0": [1.0, 2.0, 0.0], "nu": [-1.0, -1.0, 1.0], "cp": [[30.0, 0.01], [29.0, 0.004], [45.0, 0.02]],
                    "dH": -60000.0, "T_ref": 298.15, "T_in": 400.0, "key": 1, "X": 0.8}),
               _ex({"n0": [2.0, 0.0], "nu": [-1.0, 2.0], "cp": [[40.0, 0.02], [25.0, 0.01]], "dH": 30000.0,
                    "T_ref": 298.15, "T_in": 900.0, "key": 1, "X": 0.3})],
     level="advanced")


# --- adiabatic flame temperature with NASA polynomials (GRI-Mech 3.0 data, as in cantera's gri30.yaml) --------
# [T_mid, high-range a1..a7, low-range a1..a7]; h/(R T) = a1 + a2 T/2 + a3 T^2/3 + a4 T^3/4 + a5 T^4/5 + a6/T
_NASA = {
    "CH4": [1000.0, 0.074851495, 0.0133909467, -5.73285809e-06, 1.22292535e-09, -1.0181523e-13, -9468.34459, 18.437318,
            5.14987613, -0.0136709788, 4.91800599e-05, -4.84743026e-08, 1.66693956e-11, -10246.6476, -4.64130376],
    "C2H6": [1000.0, 1.0718815, 0.0216852677, -1.00256067e-05, 2.21412001e-09, -1.9000289e-13, -11426.3932, 15.1156107,
             4.29142492, -0.0055015427, 5.99438288e-05, -7.08466285e-08, 2.68685771e-11, -11522.2055, 2.66682316],
    "C3H8": [1000.0, 7.5341368, 0.018872239, -6.2718491e-06, 9.1475649e-10, -4.7838069e-14, -16467.516, -17.892349,
             0.93355381, 0.026424579, 6.1059727e-06, -2.1977499e-08, 9.5149253e-12, -13958.52, 19.201691],
    "H2": [1000.0, 3.3372792, -4.94024731e-05, 4.99456778e-07, -1.79566394e-10, 2.00255376e-14, -950.158922, -3.20502331,
           2.34433112, 0.00798052075, -1.9478151e-05, 2.01572094e-08, -7.37611761e-12, -917.935173, 0.683010238],
    "CO": [1000.0, 2.71518561, 0.00206252743, -9.98825771e-07, 2.30053008e-10, -2.03647716e-14, -14151.8724, 7.81868772,
           3.57953347, -0.00061035368, 1.01681433e-06, 9.07005884e-10, -9.04424499e-13, -14344.086, 3.50840928],
    "C2H4": [1000.0, 2.03611116, 0.0146454151, -6.71077915e-06, 1.47222923e-09, -1.25706061e-13, 4939.88614, 10.3053693,
             3.95920148, -0.00757052247, 5.70990292e-05, -6.91588753e-08, 2.69884373e-11, 5089.77593, 4.09733096],
    "C2H2": [1000.0, 4.14756964, 0.00596166664, -2.37294852e-06, 4.67412171e-10, -3.61235213e-14, 25935.9992, -1.23028121,
             0.808681094, 0.0233615629, -3.55171815e-05, 2.80152437e-08, -8.50072974e-12, 26428.9807, 13.9397051],
    "CH3OH": [1000.0, 1.78970791, 0.0140938292, -6.36500835e-06, 1.38171085e-09, -1.1706022e-13, -25374.8747, 14.5023623,
              5.71539582, -0.0152309129, 6.52441155e-05, -7.10806889e-08, 2.61352698e-11, -25642.7656, -1.50409823],
    "O2": [1000.0, 3.28253784, 0.00148308754, -7.57966669e-07, 2.09470555e-10, -2.16717794e-14, -1088.45772, 5.45323129,
           3.78245636, -0.00299673416, 9.84730201e-06, -9.68129509e-09, 3.24372837e-12, -1063.94356, 3.65767573],
    "N2": [1000.0, 2.92664, 0.0014879768, -5.68476e-07, 1.0097038e-10, -6.753351e-15, -922.7977, 5.980528,
           3.298677, 0.0014082404, -3.963222e-06, 5.641515e-09, -2.444854e-12, -1020.8999, 3.950372],
    "CO2": [1000.0, 3.85746029, 0.00441437026, -2.21481404e-06, 5.23490188e-10, -4.72084164e-14, -48759.166, 2.27163806,
            2.35677352, 0.00898459677, -7.12356269e-06, 2.45919022e-09, -1.43699548e-13, -48371.9697, 9.90105222],
    "H2O": [1000.0, 3.03399249, 0.00217691804, -1.64072518e-07, -9.7041987e-11, 1.68200992e-14, -30004.2971, 4.9667701,
            4.19864056, -0.0020364341, 6.52040211e-06, -5.48797062e-09, 1.77197817e-12, -30293.7267, -0.849032208],
}
_FLAME_FUELS = {"CH4": (1, 4, 0), "C2H6": (2, 6, 0), "C3H8": (3, 8, 0), "H2": (0, 2, 0), "CO": (1, 0, 1),
                "C2H4": (2, 4, 0), "C2H2": (2, 2, 0), "CH3OH": (1, 4, 1)}


def _h_nasa(sp, T):
    c = _NASA[sp]
    a = c[1:8] if T > c[0] else c[8:15]
    return R_GAS * T * (a[0] + T * (a[1] / 2 + T * (a[2] / 3 + T * (a[3] / 4 + T * a[4] / 5))) + a[5] / T)


def _flame_mix(fuel, excess_air):
    c, h, o = _FLAME_FUELS[fuel]
    o2 = c + h / 4.0 - o / 2.0
    reac = {fuel: 1.0, "O2": (1 + excess_air) * o2, "N2": N2_PER_O2 * (1 + excess_air) * o2}
    prod = {"CO2": float(c), "H2O": h / 2.0, "O2": excess_air * o2, "N2": N2_PER_O2 * (1 + excess_air) * o2}
    return reac, prod


def flame_temperature(fuel, excess_air, T_in):
    _need(fuel in _FLAME_FUELS, "fuel must be one of " + ", ".join(_FLAME_FUELS))
    _fin(excess_air)
    _need(0 <= excess_air <= 4, "the excess air must be between 0 and 400 %")
    _need(300 <= T_in <= 1000, "the inlet temperature must be between 300 and 1000 K (range of the data)")
    reac, prod = _flame_mix(fuel, excess_air)
    H = sum(n * _h_nasa(s, T_in) for s, n in reac.items())
    f = lambda T: sum(n * _h_nasa(s, T) for s, n in prod.items() if n) - H
    _need(f(3500.0) > 0, "the flame temperature would exceed 3500 K, the range of the data")
    return _out(T_ad=_bisect(f, T_in, 3500.0))


_CT: dict = {}


def _ref_flame_temperature(fuel, excess_air, T_in):
    """cantera: ideal-gas mixture of the fuel, O2, N2, CO2 and H2O with GRI-Mech 3.0 thermodynamics; the products
    (complete combustion, no dissociation) are brought to the reactants' enthalpy at constant pressure (HP), then
    polished by Newton steps on cantera's own h and cp."""
    import cantera as ct
    if fuel not in _CT:
        g = ct.Solution("gri30.yaml")
        _CT[fuel] = ct.Solution(thermo="ideal-gas", species=[g.species(n) for n in (fuel, "O2", "N2", "CO2", "H2O")])
    gas = _CT[fuel]
    reac, prod = _flame_mix(fuel, excess_air)
    gas.TPX = T_in, P_ATM, reac
    h = gas.h
    gas.TPX = 2000.0, P_ATM, {k: v for k, v in prod.items() if v}
    gas.HP = h, P_ATM
    for _ in range(4):
        T = gas.T
        dT = (h - gas.h) / gas.cp_mass
        gas.TP = T + dT, P_ATM
    return {"T_ad": float(gas.T)}


_add("chemeng_adiabatic_flame_temperature",
     name="Adiabatic flame temperature (complete combustion, NASA polynomials)", topic="energy balances / combustion",
     purpose="Adiabatic flame temperature of a gaseous fuel burned with excess dry air at constant pressure, complete combustion without dissociation, from NASA 7-coefficient polynomials (GRI-Mech 3.0 data).",
     inputs=[_choice("fuel", list(_FLAME_FUELS), "fuel: " + ", ".join(_FLAME_FUELS)),
             _numin("excess_air", "excess air as a fraction of theoretical (0 = stoichiometric)", pct=True),
             _phys("T_in", "temperature", "temperature of the fuel and air entering")],
     outputs=[_o("T_ad", "temperature", "adiabatic flame temperature")],
     primary="T_ad", conditions=["complete combustion to CO2 and H2O vapour, no dissociation", "300 K <= T_in <= 1000 K", "result <= 3500 K"],
     method="energy balance H_products(T_ad) = H_reactants(T_in) with NASA polynomial enthalpies (including formation enthalpies), solved by bisection",
     latex=r"\sum_{p} n_p\,h_p(T_{ad}) = \sum_{r} n_r\,h_r(T_{in}),\quad \frac{h}{RT} = a_1 + \frac{a_2T}{2} + \frac{a_3T^2}{3} + \frac{a_4T^3}{4} + \frac{a_5T^4}{5} + \frac{a_6}{T}",
     run=flame_temperature, reference=_ref_flame_temperature,
     wrong=lambda fuel, excess_air, T_in: (lambda reac, prod: _out(T_ad=_bisect(
         lambda T: sum(n * _h_nasa(s, T) for s, n in prod.items() if n and s != "N2") - sum(n * _h_nasa(s, T_in) for s, n in reac.items() if s != "N2"),
         T_in, 9000.0)))(*_flame_mix(fuel, excess_air)),
     rand=lambda rng: {"fuel": rng.choice(list(_FLAME_FUELS)), "excess_air": rng.uniform(0.05, 2.0), "T_in": rng.uniform(300, 800)},
     phrases=("Adiabatic flame temperature", "Flame temperature"),
     examples=[_ex({"fuel": "CH4", "excess_air": 0.0, "T_in": 298.15 + 1.85}), _ex({"fuel": "C3H8", "excess_air": 0.5, "T_in": 400.0})],
     level="advanced",
     extra_q=[{"q": "Adiabatic flame temperature: fuel = CH4, excess_air = 0%, T_in = 26.85 degC", "like": 0},
              {"q": "Adiabatic flame temperature: fuel = C8H18, excess_air = 0.2, T_in = 300 K", "outcome": "INVALID_INPUT"}])


def batch_heating_T(m, cp, UA, T_s, T0, t):
    _pos("m, cp and UA", m, cp, UA)
    _pos("the temperatures", T_s, T0)
    _nonneg("the time t", t)
    return _out(T=T_s - (T_s - T0) * math.exp(-UA * t / (m * cp)))


def batch_heating_time(m, cp, UA, T_s, T0, T):
    _pos("m, cp and UA", m, cp, UA)
    _pos("the temperatures", T_s, T0, T)
    _need(min(T0, T_s) <= T < max(T0, T_s) and T0 != T_s, "T must lie between T0 and the medium temperature T_s")
    return _out(t=m * cp / UA * math.log((T_s - T0) / (T_s - T)))


_BATCH_IN = [_phys("m", "mass", "mass of liquid in the tank"), _phys("cp", "specific_heat", "specific heat of the liquid"),
             _phys("UA", "thermal_conductance", "overall heat-transfer coefficient times area of the coil or jacket"),
             _phys("T_s", "temperature", "temperature of the heating or cooling medium (constant, e.g. condensing steam)"),
             _phys("T0", "temperature", "initial liquid temperature")]

_add("chemeng_batch_heating_temperature",
     name="Batch heating of a stirred tank: temperature after a time", topic="energy balances / unsteady heating",
     purpose="Temperature of a well-stirred batch heated or cooled by a medium at constant temperature (condensing steam, boiling refrigerant) after time t.",
     inputs=_BATCH_IN + [_phys("t", "time", "heating time")],
     outputs=[_o("T", "temperature", "liquid temperature at time t")],
     primary="T", conditions=["well stirred", "constant UA and cp", "medium at constant temperature"],
     method="energy balance m cp dT/dt = UA (T_s - T), integrated",
     latex=r"T = T_s - (T_s - T_0)\,e^{-UA\,t/(m c_p)}",
     run=batch_heating_T,
     reference=lambda m, cp, UA, T_s, T0, t: {"T": _ivp(lambda s, y: [UA * (T_s - y[0]) / (m * cp)], t, [T0])[0]},
     wrong=lambda m, cp, UA, T_s, T0, t: _out(T=T_s - (T_s - T0) * math.exp(-UA * t / m)),
     rand=lambda rng: {"m": _lu(rng, 100, 2e4), "cp": rng.uniform(1800, 4200), "UA": _lu(rng, 200, 2e4),
                       "T_s": rng.uniform(380, 450), "T0": rng.uniform(280, 330), "t": _lu(rng, 60, 2e4)},
     phrases=("Batch heating temperature", "Stirred tank heating temperature"),
     examples=[_ex({"m": 1000.0, "cp": 4180.0, "UA": 2000.0, "T_s": 393.15, "T0": 293.15, "t": 3600.0}),
               _ex({"m": 500.0, "cp": 2000.0, "UA": 500.0, "T_s": 273.15, "T0": 353.15, "t": 1800.0})],
     level="beginner",
     extra_q=[{"q": "Batch heating temperature: m = 1 t, cp = 4.18 kJ/(kg*K), UA = 2 kW/K, T_s = 120 degC, T0 = 20 degC, t = 1 h", "like": 0}])

_add("chemeng_batch_heating_time",
     name="Batch heating of a stirred tank: time to reach a temperature", topic="energy balances / unsteady heating",
     purpose="Time for a well-stirred batch heated or cooled by a medium at constant temperature to reach a stated temperature.",
     inputs=_BATCH_IN + [_phys("T", "temperature", "temperature to reach")],
     outputs=[_o("t", "time", "time needed")],
     primary="t", conditions=["T between T0 and T_s", "well stirred", "constant UA and cp"],
     method="t = (m cp/UA) ln((T_s - T0)/(T_s - T))",
     latex=r"t = \frac{m c_p}{UA}\ln\frac{T_s - T_0}{T_s - T}",
     run=batch_heating_time,
     reference=lambda m, cp, UA, T_s, T0, T: {"t": _quad(lambda x: m * cp / (UA * (T_s - x)), T0, T)},
     wrong=lambda m, cp, UA, T_s, T0, T: _out(t=m * cp / UA * math.log((T_s - T) / (T_s - T0))),
     rand=lambda rng: (lambda T0, Ts: {"m": _lu(rng, 100, 2e4), "cp": rng.uniform(1800, 4200), "UA": _lu(rng, 200, 2e4),
                                       "T_s": Ts, "T0": T0, "T": T0 + rng.uniform(0.05, 0.95) * (Ts - T0)})(
         rng.uniform(280, 330), rng.uniform(380, 450)),
     phrases=("Batch heating time", "Stirred tank heating time"),
     examples=[_ex({"m": 1000.0, "cp": 4180.0, "UA": 2000.0, "T_s": 393.15, "T0": 293.15, "T": 353.15}),
               _ex({"m": 500.0, "cp": 2000.0, "UA": 500.0, "T_s": 273.15, "T0": 353.15, "T": 300.0})],
     level="intermediate",
     extra_q=[{"q": "Batch heating time: m = 1000 kg, cp = 4180 J/(kg*K), UA = 2000 W/K, T_s = 393.15 K, T0 = 293.15 K, T = 400 K", "outcome": "INVALID_INPUT"}])


def batch_coil_cooling(M, cp, W, cp_c, t_in, UA, T0, t):
    _pos("M, cp, W, cp_c and UA", M, cp, W, cp_c, UA)
    _pos("the temperatures", t_in, T0)
    _nonneg("the time t", t)
    eff = 1.0 - math.exp(-UA / (W * cp_c))
    return _out(T=t_in + (T0 - t_in) * math.exp(-W * cp_c * eff * t / (M * cp)))


def _ref_batch_coil_cooling(M, cp, W, cp_c, t_in, UA, T0, t):
    """The coil: the coolant temperature along the coil integrated as an ODE in the transfer units (its outlet
    approach fraction), then the tank energy balance integrated in time."""
    ntu = UA / (W * cp_c)
    approach = _ivp(lambda s, y: [-y[0]], ntu, [1.0])[0]          # (T - t_c)/(T - t_in) at the coil outlet
    return {"T": _ivp(lambda s, y: [-W * cp_c * (1.0 - approach) * (y[0] - t_in) / (M * cp)], t, [T0])[0]}


_add("chemeng_batch_coil_cooling",
     name="Batch cooling (or heating) with a non-isothermal coolant in a coil", topic="energy balances / unsteady heating",
     purpose="Temperature of a well-stirred batch after time t when a coolant (or heating liquid) flows once through a coil and leaves warmer (Kern's batch cooling).",
     inputs=[_phys("M", "mass", "mass of liquid in the tank"), _phys("cp", "specific_heat", "specific heat of the batch liquid"),
             _phys("W", "mass_flow_rate", "coolant mass flow"), _phys("cp_c", "specific_heat", "specific heat of the coolant"),
             _phys("t_in", "temperature", "coolant inlet temperature"),
             _phys("UA", "thermal_conductance", "overall heat-transfer coefficient times coil area"),
             _phys("T0", "temperature", "initial batch temperature"), _phys("t", "time", "time")],
     outputs=[_o("T", "temperature", "batch temperature at time t")],
     primary="T", conditions=["well-stirred batch", "constant UA, cp and coolant flow"],
     method="coolant outlet approaches the batch temperature: T - t_out = (T - t_in) exp(-UA/(W cp_c)); tank balance M cp dT/dt = -W cp_c (t_out - t_in), integrated",
     latex=r"T = t_{in} + (T_0 - t_{in})\exp\!\left[-\frac{W c_{p,c}\,(1 - e^{-UA/(W c_{p,c})})\,t}{M c_p}\right]",
     run=batch_coil_cooling, reference=_ref_batch_coil_cooling,
     wrong=lambda M, cp, W, cp_c, t_in, UA, T0, t: _out(T=t_in + (T0 - t_in) * math.exp(-W * cp_c * math.exp(-UA / (W * cp_c)) * t / (M * cp))),
     rand=lambda rng: {"M": _lu(rng, 200, 2e4), "cp": rng.uniform(1800, 4200), "W": _lu(rng, 0.1, 5), "cp_c": rng.uniform(3500, 4200),
                       "t_in": rng.uniform(280, 300), "UA": _lu(rng, 200, 2e4), "T0": rng.uniform(330, 420), "t": _lu(rng, 60, 2e4)},
     phrases=("Batch coil cooling", "Batch cooling with coolant", "Kern batch cooling"),
     examples=[_ex({"M": 5000.0, "cp": 2500.0, "W": 2.0, "cp_c": 4180.0, "t_in": 288.15, "UA": 3000.0, "T0": 393.15, "t": 7200.0}),
               _ex({"M": 1000.0, "cp": 4180.0, "W": 0.5, "cp_c": 4180.0, "t_in": 363.15, "UA": 1000.0, "T0": 293.15, "t": 3600.0})],
     level="advanced")


# ================================================================================================ REACTOR DESIGN
_K1 = lambda d="first-order rate constant": _phys("k", "first_order_rate_constant", d)
_K2 = lambda d="second-order rate constant (-r_A = k C_A^2)": _phys("k", "second_order_rate_constant", d)
_CA0 = _phys("CA0", "molar_concentration", "feed (initial) concentration of A")
_V0 = _phys("v0", "flow_rate", "volumetric feed flow")
_XIN = _numin("X", "conversion of A wanted", pct=True)


def _conv_ok(X):
    _frac("the conversion X", X, hi_open=True)


def batch_time_1(k, X):
    _pos("k", k)
    _conv_ok(X)
    return _out(t=-math.log1p(-X) / k)


def batch_time_2(k, CA0, X):
    _pos("k and CA0", k, CA0)
    _conv_ok(X)
    return _out(t=X / (k * CA0 * (1.0 - X)))


_add("chemeng_batch_time_first_order",
     name="Batch reactor time for a conversion (first order)", topic="reactor design / batch reactors",
     purpose="Reaction time an isothermal batch reactor needs to reach conversion X for a first-order reaction.",
     inputs=[_K1(), _XIN], outputs=[_o("t", "time", "reaction time")], primary="t",
     conditions=["0 <= X < 1", "isothermal, constant volume, -r_A = k C_A"],
     method="design equation t = C_A0 * integral of dX/(-r_A) = ln(1/(1 - X))/k",
     latex=r"t = \frac{1}{k}\ln\frac{1}{1-X}",
     run=batch_time_1, reference=lambda k, X: {"t": _quad(lambda x: 1.0 / (k * (1.0 - x)), 0.0, X)},
     wrong=lambda k, X: _out(t=X / k), rand=lambda rng: {"k": _lu(rng, 1e-5, 1.0), "X": rng.uniform(0.05, 0.995)},
     phrases=("First-order batch reactor time", "Batch reactor time first order"),
     examples=[_ex({"k": 0.001, "X": 0.9}), _ex({"k": 0.05, "X": 0.5})], level="beginner",
     extra_q=[{"q": "First-order batch reactor time: k = 0.06 1/min, X = 90%", "like": 0},
              {"q": "First-order batch reactor time: k = 0.001 1/s, X = 1", "outcome": "INVALID_INPUT"}])

_add("chemeng_batch_time_second_order",
     name="Batch reactor time for a conversion (second order)", topic="reactor design / batch reactors",
     purpose="Reaction time an isothermal batch reactor needs to reach conversion X when -r_A = k C_A^2.",
     inputs=[_K2(), _CA0, _XIN], outputs=[_o("t", "time", "reaction time")], primary="t",
     conditions=["0 <= X < 1", "isothermal, constant volume"],
     method="t = C_A0 * integral of dX/(k C_A0^2 (1 - X)^2) = X/(k C_A0 (1 - X))",
     latex=r"t = \frac{X}{k\,C_{A0}(1-X)}",
     run=batch_time_2, reference=lambda k, CA0, X: {"t": _quad(lambda x: 1.0 / (k * CA0 * (1.0 - x) ** 2), 0.0, X)},
     wrong=lambda k, CA0, X: _out(t=-math.log1p(-X) / (k * CA0)),
     rand=lambda rng: {"k": _lu(rng, 1e-7, 1e-2), "CA0": _lu(rng, 10, 5000), "X": rng.uniform(0.05, 0.99)},
     phrases=("Second-order batch reactor time", "Batch reactor time second order"),
     examples=[_ex({"k": 1e-4, "CA0": 1000.0, "X": 0.8}), _ex({"k": 5e-6, "CA0": 2000.0, "X": 0.5})], level="intermediate",
     extra_q=[{"q": "Second-order batch reactor time: k = 0.1 L/(mol*s), CA0 = 1 mol/L, X = 0.8", "like": 0}])


def batch_conv_1(k, t):
    _pos("k", k)
    _nonneg("t", t)
    return _out(X=-math.expm1(-k * t))


def batch_conv_2(k, CA0, t):
    _pos("k and CA0", k, CA0)
    _nonneg("t", t)
    d = k * CA0 * t
    return _out(X=d / (1.0 + d), CA=CA0 / (1.0 + d))


_add("chemeng_batch_conversion_first_order",
     name="Batch reactor conversion after a time (first order)", topic="reactor design / batch reactors",
     purpose="Conversion reached in an isothermal batch reactor after time t for a first-order reaction.",
     inputs=[_K1(), _phys("t", "time", "reaction time")], outputs=[_o("X", "number", "conversion of A")], primary="X",
     conditions=["isothermal, constant volume, -r_A = k C_A"], method="X = 1 - exp(-k t)", latex=r"X = 1 - e^{-kt}",
     run=batch_conv_1, reference=lambda k, t: {"X": _ivp(lambda s, y: [k * (1.0 - y[0])], t, [0.0], atol=[1e-17])[0]},
     wrong=lambda k, t: _out(X=k * t / (1.0 + k * t)), rand=lambda rng: (lambda k: {"k": k, "t": _lu(rng, 0.05, 5.0) / k})(_lu(rng, 1e-5, 1.0)),
     phrases=("First-order batch reactor conversion", "Batch reactor conversion first order"),
     examples=[_ex({"k": 0.001, "t": 1000.0}), _ex({"k": 0.02, "t": 60.0})], level="beginner",
     extra_q=[{"q": "First-order batch reactor conversion: k = 0.06 1/min, t = 1000 s", "like": 0}])

_add("chemeng_batch_conversion_second_order",
     name="Batch reactor conversion after a time (second order)", topic="reactor design / batch reactors",
     purpose="Conversion and concentration reached in an isothermal batch reactor after time t when -r_A = k C_A^2.",
     inputs=[_K2(), _CA0, _phys("t", "time", "reaction time")],
     outputs=[_o("X", "number", "conversion of A"), _o("CA", "molar_concentration", "concentration of A at time t")], primary="X",
     conditions=["isothermal, constant volume"], method="1/C_A = 1/C_A0 + k t", latex=r"X = \frac{k C_{A0} t}{1 + k C_{A0} t}",
     run=batch_conv_2,
     reference=lambda k, CA0, t: (lambda C: {"X": 1.0 - C / CA0, "CA": C})(_ivp(lambda s, y: [-k * y[0] ** 2], t, [CA0])[0]),
     wrong=lambda k, CA0, t: _out(X=-math.expm1(-k * CA0 * t), CA=CA0 * math.exp(-k * CA0 * t)),
     rand=lambda rng: (lambda k, C: {"k": k, "CA0": C, "t": _lu(rng, 0.05, 20.0) / (k * C)})(_lu(rng, 1e-7, 1e-2), _lu(rng, 10, 5000)),
     phrases=("Second-order batch reactor conversion", "Batch reactor conversion second order"),
     examples=[_ex({"k": 1e-4, "CA0": 1000.0, "t": 40.0}), _ex({"k": 5e-6, "CA0": 2000.0, "t": 600.0})], level="intermediate")


def cstr_volume_1(v0, k, X):
    _pos("v0 and k", v0, k)
    _conv_ok(X)
    tau = X / (k * (1.0 - X))
    return _out(V=v0 * tau, tau=tau)


def _ref_cstr_volume_1(v0, k, X):
    """Segregation (RTD) route: a CSTR's exit-age distribution E(t) = exp(-t/tau)/tau weights the batch conversion
    1 - exp(-k t) (exact for first-order kinetics); tau found by brentq on that integral."""
    from scipy.integrate import quad
    xs = lambda tau: quad(lambda u: math.exp(-u) * -math.expm1(-k * tau * u), 0.0, math.inf, epsabs=0.0, epsrel=1e-13, limit=400)[0]
    tau = _brentq(lambda tau: xs(tau) - X, 1e-9 / k, 1e9 / k)
    return {"V": v0 * tau, "tau": tau}


_add("chemeng_cstr_volume_first_order",
     name="CSTR volume for a conversion (first order)", topic="reactor design / continuous stirred tanks",
     purpose="Volume and space time of a CSTR that reaches conversion X for a first-order liquid-phase reaction.",
     inputs=[_V0, _K1(), _XIN], outputs=[_o("V", "volume", "reactor volume"), _o("tau", "time", "space time V/v0")], primary="V",
     conditions=["0 <= X < 1", "steady state, perfect mixing, constant density, -r_A = k C_A"],
     method="design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X))",
     latex=r"V = \frac{v_0\,X}{k(1-X)}",
     run=cstr_volume_1, reference=_ref_cstr_volume_1,
     wrong=lambda v0, k, X: (lambda tau: _out(V=v0 * tau, tau=tau))(-math.log1p(-X) / k),
     rand=lambda rng: {"v0": _lu(rng, 1e-4, 1.0), "k": _lu(rng, 1e-5, 1.0), "X": rng.uniform(0.05, 0.98)},
     phrases=("First-order CSTR volume", "CSTR volume first order"),
     examples=[_ex({"v0": 0.01, "k": 0.001, "X": 0.9}), _ex({"v0": 0.002, "k": 0.05, "X": 0.5})], level="beginner",
     extra_q=[{"q": "First-order CSTR volume: v0 = 36 m^3/h, k = 0.06 1/min, X = 90%", "like": 0}])


def cstr_conv_1(v0, V, k):
    _pos("v0, V and k", v0, V, k)
    d = k * V / v0
    return _out(X=d / (1.0 + d))


def cstr_conv_2(v0, V, k, CA0):
    _pos("v0, V, k and CA0", v0, V, k, CA0)
    d = k * V / v0 * CA0
    c = 2.0 / (1.0 + math.sqrt(1.0 + 4.0 * d))
    return _out(X=1.0 - c, CA=CA0 * c)


def _startup(da, n, theta=60.0):
    """Reference helper: a CSTR started full of feed, c = C/C_A0 integrated in theta = t/tau to steady state."""
    return _ivp(lambda s, y: [1.0 - y[0] - da * y[0] ** n], theta, [1.0], atol=[1e-17])[0]


_add("chemeng_cstr_conversion_first_order",
     name="CSTR conversion for a volume (first order)", topic="reactor design / continuous stirred tanks",
     purpose="Steady-state conversion of a CSTR of volume V for a first-order liquid-phase reaction.",
     inputs=[_V0, _phys("V", "volume", "reactor volume"), _K1()], outputs=[_o("X", "number", "conversion of A")], primary="X",
     conditions=["steady state, perfect mixing, constant density, -r_A = k C_A"],
     method="X = k tau/(1 + k tau), tau = V/v0 (the Damkohler number k tau)", latex=r"X = \frac{k\tau}{1 + k\tau},\ \tau = V/v_0",
     run=cstr_conv_1, reference=lambda v0, V, k: {"X": 1.0 - _startup(k * V / v0, 1)},
     wrong=lambda v0, V, k: _out(X=-math.expm1(-k * V / v0)),
     rand=lambda rng: (lambda v0, k: {"v0": v0, "k": k, "V": v0 * _lu(rng, 0.05, 30.0) / k})(_lu(rng, 1e-4, 1.0), _lu(rng, 1e-5, 1.0)),
     phrases=("First-order CSTR conversion", "CSTR conversion first order"),
     examples=[_ex({"v0": 0.01, "V": 20.0, "k": 0.001}), _ex({"v0": 0.002, "V": 0.1, "k": 0.05})], level="beginner")

_add("chemeng_cstr_conversion_second_order",
     name="CSTR conversion for a volume (second order)", topic="reactor design / continuous stirred tanks",
     purpose="Steady-state conversion and exit concentration of a CSTR of volume V when -r_A = k C_A^2.",
     inputs=[_V0, _phys("V", "volume", "reactor volume"), _K2(), _CA0],
     outputs=[_o("X", "number", "conversion of A"), _o("CA", "molar_concentration", "exit concentration of A")], primary="X",
     conditions=["steady state, perfect mixing, constant density"],
     method="balance C_A0 - C_A = k tau C_A^2 solved for C_A (positive root, cancellation-free form)",
     latex=r"\frac{C_A}{C_{A0}} = \frac{2}{1 + \sqrt{1 + 4\,k\tau C_{A0}}}",
     run=cstr_conv_2, reference=lambda v0, V, k, CA0: (lambda c: {"X": 1.0 - c, "CA": CA0 * c})(_startup(k * V / v0 * CA0, 2)),
     wrong=lambda v0, V, k, CA0: (lambda d: _out(X=d / (1 + d), CA=CA0 / (1 + d)))(k * V / v0 * CA0),
     rand=lambda rng: (lambda v0, k, C: {"v0": v0, "k": k, "CA0": C, "V": v0 * _lu(rng, 0.05, 30.0) / (k * C)})(
         _lu(rng, 1e-4, 1.0), _lu(rng, 1e-7, 1e-2), _lu(rng, 10, 5000)),
     phrases=("Second-order CSTR conversion", "CSTR conversion second order"),
     examples=[_ex({"v0": 0.01, "V": 2.0, "k": 1e-5, "CA0": 1000.0}), _ex({"v0": 0.001, "V": 1.0, "k": 1e-6, "CA0": 500.0})],
     level="intermediate")


def pfr_volume_1(v0, k, X):
    _pos("v0 and k", v0, k)
    _conv_ok(X)
    return _out(V=-v0 * math.log1p(-X) / k)


def pfr_volume_2(v0, k, CA0, X):
    _pos("v0, k and CA0", v0, k, CA0)
    _conv_ok(X)
    return _out(V=v0 * X / (k * CA0 * (1.0 - X)))


_add("chemeng_pfr_volume_first_order",
     name="PFR volume for a conversion (first order, liquid)", topic="reactor design / plug-flow reactors",
     purpose="Volume of an isothermal plug-flow reactor that reaches conversion X for a first-order constant-density reaction.",
     inputs=[_V0, _K1(), _XIN], outputs=[_o("V", "volume", "reactor volume")], primary="V",
     conditions=["0 <= X < 1", "plug flow, isothermal, constant density"],
     method="V = F_A0 * integral of dX/(-r_A) = (v0/k) ln(1/(1 - X))", latex=r"V = \frac{v_0}{k}\ln\frac{1}{1-X}",
     run=pfr_volume_1, reference=lambda v0, k, X: {"V": v0 * _quad(lambda x: 1.0 / (k * (1.0 - x)), 0.0, X)},
     wrong=lambda v0, k, X: _out(V=v0 * X / (k * (1.0 - X))),
     rand=lambda rng: {"v0": _lu(rng, 1e-4, 1.0), "k": _lu(rng, 1e-5, 1.0), "X": rng.uniform(0.05, 0.995)},
     phrases=("First-order PFR volume", "PFR volume first order", "Plug-flow reactor volume first order"),
     examples=[_ex({"v0": 0.01, "k": 0.001, "X": 0.9}), _ex({"v0": 0.002, "k": 0.05, "X": 0.5})], level="beginner",
     extra_q=[{"q": "First-order PFR volume: v0 = 600 L/min, k = 0.06 1/min, X = 90%", "like": 0}])

_add("chemeng_pfr_volume_second_order",
     name="PFR volume for a conversion (second order, liquid)", topic="reactor design / plug-flow reactors",
     purpose="Volume of an isothermal plug-flow reactor that reaches conversion X when -r_A = k C_A^2 (constant density).",
     inputs=[_V0, _K2(), _CA0, _XIN], outputs=[_o("V", "volume", "reactor volume")], primary="V",
     conditions=["0 <= X < 1", "plug flow, isothermal, constant density"],
     method="V = (v0/(k C_A0)) X/(1 - X)", latex=r"V = \frac{v_0}{k C_{A0}}\,\frac{X}{1-X}",
     run=pfr_volume_2, reference=lambda v0, k, CA0, X: {"V": v0 * _quad(lambda x: 1.0 / (k * CA0 * (1.0 - x) ** 2), 0.0, X)},
     wrong=lambda v0, k, CA0, X: _out(V=v0 * X / (k * CA0 * (1.0 - X) ** 2)),
     rand=lambda rng: {"v0": _lu(rng, 1e-4, 1.0), "k": _lu(rng, 1e-7, 1e-2), "CA0": _lu(rng, 10, 5000), "X": rng.uniform(0.05, 0.99)},
     phrases=("Second-order PFR volume", "PFR volume second order", "Plug-flow reactor volume second order"),
     examples=[_ex({"v0": 0.01, "k": 1e-5, "CA0": 1000.0, "X": 0.8}), _ex({"v0": 0.001, "k": 1e-6, "CA0": 500.0, "X": 0.5})],
     level="intermediate")


def pfr_conv_1(v0, V, k):
    _pos("v0, V and k", v0, V, k)
    return _out(X=-math.expm1(-k * V / v0))


def pfr_conv_2(v0, V, k, CA0):
    _pos("v0, V, k and CA0", v0, V, k, CA0)
    d = k * V / v0 * CA0
    return _out(X=d / (1.0 + d), CA=CA0 / (1.0 + d))


def _plug(da, n):
    """Reference helper: c = C/C_A0 integrated along a plug-flow reactor in the scaled length z = V'/V."""
    return _ivp(lambda s, y: [-da * y[0] ** n], 1.0, [1.0], atol=[1e-17])[0]


_add("chemeng_pfr_conversion_first_order",
     name="PFR conversion for a volume (first order, liquid)", topic="reactor design / plug-flow reactors",
     purpose="Conversion at the exit of an isothermal plug-flow reactor of volume V for a first-order constant-density reaction.",
     inputs=[_V0, _phys("V", "volume", "reactor volume"), _K1()], outputs=[_o("X", "number", "conversion of A")], primary="X",
     conditions=["plug flow, isothermal, constant density"], method="X = 1 - exp(-k tau), tau = V/v0", latex=r"X = 1 - e^{-k\tau}",
     run=pfr_conv_1, reference=lambda v0, V, k: {"X": 1.0 - _plug(k * V / v0, 1)},
     wrong=lambda v0, V, k: (lambda d: _out(X=d / (1 + d)))(k * V / v0),
     rand=lambda rng: (lambda v0, k: {"v0": v0, "k": k, "V": v0 * _lu(rng, 0.05, 6.0) / k})(_lu(rng, 1e-4, 1.0), _lu(rng, 1e-5, 1.0)),
     phrases=("First-order PFR conversion", "PFR conversion first order", "Plug-flow reactor conversion first order"),
     examples=[_ex({"v0": 0.01, "V": 20.0, "k": 0.001}), _ex({"v0": 0.002, "V": 0.1, "k": 0.05})], level="beginner")

_add("chemeng_pfr_conversion_second_order",
     name="PFR conversion for a volume (second order, liquid)", topic="reactor design / plug-flow reactors",
     purpose="Exit conversion and concentration of an isothermal plug-flow reactor of volume V when -r_A = k C_A^2 (constant density).",
     inputs=[_V0, _phys("V", "volume", "reactor volume"), _K2(), _CA0],
     outputs=[_o("X", "number", "conversion of A"), _o("CA", "molar_concentration", "exit concentration of A")], primary="X",
     conditions=["plug flow, isothermal, constant density"], method="1/C_A = 1/C_A0 + k tau", latex=r"X = \frac{k\tau C_{A0}}{1 + k\tau C_{A0}}",
     run=pfr_conv_2, reference=lambda v0, V, k, CA0: (lambda c: {"X": 1.0 - c, "CA": CA0 * c})(_plug(k * V / v0 * CA0, 2)),
     wrong=lambda v0, V, k, CA0: (lambda d: _out(X=-math.expm1(-d), CA=CA0 * math.exp(-d)))(k * V / v0 * CA0),
     rand=lambda rng: (lambda v0, k, C: {"v0": v0, "k": k, "CA0": C, "V": v0 * _lu(rng, 0.05, 20.0) / (k * C)})(
         _lu(rng, 1e-4, 1.0), _lu(rng, 1e-7, 1e-2), _lu(rng, 10, 5000)),
     phrases=("Second-order PFR conversion", "PFR conversion second order", "Plug-flow reactor conversion second order"),
     examples=[_ex({"v0": 0.01, "V": 2.0, "k": 1e-5, "CA0": 1000.0}), _ex({"v0": 0.001, "V": 1.0, "k": 1e-6, "CA0": 500.0})],
     level="intermediate")


def _eps_ok(eps, X):
    _fin(eps)
    _need(-1 < eps <= 10 and 1 + eps * X > 0, "the volume-change factor eps must exceed -1")


def pfr_gas_1(FA0, CA0, k, eps, X):
    _pos("FA0, CA0 and k", FA0, CA0, k)
    _conv_ok(X)
    _eps_ok(eps, X)
    return _out(V=FA0 / (k * CA0) * (-(1.0 + eps) * math.log1p(-X) - eps * X))


def pfr_gas_2(FA0, CA0, k, eps, X):
    _pos("FA0, CA0 and k", FA0, CA0, k)
    _conv_ok(X)
    _eps_ok(eps, X)
    return _out(V=FA0 / (k * CA0 ** 2) * (2 * eps * (1 + eps) * math.log1p(-X) + eps ** 2 * X + (1 + eps) ** 2 * X / (1 - X)))


_GAS_IN = [_phys("FA0", "molar_flow_rate", "molar feed flow of A"), _phys("CA0", "molar_concentration", "concentration of A at the inlet")]
_EPS = _numin("eps", "volume-change factor eps = y_A0 * (change in moles per mole of A); e.g. A -> 2B with pure A: eps = 1")

_add("chemeng_pfr_gas_volume_first_order",
     name="Gas-phase PFR volume with volume change (first order)", topic="reactor design / plug-flow reactors",
     purpose="Volume of an isothermal, isobaric gas-phase plug-flow reactor for conversion X of a first-order reaction whose mole number changes (factor eps).",
     inputs=_GAS_IN + [_K1(), _EPS, _XIN], outputs=[_o("V", "volume", "reactor volume")], primary="V",
     conditions=["isothermal, isobaric ideal gas", "eps > -1", "0 <= X < 1"],
     method="V = F_A0 * integral of (1 + eps X)/(k C_A0 (1 - X)) dX = (F_A0/(k C_A0)) [(1 + eps) ln(1/(1 - X)) - eps X]",
     latex=r"V = \frac{F_{A0}}{kC_{A0}}\left[(1+\varepsilon)\ln\frac{1}{1-X} - \varepsilon X\right]",
     run=pfr_gas_1, reference=lambda FA0, CA0, k, eps, X: {"V": FA0 * _quad(lambda x: (1 + eps * x) / (k * CA0 * (1 - x)), 0.0, X)},
     wrong=lambda FA0, CA0, k, eps, X: _out(V=-FA0 / (k * CA0) * math.log1p(-X)),
     rand=lambda rng: {"FA0": _lu(rng, 0.01, 100), "CA0": _lu(rng, 1, 500), "k": _lu(rng, 1e-4, 1.0),
                       "eps": rng.choice([rng.uniform(-0.6, -0.1), rng.uniform(0.1, 2.0)]), "X": rng.uniform(0.05, 0.98)},
     phrases=("First-order gas-phase PFR volume", "Gas PFR volume first order"),
     examples=[_ex({"FA0": 10.0, "CA0": 40.0, "k": 0.05, "eps": 1.0, "X": 0.8}), _ex({"FA0": 2.0, "CA0": 20.0, "k": 0.01, "eps": -0.5, "X": 0.6})],
     level="advanced")

_add("chemeng_pfr_gas_volume_second_order",
     name="Gas-phase PFR volume with volume change (second order)", topic="reactor design / plug-flow reactors",
     purpose="Volume of an isothermal, isobaric gas-phase plug-flow reactor for conversion X when -r_A = k C_A^2 and the mole number changes (factor eps).",
     inputs=_GAS_IN + [_K2(), _EPS, _XIN], outputs=[_o("V", "volume", "reactor volume")], primary="V",
     conditions=["isothermal, isobaric ideal gas", "eps > -1", "0 <= X < 1"],
     method="V = (F_A0/(k C_A0^2)) [2 eps (1 + eps) ln(1 - X) + eps^2 X + (1 + eps)^2 X/(1 - X)]",
     latex=r"V = \frac{F_{A0}}{kC_{A0}^2}\left[2\varepsilon(1+\varepsilon)\ln(1-X) + \varepsilon^2X + \frac{(1+\varepsilon)^2X}{1-X}\right]",
     run=pfr_gas_2, reference=lambda FA0, CA0, k, eps, X: {"V": FA0 * _quad(lambda x: (1 + eps * x) ** 2 / (k * CA0 ** 2 * (1 - x) ** 2), 0.0, X)},
     wrong=lambda FA0, CA0, k, eps, X: _out(V=FA0 / (k * CA0 ** 2) * X / (1 - X)),
     rand=lambda rng: {"FA0": _lu(rng, 0.01, 100), "CA0": _lu(rng, 1, 500), "k": _lu(rng, 1e-6, 1e-2),
                       "eps": rng.choice([rng.uniform(-0.6, -0.1), rng.uniform(0.1, 2.0)]), "X": rng.uniform(0.05, 0.98)},
     phrases=("Second-order gas-phase PFR volume", "Gas PFR volume second order"),
     examples=[_ex({"FA0": 10.0, "CA0": 40.0, "k": 0.001, "eps": 1.0, "X": 0.8}), _ex({"FA0": 2.0, "CA0": 20.0, "k": 0.0005, "eps": -0.5, "X": 0.6})],
     level="expert")


def cstrs_series_1(v0, V, k, N):
    _pos("v0, V and k", v0, V, k)
    _need(isinstance(N, int) and 1 <= N <= 50, "N must be a whole number of tanks from 1 to 50")
    d = k * V / v0
    return _out(X=-math.expm1(-N * math.log1p(d)))


def cstrs_series_2(v0, V, k, CA0, N):
    _pos("v0, V, k and CA0", v0, V, k, CA0)
    _need(isinstance(N, int) and 1 <= N <= 50, "N must be a whole number of tanks from 1 to 50")
    d = k * V / v0 * CA0
    c = 1.0
    for _ in range(N):
        c = 2.0 * c / (1.0 + math.sqrt(1.0 + 4.0 * d * c))
    return _out(X=1.0 - c, CA=CA0 * c)


def _train(da, n, N, theta=80.0):
    """Reference helper: N equal CSTRs started full of feed, integrated in theta = t/tau_i to steady state."""
    def f(s, y):
        out, prev = [], 1.0
        for c in y:
            out.append(prev - c - da * c ** n)
            prev = c
        return out
    return _ivp(f, theta, [1.0] * N, atol=[1e-17] * N)[-1]


_TANKS = _intin("N", "number of equal tanks in series", 1, 50)
_add("chemeng_cstrs_in_series_first_order",
     name="Equal CSTRs in series: conversion (first order)", topic="reactor design / continuous stirred tanks",
     purpose="Conversion leaving N equal CSTRs in series, each of volume V, for a first-order liquid-phase reaction.",
     inputs=[_V0, _phys("V", "volume", "volume of EACH tank"), _K1(), _TANKS], outputs=[_o("X", "number", "overall conversion of A")],
     primary="X", conditions=["equal tanks", "steady state, perfect mixing in each tank, constant density"],
     method="each tank divides the concentration by (1 + k tau_i): X = 1 - (1 + k V/v0)^(-N)", latex=r"X = 1 - (1 + k\tau_i)^{-N}",
     run=cstrs_series_1, reference=lambda v0, V, k, N: {"X": 1.0 - _train(k * V / v0, 1, N)},
     wrong=lambda v0, V, k, N: _out(X=-math.expm1(-N * k * V / v0)),
     rand=lambda rng: (lambda v0, k: {"v0": v0, "k": k, "V": v0 * _lu(rng, 0.05, 5.0) / k, "N": rng.randint(2, 8)})(_lu(rng, 1e-4, 1.0), _lu(rng, 1e-5, 1.0)),
     phrases=("First-order CSTRs in series", "CSTRs in series first order", "Tanks in series conversion first order"),
     examples=[_ex({"v0": 0.01, "V": 5.0, "k": 0.001, "N": 3}), _ex({"v0": 0.002, "V": 0.05, "k": 0.05, "N": 2})], level="intermediate",
     extra_q=[{"q": "First-order CSTRs in series: v0 = 0.01 m^3/s, V = 5 m^3, k = 0.001 1/s, N = 2.5", "outcome": "INVALID_INPUT"}])

_add("chemeng_cstrs_in_series_second_order",
     name="Equal CSTRs in series: conversion (second order)", topic="reactor design / continuous stirred tanks",
     purpose="Conversion leaving N equal CSTRs in series, each of volume V, when -r_A = k C_A^2.",
     inputs=[_V0, _phys("V", "volume", "volume of EACH tank"), _K2(), _CA0, _TANKS],
     outputs=[_o("X", "number", "overall conversion of A"), _o("CA", "molar_concentration", "exit concentration of A")],
     primary="X", conditions=["equal tanks", "steady state, perfect mixing, constant density"],
     method="tank by tank: C_i = 2 C_(i-1)/(1 + sqrt(1 + 4 k tau_i C_(i-1)))",
     latex=r"C_i = \frac{2C_{i-1}}{1 + \sqrt{1 + 4k\tau_i C_{i-1}}}",
     run=cstrs_series_2, reference=lambda v0, V, k, CA0, N: (lambda c: {"X": 1.0 - c, "CA": CA0 * c})(_train(k * V / v0 * CA0, 2, N)),
     wrong=lambda v0, V, k, CA0, N: (lambda d: _out(X=1 - 1 / (1 + N * d), CA=CA0 / (1 + N * d)))(k * V / v0 * CA0),
     rand=lambda rng: (lambda v0, k, C: {"v0": v0, "k": k, "CA0": C, "V": v0 * _lu(rng, 0.05, 5.0) / (k * C), "N": rng.randint(2, 8)})(
         _lu(rng, 1e-4, 1.0), _lu(rng, 1e-7, 1e-2), _lu(rng, 10, 5000)),
     phrases=("Second-order CSTRs in series", "CSTRs in series second order", "Tanks in series conversion second order"),
     examples=[_ex({"v0": 0.01, "V": 1.0, "k": 1e-5, "CA0": 1000.0, "N": 3}), _ex({"v0": 0.001, "V": 0.5, "k": 1e-6, "CA0": 500.0, "N": 4})],
     level="advanced")


def recycle_reactor_1(v0, V, k, R):
    _pos("v0, V and k", v0, V, k)
    _fin(R)
    _need(0 <= R <= 1e4, "the recycle ratio R must be zero or positive")
    a = k * V / v0 / (R + 1.0)
    u = 1.0 / ((R + 1.0) * math.exp(a) - R)
    return _out(X=1.0 - u)


def _ref_recycle_reactor_1(v0, V, k, R):
    """The loop solved as a flowsheet: guess the exit concentration, mix it with the feed, integrate the plug flow
    along the reactor (ODE), and adjust the guess (brentq) until the exit matches."""
    def miss(u):
        c_in = (1.0 + R * u) / (R + 1.0)
        return _ivp(lambda s, y: [-k * V / (v0 * (R + 1.0)) * y[0]], 1.0, [c_in], atol=[1e-18])[0] - u
    u = _brentq(miss, 1e-300, 1.0)
    return {"X": 1.0 - u}


_add("chemeng_recycle_reactor_first_order",
     name="Plug-flow reactor with recycle: conversion (first order)", topic="reactor design / recycle reactors",
     purpose="Overall conversion of a plug-flow reactor with recycle ratio R (recycled flow / product flow) for a first-order constant-density reaction.",
     inputs=[_phys("v0", "flow_rate", "fresh volumetric feed flow"), _phys("V", "volume", "reactor volume"), _K1(),
             _numin("R", "recycle ratio: recycled flow divided by the flow leaving the system")],
     outputs=[_o("X", "number", "overall conversion of A")], primary="X",
     conditions=["R >= 0 (R = 0 is a plain PFR; large R approaches a CSTR)", "isothermal, constant density"],
     method="k tau/(R + 1) = ln[(1 + R(1 - X))/((R + 1)(1 - X))], tau = V/v0, solved for X",
     latex=r"\frac{k\tau}{R+1} = \ln\frac{1 + R(1-X)}{(R+1)(1-X)}",
     run=recycle_reactor_1, reference=_ref_recycle_reactor_1,
     wrong=lambda v0, V, k, R: (lambda d: _out(X=d / (1.0 + d)))(k * V / v0),
     rand=lambda rng: (lambda v0, k: {"v0": v0, "k": k, "V": v0 * _lu(rng, 0.1, 6.0) / k, "R": rng.choice([0.0, rng.uniform(0.2, 10.0)])})(
         _lu(rng, 1e-4, 1.0), _lu(rng, 1e-5, 1.0)),
     phrases=("Recycle reactor conversion", "First-order recycle reactor", "PFR with recycle conversion"),
     examples=[_ex({"v0": 0.01, "V": 20.0, "k": 0.001, "R": 2.0}), _ex({"v0": 0.002, "V": 0.1, "k": 0.05, "R": 0.0})], level="expert")


def reversible_batch(k1, k2, CA0, CB0, t):
    _pos("k1 and k2", k1, k2)
    _pos("CA0", CA0)
    _nonneg("CB0 and t", CB0, t)
    tot = CA0 + CB0
    CAe = k2 * tot / (k1 + k2)
    CA = CAe + (CA0 - CAe) * math.exp(-(k1 + k2) * t)
    return _out(CA=CA, CB=tot - CA, X=1.0 - CA / CA0, X_eq=1.0 - CAe / CA0)


def _ref_reversible_batch(k1, k2, CA0, CB0, t):
    f = lambda s, y: [-k1 * y[0] + k2 * y[1], k1 * y[0] - k2 * y[1]]
    CA, CB = _ivp(f, t, [CA0, CB0])
    CAe, _ = _ivp(f, 60.0 / (k1 + k2), [CA0, CB0])
    return {"CA": CA, "CB": CB, "X": 1.0 - CA / CA0, "X_eq": 1.0 - CAe / CA0}


_add("chemeng_reversible_first_order_batch",
     name="Reversible first-order reaction A <-> B in a batch reactor", topic="reactor design / reversible reactions",
     purpose="Concentrations and conversion after time t, and the equilibrium conversion, for A <-> B with first-order forward and reverse steps.",
     inputs=[_phys("k1", "first_order_rate_constant", "forward rate constant (A -> B)"),
             _phys("k2", "first_order_rate_constant", "reverse rate constant (B -> A)"),
             _CA0, _phys("CB0", "molar_concentration", "initial concentration of B"), _phys("t", "time", "reaction time")],
     outputs=[_o("CA", "molar_concentration", "concentration of A at t"), _o("CB", "molar_concentration", "concentration of B at t"),
              _o("X", "number", "conversion of A at t"), _o("X_eq", "number", "equilibrium conversion of A")], primary="X",
     conditions=["isothermal, constant volume"], method="C_A = C_Ae + (C_A0 - C_Ae) exp(-(k1 + k2) t), C_Ae = k2 (C_A0 + C_B0)/(k1 + k2)",
     latex=r"C_A = C_{Ae} + (C_{A0} - C_{Ae})e^{-(k_1+k_2)t},\ C_{Ae} = \frac{k_2(C_{A0}+C_{B0})}{k_1+k_2}",
     run=reversible_batch, reference=_ref_reversible_batch,
     wrong=lambda k1, k2, CA0, CB0, t: (lambda CA: _out(CA=CA, CB=CA0 + CB0 - CA, X=1 - CA / CA0, X_eq=1.0))(CA0 * math.exp(-k1 * t)),
     rand=lambda rng: (lambda k1, k2: {"k1": k1, "k2": k2, "CA0": _lu(rng, 10, 5000), "CB0": rng.choice([0.0, _lu(rng, 1, 500)]),
                                       "t": _lu(rng, 0.05, 3.0) / (k1 + k2)})(_lu(rng, 1e-5, 0.1), _lu(rng, 1e-5, 0.1)),
     phrases=("Reversible first-order batch reaction", "Reversible reaction batch"),
     examples=[_ex({"k1": 0.002, "k2": 0.001, "CA0": 1000.0, "CB0": 0.0, "t": 300.0}), _ex({"k1": 0.01, "k2": 0.04, "CA0": 500.0, "CB0": 100.0, "t": 30.0})],
     level="intermediate")


def series_max(k1, k2, CA0):
    _pos("k1, k2 and CA0", k1, k2, CA0)
    if abs(k1 - k2) <= 1e-9 * max(k1, k2):
        k = 0.5 * (k1 + k2)
        t, CB = 1.0 / k, CA0 / math.e
    else:
        t = math.log(k2 / k1) / (k2 - k1)
        CB = CA0 * (k1 / k2) ** (k2 / (k2 - k1))
    CA = CA0 * math.exp(-k1 * t)
    return _out(t_max=t, CB_max=CB, CA=CA, CC=CA0 - CA - CB)


def _ref_series_max(k1, k2, CA0):
    """ODE integration of A -> B -> C (dense output), the maximum where dC_B/dt = k1 C_A - k2 C_B = 0 (brentq)."""
    from scipy.integrate import solve_ivp
    T = 10.0 / min(k1, k2)
    sol = solve_ivp(lambda s, y: [-k1 * y[0], k1 * y[0] - k2 * y[1]], (0.0, T), [CA0, 0.0], method="DOP853",
                    rtol=1e-13, atol=[1e-16 * CA0, 1e-16 * CA0], dense_output=True)
    g = lambda t: (lambda y: k1 * y[0] - k2 * y[1])(sol.sol(t))
    t = _brentq(g, 0.0, T)
    CA, CB = (float(v) for v in sol.sol(t))
    return {"t_max": t, "CB_max": CB, "CA": CA, "CC": CA0 - CA - CB}


_add("chemeng_series_reaction_maximum",
     name="Series reactions A -> B -> C: maximum of the intermediate (batch or PFR)", topic="reactor design / multiple reactions",
     purpose="Time (or plug-flow space time) at which the intermediate B of first-order series reactions A -> B -> C peaks, starting from pure A, and the concentrations there.",
     inputs=[_phys("k1", "first_order_rate_constant", "rate constant of A -> B"),
             _phys("k2", "first_order_rate_constant", "rate constant of B -> C"), _CA0],
     outputs=[_o("t_max", "time", "time (or space time) of the maximum of B"), _o("CB_max", "molar_concentration", "maximum concentration of B"),
              _o("CA", "molar_concentration", "concentration of A at the maximum"), _o("CC", "molar_concentration", "concentration of C at the maximum")],
     primary="t_max", conditions=["starting from pure A (no B or C)", "isothermal, constant volume, first-order steps"],
     method="t_max = ln(k2/k1)/(k2 - k1), C_B,max = C_A0 (k1/k2)^(k2/(k2 - k1)) (k1 = k2: t = 1/k, C_B = C_A0/e)",
     latex=r"t_{max} = \frac{\ln(k_2/k_1)}{k_2 - k_1},\quad C_{B,max} = C_{A0}\left(\frac{k_1}{k_2}\right)^{k_2/(k_2-k_1)}",
     run=series_max, reference=_ref_series_max,
     wrong=lambda k1, k2, CA0: series_max(k1, k2, CA0) | _out(CB_max=CA0 * (k1 / k2) ** (k1 / (k2 - k1)) if k1 != k2 else CA0),
     rand=lambda rng: {"k1": _lu(rng, 1e-4, 0.1), "k2": _lu(rng, 1e-4, 0.1), "CA0": _lu(rng, 10, 5000)},
     phrases=("Series reaction maximum", "Maximum intermediate in series reactions"),
     examples=[_ex({"k1": 0.002, "k2": 0.001, "CA0": 1000.0}), _ex({"k1": 0.01, "k2": 0.05, "CA0": 200.0})], level="advanced")


def rtd_moments(t, C, t_unit):
    _need(len(t) == len(C) and len(t) >= 3, "t and C need the same number of points (at least 3)")
    f = {"s": 1.0, "min": 60.0, "h": 3600.0}[t_unit]
    ts = [x * f for x in t]
    _need(all(b > a for a, b in zip(ts, ts[1:])), "times must increase")
    _need(all(c >= 0 for c in C) and any(c > 0 for c in C), "tracer concentrations must be zero or positive, not all zero")
    trap = lambda y: sum((ts[i + 1] - ts[i]) * (y[i] + y[i + 1]) / 2.0 for i in range(len(ts) - 1))
    area = trap(C)
    tm = trap([a * b for a, b in zip(ts, C)]) / area
    var = trap([(a - tm) ** 2 * b for a, b in zip(ts, C)]) / area
    _need(var > 0, "the tracer response has no spread")
    return _out(t_mean=tm, variance=var, N_tanks=tm * tm / var)


def _ref_rtd_moments(t, C, t_unit):
    import numpy as np
    from scipy.integrate import trapezoid
    ts = np.array(t, float) * {"s": 1.0, "min": 60.0, "h": 3600.0}[t_unit]
    c = np.array(C, float)
    area = trapezoid(c, ts)
    tm = trapezoid(ts * c, ts) / area
    var = trapezoid((ts - tm) ** 2 * c, ts) / area
    return {"t_mean": float(tm), "variance": float(var), "N_tanks": float(tm * tm / var)}


def _rand_rtd(rng):
    n = rng.randint(8, 25)
    N = rng.uniform(1.0, 8.0)
    tau = _lu(rng, 1, 100)
    ts = sorted(rng.uniform(0, 3.5 * tau) for _ in range(n))
    ts[0] = 0.0
    C = [_sig(100.0 * (x / tau) ** (N - 1) * math.exp(-N * x / tau) + rng.uniform(0, 0.3)) for x in ts]
    return {"t": [_sig(x) for x in ts], "C": C, "t_unit": rng.choice(["s", "min", "h"])}


_add("chemeng_rtd_moments",
     name="Residence-time distribution: mean, variance and tanks-in-series number from tracer data", topic="reactor design / residence-time distribution",
     purpose="Mean residence time, variance and the equivalent number of tanks in series from a pulse-tracer response C(t) (trapezoidal rule, uneven spacing allowed).",
     inputs=[_vecin("t", "sample times, in the unit t_unit", 3, 200), _vecin("C", "tracer concentrations at those times (any unit)", 3, 200),
             _choice("t_unit", ["s", "min", "h"], "unit of the times: s, min or h")],
     outputs=[_o("t_mean", "time", "mean residence time"), _o("variance", "time_squared", "variance of the RTD"),
              _o("N_tanks", "number", "equivalent number of equal tanks in series, t_mean^2/variance")], primary="t_mean",
     conditions=["pulse tracer, closed vessel", "times increasing, concentrations >= 0"],
     method="E(t) = C/integral C dt; t_mean = integral t E dt; variance = integral (t - t_mean)^2 E dt; trapezoidal rule",
     latex=r"\bar t = \frac{\int t\,C\,dt}{\int C\,dt},\quad \sigma^2 = \frac{\int (t-\bar t)^2 C\,dt}{\int C\,dt}",
     run=rtd_moments, reference=_ref_rtd_moments,
     wrong=lambda t, C, t_unit: (lambda r: r | _out(variance=r["variance"] + r["t_mean"] ** 2, N_tanks=r["t_mean"] ** 2 / (r["variance"] + r["t_mean"] ** 2)))(rtd_moments(t, C, t_unit)),
     rand=_rand_rtd,
     phrases=("RTD moments", "Residence time distribution moments", "Tracer response moments"),
     examples=[_ex({"t": [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 12.0, 14.0],
                    "C": [0.0, 1.0, 5.0, 8.0, 10.0, 8.0, 6.0, 4.0, 3.0, 2.2, 1.5, 0.6, 0.0], "t_unit": "min"}),
               _ex({"t": [0.0, 5.0, 10.0, 20.0, 30.0, 45.0, 60.0], "C": [0.0, 4.0, 6.0, 5.0, 3.0, 1.0, 0.2], "t_unit": "s"})],
     level="advanced",
     extra_q=[{"q": "RTD moments: t = [0, 1, 2], C = [0, 1, 0]", "outcome": "NEEDS_INPUT"}])


# ================================================================================================ SEPARATIONS
# --- Antoine vapour pressure with a stated convention -----------------------------------------------------------
_ANT_UNITS = {"mmHg_degC": (133.322387415, 273.15), "kPa_degC": (1000.0, 273.15), "bar_degC": (1e5, 273.15),
              "mmHg_K": (133.322387415, 0.0), "Pa_K": (1.0, 0.0), "kPa_K": (1000.0, 0.0), "bar_K": (1e5, 0.0)}
_ANT_BASE = {"log10": 10.0, "ln": math.e}
_ANT_IN = [_choice("units", list(_ANT_UNITS), "units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K"),
           _choice("base", list(_ANT_BASE), "logarithm of the Antoine equation: log10 or ln")]


def _psat(row, units, base, T):
    A, B, C = row
    fac, off = _ANT_UNITS[units]
    d = T - off + C
    _need(d > 0, "the temperature is below the singular point of the Antoine equation (T + C <= 0)")
    return fac * _ANT_BASE[base] ** (A - B / d)


def _tsat(row, units, base, P):
    A, B, C = row
    fac, off = _ANT_UNITS[units]
    lg = math.log(P / fac) / math.log(_ANT_BASE[base])
    _need(A - lg > 0, "this pressure lies outside the Antoine equation's range")
    return B / (A - lg) - C + off


def _antoine_rows(ant, n=None):
    _need(all(len(r) == 3 for r in ant), "give three Antoine coefficients (A, B, C) per component")
    if n is not None:
        _need(len(ant) == n, "give one row of Antoine coefficients per component")
    for r in ant:
        _fin(*r)
        _need(r[1] > 0, "the Antoine coefficient B must be positive")
    return ant


def _ref_psat_funcs(ant, units, base):
    """chemicals' Antoine function (T in K, P in Pa) with the coefficients carried to that convention."""
    from chemicals.vapor_pressure import Antoine
    fac, off = _ANT_UNITS[units]
    b = _ANT_BASE[base]
    return [(lambda A, B, C: (lambda T: Antoine(T, A, B, C, base=b)))(r[0] + math.log(fac) / math.log(b), r[1], r[2] - off) for r in ant]


def antoine_P(A, B, C, units, base, T):
    _antoine_rows([[A, B, C]])
    _pos("T", T)
    return _out(Psat=_psat((A, B, C), units, base, T))


def antoine_T(A, B, C, units, base, P):
    _antoine_rows([[A, B, C]])
    _pos("P", P)
    return _out(Tsat=_tsat((A, B, C), units, base, P))


_ANT_ABC = [_numin("A", "Antoine coefficient A"), _numin("B", "Antoine coefficient B"), _numin("C", "Antoine coefficient C")]
_WATER = {"A": 8.07131, "B": 1730.63, "C": 233.426, "units": "mmHg_degC", "base": "log10"}
_BENZENE_NIST = {"A": 4.72583, "B": 1660.652, "C": -1.461, "units": "bar_K", "base": "log10"}


def _rand_antoine(rng):
    units = rng.choice(list(_ANT_UNITS))
    base = rng.choice(list(_ANT_BASE))
    fac, off = _ANT_UNITS[units]
    Tb = rng.uniform(250, 450)                                     # normal boiling point, K
    C_K = -rng.uniform(20, 70)                                     # C in kelvin form
    B = rng.uniform(1000, 2000) if base == "log10" else rng.uniform(2300, 4600)
    lb = math.log(_ANT_BASE[base])
    A = math.log(P_ATM / fac) / lb + B / (Tb + C_K)
    return {"A": _sig(A, 6), "B": _sig(B, 6), "C": _sig(C_K + off, 6), "units": units, "base": base}, Tb


_add("chemeng_antoine_vapour_pressure",
     name="Vapour pressure from the Antoine equation", topic="separations / vapour pressure",
     purpose="Vapour pressure of a pure liquid at temperature T from Antoine coefficients written in a stated convention (logarithm and units).",
     inputs=_ANT_ABC + _ANT_IN + [_phys("T", "temperature", "temperature")],
     outputs=[_o("Psat", "pressure", "vapour pressure")], primary="Psat",
     conditions=["the convention (units, base) is stated", "T within the range the coefficients were fitted for"],
     method="log P = A - B/(T + C) in the stated units and logarithm; P returned in Pa",
     latex=r"\log_{10} P^{sat} = A - \frac{B}{T + C}",
     run=antoine_P,
     reference=lambda A, B, C, units, base, T: {"Psat": _ref_psat_funcs([[A, B, C]], units, base)[0](T)},
     wrong=lambda A, B, C, units, base, T: _out(Psat=_ANT_BASE[base] ** (A - B / (T - _ANT_UNITS[units][1] + C))),
     rand=lambda rng: (lambda d, Tb: dict(d, T=Tb + rng.uniform(-40, 40)))(*_rand_antoine(rng)),
     phrases=("Antoine vapour pressure", "Antoine vapor pressure", "Vapour pressure from Antoine"),
     examples=[_ex(dict(_WATER, T=373.15)), _ex(dict(_BENZENE_NIST, T=350.0))], level="beginner",
     extra_q=[{"q": "Antoine vapour pressure: A = 8.07131, B = 1730.63, C = 233.426, units = mmHg_degC, base = log10, T = 100 degC", "like": 0},
              {"q": "Antoine vapour pressure: A = 8.07131, B = 1730.63, C = 233.426, T = 100 degC", "outcome": "NEEDS_INPUT"}])

_add("chemeng_antoine_boiling_temperature",
     name="Boiling (saturation) temperature from the Antoine equation", topic="separations / vapour pressure",
     purpose="Temperature at which a pure liquid boils at pressure P, from Antoine coefficients written in a stated convention.",
     inputs=_ANT_ABC + _ANT_IN + [_phys("P", "pressure", "pressure")],
     outputs=[_o("Tsat", "temperature", "saturation temperature")], primary="Tsat",
     conditions=["the convention (units, base) is stated", "P within the fitted range"],
     method="T = B/(A - log P) - C in the stated units; returned in K",
     latex=r"T^{sat} = \frac{B}{A - \log_{10} P} - C",
     run=antoine_T,
     reference=lambda A, B, C, units, base, P: (lambda f: {"Tsat": _brentq(lambda T: f(T) - P, -(C - _ANT_UNITS[units][1]) + 1e-6, 5000.0)})(
         _ref_psat_funcs([[A, B, C]], units, base)[0]),
     wrong=lambda A, B, C, units, base, P: _out(Tsat=B / (A - math.log(P / _ANT_UNITS[units][0]) / math.log(_ANT_BASE[base])) + C + _ANT_UNITS[units][1]),
     rand=lambda rng: (lambda d, Tb: dict(d, P=P_ATM * rng.uniform(0.1, 5.0)))(*_rand_antoine(rng)),
     phrases=("Antoine boiling temperature", "Boiling point from Antoine", "Saturation temperature from Antoine"),
     examples=[_ex(dict(_WATER, P=101325.0)), _ex(dict(_BENZENE_NIST, P=50000.0))], level="beginner",
     extra_q=[{"q": "Antoine boiling temperature: A = 8.07131, B = 1730.63, C = 233.426, units = mmHg_degC, base = log10, P = 1 atm", "like": 0},
              {"q": "Antoine boiling temperature: A = 8.07131, B = 1730.63, C = 233.426, units = mmHg, base = log10, P = 1 atm", "outcome": "INVALID_INPUT"}])


# --- Raoult's law: bubble and dew points, isothermal flash --------------------------------------------------------
def _comp(name, v):
    _need(2 <= len(v) <= 10, f"{name}: give 2 to 10 mole fractions")
    _fracs(name, v)


def bubble_P(x, ant, units, base, T):
    _comp("x", x)
    _antoine_rows(ant, len(x))
    _pos("T", T)
    ps = [_psat(r, units, base, T) for r in ant]
    P = sum(a * b for a, b in zip(x, ps))
    return _out(P=P, y=[a * b / P for a, b in zip(x, ps)])


def dew_P(y, ant, units, base, T):
    _comp("y", y)
    _antoine_rows(ant, len(y))
    _pos("T", T)
    ps = [_psat(r, units, base, T) for r in ant]
    P = 1.0 / sum(a / b for a, b in zip(y, ps))
    return _out(P=P, x=[a * P / b for a, b in zip(y, ps)])


def _t_bracket(ant, units, base, P):
    ts = [_tsat(r, units, base, P) for r in ant]
    return min(ts), max(ts)


def bubble_T(x, ant, units, base, P):
    _comp("x", x)
    _antoine_rows(ant, len(x))
    _pos("P", P)
    lo, hi = _t_bracket(ant, units, base, P)
    f = lambda T: sum(a * _psat(r, units, base, T) for a, r in zip(x, ant)) / P - 1.0
    T = lo if hi == lo else _bisect(f, lo, hi)
    ps = [_psat(r, units, base, T) for r in ant]
    return _out(T=T, y=[a * b / P for a, b in zip(x, ps)])


def dew_T(y, ant, units, base, P):
    _comp("y", y)
    _antoine_rows(ant, len(y))
    _pos("P", P)
    lo, hi = _t_bracket(ant, units, base, P)
    f = lambda T: 1.0 - sum(a * P / _psat(r, units, base, T) for a, r in zip(y, ant))
    T = lo if hi == lo else _bisect(f, lo, hi)
    ps = [_psat(r, units, base, T) for r in ant]
    return _out(T=T, x=[a * P / b for a, b in zip(y, ps)])


def _ref_flash_ideal(z, ant, units, base, **kw):
    from chemicals.flash_basic import flash_ideal
    funcs = _ref_psat_funcs(ant, units, base)
    P = kw.get("P", P_ATM)
    tcs = [1.6 * _tsat(r, units, base, P) for r in ant]          # starting guesses only
    return flash_ideal(list(z), funcs, Tcs=tcs, **kw)


def _ref_bubble_T(x, ant, units, base, P):
    """chemicals' ideal bubble-point pressure at T (flash_ideal, T and VF = 0), brentq on T for the stated P
    (chemicals' own P-VF bubble solver stops near 1e-7 relative)."""
    lo, hi = _t_bracket(ant, units, base, P)
    f = lambda T: _ref_flash_ideal(x, ant, units, base, T=T, VF=0)[1] - P
    T = lo if hi == lo else _brentq(f, lo, hi)
    r = _ref_flash_ideal(x, ant, units, base, T=T, VF=0)
    return {"T": T, "y": list(r[4])}


def _rand_mixture(rng, n=None):
    n = n or rng.choice([2, 2, 3, 4])
    units = rng.choice(list(_ANT_UNITS))
    base = rng.choice(list(_ANT_BASE))
    fac, off = _ANT_UNITS[units]
    lb = math.log(_ANT_BASE[base])
    ant = []
    for _ in range(n):
        Tb = rng.uniform(280, 420)
        C_K = -rng.uniform(20, 70)
        B = rng.uniform(1000, 2000) if base == "log10" else rng.uniform(2300, 4600)
        A = math.log(P_ATM / fac) / lb + B / (Tb + C_K)
        ant.append([_sig(A, 6), _sig(B, 6), _sig(C_K + off, 6)])
    z = _dirichlet(rng, n)
    return z, ant, units, base


_MIX_IN = [_matin("ant", "Antoine coefficients, one row (A, B, C) per component, in the stated convention")] + _ANT_IN
_BT = {"ant": [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], "units": "mmHg_degC", "base": "log10"}   # benzene, toluene

_add("chemeng_raoult_bubble_pressure",
     name="Bubble-point pressure (Raoult's law)", topic="separations / vapour-liquid equilibrium",
     purpose="Pressure at which a liquid of composition x starts to boil at temperature T, and the composition of the first bubble, for an ideal solution (Raoult's law, Antoine vapour pressures).",
     inputs=[_vecin("x", "liquid mole fractions", 2, 10)] + _MIX_IN + [_phys("T", "temperature", "temperature")],
     outputs=[_o("P", "pressure", "bubble-point pressure"), _o("y", "number", "vapour mole fractions of the first bubble")], primary="P",
     conditions=["ideal liquid and ideal gas (Raoult's law)", "fractions sum to 1"],
     method="P = sum x_i Psat_i(T); y_i = x_i Psat_i/P", latex=r"P_{bubble} = \sum_i x_i P_i^{sat}(T)",
     run=bubble_P, reference=lambda x, ant, units, base, T: (lambda r: {"P": r[1], "y": list(r[4])})(_ref_flash_ideal(x, ant, units, base, T=T, VF=0)),
     wrong=lambda x, ant, units, base, T: (lambda ps: _out(P=sum(ps) / len(ps), y=[1.0 / len(ps)] * len(ps)))([_psat(r, units, base, T) for r in ant]),
     rand=lambda rng: (lambda z, ant, u, b: {"x": z, "ant": ant, "units": u, "base": b, "T": rng.uniform(300, 400)})(*_rand_mixture(rng)),
     phrases=("Bubble-point pressure", "Bubble point pressure", "Raoult bubble pressure"),
     examples=[_ex(dict(_BT, x=[0.4, 0.6], T=368.15)), _ex(dict(_BT, x=[0.25, 0.75], T=380.0))], level="beginner")

_add("chemeng_raoult_dew_pressure",
     name="Dew-point pressure (Raoult's law)", topic="separations / vapour-liquid equilibrium",
     purpose="Pressure at which a vapour of composition y starts to condense at temperature T, and the composition of the first drop (Raoult's law, Antoine vapour pressures).",
     inputs=[_vecin("y", "vapour mole fractions", 2, 10)] + _MIX_IN + [_phys("T", "temperature", "temperature")],
     outputs=[_o("P", "pressure", "dew-point pressure"), _o("x", "number", "liquid mole fractions of the first drop")], primary="P",
     conditions=["ideal liquid and ideal gas", "fractions sum to 1"],
     method="1/P = sum y_i/Psat_i(T); x_i = y_i P/Psat_i", latex=r"\frac{1}{P_{dew}} = \sum_i \frac{y_i}{P_i^{sat}(T)}",
     run=dew_P, reference=lambda y, ant, units, base, T: (lambda r: {"P": r[1], "x": list(r[3])})(_ref_flash_ideal(y, ant, units, base, T=T, VF=1)),
     wrong=lambda y, ant, units, base, T: (lambda ps: _out(P=sum(a * b for a, b in zip(y, ps)), x=list(y)))([_psat(r, units, base, T) for r in ant]),
     rand=lambda rng: (lambda z, ant, u, b: {"y": z, "ant": ant, "units": u, "base": b, "T": rng.uniform(300, 400)})(*_rand_mixture(rng)),
     phrases=("Dew-point pressure", "Dew point pressure", "Raoult dew pressure"),
     examples=[_ex(dict(_BT, y=[0.4, 0.6], T=368.15)), _ex(dict(_BT, y=[0.6, 0.4], T=380.0))], level="beginner")

_add("chemeng_raoult_bubble_temperature",
     name="Bubble-point temperature (Raoult's law)", topic="separations / vapour-liquid equilibrium",
     purpose="Temperature at which a liquid of composition x starts to boil at pressure P, and the first-bubble composition (Raoult's law, Antoine vapour pressures; iterative).",
     inputs=[_vecin("x", "liquid mole fractions", 2, 10)] + _MIX_IN + [_phys("P", "pressure", "pressure")],
     outputs=[_o("T", "temperature", "bubble-point temperature"), _o("y", "number", "vapour mole fractions of the first bubble")], primary="T",
     conditions=["ideal liquid and ideal gas", "fractions sum to 1"],
     method="sum x_i Psat_i(T) = P solved by bisection between the pure-component boiling points", latex=r"\sum_i x_i P_i^{sat}(T_{bubble}) = P",
     run=bubble_T, reference=_ref_bubble_T,
     wrong=lambda x, ant, units, base, P: (lambda T: _out(T=T, y=list(x)))(sum(a * _tsat(r, units, base, P) for a, r in zip(x, ant))),
     rand=lambda rng: (lambda z, ant, u, b: {"x": z, "ant": ant, "units": u, "base": b, "P": P_ATM * rng.uniform(0.2, 3.0)})(*_rand_mixture(rng)),
     phrases=("Bubble-point temperature", "Bubble point temperature", "Raoult bubble temperature"),
     examples=[_ex(dict(_BT, x=[0.4, 0.6], P=101325.0)), _ex(dict(_BT, x=[0.7, 0.3], P=50000.0))], level="intermediate",
     extra_q=[{"q": "Bubble-point temperature: x = [0.4, 0.6], ant = [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], units = mmHg_degC, base = log10, P = 1 atm", "like": 0},
              {"q": "Bubble-point temperature: x = [0.4, 0.5], ant = [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], units = mmHg_degC, base = log10, P = 1 atm", "outcome": "INVALID_INPUT"}])

_add("chemeng_raoult_dew_temperature",
     name="Dew-point temperature (Raoult's law)", topic="separations / vapour-liquid equilibrium",
     purpose="Temperature at which a vapour of composition y starts to condense at pressure P, and the first-drop composition (Raoult's law, Antoine vapour pressures; iterative).",
     inputs=[_vecin("y", "vapour mole fractions", 2, 10)] + _MIX_IN + [_phys("P", "pressure", "pressure")],
     outputs=[_o("T", "temperature", "dew-point temperature"), _o("x", "number", "liquid mole fractions of the first drop")], primary="T",
     conditions=["ideal liquid and ideal gas", "fractions sum to 1"],
     method="sum y_i P/Psat_i(T) = 1 solved by bisection between the pure-component boiling points", latex=r"\sum_i \frac{y_i P}{P_i^{sat}(T_{dew})} = 1",
     run=dew_T, reference=lambda y, ant, units, base, P: (lambda r: {"T": r[0], "x": list(r[3])})(_ref_flash_ideal(y, ant, units, base, P=P, VF=1)),
     wrong=lambda y, ant, units, base, P: bubble_T(y, ant, units, base, P),
     rand=lambda rng: (lambda z, ant, u, b: {"y": z, "ant": ant, "units": u, "base": b, "P": P_ATM * rng.uniform(0.2, 3.0)})(*_rand_mixture(rng)),
     phrases=("Dew-point temperature", "Dew point temperature", "Raoult dew temperature"),
     examples=[_ex(dict(_BT, y=[0.4, 0.6], P=101325.0)), _ex(dict(_BT, y=[0.7, 0.3], P=50000.0))], level="intermediate")


def _rr(z, K):
    """Rachford-Rice: vapour fraction V/F with sum z_i (K_i - 1)/(1 + V (K_i - 1)) = 0, by bisection on the bracket
    where every denominator stays positive (a two-phase mixture: sum z K > 1 and sum z/K > 1)."""
    _need(sum(a * k for a, k in zip(z, K)) > 1.0 and sum(a / k for a, k in zip(z, K)) > 1.0,
          "the mixture is not two-phase at these K-values (sum zK <= 1: all liquid; sum z/K <= 1: all vapour)")
    _need(max(abs(k - 1.0) for k in K) > 0.01,
          "every K-value lies within 1 % of unity: the vapour fraction is too ill-conditioned to report")
    kmax, kmin = max(K), min(K)
    lo = 1.0 / (1.0 - kmax) if kmax > 1 else -math.inf
    hi = 1.0 / (1.0 - kmin) if kmin < 1 else math.inf
    lo, hi = max(lo, 0.0), min(hi, 1.0)
    g = lambda V: sum(a * (k - 1.0) / (1.0 + V * (k - 1.0)) for a, k in zip(z, K))
    V = _bisect(g, lo, hi)
    x = [a / (1.0 + V * (k - 1.0)) for a, k in zip(z, K)]
    return V, x, [k * xi for k, xi in zip(K, x)]


def rachford_rice(z, K):
    _comp("z", z)
    _need(len(K) == len(z) and all(k > 0 for k in K), "give one positive K-value per component")
    V, x, y = _rr(z, K)
    return _out(VF=V, x=x, y=y)


def _ref_rachford_rice(z, K):
    from chemicals.rachford_rice import Rachford_Rice_solution_LN2       # the plain solver stops near 1e-8
    V, x, y = Rachford_Rice_solution_LN2(list(z), list(K))
    return {"VF": float(V), "x": list(x), "y": list(y)}


def _rand_rr(rng):
    for _ in range(100):
        n = rng.choice([2, 3, 4, 5, 6])
        z = _dirichlet(rng, n)
        K = [_lu(rng, 0.05, 20) for _ in range(n)]
        if sum(a * k for a, k in zip(z, K)) > 1.05 and sum(a / k for a, k in zip(z, K)) > 1.05:
            return {"z": z, "K": K}
    return {"z": [0.5, 0.5], "K": [2.0, 0.5]}


_add("chemeng_rachford_rice_flash",
     name="Isothermal flash with given K-values (Rachford-Rice)", topic="separations / flash vaporisation",
     purpose="Vapour fraction and the liquid and vapour compositions of a feed flashed at conditions where the K-values are known.",
     inputs=[_vecin("z", "feed mole fractions", 2, 10), _vecin("K", "equilibrium ratios K_i = y_i/x_i", 2, 10)],
     outputs=[_o("VF", "number", "vapour fraction V/F"), _o("x", "number", "liquid mole fractions"), _o("y", "number", "vapour mole fractions")],
     primary="VF", conditions=["two-phase: sum z K > 1 and sum z/K > 1", "K-values independent of composition", "some K-value differs from 1 by more than 1 % (well-conditioned)"],
     method="Rachford-Rice equation sum z_i (K_i - 1)/(1 + V (K_i - 1)) = 0 solved for V by bisection; x_i = z_i/(1 + V (K_i - 1)), y_i = K_i x_i",
     latex=r"\sum_i \frac{z_i (K_i - 1)}{1 + \frac{V}{F}(K_i - 1)} = 0",
     run=rachford_rice, reference=_ref_rachford_rice,
     wrong=lambda z, K: (lambda r: _out(VF=r[0], x=r[1], y=[a / (1 + r[0] * (k - 1)) for a, k in zip(z, K)]))(_rr(z, [1.0 / k for k in K])),
     rand=_rand_rr,
     phrases=("Rachford-Rice flash", "Flash with K-values", "Isothermal flash with K-values"),
     examples=[_ex({"z": [0.5, 0.3, 0.2], "K": [1.685, 0.742, 0.532]}), _ex({"z": [0.2, 0.5, 0.3], "K": [4.2, 1.1, 0.15]})],
     level="advanced",
     extra_q=[{"q": "Rachford-Rice flash: z = [0.5, 0.5], K = [0.5, 0.8]", "outcome": "INVALID_INPUT"}])


def raoult_flash(z, ant, units, base, T, P):
    _comp("z", z)
    _antoine_rows(ant, len(z))
    _pos("T and P", T, P)
    K = [_psat(r, units, base, T) / P for r in ant]
    V, x, y = _rr(z, K)
    return _out(VF=V, x=x, y=y)


_add("chemeng_raoult_isothermal_flash",
     name="Isothermal flash of an ideal mixture (Raoult's law)", topic="separations / flash vaporisation",
     purpose="Vapour fraction and phase compositions of a feed flashed at temperature T and pressure P, with Raoult's-law K-values from Antoine vapour pressures.",
     inputs=[_vecin("z", "feed mole fractions", 2, 10)] + _MIX_IN + [_phys("T", "temperature", "flash temperature"), _phys("P", "pressure", "flash pressure")],
     outputs=[_o("VF", "number", "vapour fraction V/F"), _o("x", "number", "liquid mole fractions"), _o("y", "number", "vapour mole fractions")],
     primary="VF", conditions=["P between the dew- and bubble-point pressures at T (two phases)", "ideal liquid and gas", "some K-value differs from 1 by more than 1 %"],
     method="K_i = Psat_i(T)/P, then the Rachford-Rice equation solved for V/F by bisection",
     latex=r"K_i = \frac{P_i^{sat}(T)}{P},\quad \sum_i \frac{z_i (K_i - 1)}{1 + \frac{V}{F}(K_i - 1)} = 0",
     run=raoult_flash,
     reference=lambda z, ant, units, base, T, P: (lambda r: {"VF": r[2], "x": list(r[3]), "y": list(r[4])})(_ref_flash_ideal(z, ant, units, base, T=T, P=P)),
     wrong=lambda z, ant, units, base, T, P: (lambda K: (lambda r: _out(VF=1 - r[0], x=r[1], y=r[2]))(_rr(z, K)))([_psat(r, units, base, T) / P for r in ant]),
     rand=lambda rng: (lambda z, ant, u, b, T: (lambda pb, pd: {"z": z, "ant": ant, "units": u, "base": b, "T": T, "P": pd + rng.uniform(0.1, 0.9) * (pb - pd)})(
         bubble_P(z, ant, u, b, T)["P"], dew_P(z, ant, u, b, T)["P"]))(*_rand_mixture(rng), rng.uniform(320, 400)),
     phrases=("Raoult isothermal flash", "Ideal isothermal flash", "Isothermal flash with Antoine"),
     examples=[_ex(dict(_BT, z=[0.5, 0.5], T=368.15, P=101325.0)), _ex(dict(_BT, z=[0.3, 0.7], T=373.0, P=101325.0))], level="advanced",
     extra_q=[{"q": "Raoult isothermal flash: z = [0.5, 0.5], ant = [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], units = mmHg_degC, base = log10, T = 368.15 K, P = 5 bar", "outcome": "INVALID_INPUT"}])


_PUNIT = {"Pa": 1.0, "kPa": 1e3, "MPa": 1e6, "bar": 1e5, "atm": P_ATM, "psi": 6894.757293168361}


def wilson_flash(z, Tc, Pc, omega, p_unit, T, P):
    n = len(z)
    _comp("z", z)
    _need(len(Tc) == len(Pc) == len(omega) == n, "give Tc, Pc and omega for every component")
    _pos("Tc and Pc", *Tc, *Pc)
    _pos("T and P", T, P)
    pcs = [p * _PUNIT[p_unit] for p in Pc]
    K = [pc / P * math.exp(5.37 * (1.0 + w) * (1.0 - tc / T)) for tc, pc, w in zip(Tc, pcs, omega)]
    V, x, y = _rr(z, K)
    return _out(VF=V, x=x, y=y, K=K)


def _ref_wilson_flash(z, Tc, Pc, omega, p_unit, T, P):
    from chemicals.flash_basic import flash_wilson, Wilson_K_value
    pcs = [p * _PUNIT[p_unit] for p in Pc]
    _T, _P, V, x, y = flash_wilson(list(z), list(Tc), pcs, list(omega), T=T, P=P)
    return {"VF": float(V), "x": list(x), "y": list(y), "K": [Wilson_K_value(T, P, a, b, c) for a, b, c in zip(Tc, pcs, omega)]}


_HC = [("methane", 190.564, 45.99, 0.011), ("ethane", 305.32, 48.72, 0.099), ("propane", 369.83, 42.48, 0.152),
       ("n-butane", 425.12, 37.96, 0.2), ("n-pentane", 469.7, 33.7, 0.252), ("n-hexane", 507.6, 30.25, 0.3),
       ("n-heptane", 540.2, 27.4, 0.35), ("n-octane", 568.7, 24.9, 0.399), ("benzene", 562.05, 48.95, 0.21),
       ("toluene", 591.75, 41.08, 0.264)]


def _rand_wilson(rng):
    for _ in range(200):
        n = rng.choice([2, 3, 4, 5])
        comps = rng.sample(_HC, n)
        z = _dirichlet(rng, n)
        T, P = rng.uniform(250, 500), _lu(rng, 2e5, 5e6)
        pu = rng.choice(list(_PUNIT))
        x = {"z": z, "Tc": [c[1] for c in comps], "Pc": [c[2] * 1e5 / _PUNIT[pu] for c in comps], "omega": [c[3] for c in comps],
             "p_unit": pu, "T": T, "P": P}
        K = [c[2] * 1e5 / P * math.exp(5.37 * (1 + c[3]) * (1 - c[1] / T)) for c in comps]
        if sum(a * k for a, k in zip(z, K)) > 1.05 and sum(a / k for a, k in zip(z, K)) > 1.05:
            return x
    return x


_add("chemeng_wilson_k_flash",
     name="Flash with Wilson K-values (hydrocarbon mixtures)", topic="separations / flash vaporisation",
     purpose="Vapour fraction and phase compositions of a hydrocarbon feed at T and P with K-values estimated by Wilson's correlation from critical constants and acentric factors.",
     inputs=[_vecin("z", "feed mole fractions", 2, 10), _vecin("Tc", "critical temperatures, K", 2, 10),
             _vecin("Pc", "critical pressures, in the unit p_unit", 2, 10), _vecin("omega", "acentric factors", 2, 10),
             _choice("p_unit", list(_PUNIT), "unit of the critical pressures: Pa, kPa, MPa, bar, atm or psi"),
             _phys("T", "temperature", "flash temperature"), _phys("P", "pressure", "flash pressure")],
     outputs=[_o("VF", "number", "vapour fraction V/F"), _o("x", "number", "liquid mole fractions"), _o("y", "number", "vapour mole fractions"),
              _o("K", "number", "Wilson K-values")], primary="VF",
     conditions=["two-phase at the Wilson K-values", "some K-value differs from 1 by more than 1 %", "Wilson's correlation suits light hydrocarbons at low to moderate pressure"],
     method="K_i = (Pc_i/P) exp[5.37 (1 + omega_i)(1 - Tc_i/T)], then the Rachford-Rice equation by bisection",
     latex=r"K_i = \frac{P_{c,i}}{P}\exp\!\left[5.37(1+\omega_i)\left(1 - \frac{T_{c,i}}{T}\right)\right]",
     run=wilson_flash, reference=_ref_wilson_flash,
     wrong=lambda z, Tc, Pc, omega, p_unit, T, P: (lambda K: (lambda r: _out(VF=r[0], x=r[1], y=r[2], K=K))(_rr(z, K)))(
         [pc * _PUNIT[p_unit] / P * math.exp(5.37 * (1.0 + w) * (1.0 - T / tc)) for tc, pc, w in zip(Tc, Pc, omega)]),
     rand=_rand_wilson,
     phrases=("Wilson K-value flash", "Flash with Wilson K-values"),
     examples=[_ex({"z": [0.3, 0.4, 0.3], "Tc": [190.564, 369.83, 507.6], "Pc": [45.99, 42.48, 30.25], "omega": [0.011, 0.152, 0.3],
                    "p_unit": "bar", "T": 300.0, "P": 2e6}),
               _ex({"z": [0.5, 0.5], "Tc": [425.12, 507.6], "Pc": [3.796, 3.025], "omega": [0.2, 0.3], "p_unit": "MPa", "T": 380.0, "P": 1e6})],
     level="advanced")


# --- distillation: minimum reflux and McCabe-Thiele stage count (binary, constant alpha, CMO) ---------------------
def _dist_check(alpha, zF, xD, q, xB=None):
    _fin(alpha, zF, xD, q)
    _need(alpha > 1.0, "the relative volatility alpha must exceed 1")
    _need(0 < zF < xD < 1, "the compositions must satisfy 0 < zF < xD < 1")
    if xB is not None:
        _need(0 < xB < zF, "the bottoms composition must satisfy 0 < xB < zF")
    _need(-5 <= q <= 5, "the feed quality q must lie between -5 and 5")


def _underwood_rmin(alpha, zF, xD, q):
    f = lambda th: alpha * zF / (alpha - th) + (1 - zF) / (1 - th) - (1 - q)
    lo, hi = 1.0, alpha
    for _ in range(2000):                       # bisection strictly inside (1, alpha)
        m = 0.5 * (lo + hi)
        if m <= lo or m >= hi:
            break
        if f(m) > 0:
            hi = m
        else:
            lo = m
    th = 0.5 * (lo + hi)
    return alpha * xD / (alpha - th) + (1 - xD) / (1 - th) - 1.0


def _eq_y(alpha, x):
    return alpha * x / (1 + (alpha - 1) * x)


def minimum_reflux(alpha, zF, xD, q):
    _dist_check(alpha, zF, xD, q)
    Rm = _underwood_rmin(alpha, zF, xD, q)
    _need(Rm > 0, "the feed is richer than the distillate at the pinch: no minimum reflux exists")
    return _out(Rmin=Rm)


def _ref_pinch_rmin(alpha, zF, xD, q):
    """McCabe-Thiele geometry: the pinch is where the q-line meets the equilibrium curve; R_min is the slope of the
    line from (xD, xD) through the pinch."""
    import numpy as np
    if abs(q - 1.0) < 1e-15:
        xs = [zF]
    else:
        a, b, c = q * (alpha - 1), alpha + q * (1 - alpha) - zF * (alpha - 1), -zF
        xs = [float(r.real) for r in np.roots([a, b, c]) if abs(r.imag) < 1e-12] if abs(a) > 1e-15 else [-c / b]
    xs = [x for x in xs if 0 <= x <= 1]
    x = xs[0]
    y = _eq_y(alpha, x)
    return {"Rmin": (xD - y) / (y - x)}


def _rand_dist(rng):
    for _ in range(200):
        alpha, zF = rng.uniform(1.3, 6.0), rng.uniform(0.2, 0.7)
        xD, q = rng.uniform(max(zF + 0.15, 0.8), 0.995), rng.choice([1.0, 0.0, rng.uniform(-0.4, 1.6)])
        try:
            minimum_reflux(alpha, zF, xD, q)
            return {"alpha": alpha, "zF": zF, "xD": xD, "q": q}
        except ValueError:
            continue


_DIST_IN = [_numin("alpha", "relative volatility of the light component (constant)"),
            _numin("zF", "mole fraction of the light component in the feed", pct=True),
            _numin("xD", "mole fraction of the light component in the distillate", pct=True),
            _numin("q", "feed quality q: 1 saturated liquid, 0 saturated vapour, > 1 subcooled, < 0 superheated")]

_add("chemeng_binary_minimum_reflux",
     name="Minimum reflux ratio of a binary column (Underwood)", topic="separations / distillation",
     purpose="Minimum reflux ratio of a binary distillation with constant relative volatility, for any feed condition q (Underwood's equations).",
     inputs=_DIST_IN, outputs=[_o("Rmin", "number", "minimum reflux ratio L/D")], primary="Rmin",
     conditions=["binary, constant alpha, constant molar overflow", "0 < zF < xD < 1"],
     method="Underwood: sum alpha_i z_i/(alpha_i - theta) = 1 - q for theta between 1 and alpha (bisection); R_min + 1 = sum alpha_i x_Di/(alpha_i - theta)",
     latex=r"\sum_i \frac{\alpha_i z_{F,i}}{\alpha_i - \theta} = 1 - q,\quad R_{min} + 1 = \sum_i \frac{\alpha_i x_{D,i}}{\alpha_i - \theta}",
     run=minimum_reflux, reference=_ref_pinch_rmin,
     wrong=lambda alpha, zF, xD, q: _out(Rmin=_underwood_rmin(alpha, zF, xD, q) + 1.0), rand=_rand_dist,
     phrases=("Minimum reflux ratio", "Underwood minimum reflux", "Binary minimum reflux"),
     examples=[_ex({"alpha": 2.5, "zF": 0.5, "xD": 0.95, "q": 1.0}), _ex({"alpha": 2.4, "zF": 0.4, "xD": 0.97, "q": 0.5})],
     level="intermediate",
     extra_q=[{"q": "Minimum reflux ratio: alpha = 0.8, zF = 0.5, xD = 0.95, q = 1", "outcome": "INVALID_INPUT"}])


def _mt_lines(alpha, zF, xD, xB, q, R):
    a1, b1 = R / (R + 1.0), xD / (R + 1.0)
    if abs(q - 1.0) < 1e-15:
        xp = zF
    else:
        xp = (b1 + zF / (q - 1.0)) / (q / (q - 1.0) - a1)
    yp = a1 * xp + b1
    _need(xB < xp < xD and yp < _eq_y(alpha, xp), "the operating lines must meet below the equilibrium curve (R above R_min)")
    s = (yp - xB) / (xp - xB)
    return a1, b1, xp, yp, s


def mccabe_thiele(alpha, zF, xD, xB, q, R):
    _dist_check(alpha, zF, xD, q, xB)
    _fin(R)
    Rm = _underwood_rmin(alpha, zF, xD, q)
    _need(R > Rm * (1 + 1e-6) and R > 0, f"the reflux ratio must exceed the minimum reflux ratio {Rm:.6g}")
    a1, b1, xp, yp, s = _mt_lines(alpha, zF, xD, xB, q, R)
    x, n = xD / (alpha - (alpha - 1) * xD), 1             # liquid leaving the top stage (total condenser: y1 = xD)
    while x > xB:
        y = a1 * x + b1 if x > xp else xB + s * (x - xB)
        x = y / (alpha - (alpha - 1) * y)
        n += 1
        _need(n <= 400, "more than 400 stages: the reflux ratio is too close to the minimum")
    return _out(N=n, Rmin=Rm)


def _ref_mccabe_thiele(alpha, zF, xD, xB, q, R):
    """Stage-to-stage calculation from the reboiler upward (the reboiler is stage 1 counted from the bottom):
    equilibrium vapour, then the liquid from the stage above from the operating line, until the vapour reaches xD;
    R_min from the pinch geometry."""
    a1, b1, xp, yp, s = _mt_lines(alpha, zF, xD, xB, q, R)
    x, m = xB, 1
    y = _eq_y(alpha, x)
    while y < xD:
        x = xB + (y - xB) / s if y <= yp else (y - b1) / a1
        m += 1
        y = _eq_y(alpha, x)
        if m > 1000:
            break
    return {"N": m, "Rmin": _ref_pinch_rmin(alpha, zF, xD, q)["Rmin"]}


def _rand_mt(rng):
    for _ in range(200):
        d = _rand_dist(rng)
        d["xB"] = rng.uniform(0.005, min(0.15, d["zF"] - 0.05))
        d["R"] = minimum_reflux(d["alpha"], d["zF"], d["xD"], d["q"])["Rmin"] * rng.uniform(1.2, 3.0)
        try:
            mccabe_thiele(**d)
            return d
        except ValueError:
            continue


_add("chemeng_mccabe_thiele_stages",
     name="McCabe-Thiele number of theoretical stages (binary)", topic="separations / distillation",
     purpose="Number of theoretical stages (the partial reboiler counted as one) of a binary column at reflux ratio R, by stepping between the operating lines and a constant-alpha equilibrium curve, with the feed switched at the intersection of the operating lines.",
     inputs=_DIST_IN + [_numin("xB", "mole fraction of the light component in the bottoms", pct=True), _numin("R", "reflux ratio L/D")],
     outputs=[_o("N", "number", "theoretical stages including the partial reboiler"), _o("Rmin", "number", "minimum reflux ratio")],
     primary="N", conditions=["binary, constant alpha, constant molar overflow, total condenser", "R > R_min", "0 < xB < zF < xD < 1"],
     method="stepping from the top: equilibrium x from y, then y from the rectifying line above the intersection and the stripping line below it, until x <= xB",
     latex=r"y_{n+1} = \frac{R}{R+1}x_n + \frac{x_D}{R+1},\quad x_n = \frac{y_n}{\alpha - (\alpha-1)y_n}",
     run=mccabe_thiele, reference=_ref_mccabe_thiele,
     wrong=lambda alpha, zF, xD, xB, q, R: (lambda r: _out(N=r["N"] - 1, Rmin=r["Rmin"]))(mccabe_thiele(alpha, zF, xD, xB, q, R)),
     rand=_rand_mt,
     phrases=("McCabe-Thiele stages", "McCabe-Thiele number of stages", "Number of theoretical stages"),
     examples=[_ex({"alpha": 2.5, "zF": 0.5, "xD": 0.95, "q": 1.0, "xB": 0.05, "R": 2.0}),
               _ex({"alpha": 2.4, "zF": 0.4, "xD": 0.97, "q": 0.5, "xB": 0.02, "R": 3.5})],
     level="intermediate",
     extra_q=[{"q": "McCabe-Thiele stages: alpha = 2.5, zF = 0.5, xD = 0.95, q = 1, xB = 0.05, R = 1", "outcome": "INVALID_INPUT"}])


# --- absorption, stripping and extraction (dilute, linear equilibrium) ---------------------------------------------
def _kremser_frac(A, N):
    if abs(A - 1.0) <= 1e-12:
        return N / (N + 1.0)
    return (A ** (N + 1) - A) / (A ** (N + 1) - 1.0)


def _stages_check(N, *pos):
    _need(isinstance(N, int) and 1 <= N <= 200, "N must be a whole number of stages from 1 to 200")
    _pos("the factors and slopes", *pos)


def kremser_absorber(N, A, m, y_in, x_in):
    _stages_check(N, A, m)
    _nonneg("the compositions", y_in, x_in)
    _need(y_in > m * x_in, "no absorption: the entering gas is not richer than equilibrium with the entering liquid")
    y_out = y_in - _kremser_frac(A, N) * (y_in - m * x_in)
    return _out(y_out=y_out, recovery=(y_in - y_out) / y_in)


def _ref_countercurrent(N, L, Vm, top_in, bottom_in):
    """numpy: the N stage balances of a countercurrent cascade (phase 1 flows down with ratio L, phase 2 up with
    Vm = V m, equilibrium phase2 = m phase1), solved as one linear system; returns the phase-1 composition of every stage."""
    import numpy as np
    M = np.zeros((N, N))
    b = np.zeros(N)
    for n in range(N):
        M[n, n] = -(L + Vm)
        if n > 0:
            M[n, n - 1] = L
        else:
            b[n] -= L * top_in
        if n < N - 1:
            M[n, n + 1] = Vm
        else:
            b[n] -= bottom_in
    return np.linalg.solve(M, b)


def _ref_kremser_absorber(N, A, m, y_in, x_in):
    x = _ref_countercurrent(N, A * m, m, x_in, y_in)          # V = 1, L = A m
    y_out = float(m * x[0])
    return {"y_out": y_out, "recovery": (y_in - y_out) / y_in}


_add("chemeng_kremser_absorber",
     name="Absorber with N equilibrium stages (Kremser)", topic="separations / absorption",
     purpose="Solute mole fraction in the gas leaving a countercurrent absorber of N ideal stages, and the fraction of solute absorbed, for dilute gas with linear equilibrium y = m x.",
     inputs=[_intin("N", "number of ideal stages", 1, 200), _numin("A", "absorption factor A = L/(m V)"),
             _numin("m", "slope of the equilibrium line y = m x"), _numin("y_in", "solute mole fraction in the entering gas"),
             _numin("x_in", "solute mole fraction in the entering liquid (solvent)")],
     outputs=[_o("y_out", "number", "solute mole fraction in the leaving gas"), _o("recovery", "number", "fraction of the entering solute absorbed")],
     primary="y_out", conditions=["dilute, constant L and V, linear equilibrium", "y_in > m x_in"],
     method="Kremser: (y_in - y_out)/(y_in - m x_in) = (A^(N+1) - A)/(A^(N+1) - 1); N/(N + 1) when A = 1",
     latex=r"\frac{y_{in} - y_{out}}{y_{in} - m x_{in}} = \frac{A^{N+1} - A}{A^{N+1} - 1}",
     run=kremser_absorber, reference=_ref_kremser_absorber,
     wrong=lambda N, A, m, y_in, x_in: (lambda y: _out(y_out=y, recovery=(y_in - y) / y_in))(y_in - (A ** N - A) / (A ** N - 1) * (y_in - m * x_in) if A != 1 else y_in),
     rand=lambda rng: {"N": rng.randint(1, 20), "A": rng.choice([rng.uniform(0.3, 0.95), rng.uniform(1.05, 3.0)]), "m": _lu(rng, 0.2, 5.0),
                       "y_in": rng.uniform(0.005, 0.1), "x_in": 0.0} if rng.random() < 0.5 else
     (lambda m, y: {"N": rng.randint(1, 20), "A": rng.uniform(1.05, 3.0), "m": m, "y_in": y, "x_in": rng.uniform(0, 0.5) * y / m})(_lu(rng, 0.2, 5.0), rng.uniform(0.005, 0.1)),
     phrases=("Kremser absorber", "Absorber stages", "Absorption with N stages"),
     examples=[_ex({"N": 5, "A": 1.4, "m": 2.0, "y_in": 0.05, "x_in": 0.0}), _ex({"N": 3, "A": 1.0, "m": 1.5, "y_in": 0.02, "x_in": 0.001})],
     level="intermediate")


def kremser_stripper(N, S, m, x_in, y_in):
    _stages_check(N, S, m)
    _nonneg("the compositions", y_in, x_in)
    _need(x_in > y_in / m, "no stripping: the entering liquid is not richer than equilibrium with the entering gas")
    x_out = x_in - _kremser_frac(S, N) * (x_in - y_in / m)
    return _out(x_out=x_out, recovery=(x_in - x_out) / x_in)


def _ref_kremser_stripper(N, S, m, x_in, y_in):
    x = _ref_countercurrent(N, 1.0, S, x_in, S * y_in / m)     # L = 1, V m = S
    x_out = float(x[-1])
    return {"x_out": x_out, "recovery": (x_in - x_out) / x_in}


_add("chemeng_kremser_stripper",
     name="Stripper with N equilibrium stages (Kremser)", topic="separations / stripping",
     purpose="Solute mole fraction in the liquid leaving a countercurrent stripper of N ideal stages, and the fraction stripped, for dilute liquid with linear equilibrium y = m x.",
     inputs=[_intin("N", "number of ideal stages", 1, 200), _numin("S", "stripping factor S = m V/L"),
             _numin("m", "slope of the equilibrium line y = m x"), _numin("x_in", "solute mole fraction in the entering liquid"),
             _numin("y_in", "solute mole fraction in the entering stripping gas")],
     outputs=[_o("x_out", "number", "solute mole fraction in the leaving liquid"), _o("recovery", "number", "fraction of the entering solute stripped")],
     primary="x_out", conditions=["dilute, constant L and V, linear equilibrium", "x_in > y_in/m"],
     method="Kremser: (x_in - x_out)/(x_in - y_in/m) = (S^(N+1) - S)/(S^(N+1) - 1); N/(N + 1) when S = 1",
     latex=r"\frac{x_{in} - x_{out}}{x_{in} - y_{in}/m} = \frac{S^{N+1} - S}{S^{N+1} - 1}",
     run=kremser_stripper, reference=_ref_kremser_stripper,
     wrong=lambda N, S, m, x_in, y_in: (lambda x: _out(x_out=x, recovery=(x_in - x) / x_in))(x_in - _kremser_frac(1.0 / S, N) * (x_in - y_in / m)),
     rand=lambda rng: (lambda m, x: {"N": rng.randint(1, 20), "S": rng.choice([rng.uniform(0.3, 0.95), rng.uniform(1.05, 3.0)]), "m": m,
                                     "x_in": x, "y_in": rng.choice([0.0, rng.uniform(0, 0.5) * x * m])})(_lu(rng, 0.2, 5.0), rng.uniform(0.005, 0.1)),
     phrases=("Kremser stripper", "Stripper stages", "Stripping with N stages"),
     examples=[_ex({"N": 6, "S": 1.5, "m": 3.0, "x_in": 0.01, "y_in": 0.0}), _ex({"N": 4, "S": 0.8, "m": 2.0, "x_in": 0.05, "y_in": 0.01})],
     level="intermediate")


def absorber_nog(y1, y2, x2, m, A, HOG):
    _pos("m, A and HOG", m, A, HOG)
    _nonneg("the compositions", y1, y2, x2)
    _need(y1 > y2 > m * x2, "need y1 > y2 > m x2 (gas in richer than gas out, which is above equilibrium with the entering liquid)")
    x1 = x2 + (y1 - y2) / (A * m)
    _need(y1 > m * x1, "the operating line crosses the equilibrium line: the separation is impossible at this A")
    if abs(A - 1.0) <= 1e-12:
        nog = (y1 - y2) / (y2 - m * x2)
    else:
        ia = 1.0 / A
        nog = math.log((1 - ia) * (y1 - m * x2) / (y2 - m * x2) + ia) / (1 - ia)
    return _out(NOG=nog, Z=HOG * nog)


def _ref_absorber_nog(y1, y2, x2, m, A, HOG):
    f = lambda y: 1.0 / (y - m * (x2 + (y - y2) / (A * m)))
    n = _quad(f, y2, y1)
    return {"NOG": n, "Z": HOG * n}


_add("chemeng_absorber_transfer_units",
     name="Packed absorber: number of transfer units and height (Colburn)", topic="separations / absorption",
     purpose="Number of overall gas-phase transfer units and packed height of a dilute countercurrent absorber with linear equilibrium (Colburn's equation).",
     inputs=[_numin("y1", "solute mole fraction in the gas entering (bottom)"), _numin("y2", "solute mole fraction in the gas leaving (top)"),
             _numin("x2", "solute mole fraction in the liquid entering (top)"), _numin("m", "slope of the equilibrium line y = m x"),
             _numin("A", "absorption factor A = L/(m V)"), _phys("HOG", "length", "height of an overall gas-phase transfer unit")],
     outputs=[_o("NOG", "number", "number of overall gas-phase transfer units"), _o("Z", "length", "packed height")],
     primary="NOG", conditions=["dilute, constant L and V, linear equilibrium", "y1 > y2 > m x2", "no pinch"],
     method="N_OG = ln[(1 - 1/A)(y1 - m x2)/(y2 - m x2) + 1/A]/(1 - 1/A); Z = H_OG N_OG",
     latex=r"N_{OG} = \frac{\ln\!\left[\left(1-\frac{1}{A}\right)\frac{y_1 - m x_2}{y_2 - m x_2} + \frac{1}{A}\right]}{1 - 1/A}",
     run=absorber_nog, reference=_ref_absorber_nog,
     wrong=lambda y1, y2, x2, m, A, HOG: (lambda n: _out(NOG=n, Z=HOG * n))(math.log((y1 - m * x2) / (y2 - m * x2))),
     rand=lambda rng: (lambda y1, m, A: {"y1": y1, "y2": y1 * rng.uniform(0.01, 0.3), "x2": 0.0 if rng.random() < 0.5 else y1 * 0.005 / m,
                                         "m": m, "A": A, "HOG": rng.uniform(0.3, 1.5)})(rng.uniform(0.005, 0.1), _lu(rng, 0.2, 5.0), rng.choice([rng.uniform(1.1, 3.0), rng.uniform(0.75, 0.95)])),
     phrases=("Absorber transfer units", "Packed absorber height", "Number of transfer units"),
     examples=[_ex({"y1": 0.02, "y2": 0.001, "x2": 0.0, "m": 1.2, "A": 1.5, "HOG": 0.6}), _ex({"y1": 0.05, "y2": 0.005, "x2": 0.0005, "m": 0.8, "A": 2.0, "HOG": 1.0})],
     level="advanced",
     extra_q=[{"q": "Absorber transfer units: y1 = 0.02, y2 = 0.001, x2 = 0, m = 1.2, A = 1.5, HOG = 60 cm", "like": 0}])


def countercurrent_extraction(F, S, K, N, XF, YS):
    _stages_check(N, F, S, K)
    _nonneg("the solute ratios", XF, YS)
    _need(XF > YS / K, "no extraction: the feed is not richer than equilibrium with the entering solvent")
    E = K * S / F
    XN = XF - _kremser_frac(E, N) * (XF - YS / K)
    return _out(XN=XN, Y1=YS + F * (XF - XN) / S, recovery=(XF - XN) / XF)


def _ref_countercurrent_extraction(F, S, K, N, XF, YS):
    X = _ref_countercurrent(N, F, S * K, XF, S * YS)
    XN, X1 = float(X[-1]), float(X[0])
    return {"XN": XN, "Y1": K * X1, "recovery": (XF - XN) / XF}


_add("chemeng_countercurrent_extraction",
     name="Countercurrent liquid-liquid extraction (N stages, immiscible solvents)", topic="separations / liquid extraction",
     purpose="Raffinate and extract solute ratios and the fraction extracted for N ideal countercurrent stages with immiscible carrier and solvent and a constant distribution ratio Y = K X (solute-free mass-ratio basis).",
     inputs=[_phys("F", "mass_flow_rate", "solute-free carrier flow in the feed"), _phys("S", "mass_flow_rate", "solute-free solvent flow"),
             _numin("K", "distribution ratio Y = K X (mass ratios)"), _intin("N", "number of ideal stages", 1, 200),
             _numin("XF", "solute ratio in the feed, kg solute per kg carrier"), _numin("YS", "solute ratio in the entering solvent, kg solute per kg solvent")],
     outputs=[_o("XN", "number", "solute ratio in the final raffinate"), _o("Y1", "number", "solute ratio in the final extract"),
              _o("recovery", "number", "fraction of the solute extracted")],
     primary="XN", conditions=["immiscible carrier and solvent, constant K", "XF > YS/K"],
     method="Kremser form with the extraction factor E = K S/F: (XF - XN)/(XF - YS/K) = (E^(N+1) - E)/(E^(N+1) - 1)",
     latex=r"\frac{X_F - X_N}{X_F - Y_S/K} = \frac{E^{N+1} - E}{E^{N+1} - 1},\ E = \frac{KS}{F}",
     run=countercurrent_extraction, reference=_ref_countercurrent_extraction,
     wrong=lambda F, S, K, N, XF, YS: (lambda XN: _out(XN=XN, Y1=YS + F * (XF - XN) / S, recovery=(XF - XN) / XF))(XF / (1 + K * S / F) ** N),
     rand=lambda rng: {"F": _lu(rng, 0.1, 50), "S": _lu(rng, 0.1, 50), "K": _lu(rng, 0.3, 5.0), "N": rng.randint(2, 12),
                       "XF": rng.uniform(0.01, 0.3), "YS": 0.0} | ({} if rng.random() < 0.5 else {"YS": 0.0}),
     phrases=("Countercurrent extraction", "Countercurrent liquid extraction"),
     examples=[_ex({"F": 100.0, "S": 80.0, "K": 1.5, "N": 4, "XF": 0.1, "YS": 0.0}), _ex({"F": 10.0, "S": 5.0, "K": 2.0, "N": 3, "XF": 0.2, "YS": 0.01})],
     level="advanced")


def crosscurrent_extraction(F, S, K, N, XF):
    _stages_check(N, F, S, K)
    _pos("XF", XF)
    XN = XF / (1.0 + K * S / F) ** N
    return _out(XN=XN, recovery=1.0 - XN / XF)


def _ref_crosscurrent_extraction(F, S, K, N, XF):
    """numpy: the N stage balances F X_(n-1) = F X_n + S K X_n solved together (fresh solvent to every stage)."""
    import numpy as np
    M = np.zeros((N, N))
    b = np.zeros(N)
    for n in range(N):
        M[n, n] = F + S * K
        if n > 0:
            M[n, n - 1] = -F
    b[0] = F * XF
    X = np.linalg.solve(M, b)
    return {"XN": float(X[-1]), "recovery": 1.0 - float(X[-1]) / XF}


_add("chemeng_crosscurrent_extraction",
     name="Crosscurrent (multiple-contact) extraction with fresh solvent", topic="separations / liquid extraction",
     purpose="Raffinate solute ratio after N ideal stages, each fed with the same amount of fresh solvent, for immiscible solvents and a constant distribution ratio.",
     inputs=[_phys("F", "mass_flow_rate", "solute-free carrier flow (or mass) in the feed"),
             _phys("S", "mass_flow_rate", "solute-free solvent flow (or mass) added to EACH stage"),
             _numin("K", "distribution ratio Y = K X (mass ratios)"), _intin("N", "number of stages", 1, 200),
             _numin("XF", "solute ratio in the feed, kg solute per kg carrier")],
     outputs=[_o("XN", "number", "solute ratio in the final raffinate"), _o("recovery", "number", "fraction of the solute extracted")],
     primary="XN", conditions=["immiscible carrier and solvent, constant K", "fresh (solute-free) solvent to every stage"],
     method="each stage divides the raffinate ratio by (1 + K S/F): X_N = X_F/(1 + K S/F)^N",
     latex=r"X_N = \frac{X_F}{(1 + KS/F)^N}",
     run=crosscurrent_extraction, reference=_ref_crosscurrent_extraction,
     wrong=lambda F, S, K, N, XF: (lambda XN: _out(XN=XN, recovery=1 - XN / XF))(XF / (1.0 + N * K * S / F)),
     rand=lambda rng: {"F": _lu(rng, 0.1, 50), "S": _lu(rng, 0.1, 50), "K": _lu(rng, 0.3, 5.0), "N": rng.randint(2, 12), "XF": rng.uniform(0.01, 0.3)},
     phrases=("Crosscurrent extraction", "Multiple-contact extraction"),
     examples=[_ex({"F": 100.0, "S": 20.0, "K": 1.5, "N": 4, "XF": 0.1}), _ex({"F": 10.0, "S": 10.0, "K": 2.0, "N": 1, "XF": 0.2})],
     level="beginner")


# --- activity-coefficient models ------------------------------------------------------------------------------
def _liq_x(x):
    _need(2 <= len(x) <= 10, "give 2 to 10 mole fractions")
    _fracs("x", x)
    _need(all(v > 0 for v in x), "every mole fraction must be positive (use a tiny value for infinite dilution)")


def _square(name, M, n, diag=None):
    _need(len(M) == n and all(len(r) == n for r in M), f"{name} must be an n x n matrix for n = {n} components")
    if diag is not None:
        _need(all(abs(M[i][i] - diag) <= 1e-12 for i in range(n)), f"the diagonal of {name} must be {diag:g}")


def wilson_gamma(x, Lam):
    n = len(x)
    _liq_x(x)
    _square("Lam", Lam, n, 1.0)
    _need(all(v > 0 for r in Lam for v in r), "Wilson parameters must be positive")
    S = [sum(x[j] * Lam[i][j] for j in range(n)) for i in range(n)]
    lg = [1.0 - math.log(S[i]) - sum(x[k] * Lam[k][i] / S[k] for k in range(n)) for i in range(n)]
    g = [math.exp(v) for v in lg]
    return _out(gamma1=g[0], gamma=g, GE_RT=-sum(x[i] * math.log(S[i]) for i in range(n)))


def _ref_wilson_gamma(x, Lam):
    from thermo.wilson import Wilson
    GE = Wilson(T=298.15, xs=list(x), lambda_as=[[math.log(v) for v in r] for r in Lam])
    g = list(GE.gammas())
    return {"gamma1": g[0], "gamma": g, "GE_RT": GE.GE() / (R_GAS * 298.15)}


def _rand_act(rng, kind):
    n = rng.choice([2, 2, 3, 4])
    x = _dirichlet(rng, n)
    if kind == "wilson":
        M = [[1.0 if i == j else _lu(rng, 0.1, 3.0) for j in range(n)] for i in range(n)]
        return {"x": x, "Lam": M}
    if kind == "nrtl":
        t = [[0.0 if i == j else rng.uniform(-1.0, 3.0) for j in range(n)] for i in range(n)]
        a = [[0.0] * n for _ in range(n)]
        for i in range(n):
            for j in range(i + 1, n):
                a[i][j] = a[j][i] = rng.choice([0.2, 0.3, 0.47, rng.uniform(0.1, 0.5)])
        return {"x": x, "tau": t, "alpha": a}
    r = [rng.uniform(0.9, 5.0) for _ in range(n)]
    q = [rng.uniform(0.8, 4.5) for _ in range(n)]
    t = [[1.0 if i == j else _lu(rng, 0.2, 3.0) for j in range(n)] for i in range(n)]
    return {"x": x, "r": r, "q": q, "tau": t}


_add("chemeng_wilson_activity_coefficients",
     name="Activity coefficients from the Wilson equation", topic="separations / activity coefficients",
     purpose="Activity coefficients of every component and the excess Gibbs energy of a liquid mixture from Wilson parameters Lambda_ij.",
     inputs=[_vecin("x", "liquid mole fractions", 2, 10), _matin("Lam", "Wilson parameters Lambda_ij (dimensionless, Lambda_ii = 1) at the temperature of interest")],
     outputs=[_o("gamma1", "number", "activity coefficient of component 1"), _o("gamma", "number", "activity coefficients of every component"),
              _o("GE_RT", "number", "excess Gibbs energy over RT")], primary="gamma1",
     conditions=["Lambda_ii = 1, Lambda_ij > 0", "all mole fractions positive"],
     method="ln gamma_i = 1 - ln(sum_j x_j Lambda_ij) - sum_k x_k Lambda_ki/(sum_j x_j Lambda_kj)",
     latex=r"\ln\gamma_i = 1 - \ln\Big(\sum_j x_j\Lambda_{ij}\Big) - \sum_k \frac{x_k\Lambda_{ki}}{\sum_j x_j\Lambda_{kj}}",
     run=wilson_gamma, reference=_ref_wilson_gamma,
     wrong=lambda x, Lam: wilson_gamma(x, [list(r) for r in zip(*Lam)]) | {},
     rand=lambda rng: _rand_act(rng, "wilson"),
     phrases=("Wilson activity coefficients", "Wilson equation activity coefficients"),
     examples=[_ex({"x": [0.3, 0.7], "Lam": [[1.0, 0.5], [1.5, 1.0]]}), _ex({"x": [0.2, 0.3, 0.5], "Lam": [[1.0, 0.8, 0.4], [1.2, 1.0, 0.9], [0.6, 1.1, 1.0]]})],
     level="advanced")


def nrtl_gamma(x, tau, alpha):
    n = len(x)
    _liq_x(x)
    _square("tau", tau, n, 0.0)
    _square("alpha", alpha, n, 0.0)
    G = [[math.exp(-alpha[i][j] * tau[i][j]) for j in range(n)] for i in range(n)]
    D = [sum(x[k] * G[k][j] for k in range(n)) for j in range(n)]          # sum_k x_k G_kj
    N = [sum(x[k] * tau[k][j] * G[k][j] for k in range(n)) for j in range(n)]
    lg = [N[i] / D[i] + sum(x[j] * G[i][j] / D[j] * (tau[i][j] - N[j] / D[j]) for j in range(n)) for i in range(n)]
    g = [math.exp(v) for v in lg]
    return _out(gamma1=g[0], gamma=g, GE_RT=sum(x[i] * N[i] / D[i] for i in range(n)))


def _ref_nrtl_gamma(x, tau, alpha):
    from thermo.nrtl import NRTL
    GE = NRTL(T=298.15, xs=list(x), tau_as=[list(r) for r in tau], alpha_cs=[list(r) for r in alpha])
    g = list(GE.gammas())
    return {"gamma1": g[0], "gamma": g, "GE_RT": GE.GE() / (R_GAS * 298.15)}


_add("chemeng_nrtl_activity_coefficients",
     name="Activity coefficients from the NRTL equation", topic="separations / activity coefficients",
     purpose="Activity coefficients and excess Gibbs energy of a liquid mixture from NRTL interaction parameters tau_ij and non-randomness parameters alpha_ij.",
     inputs=[_vecin("x", "liquid mole fractions", 2, 10), _matin("tau", "NRTL parameters tau_ij (dimensionless, tau_ii = 0) at the temperature of interest"),
             _matin("alpha", "non-randomness parameters alpha_ij (symmetric, alpha_ii = 0)")],
     outputs=[_o("gamma1", "number", "activity coefficient of component 1"), _o("gamma", "number", "activity coefficients of every component"),
              _o("GE_RT", "number", "excess Gibbs energy over RT")], primary="gamma1",
     conditions=["tau_ii = 0, alpha_ii = 0", "all mole fractions positive"],
     method="G_ij = exp(-alpha_ij tau_ij); ln gamma_i = sum_j x_j tau_ji G_ji/sum_k x_k G_ki + sum_j (x_j G_ij/sum_k x_k G_kj)(tau_ij - sum_m x_m tau_mj G_mj/sum_k x_k G_kj)",
     latex=r"\ln\gamma_i = \frac{\sum_j x_j\tau_{ji}G_{ji}}{\sum_k x_k G_{ki}} + \sum_j \frac{x_jG_{ij}}{\sum_k x_kG_{kj}}\left(\tau_{ij} - \frac{\sum_m x_m\tau_{mj}G_{mj}}{\sum_k x_kG_{kj}}\right)",
     run=nrtl_gamma, reference=_ref_nrtl_gamma,
     wrong=lambda x, tau, alpha: nrtl_gamma(x, [list(r) for r in zip(*tau)], alpha),
     rand=lambda rng: _rand_act(rng, "nrtl"),
     phrases=("NRTL activity coefficients", "NRTL equation activity coefficients"),
     examples=[_ex({"x": [0.3, 0.7], "tau": [[0.0, 0.8], [1.2, 0.0]], "alpha": [[0.0, 0.3], [0.3, 0.0]]}),
               _ex({"x": [0.25, 0.25, 0.5], "tau": [[0.0, 1.5, 0.4], [0.2, 0.0, 2.0], [0.9, -0.3, 0.0]], "alpha": [[0.0, 0.3, 0.2], [0.3, 0.0, 0.47], [0.2, 0.47, 0.0]]})],
     level="expert")


def uniquac_gamma(x, r, q, tau):
    n = len(x)
    _liq_x(x)
    _need(len(r) == len(q) == n, "give r and q for every component")
    _pos("r and q", *r, *q)
    _square("tau", tau, n, 1.0)
    _need(all(v > 0 for row in tau for v in row), "UNIQUAC tau must be positive")
    sr, sq = sum(a * b for a, b in zip(r, x)), sum(a * b for a, b in zip(q, x))
    phi = [r[i] * x[i] / sr for i in range(n)]
    th = [q[i] * x[i] / sq for i in range(n)]
    l = [5.0 * (r[i] - q[i]) - (r[i] - 1.0) for i in range(n)]
    sl = sum(x[j] * l[j] for j in range(n))
    S = [sum(th[k] * tau[k][j] for k in range(n)) for j in range(n)]       # sum_k theta_k tau_kj
    lg = []
    for i in range(n):
        comb = math.log(phi[i] / x[i]) + 5.0 * q[i] * math.log(th[i] / phi[i]) + l[i] - phi[i] / x[i] * sl
        res = q[i] * (1.0 - math.log(S[i]) - sum(th[j] * tau[i][j] / S[j] for j in range(n)))
        lg.append(comb + res)
    g = [math.exp(v) for v in lg]
    return _out(gamma1=g[0], gamma=g)


def _ref_uniquac_gamma(x, r, q, tau):
    from thermo.uniquac import UNIQUAC
    GE = UNIQUAC(T=298.15, xs=list(x), rs=list(r), qs=list(q), tau_as=[[math.log(v) for v in row] for row in tau])
    g = list(GE.gammas())
    return {"gamma1": g[0], "gamma": g}


_add("chemeng_uniquac_activity_coefficients",
     name="Activity coefficients from the UNIQUAC equation", topic="separations / activity coefficients",
     purpose="Activity coefficients of a liquid mixture from UNIQUAC volume and area parameters r, q and interaction parameters tau_ij (coordination number 10).",
     inputs=[_vecin("x", "liquid mole fractions", 2, 10), _vecin("r", "UNIQUAC volume parameters r_i", 2, 10),
             _vecin("q", "UNIQUAC area parameters q_i", 2, 10), _matin("tau", "UNIQUAC parameters tau_ij = exp(-a_ij/T) at the temperature of interest (tau_ii = 1)")],
     outputs=[_o("gamma1", "number", "activity coefficient of component 1"), _o("gamma", "number", "activity coefficients of every component")],
     primary="gamma1", conditions=["tau_ii = 1, tau_ij > 0", "all mole fractions positive", "coordination number z = 10"],
     method="combinatorial part ln(Phi_i/x_i) + 5 q_i ln(theta_i/Phi_i) + l_i - (Phi_i/x_i) sum x_j l_j plus residual part q_i [1 - ln(sum_j theta_j tau_ji) - sum_j theta_j tau_ij/sum_k theta_k tau_kj]",
     latex=r"\ln\gamma_i = \ln\frac{\Phi_i}{x_i} + \frac{z}{2}q_i\ln\frac{\theta_i}{\Phi_i} + l_i - \frac{\Phi_i}{x_i}\sum_j x_jl_j + q_i\Big[1 - \ln\sum_j\theta_j\tau_{ji} - \sum_j\frac{\theta_j\tau_{ij}}{\sum_k\theta_k\tau_{kj}}\Big]",
     run=uniquac_gamma, reference=_ref_uniquac_gamma,
     wrong=lambda x, r, q, tau: uniquac_gamma(x, r, q, [list(rw) for rw in zip(*tau)]),
     rand=lambda rng: _rand_act(rng, "uniquac"),
     phrases=("UNIQUAC activity coefficients", "UNIQUAC equation activity coefficients"),
     examples=[_ex({"x": [0.3, 0.7], "r": [2.1055, 0.92], "q": [1.972, 1.4], "tau": [[1.0, 0.9], [1.1, 1.0]]}),
               _ex({"x": [0.2, 0.3, 0.5], "r": [2.5735, 3.1878, 0.92], "q": [2.336, 2.4, 1.4], "tau": [[1.0, 0.7, 1.3], [1.2, 1.0, 0.6], [0.5, 1.6, 1.0]]})],
     level="expert")


# ================================================================================================ PROPERTY ESTIMATION
# --- cubic equations of state --------------------------------------------------------------------------------
_PR_C1, _PR_C2 = 0.4572355289213822, 0.07779607390388846           # the exact Peng-Robinson constants (as in thermo)
_SRK_C1, _SRK_C2 = 0.4274802335403414, 0.08664034996495772


def _cubic_roots(a2, a1, a0):
    """Real roots of z^3 + a2 z^2 + a1 z + a0 (trigonometric / Cardano form, each polished by Newton steps);
    a root pair too close to a double root is refused."""
    p = a1 - a2 * a2 / 3.0
    qq = 2.0 * a2 ** 3 / 27.0 - a2 * a1 / 3.0 + a0
    disc = (qq / 2.0) ** 2 + (p / 3.0) ** 3
    scale = max(abs(qq / 2.0) ** 2, abs(p / 3.0) ** 3, 1e-300)
    _need(abs(disc) > 1e-8 * scale, "the state is too close to a double root of the cubic (near the spinodal or critical point)")
    if disc > 0:
        s = math.sqrt(disc)
        u = -qq / 2.0 + s if qq <= 0 else -qq / 2.0 - s
        u = math.copysign(abs(u) ** (1.0 / 3.0), u)
        t = u - p / (3.0 * u) if u != 0 else 0.0
        roots = [t - a2 / 3.0]
    else:
        m = 2.0 * math.sqrt(-p / 3.0)
        c = max(-1.0, min(1.0, 3.0 * qq / (p * m)))
        th = math.acos(c) / 3.0
        roots = [m * math.cos(th - 2.0 * math.pi * k / 3.0) - a2 / 3.0 for k in range(3)]
    out = []
    for z in roots:
        for _ in range(4):
            f = ((z + a2) * z + a1) * z + a0
            d = (3.0 * z + 2.0 * a2) * z + a1
            if d == 0:
                break
            z -= f / d
        out.append(z)
    return sorted(out)


def _eos(kind, Tc, Pc, omega, T, P):
    _pos("Tc, Pc, T and P", Tc, Pc, T, P)
    _fin(omega)
    _need(-0.5 < omega < 2.0, "the acentric factor must lie between -0.5 and 2")
    RT = R_GAS * T
    if kind == "PR":
        b = _PR_C2 * R_GAS * Tc / Pc
        k = 0.37464 + 1.54226 * omega - 0.26992 * omega * omega
        a = _PR_C1 * R_GAS * R_GAS * Tc * Tc / Pc * (1.0 + k * (1.0 - math.sqrt(T / Tc))) ** 2
        A, B = a * P / (RT * RT), b * P / RT
        zs = _cubic_roots(-(1.0 - B), A - 3.0 * B * B - 2.0 * B, -(A * B - B * B - B ** 3))
        s2 = math.sqrt(2.0)
        lnphi = lambda Z: Z - 1.0 - math.log(Z - B) - A / (2.0 * s2 * B) * math.log((Z + (1.0 + s2) * B) / (Z + (1.0 - s2) * B))
    else:
        b = _SRK_C2 * R_GAS * Tc / Pc
        m = 0.480 + 1.574 * omega - 0.176 * omega * omega
        a = _SRK_C1 * R_GAS * R_GAS * Tc * Tc / Pc * (1.0 + m * (1.0 - math.sqrt(T / Tc))) ** 2
        A, B = a * P / (RT * RT), b * P / RT
        zs = _cubic_roots(-1.0, A - B - B * B, -A * B)
        lnphi = lambda Z: Z - 1.0 - math.log(Z - B) - A / B * math.log(1.0 + B / Z)
    zs = [z for z in zs if z > B]
    _need(bool(zs), "no physical root of the cubic")
    if len(zs) == 3:
        zs = [zs[0], zs[2]]                       # the middle root is unstable: never a phase
    if len(zs) == 2:
        _need((zs[1] - zs[0]) > 1e-4 * zs[1], "the liquid and vapour roots nearly coincide")
    zl, zv = zs[0], zs[-1]
    return _out(Z_vapour=zv, Z_liquid=zl, phi_vapour=math.exp(lnphi(zv)), phi_liquid=math.exp(lnphi(zl)), n_roots=1 if len(zs) == 1 else 3)


def _ref_eos(kind, Tc, Pc, omega, T, P):
    from thermo.eos import PR, SRK
    e = (PR if kind == "PR" else SRK)(Tc=Tc, Pc=Pc, omega=omega, T=T, P=P)
    vs = []
    for v in e.raw_volumes:
        v = complex(v)
        if abs(v.imag) <= 1e-12 * abs(v) and v.real > e.b:
            vs.append(v.real)
    zs = sorted(P * v / (R_GAS * T) for v in vs)
    has_g, has_l = hasattr(e, "Z_g"), hasattr(e, "Z_l")
    phi_g = e.phi_g if has_g else e.phi_l
    phi_l = e.phi_l if has_l else e.phi_g
    return {"Z_vapour": zs[-1], "Z_liquid": zs[0], "phi_vapour": phi_g, "phi_liquid": phi_l, "n_roots": 1 if len(zs) == 1 else 3}


def _rand_eos(rng, kind):
    for _ in range(200):
        x = {"Tc": rng.uniform(150, 650), "Pc": rng.uniform(1.5e6, 8e6), "omega": rng.uniform(0.0, 0.6)}
        x["T"] = x["Tc"] * rng.uniform(0.55, 1.6)
        x["P"] = x["Pc"] * _lu(rng, 0.01, 1.5)
        try:
            _eos(kind, **x)
            return x
        except ValueError:
            continue


_EOS_IN = [_phys("Tc", "temperature", "critical temperature"), _phys("Pc", "pressure", "critical pressure"),
           _numin("omega", "acentric factor"), _phys("T", "temperature", "temperature"), _phys("P", "pressure", "pressure")]
_EOS_OUT = [_o("Z_vapour", "number", "compressibility factor of the vapour-like (largest) root"),
            _o("Z_liquid", "number", "compressibility factor of the liquid-like (smallest) root"),
            _o("phi_vapour", "number", "fugacity coefficient on the vapour root"), _o("phi_liquid", "number", "fugacity coefficient on the liquid root"),
            _o("n_roots", "number", "number of real roots above the covolume (1 or 3)")]

_add("chemeng_peng_robinson",
     name="Peng-Robinson equation of state: compressibility and fugacity coefficient", topic="property estimation / equations of state",
     purpose="Compressibility factors (vapour and liquid roots) and fugacity coefficients of a pure substance at T and P from the Peng-Robinson equation of state.",
     inputs=_EOS_IN, outputs=_EOS_OUT, primary="Z_vapour",
     conditions=["pure substance", "not too close to the critical point or a spinodal (a double root is refused)", "one real root: both roots reported equal"],
     method="Z^3 - (1 - B)Z^2 + (A - 3B^2 - 2B)Z - (AB - B^2 - B^3) = 0 with a = 0.45724 R^2Tc^2/Pc alpha(T), b = 0.07780 R Tc/Pc, kappa = 0.37464 + 1.54226 omega - 0.26992 omega^2",
     latex=r"Z^3 - (1-B)Z^2 + (A - 3B^2 - 2B)Z - (AB - B^2 - B^3) = 0",
     run=lambda Tc, Pc, omega, T, P: _eos("PR", Tc, Pc, omega, T, P), reference=lambda Tc, Pc, omega, T, P: _ref_eos("PR", Tc, Pc, omega, T, P),
     wrong=lambda Tc, Pc, omega, T, P: _eos("SRK", Tc, Pc, omega, T, P), rand=lambda rng: _rand_eos(rng, "PR"),
     phrases=("Peng-Robinson", "Peng-Robinson compressibility", "Peng-Robinson equation of state"),
     examples=[_ex({"Tc": 507.6, "Pc": 3025000.0, "omega": 0.2975, "T": 400.0, "P": 1e6}), _ex({"Tc": 190.564, "Pc": 4599000.0, "omega": 0.011, "T": 300.0, "P": 5e6})],
     level="advanced")

_add("chemeng_srk",
     name="Soave-Redlich-Kwong equation of state: compressibility and fugacity coefficient", topic="property estimation / equations of state",
     purpose="Compressibility factors (vapour and liquid roots) and fugacity coefficients of a pure substance at T and P from the Soave-Redlich-Kwong equation of state.",
     inputs=_EOS_IN, outputs=_EOS_OUT, primary="Z_vapour",
     conditions=["pure substance", "not too close to the critical point or a spinodal", "one real root: both roots reported equal"],
     method="Z^3 - Z^2 + (A - B - B^2)Z - AB = 0 with a = 0.42748 R^2Tc^2/Pc alpha(T), b = 0.08664 R Tc/Pc, m = 0.480 + 1.574 omega - 0.176 omega^2",
     latex=r"Z^3 - Z^2 + (A - B - B^2)Z - AB = 0",
     run=lambda Tc, Pc, omega, T, P: _eos("SRK", Tc, Pc, omega, T, P), reference=lambda Tc, Pc, omega, T, P: _ref_eos("SRK", Tc, Pc, omega, T, P),
     wrong=lambda Tc, Pc, omega, T, P: _eos("PR", Tc, Pc, omega, T, P), rand=lambda rng: _rand_eos(rng, "SRK"),
     phrases=("Soave-Redlich-Kwong", "SRK compressibility", "SRK equation of state"),
     examples=[_ex({"Tc": 507.6, "Pc": 3025000.0, "omega": 0.2975, "T": 400.0, "P": 1e6}), _ex({"Tc": 190.564, "Pc": 4599000.0, "omega": 0.011, "T": 300.0, "P": 5e6})],
     level="advanced")


# --- corresponding-states and other estimation correlations ---------------------------------------------------
def lee_kesler(T, Tc, Pc, omega):
    _pos("T, Tc and Pc", T, Tc, Pc)
    _fin(omega)
    _need(T < Tc, "T must be below the critical temperature")
    Tr = T / Tc
    f0 = 5.92714 - 6.09648 / Tr - 1.28862 * math.log(Tr) + 0.169347 * Tr ** 6
    f1 = 15.2518 - 15.6875 / Tr - 13.4721 * math.log(Tr) + 0.43577 * Tr ** 6
    return _out(Psat=Pc * math.exp(f0 + omega * f1))


_add("chemeng_lee_kesler_vapour_pressure",
     name="Vapour pressure by the Lee-Kesler correlation", topic="property estimation / vapour pressure",
     purpose="Vapour pressure of a pure substance from its critical constants and acentric factor (Lee-Kesler corresponding-states correlation).",
     inputs=[_phys("T", "temperature", "temperature"), _phys("Tc", "temperature", "critical temperature"),
             _phys("Pc", "pressure", "critical pressure"), _numin("omega", "acentric factor")],
     outputs=[_o("Psat", "pressure", "vapour pressure")], primary="Psat", conditions=["T < Tc", "non-polar or slightly polar substances"],
     method="ln(Psat/Pc) = f0(Tr) + omega f1(Tr) with the Lee-Kesler functions",
     latex=r"\ln\frac{P^{sat}}{P_c} = f^{(0)}(T_r) + \omega f^{(1)}(T_r)",
     run=lee_kesler, reference=lambda T, Tc, Pc, omega: {"Psat": __import__("chemicals.vapor_pressure", fromlist=["Lee_Kesler"]).Lee_Kesler(T, Tc, Pc, omega)},
     wrong=lambda T, Tc, Pc, omega: _out(Psat=Pc * math.exp(5.92714 - 6.09648 * T / Tc + omega * (15.2518 - 15.6875 * T / Tc))),
     rand=lambda rng: (lambda Tc: {"Tc": Tc, "T": Tc * rng.uniform(0.45, 0.98), "Pc": rng.uniform(1.5e6, 8e6), "omega": rng.uniform(0.0, 0.6)})(rng.uniform(150, 700)),
     phrases=("Lee-Kesler vapour pressure", "Lee-Kesler vapor pressure"),
     examples=[_ex({"T": 400.0, "Tc": 568.7, "Pc": 2490000.0, "omega": 0.399}), _ex({"T": 300.0, "Tc": 425.12, "Pc": 3796000.0, "omega": 0.2})],
     level="intermediate")


def watson(T, T_ref, Hvap_ref, Tc):
    _pos("T, T_ref, Hvap_ref and Tc", T, T_ref, Hvap_ref, Tc)
    _need(T < Tc and T_ref < Tc, "both temperatures must be below the critical temperature")
    return _out(Hvap=Hvap_ref * ((1.0 - T / Tc) / (1.0 - T_ref / Tc)) ** 0.38)


_add("chemeng_watson_heat_of_vaporization",
     name="Heat of vaporisation at another temperature (Watson)", topic="property estimation / heat of vaporisation",
     purpose="Heat of vaporisation at T from a known value at T_ref and the critical temperature (Watson's correlation, exponent 0.38).",
     inputs=[_phys("T", "temperature", "temperature wanted"), _phys("T_ref", "temperature", "temperature of the known value"),
             _phys("Hvap_ref", "molar_energy", "heat of vaporisation at T_ref"), _phys("Tc", "temperature", "critical temperature")],
     outputs=[_o("Hvap", "molar_energy", "heat of vaporisation at T")], primary="Hvap", conditions=["T, T_ref < Tc"],
     method="dHvap(T) = dHvap(T_ref) [(1 - T/Tc)/(1 - T_ref/Tc)]^0.38", latex=r"\Delta H_v(T) = \Delta H_v(T_{ref})\left(\frac{1 - T_r}{1 - T_{r,ref}}\right)^{0.38}",
     run=watson, reference=lambda T, T_ref, Hvap_ref, Tc: {"Hvap": __import__("chemicals.phase_change", fromlist=["Watson"]).Watson(T, Hvap_ref, T_ref, Tc)},
     wrong=lambda T, T_ref, Hvap_ref, Tc: _out(Hvap=Hvap_ref * ((1.0 - T / Tc) / (1.0 - T_ref / Tc)) ** 0.5),
     rand=lambda rng: (lambda Tc: {"Tc": Tc, "T": Tc * rng.uniform(0.4, 0.95), "T_ref": Tc * rng.uniform(0.4, 0.95), "Hvap_ref": rng.uniform(1e4, 6e4)})(rng.uniform(150, 700)),
     phrases=("Watson heat of vaporization", "Watson heat of vaporisation", "Watson correlation"),
     examples=[_ex({"T": 298.15, "T_ref": 373.15, "Hvap_ref": 40656.0, "Tc": 647.1}), _ex({"T": 400.0, "T_ref": 353.24, "Hvap_ref": 30720.0, "Tc": 562.05})],
     level="beginner")


def riedel(Tb, Tc, Pc):
    _pos("Tb, Tc and Pc", Tb, Tc, Pc)
    _need(Tb < 0.93 * Tc, "the normal boiling point must lie well below Tc")
    Tbr = Tb / Tc
    return _out(Hvap_b=1.093 * Tb * R_GAS * (math.log(Pc / 1e5) - 1.013) / (0.93 - Tbr))


_add("chemeng_riedel_heat_of_vaporization",
     name="Heat of vaporisation at the normal boiling point (Riedel)", topic="property estimation / heat of vaporisation",
     purpose="Heat of vaporisation at the normal boiling point from the boiling point and critical constants (Riedel's equation, Pc in bar).",
     inputs=[_phys("Tb", "temperature", "normal boiling point"), _phys("Tc", "temperature", "critical temperature"), _phys("Pc", "pressure", "critical pressure")],
     outputs=[_o("Hvap_b", "molar_energy", "heat of vaporisation at the normal boiling point")], primary="Hvap_b", conditions=["Tb well below Tc"],
     method="dHvb = 1.093 R Tb (ln Pc[bar] - 1.013)/(0.930 - Tbr)", latex=r"\Delta H_{vb} = 1.093\,R\,T_b\,\frac{\ln P_c - 1.013}{0.930 - T_{br}}",
     run=riedel, reference=lambda Tb, Tc, Pc: {"Hvap_b": __import__("chemicals.phase_change", fromlist=["Riedel"]).Riedel(Tb, Tc, Pc)},
     wrong=lambda Tb, Tc, Pc: _out(Hvap_b=1.093 * Tb * R_GAS * (math.log(Pc / 101325.0) - 1.013) / (0.93 - Tb / Tc)),
     rand=lambda rng: (lambda Tc: {"Tc": Tc, "Tb": Tc * rng.uniform(0.5, 0.75), "Pc": rng.uniform(1.5e6, 2.2e7)})(rng.uniform(150, 700)),
     phrases=("Riedel heat of vaporization", "Riedel heat of vaporisation", "Riedel equation"),
     examples=[_ex({"Tb": 294.0, "Tc": 466.0, "Pc": 5.55e6}), _ex({"Tb": 353.24, "Tc": 562.05, "Pc": 4.895e6})], level="intermediate")


def pitzer_hvap(T, Tc, omega):
    _pos("T and Tc", T, Tc)
    _fin(omega)
    _need(T < Tc, "T must be below the critical temperature")
    tr = 1.0 - T / Tc
    return _out(Hvap=R_GAS * Tc * (7.08 * tr ** 0.354 + 10.95 * omega * tr ** 0.456))


_add("chemeng_pitzer_heat_of_vaporization",
     name="Heat of vaporisation from the acentric factor (Pitzer)", topic="property estimation / heat of vaporisation",
     purpose="Heat of vaporisation at T from the critical temperature and acentric factor (Pitzer's corresponding-states correlation as fitted by Carruth and Kobayashi).",
     inputs=[_phys("T", "temperature", "temperature"), _phys("Tc", "temperature", "critical temperature"), _numin("omega", "acentric factor")],
     outputs=[_o("Hvap", "molar_energy", "heat of vaporisation")], primary="Hvap", conditions=["0.6 < Tr < 1 (fit range)"],
     method="dHv/(R Tc) = 7.08 (1 - Tr)^0.354 + 10.95 omega (1 - Tr)^0.456", latex=r"\frac{\Delta H_v}{RT_c} = 7.08(1-T_r)^{0.354} + 10.95\,\omega(1-T_r)^{0.456}",
     run=pitzer_hvap, reference=lambda T, Tc, omega: {"Hvap": __import__("chemicals.phase_change", fromlist=["Pitzer"]).Pitzer(T, Tc, omega)},
     wrong=lambda T, Tc, omega: _out(Hvap=R_GAS * T * (7.08 * (1 - T / Tc) ** 0.354 + 10.95 * omega * (1 - T / Tc) ** 0.456)),
     rand=lambda rng: (lambda Tc: {"Tc": Tc, "T": Tc * rng.uniform(0.6, 0.97), "omega": rng.uniform(0.0, 0.6)})(rng.uniform(150, 700)),
     phrases=("Pitzer heat of vaporization", "Pitzer heat of vaporisation"),
     examples=[_ex({"T": 452.0, "Tc": 645.6, "omega": 0.35}), _ex({"T": 300.0, "Tc": 425.12, "omega": 0.2})], level="intermediate")


def rackett(T, Tc, Pc, Zc):
    _pos("T, Tc, Pc and Zc", T, Tc, Pc, Zc)
    _need(T < Tc and Zc < 1, "T must be below Tc and Zc below 1")
    return _out(Vm=R_GAS * Tc / Pc * Zc ** (1.0 + (1.0 - T / Tc) ** (2.0 / 7.0)))


_add("chemeng_rackett_liquid_volume",
     name="Saturated liquid molar volume (Rackett)", topic="property estimation / liquid density",
     purpose="Molar volume of a saturated liquid from the critical constants and the critical compressibility factor (Rackett equation).",
     inputs=[_phys("T", "temperature", "temperature"), _phys("Tc", "temperature", "critical temperature"),
             _phys("Pc", "pressure", "critical pressure"), _numin("Zc", "critical compressibility factor")],
     outputs=[_o("Vm", "molar_volume", "saturated liquid molar volume")], primary="Vm", conditions=["T < Tc"],
     method="Vs = (R Tc/Pc) Zc^[1 + (1 - Tr)^(2/7)]", latex=r"V_s = \frac{RT_c}{P_c}Z_c^{\,1 + (1-T_r)^{2/7}}",
     run=rackett, reference=lambda T, Tc, Pc, Zc: {"Vm": __import__("chemicals.volume", fromlist=["Rackett"]).Rackett(T, Tc, Pc, Zc)},
     wrong=lambda T, Tc, Pc, Zc: _out(Vm=R_GAS * Tc / Pc * Zc ** ((1.0 - T / Tc) ** (2.0 / 7.0))),
     rand=lambda rng: (lambda Tc: {"Tc": Tc, "T": Tc * rng.uniform(0.4, 0.95), "Pc": rng.uniform(1.5e6, 8e6), "Zc": rng.uniform(0.22, 0.31)})(rng.uniform(150, 700)),
     phrases=("Rackett liquid volume", "Rackett equation", "Saturated liquid volume"),
     examples=[_ex({"T": 272.03889, "Tc": 369.83, "Pc": 4248000.0, "Zc": 0.2763}), _ex({"T": 298.15, "Tc": 562.05, "Pc": 4895000.0, "Zc": 0.268})],
     level="intermediate")


def abbott_virial(T, P, Tc, Pc, omega):
    _pos("T, P, Tc and Pc", T, P, Tc, Pc)
    _fin(omega)
    Tr = T / Tc
    B = (0.083 - 0.422 / Tr ** 1.6 + omega * (0.139 - 0.172 / Tr ** 4.2)) * R_GAS * Tc / Pc
    Z = 1.0 + B * P / (R_GAS * T)
    _need(Z > 0.5, "the truncated virial equation is not valid here (Z would fall below 0.5)")
    return _out(B=B, Z=Z)


_add("chemeng_virial_compressibility",
     name="Gas compressibility from the virial equation (Pitzer-Abbott correlation)", topic="property estimation / gas compressibility",
     purpose="Second virial coefficient from the Abbott (Pitzer) correlation and the compressibility factor Z = 1 + BP/(RT) of a gas at moderate pressure.",
     inputs=[_phys("T", "temperature", "temperature"), _phys("P", "pressure", "pressure"), _phys("Tc", "temperature", "critical temperature"),
             _phys("Pc", "pressure", "critical pressure"), _numin("omega", "acentric factor")],
     outputs=[_o("B", "molar_volume", "second virial coefficient"), _o("Z", "number", "compressibility factor")], primary="Z",
     conditions=["moderate pressure (truncated virial equation)", "non-polar or slightly polar gas"],
     method="B Pc/(R Tc) = B0 + omega B1, B0 = 0.083 - 0.422/Tr^1.6, B1 = 0.139 - 0.172/Tr^4.2; Z = 1 + B P/(R T)",
     latex=r"\frac{BP_c}{RT_c} = 0.083 - \frac{0.422}{T_r^{1.6}} + \omega\left(0.139 - \frac{0.172}{T_r^{4.2}}\right),\quad Z = 1 + \frac{BP}{RT}",
     run=abbott_virial,
     reference=lambda T, P, Tc, Pc, omega: (lambda B: {"B": B, "Z": 1.0 + B * P / (R_GAS * T)})(__import__("chemicals.virial", fromlist=["BVirial_Abbott"]).BVirial_Abbott(T, Tc, Pc, omega)),
     wrong=lambda T, P, Tc, Pc, omega: (lambda B: _out(B=B, Z=1 + B * P / (R_GAS * T)))((0.083 - 0.422 / (T / Tc) ** 1.6 + omega * (0.139 - 0.172 / (T / Tc) ** 4.2)) * R_GAS * T / Pc),
     rand=lambda rng: (lambda Tc, Pc: {"Tc": Tc, "Pc": Pc, "T": Tc * rng.uniform(0.8, 2.5), "P": Pc * _lu(rng, 0.005, 0.4), "omega": rng.uniform(0.0, 0.5)})(rng.uniform(150, 650), rng.uniform(2e6, 8e6)),
     phrases=("Virial compressibility", "Pitzer virial compressibility", "Abbott virial coefficient"),
     examples=[_ex({"T": 510.0, "P": 2.5e6, "Tc": 425.1, "Pc": 3.796e6, "omega": 0.2}), _ex({"T": 300.0, "P": 1e6, "Tc": 304.2, "Pc": 7.383e6, "omega": 0.224})],
     level="intermediate")


def mass_to_mole(w, MW):
    _need(2 <= len(w) == len(MW) <= 20, "give one molar mass per mass fraction (2 to 20 components)")
    _fracs("w", w)
    _pos("the molar masses", *MW)
    s = sum(a / b for a, b in zip(w, MW))
    z = [a / b / s for a, b in zip(w, MW)]
    return _out(z=z, MW_mean=1e-3 * sum(a * b for a, b in zip(z, MW)))


def _ref_mass_to_mole(w, MW):
    from chemicals.utils import ws_to_zs, mixing_simple
    z = ws_to_zs(list(w), list(MW))
    return {"z": list(z), "MW_mean": 1e-3 * mixing_simple(z, list(MW))}


_add("chemeng_mass_to_mole_fractions",
     name="Mass fractions to mole fractions and mean molar mass", topic="material balances / compositions",
     purpose="Mole fractions of a mixture and its mean molar mass from the mass fractions and the molar masses of the components.",
     inputs=[_vecin("w", "mass fractions (sum to 1)", 2, 20), _vecin("MW", "molar masses of the components, g/mol", 2, 20)],
     outputs=[_o("z", "number", "mole fractions"), _o("MW_mean", "molar_mass", "mean molar mass of the mixture")], primary="MW_mean",
     conditions=["fractions sum to 1", "positive molar masses"],
     method="z_i = (w_i/M_i)/sum_j (w_j/M_j); M = sum z_i M_i", latex=r"z_i = \frac{w_i/M_i}{\sum_j w_j/M_j},\quad \bar M = \sum_i z_iM_i",
     run=mass_to_mole, reference=_ref_mass_to_mole,
     wrong=lambda w, MW: (lambda s: _out(z=[a * b / s for a, b in zip(w, MW)], MW_mean=1e-3 * sum(a * b for a, b in zip(w, MW))))(sum(a * b for a, b in zip(w, MW))),
     rand=lambda rng: (lambda n: {"w": _dirichlet(rng, n), "MW": [rng.uniform(2, 300) for _ in range(n)]})(rng.randint(2, 6)),
     phrases=("Mass to mole fractions", "Mass fractions to mole fractions"),
     examples=[_ex({"w": [0.5, 0.5], "MW": [32.042, 18.015]}), _ex({"w": [0.2, 0.3, 0.5], "MW": [16.043, 30.07, 44.097]})],
     level="beginner")


# ================================================================================================ TRANSPORT AND PARTICLE BEDS
def stefan_flux(D, P, T, L, yA1, yA2):
    _pos("D, P, T and L", D, P, T, L)
    _frac("yA1", yA1, hi_open=True)
    _frac("yA2", yA2, hi_open=True)
    _need(yA1 > yA2, "A diffuses from yA1 to yA2: yA1 must exceed yA2")
    c = P / (R_GAS * T)
    return _out(NA=c * D / L * math.log((1.0 - yA2) / (1.0 - yA1)))


def _ref_stefan_flux(D, P, T, L, yA1, yA2):
    """Shooting: the film equation dy/dz = -N_A (1 - y)/(c D) (A through stagnant B) integrated across the film for a
    trial flux; brentq on the flux until the composition at z = L is yA2."""
    c = P / (R_GAS * T)
    y_end = lambda N: _ivp(lambda s, y: [-N * (1.0 - y[0]) / (c * D)], L, [yA1], atol=[1e-18])[0]
    hi = c * D / L
    while y_end(hi) > yA2:
        hi *= 2.0
    return {"NA": _brentq(lambda N: y_end(N) - yA2, 0.0, hi)}


_add("chemeng_stefan_diffusion_flux",
     name="Diffusion of A through stagnant B (Stefan film)", topic="transport / mass transfer",
     purpose="Steady molar flux of A diffusing through a stagnant film of non-diffusing B in an ideal gas (evaporation, absorption through an inert).",
     inputs=[_phys("D", "diffusion_coefficient", "diffusivity of A in B"), _phys("P", "pressure", "total pressure"),
             _phys("T", "temperature", "temperature"), _phys("L", "length", "film thickness (diffusion path length)"),
             _numin("yA1", "mole fraction of A at the start of the path", pct=True), _numin("yA2", "mole fraction of A at the end of the path", pct=True)],
     outputs=[_o("NA", "molar_flux", "molar flux of A")], primary="NA",
     conditions=["steady state, ideal gas, constant D", "B stagnant (no net flux of B)"],
     method="N_A = (c D/L) ln[(1 - yA2)/(1 - yA1)], c = P/(R T)", latex=r"N_A = \frac{cD_{AB}}{L}\ln\frac{1 - y_{A2}}{1 - y_{A1}},\quad c = \frac{P}{RT}",
     run=stefan_flux, reference=_ref_stefan_flux,
     wrong=lambda D, P, T, L, yA1, yA2: _out(NA=P / (R_GAS * T) * D / L * (yA1 - yA2)),
     rand=lambda rng: (lambda y1: {"D": _lu(rng, 5e-6, 5e-5), "P": rng.uniform(5e4, 3e5), "T": rng.uniform(280, 400), "L": _lu(rng, 1e-3, 0.2),
                                   "yA1": y1, "yA2": y1 * rng.uniform(0.0, 0.8)})(rng.uniform(0.05, 0.9)),
     phrases=("Stefan diffusion flux", "Diffusion through stagnant film", "Diffusion through stagnant B"),
     examples=[_ex({"D": 2.6e-5, "P": 101325.0, "T": 298.15, "L": 0.1, "yA1": 0.0313, "yA2": 0.0}),
               _ex({"D": 1.0e-5, "P": 200000.0, "T": 350.0, "L": 0.005, "yA1": 0.6, "yA2": 0.2})],
     level="advanced",
     extra_q=[{"q": "Stefan diffusion flux: D = 0.26 cm^2/s, P = 1 atm, T = 25 degC, L = 10 cm, yA1 = 3.13%, yA2 = 0", "like": 0}])


_BED_IN = [_phys("dp", "length", "particle diameter"), _numin("eps", "bed voidage (porosity)"),
           _phys("vs", "speed", "superficial velocity"), _phys("rho", "density", "fluid density"),
           _phys("mu", "dynamic_viscosity", "fluid viscosity"), _phys("L", "length", "bed depth")]


def _bed_check(dp, eps, vs, rho, mu, L):
    _pos("dp, vs, rho, mu and L", dp, vs, rho, mu, L)
    _frac("eps", eps, lo_open=True, hi_open=True)


def ergun(dp, eps, vs, rho, mu, L):
    _bed_check(dp, eps, vs, rho, mu, L)
    Re = dp * rho * vs / mu
    fp = (150.0 + 1.75 * Re / (1.0 - eps)) * (1.0 - eps) ** 2 / (eps ** 3 * Re)
    return _out(dP=fp * rho * vs * vs * L / dp)


def carman(dp, eps, vs, rho, mu, L):
    _bed_check(dp, eps, vs, rho, mu, L)
    Re = dp * rho * vs / mu
    fp = (180.0 + 2.871 * (Re / (1.0 - eps)) ** 0.9) * (1.0 - eps) ** 2 / (eps ** 3 * Re)
    return _out(dP=fp * rho * vs * vs * L / dp)


def _rand_bed(rng):
    return {"dp": _lu(rng, 1e-4, 0.02), "eps": rng.uniform(0.3, 0.6), "vs": _lu(rng, 1e-3, 2.0), "rho": rng.choice([rng.uniform(0.5, 10), rng.uniform(700, 1500)]),
            "mu": _lu(rng, 1e-5, 0.05), "L": rng.uniform(0.1, 5.0)}


_add("chemeng_ergun_pressure_drop",
     name="Packed-bed pressure drop (Ergun equation)", topic="transport / packed beds",
     purpose="Pressure drop of a fluid flowing through a packed bed of spheres from the particle size, voidage, superficial velocity and fluid properties (Ergun equation).",
     inputs=_BED_IN, outputs=[_o("dP", "pressure", "pressure drop across the bed")], primary="dP",
     conditions=["0 < eps < 1", "incompressible flow (or small pressure drop for a gas)", "spherical or equivalent particles"],
     method="dP/L = 150 mu (1 - eps)^2 vs/(eps^3 dp^2) + 1.75 (1 - eps) rho vs^2/(eps^3 dp)",
     latex=r"\frac{\Delta P}{L} = \frac{150\,\mu(1-\varepsilon)^2 v_s}{\varepsilon^3 d_p^2} + \frac{1.75\,(1-\varepsilon)\rho v_s^2}{\varepsilon^3 d_p}",
     run=ergun, reference=lambda dp, eps, vs, rho, mu, L: {"dP": __import__("fluids.packed_bed", fromlist=["Ergun"]).Ergun(dp=dp, voidage=eps, vs=vs, rho=rho, mu=mu, L=L)},
     wrong=lambda dp, eps, vs, rho, mu, L: _out(dP=L * (150 * mu * (1 - eps) ** 2 * vs / (eps ** 3 * dp ** 2) + 1.75 * rho * vs * vs / (eps ** 3 * dp))),
     rand=_rand_bed,
     phrases=("Ergun pressure drop", "Packed bed pressure drop Ergun", "Ergun equation"),
     examples=[_ex({"dp": 0.0008, "eps": 0.4, "vs": 0.05, "rho": 1000.0, "mu": 0.001, "L": 1.0}),
               _ex({"dp": 0.005, "eps": 0.38, "vs": 1.2, "rho": 1.2, "mu": 1.8e-5, "L": 2.0})],
     level="intermediate",
     extra_q=[{"q": "Ergun pressure drop: dp = 0.8 mm, eps = 0.4, vs = 5 cm/s, rho = 1000 kg/m^3, mu = 1 cP, L = 1 m", "like": 0}])

_add("chemeng_carman_pressure_drop",
     name="Packed-bed pressure drop (Carman correlation)", topic="transport / packed beds",
     purpose="Pressure drop through a packed bed from Carman's correlation of the bed friction factor (180/Re' + 2.87/Re'^0.1 form).",
     inputs=_BED_IN, outputs=[_o("dP", "pressure", "pressure drop across the bed")], primary="dP",
     conditions=["0 < eps < 1", "incompressible flow"],
     method="f_p = (180 + 2.871 (Re/(1 - eps))^0.9)(1 - eps)^2/(eps^3 Re); dP = f_p rho vs^2 L/dp, Re = dp rho vs/mu",
     latex=r"f_p = \frac{\left(180 + 2.871\left(\frac{Re}{1-\varepsilon}\right)^{0.9}\right)(1-\varepsilon)^2}{\varepsilon^3 Re},\quad \Delta P = \frac{f_p\,\rho v_s^2 L}{d_p}",
     run=carman, reference=lambda dp, eps, vs, rho, mu, L: {"dP": __import__("fluids.packed_bed", fromlist=["Carman"]).Carman(dp=dp, voidage=eps, vs=vs, rho=rho, mu=mu, L=L)},
     wrong=lambda dp, eps, vs, rho, mu, L: ergun(dp, eps, vs, rho, mu, L),
     rand=_rand_bed,
     phrases=("Carman pressure drop", "Packed bed pressure drop Carman"),
     examples=[_ex({"dp": 0.0008, "eps": 0.4, "vs": 0.05, "rho": 1000.0, "mu": 0.001, "L": 1.0}),
               _ex({"dp": 0.005, "eps": 0.38, "vs": 1.2, "rho": 1.2, "mu": 1.8e-5, "L": 2.0})],
     level="intermediate")


def filtration_volume(A, dP, mu, alpha, c, Rm, t):
    _pos("A, dP, mu, alpha, c and Rm", A, dP, mu, alpha, c, Rm)
    _nonneg("t", t)
    a = mu * alpha * c / (2.0 * A * A * dP)
    b = mu * Rm / (A * dP)
    return _out(V=2.0 * t / (b + math.sqrt(b * b + 4.0 * a * t)))


_add("chemeng_cake_filtration_volume",
     name="Constant-pressure cake filtration: filtrate volume after a time", topic="transport / filtration",
     purpose="Volume of filtrate collected after time t in constant-pressure cake filtration (Ruth equation with cake and medium resistances).",
     inputs=[_phys("A", "area", "filter area"), _phys("dP", "pressure", "pressure difference across cake and medium"),
             _phys("mu", "dynamic_viscosity", "filtrate viscosity"), _phys("alpha", "specific_cake_resistance", "specific cake resistance"),
             _phys("c", "mass_concentration", "mass of dry cake deposited per volume of filtrate"),
             _phys("Rm", "filter_medium_resistance", "filter medium resistance"), _phys("t", "time", "filtration time")],
     outputs=[_o("V", "volume", "filtrate volume collected")], primary="V",
     conditions=["constant pressure, incompressible cake", "Rm > 0"],
     method="dV/dt = A^2 dP/(mu (alpha c V + A Rm)), integrated: t = mu alpha c V^2/(2 A^2 dP) + mu Rm V/(A dP), solved for V",
     latex=r"t = \frac{\mu\alpha c}{2A^2\Delta P}V^2 + \frac{\mu R_m}{A\,\Delta P}V",
     run=filtration_volume,
     reference=lambda A, dP, mu, alpha, c, Rm, t: {"V": _ivp(lambda s, y: [A * A * dP / (mu * (alpha * c * y[0] + A * Rm))], t, [0.0], atol=[1e-20])[0]},
     wrong=lambda A, dP, mu, alpha, c, Rm, t: _out(V=math.sqrt(2.0 * A * A * dP * t / (mu * alpha * c))),
     rand=lambda rng: {"A": _lu(rng, 0.05, 20), "dP": _lu(rng, 2e4, 5e5), "mu": _lu(rng, 5e-4, 5e-3), "alpha": _lu(rng, 1e9, 1e12),
                       "c": _lu(rng, 2, 100), "Rm": _lu(rng, 1e9, 1e11), "t": _lu(rng, 10, 1e4)},
     phrases=("Cake filtration volume", "Filtrate volume", "Constant-pressure filtration volume"),
     examples=[_ex({"A": 1.0, "dP": 100000.0, "mu": 0.001, "alpha": 1e11, "c": 20.0, "Rm": 1e10, "t": 600.0}),
               _ex({"A": 10.0, "dP": 300000.0, "mu": 0.0012, "alpha": 5e10, "c": 50.0, "Rm": 5e10, "t": 3600.0})],
     level="intermediate")


def filtration_fit(t, V, t_unit, v_unit, A, dP, mu, c):
    _need(len(t) == len(V) >= 3, "t and V need the same number of points (at least 3)")
    _pos("A, dP, mu and c", A, dP, mu, c)
    ts = [x * {"s": 1.0, "min": 60.0, "h": 3600.0}[t_unit] for x in t]
    vs = [x * {"m3": 1.0, "L": 1e-3}[v_unit] for x in V]
    _need(all(v > 0 for v in vs) and all(x > 0 for x in ts), "times and volumes must be positive")
    yv = [a / b for a, b in zip(ts, vs)]
    n = len(vs)
    mx, my = sum(vs) / n, sum(yv) / n
    sxx = sum((x - mx) ** 2 for x in vs)
    _need(sxx > 0, "the volumes must not all be equal")
    slope = sum((x - mx) * (y - my) for x, y in zip(vs, yv)) / sxx
    icpt = my - slope * mx
    _need(slope > 0 and icpt > 0, "the data do not give a positive slope and intercept of t/V against V")
    return _out(alpha=2.0 * slope * A * A * dP / (mu * c), Rm=icpt * A * dP / mu, slope=slope, intercept=icpt)


def _ref_filtration_fit(t, V, t_unit, v_unit, A, dP, mu, c):
    import numpy as np
    from scipy.stats import linregress
    ts = np.array(t, float) * {"s": 1.0, "min": 60.0, "h": 3600.0}[t_unit]
    vs = np.array(V, float) * {"m3": 1.0, "L": 1e-3}[v_unit]
    r = linregress(vs, ts / vs)
    return {"alpha": 2.0 * float(r.slope) * A * A * dP / (mu * c), "Rm": float(r.intercept) * A * dP / mu,
            "slope": float(r.slope), "intercept": float(r.intercept)}


def _rand_filtration(rng):
    A, dP, mu, c = _lu(rng, 0.05, 5), _lu(rng, 2e4, 5e5), _lu(rng, 5e-4, 5e-3), _lu(rng, 2, 100)
    alpha, Rm = _lu(rng, 1e10, 1e12), _lu(rng, 1e9, 1e11)
    a, b = mu * alpha * c / (2 * A * A * dP), mu * Rm / (A * dP)
    vmax = _lu(rng, 0.005, 0.5) * A
    n = rng.randint(4, 12)
    vu = rng.choice(["m3", "L"])
    tu = rng.choice(["s", "min"])
    Vs = sorted(vmax * (i + 1) / n for i in range(n))
    ts = [(a * v * v + b * v) * (1 + rng.uniform(-0.01, 0.01)) for v in Vs]
    return {"t": [_sig(x / {"s": 1.0, "min": 60.0}[tu], 5) for x in ts], "V": [_sig(v / {"m3": 1.0, "L": 1e-3}[vu], 5) for v in Vs],
            "t_unit": tu, "v_unit": vu, "A": A, "dP": dP, "mu": mu, "c": c}


_add("chemeng_filtration_constants_fit",
     name="Specific cake and medium resistance from constant-pressure filtration data", topic="transport / filtration",
     purpose="Specific cake resistance and filter-medium resistance from a constant-pressure filtration test (times and filtrate volumes), by the straight line of t/V against V.",
     inputs=[_vecin("t", "times at which the filtrate volumes were read, in t_unit", 3, 100), _vecin("V", "cumulative filtrate volumes, in v_unit", 3, 100),
             _choice("t_unit", ["s", "min", "h"], "unit of the times"), _choice("v_unit", ["m3", "L"], "unit of the volumes"),
             _phys("A", "area", "filter area"), _phys("dP", "pressure", "constant pressure difference"),
             _phys("mu", "dynamic_viscosity", "filtrate viscosity"), _phys("c", "mass_concentration", "dry cake mass per volume of filtrate")],
     outputs=[_o("alpha", "specific_cake_resistance", "specific cake resistance"), _o("Rm", "filter_medium_resistance", "filter medium resistance"),
              _o("slope", "number", "slope of t/V against V, s/m^6"), _o("intercept", "number", "intercept of t/V against V, s/m^3")],
     primary="alpha", conditions=["constant pressure, incompressible cake", "a positive slope and intercept"],
     method="least squares of t/V = (mu alpha c/(2 A^2 dP)) V + mu Rm/(A dP)",
     latex=r"\frac{t}{V} = \frac{\mu\alpha c}{2A^2\Delta P}V + \frac{\mu R_m}{A\Delta P}",
     run=filtration_fit, reference=_ref_filtration_fit,
     wrong=lambda t, V, t_unit, v_unit, A, dP, mu, c: (lambda r: r | _out(alpha=r["slope"] * A * A * dP / (mu * c)))(filtration_fit(t, V, t_unit, v_unit, A, dP, mu, c)),
     rand=_rand_filtration,
     phrases=("Filtration constants fit", "Cake resistance from filtration data", "Filtration test analysis"),
     examples=[_ex({"t": [17.3, 41.3, 72.0, 108.3, 152.1, 201.7], "V": [0.5, 1.0, 1.5, 2.0, 2.5, 3.0], "t_unit": "s", "v_unit": "L",
                    "A": 0.0439, "dP": 338000.0, "mu": 0.000895, "c": 23.47}),
               _ex({"t": [2.0, 5.0, 9.5, 15.0, 21.5], "V": [0.1, 0.2, 0.3, 0.4, 0.5], "t_unit": "min", "v_unit": "m3",
                    "A": 2.0, "dP": 150000.0, "mu": 0.001, "c": 30.0})],
     level="advanced")


# ================================================================================================ PROCESS CONTROL
_TU = {"s": 1.0, "min": 60.0, "h": 3600.0}


def fopdt_step(K, tau, theta, M, t):
    _fin(K, M)
    _pos("tau", tau)
    _nonneg("theta and t", theta, t)
    y = 0.0 if t <= theta else K * M * -math.expm1(-(t - theta) / tau)
    return _out(y=y)


def _ref_fopdt_step(K, tau, theta, M, t):
    import control
    if t <= theta:
        return {"y": 0.0}
    resp = control.step_response(control.tf([K], [tau, 1.0]), T=[0.0, t - theta])
    return {"y": float(resp.outputs[-1]) * M}


_add("chemeng_fopdt_step_response",
     name="First-order-plus-dead-time process: response to a step", topic="process control / process dynamics",
     purpose="Deviation of the output of a first-order process with dead time at time t after a step of size M in its input.",
     inputs=[_numin("K", "process gain (output change per unit input change)"), _phys("tau", "time", "process time constant"),
             _phys("theta", "time", "dead time"), _numin("M", "size of the input step (deviation)"), _phys("t", "time", "time after the step")],
     outputs=[_o("y", "number", "output deviation at time t (in the output's units)")], primary="y",
     conditions=["the process starts at steady state", "linear first-order dynamics with dead time"],
     method="y(t) = K M (1 - exp(-(t - theta)/tau)) for t > theta, 0 before",
     latex=r"y(t) = KM\left(1 - e^{-(t-\theta)/\tau}\right)\,u(t-\theta)",
     run=fopdt_step, reference=_ref_fopdt_step,
     wrong=lambda K, tau, theta, M, t: _out(y=K * M * -math.expm1(-t / tau)),
     rand=lambda rng: (lambda tau, th: {"K": rng.choice([-1, 1]) * _lu(rng, 0.1, 10), "tau": tau, "theta": th, "M": rng.uniform(0.5, 20),
                                        "t": th + tau * _lu(rng, 0.05, 5.0)})(_lu(rng, 1, 1000), _lu(rng, 0.5, 300)),
     phrases=("FOPDT step response", "First-order plus dead time step response", "First-order process step response"),
     examples=[_ex({"K": 2.0, "tau": 300.0, "theta": 60.0, "M": 5.0, "t": 360.0}), _ex({"K": -0.8, "tau": 120.0, "theta": 30.0, "M": 10.0, "t": 600.0})],
     level="beginner",
     extra_q=[{"q": "FOPDT step response: K = 2, tau = 5 min, theta = 1 min, M = 5, t = 6 min", "like": 0}])


def _poly_mul(a, b):
    out = [0.0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def _lags_poly(taus):
    p = [1.0]
    for t in taus:
        p = _poly_mul(p, [t, 1.0])
    return p


def _routh_stable(c):
    """True when every root of c[0] s^n + ... + c[n] has a negative real part (Routh array); a zero in the first
    column (a marginal or singular case) is refused."""
    c = list(c)
    if c[0] < 0:
        c = [-x for x in c]
    n = len(c) - 1
    rows = [c[0::2], c[1::2] + [0.0] * (len(c[0::2]) - len(c[1::2]))]
    for _ in range(n - 1):
        a, b = rows[-2], rows[-1]
        _need(abs(b[0]) > 1e-12 * max(abs(x) for x in c), "the Routh array has a zero in its first column (marginal case)")
        rows.append([(b[0] * a[i + 1] - a[0] * b[i + 1]) / b[0] for i in range(len(a) - 1)] + [0.0])
    first = [r[0] for r in rows[:n + 1]]
    _need(all(abs(x) > 1e-12 * max(abs(y) for y in c) for x in first), "the Routh array has a zero in its first column (marginal case)")
    return all(x > 0 for x in first)


def p_offset(Kc, K, taus, t_unit):
    _fin(Kc, K)
    _need(Kc * K > 0, "the loop gain Kc K must be positive (negative feedback)")
    _need(1 <= len(taus) <= 6, "give 1 to 6 time constants")
    _pos("the time constants", *taus)
    char = _lags_poly([t * _TU[t_unit] for t in taus])
    char[-1] += Kc * K
    _need(_routh_stable(char), "the closed loop is unstable at this controller gain: there is no steady state")
    y = Kc * K / (1.0 + Kc * K)
    return _out(y_ss=y, offset=1.0 / (1.0 + Kc * K))


def _ref_p_offset(Kc, K, taus, t_unit):
    import control
    G = control.tf([K], _lags_poly([t * _TU[t_unit] for t in taus]))
    T = control.feedback(Kc * G, 1)
    y = float(T.dcgain())
    return {"y_ss": y, "offset": 1.0 - y}


_LAGS_IN = [_numin("K", "process gain"), _vecin("taus", "time constants of the first-order lags in series, in t_unit", 1, 6),
            _choice("t_unit", ["s", "min", "h"], "unit of the time constants")]

_add("chemeng_p_control_offset",
     name="Proportional control: steady-state offset after a set-point step", topic="process control / feedback control",
     purpose="Steady-state output (as a fraction of a set-point step) and the offset of a P-only feedback loop around a process of first-order lags in series, after checking the loop is stable.",
     inputs=[_numin("Kc", "controller gain")] + _LAGS_IN,
     outputs=[_o("y_ss", "number", "final output change per unit set-point change"), _o("offset", "number", "steady-state offset per unit set-point change")],
     primary="offset", conditions=["Kc K > 0", "the closed loop is stable (checked by the Routh array)"],
     method="final-value theorem: y_ss = Kc K/(1 + Kc K), offset = 1/(1 + Kc K); stability of prod(tau_i s + 1) + Kc K by the Routh array",
     latex=r"\text{offset} = \frac{1}{1 + K_cK}",
     run=p_offset, reference=_ref_p_offset,
     wrong=lambda Kc, K, taus, t_unit: _out(y_ss=1.0, offset=0.0),
     rand=lambda rng: (lambda taus: {"Kc": _lu(rng, 0.1, 5), "K": _lu(rng, 0.2, 3), "taus": taus, "t_unit": rng.choice(["s", "min"])})(
         [_sig(_lu(rng, 1, 60)) for _ in range(rng.randint(1, 3))]),
     phrases=("P-only control offset", "Proportional control offset", "Offset with proportional control"),
     examples=[_ex({"Kc": 2.0, "K": 1.5, "taus": [5.0, 2.0], "t_unit": "min"}), _ex({"Kc": 0.5, "K": 2.0, "taus": [10.0, 5.0, 1.0], "t_unit": "s"})],
     level="beginner",
     extra_q=[{"q": "P-only control offset: Kc = 20, K = 2, taus = [10, 10, 10], t_unit = s", "outcome": "INVALID_INPUT"}])


def lag_freq(K, taus, t_unit, w):
    _fin(K)
    _need(K > 0, "the process gain must be positive")
    _need(1 <= len(taus) <= 3, "give 1 to 3 time constants")
    _pos("the time constants and the frequency", *taus, w)
    ts = [t * _TU[t_unit] for t in taus]
    ar = K
    for t in ts:
        ar /= math.sqrt(1.0 + (w * t) ** 2)
    return _out(AR=ar, phase_deg=-sum(math.degrees(math.atan(w * t)) for t in ts))


def _ref_lag_freq(K, taus, t_unit, w):
    import control
    G = control.tf([K], _lags_poly([t * _TU[t_unit] for t in taus]))
    g = complex(G(1j * w))
    ph = math.degrees(math.atan2(g.imag, g.real))
    while ph > 1e-9:                       # true phase lies in (-270, 0] for up to three lags: one unwrapping branch
        ph -= 360.0
    return {"AR": abs(g), "phase_deg": ph}


_add("chemeng_lag_frequency_response",
     name="Frequency response of first-order lags in series (amplitude ratio and phase)", topic="process control / frequency response",
     purpose="Amplitude ratio and phase angle of a process made of one to three first-order lags in series, at angular frequency w.",
     inputs=_LAGS_IN[:1] + [_vecin("taus", "time constants of the lags, in t_unit", 1, 3), _LAGS_IN[2],
                            _phys("w", "angular_frequency", "angular frequency of the input sinusoid")],
     outputs=[_o("AR", "number", "amplitude ratio (output amplitude per input amplitude)"), _o("phase_deg", "number", "phase angle in degrees (negative: lag)")],
     primary="AR", conditions=["K > 0", "linear lags without dead time"],
     method="AR = K/prod sqrt(1 + w^2 tau_i^2); phase = -sum atan(w tau_i)",
     latex=r"AR = \frac{K}{\prod_i\sqrt{1 + \omega^2\tau_i^2}},\quad \varphi = -\sum_i \arctan(\omega\tau_i)",
     run=lag_freq, reference=_ref_lag_freq,
     wrong=lambda K, taus, t_unit, w: (lambda ts: _out(AR=K / math.prod(1 + w * t for t in ts), phase_deg=-sum(math.degrees(math.atan(w * t)) for t in ts)))([t * _TU[t_unit] for t in taus]),
     rand=lambda rng: (lambda taus, tu: {"K": _lu(rng, 0.1, 10), "taus": taus, "t_unit": tu, "w": _lu(rng, 0.05, 20) / (max(taus) * _TU[tu])})(
         [_sig(_lu(rng, 0.5, 100)) for _ in range(rng.randint(1, 3))], rng.choice(["s", "min"])),
     phrases=("Lag frequency response", "Frequency response of lags", "Amplitude ratio and phase"),
     examples=[_ex({"K": 2.0, "taus": [10.0], "t_unit": "s", "w": 0.1}), _ex({"K": 1.0, "taus": [5.0, 2.0, 1.0], "t_unit": "min", "w": 0.01})],
     level="intermediate")


def pid_position(Kc, tauI, tauD, dt, e, bias):
    _fin(Kc, bias)
    _pos("tauI and dt", tauI, dt)
    _nonneg("tauD", tauD)
    _need(1 <= len(e) <= 500, "give 1 to 500 error samples")
    u, s, prev = [], 0.0, 0.0
    for ek in e:
        s += ek
        u.append(bias + Kc * (ek + dt / tauI * s + tauD / dt * (ek - prev)))
        prev = ek
    return _out(u_last=u[-1], u=u)


def _ref_pid_position(Kc, tauI, tauD, dt, e, bias):
    """python-control: the controller as a discrete transfer function C(z) = Kc [1 + (dt/tauI) z/(z - 1) +
    (tauD/dt)(z - 1)/z], driven by the error sequence from rest."""
    import numpy as np
    import control
    num = [Kc * (1.0 + dt / tauI + tauD / dt), -Kc * (1.0 + 2.0 * tauD / dt), Kc * tauD / dt]
    C = control.tf(num, [1.0, -1.0, 0.0], dt)
    r = control.forced_response(C, T=np.arange(len(e)) * dt, U=np.array(e, float))
    u = [bias + float(v) for v in np.ravel(r.outputs)]
    return {"u_last": u[-1], "u": u}


_add("chemeng_pid_controller_output",
     name="Discrete PID controller output for an error history", topic="process control / PID control",
     purpose="Output of a digital position-form PID controller at each sample for a stated history of errors (rectangular integration, derivative on the error, loop at rest before the first sample).",
     inputs=[_numin("Kc", "controller gain"), _phys("tauI", "time", "integral (reset) time"), _phys("tauD", "time", "derivative time (0 for PI)"),
             _phys("dt", "time", "sampling interval"), _vecin("e", "error samples e_1 ... e_n (set point minus measurement)", 1, 500),
             _numin("bias", "controller output bias (output when the error has always been zero)")],
     outputs=[_o("u_last", "number", "controller output at the last sample"), _o("u", "number", "controller output at every sample")],
     primary="u_last", conditions=["position form, e_0 = 0 before the first sample", "rectangular integration including the current sample"],
     method="u_k = bias + Kc [e_k + (dt/tauI) sum_(j<=k) e_j + (tauD/dt)(e_k - e_(k-1))]",
     latex=r"u_k = \bar u + K_c\left[e_k + \frac{\Delta t}{\tau_I}\sum_{j=1}^{k} e_j + \frac{\tau_D}{\Delta t}(e_k - e_{k-1})\right]",
     run=pid_position, reference=_ref_pid_position,
     wrong=lambda Kc, tauI, tauD, dt, e, bias: pid_position(Kc, dt * dt / tauI, tauD, dt, e, bias),
     rand=lambda rng: (lambda n: {"Kc": _lu(rng, 0.2, 5), "tauI": _lu(rng, 5, 300), "tauD": rng.choice([0.0, _lu(rng, 0.5, 30)]),
                                  "dt": rng.choice([0.5, 1.0, 2.0, 5.0]), "e": [_sig(rng.uniform(-1, 4), 3) for _ in range(n)], "bias": rng.uniform(20, 80)})(rng.randint(3, 40)),
     phrases=("PID controller output", "Discrete PID output", "Digital PID controller output"),
     examples=[_ex({"Kc": 2.0, "tauI": 10.0, "tauD": 1.0, "dt": 1.0, "e": [1.0, 0.8, 0.5, 0.3], "bias": 50.0}),
               _ex({"Kc": 0.5, "tauI": 60.0, "tauD": 0.0, "dt": 2.0, "e": [2.0, 2.0, 1.5, 1.0, 0.5], "bias": 40.0})],
     level="intermediate")


def _ultimate(K, ts):
    _need(len(ts) >= 3, "at least three lags are needed for a finite ultimate gain")
    f = lambda w: sum(math.atan(w * t) for t in ts) - math.pi
    hi = 1.0 / min(ts)
    while f(hi) < 0:
        hi *= 2.0
    wu = _bisect(f, 0.0, hi)
    ku = math.prod(math.sqrt(1.0 + (wu * t) ** 2) for t in ts) / K
    return ku, 2.0 * math.pi / wu


def ziegler_nichols(K, taus, t_unit, controller):
    _fin(K)
    _need(K > 0, "the process gain must be positive")
    _need(3 <= len(taus) <= 6, "give 3 to 6 time constants")
    _pos("the time constants", *taus)
    ku, pu = _ultimate(K, [t * _TU[t_unit] for t in taus])
    return _zn(ku, pu, controller)


def _zn(ku, pu, controller):
    if controller == "P":
        return _out(Ku=ku, Pu=pu, Kc=0.5 * ku)
    if controller == "PI":
        return _out(Ku=ku, Pu=pu, Kc=0.45 * ku, tauI=pu / 1.2)
    return _out(Ku=ku, Pu=pu, Kc=0.6 * ku, tauI=pu / 2.0, tauD=pu / 8.0)


def _ref_ziegler_nichols(K, taus, t_unit, controller):
    import control
    G = control.tf([K], _lags_poly([t * _TU[t_unit] for t in taus]))
    gm, pm, wpc, wgc = control.margin(G)
    return _zn(float(gm), 2.0 * math.pi / float(wpc), controller)


_add("chemeng_ziegler_nichols_tuning",
     name="Ziegler-Nichols closed-loop tuning of lags in series", topic="process control / controller tuning",
     purpose="Ultimate gain and period of a process of first-order lags in series, and the Ziegler-Nichols closed-loop settings of a P, PI or PID controller.",
     inputs=[_numin("K", "process gain"), _vecin("taus", "time constants of the lags (three or more), in t_unit", 3, 6), _LAGS_IN[2],
             _choice("controller", ["P", "PI", "PID"], "controller type: P, PI or PID")],
     outputs=[_o("Ku", "number", "ultimate gain"), _o("Pu", "time", "ultimate period"), _o("Kc", "number", "controller gain"),
              _o("tauI", "time", "integral time (PI, PID)"), _o("tauD", "time", "derivative time (PID)")],
     primary="Ku", conditions=["three or more lags (a finite ultimate gain)", "K > 0"],
     method="phase crossover sum atan(w tau_i) = pi (bisection); Ku = 1/|G(j wu)|, Pu = 2 pi/wu; P: 0.5 Ku; PI: 0.45 Ku, Pu/1.2; PID: 0.6 Ku, Pu/2, Pu/8",
     latex=r"\sum_i\arctan(\omega_u\tau_i) = \pi,\quad K_u = \frac{1}{|G(j\omega_u)|},\quad P_u = \frac{2\pi}{\omega_u}",
     run=ziegler_nichols, reference=_ref_ziegler_nichols,
     wrong=lambda K, taus, t_unit, controller: (lambda r: r | _out(Pu=r["Pu"] / (2 * math.pi)))(ziegler_nichols(K, taus, t_unit, controller)),
     rand=lambda rng: {"K": _lu(rng, 0.2, 5), "taus": [_sig(_lu(rng, 0.5, 60)) for _ in range(rng.randint(3, 5))], "t_unit": rng.choice(["s", "min"]),
                       "controller": rng.choice(["P", "PI", "PID"])},
     phrases=("Ziegler-Nichols tuning", "Ziegler-Nichols closed-loop tuning"),
     examples=[_ex({"K": 2.0, "taus": [1.0, 1.0, 1.0], "t_unit": "min", "controller": "PID"}), _ex({"K": 1.5, "taus": [10.0, 5.0, 2.0], "t_unit": "s", "controller": "PI"})],
     level="advanced",
     extra_q=[{"q": "Ziegler-Nichols tuning: K = 2, taus = [1, 1], t_unit = min, controller = PID", "outcome": "INVALID_INPUT"}])


def _roots(c):
    import mpmath
    with mpmath.workdps(30):
        rs = mpmath.polyroots([mpmath.mpf(repr(float(x))) for x in c], maxsteps=400, extraprec=80)
    return [complex(r) for r in (rs if isinstance(rs, list) else [rs])]


def closed_loop_stability(num, den, Kc):
    _fin(Kc, *num, *den)
    while len(den) > 1 and den[0] == 0:
        den = den[1:]
    while len(num) > 1 and num[0] == 0:
        num = num[1:]
    _need(2 <= len(den) <= 9, "the denominator must have degree 1 to 8")
    _need(len(num) <= len(den), "the process must be proper (numerator degree <= denominator degree)")
    char = list(den)
    for i, v in enumerate(num):
        char[len(den) - len(num) + i] += Kc * v
    _need(char[0] != 0, "the closed-loop characteristic polynomial loses its leading term")
    rs = _roots(char)
    scale = max(math.hypot(r.real, r.imag) for r in rs)
    for i in range(len(rs)):
        for j in range(i + 1, len(rs)):
            _need(math.hypot(rs[i].real - rs[j].real, rs[i].imag - rs[j].imag) > 1e-3 * scale, "closed-loop poles nearly repeated: refused")
    mr = max(r.real for r in rs)
    _need(abs(mr) > 1e-6 * scale, "a closed-loop pole lies on the imaginary axis (marginal stability)")
    n_rhp = sum(1 for r in rs if r.real > 0)
    return _out(max_real=mr, n_rhp=n_rhp, stable="stable" if n_rhp == 0 else "unstable")


def _ref_closed_loop_stability(num, den, Kc):
    import numpy as np
    import control
    T = control.feedback(Kc * control.tf(list(num), list(den)), 1)
    p = np.asarray(T.poles())
    mr = float(np.max(p.real))
    n = int(np.sum(p.real > 0))
    return {"max_real": mr, "n_rhp": n, "stable": "stable" if n == 0 else "unstable"}


def _rand_cls(rng):
    for _ in range(200):
        n = rng.randint(2, 5)
        poles = []
        while len(poles) < n:
            if n - len(poles) >= 2 and rng.random() < 0.3:
                a, b = -_lu(rng, 0.05, 5), _lu(rng, 0.1, 5)
                poles += [complex(a, b), complex(a, -b)]
            else:
                poles.append(complex(-_lu(rng, 0.05, 5), 0))
        den = [1.0 + 0j]
        for p in poles:
            den = _poly_mul_c(den, [1.0, -p])
        den = [_sig(float(x.real), 6) for x in den]
        num = [_sig(rng.uniform(0.5, 3), 4)] if rng.random() < 0.6 else [_sig(rng.uniform(0.5, 3), 4), _sig(rng.uniform(0.5, 3), 4)]
        x = {"num": num, "den": den, "Kc": _sig(_lu(rng, 0.05, 50), 4)}
        try:
            closed_loop_stability(**x)
            return x
        except ValueError:
            continue


def _poly_mul_c(a, b):
    out = [0j] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


_add("chemeng_closed_loop_stability",
     name="Closed-loop stability of a P-controlled process (closed-loop poles)", topic="process control / stability",
     purpose="Whether a feedback loop with a proportional controller Kc around a process G(s) = num(s)/den(s) is stable: the largest real part of the closed-loop poles and how many lie in the right half-plane.",
     inputs=[_vecin("num", "numerator coefficients of G(s), highest power of s first", 1, 9),
             _vecin("den", "denominator coefficients of G(s), highest power of s first", 2, 9), _numin("Kc", "controller gain")],
     outputs=[_o("max_real", "number", "largest real part of the closed-loop poles, 1/(time unit of the coefficients)"),
              _o("n_rhp", "number", "number of closed-loop poles in the right half-plane"), _o("stable", "number", "stable or unstable (text)")],
     primary="max_real", conditions=["proper process", "no pole on the imaginary axis or nearly repeated poles (refused)"],
     method="roots of den(s) + Kc num(s) = 0 (30-digit polynomial root finding)",
     latex=r"1 + K_cG(s) = 0 \iff \mathrm{den}(s) + K_c\,\mathrm{num}(s) = 0",
     run=closed_loop_stability, reference=_ref_closed_loop_stability,
     wrong=lambda num, den, Kc: closed_loop_stability(num, den, -Kc),
     rand=_rand_cls,
     phrases=("Closed-loop stability", "Closed loop stability with P control", "Closed-loop poles"),
     examples=[_ex({"num": [1.0], "den": [1.0, 3.0, 3.0, 1.0], "Kc": 10.0}), _ex({"num": [2.0], "den": [1.0, 3.0, 3.0, 1.0], "Kc": 2.0})],
     level="advanced")


def _pi_loop(Kc, tauI, K, ts):
    mag = lambda w: Kc * K * math.sqrt(1.0 + 1.0 / (tauI * w) ** 2) / math.prod(math.sqrt(1.0 + (w * t) ** 2) for t in ts)
    ph = lambda w: -math.pi / 2 + math.atan(tauI * w) - sum(math.atan(w * t) for t in ts)
    return mag, ph


def margins_pi(Kc, tauI, K, taus, t_unit):
    _fin(Kc, K)
    _need(Kc * K > 0, "the loop gain must be positive")
    _need(3 <= len(taus) <= 5, "give 3 to 5 time constants")
    _pos("tauI and the time constants", tauI, *taus)
    ts = [t * _TU[t_unit] for t in taus]
    ti = tauI * _TU[t_unit]
    mag, ph = _pi_loop(Kc, ti, K, ts)
    lo, hi = 1e-6 / max(ts + [ti]), 1e4 / min(ts)
    grid = [lo * (hi / lo) ** (i / 4000.0) for i in range(4001)]
    cross = [i for i in range(4000) if (ph(grid[i]) + math.pi) * (ph(grid[i + 1]) + math.pi) <= 0]
    _need(len(cross) == 1, "the phase crosses -180 degrees more than once (or never): margins are not unique")
    i = cross[0]
    wpc = _bisect(lambda w: ph(w) + math.pi, grid[i], grid[i + 1])
    lg = lambda u: math.log(mag(math.exp(u)))
    wgc = math.exp(_bisect(lg, math.log(lo), math.log(hi)))
    return _out(GM=1.0 / mag(wpc), PM_deg=180.0 + math.degrees(ph(wgc)), w_pc=wpc, w_gc=wgc)


def _ref_margins_pi(Kc, tauI, K, taus, t_unit):
    import control
    ts = [t * _TU[t_unit] for t in taus]
    ti = tauI * _TU[t_unit]
    L = control.tf([Kc * ti, Kc], [ti, 0.0]) * control.tf([K], _lags_poly(ts))
    gm, pm, wpc, wgc = control.margin(L)
    return {"GM": float(gm), "PM_deg": float(pm), "w_pc": float(wpc), "w_gc": float(wgc)}


_add("chemeng_pi_loop_margins",
     name="Gain and phase margins of a PI loop around lags in series", topic="process control / stability",
     purpose="Gain margin, phase margin and the two crossover frequencies of a loop with a PI controller and a process of three to five first-order lags.",
     inputs=[_numin("Kc", "controller gain"), _phys("tauI", "time", "integral time"), _numin("K", "process gain"),
             _vecin("taus", "time constants of the lags, in t_unit (the integral time is converted to this unit too)", 3, 5), _LAGS_IN[2]],
     outputs=[_o("GM", "number", "gain margin (ratio)"), _o("PM_deg", "number", "phase margin, degrees"),
              _o("w_pc", "angular_frequency", "phase-crossover frequency (phase -180 degrees)"), _o("w_gc", "angular_frequency", "gain-crossover frequency (|L| = 1)")],
     primary="GM", conditions=["one phase crossover (refused otherwise)", "Kc K > 0"],
     method="L(jw) = Kc (1 + 1/(j w tauI)) K/prod(1 + j w tau_i); |L| = 1 and arg L = -180 degrees by bisection; GM = 1/|L(j w_pc)|, PM = 180 + arg L(j w_gc)",
     latex=r"GM = \frac{1}{|L(j\omega_{pc})|},\quad PM = 180^\circ + \arg L(j\omega_{gc})",
     run=margins_pi, reference=_ref_margins_pi,
     wrong=lambda Kc, tauI, K, taus, t_unit: (lambda r: r | _out(PM_deg=r["PM_deg"] - 90.0))(margins_pi(Kc, tauI, K, taus, t_unit)),
     rand=lambda rng: {"Kc": _lu(rng, 0.1, 2.0), "tauI": _sig(_lu(rng, 2, 50)), "K": _lu(rng, 0.5, 3), "taus": [_sig(_lu(rng, 0.5, 20)) for _ in range(rng.randint(3, 4))],
                       "t_unit": rng.choice(["s", "min"])},
     phrases=("PI loop margins", "Gain and phase margins", "Stability margins of a PI loop"),
     examples=[_ex({"Kc": 1.0, "tauI": 5.0, "K": 2.0, "taus": [1.0, 1.0, 1.0], "t_unit": "min"}),
               _ex({"Kc": 0.5, "tauI": 20.0, "K": 1.5, "taus": [10.0, 5.0, 2.0], "t_unit": "s"})],
     level="expert")


def fit_first_order(t, y, t_unit, M, theta):
    _need(len(t) == len(y) >= 4, "t and y need the same number of points (at least 4)")
    _fin(M, *y)
    _need(M != 0, "the step size M must not be zero")
    _nonneg("theta", theta)
    ts = [x * _TU[t_unit] for x in t]
    _need(all(b > a for a, b in zip(ts, ts[1:])), "times must increase")
    pts = [(a - theta, b) for a, b in zip(ts, y) if a > theta]
    _need(len(pts) >= 3, "need at least three points after the dead time")

    def kfit(tau):
        f = [-math.expm1(-s / tau) for s, _ in pts]
        sff = sum(v * v for v in f)
        return sum(v * yy for v, (_, yy) in zip(f, pts)) / (M * sff), f

    def sse(tau):
        K, f = kfit(tau)
        return sum((yy - K * M * v) ** 2 for v, (_, yy) in zip(f, pts))

    def dsse(tau):
        K, f = kfit(tau)
        return sum((yy - K * M * v) * (s / tau ** 2) * math.exp(-s / tau) for v, (s, yy) in zip(f, pts))
    span = max(s for s, _ in pts)
    grid = [span * 1e-3 * (1e5) ** (i / 400.0) for i in range(401)]
    vals = [sse(g) for g in grid]
    i = min(range(len(grid)), key=lambda j: vals[j])
    _need(0 < i < len(grid) - 1, "the data do not determine a time constant (no interior minimum)")
    tau = _bisect(dsse, grid[i - 1], grid[i + 1]) if dsse(grid[i - 1]) * dsse(grid[i + 1]) < 0 else grid[i]
    K, _ = kfit(tau)
    return _out(K=K, tau=tau)


def _ref_fit_first_order(t, y, t_unit, M, theta):
    """scipy's regression: curve_fit (MINPACK Levenberg-Marquardt, analytic Jacobian), then least_squares (trust-region
    reflective) continued with only the step-size stop: MINPACK's sum-of-squares stop leaves about 1e-9 in tau."""
    import numpy as np
    from scipy.optimize import curve_fit
    ts = np.array(t, float) * _TU[t_unit]
    yy = np.array(y, float)
    m = ts > theta
    s = ts[m] - theta
    ym = yy[m]
    model = lambda s, K, tau: K * M * -np.expm1(-s / tau)
    jac = lambda s, K, tau: np.column_stack([M * -np.expm1(-s / tau), -K * M * s / tau ** 2 * np.exp(-s / tau)])
    p, _ = curve_fit(model, s, ym, p0=[ym[-1] / M, (s[-1] - s[0]) / 3.0], jac=jac, method="lm", ftol=1e-15, xtol=1e-15,
                     gtol=1e-15, maxfev=20000)
    from scipy.optimize import least_squares
    r = least_squares(lambda pp: model(s, *pp) - ym, p, jac=lambda pp: jac(s, *pp), method="trf", x_scale="jac",
                      ftol=None, gtol=None, xtol=1e-15, max_nfev=2000)
    p = r.x
    return {"K": float(p[0]), "tau": float(p[1])}


def _rand_fit(rng):
    K, tau, th, M = rng.choice([-1, 1]) * _lu(rng, 0.2, 5), _lu(rng, 2, 200), _lu(rng, 0.5, 50), rng.uniform(1, 10)
    tu = rng.choice(["s", "min"])
    n = rng.randint(10, 30)
    T = th + tau * rng.uniform(3, 6)
    ts = [T * i / (n - 1) for i in range(n)]
    ys = [(0.0 if s <= th else K * M * -math.expm1(-(s - th) / tau)) + rng.gauss(0, 0.0001 * abs(K * M)) for s in ts]
    return {"t": [_sig(s / _TU[tu], 6) for s in ts], "y": [_sig(v, 7) for v in ys], "t_unit": tu, "M": M, "theta": th}


_add("chemeng_first_order_step_fit",
     name="Process gain and time constant from step-test data (least squares)", topic="process control / process identification",
     purpose="Gain and time constant of a first-order process (dead time known) fitted by least squares to the recorded response of a step test.",
     inputs=[_vecin("t", "sample times from the moment of the step, in t_unit", 4, 500), _vecin("y", "output deviations at those times", 4, 500),
             _choice("t_unit", ["s", "min", "h"], "unit of the times"), _numin("M", "size of the input step"), _phys("theta", "time", "dead time (known)")],
     outputs=[_o("K", "number", "process gain"), _o("tau", "time", "time constant")], primary="tau",
     conditions=["step from steady state", "first-order response after the dead time", "a unique interior least-squares minimum"],
     method="least squares of y = K M (1 - exp(-(t - theta)/tau)): K solved linearly for each tau, tau from the zero of the derivative of the residual sum (bisection)",
     latex=r"\min_{K,\tau}\sum_k\left[y_k - KM\left(1 - e^{-(t_k-\theta)/\tau}\right)\right]^2",
     run=fit_first_order, reference=_ref_fit_first_order,
     wrong=lambda t, y, t_unit, M, theta: fit_first_order(t, y, t_unit, M, 0.0) if theta > 0 else {"K": 0.0, "tau": 0.0},
     rand=_rand_fit,
     phrases=("First-order step test fit", "Step test fit", "Fit gain and time constant from step data"),
     examples=[_ex({"t": [0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20], "y": [0.0, 0.0, 1.7, 3.1, 4.3, 5.2, 6.0, 7.1, 7.9, 8.4, 8.8, 9.1],
                    "t_unit": "min", "M": 5.0, "theta": 60.0}),
               _ex({"t": [0, 10, 20, 30, 40, 60, 80, 100, 150, 200], "y": [0.0, -0.9, -1.75, -2.4, -2.95, -3.7, -4.2, -4.5, -4.85, -4.95],
                    "t_unit": "s", "M": 2.0, "theta": 0.0})],
     level="expert")


# ================================================================================================ RESERVOIR AND PETROLEUM
_SCF = 0.028316846592                   # m^3 per standard cubic foot
_BBL = 0.158987294928                   # m^3 per barrel


def radial_flow(k, h, mu, pe, pw, re, rw, S=0.0):
    _pos("k, h, mu, re and rw", k, h, mu, re, rw)
    _pos("the pressures", pe, pw)
    _fin(S)
    _need(re > rw, "the drainage radius re must exceed the well radius rw")
    _need(pe > pw, "the boundary pressure must exceed the flowing well pressure")
    d = math.log(re / rw) + S
    _need(d > 0, "the skin is so negative that the flow resistance vanishes")
    q = 2.0 * math.pi * k * h * (pe - pw) / (mu * d)
    return _out(q=q, PI=q / (pe - pw))


def _ref_radial_flow(k, h, mu, pe, pw, re, rw, S=0.0):
    """Darcy's law integrated across the drainage area: the radial resistance integral mu/(2 pi k h r) dr by
    quadrature, plus the skin resistance."""
    Rf = _quad(lambda r: mu / (2.0 * math.pi * k * h * r), rw, re) + S * mu / (2.0 * math.pi * k * h)
    return {"q": (pe - pw) / Rf, "PI": 1.0 / Rf}


_MD = 9.869233e-16
_add("chemeng_radial_darcy_flow",
     name="Steady radial flow of oil to a well (Darcy) and productivity index", topic="reservoir engineering / well inflow",
     purpose="Steady-state reservoir flow rate into a vertical well from a circular drainage area, with an optional skin factor, and the productivity index.",
     inputs=[_phys("k", "rock_permeability", "permeability (md is converted here)"), _phys("h", "length", "net pay thickness"),
             _phys("mu", "dynamic_viscosity", "fluid viscosity (cP is converted here)"), _phys("pe", "pressure", "pressure at the drainage radius"),
             _phys("pw", "pressure", "flowing bottom-hole pressure"), _phys("re", "length", "drainage radius"), _phys("rw", "length", "wellbore radius"),
             _numin("S", "skin factor (default 0: undamaged well)", required=False)],
     outputs=[_o("q", "flow_rate", "flow rate at reservoir conditions"), _o("PI", "productivity_index", "productivity index q/(pe - pw)")],
     primary="q", conditions=["steady state, single-phase incompressible flow", "re > rw, pe > pw"],
     method="q = 2 pi k h (pe - pw)/(mu (ln(re/rw) + S))", latex=r"q = \frac{2\pi kh\,(p_e - p_w)}{\mu\left(\ln\frac{r_e}{r_w} + S\right)}",
     run=radial_flow, reference=_ref_radial_flow,
     wrong=lambda k, h, mu, pe, pw, re, rw, S=0.0: (lambda q: _out(q=q, PI=q / (pe - pw)))(2 * math.pi * k * h * (pe - pw) / (mu * (math.log10(re / rw) + S))),
     rand=lambda rng: {"k": _lu(rng, 1, 2000) * _MD, "h": _lu(rng, 2, 100), "mu": _lu(rng, 2e-4, 0.05), "pe": rng.uniform(1e7, 4e7),
                       "pw": rng.uniform(5e6, 9e6), "re": _lu(rng, 100, 1000), "rw": rng.uniform(0.05, 0.2), "S": rng.choice([0.0, rng.uniform(-2, 10)])},
     phrases=("Radial Darcy flow", "Radial flow to a well", "Well inflow rate"),
     examples=[_ex({"k": 100 * _MD, "h": 30 * 0.3048, "mu": 1.5 * 1e-3, "pe": 3000 * 6894.757293168361, "pw": 2500 * 6894.757293168361,
                    "re": 1000 * 0.3048, "rw": 0.3 * 0.3048}),
               _ex({"k": 5e-14, "h": 15.0, "mu": 0.002, "pe": 25e6, "pw": 18e6, "re": 300.0, "rw": 0.1, "S": 3.0})],
     level="beginner",
     extra_q=[{"q": "Radial Darcy flow: k = 100 md, h = 30 ft, mu = 1.5 cP, pe = 3000 psi, pw = 2500 psi, re = 1000 ft, rw = 0.3 ft", "like": 0},
              {"q": "Radial Darcy flow: k = 100 md, h = 30 ft, mu = 1.5 cP, pe = 2000 psi, pw = 2500 psi, re = 1000 ft, rw = 0.3 ft", "outcome": "INVALID_INPUT"}])


def _decl_check(qi, Di, t):
    _pos("qi and Di", qi, Di)
    _nonneg("t", t)


def decline_exp(qi, Di, t):
    _decl_check(qi, Di, t)
    q = qi * math.exp(-Di * t)
    return _out(q=q, Np=qi * -math.expm1(-Di * t) / Di)


def decline_hyp(qi, Di, b, t):
    _decl_check(qi, Di, t)
    _fin(b)
    _need(0 < b < 1, "the hyperbolic exponent b must lie strictly between 0 and 1 (b = 0 exponential, b = 1 harmonic)")
    q = qi * (1.0 + b * Di * t) ** (-1.0 / b)
    return _out(q=q, Np=qi ** b / ((1.0 - b) * Di) * (qi ** (1.0 - b) - q ** (1.0 - b)))


def decline_harm(qi, Di, t):
    _decl_check(qi, Di, t)
    return _out(q=qi / (1.0 + Di * t), Np=qi / Di * math.log1p(Di * t))


def _ref_decline(qi, Di, t, b):
    """The Arps loss-ratio equation dq/dt = -Di (q/qi)^b q integrated as an ODE for the rate; the cumulative by
    quadrature of the rate-time curve."""
    q = _ivp(lambda s, y: [-Di * (y[0] / qi) ** b * y[0]], t, [qi], atol=[1e-18 * qi])[0]
    if b == 0:
        rate = lambda s: qi * math.exp(-Di * s)
    elif b == 1:
        rate = lambda s: qi / (1.0 + Di * s)
    else:
        rate = lambda s: qi * (1.0 + b * Di * s) ** (-1.0 / b)
    return {"q": q, "Np": _quad(rate, 0.0, t)}


_YR = 3.15576e7
_DECL_IN = [_phys("qi", "flow_rate", "initial rate (bbl/day and STB/day are converted here)"),
            _phys("Di", "decline_rate", "initial nominal decline rate, e.g. 0.3 1/yr")]
_decl_rand = lambda rng, b=None: dict({"qi": _lu(rng, 1e-4, 0.05), "Di": _lu(rng, 0.05, 2.0) / _YR, "t": _lu(rng, 0.1, 20) * _YR},
                                      **({} if b is None else {"b": b}))

_add("chemeng_exponential_decline",
     name="Exponential decline curve: rate and cumulative production", topic="petroleum engineering / decline curves",
     purpose="Production rate and cumulative production at time t of a well in exponential (constant-percentage) decline (Arps b = 0).",
     inputs=_DECL_IN + [_phys("t", "time", "time since the start of the decline")],
     outputs=[_o("q", "flow_rate", "rate at time t"), _o("Np", "volume", "cumulative production to time t")], primary="q",
     conditions=["boundary-dominated flow, constant operating conditions"], method="q = qi exp(-Di t); Np = (qi - q)/Di",
     latex=r"q = q_ie^{-D_it},\quad N_p = \frac{q_i - q}{D_i}",
     run=decline_exp, reference=lambda qi, Di, t: _ref_decline(qi, Di, t, 0),
     wrong=lambda qi, Di, t: _out(q=qi * math.exp(-Di * t), Np=qi * t * math.exp(-Di * t)),
     rand=lambda rng: _decl_rand(rng),
     phrases=("Exponential decline", "Exponential decline curve"),
     examples=[_ex({"qi": 1000 * _BBL / 86400, "Di": 0.3 / _YR, "t": 2 * _YR}), _ex({"qi": 0.01, "Di": 0.1 / _YR, "t": 5 * _YR})],
     level="beginner",
     extra_q=[{"q": "Exponential decline: qi = 1000 bbl/day, Di = 0.3 1/yr, t = 2 yr", "like": 0},
              {"q": "Exponential decline: qi = 1000 bbl/day, Di = 0.3, t = 2 yr", "outcome": "UNRECOGNIZED_FORMAT"}])

_add("chemeng_hyperbolic_decline",
     name="Hyperbolic decline curve: rate and cumulative production", topic="petroleum engineering / decline curves",
     purpose="Production rate and cumulative production at time t of a well in hyperbolic decline with Arps exponent 0 < b < 1.",
     inputs=_DECL_IN + [_numin("b", "Arps decline exponent (0 < b < 1)"), _phys("t", "time", "time since the start of the decline")],
     outputs=[_o("q", "flow_rate", "rate at time t"), _o("Np", "volume", "cumulative production to time t")], primary="q",
     conditions=["0 < b < 1", "boundary-dominated flow"],
     method="q = qi (1 + b Di t)^(-1/b); Np = qi^b/((1 - b) Di) (qi^(1-b) - q^(1-b))",
     latex=r"q = \frac{q_i}{(1 + bD_it)^{1/b}},\quad N_p = \frac{q_i^b}{(1-b)D_i}\left(q_i^{1-b} - q^{1-b}\right)",
     run=decline_hyp, reference=lambda qi, Di, b, t: _ref_decline(qi, Di, t, b),
     wrong=lambda qi, Di, b, t: (lambda q: _out(q=q, Np=qi ** b / ((1.0 - b) * Di) * (qi ** (1.0 - b) - q ** (1.0 - b))))(qi * (1.0 + b * Di * t) ** (-b)),
     rand=lambda rng: _decl_rand(rng, rng.uniform(0.1, 0.9)),
     phrases=("Hyperbolic decline", "Hyperbolic decline curve"),
     examples=[_ex({"qi": 1000 * _BBL / 86400, "Di": 0.5 / _YR, "b": 0.5, "t": 3 * _YR}), _ex({"qi": 0.01, "Di": 1.0 / _YR, "b": 0.8, "t": 10 * _YR})],
     level="intermediate",
     extra_q=[{"q": "Hyperbolic decline: qi = 1000 bbl/day, Di = 0.5 1/yr, b = 1, t = 3 yr", "outcome": "INVALID_INPUT"}])

_add("chemeng_harmonic_decline",
     name="Harmonic decline curve: rate and cumulative production", topic="petroleum engineering / decline curves",
     purpose="Production rate and cumulative production at time t of a well in harmonic decline (Arps b = 1).",
     inputs=_DECL_IN + [_phys("t", "time", "time since the start of the decline")],
     outputs=[_o("q", "flow_rate", "rate at time t"), _o("Np", "volume", "cumulative production to time t")], primary="q",
     conditions=["boundary-dominated flow"], method="q = qi/(1 + Di t); Np = (qi/Di) ln(qi/q)",
     latex=r"q = \frac{q_i}{1 + D_it},\quad N_p = \frac{q_i}{D_i}\ln\frac{q_i}{q}",
     run=decline_harm, reference=lambda qi, Di, t: _ref_decline(qi, Di, t, 1),
     wrong=lambda qi, Di, t: (lambda q: _out(q=q, Np=(qi - q) / Di))(qi / (1.0 + Di * t)),
     rand=lambda rng: _decl_rand(rng),
     phrases=("Harmonic decline", "Harmonic decline curve"),
     examples=[_ex({"qi": 1000 * _BBL / 86400, "Di": 0.4 / _YR, "t": 3 * _YR}), _ex({"qi": 0.01, "Di": 1.0 / _YR, "t": 10 * _YR})],
     level="intermediate")


_VU = {"m3": 1.0, "bbl": _BBL}


def _ho_check(*vecs):
    n = len(vecs[0])
    _need(n >= 2 and all(len(v) == n for v in vecs), "the lists must have the same length (at least 2 points)")
    for v in vecs:
        _fin(*v)


def ho_oil(F, Et, v_unit):
    _ho_check(F, Et)
    _need(all(e > 0 for e in Et) and all(f > 0 for f in F), "F and Et must be positive")
    N = sum(f * e for f, e in zip(F, Et)) / sum(e * e for e in Et)
    return _out(N=N * _VU[v_unit])


def _ref_ho_oil(F, Et, v_unit):
    import numpy as np
    sol = np.linalg.lstsq(np.array(Et, float)[:, None], np.array(F, float), rcond=None)[0]
    return {"N": float(sol[0]) * _VU[v_unit]}


def _rand_ho(rng, gascap=False):
    n = rng.randint(4, 12)
    N = _lu(rng, 1e6, 1e8)
    m = rng.uniform(0.1, 1.0)
    Eo = sorted(_lu(rng, 1e-3, 0.08) for _ in range(n))
    Eg = [e * rng.uniform(1.0, 3.0) for e in Eo]
    F = [N * (eo + (m * eg if gascap else 0.0)) * (1 + rng.uniform(-0.02, 0.02)) for eo, eg in zip(Eo, Eg)]
    out = {"F": [_sig(f, 6) for f in F], "v_unit": rng.choice(["m3", "bbl"])}
    if gascap:
        out |= {"Eo": [_sig(e, 5) for e in Eo], "Eg": [_sig(e, 5) for e in Eg]}
    else:
        out |= {"Et": [_sig(e, 5) for e in Eo]}
    return out


_add("chemeng_havlena_odeh_oil_in_place",
     name="Oil initially in place from the material balance (Havlena-Odeh straight line)", topic="reservoir engineering / material balance",
     purpose="Stock-tank oil initially in place from the underground withdrawal F and the total expansion term Et of successive pressure surveys (no water influx), by the straight line F = N Et through the origin.",
     inputs=[_vecin("F", "underground withdrawals at each survey, in v_unit at reservoir conditions", 2, 100),
             _vecin("Et", "total expansion per unit of oil in place at each survey (reservoir volume per stock-tank volume)", 2, 100),
             _choice("v_unit", list(_VU), "unit of F: m3 or bbl")],
     outputs=[_o("N", "volume", "oil initially in place (stock-tank volume)")], primary="N",
     conditions=["volumetric reservoir (no water influx)", "F and Et on one consistent basis"],
     method="least squares through the origin: N = sum F Et / sum Et^2", latex=r"F = N\,E_t,\quad N = \frac{\sum F E_t}{\sum E_t^2}",
     run=ho_oil, reference=_ref_ho_oil,
     wrong=lambda F, Et, v_unit: _out(N=sum(f / e for f, e in zip(F, Et)) / len(F) * _VU[v_unit]),
     rand=lambda rng: _rand_ho(rng),
     phrases=("Havlena-Odeh oil in place", "Material balance oil in place"),
     examples=[_ex({"F": [5.807e5, 1.6e6, 2.8e6, 4.1e6], "Et": [0.0101, 0.0278, 0.0485, 0.0712], "v_unit": "bbl"}),
               _ex({"F": [1.2e5, 2.6e5, 4.1e5], "Et": [0.004, 0.0085, 0.0137], "v_unit": "m3"})],
     level="advanced")


def ho_gascap(F, Eo, Eg, v_unit):
    _ho_check(F, Eo, Eg)
    _need(all(e > 0 for e in Eo) and len(F) >= 3, "Eo must be positive and at least three surveys are needed")
    xs = [g / o for g, o in zip(Eg, Eo)]
    ys = [f / o for f, o in zip(F, Eo)]
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    _need(sxx > 0, "Eg/Eo must vary between surveys")
    slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    N = my - slope * mx
    _need(N > 0, "the fitted oil in place is not positive")
    return _out(N=N * _VU[v_unit], m=slope / N)


def _ref_ho_gascap(F, Eo, Eg, v_unit):
    import numpy as np
    from scipy.stats import linregress
    r = linregress(np.array(Eg, float) / np.array(Eo, float), np.array(F, float) / np.array(Eo, float))
    return {"N": float(r.intercept) * _VU[v_unit], "m": float(r.slope) / float(r.intercept)}


_add("chemeng_havlena_odeh_gas_cap",
     name="Oil in place and gas-cap size from the material balance (Havlena-Odeh, gas cap)", topic="reservoir engineering / material balance",
     purpose="Oil initially in place and the gas-cap ratio m from the straight line F/Eo = N + N m Eg/Eo of successive surveys (no water influx).",
     inputs=[_vecin("F", "underground withdrawals, in v_unit at reservoir conditions", 3, 100), _vecin("Eo", "oil and dissolved-gas expansion per unit of oil in place", 3, 100),
             _vecin("Eg", "gas-cap expansion term per unit of oil in place", 3, 100), _choice("v_unit", list(_VU), "unit of F: m3 or bbl")],
     outputs=[_o("N", "volume", "oil initially in place (stock-tank volume)"), _o("m", "number", "ratio of initial gas-cap to oil-zone reservoir volume")],
     primary="N", conditions=["no water influx", "consistent units"],
     method="least squares of F/Eo against Eg/Eo: intercept N, slope N m", latex=r"\frac{F}{E_o} = N + Nm\,\frac{E_g}{E_o}",
     run=ho_gascap, reference=_ref_ho_gascap,
     wrong=lambda F, Eo, Eg, v_unit: (lambda r: _out(N=r["N"] * (1 + r["m"]), m=r["m"]))(ho_gascap(F, Eo, Eg, v_unit)),
     rand=lambda rng: _rand_ho(rng, gascap=True),
     phrases=("Havlena-Odeh gas cap", "Material balance with gas cap"),
     examples=[_ex({"F": [5.0e6, 9.3e6, 1.43e7, 1.95e7], "Eo": [0.01, 0.018, 0.027, 0.036], "Eg": [0.025, 0.05, 0.08, 0.11], "v_unit": "bbl"}),
               _ex({"F": [2.0e5, 3.9e5, 6.2e5], "Eo": [0.005, 0.0095, 0.015], "Eg": [0.008, 0.016, 0.026], "v_unit": "m3"})],
     level="expert")


_GPU = {"m3": 1.0, "MMscf": 1e6 * _SCF, "Bscf": 1e9 * _SCF}


def gas_pz(p, z, Gp, p_unit, gp_unit):
    _ho_check(p, z, Gp)
    _pos("pressures and z", *p, *z)
    xs = [g * _GPU[gp_unit] for g in Gp]
    ys = [a * _PUNIT[p_unit] / b for a, b in zip(p, z)]
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    _need(sxx > 0, "the cumulative productions must differ")
    slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    icpt = my - slope * mx
    _need(slope < 0 and icpt > 0, "p/z must fall as gas is produced")
    return _out(G=-icpt / slope, pz_initial=icpt)


def _ref_gas_pz(p, z, Gp, p_unit, gp_unit):
    import numpy as np
    from scipy.stats import linregress
    r = linregress(np.array(Gp, float) * _GPU[gp_unit], np.array(p, float) * _PUNIT[p_unit] / np.array(z, float))
    return {"G": -float(r.intercept) / float(r.slope), "pz_initial": float(r.intercept)}


def _rand_pz(rng):
    n = rng.randint(3, 10)
    G = _lu(rng, 1e8, 1e11)
    pz0 = rng.uniform(1.5e7, 4e7)
    pu, gu = rng.choice(["kPa", "MPa", "bar", "psi"]), rng.choice(list(_GPU))
    Gps = sorted(rng.uniform(0.02, 0.7) * G for _ in range(n))
    zs = [rng.uniform(0.8, 1.0) for _ in range(n)]
    ps = [pz0 * (1 - g / G) * (1 + rng.uniform(-0.01, 0.01)) * zz for g, zz in zip(Gps, zs)]
    return {"p": [_sig(v / _PUNIT[pu], 6) for v in ps], "z": [_sig(v, 4) for v in zs], "Gp": [_sig(g / _GPU[gu], 6) for g in Gps],
            "p_unit": pu, "gp_unit": gu}


_add("chemeng_gas_pz_material_balance",
     name="Gas initially in place from the p/z plot", topic="reservoir engineering / gas material balance",
     purpose="Original gas in place of a volumetric gas reservoir from the straight line of p/z against cumulative gas production.",
     inputs=[_vecin("p", "average reservoir pressures, in p_unit", 2, 100), _vecin("z", "gas deviation factors at those pressures", 2, 100),
             _vecin("Gp", "cumulative gas production at standard conditions, in gp_unit", 2, 100),
             _choice("p_unit", list(_PUNIT), "unit of the pressures"), _choice("gp_unit", list(_GPU), "unit of Gp: m3, MMscf or Bscf")],
     outputs=[_o("G", "volume", "original gas in place at standard conditions"), _o("pz_initial", "pressure", "p/z extrapolated to no production")],
     primary="G", conditions=["volumetric (no water drive) gas reservoir", "p/z falls with production"],
     method="least squares of p/z = (p/z)_i (1 - Gp/G): G = -intercept/slope", latex=r"\frac{p}{z} = \frac{p_i}{z_i}\left(1 - \frac{G_p}{G}\right)",
     run=gas_pz, reference=_ref_gas_pz,
     wrong=lambda p, z, Gp, p_unit, gp_unit: (lambda r: _out(G=r["G"] * r["pz_initial"] / (p[0] * _PUNIT[p_unit]), pz_initial=r["pz_initial"]))(gas_pz(p, z, Gp, p_unit, gp_unit)),
     rand=_rand_pz,
     phrases=("Gas p/z material balance", "p/z plot gas in place", "Gas in place from p/z"),
     examples=[_ex({"p": [3200.0, 2925.0, 2525.0, 2125.0], "z": [0.85, 0.84, 0.83, 0.82], "Gp": [0.0, 15.0, 35.0, 55.0], "p_unit": "psi", "gp_unit": "Bscf"}),
               _ex({"p": [25.0, 22.1, 18.9], "z": [0.9, 0.89, 0.88], "Gp": [1e8, 3e8, 5e8], "p_unit": "MPa", "gp_unit": "m3"})],
     level="advanced")


def horner(dt, pws, t_unit, p_unit, tp, q, mu, h):
    _ho_check(dt, pws)
    _pos("tp, q, mu and h", tp, q, mu, h)
    ds = [x * _TU[t_unit] for x in dt]
    _pos("the shut-in times", *ds)
    xs = [math.log10((tp + d) / d) for d in ds]
    ys = [v * _PUNIT[p_unit] for v in pws]
    n = len(xs)
    _need(n >= 3, "at least three build-up points are needed")
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    _need(sxx > 0, "the shut-in times must differ")
    slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    m = -slope
    _need(m > 0, "the build-up pressure must rise with shut-in time")
    return _out(m=m, k=q * mu * math.log(10.0) / (4.0 * math.pi * m * h), p_star=my - slope * mx)


def _ref_horner(dt, pws, t_unit, p_unit, tp, q, mu, h):
    import numpy as np
    from scipy.stats import linregress
    d = np.array(dt, float) * _TU[t_unit]
    r = linregress(np.log10((tp + d) / d), np.array(pws, float) * _PUNIT[p_unit])
    m = -float(r.slope)
    return {"m": m, "k": q * mu * math.log(10.0) / (4.0 * math.pi * m * h), "p_star": float(r.intercept)}


def _rand_horner(rng):
    k, h, mu, q = _lu(rng, 5, 1000) * _MD, _lu(rng, 3, 60), _lu(rng, 3e-4, 0.01), _lu(rng, 1e-4, 0.02)
    tp, pst = _lu(rng, 24, 2000) * 3600.0, rng.uniform(1.5e7, 4e7)
    m = q * mu * math.log(10) / (4 * math.pi * k * h)
    tu, pu = rng.choice(["h", "min"]), rng.choice(["kPa", "MPa", "psi"])
    n = rng.randint(4, 12)
    ds = sorted(_lu(rng, 1, 72) * 3600.0 for _ in range(n))
    ps = [pst - m * math.log10((tp + d) / d) + rng.gauss(0, 0.002 * m) for d in ds]
    return {"dt": [_sig(d / _TU[tu], 5) for d in ds], "pws": [_sig(v / _PUNIT[pu], 7) for v in ps], "t_unit": tu, "p_unit": pu,
            "tp": tp, "q": q, "mu": mu, "h": h}


_add("chemeng_horner_buildup",
     name="Pressure build-up test: Horner analysis for permeability and extrapolated pressure", topic="reservoir engineering / well testing",
     purpose="Permeability and the extrapolated pressure p* from the semilog straight line of shut-in pressure against the Horner time ratio (tp + dt)/dt.",
     inputs=[_vecin("dt", "shut-in times, in t_unit", 3, 200), _vecin("pws", "shut-in bottom-hole pressures, in p_unit", 3, 200),
             _choice("t_unit", ["s", "min", "h"], "unit of the shut-in times"), _choice("p_unit", list(_PUNIT), "unit of the pressures"),
             _phys("tp", "time", "producing time before shut-in"), _phys("q", "flow_rate", "flow rate before shut-in at reservoir conditions"),
             _phys("mu", "dynamic_viscosity", "fluid viscosity"), _phys("h", "length", "net pay thickness")],
     outputs=[_o("m", "pressure", "semilog slope, pressure change per log cycle"), _o("k", "rock_permeability", "permeability"),
              _o("p_star", "pressure", "extrapolated pressure p* at (tp + dt)/dt = 1")],
     primary="k", conditions=["infinite-acting radial flow on the straight line", "the points given lie on the semilog straight line"],
     method="least squares of pws against log10((tp + dt)/dt): slope -m; k = q mu ln 10/(4 pi m h)",
     latex=r"p_{ws} = p^* - m\log_{10}\frac{t_p + \Delta t}{\Delta t},\quad k = \frac{q\mu\ln 10}{4\pi mh}",
     run=horner, reference=_ref_horner,
     wrong=lambda dt, pws, t_unit, p_unit, tp, q, mu, h: (lambda r: r | _out(k=q * mu / (4 * math.pi * r["m"] * h)))(horner(dt, pws, t_unit, p_unit, tp, q, mu, h)),
     rand=_rand_horner,
     phrases=("Horner build-up analysis", "Pressure build-up test", "Horner plot"),
     examples=[_ex({"dt": [2.0, 4.0, 8.0, 16.0, 24.0], "pws": [2960.0, 3015.0, 3069.0, 3122.0, 3150.0], "t_unit": "h", "p_unit": "psi",
                    "tp": 500 * 3600.0, "q": 250 * _BBL / 86400 * 1.2, "mu": 0.8e-3, "h": 20 * 0.3048}),
               _ex({"dt": [1.0, 3.0, 10.0, 30.0], "pws": [20.1, 20.6, 21.05, 21.4], "t_unit": "h", "p_unit": "MPa", "tp": 200 * 3600.0,
                    "q": 0.002, "mu": 0.001, "h": 10.0})],
     level="expert")


def _e1(x):
    """Exponential integral E1(x), x > 0: power series below 1, continued fraction (modified Lentz) above."""
    if x <= 1.0:
        s, term, n = 0.0, 1.0, 1
        while True:
            term *= -x / n
            add = -term / n
            s += add
            if abs(add) <= 1e-18 * abs(s) or n > 200:
                break
            n += 1
        return -0.5772156649015329 - math.log(x) + s
    b = x + 1.0
    c, d = 1e300, 1.0 / b
    h = d
    for i in range(1, 500):
        an = -i * i
        b += 2.0
        d = 1.0 / (an * d + b)
        c = b + an / c
        de = c * d
        h *= de
        if abs(de - 1.0) <= 1e-16:
            break
    return h * math.exp(-x)


def line_source(q, mu, k, h, phi, ct, r, t, pi):
    _pos("q, mu, k, h, ct, r, t and pi", q, mu, k, h, ct, r, t, pi)
    _frac("the porosity phi", phi, lo_open=True, hi_open=True)
    x = phi * mu * ct * r * r / (4.0 * k * t)
    _need(x <= 50.0, "the pressure disturbance has not reached this radius yet (Ei argument above 50)")
    dp = q * mu / (4.0 * math.pi * k * h) * _e1(x)
    _need(dp < pi, "the drawdown exceeds the initial pressure")
    return _out(dp=dp, p=pi - dp)


def _ref_line_source(q, mu, k, h, phi, ct, r, t, pi):
    """Superposition in time of the instantaneous line source: dp = (q mu/(4 pi k h)) integral from 0 to t of
    exp(-r^2/(4 eta s)) ds/s, eta = k/(phi mu ct), integrated by scipy quadrature (in u = ln s)."""
    a = r * r * phi * mu * ct / (4.0 * k)
    g = _quad(lambda u: math.exp(-a * math.exp(-u)), math.log(a) - 6.5, math.log(t)) if math.log(t) > math.log(a) - 6.5 else 0.0
    dp = q * mu / (4.0 * math.pi * k * h) * g
    return {"dp": dp, "p": pi - dp}


_add("chemeng_line_source_drawdown",
     name="Transient drawdown around a well (line-source solution)", topic="reservoir engineering / well testing",
     purpose="Pressure drop and pressure at radius r and time t after a well starts producing at a constant rate in an infinite reservoir (line-source, exponential-integral solution).",
     inputs=[_phys("q", "flow_rate", "flow rate at reservoir conditions"), _phys("mu", "dynamic_viscosity", "fluid viscosity"),
             _phys("k", "rock_permeability", "permeability"), _phys("h", "length", "net pay thickness"), _numin("phi", "porosity (fraction)", pct=True),
             _phys("ct", "compressibility", "total compressibility (1/psi is converted here)"), _phys("r", "length", "radius from the well"),
             _phys("t", "time", "time since production started"), _phys("pi", "pressure", "initial reservoir pressure")],
     outputs=[_o("dp", "pressure", "pressure drop at r and t"), _o("p", "pressure", "pressure at r and t")], primary="dp",
     conditions=["infinite-acting, homogeneous, slightly compressible fluid", "constant rate from t = 0"],
     method="dp = (q mu/(4 pi k h)) E1(phi mu ct r^2/(4 k t)); E1 by series or continued fraction",
     latex=r"\Delta p = \frac{q\mu}{4\pi kh}\,E_1\!\left(\frac{\phi\mu c_t r^2}{4kt}\right)",
     run=line_source, reference=_ref_line_source,
     wrong=lambda q, mu, k, h, phi, ct, r, t, pi: (lambda x: (lambda dp: _out(dp=dp, p=pi - dp))(q * mu / (4 * math.pi * k * h) * (-0.5772156649015329 - math.log(x))))(
         phi * mu * ct * r * r / (4 * k * t)) if phi * mu * ct * r * r / (4 * k * t) < 0.5 else {"dp": 0.0, "p": 0.0},
     rand=lambda rng: (lambda k, phi, mu, ct, r: {"q": _lu(rng, 1e-4, 0.01), "mu": mu, "k": k, "h": _lu(rng, 3, 60), "phi": phi, "ct": ct, "r": r,
                                                 "t": phi * mu * ct * r * r / (4 * k * _lu(rng, 1e-4, 20)), "pi": rng.uniform(2e7, 4e7)})(
         _lu(rng, 5, 1000) * _MD, rng.uniform(0.05, 0.35), _lu(rng, 3e-4, 0.01), _lu(rng, 3e-10, 3e-9), _lu(rng, 0.1, 500)),
     phrases=("Line-source drawdown", "Transient drawdown", "Exponential integral drawdown"),
     examples=[_ex({"q": 0.001, "mu": 0.001, "k": 1e-13, "h": 10.0, "phi": 0.2, "ct": 1e-9, "r": 0.1, "t": 3600.0, "pi": 3e7}),
               _ex({"q": 0.002, "mu": 0.0015, "k": 5e-14, "h": 20.0, "phi": 0.15, "ct": 2e-9, "r": 100.0, "t": 86400.0, "pi": 2.5e7})],
     level="expert",
     extra_q=[{"q": "Line-source drawdown: q = 3.6 m^3/h, mu = 1 cP, k = 1e-13 m^2, h = 10 m, phi = 20%, ct = 1e-9 1/Pa, r = 10 cm, t = 1 h, pi = 300 bar", "like": 0}])


_QU = {"m3_per_day": 1.0 / 86400.0, "Mscf_per_day": 1e3 * _SCF / 86400.0}


def backpressure(q, pwf, q_unit, p_unit, pr):
    _ho_check(q, pwf)
    _pos("pr", pr)
    _need(len(q) >= 2, "at least two test rates are needed")
    qs = [v * _QU[q_unit] for v in q]
    ps = [v * _PUNIT[p_unit] for v in pwf]
    _pos("the rates", *qs)
    _need(all(0 <= p < pr for p in ps), "flowing pressures must lie between 0 and the reservoir pressure")
    xs = [math.log10(pr * pr - p * p) for p in ps]
    ys = [math.log10(v) for v in qs]
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    _need(sxx > 0, "the flowing pressures must differ")
    nn = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    lc = my - nn * mx
    _need(0.3 < nn < 1.5, "the back-pressure exponent n falls outside 0.3 to 1.5: the data do not fit the back-pressure equation")
    return _out(n=nn, log10_C=lc, AOF=10.0 ** (lc + nn * math.log10(pr * pr)))


def _ref_backpressure(q, pwf, q_unit, p_unit, pr):
    import numpy as np
    from scipy.stats import linregress
    qs = np.array(q, float) * _QU[q_unit]
    ps = np.array(pwf, float) * _PUNIT[p_unit]
    r = linregress(np.log10(pr * pr - ps * ps), np.log10(qs))
    n, lc = float(r.slope), float(r.intercept)
    return {"n": n, "log10_C": lc, "AOF": 10.0 ** (lc + n * math.log10(pr * pr))}


def _rand_bp(rng):
    pr = rng.uniform(1e7, 3e7)
    n = rng.uniform(0.55, 0.95)
    C = 10 ** rng.uniform(-14, -10) / (1e14 ** n) * 1e6
    pu, qu = rng.choice(["kPa", "MPa", "psi"]), rng.choice(list(_QU))
    pws = sorted(pr * rng.uniform(0.3, 0.95) for _ in range(rng.randint(3, 6)))
    qs = [C * (pr * pr - p * p) ** n * (1 + rng.uniform(-0.02, 0.02)) for p in pws]
    return {"q": [_sig(v / _QU[qu], 6) for v in qs], "pwf": [_sig(p / _PUNIT[pu], 6) for p in pws], "q_unit": qu, "p_unit": pu, "pr": pr}


_add("chemeng_gas_well_backpressure",
     name="Gas-well deliverability from a back-pressure test (Rawlins-Schellhardt)", topic="petroleum engineering / gas well testing",
     purpose="Exponent n, coefficient C and absolute open flow of a gas well from multi-rate test data fitted to q = C (pr^2 - pwf^2)^n.",
     inputs=[_vecin("q", "test gas rates, in q_unit", 2, 50), _vecin("pwf", "flowing bottom-hole pressures at those rates, in p_unit", 2, 50),
             _choice("q_unit", list(_QU), "unit of the rates: m3_per_day or Mscf_per_day"), _choice("p_unit", list(_PUNIT), "unit of the flowing pressures"),
             _phys("pr", "pressure", "average reservoir (shut-in) pressure")],
     outputs=[_o("n", "number", "back-pressure exponent"), _o("log10_C", "number", "log10 of the coefficient C, with q in m^3/s and pressures in Pa"),
              _o("AOF", "flow_rate", "absolute open flow (rate at pwf = 0)")], primary="AOF",
     conditions=["stabilised flow at every rate", "0.3 < n < 1.5"],
     method="least squares of log q against log(pr^2 - pwf^2): slope n, intercept log C; AOF = C pr^(2n)",
     latex=r"q = C\left(\bar p_r^2 - p_{wf}^2\right)^n,\quad AOF = C\,\bar p_r^{2n}",
     run=backpressure, reference=_ref_backpressure,
     wrong=lambda q, pwf, q_unit, p_unit, pr: (lambda r: r | _out(AOF=10.0 ** (r["log10_C"] + r["n"] * math.log10(pr))))(backpressure(q, pwf, q_unit, p_unit, pr)),
     rand=_rand_bp,
     phrases=("Gas well back-pressure test", "Back-pressure deliverability", "Absolute open flow"),
     examples=[_ex({"q": [4288.0, 9265.0, 15552.0, 20177.0], "pwf": [403.1, 394.0, 378.5, 362.6], "q_unit": "Mscf_per_day", "p_unit": "psi", "pr": 408.2 * 6894.757293168361}),
               _ex({"q": [1.0e5, 2.0e5, 3.2e5], "pwf": [18.0, 15.5, 11.0], "q_unit": "m3_per_day", "p_unit": "MPa", "pr": 2e7})],
     level="advanced")


# ================================================================================================ DEFINITIONAL ADDITIONS
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). Fenske minimum
# stages, pump head and NPSH available, mass-transfer coefficient correlations (stated), multicomponent Underwood,
# second-order CSTR volume, equimolar counterdiffusion flux, modified Raoult's law with activity coefficients.

def fenske(alpha, xD, xB):
    _fin(alpha, xD, xB)
    _need(alpha > 1.0, "the relative volatility alpha must exceed 1")
    _need(0 < xB < xD < 1, "the compositions must satisfy 0 < xB < xD < 1")
    N = math.log((xD / (1 - xD)) * ((1 - xB) / xB)) / math.log(alpha)
    return _out(Nmin=N, Nmin_trays=N - 1.0)


def _ref_fenske(alpha, xD, xB):
    """Total reflux in exact arithmetic (sympy, 30 digits): each equilibrium stage multiplies the light/heavy ratio by
    alpha, so N is the logarithm of the separation ratio to the base alpha."""
    import sympy as sp
    a, d, b = (sp.Rational(repr(v)) for v in (alpha, xD, xB))
    N = sp.N(sp.log((d / (1 - d)) * ((1 - b) / b)) / sp.log(a), 30)
    return {"Nmin": float(N), "Nmin_trays": float(N - 1)}


_add("chemeng_fenske_minimum_stages",
     name="Minimum number of stages at total reflux (Fenske)", topic="separations / distillation",
     purpose="The minimum number of equilibrium stages of a binary distillation at total reflux by the Fenske equation (constant relative volatility), counting the partial reboiler as a stage.",
     inputs=[_numin("alpha", "relative volatility of the light component (constant)"),
             _numin("xD", "mole fraction of the light component in the distillate", pct=True),
             _numin("xB", "mole fraction of the light component in the bottoms", pct=True)],
     outputs=[_o("Nmin", "number", "minimum number of equilibrium stages (partial reboiler included)"),
              _o("Nmin_trays", "number", "minimum number of trays (Nmin - 1, the reboiler excluded)")],
     primary="Nmin", conditions=["binary, constant alpha", "0 < xB < xD < 1", "total reflux, total condenser"],
     method="N_min = ln[(xD/(1 - xD)) ((1 - xB)/xB)]/ln(alpha)",
     latex=r"N_{min} = \frac{\ln\left[\frac{x_D}{1 - x_D}\,\frac{1 - x_B}{x_B}\right]}{\ln\alpha}",
     run=fenske, reference=_ref_fenske,
     wrong=lambda alpha, xD, xB: (lambda N: _out(Nmin=N, Nmin_trays=N - 1))(math.log(xD / xB) / math.log(alpha)),
     rand=lambda rng: {"alpha": rng.uniform(1.2, 6.0), "xD": rng.uniform(0.8, 0.999), "xB": rng.uniform(0.001, 0.2)},
     phrases=("Fenske minimum stages", "Minimum stages at total reflux", "Fenske equation"),
     examples=[_ex({"alpha": 2.5, "xD": 0.95, "xB": 0.05}), _ex({"alpha": 1.5, "xD": 0.99, "xB": 0.01})],
     level="intermediate",
     extra_q=[{"q": "Fenske minimum stages: alpha = 0.9, xD = 0.95, xB = 0.05", "outcome": "INVALID_INPUT"}])


def pump_head(p1, p2, rho, dz=0.0, v1=0.0, v2=0.0, Q=None, eta=None):
    _pos("rho", rho)
    _need(p1 > 0 and p2 > 0, "pressures are absolute and must be positive")
    _fin(dz, v1, v2)
    _need(v1 >= 0 and v2 >= 0, "the speeds must be zero or positive")
    H = (p2 - p1) / (rho * G0) + dz + (v2 * v2 - v1 * v1) / (2 * G0)
    _need(H > 0, "the pump must add head (H > 0)")
    out = dict(H=H, dp_equivalent=rho * G0 * H)
    if Q is not None:
        _pos("Q", Q)
        out["P_hydraulic"] = rho * G0 * Q * H
        if eta is not None:
            _need(0 < eta <= 1, "the pump efficiency must lie in (0, 1]")
            out["P_shaft"] = rho * G0 * Q * H / eta
    elif eta is not None:
        raise ValueError("the shaft power needs the flow Q as well as the efficiency")
    return _out(**out)


def _ref_pump_head(p1, p2, rho, dz=0.0, v1=0.0, v2=0.0, Q=None, eta=None):
    """The pressure part by fluids' own head_from_P, the velocity head from the kinetic energy per unit weight, the
    hydraulic power as the work done on the flow (fluids)."""
    from fluids.core import head_from_P
    H = head_from_P(p2 - p1, rho, g=G0) + dz + (v2 * v2 - v1 * v1) / (2 * G0)
    out = {"H": H, "dp_equivalent": rho * G0 * H}
    if Q is not None:
        out["P_hydraulic"] = Q * rho * G0 * H
        if eta is not None:
            out["P_shaft"] = out["P_hydraulic"] / eta
    return out


def _rand_pump(rng):
    p1 = rng.uniform(5e4, 5e5)
    x = {"p1": p1, "p2": p1 + rng.uniform(5e4, 2e6), "rho": rng.uniform(600, 1500)}
    if rng.random() < 0.6:
        x["dz"] = rng.uniform(-5, 30)
    if rng.random() < 0.4:
        x["v1"], x["v2"] = rng.uniform(0.5, 3), rng.uniform(0.5, 6)
    if rng.random() < 0.6:
        x["Q"] = _lu(rng, 1e-4, 1.0)
        if rng.random() < 0.6:
            x["eta"] = rng.uniform(0.4, 0.9)
    return x


_add("chemeng_pump_head",
     name="Pump total head and power", topic="fluid transport / pumps",
     purpose="The total head a pump adds between its suction and discharge, H = (p2 - p1)/(rho g) + (z2 - z1) + (v2^2 - v1^2)/(2g), with the hydraulic power rho g Q H and the shaft power for an efficiency.",
     inputs=[_phys("p1", "pressure", "suction pressure (absolute)"), _phys("p2", "pressure", "discharge pressure (absolute)"),
             _phys("rho", "density", "liquid density"), _phys("dz", "length", "elevation of the discharge gauge above the suction gauge (default 0)", required=False),
             _phys("v1", "speed", "suction velocity (default 0)", required=False), _phys("v2", "speed", "discharge velocity (default 0)", required=False),
             _phys("Q", "flow_rate", "volumetric flow (optional, for the power)", required=False),
             _numin("eta", "pump efficiency (optional, with Q)", required=False, pct=True)],
     outputs=[_o("H", "length", "total head"), _o("dp_equivalent", "pressure", "head as a pressure rho g H"),
              _o("P_hydraulic", "power", "hydraulic power rho g Q H"), _o("P_shaft", "power", "shaft power rho g Q H/eta")],
     primary="H", conditions=["absolute pressures", "incompressible liquid", "H > 0"],
     method="H = (p2 - p1)/(rho g) + dz + (v2^2 - v1^2)/(2 g); P = rho g Q H; P_shaft = P/eta",
     latex=r"H = \frac{p_2 - p_1}{\rho g} + (z_2 - z_1) + \frac{v_2^2 - v_1^2}{2g},\quad P = \rho g Q H",
     run=pump_head, reference=_ref_pump_head,
     wrong=lambda p1, p2, rho, dz=0.0, v1=0.0, v2=0.0, Q=None, eta=None: (lambda o: o | _out(H=o["H"] * G0, dp_equivalent=o["dp_equivalent"] * G0))(pump_head(p1, p2, rho, dz, v1, v2, Q, eta)),
     rand=_rand_pump,
     phrases=("Pump total head", "Pump head", "Total dynamic head"),
     examples=[_ex({"p1": 100000.0, "p2": 400000.0, "rho": 1000.0}), _ex({"p1": 150000.0, "p2": 650000.0, "rho": 870.0, "dz": 2.0, "v1": 1.5, "v2": 3.0, "Q": 0.02, "eta": 0.7})],
     level="beginner",
     extra_q=[{"q": "Pump total head: p1 = 400000 Pa, p2 = 100000 Pa, rho = 1000 kg/m^3", "outcome": "INVALID_INPUT"}])


def npsh_available(p_s, p_v, rho, z_s, h_f, NPSHr=None):
    _pos("p_s and rho", p_s, rho)
    _need(p_v >= 0, "the vapour pressure must be zero or positive")
    _fin(z_s, h_f)
    _need(h_f >= 0, "the suction friction head loss must be zero or positive")
    N = (p_s - p_v) / (rho * G0) + z_s - h_f
    out = dict(NPSHa=N, pressure_head=p_s / (rho * G0), vapour_head=p_v / (rho * G0))
    if NPSHr is not None:
        _need(NPSHr >= 0, "the required NPSH must be zero or positive")
        out["margin"] = N - NPSHr
    return _out(**out)


def _ref_npsh_available(p_s, p_v, rho, z_s, h_f, NPSHr=None):
    """The heads by fluids' own head_from_P (pressure and vapour pressure), combined along the suction line."""
    from fluids.core import head_from_P
    hp, hv = head_from_P(p_s, rho, g=G0), head_from_P(p_v, rho, g=G0)
    N = hp - hv + z_s - h_f
    out = {"NPSHa": N, "pressure_head": hp, "vapour_head": hv}
    if NPSHr is not None:
        out["margin"] = N - NPSHr
    return out


_add("chemeng_npsh_available",
     name="Net positive suction head available", topic="fluid transport / pumps",
     purpose="The net positive suction head available at a pump inlet, NPSHa = (p_s - p_v)/(rho g) + z_s - h_f, from the pressure on the liquid surface, the vapour pressure, the static head and the suction-line loss; with the required NPSH, the margin.",
     inputs=[_phys("p_s", "pressure", "absolute pressure on the suction liquid surface"), _phys("p_v", "pressure", "vapour pressure of the liquid at the pumping temperature"),
             _phys("rho", "density", "liquid density"), _phys("z_s", "length", "liquid surface above the pump inlet (negative for a suction lift)"),
             _phys("h_f", "length", "friction head loss in the suction line"), _phys("NPSHr", "length", "required NPSH of the pump (optional)", required=False)],
     outputs=[_o("NPSHa", "length", "NPSH available"), _o("pressure_head", "length", "p_s/(rho g)"), _o("vapour_head", "length", "p_v/(rho g)"),
              _o("margin", "length", "NPSHa - NPSHr (with NPSHr)")],
     primary="NPSHa", conditions=["absolute pressures", "h_f >= 0"],
     method="NPSHa = (p_s - p_v)/(rho g) + z_s - h_f", latex=r"NPSH_a = \frac{p_s - p_v}{\rho g} + z_s - h_f",
     run=npsh_available, reference=_ref_npsh_available,
     wrong=lambda p_s, p_v, rho, z_s, h_f, NPSHr=None: (lambda o: o | _out(NPSHa=o["NPSHa"] + 2 * h_f, **({"margin": o["margin"] + 2 * h_f} if NPSHr is not None else {})))(npsh_available(p_s, p_v, rho, z_s, h_f, NPSHr)),
     rand=lambda rng: {"p_s": rng.uniform(5e4, 3e5), "p_v": rng.uniform(500, 4e4), "rho": rng.uniform(600, 1300), "z_s": rng.uniform(-4, 8), "h_f": rng.uniform(0.1, 3),
                       **({"NPSHr": rng.uniform(1, 6)} if rng.random() < 0.5 else {})},
     phrases=("NPSH available", "Net positive suction head available", "NPSHa"),
     examples=[_ex({"p_s": 101325.0, "p_v": 2339.0, "rho": 998.0, "z_s": 2.0, "h_f": 0.5}), _ex({"p_s": 101325.0, "p_v": 47400.0, "rho": 971.8, "z_s": -3.0, "h_f": 0.8, "NPSHr": 2.0})],
     level="intermediate",
     extra_q=[{"q": "NPSH available: p_s = 101325 Pa, p_v = 2339 Pa, rho = 998 kg/m^3, z_s = 2 m, h_f = -0.5 m", "outcome": "INVALID_INPUT"}])

_SH_CORR = {"sphere": "Ranz-Marshall Sh = 2 + 0.6 Re^(1/2) Sc^(1/3)", "flat_plate": "laminar flat plate (average) Sh = 0.664 Re^(1/2) Sc^(1/3)",
            "pipe": "Linton-Sherwood turbulent pipe Sh = 0.023 Re^0.83 Sc^(1/3)"}


def _sherwood(geometry, Re, Sc):
    if geometry == "sphere":
        return 2.0 + 0.6 * math.sqrt(Re) * Sc ** (1 / 3)
    if geometry == "flat_plate":
        return 0.664 * math.sqrt(Re) * Sc ** (1 / 3)
    return 0.023 * Re ** 0.83 * Sc ** (1 / 3)


def mass_transfer_coefficient(geometry, u, L, D, rho, mu):
    _pos("u, L, D, rho and mu", u, L, D, rho, mu)
    _need(geometry in _SH_CORR, "geometry must be sphere, flat_plate or pipe")
    Re = rho * u * L / mu
    Sc = mu / (rho * D)
    if geometry == "sphere":
        _need(2 <= Re <= 2e5 and 0.6 <= Sc <= 400, "Ranz-Marshall is stated for 2 <= Re <= 200000 and 0.6 <= Sc <= 400")
    elif geometry == "flat_plate":
        _need(Re < 5e5 and Sc >= 0.6, "the laminar flat-plate correlation needs Re < 500000 and Sc >= 0.6")
    else:
        _need(2000 <= Re <= 70000 and 0.6 <= Sc <= 3000, "Linton-Sherwood is stated for 2000 <= Re <= 70000 and 0.6 <= Sc <= 3000")
    Sh = _sherwood(geometry, Re, Sc)
    return _out(Re=Re, Sc=Sc, Sh=Sh, kc=Sh * D / L)


def _ref_mass_transfer_coefficient(geometry, u, L, D, rho, mu):
    """The stated correlation re-evaluated with numpy from the dimensionless groups (fluids' Reynolds and Schmidt
    numbers)."""
    import numpy as np
    from fluids.core import Reynolds, Schmidt
    Re = Reynolds(V=u, D=L, rho=rho, mu=mu)
    Sc = Schmidt(D=D, mu=mu, rho=rho)
    if geometry == "sphere":
        Sh = 2.0 + 0.6 * np.sqrt(Re) * np.cbrt(Sc)
    elif geometry == "flat_plate":
        Sh = 0.664 * np.sqrt(Re) * np.cbrt(Sc)
    else:
        Sh = 0.023 * np.power(Re, 0.83) * np.cbrt(Sc)
    return {"Re": Re, "Sc": Sc, "Sh": float(Sh), "kc": float(Sh) * D / L}


def _rand_mtc(rng):
    g = rng.choice(["sphere", "flat_plate", "pipe"])
    rho, mu = rng.uniform(700, 1100), rng.uniform(4e-4, 3e-3)
    Sc = rng.uniform(1, 300)
    D = mu / (rho * Sc)
    Re = {"sphere": _lu(rng, 3, 1e5), "flat_plate": _lu(rng, 100, 4e5), "pipe": _lu(rng, 3000, 6e4)}[g]
    L = _lu(rng, 1e-3, 1.0)
    return {"geometry": g, "u": Re * mu / (rho * L), "L": L, "D": D, "rho": rho, "mu": mu}


_add("chemeng_mass_transfer_coefficient",
     name="Mass-transfer coefficient from a stated Sherwood correlation", topic="transport / mass transfer",
     purpose="The convective mass-transfer coefficient k_c = Sh D/L from the Reynolds and Schmidt numbers by a stated correlation: a sphere (Ranz-Marshall), a flat plate in laminar flow (average) or turbulent flow in a pipe (Linton-Sherwood).",
     inputs=[_choice("geometry", list(_SH_CORR), "correlation: sphere (Ranz-Marshall), flat_plate (laminar, average) or pipe (Linton-Sherwood, turbulent)"),
             _phys("u", "speed", "fluid velocity"), _phys("L", "length", "characteristic length: sphere or pipe diameter, plate length"),
             _phys("D", "diffusion_coefficient", "diffusivity of the solute in the fluid"), _phys("rho", "density", "fluid density"),
             _phys("mu", "dynamic_viscosity", "fluid viscosity")],
     outputs=[_o("Re", "number", "Reynolds number rho u L/mu"), _o("Sc", "number", "Schmidt number mu/(rho D)"), _o("Sh", "number", "Sherwood number"),
              _o("kc", "speed", "mass-transfer coefficient Sh D/L")],
     primary="kc", conditions=["within each correlation's stated range of Re and Sc", "dilute solute"],
     method="Re = rho u L/mu, Sc = mu/(rho D); sphere Sh = 2 + 0.6 Re^1/2 Sc^1/3; plate Sh = 0.664 Re^1/2 Sc^1/3; pipe Sh = 0.023 Re^0.83 Sc^1/3; k_c = Sh D/L",
     latex=r"Sh = \frac{k_c L}{D},\quad Sh = 2 + 0.6\,Re^{1/2}Sc^{1/3}\ \text{(sphere)}",
     run=mass_transfer_coefficient, reference=_ref_mass_transfer_coefficient,
     wrong=lambda geometry, u, L, D, rho, mu: (lambda o: o | _out(kc=o["Sh"] * L / D))(mass_transfer_coefficient(geometry, u, L, D, rho, mu)),
     rand=_rand_mtc,
     phrases=("Mass-transfer coefficient", "Mass transfer coefficient", "Sherwood correlation"),
     examples=[_ex({"geometry": "sphere", "u": 0.1, "L": 0.001, "D": 5e-9, "rho": 1000.0, "mu": 0.001}),
               _ex({"geometry": "pipe", "u": 1.0, "L": 0.025, "D": 1.5e-9, "rho": 998.0, "mu": 0.001})],
     level="intermediate",
     extra_q=[{"q": "Mass-transfer coefficient: geometry = pipe, u = 0.01 m/s, L = 0.025 m, D = 1.5e-9 m^2/s, rho = 998 kg/m^3, mu = 0.001 Pa*s", "outcome": "INVALID_INPUT"}])


def _underwood_theta(alpha, z, q, a_lo, a_hi):
    f = lambda th: sum(a * zi / (a - th) for a, zi in zip(alpha, z)) - (1 - q)
    lo, hi = a_lo, a_hi
    for _ in range(3000):                       # f rises from -inf to +inf between adjacent poles: bisection inside
        m = 0.5 * (lo + hi)
        if m <= lo or m >= hi:
            break
        if f(m) > 0:
            hi = m
        else:
            lo = m
    return 0.5 * (lo + hi)


def underwood_multi(alpha, z, xD, q, lk, hk):
    n = len(alpha)
    _need(2 <= n <= 10 and len(z) == n and len(xD) == n, "alpha, z and xD need one entry per component (2 to 10)")
    _need(isinstance(lk, int) and isinstance(hk, int) and 1 <= lk <= n and 1 <= hk <= n and lk != hk, "lk and hk are component numbers (1 to n)")
    for a in alpha:
        _need(math.isfinite(a) and a > 0, "every relative volatility must be positive")
    _fracs("z", z, tol=1e-6)
    _fracs("xD", xD, tol=1e-6)
    _need(-5 <= q <= 5, "the feed quality q must lie between -5 and 5")
    a_lk, a_hk = alpha[lk - 1], alpha[hk - 1]
    _need(a_lk > a_hk, "the light key must be more volatile than the heavy key")
    _need(not any(a_hk < a < a_lk for a in alpha), "the keys must be adjacent in volatility (no component between them)")
    _need(len(set(alpha)) == n, "relative volatilities must be distinct")
    th = _underwood_theta(alpha, z, q, a_hk, a_lk)
    Rm = sum(a * x / (a - th) for a, x in zip(alpha, xD)) - 1.0
    _need(Rm > 0, "no positive minimum reflux for this distillate")
    return _out(theta=th, Rmin=Rm)


def _ref_underwood_multi(alpha, z, xD, q, lk, hk):
    """Underwood's feed equation cleared of its denominators into a polynomial in theta (numpy.polynomial), its roots
    by the companion-matrix eigenvalues; the root between the keys gives R_min."""
    import numpy as np
    from numpy.polynomial import polynomial as P
    num = np.zeros(1)
    den = np.array([1.0])
    for a in alpha:
        den = P.polymul(den, [a, -1.0])                                  # prod (a - theta)
    total = -(1 - q) * den
    for k, (a, zi) in enumerate(zip(alpha, z)):
        others = np.array([1.0])
        for j, b in enumerate(alpha):
            if j != k:
                others = P.polymul(others, [b, -1.0])
        total = P.polyadd(total, a * zi * others)
    a_lk, a_hk = alpha[lk - 1], alpha[hk - 1]
    roots = [r.real for r in P.polyroots(total) if abs(r.imag) < 1e-9 and a_hk < r.real < a_lk]
    th = roots[0]
    for _ in range(4):                                                   # polish on the original equation (Newton)
        f = sum(a * zi / (a - th) for a, zi in zip(alpha, z)) - (1 - q)
        df = sum(a * zi / (a - th) ** 2 for a, zi in zip(alpha, z))
        th -= f / df
    return {"theta": th, "Rmin": sum(a * x / (a - th) for a, x in zip(alpha, xD)) - 1.0}


def _rand_underwood(rng):
    n = rng.randint(3, 5)
    alpha = sorted(set(round(rng.uniform(0.3, 6.0), 3) for _ in range(n)), reverse=True)
    while len(alpha) < 3:
        alpha = sorted(set(alpha + [round(rng.uniform(0.3, 6.0), 3)]), reverse=True)
    n = len(alpha)
    k = rng.randint(0, n - 2)                                            # light key index k, heavy key k + 1
    z = _dirichlet(rng, n)
    rec_l, rec_h = rng.uniform(0.9, 0.995), rng.uniform(0.005, 0.1)
    d = [z[i] * (1.0 if i < k else (rec_l if i == k else (rec_h if i == k + 1 else 0.0))) for i in range(n)]
    s = sum(d)
    return {"alpha": alpha, "z": z, "xD": [v / s for v in d], "q": rng.choice([1.0, 0.0, rng.uniform(0.2, 1.2)]), "lk": k + 1, "hk": k + 2}


_add("chemeng_multicomponent_underwood",
     name="Minimum reflux of a multicomponent column (Underwood)", topic="separations / distillation",
     purpose="The minimum reflux ratio of a multicomponent distillation with constant relative volatilities by Underwood's equations: the root theta between the key volatilities of sum alpha_i z_i/(alpha_i - theta) = 1 - q, then R_min + 1 = sum alpha_i x_D,i/(alpha_i - theta).",
     inputs=[_vecin("alpha", "relative volatility of each component (any common reference, constant)", 2, 10),
             _vecin("z", "feed mole fractions (sum to 1)", 2, 10), _vecin("xD", "distillate mole fractions at minimum reflux (sum to 1)", 2, 10),
             _numin("q", "feed quality q (1 saturated liquid, 0 saturated vapour)"),
             _intin("lk", "light key: its position in the lists (1 = first)", 1, 10), _intin("hk", "heavy key: its position in the lists", 1, 10)],
     outputs=[_o("theta", "number", "Underwood root between the key volatilities"), _o("Rmin", "number", "minimum reflux ratio L/D")],
     primary="Rmin", conditions=["constant relative volatilities and molar overflow", "adjacent keys (no component between them in volatility)",
                                 "the distillate composition at minimum reflux is stated"],
     method="sum alpha_i z_i/(alpha_i - theta) = 1 - q, theta between alpha_HK and alpha_LK (bisection); R_min = sum alpha_i x_Di/(alpha_i - theta) - 1",
     latex=r"\sum_i \frac{\alpha_i z_i}{\alpha_i - \theta} = 1 - q,\quad R_{min} + 1 = \sum_i \frac{\alpha_i x_{D,i}}{\alpha_i - \theta}",
     run=underwood_multi, reference=_ref_underwood_multi,
     wrong=lambda alpha, z, xD, q, lk, hk: (lambda o: o | _out(Rmin=o["Rmin"] + 1.0))(underwood_multi(alpha, z, xD, q, lk, hk)),
     rand=_rand_underwood,
     phrases=("Multicomponent minimum reflux", "Multicomponent Underwood", "Underwood multicomponent minimum reflux"),
     examples=[_ex({"alpha": [2.5, 1.0, 0.5], "z": [0.4, 0.3, 0.3], "xD": [0.9, 0.1, 0.0], "q": 1.0, "lk": 1, "hk": 2}),
               _ex({"alpha": [4.0, 2.0, 1.0, 0.6], "z": [0.1, 0.3, 0.3, 0.3], "xD": [0.24, 0.7, 0.06, 0.0], "q": 0.5, "lk": 2, "hk": 3})],
     level="advanced",
     extra_q=[{"q": "Multicomponent minimum reflux: alpha = [2.5, 1.0, 1.5], z = [0.4, 0.3, 0.3], xD = [0.9, 0.1, 0.0], q = 1, lk = 1, hk = 2", "outcome": "INVALID_INPUT"}])


def cstr_volume_2(v0, k, CA0, X):
    _pos("v0, k and CA0", v0, k, CA0)
    _conv_ok(X)
    tau = X / (k * CA0 * (1 - X) ** 2)
    return _out(V=v0 * tau, tau=tau)


def _ref_cstr_volume_2(v0, k, CA0, X):
    """A CSTR started full of feed integrated to steady state for a trial Damkohler number (scipy DOP853); brentq on
    the Damkohler number until the steady conversion is X."""
    if X <= 0:
        return {"V": 0.0, "tau": 0.0}
    g = X / (1 - X) ** 2                               # a bracket around the expected Damkohler number (speed only:
    lo, hi = 0.5 * g, 2.0 * g                          # the root is wherever the simulated steady state gives X)
    f = lambda d: (1.0 - _startup(d, 2)) - X           # noqa: E731
    while f(lo) > 0:
        lo *= 0.5
    while f(hi) < 0:
        hi *= 2.0
    da = _brentq(f, lo, hi)
    tau = da / (k * CA0)
    return {"V": v0 * tau, "tau": tau}


_add("chemeng_cstr_volume_second_order",
     name="CSTR volume for a conversion (second order)", topic="reactor design / continuous stirred tanks",
     purpose="Volume and space time of a CSTR that reaches conversion X when -r_A = k C_A^2 (liquid phase, constant density).",
     inputs=[_V0, _K2(), _CA0, _XIN], outputs=[_o("V", "volume", "reactor volume"), _o("tau", "time", "space time V/v0")], primary="V",
     conditions=["0 <= X < 1", "steady state, perfect mixing, constant density, -r_A = k C_A^2"],
     method="design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k C_A0 (1 - X)^2)",
     latex=r"V = \frac{v_0 X}{k C_{A0}(1 - X)^2}",
     run=cstr_volume_2, reference=_ref_cstr_volume_2,
     wrong=lambda v0, k, CA0, X: (lambda tau: _out(V=v0 * tau, tau=tau))(X / (k * CA0 * (1 - X))),
     rand=lambda rng: {"v0": _lu(rng, 1e-4, 1.0), "k": _lu(rng, 1e-7, 1e-2), "CA0": _lu(rng, 10, 5000), "X": rng.uniform(0.05, 0.97)},
     phrases=("Second-order CSTR volume", "CSTR volume second order"),
     examples=[_ex({"v0": 0.01, "k": 1e-5, "CA0": 1000.0, "X": 0.8}), _ex({"v0": 0.002, "k": 1e-4, "CA0": 200.0, "X": 0.5})],
     level="intermediate",
     extra_q=[{"q": "Second-order CSTR volume: v0 = 0.01 m^3/s, k = 1e-5 m^3/(mol*s), CA0 = 1000 mol/m^3, X = 100%", "outcome": "INVALID_INPUT"}])


def equimolar_flux(D, P, T, L, yA1, yA2):
    _pos("D, P, T and L", D, P, T, L)
    _frac("yA1", yA1)
    _frac("yA2", yA2)
    _need(yA1 > yA2, "A diffuses from yA1 to yA2: yA1 must exceed yA2")
    c = P / (R_GAS * T)
    return _out(NA=c * D * (yA1 - yA2) / L, NB=-c * D * (yA1 - yA2) / L)


def _ref_equimolar_flux(D, P, T, L, yA1, yA2):
    """Shooting: the film equation dy/dz = -N_A/(c D) (equimolar counterdiffusion, N_B = -N_A) integrated across the film
    for a trial flux (scipy DOP853); brentq on the flux until the composition at z = L is yA2."""
    c = P / (R_GAS * T)
    y_end = lambda N: _ivp(lambda s, y: [-N / (c * D)], L, [yA1], atol=[1e-18])[0]
    hi = c * D / L * 2.0
    NA = _brentq(lambda N: y_end(N) - yA2, 0.0, hi)
    return {"NA": NA, "NB": -NA}


_add("chemeng_equimolar_counterdiffusion",
     name="Equimolar counterdiffusion flux", topic="transport / mass transfer",
     purpose="The molar flux of A through a film of thickness L in equimolar counterdiffusion (N_B = -N_A, ideal gas): N_A = c D (y_A1 - y_A2)/L with c = P/(R T).",
     inputs=[_phys("D", "diffusion_coefficient", "diffusivity of A in B"), _phys("P", "pressure", "total pressure"), _phys("T", "temperature", "temperature"),
             _phys("L", "length", "film (path) thickness"), _numin("yA1", "mole fraction of A at z = 0", pct=True), _numin("yA2", "mole fraction of A at z = L", pct=True)],
     outputs=[_o("NA", "molar_flux", "molar flux of A"), _o("NB", "molar_flux", "molar flux of B (= -N_A)")], primary="NA",
     conditions=["steady state, ideal gas, constant D", "equimolar counterdiffusion (N_B = -N_A)", "yA1 > yA2"],
     method="N_A = (P/(R T)) D (y_A1 - y_A2)/L", latex=r"N_A = \frac{P}{RT}\,\frac{D_{AB}\,(y_{A1} - y_{A2})}{L}",
     run=equimolar_flux, reference=_ref_equimolar_flux,
     wrong=lambda D, P, T, L, yA1, yA2: (lambda c: _out(NA=c * D / L * math.log((1.0 - yA2) / (1.0 - yA1)), NB=-c * D / L * math.log((1.0 - yA2) / (1.0 - yA1))))(P / (R_GAS * T)),
     rand=lambda rng: (lambda y1: {"D": _lu(rng, 5e-6, 1e-4), "P": rng.uniform(2e4, 1e6), "T": rng.uniform(250, 700), "L": _lu(rng, 1e-4, 0.5),
                                   "yA1": y1, "yA2": y1 * rng.uniform(0.0, 0.8)})(rng.uniform(0.1, 0.9)),
     phrases=("Equimolar counterdiffusion flux", "Equimolar counterdiffusion", "Equimolar counter-diffusion"),
     examples=[_ex({"D": 2.3e-5, "P": 101325.0, "T": 298.15, "L": 0.1, "yA1": 0.2, "yA2": 0.05}), _ex({"D": 1e-5, "P": 200000.0, "T": 400.0, "L": 0.02, "yA1": 0.6, "yA2": 0.0})],
     level="intermediate",
     extra_q=[{"q": "Equimolar counterdiffusion flux: D = 2.3e-5 m^2/s, P = 101325 Pa, T = 298.15 K, L = 0.1 m, yA1 = 0.05, yA2 = 0.2", "outcome": "INVALID_INPUT"}])


def modified_raoult_bubble(x, gamma, Psat, p_unit):
    n = len(x)
    _need(2 <= n <= 10 and len(gamma) == n and len(Psat) == n, "x, gamma and Psat need one entry per component (2 to 10)")
    _fracs("x", x)
    for g in gamma:
        _need(math.isfinite(g) and 0 < g <= 100, "every activity coefficient must lie in (0, 100]")
    for p in Psat:
        _need(math.isfinite(p) and p > 0, "every vapour pressure must be positive")
    f = _PUNIT[p_unit]
    parts = [xi * gi * pi * f for xi, gi, pi in zip(x, gamma, Psat)]
    P = sum(parts)
    y = [v / P for v in parts]
    return _out(P=P, y1=y[0], y=y)


def _ref_modified_raoult_bubble(x, gamma, Psat, p_unit):
    """The bubble pressure as the dot product x . (gamma * Psat) (numpy), the vapour mole fractions normalised."""
    import numpy as np
    f = {"Pa": 1.0, "kPa": 1e3, "MPa": 1e6, "bar": 1e5, "atm": 101325.0, "psi": 6894.757293168361}[p_unit]
    parts = np.asarray(x, float) * np.asarray(gamma, float) * np.asarray(Psat, float) * f
    P = float(np.dot(np.asarray(x, float), np.asarray(gamma, float) * np.asarray(Psat, float) * f))
    y = [float(v) / P for v in parts]
    return {"P": P, "y1": y[0], "y": y}


_add("chemeng_modified_raoult_bubble_pressure",
     name="Bubble-point pressure by the modified Raoult's law (activity coefficients)", topic="separations / vapour-liquid equilibrium",
     purpose="The bubble-point pressure P = sum x_i gamma_i P_i^sat of a non-ideal liquid with stated activity coefficients and vapour pressures (low pressure, ideal vapour), and the vapour composition y_i = x_i gamma_i P_i^sat/P.",
     inputs=[_vecin("x", "liquid mole fractions (sum to 1)", 2, 10), _vecin("gamma", "activity coefficient of each component at this composition", 2, 10),
             _vecin("Psat", "vapour pressure of each pure component at the temperature, in the unit p_unit", 2, 10),
             _choice("p_unit", list(_PUNIT), "unit of the vapour pressures: Pa, kPa, MPa, bar, atm or psi")],
     outputs=[_o("P", "pressure", "bubble-point pressure"), _o("y1", "number", "vapour mole fraction of the first component"),
              _o("y", "number", "vapour mole fractions, in the order of x")],
     primary="P", conditions=["low pressure (ideal vapour, Poynting factor 1)", "activity coefficients stated for this liquid composition and temperature"],
     method="P = sum x_i gamma_i P_i^sat, y_i = x_i gamma_i P_i^sat/P", latex=r"P = \sum_i x_i\gamma_i P_i^{sat},\quad y_i = \frac{x_i\gamma_i P_i^{sat}}{P}",
     run=modified_raoult_bubble, reference=_ref_modified_raoult_bubble,
     wrong=lambda x, gamma, Psat, p_unit: (lambda P: _out(P=P, y1=x[0] * Psat[0] * _PUNIT[p_unit] / P, y=[xi * pi * _PUNIT[p_unit] / P for xi, pi in zip(x, Psat)]))(
         sum(xi * pi * _PUNIT[p_unit] for xi, pi in zip(x, Psat))),
     rand=lambda rng: (lambda n: {"x": _dirichlet(rng, n), "gamma": [rng.uniform(1.05, 3.0) for _ in range(n)], "Psat": [_lu(rng, 1.0, 500.0) for _ in range(n)],
                                  "p_unit": rng.choice(["kPa", "bar", "Pa"])})(rng.randint(2, 5)),
     phrases=("Modified Raoult bubble pressure", "Bubble pressure with activity coefficients", "Modified Raoult's law"),
     examples=[_ex({"x": [0.4, 0.6], "gamma": [1.8, 1.2], "Psat": [55.0, 30.0], "p_unit": "kPa"}),
               _ex({"x": [0.2, 0.3, 0.5], "gamma": [2.0, 1.1, 1.05], "Psat": [1.2, 0.8, 0.3], "p_unit": "bar"})],
     level="intermediate",
     extra_q=[{"q": "Modified Raoult bubble pressure: x = [0.4, 0.7], gamma = [1.8, 1.2], Psat = [55, 30], p_unit = kPa", "outcome": "INVALID_INPUT"}])


# @@END@@


# ================================================================================================ worked examples
# The documented outputs of every worked example, computed by the procedure's INDEPENDENT reference (the checking
# libraries, on the build machine) and rounded to 12 significant digits; certification checks that `run` reproduces
# them, and the questions written from the examples must give them through the question reader.
# @@EXV-BEGIN@@
_EXV = {
    'chemeng_absorber_transfer_units': [
        {'NOG': 5.97729049407, 'Z': 3.58637429644},
        {'NOG': 3.54695484878, 'Z': 3.54695484878},
    ],
    'chemeng_adiabatic_flame_temperature': [
        {'T_ad': 2326.32176453},
        {'T_ad': 1906.88271101},
    ],
    'chemeng_adiabatic_reactor_temperature': [
        {'T_out': 932.348285267},
        {'T_out': 717.39341563},
    ],
    'chemeng_antoine_boiling_temperature': [
        {'Tsat': 373.146825762},
        {'Tsat': 331.816729309},
    ],
    'chemeng_antoine_vapour_pressure': [
        {'Psat': 101336.529379},
        {'Psat': 91457.8865716},
    ],
    'chemeng_batch_coil_cooling': [
        {'T': 312.732212458},
        {'T': 327.845444426},
    ],
    'chemeng_batch_conversion_first_order': [
        {'X': 0.632120558829},
        {'X': 0.698805788088},
    ],
    'chemeng_batch_conversion_second_order': [
        {'X': 0.8, 'CA': 200.0},
        {'X': 0.857142857143, 'CA': 285.714285714},
    ],
    'chemeng_batch_heating_temperature': [
        {'T': 375.287882174},
        {'T': 305.675572779},
    ],
    'chemeng_batch_heating_time': [
        {'t': 1915.04762962},
        {'t': 2183.52162744},
    ],
    'chemeng_batch_time_first_order': [
        {'t': 2302.58509299},
        {'t': 13.8629436112},
    ],
    'chemeng_batch_time_second_order': [
        {'t': 40.0},
        {'t': 100.0},
    ],
    'chemeng_binary_minimum_reflux': [
        {'Rmin': 1.1},
        {'Rmin': 2.26136699197},
    ],
    'chemeng_binary_separator_balance': [
        {'D': 38.8888888889, 'B': 61.1111111111, 'recovery_D': 0.923611111111, 'recovery_B': 0.967592592593},
        {'D': 0.795454545455, 'B': 1.70454545455, 'recovery_D': 0.954545454545, 'recovery_B': 0.954545454545},
    ],
    'chemeng_blending_flows': [
        {'F1': 42.8571428571, 'F': [42.8571428571, 57.1428571429]},
        {'F1': 4.4578313253, 'F': [4.4578313253, 3.13253012048, 2.40963855422]},
    ],
    'chemeng_cake_filtration_volume': [
        {'V': 0.24},
        {'V': 8.28763806957},
    ],
    'chemeng_carman_pressure_drop': [
        {'dP': 134368.273068},
        {'dP': 13920.1174625},
    ],
    'chemeng_closed_loop_stability': [
        {'max_real': 0.0772173450159, 'n_rhp': 2, 'stable': 'unstable'},
        {'max_real': -0.206299474016, 'n_rhp': 0, 'stable': 'stable'},
    ],
    'chemeng_combustion_flue_gas': [
        {'O2_theoretical': 2.0, 'air': 11.4285714286, 'n_CO2': 1.0, 'n_H2O': 2.0, 'n_SO2': 0.0, 'n_O2': 0.4, 'n_N2': 9.02857142857, 'y_CO2_dry': 0.0958904109589, 'y_O2_dry': 0.0383561643836, 'y_H2O_wet': 0.16091954023, 'AFR_mass': 20.5528393312},
        {'O2_theoretical': 5.0, 'air': 23.8095238095, 'n_CO2': 3.0, 'n_H2O': 4.0, 'n_SO2': 0.0, 'n_O2': 0.0, 'n_N2': 18.8095238095, 'y_CO2_dry': 0.137554585153, 'y_O2_dry': 0.0, 'y_H2O_wet': 0.154981549815, 'AFR_mass': 15.5777992074},
        {'O2_theoretical': 3.0, 'air': 21.4285714286, 'n_CO2': 2.0, 'n_H2O': 3.0, 'n_SO2': 0.0, 'n_O2': 1.5, 'n_N2': 16.9285714286, 'y_CO2_dry': 0.0979020979021, 'y_O2_dry': 0.0734265734266, 'y_H2O_wet': 0.128048780488, 'AFR_mass': 13.4196305075},
    ],
    'chemeng_countercurrent_extraction': [
        {'XN': 0.013437970329, 'Y1': 0.108202537089, 'recovery': 0.86562029671},
        {'XN': 0.05375, 'Y1': 0.3025, 'recovery': 0.73125},
    ],
    'chemeng_crosscurrent_extraction': [
        {'XN': 0.0350127796646, 'recovery': 0.649872203354},
        {'XN': 0.0666666666667, 'recovery': 0.666666666667},
    ],
    'chemeng_cstr_conversion_first_order': [
        {'X': 0.666666666667},
        {'X': 0.714285714286},
    ],
    'chemeng_cstr_conversion_second_order': [
        {'X': 0.5, 'CA': 500.0},
        {'X': 0.267949192431, 'CA': 366.025403784},
    ],
    'chemeng_cstr_volume_first_order': [
        {'V': 90.0, 'tau': 9000.0},
        {'V': 0.04, 'tau': 20.0},
    ],
    'chemeng_cstrs_in_series_first_order': [
        {'X': 0.703703703704},
        {'X': 0.802469135802},
    ],
    'chemeng_cstrs_in_series_second_order': [
        {'X': 0.674358784586, 'CA': 325.641215414},
        {'X': 0.461462316893, 'CA': 269.268841554},
    ],
    'chemeng_ergun_pressure_drop': [
        {'dP': 117187.5},
        {'dP': 15483.0966613},
    ],
    'chemeng_exponential_decline': [
        {'q': 0.00100988515564, 'Np': 87335.1922824},
        {'q': 0.00606530659713, 'Np': 1241694.80531},
    ],
    'chemeng_filtration_constants_fit': [
        {'alpha': 807822147431.0, 'Rm': 467973433197.0, 'slope': 13024952.381, 'intercept': 28226.8888889},
        {'alpha': 140400000000.0, 'Rm': 249900000000.0, 'slope': 3510.0, 'intercept': 833.0},
    ],
    'chemeng_first_order_step_fit': [
        {'K': 1.86707414832, 'tau': 292.778137661},
        {'K': -2.52644722528, 'tau': 46.090537622},
    ],
    'chemeng_fopdt_step_response': [
        {'y': 6.32120558829},
        {'y': -7.93078643838},
    ],
    'chemeng_gas_pz_material_balance': [
        {'G': 4993507776.93, 'pz_initial': 26084853.1419},
        {'G': 1867842801.71, 'pz_initial': 29420882.5143},
    ],
    'chemeng_gas_well_backpressure': [
        {'n': 0.727757448882, 'log10_C': -8.06552746592, 'AOF': 20.9729077823},
        {'n': 0.896485387548, 'log10_C': -12.3769193596, 'AOF': 5.17147048914},
    ],
    'chemeng_harmonic_decline': [
        {'q': 0.000836423058333, 'Np': 114464.513077},
        {'q': 0.000909090909091, 'Np': 756718.198609},
    ],
    'chemeng_havlena_odeh_gas_cap': [
        {'N': 50598337.5128, 'm': 0.226683291771},
        {'N': 23617328.5199, 'm': 0.434882298991},
    ],
    'chemeng_havlena_odeh_oil_in_place': [
        {'N': 9161201.20904},
        {'N': 30104370.5153},
    ],
    'chemeng_horner_buildup': [
        {'m': 1239285.68682, 'k': 1.07114768102e-14, 'p_star': 23388029.284},
        {'m': 913808.680837, 'k': 4.01033396952e-14, 'p_star': 22235977.3899},
    ],
    'chemeng_hyperbolic_decline': [
        {'q': 0.000600859013333, 'Np': 99548.7590956},
        {'q': 0.0006415002991, 'Np': 666890.557251},
    ],
    'chemeng_kremser_absorber': [
        {'y_out': 0.00306300478319, 'recovery': 0.938739904336},
        {'y_out': 0.006125, 'recovery': 0.69375},
    ],
    'chemeng_kremser_stripper': [
        {'x_out': 0.000310830500243, 'recovery': 0.968916949976},
        {'x_out': 0.0183864826273, 'recovery': 0.632270347454},
    ],
    'chemeng_lag_frequency_response': [
        {'AR': 1.41421356237, 'phase_deg': -45.0},
        {'AR': 0.173594368634, 'phase_deg': -152.723236617},
    ],
    'chemeng_lee_kesler_vapour_pressure': [
        {'Psat': 104829.456804},
        {'Psat': 258309.661443},
    ],
    'chemeng_line_source_drawdown': [
        {'dp': 1027328.62265, 'p': 28972671.3773},
        {'dp': 241753.411809, 'p': 24758246.5882},
    ],
    'chemeng_mass_to_mole_fractions': [
        {'z': [0.359889725713, 0.640110274287], 'MW_mean': 0.0230631731826},
        {'z': [0.369029335988, 0.2953277837, 0.335642880312], 'MW_mean': 0.0296016881862},
    ],
    'chemeng_mccabe_thiele_stages': [
        {'N': 11, 'Rmin': 1.1},
        {'N': 14, 'Rmin': 2.26136699197},
    ],
    'chemeng_nrtl_activity_coefficients': [
        {'gamma1': 2.25346353086, 'gamma': [2.25346353086, 1.18777953647], 'GE_RT': 0.364200453756},
        {'gamma1': 1.82200497499, 'gamma': [1.82200497499, 1.44773823655, 1.13894017298], 'GE_RT': 0.307534086359},
    ],
    'chemeng_p_control_offset': [
        {'y_ss': 0.75, 'offset': 0.25},
        {'y_ss': 0.5, 'offset': 0.5},
    ],
    'chemeng_peng_robinson': [
        {'Z_vapour': 0.644021422402, 'Z_liquid': 0.046928223733, 'phi_vapour': 0.746231948789, 'phi_liquid': 0.421597007858, 'n_roots': 3},
        {'Z_vapour': 0.901759542654, 'Z_liquid': 0.901759542654, 'phi_vapour': 0.901311515296, 'phi_liquid': 0.901311515296, 'n_roots': 1},
    ],
    'chemeng_pfr_conversion_first_order': [
        {'X': 0.864664716763},
        {'X': 0.917915001376},
    ],
    'chemeng_pfr_conversion_second_order': [
        {'X': 0.666666666667, 'CA': 333.333333333},
        {'X': 0.333333333333, 'CA': 333.333333333},
    ],
    'chemeng_pfr_gas_volume_first_order': [
        {'V': 12.0943791243},
        {'V': 7.58145365937},
    ],
    'chemeng_pfr_gas_volume_second_order': [
        {'V': 64.7640521891},
        {'V': 9.83145365937},
    ],
    'chemeng_pfr_volume_first_order': [
        {'V': 23.0258509299},
        {'V': 0.0277258872224},
    ],
    'chemeng_pfr_volume_second_order': [
        {'V': 4.0},
        {'V': 2.0},
    ],
    'chemeng_pi_loop_margins': [
        {'GM': 3.22093727123, 'PM_deg': 51.0530801489, 'w_pc': 0.0262499991931, 'w_gc': 0.0131316858305},
        {'GM': 12.9321211119, 'PM_deg': 90.0228956648, 'w_pc': 0.3657029333, 'w_gc': 0.044620469291},
    ],
    'chemeng_pid_controller_output': [
        {'u_last': 50.72, 'u': [54.2, 51.56, 50.86, 50.72]},
        {'u_last': 40.3666666667, 'u': [41.0333333333, 41.0666666667, 40.8416666667, 40.6083333333, 40.3666666667]},
    ],
    'chemeng_pitzer_heat_of_vaporization': [
        {'Hvap': 36690.9794714},
        {'Hvap': 20662.4782953},
    ],
    'chemeng_rachford_rice_flash': [
        {'VF': 0.690730262774, 'x': [0.339408696966, 0.365056059037, 0.295535243996], 'y': [0.571903654388, 0.270871595806, 0.157224749806]},
        {'VF': 0.335818485819, 'x': [0.096403236013, 0.483754625418, 0.419842138569], 'y': [0.404893591254, 0.53213008796, 0.0629763207853]},
    ],
    'chemeng_rackett_liquid_volume': [
        {'Vm': 8.29922500546e-05},
        {'Vm': 8.85568089481e-05},
    ],
    'chemeng_radial_darcy_flow': [
        {'q': 0.00160651147542, 'PI': 4.66009580066e-10},
        {'q': 0.00149852904057, 'PI': 2.14075577225e-10},
    ],
    'chemeng_raoult_bubble_pressure': [
        {'P': 100906.421428, 'y': [0.621960532969, 0.378039467031]},
        {'P': 122162.230198, 'y': [0.441901544104, 0.558098455896]},
    ],
    'chemeng_raoult_bubble_temperature': [
        {'T': 368.291694698, 'y': [0.62184905372, 0.37815094628]},
        {'T': 338.023763894, 'y': [0.865597485424, 0.134402514576]},
    ],
    'chemeng_raoult_dew_pressure': [
        {'P': 83425.964355, 'x': [0.212686334545, 0.787313665455]},
        {'P': 139298.495775, 'x': [0.387057256563, 0.612942743437]},
    ],
    'chemeng_raoult_dew_temperature': [
        {'T': 374.634655031, 'x': [0.216267034105, 0.783732965895]},
        {'T': 343.960412332, 'x': [0.464057188008, 0.535942811992]},
    ],
    'chemeng_raoult_isothermal_flash': [
        {'VF': 0.43053470726, 'x': [0.404485322459, 0.595514677541], 'y': [0.626336606294, 0.373663393706]},
        {'VF': 0.196310670147, 'x': [0.260617931568, 0.739382068432], 'y': [0.461228873413, 0.538771126587]},
    ],
    'chemeng_reaction_enthalpy_at_temperature': [
        {'dH': -105497.164754},
        {'dH': -247796.91869},
    ],
    'chemeng_recycle_purge': [
        {'R': 221.538461538, 'P': 24.6153846154, 'reactor_feed': 321.538461538, 'X_overall': 0.769230769231, 'product': 75.3846153846, 'y_inert': 0.08125},
        {'R': 122.580645161, 'P': 6.45161290323, 'reactor_feed': 172.580645161, 'X_overall': 0.967741935484, 'product': 43.5483870968, 'y_inert': 0.775},
    ],
    'chemeng_recycle_reactor_first_order': [
        {'X': 0.739800310274},
        {'X': 0.917915001376},
    ],
    'chemeng_reversible_first_order_batch': [
        {'CA': 604.37977316, 'CB': 395.62022684, 'X': 0.39562022684, 'X_eq': 0.666666666667},
        {'CA': 484.462603203, 'CB': 115.537396797, 'X': 0.0310747935941, 'X_eq': 0.04},
    ],
    'chemeng_riedel_heat_of_vaporization': [
        {'Hvap_b': 26828.5904073},
        {'Hvap_b': 30639.0875944},
    ],
    'chemeng_rtd_moments': [
        {'t_mean': 307.640671273, 'variance': 21424.3447229, 'N_tanks': 4.41753453118},
        {'t_mean': 20.3550295858, 'variance': 144.992297188, 'N_tanks': 2.85758097135},
    ],
    'chemeng_sensible_heat_cp_polynomial': [
        {'Q': 59600.0, 'cp_mean': 29.8},
        {'Q': 38233.9339758, 'cp_mean': 54.4759335695},
    ],
    'chemeng_series_reaction_maximum': [
        {'t_max': 693.14718056, 'CB_max': 500.0, 'CA': 250.0, 'CC': 250.0},
        {'t_max': 40.2359478109, 'CB_max': 26.7496121991, 'CA': 133.748060995, 'CC': 39.5023268057},
    ],
    'chemeng_srk': [
        {'Z_vapour': 0.658909723032, 'Z_liquid': 0.0532183627277, 'phi_vapour': 0.757696139207, 'phi_liquid': 0.430644415808, 'n_roots': 3},
        {'Z_vapour': 0.923848022363, 'Z_liquid': 0.923848022363, 'phi_vapour': 0.923117330729, 'phi_liquid': 0.923117330729, 'n_roots': 1},
    ],
    'chemeng_stefan_diffusion_flux': [
        {'NA': 0.000337949921666},
        {'NA': 0.0952759357682},
    ],
    'chemeng_uniquac_activity_coefficients': [
        {'gamma1': 1.29868568479, 'gamma': [1.29868568479, 1.10358464339]},
        {'gamma1': 1.19549838541, 'gamma': [1.19549838541, 1.20648140089, 1.47615584685]},
    ],
    'chemeng_unsteady_mixing_tank': [
        {'C': 316.060279414},
        {'C': 132.239110577},
    ],
    'chemeng_variable_volume_tank': [
        {'V': 2.0, 'C': 750.0},
        {'V': 3.0, 'C': 387.298334621},
    ],
    'chemeng_virial_compressibility': [
        {'B': -0.000205361258975, 'Z': 0.878925087003},
        {'B': -0.000122712129965, 'Z': 0.95080374379},
    ],
    'chemeng_watson_heat_of_vaporization': [
        {'Hvap': 44571.7379903},
        {'Hvap': 27898.5963476},
    ],
    'chemeng_wilson_activity_coefficients': [
        {'gamma1': 1.05781025716, 'gamma': [1.05781025716, 1.02099043889], 'GE_RT': 0.0314015151651},
        {'gamma1': 1.57516694787, 'gamma': [1.57516694787, 0.885212689617, 1.13570117122], 'GE_RT': 0.117919168476},
    ],
    'chemeng_wilson_k_flash': [
        {'VF': 0.3623319151, 'x': [0.0449443371671, 0.487795919806, 0.467259743027], 'y': [0.748872564861, 0.245487924989, 0.00563951014995], 'K': [16.6622229198, 0.503259488285, 0.0120693259672]},
        {'VF': 0.0518040574017, 'x': [0.480912489953, 0.519087510047], 'y': [0.849368379403, 0.150631620597], 'K': [1.7661599504, 0.290185407435]},
    ],
    'chemeng_ziegler_nichols_tuning': [
        {'Ku': 4.0, 'Pu': 217.655923708, 'Kc': 2.4, 'tauI': 108.827961854, 'tauD': 27.2069904635},
        {'Ku': 8.4, 'Pu': 15.238962757, 'Kc': 3.78, 'tauI': 12.6991356308},
    ],
}
# @@EXV-END@@

# the worked examples of the definitional additions (computed by their references on the build machine)
_EXV.update({
    'chemeng_fenske_minimum_stages': [{"Nmin": 6.4268662265, "Nmin_trays": 5.4268662265}, {"Nmin": 22.6659199928, "Nmin_trays": 21.6659199928}],
    'chemeng_pump_head': [{"H": 30.5914863893, "dp_equivalent": 300000.0}, {"H": 60.9485342781, "dp_equivalent": 519999.821, "P_hydraulic": 10399.99642, "P_shaft": 14857.1377429}],
    'chemeng_npsh_available': [{"NPSHa": 11.6139908876, "pressure_head": 10.352980489, "vapour_head": 0.238989601418}, {"NPSHa": 1.85838616843, "pressure_head": 10.6320997407, "vapour_head": 4.97371357225, "margin": -0.141613831567}],
    'chemeng_mass_transfer_coefficient': [{"Re": 100.0, "Sc": 200.0, "Sh": 37.0882128586, "kc": 0.000185441064293}, {"Re": 24950.0, "Sc": 668.002672011, "Sh": 897.199712413, "kc": 5.38319827448e-05}],
    'chemeng_multicomponent_underwood': [{"theta": 1.40263430628, "Rmin": 0.802000477364}, {"theta": 1.48443920513, "Rmin": 1.97325974168}],
    'chemeng_cstr_volume_second_order': [{"V": 20.0, "tau": 2000.0}, {"V": 0.2, "tau": 100.0}],
    'chemeng_equimolar_counterdiffusion': [{"NA": 0.00141015453606, "NB": -0.00141015453606}, {"NA": 0.0180408532564, "NB": -0.0180408532564}],
    'chemeng_modified_raoult_bubble_pressure': [{"P": 61200.0, "y1": 0.647058823529, "y": [0.647058823529, 0.352941176471]}, {"P": 90150.0, "y1": 0.532445923461, "y": [0.532445923461, 0.292845257903, 0.174708818636]}],
})

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)
