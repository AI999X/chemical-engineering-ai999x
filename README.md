# chemical-engineering-ai999x: offline, verified chemical engineering solver with step-by-step answers

[![tests](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemical-engineering-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemical-engineering-ai999x)](https://pypi.org/project/chemical-engineering-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemical-engineering-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/chemical-engineering-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/chemical-engineering-ai999x/)** · **[Guides](https://huggingface.co/AI999X/chemical-engineering-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/chemical-engineering-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Engineering** · Tasks: Question Answering

chemical-engineering-ai999x is an offline, deterministic chemical engineering solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers chemical engineering questions with verified results from 92 certified capabilities, gives 611 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**chemical-engineering-ai999x** reads a question in plain English (or a typed call), picks a certified chemical engineering capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** chemical-engineering-ai999x comes as the AI999X desktop app and as the command-line program `chemical-engineering` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Chemical Engineering desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-chemical-engineering-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-chemical-engineering-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`chemical-engineering`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/chemical-engineering-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install chemical-engineering-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Chemical Engineering** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Chemical Engineering among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-chemical-engineering-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-chemical-engineering-1.5.0-linux-x86_64.tar.gz
./ai999x-chemical-engineering-1.5.0-linux-x86_64/ai999x-chemical-engineering
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
chemical-engineering --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
chemical-engineering --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ chemical-engineering ask "First-order CSTR volume: v0 = 36 m^3/h, k = 0.06 1/min, X = 90%"
Read as: CSTR volume for a conversion (first order)
V = 90 m^3
tau = 9000 s
  by: CSTR volume for a conversion (first order)  (design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X)))
  X = 0.9
  k = 0.001 1/s
  v0 = 0.01 m^3/s
  assumes: 0 <= X < 1; steady state, perfect mixing, constant density, -r_A = k C_A
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ chemical-engineering info chemeng_cstr_volume_first_order
CSTR volume for a conversion (first order)
  design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X))
  in   v0       volumetric feed flow [m^3/s]
  in   k        first-order rate constant [1/s]
  in   X        conversion of A wanted []
  out  V        reactor volume [m^3]
  out  tau      space time V/v0 [s]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ chemical-engineering solve chemeng_cstr_volume_first_order "v0=36 m^3/h" "k=0.06 1/min" X=0.9
V = 90 m^3
tau = 9000 s
  by: CSTR volume for a conversion (first order)  (design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X)))
  v0 = 0.01 m^3/s
  k = 0.001 1/s
  X = 0.9
  assumes: 0 <= X < 1; steady state, perfect mixing, constant density, -r_A = k C_A
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | chemical-engineering batch

`cases.jsonl`:

```json
{"key": "chemeng_cstr_volume_first_order", "inputs": {"v0": "36 m^3/h", "k": "0.06 1/min", "X": 0.9}}
{"key": "chemeng_combustion_flue_gas", "inputs": {"fuel": "CH4", "excess_air": 0.2}}
```

```console
$ chemical-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":90.0,"unit":"m^3","solved_for":"V","si":[["v0",0.01],["k",0.001],["X",0.9]],"values":[["V",90.0,"m^3"],["tau",9000.0,"s"]],"target":"V","phrase":"reactor volume","assumptions":["0 <= X < 1","steady state, perfect mixing, constant density, -r_A = k C_A"],"name":"CSTR volume for a conversion (first order)","equation":"design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X))"}
{"outcome":"SOLVED","message":"","value":0.0958904109589,"unit":"","solved_for":"y_CO2_dry","si":[["excess_air",0.2]],"values":[["O2_theoretical",2.0,""],["air",11.4285714286,""],["n_CO2",1.0,""],["n_H2O",2.0,""],["n_SO2",0.0,""],["n_O2",0.4,""],["n_N2",9.02857142857,""],["y_CO2_dry",0.0958904109589,""],["y_O2_dry",0.0383561643836,""],["y_H2O_wet",0.16091954023,""],["AFR_mass",20.5528393312,""]],"target":"y_CO2_dry","phrase":"theoretical O2, mol per mol fuel","assumptions":["complete combustion to CO2, H2O and SO2; fuel N leaves as N2","dry air 21 % O2 / 79 % N2","0 <= excess air <= 500 %"],"name":"Combustion with excess air: air requirement and flue-gas composition","equation":"O2 theoretical = C + H/4 + S - O/2 per mol fuel; air = (1 + e) O2/0.21; products by element balance"}
```

