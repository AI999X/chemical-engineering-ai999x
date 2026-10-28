# chemical-engineering-ai999x — offline, verified chemical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemical-engineering-ai999x)](https://pypi.org/project/chemical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemical-engineering-ai999x)

**[Install](https://pypi.org/project/chemical-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/chemical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/chemical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.2

Field: **Engineering** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

chemical-engineering-ai999x is an offline, deterministic Python solver that answers chemical engineering questions with verified results from 91 certified capabilities, gives 602 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**chemical-engineering-ai999x** is a compact, deterministic chemical engineering engine for Python. It reads a question in plain English (or a typed call), picks a certified chemical engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified chemical engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 602 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** material and energy balances, reactor design, separations (distillation, absorption, extraction, flash), activity coefficients and equations of state, transport and packed beds, process control, and reservoir and petroleum engineering.

## Model overview

| Property | Value |
|---|---|
| Package | `chemical-engineering-ai999x` 1.0.2 (import `chemical_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic chemical engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 91 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 602 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install chemical-engineering-ai999x
```

```python
import chemical_engineering_ai999x as chemical_engineering

chemical_engineering.ask("First-order CSTR volume: v0 = 36 m^3/h, k = 0.06 1/min, X = 90%").show()
# -> SOLVED — V = 90, tau = 9000  (chemeng_cstr_volume_first_order)
chemical_engineering.ask("Combustion flue gas: fuel = CH4, excess_air = 20%").show()
# -> SOLVED — O2_theoretical = 2, air = 11.4286, n_CO2 = 1, n_H2O = 2  (chemeng_combustion_flue_gas)
chemical_engineering.ask("Bubble-point temperature: x = [0.4, 0.6], ant = [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], units = mmHg_degC, base = log10, P = 1 atm").show()
# -> SOLVED — T = 368.292, y = [0.62184905372, 0.37815094628]  (chemeng_raoult_bubble_temperature)
chemical_engineering.ask("Radial Darcy flow: k = 100 md, h = 30 ft, mu = 1.5 cP, pe = 3000 psi, pw = 2500 psi, re = 1000 ft, rw = 0.3 ft").show()
# -> SOLVED — q = 0.00160651, PI = 4.6601e-10  (chemeng_radial_darcy_flow)
chemical_engineering.solve("chemeng_mccabe_thiele_stages", alpha=2.5, zF=0.5, xD=0.95, q=1, xB=0.05, R=2).show()
# -> SOLVED — N = 11, Rmin = 1.1  (chemeng_mccabe_thiele_stages)
chemical_engineering.ask("Who created you?").message
# -> chemical-engineering-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `chemical-engineering-ai999x "<question>"`, `chemical-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on chemical-engineering-ai999x 1.0.2.

## Find the right capability, and draw it

`chemical_engineering.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "chemical-engineering-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import chemical_engineering_ai999x as chemical_engineering

chemical_engineering.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `chemical-engineering-ai999x`; the answer is the one it returned, with the steps it took.

**CSTR volume for a conversion (first order)** · _Reactor design_

> First-order CSTR volume: v0 = 36 m^3/h, k = 0.06 1/min, X = 90%

→ **V = 90 m^3, tau = 9000 s**

<details><summary>Steps</summary>

- CSTR volume for a conversion (first order): v0 = 36 m^3/h, k = 0.06 1/min, X = 0.9 → V = 90 m^3, tau = 9000 s

</details>

**Combustion with excess air: air requirement and flue-gas composition** · _Material balances_

> Combustion flue gas: fuel = CH4, excess_air = 20%

→ **O2_theoretical = 2, air = 11.42857143, n_CO2 = 1, n_H2O = 2, n_SO2 = 0, n_O2 = 0.4, n_N2 = 9.028571429, y_CO2_dry = 0.09589041096, y_O2_dry = 0.03835616438, y_H2O_wet = 0.1609195402, AFR_mass = 20.55283933**

<details><summary>Steps</summary>

- Combustion with excess air: air requirement and flue-gas composition: fuel = CH4, excess_air = 0.2 → O2_theoretical = 2, air = 11.42857143, n_CO2 = 1, n_H2O = 2, n_SO2 = 0, n_O2 = 0.4, n_N2 = 9.028571429, y_CO2_dry = 0.09589041096, y_O2_dry = 0.03835616438, y_H2O_wet = 0.1609195402, AFR_mass = 20.55283933

</details>

**Bubble-point temperature (Raoult's law)** · _Separations_

> Bubble-point temperature: x = [0.4, 0.6], ant = [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], units = mmHg_degC, base = log10, P = 1 atm

→ **T = 368.2916947 K, y = [0.6218490537, 0.3781509463]**

<details><summary>Steps</summary>

- Bubble-point temperature (Raoult's law): x = [0.4, 0.6], ant = [6.90565 1211.03 220.79; 6.95464 1344.8 219.482], units = mmHg_degC, base = log10, P = 1 atm → T = 368.2916947 K, y = [0.6218490537, 0.3781509463] K

</details>

**Steady radial flow of oil to a well (Darcy) and productivity index** · _Reservoir engineering_

> Radial Darcy flow: k = 100 md, h = 30 ft, mu = 1.5 cP, pe = 3000 psi, pw = 2500 psi, re = 1000 ft, rw = 0.3 ft

→ **q = 0.001606511475 m^3/s, PI = 4.660095801e-10 m^3/(s*Pa)**

<details><summary>Steps</summary>

- Steady radial flow of oil to a well (Darcy) and productivity index: k = 9.869233e-14 m^2, h = 30 ft, mu = 0.0015 (Pa*s), pe = 20684271.88 Pa, pw = 17236893.23 Pa, re = 1000 ft, rw = 0.3 ft → q = 0.001606511475 m^3/s, PI = 4.660095801e-10 m^3/(s*Pa)

</details>

**McCabe-Thiele number of theoretical stages (binary)** · _Separations_

`m.solve("chemeng_mccabe_thiele_stages", alpha=2.5, zF=0.5, xD=0.95, q=1, xB=0.05, R=2)`

→ **N = 11, Rmin = 1.1**

<details><summary>Steps</summary>

- McCabe-Thiele number of theoretical stages (binary): alpha = 2.5, zF = 0.5, xD = 0.95, q = 1, xB = 0.05, R = 2 → N = 11, Rmin = 1.1

</details>

**Adiabatic flame temperature (complete combustion, NASA polynomials)** · _Energy balances_

> Adiabatic flame temperature: fuel = CH4, excess_air = 0.0, T_in = 300.0 K

→ **T_ad = 2326.321765 K**

<details><summary>Steps</summary>

- Adiabatic flame temperature (complete combustion, NASA polynomials): fuel = CH4, excess_air = 0, T_in = 300 K → T_ad = 2326.321765 K

</details>

## Figures

Every point was computed by `chemical-engineering-ai999x`.

**Boiling-point diagram: bubble and dew curves of a binary mixture**

![Boiling-point diagram: bubble and dew curves of a binary mixture — computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-raoult-bubble-temperature-phase.svg)

_Boiling-point diagram of an ideal binary mixture at P = 101325 Pa (Raoult's law with the Antoine constants [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], mmHg and °C): bubble and dew temperatures computed by the package at 40 compositions each; between them (shaded) liquid and vapour coexist. The tie line joins a liquid with x₁ = 0.4 to its vapour y₁ = 0.621849; marked: x₁ = 0.4, T = 368.292 K._

**Reactor conversion: CSTR, CSTRs in series, PFR**

![Reactor conversion: CSTR, CSTRs in series, PFR — computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-cstr-conversion-first-order-reactor.svg)

_Conversion of a first-order reaction (k = 0.001 1/s, feed 0.01 m³/s) against total volume for one CSTR, three equal CSTRs in series and a plug-flow reactor, each computed by the package at 60 volumes. At V = 20 m³ the PFR reaches X = 0.864665; marked: V = 20 m³, X = 0.666667 (CSTR)._

**Production decline: exponential, hyperbolic, harmonic**

![Production decline: exponential, hyperbolic, harmonic — computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-hyperbolic-decline-decline.svg)

_Well production rate against time for three decline laws from the same initial rate qi = 158.987 m³/day, each computed by the package at 81 times over 10 years. After 10 years: exponential (Di = 0.3 per year) 7.91551 m³/day; hyperbolic, b = 0.5 (Di = 0.5 per year) 12.9786 m³/day; harmonic (Di = 0.4 per year) 31.7975 m³/day._

**Flash separation: feed, liquid and vapour compositions**

![Flash separation: feed, liquid and vapour compositions — computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-rachford-rice-flash-bars.svg)

_Isothermal flash of a 3-component feed z = [0.5, 0.3, 0.2] with equilibrium ratios K = [1.685, 0.742, 0.532]: the package solves the Rachford–Rice equation for a vapour fraction of 0.69073 and returns the liquid x = [0.339409, 0.365056, 0.295535] and vapour y = [0.571904, 0.270872, 0.157225] compositions (bars)._

**Conversion table: tanks in series against total volume**

![Conversion table: tanks in series against total volume — computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-cstrs-in-series-first-order-heatmap.svg)

_Conversion of a first-order reaction (k = 0.001 1/s, feed 0.01 m³/s) in N equal CSTRs in series (rows, N = 1 to 5) sharing a total volume of 5, 10, 20, 40, 80 m³ (columns), each cell computed by the package. More tanks approach plug flow: a PFR of the same volumes gives 0.393469, 0.632121, 0.864665, 0.981684, 0.999665._

## Capabilities

91 certified procedures. A few of them:

- **Procedures:** Absorber with N equilibrium stages (Kremser); Activity coefficients from the NRTL equation; Activity coefficients from the UNIQUAC equation; Activity coefficients from the Wilson equation; Adiabatic flame temperature (complete combustion, NASA polynomials); Adiabatic reactor outlet temperature; Batch cooling (or heating) with a non-isothermal coolant in a coil; Batch heating of a stirred tank: temperature after a time; Batch heating of a stirred tank: time to reach a temperature; Batch reactor conversion after a time (first order); Batch reactor conversion after a time (second order); Batch reactor time for a conversion (first order); Batch reactor time for a conversion (second order); Blending: feed flows that make a product of given composition; Boiling (saturation) temperature from the Antoine equation; Bubble-point pressure (Raoult's law); Bubble-point pressure by the modified Raoult's law (activity coefficients); Bubble-point temperature (Raoult's law); CSTR conversion for a volume (first order); CSTR conversion for a volume (second order); CSTR volume for a conversion (first order); CSTR volume for a conversion (second order); Closed-loop stability of a P-controlled process (closed-loop poles); Combustion with excess air: air requirement and flue-gas composition …

`chemical_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified chemical engineering capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (material and energy balances, reactor design, separations (distillation, absorption, extraction, flash), activity coefficients and equations of state, transport and packed beds, process control, and reservoir and petroleum engineering); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{chemical_engineering_ai999x_1_0_2,
  title  = {chemical-engineering-ai999x: an offline, verified chemical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.2},
  url    = {https://huggingface.co/AI999X/chemical-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/chemical-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

chemical-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/chemical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/chemical-engineering-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/chemical-engineering-ai999x/tree/main/onnx).
