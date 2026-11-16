# Chemical Engineering (chemical-engineering-ai999x)

[![tests](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemical-engineering-ai999x)](https://pypi.org/project/chemical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemical-engineering-ai999x)

Chemical Engineering covers material and energy balances, reactor design, separations (distillation, absorption, extraction), transport phenomena, process control and reservoir and petroleum basics: 92 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Chemical Engineering desktop app: Bubble-point pressure (Raoult's law), with its worked example and live scene](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `chemical-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/chemical-engineering-ai999x/#try) |
| **Python** 3.10+ | `pip install chemical-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import chemical_engineering_ai999x as chemical_engineering

a = chemical_engineering.ask(
    "Adiabatic flame temperature: fuel = CH4, excess_air = 0.0, T_in = "
    "300.0 K"
)
print(a.outcome, a.values)
# SOLVED {'T_ad': 2326.32176453}

a = chemical_engineering.ask(
    "Closed-loop stability: num = [1.0], den = [1.0, 3.0, 3.0, 1.0], "
    "Kc = 10.0"
)
print(a.outcome, a.values)
# SOLVED {'max_real': 0.0772173450159, 'n_rhp': 2, 'stable': 'unstable'}

a = chemical_engineering.ask(
    "Virial compressibility: T = 510.0 K, P = 2500000.0 Pa, Tc = 425.1 "
    "K, Pc = 3796000.0 Pa, omega = 0.2"
)
print(a.outcome, a.values)
# SOLVED {'B': -0.000205361258975, 'Z': 0.878925087003}

a = chemical_engineering.ask(
    "First-order batch reactor conversion: k = 0.001 1/s, t = 1000.0 s"
)
print(a.outcome, a.values)
# SOLVED {'X': 0.632120558829}

a = chemical_engineering.ask(
    "Gas p/z material balance: p = [3200.0, 2925.0, 2525.0, 2125.0], z "
    "= [0.85, 0.84, 0.83, 0.82], Gp = [0.0, 15.0, 35.0, 55.0], p_unit "
    "= psi, gp_unit = Bscf"
)
print(a.outcome, a.values)
# SOLVED {'G': 4993507776.93, 'pz_initial': 26084853.1419}

a = chemical_engineering.ask(
    "Kremser absorber: N = 5, A = 1.4, m = 2.0, y_in = 0.05, x_in = "
    "0.0"
)
print(a.outcome, a.values)
# SOLVED {'y_out': 0.00306300478319, 'recovery': 0.938739904336}

a = chemical_engineering.ask(
    "Adiabatic reactor temperature: n0 = [1.0, 2.0, 0.0], nu = [-1.0, "
    "-1.0, 1.0], cp = [[30.0, 0.01], [29.0, 0.004], [45.0, 0.02]], dH "
    "= -60000.0 J/mol, T_ref = 298.15 K, T_in = 400.0 K, key = 1, X = "
    "0.8"
)
print(a.outcome, a.values)
# SOLVED {'T_out': 932.348285267}

a = chemical_engineering.ask(
    "Pump total head: p1 = 100000.0 Pa, p2 = 400000.0 Pa, rho = 1000.0 "
    "kg/m^3"
)
print(a.outcome, a.values)
# SOLVED {'H': 30.5914863893, 'dp_equivalent': 300000.0}

a = chemical_engineering.ask(
    "PID controller output: Kc = 2.0, tauI = 10.0 s, tauD = 1.0 s, dt "
    "= 1.0 s, e = [1.0, 0.8, 0.5, 0.3], bias = 50.0"
)
print(a.outcome, a.values)
# SOLVED {'u_last': 50.72, 'u': [54.2, 51.56, 50.86, 50.72]}

a = chemical_engineering.ask(
    "Watson heat of vaporization: T = 298.15 K, T_ref = 373.15 K, "
    "Hvap_ref = 40656.0 J/mol, Tc = 647.1 K"
)
print(a.outcome, a.values)
# SOLVED {'Hvap': 44571.7379903}

a = chemical_engineering.ask(
    "Second-order batch reactor conversion: k = 0.0001 m^3/(mol*s), "
    "CA0 = 1000.0 mol/m^3, t = 40.0 s"
)
print(a.outcome, a.values)
# SOLVED {'X': 0.8, 'CA': 200.0}

a = chemical_engineering.ask(
    "Havlena-Odeh gas cap: F = [5000000.0, 9300000.0, 14300000.0, "
    "19500000.0], Eo = [0.01, 0.018, 0.027, 0.036], Eg = [0.025, 0.05, "
    "0.08, 0.11], v_unit = bbl"
)
print(a.outcome, a.values)
# SOLVED {'N': 50598337.5128, 'm': 0.226683291771}

a = chemical_engineering.ask(
    "Stefan diffusion flux: D = 2.6e-05 m^2/s, P = 101325.0 Pa, T = "
    "298.15 K, L = 0.1 m, yA1 = 0.0313, yA2 = 0.0"
)
print(a.outcome, a.values)
# SOLVED {'NA': 0.000337949921666}

a = chemical_engineering.ask(
    "Batch coil cooling: M = 5000.0 kg, cp = 2500.0 J/(kg*K), W = 2.0 "
    "kg/s, cp_c = 4180.0 J/(kg*K), t_in = 288.15 K, UA = 3000.0 W/K, "
    "T0 = 393.15 K, t = 7200.0 s"
)
print(a.outcome, a.values)
# SOLVED {'T': 312.732212458}

a = chemical_engineering.ask(
    "Variable-volume tank: V0 = 1.0 m^3, q_in = 0.002 m^3/s, q_out = "
    "0.001 m^3/s, C_in = 1000.0 mol/m^3, C0 = 0.0 mol/m^3, t = 1000.0 "
    "s"
)
print(a.outcome, a.values)
# SOLVED {'V': 2.0, 'C': 750.0}
```

## Screenshots

[![The Chemical Engineering desktop app: Bubble-point pressure (Raoult's law), with its worked example and live scene](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/hero-dark.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/)

[![Batch reactor time for a conversion (second order): the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/see-1.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/)

*Batch reactor time for a conversion (second order)*

[![Heat of vaporisation at the normal boiling point (Riedel): the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/see-2.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/)

*Heat of vaporisation at the normal boiling point (Riedel)*

[![Reactor with recycle and purge: the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/see-3.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/)

*Reactor with recycle and purge*

Every calculation, with its formula, a worked example and its figures: **[Chemical Engineering on ai999x.com](https://www.ai999x.com/models/chemical-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/chemical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/chemical-engineering-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `chemical-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/chemical-engineering`) and its data (`rust/models/chemical_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p chemical-engineering-ai999x
./target/release/chemical-engineering --help
cargo test -p chemical-engineering-ai999x
```