### In your browser

Open [Chemical Engineering on ai999x.com](https://www.ai999x.com/models/chemical-engineering-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `chemical_engineering_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install chemical-engineering-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Adiabatic flame temperature (complete combustion, NASA polynomials)

Adiabatic flame temperature of a gaseous fuel burned with excess dry air at constant pressure, complete combustion without dissociation, from NASA 7-coefficient polynomials (GRI-Mech 3.0 data).

Command line:

```console
$ chemical-engineering solve chemeng_adiabatic_flame_temperature fuel=CH4 excess_air=0 "T_in=300 K"
T_ad = 2326.321765 K
  by: Adiabatic flame temperature (complete combustion, NASA polynomials)  (energy balance H_products(T_ad) = H_reactants(T_in) with NASA polynomial enthalpies (including formation enthalpies), solved by bisection)
  excess_air = 0
  T_in = 300 K
  fuel = CH4
  assumes: complete combustion to CO2 and H2O vapour, no dissociation; 300 K <= T_in <= 1000 K; result <= 3500 K
```

Python:

```python
import chemical_engineering_ai999x as m

a = m.solve("chemeng_adiabatic_flame_temperature", fuel="CH4", excess_air="0", T_in="300 K")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_adiabatic_flame_temperature) and press **Compute in your browser**.

_What this means:_ This calculation determines the theoretical maximum temperature reached during complete combustion at constant pressure, assuming no heat loss. It uses energy balance principles with NASA polynomial enthalpies to equate the total enthalpy of reactants entering at 300 K with the products exiting. The resulting adiabatic flame temperature is 2326.32 K, representing the thermal limit under these idealized conditions where excess air is set to 0.

### Adiabatic reactor outlet temperature

Outlet temperature of an adiabatic reactor from the feed, the conversion of the limiting reactant, the heat of reaction at a reference temperature and polynomial heat capacities.

Command line:

```console
$ chemical-engineering solve chemeng_adiabatic_reactor_temperature "n0=[1, 2, 0]" "nu=[-1, -1, 1]" "cp=[[30, 0.01], [29, 0.004], [45, 0.02]]" "dH=-60000 J/mol" "T_ref=298.15 K" "T_in=400 K" key=1 X=0.8
T_out = 932.3482853 K
  by: Adiabatic reactor outlet temperature  (energy balance along the path: cool the feed to T_ref, react at T_ref, heat the products to T_out; T_out by bisection)
  dH = -60000 J/mol
  T_ref = 298.15 K
  T_in = 400 K
  X = 0.8
  assumes: adiabatic, no shaft work, no phase change; outlet between 200 and 3200 K
```

Python:

```python
import chemical_engineering_ai999x as m

a = m.solve("chemeng_adiabatic_reactor_temperature", n0=[1, 2, 0], nu=[-1, -1, 1], cp=[[30, 0.01], [29, 0.004], [45, 0.02]], dH="-60000 J/mol", T_ref="298.15 K", T_in="400 K", key="1", X="0.8")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_adiabatic_reactor_temperature) and press **Compute in your browser**.

_What this means:_ This calculation determines the final temperature of an adiabatic reactor, which is 932.348 K. It uses a three-step energy balance: cooling the feed from 400 K to the reference temperature 298.15 K, reacting at that reference with a heat of reaction of -60000 J/mol, and then heating the products back up. The result reflects how the conversion of the limiting reactant, currently 0.8, shifts the thermal state without any external heat exchange.

### Batch cooling (or heating) with a non-isothermal coolant in a coil

