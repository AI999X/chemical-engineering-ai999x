# Chemical Engineering (chemical-engineering-ai999x)

[![tests](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemical-engineering-ai999x)](https://pypi.org/project/chemical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemical-engineering-ai999x)

Chemical Engineering covers material and energy balances, reactor design, separations (distillation, absorption, extraction), transport phenomena, process control and reservoir and petroleum basics: 92 calculations in 11 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Chemical Engineering desktop app: CSTR volume for a conversion (first order), with its worked example and live scene](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/hero.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-chemical-engineering-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-chemical-engineering-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `chemical-engineering` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/chemical-engineering-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install chemical-engineering-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Chemical Engineering desktop app: every scene and chart is drawn from the computed answer.

[![Chemical Engineering desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/clip.mp4)*

| | |
|---|---|
| [![CSTR volume for a conversion (first order): the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-1.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_cstr_volume_first_order)<br>CSTR volume for a conversion (first order) | [![Combustion with excess air: air requirement and flue-gas composition: the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-2.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_combustion_flue_gas)<br>Combustion with excess air: air requirement and flue-gas composition |
| [![Bubble-point temperature (Raoult's law): the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-3.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_raoult_bubble_temperature)<br>Bubble-point temperature (Raoult's law) | [![Steady radial flow of oil to a well (Darcy) and productivity index: the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-4.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_radial_darcy_flow)<br>Steady radial flow of oil to a well (Darcy) and productivity index |
| [![Net positive suction head available: the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-5.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_npsh_available)<br>Net positive suction head available | [![Closed-loop stability of a P-controlled process (closed-loop poles): the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-6.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_closed_loop_stability)<br>Closed-loop stability of a P-controlled process (closed-loop poles) |
| [![Equipment cost scaling with a capacity exponent and cost indices: the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-7.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_equipment_cost_scaling)<br>Equipment cost scaling with a capacity exponent and cost indices | [![Gas compressibility from the virial equation (Pitzer-Abbott correlation): the live scene in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-8.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_virial_compressibility)<br>Gas compressibility from the virial equation (Pitzer-Abbott correlation) |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![CSTR volume for a conversion (first order): a chart in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/chart-1.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_cstr_volume_first_order)<br>CSTR volume for a conversion (first order) | [![Combustion with excess air: air requirement and flue-gas composition: a chart in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/chart-2.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_combustion_flue_gas)<br>Combustion with excess air: air requirement and flue-gas composition | [![Bubble-point temperature (Raoult's law): a chart in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/chart-3.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_raoult_bubble_temperature)<br>Bubble-point temperature (Raoult's law) | [![Steady radial flow of oil to a well (Darcy) and productivity index: a chart in the Chemical Engineering desktop app](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/chart-4.webp)](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_radial_darcy_flow)<br>Steady radial flow of oil to a well (Darcy) and productivity index |

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

## Command line: 15 examples

Each command below was run with `chemical-engineering` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ chemical-engineering solve chemeng_adiabatic_flame_temperature fuel=CH4 excess_air=0 "T_in=300 K"
T_ad = 2326.321765 K

$ chemical-engineering solve chemeng_npsh_available "p_s=101325 Pa" "p_v=2339 Pa" "rho=998 kg/m^3" "z_s=2 m" "h_f=0.5 m"
NPSHa = 11.61399089 m
pressure_head = 10.35298049 m
vapour_head = 0.2389896014 m

$ chemical-engineering solve chemeng_blending_flows "W=[[0.9, 0.1], [0.2, 0.8]]" "p=[0.5, 0.5]" "P=100 kg/s"
F1 = 42.85714286 kg/s
F = [42.8571428571,57.1428571429]

$ chemical-engineering solve chemeng_exponential_decline "qi=0.001840130728 m^3/s" "Di=9.506426344e-09 1/s" "t=63115200 s"
q = 0.001009885155 m^3/s
Np = 87335.19227 m^3

$ chemical-engineering solve chemeng_closed_loop_stability "num=[1]" "den=[1, 3, 3, 1]" Kc=10
max_real = 0.07721734502
n_rhp = 2
stable = unstable

$ chemical-engineering solve chemeng_equipment_cost_scaling C1=100000 S1=10 S2=20 n=0.6 I1=500 I2=800
capacity_ratio = 2
scale_factor = 1.515716567
C_scaled = 151571.6567
index_ratio = 1.6
C2 = 242514.6506

$ chemical-engineering solve chemeng_virial_compressibility "T=510 K" "P=2500000 Pa" "Tc=425.1 K" "Pc=3796000 Pa" omega=0.2
B = -0.000205361259 m^3/mol
Z = 0.878925087

$ chemical-engineering solve chemeng_batch_conversion_first_order "k=0.001 1/s" "t=1000 s"
X = 0.6321205588

$ chemical-engineering solve chemeng_gas_pz_material_balance "p=[3200, 2925, 2525, 2125]" "z=[0.85, 0.84, 0.83, 0.82]" "Gp=[0, 15, 35, 55]" p_unit=psi gp_unit=Bscf
G = 4993507777 m^3
pz_initial = 26084853.14 Pa

$ chemical-engineering solve chemeng_kremser_absorber N=5 A=1.4 m=2 y_in=0.05 x_in=0
y_out = 0.003063004783
recovery = 0.9387399043

$ chemical-engineering solve chemeng_cake_filtration_volume "A=1 m^2" "dP=100000 Pa" "mu=0.001 Pa*s" "alpha=1e+11 m/kg" "c=20 kg/m^3" "Rm=1e+10 1/m" "t=600 s"
V = 0.24 m^3

$ chemical-engineering solve chemeng_adiabatic_reactor_temperature "n0=[1, 2, 0]" "nu=[-1, -1, 1]" "cp=[[30, 0.01], [29, 0.004], [45, 0.02]]" "dH=-60000 J/mol" "T_ref=298.15 K" "T_in=400 K" key=1 X=0.8
T_out = 932.3482853 K

$ chemical-engineering solve chemeng_pump_head "p1=100000 Pa" "p2=400000 Pa" "rho=1000 kg/m^3"
H = 30.59148639 m
dp_equivalent = 300000 Pa

$ chemical-engineering solve chemeng_combustion_flue_gas fuel=CH4 excess_air=0.2
O2_theoretical = 2
air = 11.42857143
n_CO2 = 1
n_H2O = 2
n_SO2 = 0
n_O2 = 0.4

$ chemical-engineering solve chemeng_gas_well_backpressure "q=[4288, 9265, 15552, 20177]" "pwf=[403.1, 394, 378.5, 362.6]" q_unit=Mscf_per_day p_unit=psi "pr=2814439.927 Pa"
n = 0.7277574483
log10_C = -8.065527458
AOF = 20.97290776 m^3/s
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Chemical Engineering on ai999x.com](https://www.ai999x.com/models/chemical-engineering-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/chemical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/chemical-engineering-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `chemical-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/chemical-engineering`) and its data (`rust/models/chemical_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p chemical-engineering-ai999x
./target/release/chemical-engineering --help
cargo test -p chemical-engineering-ai999x
```
