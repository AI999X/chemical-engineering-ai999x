# chemical-engineering-ai999x — offline, verified chemical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemical-engineering-ai999x)](https://pypi.org/project/chemical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemical-engineering-ai999x)

**[Install](https://pypi.org/project/chemical-engineering-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/chemical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/chemical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

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
| Package | `chemical-engineering-ai999x` 1.0.0 (import `chemical_engineering_ai999x`) |
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

Command line: `chemical-engineering-ai999x "<question>"`, `chemical-engineering-ai999x --capabilities`. The outputs above were produced by running this exact code on chemical-engineering-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

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
@software{chemical_engineering_ai999x_1_0_0,
  title  = {chemical-engineering-ai999x: an offline, verified chemical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
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