Temperature of a well-stirred batch after time t when a coolant (or heating liquid) flows once through a coil and leaves warmer (Kern's batch cooling).

Command line:

```console
$ chemical-engineering solve chemeng_batch_coil_cooling "M=5000 kg" "cp=2500 J/(kg*K)" "W=2 kg/s" "cp_c=4180 J/(kg*K)" "t_in=288.15 K" "UA=3000 W/K" "T0=393.15 K" "t=7200 s"
T = 312.7322125 K
  by: Batch cooling (or heating) with a non-isothermal coolant in a coil  (coolant outlet approaches the batch temperature: T - t_out = (T - t_in) exp(-UA/(W cp_c)); tank balance M cp dT/dt = -W cp_c (t_out - t_in), integrated)
  M = 5000 kg
  cp = 2500 J/(kg*K)
  W = 2 kg/s
  cp_c = 4180 J/(kg*K)
  t_in = 288.15 K
  UA = 3000 W/K
  T0 = 393.15 K
  t = 7200 s
  assumes: well-stirred batch; constant UA, cp and coolant flow
```

Python:

```python
import chemical_engineering_ai999x as m

a = m.solve("chemeng_batch_coil_cooling", M="5000 kg", cp="2500 J/(kg*K)", W="2 kg/s", cp_c="4180 J/(kg*K)", t_in="288.15 K", UA="3000 W/K", T0="393.15 K", t="7200 s")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_batch_coil_cooling) and press **Compute in your browser**.

_What this means:_ This calculation finds the final temperature of a well-stirred batch liquid after a specific time, accounting for a coolant that changes temperature as it flows through a coil. Using the initial batch temperature of 393.15 K, the coolant inlet temperature of 288.15 K, and system parameters like mass and heat transfer capacity, the model determines that the batch reaches 312.732 K at the end of the process.

### Batch heating of a stirred tank: temperature after a time

Temperature of a well-stirred batch heated or cooled by a medium at constant temperature (condensing steam, boiling refrigerant) after time t.

Command line:

```console
$ chemical-engineering solve chemeng_batch_heating_temperature "m=1000 kg" "cp=4180 J/(kg*K)" "UA=2000 W/K" "T_s=393.15 K" "T0=293.15 K" "t=3600 s"
T = 375.2878822 K
  by: Batch heating of a stirred tank: temperature after a time  (energy balance m cp dT/dt = UA (T_s - T), integrated)
  m = 1000 kg
  cp = 4180 J/(kg*K)
  UA = 2000 W/K
  T_s = 393.15 K
  T0 = 293.15 K
  t = 3600 s
  assumes: well stirred; constant UA and cp; medium at constant temperature
```

Python:

```python
import chemical_engineering_ai999x as m

a = m.solve("chemeng_batch_heating_temperature", m="1000 kg", cp="4180 J/(kg*K)", UA="2000 W/K", T_s="393.15 K", T0="293.15 K", t="3600 s")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/chemical-engineering-ai999x/#cap-chemeng_batch_heating_temperature) and press **Compute in your browser**.

_What this means:_ This calculation determines the liquid temperature in a well-stirred tank after a specified time. Using an energy balance, it accounts for the heat transfer rate driven by the difference between the medium temperature and the liquid's initial state. The result shows that with an initial temperature of 293.15 K, heated by a medium at 393.15 K for 3600 s, the liquid reaches 375.288 K. This assumes constant physical properties and perfect mixing.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/chemical-engineering-ai999x/blob/main/docs/chemical_engineering_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/chemical-engineering-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified chemical engineering capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 611 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `chemical-engineering` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** material and energy balances, reactor design, separations (distillation, absorption, extraction, flash), activity coefficients and equations of state, transport and packed beds, process control, and reservoir and petroleum engineering.

## Model overview

| Property | Value |
|---|---|
| Package | `chemical-engineering-ai999x` 1.6.0 (import `chemical_engineering_ai999x`) |
| Type | Deterministic neuro-symbolic chemical engineering engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 92 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 611 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `chemical-engineering-ai999x "<question>"`; for the native program use `chemical-engineering` (above). The outputs above were produced by running this exact code on chemical-engineering-ai999x 1.6.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

![Boiling-point diagram: bubble and dew curves of a binary mixture, computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-raoult-bubble-temperature-phase.svg)

_Boiling-point diagram of an ideal binary mixture at P = 101325 Pa (Raoult's law with the Antoine constants [[6.90565, 1211.033, 220.79], [6.95464, 1344.8, 219.482]], mmHg and °C): bubble and dew temperatures computed by the package at 40 compositions each; between them (shaded) liquid and vapour coexist. The tie line joins a liquid with x₁ = 0.4 to its vapour y₁ = 0.621849; marked: x₁ = 0.4, T = 368.292 K._

**Reactor conversion: CSTR, CSTRs in series, PFR**

![Reactor conversion: CSTR, CSTRs in series, PFR, computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-cstr-conversion-first-order-reactor.svg)

_Conversion of a first-order reaction (k = 0.001 1/s, feed 0.01 m³/s) against total volume for one CSTR, three equal CSTRs in series and a plug-flow reactor, each computed by the package at 60 volumes. At V = 20 m³ the PFR reaches X = 0.864665; marked: V = 20 m³, X = 0.666667 (CSTR)._

**Production decline: exponential, hyperbolic, harmonic**

![Production decline: exponential, hyperbolic, harmonic, computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-hyperbolic-decline-decline.svg)

_Well production rate against time for three decline laws from the same initial rate qi = 158.987 m³/day, each computed by the package at 81 times over 10 years. After 10 years: exponential (Di = 0.3 per year) 7.91551 m³/day; hyperbolic, b = 0.5 (Di = 0.5 per year) 12.9786 m³/day; harmonic (Di = 0.4 per year) 31.7975 m³/day._

**Flash separation: feed, liquid and vapour compositions**

![Flash separation: feed, liquid and vapour compositions, computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-rachford-rice-flash-bars.svg)

_Isothermal flash of a 3-component feed z = [0.5, 0.3, 0.2] with equilibrium ratios K = [1.685, 0.742, 0.532]: the package solves the Rachford–Rice equation for a vapour fraction of 0.69073 and returns the liquid x = [0.339409, 0.365056, 0.295535] and vapour y = [0.571904, 0.270872, 0.157225] compositions (bars)._

**Conversion table: tanks in series against total volume**

![Conversion table: tanks in series against total volume, computed by chemical-engineering-ai999x](https://huggingface.co/AI999X/chemical-engineering-ai999x/resolve/main/figures/chemeng-cstrs-in-series-first-order-heatmap.svg)

_Conversion of a first-order reaction (k = 0.001 1/s, feed 0.01 m³/s) in N equal CSTRs in series (rows, N = 1 to 5) sharing a total volume of 5, 10, 20, 40, 80 m³ (columns), each cell computed by the package. More tanks approach plug flow: a PFR of the same volumes gives 0.393469, 0.632121, 0.864665, 0.981684, 0.999665._

## Capabilities

92 certified procedures. A few of them:

- **Procedures:** Absorber with N equilibrium stages (Kremser); Activity coefficients from the NRTL equation; Activity coefficients from the UNIQUAC equation; Activity coefficients from the Wilson equation; Adiabatic flame temperature (complete combustion, NASA polynomials); Adiabatic reactor outlet temperature; Batch cooling (or heating) with a non-isothermal coolant in a coil; Batch heating of a stirred tank: temperature after a time; Batch heating of a stirred tank: time to reach a temperature; Batch reactor conversion after a time (first order); Batch reactor conversion after a time (second order); Batch reactor time for a conversion (first order); Batch reactor time for a conversion (second order); Blending: feed flows that make a product of given composition; Boiling (saturation) temperature from the Antoine equation; Bubble-point pressure (Raoult's law); Bubble-point pressure by the modified Raoult's law (activity coefficients); Bubble-point temperature (Raoult's law); CSTR conversion for a volume (first order); CSTR conversion for a volume (second order); CSTR volume for a conversion (first order); CSTR volume for a conversion (second order); Closed-loop stability of a P-controlled process (closed-loop poles); Combustion with excess air: air requirement and flue-gas composition …

`chemical_engineering_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified chemical engineering capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (material and energy balances, reactor design, separations (distillation, absorption, extraction, flash), activity coefficients and equations of state, transport and packed beds, process control, and reservoir and petroleum engineering); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{chemical_engineering_ai999x_1_6_0,
  title  = {chemical-engineering-ai999x: an offline, verified chemical engineering solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/chemical-engineering-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/chemical-engineering-ai999x/discussions or visit https://ai999x.com.

## Support the project

chemical-engineering-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/chemical-engineering-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/chemical-engineering-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `chemical-engineering` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/chemical-engineering`) and its data (`rust/models/chemical_engineering/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p chemical-engineering-ai999x
./target/release/chemical-engineering --help
cargo test -p chemical-engineering-ai999x
```
