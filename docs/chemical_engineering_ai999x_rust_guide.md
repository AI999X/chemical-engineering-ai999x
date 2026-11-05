# chemical-engineering: the Chemical Engineering program, user guide

**chemical-engineering-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Chemical Engineering covers material and energy balances, reactor design, separations (distillation, absorption, extraction), transport phenomena, process control and reservoir and petroleum basics. This guide is about `chemical-engineering`, the model as a self-contained program for Windows and Linux: 91 capabilities in 10 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/chemical-engineering-ai999x/); the Python package (`pip install chemical-engineering-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Chemical Engineering | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-chemical-engineering-1.5.0-windows-x86_64.zip |
| Desktop app: Chemical Engineering | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-chemical-engineering-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Chemical Engineering in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-chemical-engineering-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-chemical-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-chemical-engineering-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-chemical-engineering-1.5.0-linux-x86_64.tar.gz
./ai999x-chemical-engineering-1.5.0-linux-x86_64/ai999x-chemical-engineering
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `chemical-engineering` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
chemical-engineering --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

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

### In your browser

Open https://www.ai999x.com/models/chemical-engineering-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install chemical-engineering-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ chemical-engineering --version
chemical-engineering-ai999x 1.5.0 (Rust build, ax-engine 1.5.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ chemical-engineering search "cstr volume for a conversion (first order)"
chemeng_cstr_volume_first_order      CSTR volume for a conversion (first order)  (reactor design / continuous stirred tanks)
chemeng_cstr_conversion_first_order  CSTR conversion for a volume (first order)  (reactor design / continuous stirred tanks)
chemeng_cstrs_in_series_first_order  Equal CSTRs in series: conversion (first order)  (reactor design / continuous stirred tanks)
chemeng_cstr_conversion_second_order CSTR conversion for a volume (second order)  (reactor design / continuous stirred tanks)
chemeng_cstr_volume_second_order     CSTR volume for a conversion (second order)  (reactor design / continuous stirred tanks)
chemeng_pfr_conversion_first_order   PFR conversion for a volume (first order, liquid)  (reactor design / plug-flow reactors)
chemeng_pfr_volume_first_order       PFR volume for a conversion (first order, liquid)  (reactor design / plug-flow reactors)
chemeng_cstrs_in_series_second_order Equal CSTRs in series: conversion (second order)  (reactor design / continuous stirred tanks)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ chemical-engineering list
chemeng_absorber_transfer_units          Packed absorber: number of transfer units and height (Colburn)
chemeng_adiabatic_flame_temperature      Adiabatic flame temperature (complete combustion, NASA polynomials)
chemeng_adiabatic_reactor_temperature    Adiabatic reactor outlet temperature
chemeng_antoine_boiling_temperature      Boiling (saturation) temperature from the Antoine equation
chemeng_antoine_vapour_pressure          Vapour pressure from the Antoine equation
chemeng_batch_coil_cooling               Batch cooling (or heating) with a non-isothermal coolant in a coil
chemeng_batch_conversion_first_order     Batch reactor conversion after a time (first order)
chemeng_batch_conversion_second_order    Batch reactor conversion after a time (second order)
… 83 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ chemical-engineering info chemeng_cstr_volume_first_order
CSTR volume for a conversion (first order)
  design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X))
  in   v0       volumetric feed flow [m^3/s]
  in   k        first-order rate constant [1/s]
  in   X        conversion of A wanted []
  out  V        reactor volume [m^3]
  out  tau      space time V/v0 [s]
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ chemical-engineering solve chemeng_cstr_volume_first_order "v0=36 m^3/h" "k=0.06 1/min" X=0.9
V = 90 m^3
tau = 9000 s
  by: CSTR volume for a conversion (first order)  (design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X)))
  v0 = 0.01 m^3/s
  k = 0.001 1/s
  X = 0.9
  assumes: 0 <= X < 1; steady state, perfect mixing, constant density, -r_A = k C_A
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ chemical-engineering solve chemeng_cstr_volume_first_order "v0=36 m^3/h" "k=0.06 1/min" X=0.9 --json
{"outcome":"SOLVED","message":"","value":90.0,"unit":"m^3","solved_for":"V","si":[["v0",0.01],["k",0.001],["X",0.9]],"values":[["V",90.0,"m^3"],["tau",9000.0,"s"]],"target":"V","phrase":"reactor volume","assumptions":["0 <= X < 1","steady state, perfect mixing, constant density, -r_A = k C_A"],"name":"CSTR volume for a conversion (first order)","equation":"design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X))"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ chemical-engineering solve chemeng_cstr_volume_first_order "v0=36 m^3/h" "k=0.06 1/min"
NEEDS_INPUT: chemeng_cstr_volume_first_order needs X (conversion of A wanted). Example: solve('chemeng_cstr_volume_first_order', v0='0.01 m^3/s', k='0.001 1/s', X=0.9).
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | chemical-engineering batch

`cases.jsonl`:

```json
{"key": "chemeng_cstr_volume_first_order", "inputs": {"v0": "36 m^3/h", "k": "0.06 1/min", "X": 0.9}}
{"key": "chemeng_combustion_flue_gas", "inputs": {"fuel": "CH4", "excess_air": 0.2}}
```

```shell
$ chemical-engineering batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":90.0,"unit":"m^3","solved_for":"V","si":[["v0",0.01],["k",0.001],["X",0.9]],"values":[["V",90.0,"m^3"],["tau",9000.0,"s"]],"target":"V","phrase":"reactor volume","assumptions":["0 <= X < 1","steady state, perfect mixing, constant density, -r_A = k C_A"],"name":"CSTR volume for a conversion (first order)","equation":"design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X))"}
{"outcome":"SOLVED","message":"","value":0.0958904109589,"unit":"","solved_for":"y_CO2_dry","si":[["excess_air",0.2]],"values":[["O2_theoretical",2.0,""],["air",11.4285714286,""],["n_CO2",1.0,""],["n_H2O",2.0,""],["n_SO2",0.0,""],["n_O2",0.4,""],["n_N2",9.02857142857,""],["y_CO2_dry",0.0958904109589,""],["y_O2_dry",0.0383561643836,""],["y_H2O_wet",0.16091954023,""],["AFR_mass",20.5528393312,""]],"target":"y_CO2_dry","phrase":"theoretical O2, mol per mol fuel","assumptions":["complete combustion to CO2, H2O and SO2; fuel N leaves as N2","dry air 21 % O2 / 79 % N2","0 <= excess air <= 500 %"],"name":"Combustion with excess air: air requirement and flue-gas composition","equation":"O2 theoretical = C + H/4 + S - O/2 per mol fuel; air = (1 + e) O2/0.21; products by element balance"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `chemical-engineering info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`chemical-engineering solve ... --json` prints one answer as one line of JSON. `chemical-engineering batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | chemical-engineering batch`. `chemical-engineering catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["chemical-engineering", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`chemical-engineering figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `chemical_engineering_ai999x.wasm` (https://www.ai999x.com/compute/rust/chemical_engineering_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("chemical_engineering_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `chemical-engineering info <id>` shows the same inputs and outputs in the terminal.

### Energy balances (7)

<a id="chemeng_adiabatic_flame_temperature"></a>

#### Adiabatic flame temperature (complete combustion, NASA polynomials), `chemeng_adiabatic_flame_temperature`

Adiabatic flame temperature of a gaseous fuel burned with excess dry air at constant pressure, complete combustion without dissociation, from NASA 7-coefficient polynomials (GRI-Mech 3.0 data).

$$
\sum_{p} n_p\,h_p(T_{ad}) = \sum_{r} n_r\,h_r(T_{in}),\quad \frac{h}{RT} = a_1 + \frac{a_2T}{2} + \frac{a_3T^2}{3} + \frac{a_4T^3}{4} + \frac{a_5T^4}{5} + \frac{a_6}{T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `fuel` | in | fuel: CH4, C2H6, C3H8, H2, CO, C2H4, C2H2, CH3OH | - |
| `excess_air` | in | excess air as a fraction of theoretical (0 = stoichiometric) | - |
| `T_in` | in | temperature of the fuel and air entering | K |
| `T_ad` | out | adiabatic flame temperature | K |

**Applies when:** complete combustion to CO2 and H2O vapour, no dissociation; 300 K <= T_in <= 1000 K; result <= 3500 K.

```shell
$ chemical-engineering solve chemeng_adiabatic_flame_temperature fuel=CH4 excess_air=0 "T_in=300 K"
T_ad = 2326.321765 K
  by: Adiabatic flame temperature (complete combustion, NASA polynomials)  (energy balance H_products(T_ad) = H_reactants(T_in) with NASA polynomial enthalpies (including formation enthalpies), solved by bisection)
  excess_air = 0
  T_in = 300 K
  fuel = CH4
  assumes: complete combustion to CO2 and H2O vapour, no dissociation; 300 K <= T_in <= 1000 K; result <= 3500 K
```

**What the worked example means:** This calculation determines the theoretical maximum temperature reached during complete combustion at constant pressure, assuming no heat loss. It uses energy balance principles with NASA polynomial enthalpies to equate the total enthalpy of reactants entering at 300 K with the products exiting. The resulting adiabatic flame temperature is 2326.32 K, representing the thermal limit under these idealized conditions where excess air is set to 0.

<a id="chemeng_adiabatic_reactor_temperature"></a>

#### Adiabatic reactor outlet temperature: `chemeng_adiabatic_reactor_temperature`

Outlet temperature of an adiabatic reactor from the feed, the conversion of the limiting reactant, the heat of reaction at a reference temperature and polynomial heat capacities.

$$
\sum_i n_{i0}\!\int_{T_{in}}^{T_{ref}}\!c_{p,i}dT + \xi\,\Delta H_r(T_{ref}) + \sum_i n_i\!\int_{T_{ref}}^{T_{out}}\!c_{p,i}dT = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n0` | in | feed amounts of each species, mol (or mol/s) on one basis | - |
| `nu` | in | stoichiometric coefficients, negative for reactants | - |
| `cp` | in | heat-capacity coefficients, one row (a, b, c, d) per species; cp = a + bT + cT^2 + dT^3 in J/(mol K) | - |
| `dH` | in | heat of reaction at T_ref, per mole of extent | J/mol |
| `T_ref` | in | reference temperature of dH | K |
| `T_in` | in | feed temperature | K |
| `key` | in | number of the limiting reactant in the species list (1 = first) | - |
| `X` | in | conversion of the limiting reactant | - |
| `T_out` | out | adiabatic outlet temperature | K |

**Applies when:** adiabatic, no shaft work, no phase change; outlet between 200 and 3200 K.

```shell
$ chemical-engineering solve chemeng_adiabatic_reactor_temperature "n0=[1, 2, 0]" "nu=[-1, -1, 1]" "cp=[[30, 0.01], [29, 0.004], [45, 0.02]]" "dH=-60000 J/mol" "T_ref=298.15 K" "T_in=400 K" key=1 X=0.8
T_out = 932.3482853 K
  by: Adiabatic reactor outlet temperature  (energy balance along the path: cool the feed to T_ref, react at T_ref, heat the products to T_out; T_out by bisection)
  dH = -60000 J/mol
  T_ref = 298.15 K
  T_in = 400 K
  X = 0.8
  assumes: adiabatic, no shaft work, no phase change; outlet between 200 and 3200 K
```

**What the worked example means:** This calculation determines the final temperature of an adiabatic reactor, which is 932.348 K. It uses a three-step energy balance: cooling the feed from 400 K to the reference temperature 298.15 K, reacting at that reference with a heat of reaction of -60000 J/mol, and then heating the products back up. The result reflects how the conversion of the limiting reactant, currently 0.8, shifts the thermal state without any external heat exchange.

<a id="chemeng_batch_coil_cooling"></a>

#### Batch cooling (or heating) with a non-isothermal coolant in a coil, `chemeng_batch_coil_cooling`

Temperature of a well-stirred batch after time t when a coolant (or heating liquid) flows once through a coil and leaves warmer (Kern's batch cooling).

$$
T = t_{in} + (T_0 - t_{in})\exp\!\left[-\frac{W c_{p,c}\,(1 - e^{-UA/(W c_{p,c})})\,t}{M c_p}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M` | in | mass of liquid in the tank | kg |
| `cp` | in | specific heat of the batch liquid | J/(kg*K) |
| `W` | in | coolant mass flow | kg/s |
| `cp_c` | in | specific heat of the coolant | J/(kg*K) |
| `t_in` | in | coolant inlet temperature | K |
| `UA` | in | overall heat-transfer coefficient times coil area | W/K |
| `T0` | in | initial batch temperature | K |
| `t` | in | time | s |
| `T` | out | batch temperature at time t | K |

**Applies when:** well-stirred batch; constant UA, cp and coolant flow.

```shell
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

**What the worked example means:** This calculation finds the final temperature of a well-stirred batch liquid after a specific time, accounting for a coolant that changes temperature as it flows through a coil. Using the initial batch temperature of 393.15 K, the coolant inlet temperature of 288.15 K, and system parameters like mass and heat transfer capacity, the model determines that the batch reaches 312.732 K at the end of the process.

<a id="chemeng_batch_heating_temperature"></a>

#### Batch heating of a stirred tank: temperature after a time, `chemeng_batch_heating_temperature`

Temperature of a well-stirred batch heated or cooled by a medium at constant temperature (condensing steam, boiling refrigerant) after time t.

$$
T = T_s - (T_s - T_0)\,e^{-UA\,t/(m c_p)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of liquid in the tank | kg |
| `cp` | in | specific heat of the liquid | J/(kg*K) |
| `UA` | in | overall heat-transfer coefficient times area of the coil or jacket | W/K |
| `T_s` | in | temperature of the heating or cooling medium (constant, e.g. condensing steam) | K |
| `T0` | in | initial liquid temperature | K |
| `t` | in | heating time | s |
| `T` | out | liquid temperature at time t | K |

**Applies when:** well stirred; constant UA and cp; medium at constant temperature.

```shell
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

**What the worked example means:** This calculation determines the liquid temperature in a well-stirred tank after a specified time. Using an energy balance, it accounts for the heat transfer rate driven by the difference between the medium temperature and the liquid's initial state. The result shows that with an initial temperature of 293.15 K, heated by a medium at 393.15 K for 3600 s, the liquid reaches 375.288 K. This assumes constant physical properties and perfect mixing.

<a id="chemeng_batch_heating_time"></a>

#### Batch heating of a stirred tank: time to reach a temperature, `chemeng_batch_heating_time`

Time for a well-stirred batch heated or cooled by a medium at constant temperature to reach a stated temperature.

$$
t = \frac{m c_p}{UA}\ln\frac{T_s - T_0}{T_s - T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of liquid in the tank | kg |
| `cp` | in | specific heat of the liquid | J/(kg*K) |
| `UA` | in | overall heat-transfer coefficient times area of the coil or jacket | W/K |
| `T_s` | in | temperature of the heating or cooling medium (constant, e.g. condensing steam) | K |
| `T0` | in | initial liquid temperature | K |
| `T` | in | temperature to reach | K |
| `t` | out | time needed | s |

**Applies when:** T between T0 and T_s; well stirred; constant UA and cp.

```shell
$ chemical-engineering solve chemeng_batch_heating_time "m=1000 kg" "cp=4180 J/(kg*K)" "UA=2000 W/K" "T_s=393.15 K" "T0=293.15 K" "T=353.15 K"
t = 1915.04763 s
  by: Batch heating of a stirred tank: time to reach a temperature  (t = (m cp/UA) ln((T_s - T0)/(T_s - T)))
  m = 1000 kg
  cp = 4180 J/(kg*K)
  UA = 2000 W/K
  T_s = 393.15 K
  T0 = 293.15 K
  T = 353.15 K
  assumes: T between T0 and T_s; well stirred; constant UA and cp
```

**What the worked example means:** This calculation determines the duration needed for a well-stirred liquid in a tank to reach a specific temperature when heated or cooled by a constant-temperature medium. Using the mass of the liquid, its specific heat, the overall heat transfer coefficient times area, and the relevant temperatures, the model computes that the process takes 1915.05 s. This result helps operators plan batch cycles and assess equipment adequacy.

<a id="chemeng_reaction_enthalpy_at_temperature"></a>

#### Heat of reaction at another temperature (Kirchhoff), `chemeng_reaction_enthalpy_at_temperature`

The heat of reaction at T from its value at T0 and the polynomial heat capacities of the species (Kirchhoff's law).

$$
\Delta H_r(T) = \Delta H_r(T_0) + \int_{T_0}^{T}\sum_i \nu_i c_{p,i}\,dT
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `nu` | in | stoichiometric coefficients, negative for reactants and positive for products | - |
| `cp` | in | heat-capacity coefficients, one row (a, b, c, d) per species in the order of nu; cp = a + bT + cT^2 + dT^3 in J/(mol K) | - |
| `dH0` | in | heat of reaction at T0, per mole of extent | J/mol |
| `T0` | in | temperature of the known heat of reaction | K |
| `T` | in | temperature wanted | K |
| `dH` | out | heat of reaction at T, per mole of extent | J/mol |

**Applies when:** no phase change of any species between T0 and T.

```shell
$ chemical-engineering solve chemeng_reaction_enthalpy_at_temperature "nu=[-1, -3, 2]" "cp=[[27.27, 0.004927, 0, 0], [27.01, 0.003509, 0, 0], [24.62, 0.03753, -7.1e-06, 0]]" "dH0=-92220 J/mol" "T0=298.15 K" "T=700 K"
dH = -105497.1648 J/mol
  by: Heat of reaction at another temperature (Kirchhoff)  (dH(T) = dH(T0) + integral from T0 to T of sum_i nu_i cp_i dT)
  dH0 = -92220 J/mol
  T0 = 298.15 K
  T = 700 K
  assumes: no phase change of any species between T0 and T
```

**What the worked example means:** Kirchhoff's law calculates the heat of reaction at a target temperature by integrating the difference in heat capacities between products and reactants from the reference temperature. Using the known heat of reaction at 298.15 K, which is -92220 J/mol, and the stoichiometric coefficients, the model determines the thermal load at 700 K. The resulting heat of reaction is -105497 J/mol, providing an accurate estimate for reactor design under non-standard conditions.

<a id="chemeng_sensible_heat_cp_polynomial"></a>

#### Sensible heat with a polynomial heat capacity, `chemeng_sensible_heat_cp_polynomial`

Heat needed to take n moles from T1 to T2 when cp = a + bT + cT^2 + dT^3 (J/(mol K), T in K), and the mean heat capacity over the range.

$$
Q = n\int_{T_1}^{T_2}(a + bT + cT^2 + dT^3)\,dT
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | amount of substance | mol |
| `a` | in | cp coefficient a, J/(mol K) | - |
| `b` | in | cp coefficient b, J/(mol K^2) | - |
| `c` | in | cp coefficient c, J/(mol K^3) (default 0) (optional) | - |
| `d` | in | cp coefficient d, J/(mol K^4) (default 0) (optional) | - |
| `T1` | in | initial temperature | K |
| `T2` | in | final temperature | K |
| `Q` | out | heat added (negative when cooled) | J |
| `cp_mean` | out | mean heat capacity between T1 and T2 | J/(mol*K) |

**Applies when:** no phase change between T1 and T2; T in kelvin inside the polynomial.

```shell
$ chemical-engineering solve chemeng_sensible_heat_cp_polynomial "n=10 mol" a=29 b=0.002 c=0 d=0 "T1=300 K" "T2=500 K"
Q = 59600 J
cp_mean = 29.8 J/(mol*K)
  by: Sensible heat with a polynomial heat capacity  (Q = n * integral of cp dT, integrated term by term)
  n = 10 mol
  a = 29
  b = 0.002
  c = 0
  d = 0
  T1 = 300 K
  T2 = 500 K
  assumes: no phase change between T1 and T2; T in kelvin inside the polynomial
```

**What the worked example means:** This calculation determines the total heat energy required to raise or lower the temperature of a substance by integrating its polynomial heat capacity over the specified range. Using 10 mol moles heated from 300 K to 500 K, with coefficients 29, 0.002, 0, and 0, the result shows that 59600 J joules of heat are needed, corresponding to an average heat capacity of 29.8 J/(mol*K) per mole per kelvin.

### Fluid transport (2)

<a id="chemeng_npsh_available"></a>

#### Net positive suction head available: `chemeng_npsh_available`

The net positive suction head available at a pump inlet, NPSHa = (p_s - p_v)/(rho g) + z_s - h_f, from the pressure on the liquid surface, the vapour pressure, the static head and the suction-line loss; with the required NPSH, the margin.

$$
NPSH_a = \frac{p_s - p_v}{\rho g} + z_s - h_f
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p_s` | in | absolute pressure on the suction liquid surface | Pa |
| `p_v` | in | vapour pressure of the liquid at the pumping temperature | Pa |
| `rho` | in | liquid density | kg/m^3 |
| `z_s` | in | liquid surface above the pump inlet (negative for a suction lift) | m |
| `h_f` | in | friction head loss in the suction line | m |
| `NPSHr` | in | required NPSH of the pump (optional) (optional) | m |
| `NPSHa` | out | NPSH available | m |
| `pressure_head` | out | p_s/(rho g) | m |
| `vapour_head` | out | p_v/(rho g) | m |
| `margin` | out | NPSHa - NPSHr (with NPSHr) | m |

**Applies when:** absolute pressures; h_f >= 0.

```shell
$ chemical-engineering solve chemeng_npsh_available "p_s=101325 Pa" "p_v=2339 Pa" "rho=998 kg/m^3" "z_s=2 m" "h_f=0.5 m"
NPSHa = 11.61399089 m
pressure_head = 10.35298049 m
vapour_head = 0.2389896014 m
  by: Net positive suction head available  (NPSHa = (p_s - p_v)/(rho g) + z_s - h_f)
  p_s = 101325 Pa
  p_v = 2339 Pa
  rho = 998 kg/m^3
  z_s = 2 m
  h_f = 0.5 m
  assumes: absolute pressures; h_f >= 0
```

In the worked example, p_s = 101325 Pa, p_v = 2339 Pa, rho = 998 kg/m^3, z_s = 2 m, h_f = 0.5 m go through the equation above and give NPSHa = 11.61399089 m, pressure_head = 10.35298049 m, vapour_head = 0.2389896014 m. It applies only when absolute pressures; h_f >= 0: outside that it says what is wrong instead of returning a number.

<a id="chemeng_pump_head"></a>

#### Pump total head and power: `chemeng_pump_head`

The total head a pump adds between its suction and discharge, H = (p2 - p1)/(rho g) + (z2 - z1) + (v2^2 - v1^2)/(2g), with the hydraulic power rho g Q H and the shaft power for an efficiency.

$$
H = \frac{p_2 - p_1}{\rho g} + (z_2 - z_1) + \frac{v_2^2 - v_1^2}{2g},\quad P = \rho g Q H
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p1` | in | suction pressure (absolute) | Pa |
| `p2` | in | discharge pressure (absolute) | Pa |
| `rho` | in | liquid density | kg/m^3 |
| `dz` | in | elevation of the discharge gauge above the suction gauge (default 0) (optional) | m |
| `v1` | in | suction velocity (default 0) (optional) | m/s |
| `v2` | in | discharge velocity (default 0) (optional) | m/s |
| `Q` | in | volumetric flow (optional, for the power) (optional) | m^3/s |
| `eta` | in | pump efficiency (optional, with Q) (optional) | - |
| `H` | out | total head | m |
| `dp_equivalent` | out | head as a pressure rho g H | Pa |
| `P_hydraulic` | out | hydraulic power rho g Q H | W |
| `P_shaft` | out | shaft power rho g Q H/eta | W |

**Applies when:** absolute pressures; incompressible liquid; H > 0.

```shell
$ chemical-engineering solve chemeng_pump_head "p1=100000 Pa" "p2=400000 Pa" "rho=1000 kg/m^3"
H = 30.59148639 m
dp_equivalent = 300000 Pa
  by: Pump total head and power  (H = (p2 - p1)/(rho g) + dz + (v2^2 - v1^2)/(2 g); P = rho g Q H; P_shaft = P/eta)
  p1 = 100000 Pa
  p2 = 400000 Pa
  rho = 1000 kg/m^3
  assumes: absolute pressures; incompressible liquid; H > 0
```

In the worked example, p1 = 100000 Pa, p2 = 400000 Pa, rho = 1000 kg/m^3 go through the equation above and give H = 30.59148639 m, dp_equivalent = 300000 Pa. It applies only when absolute pressures; incompressible liquid; H > 0: outside that it says what is wrong instead of returning a number.

### Material balances (7)

<a id="chemeng_blending_flows"></a>

#### Blending: feed flows that make a product of given composition, `chemeng_blending_flows`

The mass flow of each feed stream needed to make a product of stated flow and composition, from the component balances (as many feeds as components).

$$
\sum_i F_i\,w_{ij} = P\,p_j \quad (j = 1..n)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | mass fractions of the feed streams: one row per feed stream, one column per component (rows sum to 1) | - |
| `p` | in | mass fractions of the product, one per component (sum to 1) | - |
| `P` | in | product mass flow | kg/s |
| `F1` | out | flow of the first feed stream | kg/s |
| `F` | out | flow of every feed stream, in kg/s, in the order of the rows of W | - |

**Applies when:** as many feed streams as components (2 to 6); each composition sums to 1; the product lies within the reach of the feeds (no negative flow).

```shell
$ chemical-engineering solve chemeng_blending_flows "W=[[0.9, 0.1], [0.2, 0.8]]" "p=[0.5, 0.5]" "P=100 kg/s"
F1 = 42.85714286 kg/s
F = [42.8571428571,57.1428571429]
  by: Blending: feed flows that make a product of given composition  (component balances sum_i F_i w_ij = P p_j for every component j, solved by Gaussian elimination with partial pivoting)
  P = 100 kg/s
  assumes: as many feed streams as components (2 to 6); each composition sums to 1; the product lies within the reach of the feeds (no negative flow)
```

**What the worked example means:** This calculation determines the mass flow rates for each input stream needed to produce a blended output. By applying component mass balances, the model solves for the unknown feed flows that sum to the target product flow of 100 kg/s. The resulting flow for the first feed stream is 42.8571 kg/s, ensuring the final mixture meets the desired mass fractions for every component without any chemical reaction or loss of material.

<a id="chemeng_combustion_flue_gas"></a>

#### Combustion with excess air: air requirement and flue-gas composition, `chemeng_combustion_flue_gas`

Theoretical oxygen, air supplied, flue-gas moles and wet and dry compositions, and the air-fuel mass ratio for complete combustion of a fuel with excess dry air (21 % O2, 79 % N2).

$$
n_{O_2,th} = x + \tfrac{y}{4} + v - \tfrac{z}{2},\quad n_{air} = \frac{(1+e)\,n_{O_2,th}}{0.21}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `fuel` | in | molecular formula of the fuel (C, H, O, N and S), e.g. CH4, C3H8, C2H5OH | - |
| `excess_air` | in | excess air as a fraction of the theoretical air (20% = 0.2) | - |
| `O2_theoretical` | out | theoretical O2, mol per mol fuel | - |
| `air` | out | air supplied, mol per mol fuel | - |
| `n_CO2` | out | CO2 in the flue gas, mol per mol fuel | - |
| `n_H2O` | out | H2O, mol per mol fuel | - |
| `n_SO2` | out | SO2, mol per mol fuel | - |
| `n_O2` | out | O2, mol per mol fuel | - |
| `n_N2` | out | N2, mol per mol fuel | - |
| `y_CO2_dry` | out | CO2 mole fraction, dry basis | - |
| `y_O2_dry` | out | O2 mole fraction, dry basis | - |
| `y_H2O_wet` | out | H2O mole fraction, wet basis | - |
| `AFR_mass` | out | air-fuel ratio, kg air per kg fuel | - |

**Applies when:** complete combustion to CO2, H2O and SO2; fuel N leaves as N2; dry air 21 % O2 / 79 % N2; 0 <= excess air <= 500 %.

```shell
$ chemical-engineering solve chemeng_combustion_flue_gas fuel=CH4 excess_air=0.2
O2_theoretical = 2
air = 11.42857143
n_CO2 = 1
n_H2O = 2
n_SO2 = 0
n_O2 = 0.4
n_N2 = 9.028571429
y_CO2_dry = 0.09589041096
y_O2_dry = 0.03835616438
y_H2O_wet = 0.1609195402
AFR_mass = 20.55283933
  by: Combustion with excess air: air requirement and flue-gas composition  (O2 theoretical = C + H/4 + S - O/2 per mol fuel; air = (1 + e) O2/0.21; products by element balance)
  excess_air = 0.2
  fuel = CH4
  assumes: complete combustion to CO2, H2O and SO2; fuel N leaves as N2; dry air 21 % O2 / 79 % N2; 0 <= excess air <= 500 %
```

**What the worked example means:** This calculation determines the combustion requirements for CH4 using 0.2 excess air. First, it computes the stoichiometric oxygen needed to fully burn the fuel's carbon, hydrogen, sulfur, and available oxygen, resulting in 2. The total air supply, including the excess portion, is then calculated as 11.4286. Based on element balances, the flue gas composition includes 1 moles of carbon dioxide, 2 moles of water vapor, 0 moles of sulfur dioxide, 0.4 moles of unreacted oxygen, and 9.02857 moles of nitrogen per mole of fuel. These quantities allow derivation of the wet and dry mole fractions, such as 0.0958904 for carbon dioxide on a dry basis, and the overall mass-based air-fuel ratio of 20.5528.

<a id="chemeng_variable_volume_tank"></a>

#### Filling or draining well-mixed tank (variable volume), `chemeng_variable_volume_tank`

Volume and concentration of a well-mixed tank whose inflow and outflow differ, at time t.

$$
C = C_{in} - (C_{in} - C_0)\left(\frac{V_0}{V}\right)^{q_{in}/(q_{in}-q_{out})},\ V = V_0 + (q_{in}-q_{out})t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V0` | in | liquid volume at t = 0 | m^3 |
| `q_in` | in | volumetric inflow | m^3/s |
| `q_out` | in | volumetric outflow | m^3/s |
| `C_in` | in | inlet concentration | mol/m^3 |
| `C0` | in | concentration in the tank at t = 0 | mol/m^3 |
| `t` | in | time | s |
| `V` | out | liquid volume at time t | m^3 |
| `C` | out | concentration at time t | mol/m^3 |

**Applies when:** V0 > 0, q_in > 0; the tank does not empty before t; perfect mixing.

```shell
$ chemical-engineering solve chemeng_variable_volume_tank "V0=1 m^3" "q_in=0.002 m^3/s" "q_out=0.001 m^3/s" "C_in=1000 mol/m^3" "C0=0 mol/m^3" "t=1000 s"
V = 2 m^3
C = 750 mol/m^3
  by: Filling or draining well-mixed tank (variable volume)  (total balance dV/dt = q_in - q_out and solute balance d(VC)/dt = q_in C_in - q_out C, integrated: C = C_in - (C_in - C0)(V0/V)^(q_in/(q_in - q_out)))
  V0 = 1 m^3
  q_in = 0.002 m^3/s
  q_out = 0.001 m^3/s
  C_in = 1000 mol/m^3
  C0 = 0 mol/m^3
  t = 1000 s
  assumes: V0 > 0, q_in > 0; the tank does not empty before t; perfect mixing
```

**What the worked example means:** This calculation determines the liquid volume and solute concentration in a well-mixed tank at a specific time, given that inflow and outflow rates differ. Starting from an initial volume of 1 m^3 and concentration of 0 mol/m^3, with an inlet concentration of 1000 mol/m^3, the system evolves over time 1000 s. The resulting volume is 2 m^3 and the concentration is 750 mol/m^3. This result helps understand how the tank contents change during non-steady-state operations like startup or shutdown.

<a id="chemeng_mass_to_mole_fractions"></a>

#### Mass fractions to mole fractions and mean molar mass, `chemeng_mass_to_mole_fractions`

Mole fractions of a mixture and its mean molar mass from the mass fractions and the molar masses of the components.

$$
z_i = \frac{w_i/M_i}{\sum_j w_j/M_j},\quad \bar M = \sum_i z_iM_i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `w` | in | mass fractions (sum to 1) | - |
| `MW` | in | molar masses of the components, g/mol | - |
| `z` | out | mole fractions | - |
| `MW_mean` | out | mean molar mass of the mixture | kg/mol |

**Applies when:** fractions sum to 1; positive molar masses.

```shell
$ chemical-engineering solve chemeng_mass_to_mole_fractions "w=[0.5, 0.5]" "MW=[32.042, 18.015]"
z = [0.359889725713,0.640110274287]
MW_mean = 0.02306317318 kg/mol
  by: Mass fractions to mole fractions and mean molar mass  (z_i = (w_i/M_i)/sum_j (w_j/M_j); M = sum z_i M_i)
  assumes: fractions sum to 1; positive molar masses
```

**What the worked example means:** This calculation converts the given mass fractions [0.5, 0.5] and component molar masses [32.042, 18.015] into mole fractions [0.35989, 0.64011] and a mean molar mass 0.0230632 kg/mol. The mole fractions represent the proportion of each component on a molar basis, while the mean molar mass provides the average mass per mole of the mixture. These values are essential for thermodynamic property estimation and stoichiometric calculations that require molar quantities rather than mass-based data.

<a id="chemeng_recycle_purge"></a>

#### Reactor with recycle and purge: `chemeng_recycle_purge`

Steady-state flows of a reactor A -> B with complete product separation, recycle of unreacted A and inert, and a purge that removes a fraction of the recycle stream to stop the inert accumulating.

$$
A_{in} = \frac{F_0 z_A}{1 - (1-f)(1-X)},\quad I_{in} = \frac{F_0(1-z_A)}{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F0` | in | fresh feed flow (reactant A plus inert) | mol/s |
| `zA` | in | mole fraction of A in the fresh feed (the rest is inert) | - |
| `X` | in | single-pass conversion of A in the reactor | - |
| `f` | in | purge fraction: the fraction of the separator overhead (unreacted A + inert) that is purged | - |
| `R` | out | recycle flow | mol/s |
| `P` | out | purge flow | mol/s |
| `reactor_feed` | out | combined reactor feed flow | mol/s |
| `X_overall` | out | overall conversion of A (fresh feed to product) | - |
| `product` | out | production rate of B | mol/s |
| `y_inert` | out | mole fraction of inert in the recycle and purge | - |

**Applies when:** 0 < zA <= 1; 0 < X < 1; 0 < f < 1; A -> B one to one; the separator removes all B.

```shell
$ chemical-engineering solve chemeng_recycle_purge "F0=100 mol/s" zA=0.98 X=0.25 f=0.1
R = 221.5384615 mol/s
P = 24.61538462 mol/s
reactor_feed = 321.5384615 mol/s
X_overall = 0.7692307692
product = 75.38461538 mol/s
y_inert = 0.08125
  by: Reactor with recycle and purge  (balances on A and inert around the mixer, reactor, separator and splitter: A_in = F0 zA/(1 - (1 - f)(1 - X)), inert_in = F0 (1 - zA)/f)
  F0 = 100 mol/s
  zA = 0.98
  X = 0.25
  f = 0.1
  assumes: 0 < zA <= 1; 0 < X < 1; 0 < f < 1; A -> B one to one; the separator removes all B
```

**What the worked example means:** This calculation determines the steady-state flows for a reactor system featuring recycle and purge streams. Using the fresh feed rate 100 mol/s, reactant fraction 0.98, single-pass conversion 0.25, and purge fraction 0.1, it computes the combined reactor feed 321.538 mol/s, recycle flow 221.538 mol/s, and purge flow 24.6154 mol/s. The results indicate how much unreacted material is returned to the reactor versus vented, ensuring inert accumulation is controlled while maintaining the desired production rate 75.3846 mol/s and overall conversion 0.769231.

<a id="chemeng_binary_separator_balance"></a>

#### Two-product separator balance (binary distillation column), `chemeng_binary_separator_balance`

Distillate and bottoms flows of a binary separator from the feed flow and the three compositions, with the recoveries of the light component in the distillate and of the heavy component in the bottoms.

$$
D = F\,\frac{z_F - x_B}{x_D - x_B},\quad B = F - D
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | feed molar flow | mol/s |
| `zF` | in | mole fraction of the light component in the feed | - |
| `xD` | in | mole fraction of the light component in the distillate (top product) | - |
| `xB` | in | mole fraction of the light component in the bottoms | - |
| `D` | out | distillate flow | mol/s |
| `B` | out | bottoms flow | mol/s |
| `recovery_D` | out | fraction of the light component recovered in the distillate | - |
| `recovery_B` | out | fraction of the heavy component recovered in the bottoms | - |

**Applies when:** xB < zF < xD; steady state, no reaction.

```shell
$ chemical-engineering solve chemeng_binary_separator_balance "F=100 mol/s" zF=0.4 xD=0.95 xB=0.05
D = 38.88888889 mol/s
B = 61.11111111 mol/s
recovery_D = 0.9236111111
recovery_B = 0.9675925926
  by: Two-product separator balance (binary distillation column)  (total balance F = D + B and light-component balance F zF = D xD + B xB)
  F = 100 mol/s
  zF = 0.4
  xD = 0.95
  xB = 0.05
  assumes: xB < zF < xD; steady state, no reaction
```

**What the worked example means:** This calculation determines the product flows from a binary separator using total and component balances. Given a feed flow of 100 mol/s and compositions of 0.4 in the feed, 0.95 in the distillate, and 0.05 in the bottoms, the model computes the distillate rate as 38.8889 mol/s and the bottoms rate as 61.1111 mol/s. The results indicate that 0.923611 of the light component is recovered in the top product, while 0.967593 of the heavy component is recovered in the bottom product.

<a id="chemeng_unsteady_mixing_tank"></a>

#### Unsteady well-mixed tank (concentration change), `chemeng_unsteady_mixing_tank`

Concentration in a constant-volume well-mixed tank at time t after the inlet concentration changes (inflow equals outflow).

$$
C(t) = C_{in} + (C_0 - C_{in})\,e^{-q t/V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | liquid volume of the tank | m^3 |
| `q` | in | volumetric flow in and out | m^3/s |
| `C_in` | in | inlet concentration | mol/m^3 |
| `C0` | in | concentration in the tank at t = 0 | mol/m^3 |
| `t` | in | time | s |
| `C` | out | concentration in the tank (and outlet) at time t | mol/m^3 |

**Applies when:** V > 0, q > 0; perfect mixing; constant volume (inflow = outflow).

```shell
$ chemical-engineering solve chemeng_unsteady_mixing_tank "V=10 m^3" "q=0.01 m^3/s" "C_in=500 mol/m^3" "C0=0 mol/m^3" "t=1000 s"
C = 316.0602794 mol/m^3
  by: Unsteady well-mixed tank (concentration change)  (solute balance V dC/dt = q (C_in - C), integrated: C = C_in + (C0 - C_in) exp(-q t/V))
  V = 10 m^3
  q = 0.01 m^3/s
  C_in = 500 mol/m^3
  C0 = 0 mol/m^3
  t = 1000 s
  assumes: V > 0, q > 0; perfect mixing; constant volume (inflow = outflow)
```

**What the worked example means:** This calculation determines the solute concentration inside a well-mixed tank at a specific moment. It uses the initial concentration 0 mol/m^3, the new inlet concentration 500 mol/m^3, and the system parameters volume 10 m^3 and flow rate 0.01 m^3/s. The result, 316.06 mol/m^3, shows how far the mixture has moved toward the new inlet value over time 1000 s.

### Petroleum engineering (4)

<a id="chemeng_exponential_decline"></a>

#### Exponential decline curve: rate and cumulative production, `chemeng_exponential_decline`

Production rate and cumulative production at time t of a well in exponential (constant-percentage) decline (Arps b = 0).

$$
q = q_ie^{-D_it},\quad N_p = \frac{q_i - q}{D_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `qi` | in | initial rate (bbl/day and STB/day are converted here) | m^3/s |
| `Di` | in | initial nominal decline rate, e.g. 0.3 1/yr | 1/s |
| `t` | in | time since the start of the decline | s |
| `q` | out | rate at time t | m^3/s |
| `Np` | out | cumulative production to time t | m^3 |

**Applies when:** boundary-dominated flow, constant operating conditions.

```shell
$ chemical-engineering solve chemeng_exponential_decline "qi=0.001840130728 m^3/s" "Di=9.506426344e-09 1/s" "t=63115200 s"
q = 0.001009885155 m^3/s
Np = 87335.19227 m^3
  by: Exponential decline curve: rate and cumulative production  (q = qi exp(-Di t); Np = (qi - q)/Di)
  qi = 0.001840130728 m^3/s
  Di = 9.506426344e-09 1/s
  t = 63115200 s
  assumes: boundary-dominated flow, constant operating conditions
```

**What the worked example means:** This calculation determines the instantaneous production rate and total accumulated volume for a well undergoing exponential decline. Using an initial rate of 0.00184013 m^3/s, a decline rate of 9.50643e-09 1/s, and elapsed time of 6.31152e+07 s, the model computes the current rate as 0.00100989 m^3/s and the cumulative production as 87335.2 m^3. These results reflect a scenario where the percentage decline remains constant, typical for boundary-dominated flow conditions.

<a id="chemeng_gas_well_backpressure"></a>

#### Gas-well deliverability from a back-pressure test (Rawlins-Schellhardt), `chemeng_gas_well_backpressure`

Exponent n, coefficient C and absolute open flow of a gas well from multi-rate test data fitted to q = C (pr^2 - pwf^2)^n.

$$
q = C\left(\bar p_r^2 - p_{wf}^2\right)^n,\quad AOF = C\,\bar p_r^{2n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | test gas rates, in q_unit | - |
| `pwf` | in | flowing bottom-hole pressures at those rates, in p_unit | - |
| `q_unit` | in | unit of the rates: m3_per_day or Mscf_per_day | - |
| `p_unit` | in | unit of the flowing pressures | - |
| `pr` | in | average reservoir (shut-in) pressure | Pa |
| `n` | out | back-pressure exponent | - |
| `log10_C` | out | log10 of the coefficient C, with q in m^3/s and pressures in Pa | - |
| `AOF` | out | absolute open flow (rate at pwf = 0) | m^3/s |

**Applies when:** stabilised flow at every rate; 0.3 < n < 1.5.

```shell
$ chemical-engineering solve chemeng_gas_well_backpressure "q=[4288, 9265, 15552, 20177]" "pwf=[403.1, 394, 378.5, 362.6]" q_unit=Mscf_per_day p_unit=psi "pr=2814439.927 Pa"
n = 0.7277574483
log10_C = -8.065527458
AOF = 20.97290776 m^3/s
  by: Gas-well deliverability from a back-pressure test (Rawlins-Schellhardt)  (least squares of log q against log(pr^2 - pwf^2): slope n, intercept log C; AOF = C pr^(2n))
  pr = 2814439.927 Pa
  q_unit = Mscf_per_day
  p_unit = psi
  assumes: stabilised flow at every rate; 0.3 < n < 1.5
```

**What the worked example means:** This calculation determines the gas well deliverability parameters by fitting test rates [4288, 9265, 15552, 20177] and flowing bottom-hole pressures [403.1, 394, 378.5, 362.6] against the average reservoir pressure 2.81444e+06 Pa. The method uses least squares on the logarithmic form of the Rawlins-Schellhardt equation to find the exponent 0.727757 and the intercept representing the coefficient -8.06553. These values allow prediction of the absolute open flow potential, which is calculated as 20.9729 m^3/s, assuming stabilized flow conditions throughout the test.

<a id="chemeng_harmonic_decline"></a>

#### Harmonic decline curve: rate and cumulative production, `chemeng_harmonic_decline`

Production rate and cumulative production at time t of a well in harmonic decline (Arps b = 1).

$$
q = \frac{q_i}{1 + D_it},\quad N_p = \frac{q_i}{D_i}\ln\frac{q_i}{q}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `qi` | in | initial rate (bbl/day and STB/day are converted here) | m^3/s |
| `Di` | in | initial nominal decline rate, e.g. 0.3 1/yr | 1/s |
| `t` | in | time since the start of the decline | s |
| `q` | out | rate at time t | m^3/s |
| `Np` | out | cumulative production to time t | m^3 |

**Applies when:** boundary-dominated flow.

```shell
$ chemical-engineering solve chemeng_harmonic_decline "qi=0.001840130728 m^3/s" "Di=1.267523513e-08 1/s" "t=94672800 s"
q = 0.000836423058 m^3/s
Np = 114464.513 m^3
  by: Harmonic decline curve: rate and cumulative production  (q = qi/(1 + Di t); Np = (qi/Di) ln(qi/q))
  qi = 0.001840130728 m^3/s
  Di = 1.267523513e-08 1/s
  t = 94672800 s
  assumes: boundary-dominated flow
```

**What the worked example means:** This calculation determines the current flow rate and total volume produced for a well undergoing harmonic decline. Given an initial rate of 0.00184013 m^3/s and a decline factor of 1.26752e-08 1/s, the model predicts that after 9.46728e+07 s has passed, the instantaneous rate drops to 0.000836423 m^3/s. Simultaneously, the accumulated fluid extracted from the reservoir up to this point totals 114465 m^3. This relationship assumes boundary-dominated flow where the decline rate is proportional to the square of the flow rate.

<a id="chemeng_hyperbolic_decline"></a>

#### Hyperbolic decline curve: rate and cumulative production, `chemeng_hyperbolic_decline`

Production rate and cumulative production at time t of a well in hyperbolic decline with Arps exponent 0 < b < 1.

$$
q = \frac{q_i}{(1 + bD_it)^{1/b}},\quad N_p = \frac{q_i^b}{(1-b)D_i}\left(q_i^{1-b} - q^{1-b}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `qi` | in | initial rate (bbl/day and STB/day are converted here) | m^3/s |
| `Di` | in | initial nominal decline rate, e.g. 0.3 1/yr | 1/s |
| `b` | in | Arps decline exponent (0 < b < 1) | - |
| `t` | in | time since the start of the decline | s |
| `q` | out | rate at time t | m^3/s |
| `Np` | out | cumulative production to time t | m^3 |

**Applies when:** 0 < b < 1; boundary-dominated flow.

```shell
$ chemical-engineering solve chemeng_hyperbolic_decline "qi=0.001840130728 m^3/s" "Di=1.584404391e-08 1/s" b=0.5 "t=94672800 s"
q = 0.0006008590131 m^3/s
Np = 99548.75907 m^3
  by: Hyperbolic decline curve: rate and cumulative production  (q = qi (1 + b Di t)^(-1/b); Np = qi^b/((1 - b) Di) (qi^(1-b) - q^(1-b)))
  qi = 0.001840130728 m^3/s
  Di = 1.584404391e-08 1/s
  b = 0.5
  t = 94672800 s
  assumes: 0 < b < 1; boundary-dominated flow
```

**What the worked example means:** This calculation determines the production rate and cumulative volume for a well in hyperbolic decline. Using an initial rate of 0.00184013 m^3/s and a decline rate of 1.5844e-08 1/s, with an Arps exponent of 0.5, the current rate is 0.000600859 m^3/s and total production is 99548.8 m^3. The result reflects how quickly output drops over time under boundary-dominated flow conditions.

### Process control (8)

<a id="chemeng_closed_loop_stability"></a>

#### Closed-loop stability of a P-controlled process (closed-loop poles), `chemeng_closed_loop_stability`

Whether a feedback loop with a proportional controller Kc around a process G(s) = num(s)/den(s) is stable: the largest real part of the closed-loop poles and how many lie in the right half-plane.

$$
1 + K_cG(s) = 0 \iff \mathrm{den}(s) + K_c\,\mathrm{num}(s) = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `num` | in | numerator coefficients of G(s), highest power of s first | - |
| `den` | in | denominator coefficients of G(s), highest power of s first | - |
| `Kc` | in | controller gain | - |
| `max_real` | out | largest real part of the closed-loop poles, 1/(time unit of the coefficients) | - |
| `n_rhp` | out | number of closed-loop poles in the right half-plane | - |
| `stable` | out | stable or unstable (text) | - |

**Applies when:** proper process; no pole on the imaginary axis or nearly repeated poles (refused).

```shell
$ chemical-engineering solve chemeng_closed_loop_stability "num=[1]" "den=[1, 3, 3, 1]" Kc=10
max_real = 0.07721734502
n_rhp = 2
stable = unstable
  by: Closed-loop stability of a P-controlled process (closed-loop poles)  (roots of den(s) + Kc num(s) = 0 (30-digit polynomial root finding))
  Kc = 10
  assumes: proper process; no pole on the imaginary axis or nearly repeated poles (refused)
```

**What the worked example means:** This calculation determines the stability of a feedback loop by finding the roots of the characteristic polynomial formed from the process numerator, denominator, and controller gain. The result indicates that the largest real part of any closed-loop pole is 0.0772173, which means there are 2 unstable poles, classifying the system as unstable.

<a id="chemeng_pid_controller_output"></a>

#### Discrete PID controller output for an error history, `chemeng_pid_controller_output`

Output of a digital position-form PID controller at each sample for a stated history of errors (rectangular integration, derivative on the error, loop at rest before the first sample).

$$
u_k = \bar u + K_c\left[e_k + \frac{\Delta t}{\tau_I}\sum_{j=1}^{k} e_j + \frac{\tau_D}{\Delta t}(e_k - e_{k-1})\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kc` | in | controller gain | - |
| `tauI` | in | integral (reset) time | s |
| `tauD` | in | derivative time (0 for PI) | s |
| `dt` | in | sampling interval | s |
| `e` | in | error samples e_1 ... e_n (set point minus measurement) | - |
| `bias` | in | controller output bias (output when the error has always been zero) | - |
| `u_last` | out | controller output at the last sample | - |
| `u` | out | controller output at every sample | - |

**Applies when:** position form, e_0 = 0 before the first sample; rectangular integration including the current sample.

```shell
$ chemical-engineering solve chemeng_pid_controller_output Kc=2 "tauI=10 s" "tauD=1 s" "dt=1 s" "e=[1, 0.8, 0.5, 0.3]" bias=50
u_last = 50.72
u = [54.2,51.56,50.86,50.72]
  by: Discrete PID controller output for an error history  (u_k = bias + Kc [e_k + (dt/tauI) sum_(j<=k) e_j + (tauD/dt)(e_k - e_(k-1))])
  Kc = 2
  tauI = 10 s
  tauD = 1 s
  dt = 1 s
  bias = 50
  assumes: position form, e_0 = 0 before the first sample; rectangular integration including the current sample
```

**What the worked example means:** This calculation determines the discrete position-form PID controller output for each time step using the provided error history. The final result, 50.72, reflects the combined proportional, integral, and derivative actions based on the sampling interval 1 s and tuning parameters 10 s and 1 s. It represents the corrective signal sent to the actuator assuming the loop was initially at rest.

<a id="chemeng_fopdt_step_response"></a>

#### First-order-plus-dead-time process: response to a step, `chemeng_fopdt_step_response`

Deviation of the output of a first-order process with dead time at time t after a step of size M in its input.

$$
y(t) = KM\left(1 - e^{-(t-\theta)/\tau}\right)\,u(t-\theta)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | process gain (output change per unit input change) | - |
| `tau` | in | process time constant | s |
| `theta` | in | dead time | s |
| `M` | in | size of the input step (deviation) | - |
| `t` | in | time after the step | s |
| `y` | out | output deviation at time t (in the output's units) | - |

**Applies when:** the process starts at steady state; linear first-order dynamics with dead time.

```shell
$ chemical-engineering solve chemeng_fopdt_step_response K=2 "tau=300 s" "theta=60 s" M=5 "t=360 s"
y = 6.321205588
  by: First-order-plus-dead-time process: response to a step  (y(t) = K M (1 - exp(-(t - theta)/tau)) for t > theta, 0 before)
  K = 2
  tau = 300 s
  theta = 60 s
  M = 5
  t = 360 s
  assumes: the process starts at steady state; linear first-order dynamics with dead time
```

**What the worked example means:** This calculation determines the dynamic response of a first-order process with dead time following a step input. The result, 6.32121, represents the deviation of the output variable at time 360 s after the change. This value is derived from the process gain 2, the magnitude of the input step 5, the time constant 300 s, and the dead time 60 s. Before the dead time elapses, the output remains unchanged; afterwards, it gradually approaches the final steady-state value determined by the product of the gain and the step size.

<a id="chemeng_lag_frequency_response"></a>

#### Frequency response of first-order lags in series (amplitude ratio and phase), `chemeng_lag_frequency_response`

Amplitude ratio and phase angle of a process made of one to three first-order lags in series, at angular frequency w.

$$
AR = \frac{K}{\prod_i\sqrt{1 + \omega^2\tau_i^2}},\quad \varphi = -\sum_i \arctan(\omega\tau_i)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | process gain | - |
| `taus` | in | time constants of the lags, in t_unit | - |
| `t_unit` | in | unit of the time constants | - |
| `w` | in | angular frequency of the input sinusoid | rad/s |
| `AR` | out | amplitude ratio (output amplitude per input amplitude) | - |
| `phase_deg` | out | phase angle in degrees (negative: lag) | - |

**Applies when:** K > 0; linear lags without dead time.

```shell
$ chemical-engineering solve chemeng_lag_frequency_response K=2 "taus=[10]" t_unit=s "w=0.1 rad/s"
AR = 1.414213562
phase_deg = -45
  by: Frequency response of first-order lags in series (amplitude ratio and phase)  (AR = K/prod sqrt(1 + w^2 tau_i^2); phase = -sum atan(w tau_i))
  K = 2
  w = 0.1 rad/s
  t_unit = s
  assumes: K > 0; linear lags without dead time
```

**What the worked example means:** This calculation determines the frequency response of a process composed of one to three first-order lags in series. Given the process gain 2, time constants [10], and angular frequency 0.1 rad/s, it computes the amplitude ratio 1.41421 and phase angle -45. The amplitude ratio represents the output amplitude relative to the input amplitude, while the phase angle indicates the time delay expressed in degrees, where negative values signify that the output lags behind the input.

<a id="chemeng_pi_loop_margins"></a>

#### Gain and phase margins of a PI loop around lags in series, `chemeng_pi_loop_margins`

Gain margin, phase margin and the two crossover frequencies of a loop with a PI controller and a process of three to five first-order lags.

$$
GM = \frac{1}{|L(j\omega_{pc})|},\quad PM = 180^\circ + \arg L(j\omega_{gc})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kc` | in | controller gain | - |
| `tauI` | in | integral time | s |
| `K` | in | process gain | - |
| `taus` | in | time constants of the lags, in t_unit (the integral time is converted to this unit too) | - |
| `t_unit` | in | unit of the time constants | - |
| `GM` | out | gain margin (ratio) | - |
| `PM_deg` | out | phase margin, degrees | - |
| `w_pc` | out | phase-crossover frequency (phase -180 degrees) | rad/s |
| `w_gc` | out | gain-crossover frequency (\|L\| = 1) | rad/s |

**Applies when:** one phase crossover (refused otherwise); Kc K > 0.

```shell
$ chemical-engineering solve chemeng_pi_loop_margins Kc=1 "tauI=5 s" K=2 "taus=[1, 1, 1]" t_unit=min
GM = 3.220937271
PM_deg = 51.05308015
w_pc = 0.02624999919 rad/s
w_gc = 0.01313168583 rad/s
  by: Gain and phase margins of a PI loop around lags in series  (L(jw) = Kc (1 + 1/(j w tauI)) K/prod(1 + j w tau_i); |L| = 1 and arg L = -180 degrees by bisection; GM = 1/|L(j w_pc)|, PM = 180 + arg L(j w_gc))
  Kc = 1
  tauI = 5 s
  K = 2
  t_unit = min
  assumes: one phase crossover (refused otherwise); Kc K > 0
```

**What the worked example means:** This calculation determines the stability margins for a control loop containing a proportional-integral controller and a process composed of several first-order lags. Using the open-loop transfer function, it identifies the frequency where the phase shift reaches minus one hundred eighty degrees and the frequency where the magnitude equals unity. The resulting gain margin of 3.22094 indicates how much the loop gain can increase before instability occurs, while the phase margin of 51.0531 shows how much additional phase lag can be tolerated. These values are derived from the controller gain 1, integral time 5 s, process gain 2, and the set of time constants [1, 1, 1].

<a id="chemeng_first_order_step_fit"></a>

#### Process gain and time constant from step-test data (least squares), `chemeng_first_order_step_fit`

Gain and time constant of a first-order process (dead time known) fitted by least squares to the recorded response of a step test.

$$
\min_{K,\tau}\sum_k\left[y_k - KM\left(1 - e^{-(t_k-\theta)/\tau}\right)\right]^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | sample times from the moment of the step, in t_unit | - |
| `y` | in | output deviations at those times | - |
| `t_unit` | in | unit of the times | - |
| `M` | in | size of the input step | - |
| `theta` | in | dead time (known) | s |
| `K` | out | process gain | - |
| `tau` | out | time constant | s |

**Applies when:** step from steady state; first-order response after the dead time; a unique interior least-squares minimum.

```shell
$ chemical-engineering solve chemeng_first_order_step_fit "t=[0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 20]" "y=[0, 0, 1.7, 3.1, 4.3, 5.2, 6, 7.1, 7.9, 8.4, 8.8, 9.1]" t_unit=min M=5 "theta=60 s"
K = 1.867074148
tau = 292.7781377 s
  by: Process gain and time constant from step-test data (least squares)  (least squares of y = K M (1 - exp(-(t - theta)/tau)): K solved linearly for each tau, tau from the zero of the derivative of the residual sum (bisection))
  M = 5
  theta = 60 s
  t_unit = min
  assumes: step from steady state; first-order response after the dead time; a unique interior least-squares minimum
```

**What the worked example means:** This calculation determines the dynamic parameters of a first-order system by fitting recorded step response data to an exponential model. The process gain, which represents the ratio of the output change to the input step size, is computed as 1.86707. The time constant, indicating how quickly the system reaches approximately sixty-three percent of its final value after dead time, is found to be 292.778 s. These values are essential for understanding the speed and magnitude of the system's reaction to control inputs.

<a id="chemeng_p_control_offset"></a>

#### Proportional control: steady-state offset after a set-point step, `chemeng_p_control_offset`

Steady-state output (as a fraction of a set-point step) and the offset of a P-only feedback loop around a process of first-order lags in series, after checking the loop is stable.

$$
\text{offset} = \frac{1}{1 + K_cK}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kc` | in | controller gain | - |
| `K` | in | process gain | - |
| `taus` | in | time constants of the first-order lags in series, in t_unit | - |
| `t_unit` | in | unit of the time constants | - |
| `y_ss` | out | final output change per unit set-point change | - |
| `offset` | out | steady-state offset per unit set-point change | - |

**Applies when:** Kc K > 0; the closed loop is stable (checked by the Routh array).

```shell
$ chemical-engineering solve chemeng_p_control_offset Kc=2 K=1.5 "taus=[5, 2]" t_unit=min
y_ss = 0.75
offset = 0.25
  by: Proportional control: steady-state offset after a set-point step  (final-value theorem: y_ss = Kc K/(1 + Kc K), offset = 1/(1 + Kc K); stability of prod(tau_i s + 1) + Kc K by the Routh array)
  Kc = 2
  K = 1.5
  t_unit = min
  assumes: Kc K > 0; the closed loop is stable (checked by the Routh array)
```

**What the worked example means:** This calculation determines the permanent error, or offset, that remains in a proportional-only feedback loop after a step change in the set-point. Using the controller gain 2 and the process gain 1.5, the tool computes the fraction of the set-point change that the system fails to achieve. The resulting offset is 0.25, while the actual final output change per unit set-point change is 0.75. This result assumes the closed-loop system is stable, which is verified through analysis of the time constants [5, 2] and the gains.

<a id="chemeng_ziegler_nichols_tuning"></a>

#### Ziegler-Nichols closed-loop tuning of lags in series, `chemeng_ziegler_nichols_tuning`

Ultimate gain and period of a process of first-order lags in series, and the Ziegler-Nichols closed-loop settings of a P, PI or PID controller.

$$
\sum_i\arctan(\omega_u\tau_i) = \pi,\quad K_u = \frac{1}{|G(j\omega_u)|},\quad P_u = \frac{2\pi}{\omega_u}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | process gain | - |
| `taus` | in | time constants of the lags (three or more), in t_unit | - |
| `t_unit` | in | unit of the time constants | - |
| `controller` | in | controller type: P, PI or PID | - |
| `Ku` | out | ultimate gain | - |
| `Pu` | out | ultimate period | s |
| `Kc` | out | controller gain | - |
| `tauI` | out | integral time (PI, PID) | s |
| `tauD` | out | derivative time (PID) | s |

**Applies when:** three or more lags (a finite ultimate gain); K > 0.

```shell
$ chemical-engineering solve chemeng_ziegler_nichols_tuning K=2 "taus=[1, 1, 1]" t_unit=min controller=PID
Ku = 4
Pu = 217.6559237 s
Kc = 2.4
tauI = 108.8279619 s
tauD = 27.20699046 s
  by: Ziegler-Nichols closed-loop tuning of lags in series  (phase crossover sum atan(w tau_i) = pi (bisection); Ku = 1/|G(j wu)|, Pu = 2 pi/wu; P: 0.5 Ku; PI: 0.45 Ku, Pu/1.2; PID: 0.6 Ku, Pu/2, Pu/8)
  K = 2
  t_unit = min
  controller = PID
  assumes: three or more lags (a finite ultimate gain); K > 0
```

**What the worked example means:** This calculation determines the ultimate gain 4 and oscillation period 217.656 s for a process with time constants [1, 1, 1] and gain 2. Based on these values, the recommended proportional controller setting is 2.4. For PI or PID configurations, the integral time is 108.828 s and the derivative time is 27.207 s, depending on the selected controller type.

### Property estimation (8)

<a id="chemeng_virial_compressibility"></a>

#### Gas compressibility from the virial equation (Pitzer-Abbott correlation), `chemeng_virial_compressibility`

Second virial coefficient from the Abbott (Pitzer) correlation and the compressibility factor Z = 1 + BP/(RT) of a gas at moderate pressure.

$$
\frac{BP_c}{RT_c} = 0.083 - \frac{0.422}{T_r^{1.6}} + \omega\left(0.139 - \frac{0.172}{T_r^{4.2}}\right),\quad Z = 1 + \frac{BP}{RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `P` | in | pressure | Pa |
| `Tc` | in | critical temperature | K |
| `Pc` | in | critical pressure | Pa |
| `omega` | in | acentric factor | - |
| `B` | out | second virial coefficient | m^3/mol |
| `Z` | out | compressibility factor | - |

**Applies when:** moderate pressure (truncated virial equation); non-polar or slightly polar gas.

```shell
$ chemical-engineering solve chemeng_virial_compressibility "T=510 K" "P=2500000 Pa" "Tc=425.1 K" "Pc=3796000 Pa" omega=0.2
B = -0.000205361259 m^3/mol
Z = 0.878925087
  by: Gas compressibility from the virial equation (Pitzer-Abbott correlation)  (B Pc/(R Tc) = B0 + omega B1, B0 = 0.083 - 0.422/Tr^1.6, B1 = 0.139 - 0.172/Tr^4.2; Z = 1 + B P/(R T))
  T = 510 K
  P = 2500000 Pa
  Tc = 425.1 K
  Pc = 3796000 Pa
  omega = 0.2
  assumes: moderate pressure (truncated virial equation); non-polar or slightly polar gas
```

**What the worked example means:** This calculation determines the gas compressibility factor using the Pitzer-Abbott correlation for the second virial coefficient. By inputting the temperature, pressure, critical properties, and acentric factor, the method computes the reduced temperature to find the dimensionless virial coefficients. The resulting second virial coefficient of -0.000205361 m^3/mol indicates the deviation from ideal behavior due to intermolecular forces at moderate pressures, leading to a final compressibility factor of 0.878925. This value adjusts the ideal gas law to better predict real gas density and flow characteristics.

<a id="chemeng_watson_heat_of_vaporization"></a>

#### Heat of vaporisation at another temperature (Watson), `chemeng_watson_heat_of_vaporization`

Heat of vaporisation at T from a known value at T_ref and the critical temperature (Watson's correlation, exponent 0.38).

$$
\Delta H_v(T) = \Delta H_v(T_{ref})\left(\frac{1 - T_r}{1 - T_{r,ref}}\right)^{0.38}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature wanted | K |
| `T_ref` | in | temperature of the known value | K |
| `Hvap_ref` | in | heat of vaporisation at T_ref | J/mol |
| `Tc` | in | critical temperature | K |
| `Hvap` | out | heat of vaporisation at T | J/mol |

**Applies when:** T, T_ref < Tc.

```shell
$ chemical-engineering solve chemeng_watson_heat_of_vaporization "T=298.15 K" "T_ref=373.15 K" "Hvap_ref=40656 J/mol" "Tc=647.1 K"
Hvap = 44571.73799 J/mol
  by: Heat of vaporisation at another temperature (Watson)  (dHvap(T) = dHvap(T_ref) [(1 - T/Tc)/(1 - T_ref/Tc)]^0.38)
  T = 298.15 K
  T_ref = 373.15 K
  Hvap_ref = 40656 J/mol
  Tc = 647.1 K
  assumes: T, T_ref < Tc
```

**What the worked example means:** This calculation determines the latent heat required for vaporisation at a specific temperature by scaling a known reference value using Watson's correlation. Starting with the heat of vaporisation measured at the reference temperature, which is 40656 J/mol, and adjusting for the reduced temperatures relative to the critical point, the method yields an estimated value of 44571.7 J/mol at the target temperature. This result helps engineers predict energy needs for phase changes under different operating conditions.

<a id="chemeng_riedel_heat_of_vaporization"></a>

#### Heat of vaporisation at the normal boiling point (Riedel), `chemeng_riedel_heat_of_vaporization`

Heat of vaporisation at the normal boiling point from the boiling point and critical constants (Riedel's equation, Pc in bar).

$$
\Delta H_{vb} = 1.093\,R\,T_b\,\frac{\ln P_c - 1.013}{0.930 - T_{br}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tb` | in | normal boiling point | K |
| `Tc` | in | critical temperature | K |
| `Pc` | in | critical pressure | Pa |
| `Hvap_b` | out | heat of vaporisation at the normal boiling point | J/mol |

**Applies when:** Tb well below Tc.

```shell
$ chemical-engineering solve chemeng_riedel_heat_of_vaporization "Tb=294 K" "Tc=466 K" "Pc=5550000 Pa"
Hvap_b = 26828.59041 J/mol
  by: Heat of vaporisation at the normal boiling point (Riedel)  (dHvb = 1.093 R Tb (ln Pc[bar] - 1.013)/(0.930 - Tbr))
  Tb = 294 K
  Tc = 466 K
  Pc = 5550000 Pa
  assumes: Tb well below Tc
```

**What the worked example means:** This calculation determines the energy required to vaporise one mole of substance at its normal boiling point using Riedel's correlation. It relies on the normal boiling temperature 294 K, the critical temperature 466 K, and the critical pressure 5.55e+06 Pa. The resulting latent heat value, 26828.6 J/mol, provides a thermodynamic baseline for substances where experimental data is scarce.

<a id="chemeng_pitzer_heat_of_vaporization"></a>

#### Heat of vaporisation from the acentric factor (Pitzer), `chemeng_pitzer_heat_of_vaporization`

Heat of vaporisation at T from the critical temperature and acentric factor (Pitzer's corresponding-states correlation as fitted by Carruth and Kobayashi).

$$
\frac{\Delta H_v}{RT_c} = 7.08(1-T_r)^{0.354} + 10.95\,\omega(1-T_r)^{0.456}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `Tc` | in | critical temperature | K |
| `omega` | in | acentric factor | - |
| `Hvap` | out | heat of vaporisation | J/mol |

**Applies when:** 0.6 < Tr < 1 (fit range).

```shell
$ chemical-engineering solve chemeng_pitzer_heat_of_vaporization "T=452 K" "Tc=645.6 K" omega=0.35
Hvap = 36690.97947 J/mol
  by: Heat of vaporisation from the acentric factor (Pitzer)  (dHv/(R Tc) = 7.08 (1 - Tr)^0.354 + 10.95 omega (1 - Tr)^0.456)
  T = 452 K
  Tc = 645.6 K
  omega = 0.35
  assumes: 0.6 < Tr < 1 (fit range)
```

**What the worked example means:** This calculation determines the energy required to vaporize one mole of liquid at a specific temperature using Pitzer's corresponding-states correlation. The result, 36691 J/mol, depends on the operating temperature 452 K, the fluid's critical temperature 645.6 K, and its acentric factor 0.35. This value is essential for performing accurate energy balances in distillation and evaporation units where experimental data is not readily available.

<a id="chemeng_peng_robinson"></a>

#### Peng-Robinson equation of state: compressibility and fugacity coefficient, `chemeng_peng_robinson`

Compressibility factors (vapour and liquid roots) and fugacity coefficients of a pure substance at T and P from the Peng-Robinson equation of state.

$$
Z^3 - (1-B)Z^2 + (A - 3B^2 - 2B)Z - (AB - B^2 - B^3) = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | critical temperature | K |
| `Pc` | in | critical pressure | Pa |
| `omega` | in | acentric factor | - |
| `T` | in | temperature | K |
| `P` | in | pressure | Pa |
| `Z_vapour` | out | compressibility factor of the vapour-like (largest) root | - |
| `Z_liquid` | out | compressibility factor of the liquid-like (smallest) root | - |
| `phi_vapour` | out | fugacity coefficient on the vapour root | - |
| `phi_liquid` | out | fugacity coefficient on the liquid root | - |
| `n_roots` | out | number of real roots above the covolume (1 or 3) | - |

**Applies when:** pure substance; not too close to the critical point or a spinodal (a double root is refused); one real root: both roots reported equal.

```shell
$ chemical-engineering solve chemeng_peng_robinson "Tc=507.6 K" "Pc=3025000 Pa" omega=0.2975 "T=400 K" "P=1000000 Pa"
Z_vapour = 0.6440214224
Z_liquid = 0.04692822373
phi_vapour = 0.7462319488
phi_liquid = 0.4215970079
n_roots = 3
  by: Peng-Robinson equation of state: compressibility and fugacity coefficient  (Z^3 - (1 - B)Z^2 + (A - 3B^2 - 2B)Z - (AB - B^2 - B^3) = 0 with a = 0.45724 R^2Tc^2/Pc alpha(T), b = 0.07780 R Tc/Pc, kappa = 0.37464 + 1.54226 omega - 0.26992 omega^2)
  Tc = 507.6 K
  Pc = 3025000 Pa
  omega = 0.2975
  T = 400 K
  P = 1000000 Pa
  assumes: pure substance; not too close to the critical point or a spinodal (a double root is refused); one real root: both roots reported equal
```

In the worked example, Tc = 507.6 K, Pc = 3025000 Pa, omega = 0.2975, T = 400 K, P = 1000000 Pa go through the equation above and give Z_vapour = 0.6440214224, Z_liquid = 0.04692822373, phi_vapour = 0.7462319488, phi_liquid = 0.4215970079. It applies only when pure substance; not too close to the critical point or a spinodal (a double root is refused); one real root: both roots reported equal: outside that it says what is wrong instead of returning a number.

<a id="chemeng_rackett_liquid_volume"></a>

#### Saturated liquid molar volume (Rackett): `chemeng_rackett_liquid_volume`

Molar volume of a saturated liquid from the critical constants and the critical compressibility factor (Rackett equation).

$$
V_s = \frac{RT_c}{P_c}Z_c^{\,1 + (1-T_r)^{2/7}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `Tc` | in | critical temperature | K |
| `Pc` | in | critical pressure | Pa |
| `Zc` | in | critical compressibility factor | - |
| `Vm` | out | saturated liquid molar volume | m^3/mol |

**Applies when:** T < Tc.

```shell
$ chemical-engineering solve chemeng_rackett_liquid_volume "T=272.03889 K" "Tc=369.83 K" "Pc=4248000 Pa" Zc=0.2763
Vm = 8.299225005e-05 m^3/mol
  by: Saturated liquid molar volume (Rackett)  (Vs = (R Tc/Pc) Zc^[1 + (1 - Tr)^(2/7)])
  T = 272.03889 K
  Tc = 369.83 K
  Pc = 4248000 Pa
  Zc = 0.2763
  assumes: T < Tc
```

**What the worked example means:** This calculation determines the molar volume of a saturated liquid using the Rackett equation. It relies on the system temperature, critical temperature, critical pressure, and critical compressibility factor. The resulting value, 8.29923e-05 m^3/mol, represents the space occupied by one mole of the liquid phase under these conditions, providing an estimate useful for process design when experimental density data is unavailable.

<a id="chemeng_srk"></a>

#### Soave-Redlich-Kwong equation of state: compressibility and fugacity coefficient, `chemeng_srk`

Compressibility factors (vapour and liquid roots) and fugacity coefficients of a pure substance at T and P from the Soave-Redlich-Kwong equation of state.

$$
Z^3 - Z^2 + (A - B - B^2)Z - AB = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | critical temperature | K |
| `Pc` | in | critical pressure | Pa |
| `omega` | in | acentric factor | - |
| `T` | in | temperature | K |
| `P` | in | pressure | Pa |
| `Z_vapour` | out | compressibility factor of the vapour-like (largest) root | - |
| `Z_liquid` | out | compressibility factor of the liquid-like (smallest) root | - |
| `phi_vapour` | out | fugacity coefficient on the vapour root | - |
| `phi_liquid` | out | fugacity coefficient on the liquid root | - |
| `n_roots` | out | number of real roots above the covolume (1 or 3) | - |

**Applies when:** pure substance; not too close to the critical point or a spinodal; one real root: both roots reported equal.

```shell
$ chemical-engineering solve chemeng_srk "Tc=507.6 K" "Pc=3025000 Pa" omega=0.2975 "T=400 K" "P=1000000 Pa"
Z_vapour = 0.658909723
Z_liquid = 0.05321836273
phi_vapour = 0.7576961392
phi_liquid = 0.4306444158
n_roots = 3
  by: Soave-Redlich-Kwong equation of state: compressibility and fugacity coefficient  (Z^3 - Z^2 + (A - B - B^2)Z - AB = 0 with a = 0.42748 R^2Tc^2/Pc alpha(T), b = 0.08664 R Tc/Pc, m = 0.480 + 1.574 omega - 0.176 omega^2)
  Tc = 507.6 K
  Pc = 3025000 Pa
  omega = 0.2975
  T = 400 K
  P = 1000000 Pa
  assumes: pure substance; not too close to the critical point or a spinodal; one real root: both roots reported equal
```

In the worked example, Tc = 507.6 K, Pc = 3025000 Pa, omega = 0.2975, T = 400 K, P = 1000000 Pa go through the equation above and give Z_vapour = 0.658909723, Z_liquid = 0.05321836273, phi_vapour = 0.7576961392, phi_liquid = 0.4306444158. It applies only when pure substance; not too close to the critical point or a spinodal; one real root: both roots reported equal: outside that it says what is wrong instead of returning a number.

<a id="chemeng_lee_kesler_vapour_pressure"></a>

#### Vapour pressure by the Lee-Kesler correlation, `chemeng_lee_kesler_vapour_pressure`

Vapour pressure of a pure substance from its critical constants and acentric factor (Lee-Kesler corresponding-states correlation).

$$
\ln\frac{P^{sat}}{P_c} = f^{(0)}(T_r) + \omega f^{(1)}(T_r)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `Tc` | in | critical temperature | K |
| `Pc` | in | critical pressure | Pa |
| `omega` | in | acentric factor | - |
| `Psat` | out | vapour pressure | Pa |

**Applies when:** T < Tc; non-polar or slightly polar substances.

```shell
$ chemical-engineering solve chemeng_lee_kesler_vapour_pressure "T=400 K" "Tc=568.7 K" "Pc=2490000 Pa" omega=0.399
Psat = 104829.4568 Pa
  by: Vapour pressure by the Lee-Kesler correlation  (ln(Psat/Pc) = f0(Tr) + omega f1(Tr) with the Lee-Kesler functions)
  T = 400 K
  Tc = 568.7 K
  Pc = 2490000 Pa
  omega = 0.399
  assumes: T < Tc; non-polar or slightly polar substances
```

**What the worked example means:** This calculation determines the saturation pressure of a pure substance using its critical properties and acentric factor. By applying the Lee-Kesler correlation to an input temperature of 400 K, a critical temperature of 568.7 K, a critical pressure of 2.49e+06 Pa, and an acentric factor of 0.399, the model estimates the vapour pressure to be 104829 Pa. This result helps predict how readily the substance evaporates under these specific conditions.

### Reactor design (20)

<a id="chemeng_batch_conversion_first_order"></a>

#### Batch reactor conversion after a time (first order), `chemeng_batch_conversion_first_order`

Conversion reached in an isothermal batch reactor after time t for a first-order reaction.

$$
X = 1 - e^{-kt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | first-order rate constant | 1/s |
| `t` | in | reaction time | s |
| `X` | out | conversion of A | - |

**Applies when:** isothermal, constant volume, -r_A = k C_A.

```shell
$ chemical-engineering solve chemeng_batch_conversion_first_order "k=0.001 1/s" "t=1000 s"
X = 0.6321205588
  by: Batch reactor conversion after a time (first order)  (X = 1 - exp(-k t))
  k = 0.001 1/s
  t = 1000 s
  assumes: isothermal, constant volume, -r_A = k C_A
```

**What the worked example means:** This calculation determines the fractional conversion of reactant A, denoted as 0.632121, achieved in an isothermal batch reactor. The result is derived from the first-order rate constant 0.001 1/s and the elapsed reaction time 1000 s. A higher value indicates that a larger portion of the initial reactant has been transformed into products during this period.

<a id="chemeng_batch_conversion_second_order"></a>

#### Batch reactor conversion after a time (second order), `chemeng_batch_conversion_second_order`

Conversion and concentration reached in an isothermal batch reactor after time t when -r_A = k C_A^2.

$$
X = \frac{k C_{A0} t}{1 + k C_{A0} t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `t` | in | reaction time | s |
| `X` | out | conversion of A | - |
| `CA` | out | concentration of A at time t | mol/m^3 |

**Applies when:** isothermal, constant volume.

```shell
$ chemical-engineering solve chemeng_batch_conversion_second_order "k=0.0001 m^3/(mol*s)" "CA0=1000 mol/m^3" "t=40 s"
X = 0.8
CA = 200 mol/m^3
  by: Batch reactor conversion after a time (second order)  (1/C_A = 1/C_A0 + k t)
  k = 0.0001 m^3/(mol*s)
  CA0 = 1000 mol/m^3
  t = 40 s
  assumes: isothermal, constant volume
```

**What the worked example means:** This calculation determines the extent of reaction in an isothermal batch reactor assuming constant volume. Using the rate constant 0.0001 m^3/(mol*s), initial concentration 1000 mol/m^3, and time 40 s, it applies the integrated second-order rate law to find that the final concentration of reactant A is 200 mol/m^3 and the fractional conversion is 0.8. This result indicates how much of the starting material has been consumed after the specified duration.

<a id="chemeng_batch_time_first_order"></a>

#### Batch reactor time for a conversion (first order), `chemeng_batch_time_first_order`

Reaction time an isothermal batch reactor needs to reach conversion X for a first-order reaction.

$$
t = \frac{1}{k}\ln\frac{1}{1-X}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | first-order rate constant | 1/s |
| `X` | in | conversion of A wanted | - |
| `t` | out | reaction time | s |

**Applies when:** 0 <= X < 1; isothermal, constant volume, -r_A = k C_A.

```shell
$ chemical-engineering solve chemeng_batch_time_first_order "k=0.001 1/s" X=0.9
t = 2302.585093 s
  by: Batch reactor time for a conversion (first order)  (design equation t = C_A0 * integral of dX/(-r_A) = ln(1/(1 - X))/k)
  k = 0.001 1/s
  X = 0.9
  assumes: 0 <= X < 1; isothermal, constant volume, -r_A = k C_A
```

**What the worked example means:** This calculation determines the reaction time needed for an isothermal batch reactor to achieve a specific conversion for a first-order reaction. Using the rate constant 0.001 1/s and the target conversion 0.9, the model computes that the process must run for 2302.59 s. This duration is critical for scheduling production batches and ensuring the reactant is sufficiently converted before stopping the reaction.

<a id="chemeng_batch_time_second_order"></a>

#### Batch reactor time for a conversion (second order), `chemeng_batch_time_second_order`

Reaction time an isothermal batch reactor needs to reach conversion X when -r_A = k C_A^2.

$$
t = \frac{X}{k\,C_{A0}(1-X)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `X` | in | conversion of A wanted | - |
| `t` | out | reaction time | s |

**Applies when:** 0 <= X < 1; isothermal, constant volume.

```shell
$ chemical-engineering solve chemeng_batch_time_second_order "k=0.0001 m^3/(mol*s)" "CA0=1000 mol/m^3" X=0.8
t = 40 s
  by: Batch reactor time for a conversion (second order)  (t = C_A0 * integral of dX/(k C_A0^2 (1 - X)^2) = X/(k C_A0 (1 - X)))
  k = 0.0001 m^3/(mol*s)
  CA0 = 1000 mol/m^3
  X = 0.8
  assumes: 0 <= X < 1; isothermal, constant volume
```

**What the worked example means:** This calculation determines the reaction time needed for an isothermal batch reactor with constant volume to achieve a specific conversion level for a second-order reaction. Using the initial concentration of reactant A, which is 1000 mol/m^3, and the rate constant 0.0001 m^3/(mol*s), the model computes that it takes 40 s to reach the target conversion of 0.8. This result highlights how higher-order kinetics demand disproportionately more time as conversion increases.

<a id="chemeng_cstr_conversion_first_order"></a>

#### CSTR conversion for a volume (first order), `chemeng_cstr_conversion_first_order`

Steady-state conversion of a CSTR of volume V for a first-order liquid-phase reaction.

$$
X = \frac{k\tau}{1 + k\tau},\ \tau = V/v_0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `V` | in | reactor volume | m^3 |
| `k` | in | first-order rate constant | 1/s |
| `X` | out | conversion of A | - |

**Applies when:** steady state, perfect mixing, constant density, -r_A = k C_A.

```shell
$ chemical-engineering solve chemeng_cstr_conversion_first_order "v0=0.01 m^3/s" "V=20 m^3" "k=0.001 1/s"
X = 0.6666666667
  by: CSTR conversion for a volume (first order)  (X = k tau/(1 + k tau), tau = V/v0 (the Damkohler number k tau))
  v0 = 0.01 m^3/s
  V = 20 m^3
  k = 0.001 1/s
  assumes: steady state, perfect mixing, constant density, -r_A = k C_A
```

**What the worked example means:** This calculation determines the fractional conversion of reactant A in a continuous stirred tank reactor operating at steady state. It uses the first-order rate constant 0.001 1/s, the reactor volume 20 m^3, and the volumetric feed flow 0.01 m^3/s to compute the residence time and subsequent conversion. The resulting value 0.666667 represents the proportion of reactant converted, indicating how effectively the current reactor size handles the incoming flow rate.

<a id="chemeng_cstr_conversion_second_order"></a>

#### CSTR conversion for a volume (second order), `chemeng_cstr_conversion_second_order`

Steady-state conversion and exit concentration of a CSTR of volume V when -r_A = k C_A^2.

$$
\frac{C_A}{C_{A0}} = \frac{2}{1 + \sqrt{1 + 4\,k\tau C_{A0}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `V` | in | reactor volume | m^3 |
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `X` | out | conversion of A | - |
| `CA` | out | exit concentration of A | mol/m^3 |

**Applies when:** steady state, perfect mixing, constant density.

```shell
$ chemical-engineering solve chemeng_cstr_conversion_second_order "v0=0.01 m^3/s" "V=2 m^3" "k=1e-05 m^3/(mol*s)" "CA0=1000 mol/m^3"
X = 0.5
CA = 500 mol/m^3
  by: CSTR conversion for a volume (second order)  (balance C_A0 - C_A = k tau C_A^2 solved for C_A (positive root, cancellation-free form))
  v0 = 0.01 m^3/s
  V = 2 m^3
  k = 1e-05 m^3/(mol*s)
  CA0 = 1000 mol/m^3
  assumes: steady state, perfect mixing, constant density
```

**What the worked example means:** This calculation determines the performance of a Continuous Stirred Tank Reactor operating at steady state with perfect mixing. Using the feed flow rate, reactor volume, rate constant, and initial concentration, it solves the mass balance for a second-order reaction. The result indicates that the fraction of reactant converted is 0.5, leaving an exit concentration of 500 mol/m^3. This value represents the equilibrium between the input stream and the reaction kinetics within the defined volume.

<a id="chemeng_cstr_volume_first_order"></a>

#### CSTR volume for a conversion (first order), `chemeng_cstr_volume_first_order`

Volume and space time of a CSTR that reaches conversion X for a first-order liquid-phase reaction.

$$
V = \frac{v_0\,X}{k(1-X)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `k` | in | first-order rate constant | 1/s |
| `X` | in | conversion of A wanted | - |
| `V` | out | reactor volume | m^3 |
| `tau` | out | space time V/v0 | s |

**Applies when:** 0 <= X < 1; steady state, perfect mixing, constant density, -r_A = k C_A.

```shell
$ chemical-engineering solve chemeng_cstr_volume_first_order "v0=0.01 m^3/s" "k=0.001 1/s" X=0.9
V = 90 m^3
tau = 9000 s
  by: CSTR volume for a conversion (first order)  (design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k (1 - X)))
  v0 = 0.01 m^3/s
  k = 0.001 1/s
  X = 0.9
  assumes: 0 <= X < 1; steady state, perfect mixing, constant density, -r_A = k C_A
```

**What the worked example means:** This calculation determines the continuous stirred tank reactor volume needed to achieve a specific conversion for a first-order reaction. Using the volumetric feed flow of 0.01 m^3/s, the rate constant of 0.001 1/s, and the target conversion of 0.9, the resulting reactor volume is 90 m^3. The corresponding space time, which represents the average residence time of fluid elements in the reactor, is calculated as 9000 s. These values ensure the system operates at steady state with perfect mixing.

<a id="chemeng_cstr_volume_second_order"></a>

#### CSTR volume for a conversion (second order), `chemeng_cstr_volume_second_order`

Volume and space time of a CSTR that reaches conversion X when -r_A = k C_A^2 (liquid phase, constant density).

$$
V = \frac{v_0 X}{k C_{A0}(1 - X)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `X` | in | conversion of A wanted | - |
| `V` | out | reactor volume | m^3 |
| `tau` | out | space time V/v0 | s |

**Applies when:** 0 <= X < 1; steady state, perfect mixing, constant density, -r_A = k C_A^2.

```shell
$ chemical-engineering solve chemeng_cstr_volume_second_order "v0=0.01 m^3/s" "k=1e-05 m^3/(mol*s)" "CA0=1000 mol/m^3" X=0.8
V = 20 m^3
tau = 2000 s
  by: CSTR volume for a conversion (second order)  (design equation V = F_A0 X/(-r_A) at the exit concentration: V = v0 X/(k C_A0 (1 - X)^2))
  v0 = 0.01 m^3/s
  k = 1e-05 m^3/(mol*s)
  CA0 = 1000 mol/m^3
  X = 0.8
  assumes: 0 <= X < 1; steady state, perfect mixing, constant density, -r_A = k C_A^2
```

**What the worked example means:** This calculation determines the reactor volume and space time needed for a continuous stirred-tank reactor to achieve the target conversion of 0.8 using a volumetric feed flow of 0.01 m^3/s, a rate constant of 1e-05 m^3/(mol*s), and an initial concentration of 1000 mol/m^3. The resulting volume of 20 m^3 ensures that the reactant undergoes sufficient reaction time within the vessel, while the space time of 2000 s represents the average residence time of fluid elements in the reactor under steady-state conditions.

<a id="chemeng_cstrs_in_series_first_order"></a>

#### Equal CSTRs in series: conversion (first order), `chemeng_cstrs_in_series_first_order`

Conversion leaving N equal CSTRs in series, each of volume V, for a first-order liquid-phase reaction.

$$
X = 1 - (1 + k\tau_i)^{-N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `V` | in | volume of EACH tank | m^3 |
| `k` | in | first-order rate constant | 1/s |
| `N` | in | number of equal tanks in series | - |
| `X` | out | overall conversion of A | - |

**Applies when:** equal tanks; steady state, perfect mixing in each tank, constant density.

```shell
$ chemical-engineering solve chemeng_cstrs_in_series_first_order "v0=0.01 m^3/s" "V=5 m^3" "k=0.001 1/s" N=3
X = 0.7037037037
  by: Equal CSTRs in series: conversion (first order)  (each tank divides the concentration by (1 + k tau_i): X = 1 - (1 + k V/v0)^(-N))
  v0 = 0.01 m^3/s
  V = 5 m^3
  k = 0.001 1/s
  assumes: equal tanks; steady state, perfect mixing in each tank, constant density
```

**What the worked example means:** This calculation determines the fractional conversion of reactant A after passing through a series of identical continuous stirred-tank reactors. It uses the first-order reaction rate constant, the volumetric feed flow, the volume of each individual tank, and the total number of tanks in the series. The result, 0.703704, represents the proportion of reactant converted to product, showing how staging multiple smaller tanks improves efficiency over a single large reactor for this specific kinetic model.

<a id="chemeng_cstrs_in_series_second_order"></a>

#### Equal CSTRs in series: conversion (second order), `chemeng_cstrs_in_series_second_order`

Conversion leaving N equal CSTRs in series, each of volume V, when -r_A = k C_A^2.

$$
C_i = \frac{2C_{i-1}}{1 + \sqrt{1 + 4k\tau_i C_{i-1}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `V` | in | volume of EACH tank | m^3 |
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `N` | in | number of equal tanks in series | - |
| `X` | out | overall conversion of A | - |
| `CA` | out | exit concentration of A | mol/m^3 |

**Applies when:** equal tanks; steady state, perfect mixing, constant density.

```shell
$ chemical-engineering solve chemeng_cstrs_in_series_second_order "v0=0.01 m^3/s" "V=1 m^3" "k=1e-05 m^3/(mol*s)" "CA0=1000 mol/m^3" N=3
X = 0.6743587846
CA = 325.6412154 mol/m^3
  by: Equal CSTRs in series: conversion (second order)  (tank by tank: C_i = 2 C_(i-1)/(1 + sqrt(1 + 4 k tau_i C_(i-1))))
  v0 = 0.01 m^3/s
  V = 1 m^3
  k = 1e-05 m^3/(mol*s)
  CA0 = 1000 mol/m^3
  assumes: equal tanks; steady state, perfect mixing, constant density
```

**What the worked example means:** This calculation determines the final state of a cascade of equal-sized continuous stirred-tank reactors operating at steady state. By applying a specific recurrence relation for second-order kinetics, it computes the concentration leaving each stage sequentially. The result shows that with an initial feed concentration of 1000 mol/m^3, passing through 3 tanks yields an exit concentration of 325.641 mol/m^3, corresponding to an overall conversion of 0.674359.

<a id="chemeng_pfr_gas_volume_first_order"></a>

#### Gas-phase PFR volume with volume change (first order), `chemeng_pfr_gas_volume_first_order`

Volume of an isothermal, isobaric gas-phase plug-flow reactor for conversion X of a first-order reaction whose mole number changes (factor eps).

$$
V = \frac{F_{A0}}{kC_{A0}}\left[(1+\varepsilon)\ln\frac{1}{1-X} - \varepsilon X\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `FA0` | in | molar feed flow of A | mol/s |
| `CA0` | in | concentration of A at the inlet | mol/m^3 |
| `k` | in | first-order rate constant | 1/s |
| `eps` | in | volume-change factor eps = y_A0 * (change in moles per mole of A); e.g. A -> 2B with pure A: eps = 1 | - |
| `X` | in | conversion of A wanted | - |
| `V` | out | reactor volume | m^3 |

**Applies when:** isothermal, isobaric ideal gas; eps > -1; 0 <= X < 1.

```shell
$ chemical-engineering solve chemeng_pfr_gas_volume_first_order "FA0=10 mol/s" "CA0=40 mol/m^3" "k=0.05 1/s" eps=1 X=0.8
V = 12.09437912 m^3
  by: Gas-phase PFR volume with volume change (first order)  (V = F_A0 * integral of (1 + eps X)/(k C_A0 (1 - X)) dX = (F_A0/(k C_A0)) [(1 + eps) ln(1/(1 - X)) - eps X])
  FA0 = 10 mol/s
  CA0 = 40 mol/m^3
  k = 0.05 1/s
  eps = 1
  X = 0.8
  assumes: isothermal, isobaric ideal gas; eps > -1; 0 <= X < 1
```

**What the worked example means:** This calculation determines the volume of an isothermal, isobaric gas-phase plug-flow reactor needed to achieve a specific conversion for a first-order reaction where the total number of moles changes. Using the molar feed flow of A (10 mol/s), the inlet concentration of A (40 mol/m^3), the rate constant (0.05 1/s), and the volume-change factor (1) for a target conversion (0.8), the model computes the necessary size. The resulting volume is 12.0944 m^3, which represents the physical space required within the reactor to facilitate the chemical transformation under these conditions.

<a id="chemeng_pfr_gas_volume_second_order"></a>

#### Gas-phase PFR volume with volume change (second order), `chemeng_pfr_gas_volume_second_order`

Volume of an isothermal, isobaric gas-phase plug-flow reactor for conversion X when -r_A = k C_A^2 and the mole number changes (factor eps).

$$
V = \frac{F_{A0}}{kC_{A0}^2}\left[2\varepsilon(1+\varepsilon)\ln(1-X) + \varepsilon^2X + \frac{(1+\varepsilon)^2X}{1-X}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `FA0` | in | molar feed flow of A | mol/s |
| `CA0` | in | concentration of A at the inlet | mol/m^3 |
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `eps` | in | volume-change factor eps = y_A0 * (change in moles per mole of A); e.g. A -> 2B with pure A: eps = 1 | - |
| `X` | in | conversion of A wanted | - |
| `V` | out | reactor volume | m^3 |

**Applies when:** isothermal, isobaric ideal gas; eps > -1; 0 <= X < 1.

```shell
$ chemical-engineering solve chemeng_pfr_gas_volume_second_order "FA0=10 mol/s" "CA0=40 mol/m^3" "k=0.001 m^3/(mol*s)" eps=1 X=0.8
V = 64.76405219 m^3
  by: Gas-phase PFR volume with volume change (second order)  (V = (F_A0/(k C_A0^2)) [2 eps (1 + eps) ln(1 - X) + eps^2 X + (1 + eps)^2 X/(1 - X)])
  FA0 = 10 mol/s
  CA0 = 40 mol/m^3
  k = 0.001 m^3/(mol*s)
  eps = 1
  X = 0.8
  assumes: isothermal, isobaric ideal gas; eps > -1; 0 <= X < 1
```

**What the worked example means:** This calculation determines the plug-flow reactor volume needed to achieve a specific conversion in an isothermal, isobaric gas-phase reaction where the rate follows second-order kinetics and the total number of moles changes. Using a feed flow rate of 10 mol/s, an inlet concentration of 40 mol/m^3, and a rate constant of 0.001 m^3/(mol*s), with a volume-change factor of 1, the resulting reactor volume is 64.7641 m^3. This size ensures that the reactant concentration profile accounts for expansion or contraction effects during the reaction.

<a id="chemeng_pfr_conversion_first_order"></a>

#### PFR conversion for a volume (first order, liquid), `chemeng_pfr_conversion_first_order`

Conversion at the exit of an isothermal plug-flow reactor of volume V for a first-order constant-density reaction.

$$
X = 1 - e^{-k\tau}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `V` | in | reactor volume | m^3 |
| `k` | in | first-order rate constant | 1/s |
| `X` | out | conversion of A | - |

**Applies when:** plug flow, isothermal, constant density.

```shell
$ chemical-engineering solve chemeng_pfr_conversion_first_order "v0=0.01 m^3/s" "V=20 m^3" "k=0.001 1/s"
X = 0.8646647168
  by: PFR conversion for a volume (first order, liquid)  (X = 1 - exp(-k tau), tau = V/v0)
  v0 = 0.01 m^3/s
  V = 20 m^3
  k = 0.001 1/s
  assumes: plug flow, isothermal, constant density
```

**What the worked example means:** This calculation determines the fraction of reactant converted at the outlet of an isothermal plug-flow reactor. Using a first-order rate constant of 0.001 1/s and a volumetric feed flow of 0.01 m^3/s, the residence time derived from the reactor volume 20 m^3 yields an exit conversion of 0.864665. This result indicates how effectively the specified vessel size processes the input stream under ideal steady-state conditions.

<a id="chemeng_pfr_conversion_second_order"></a>

#### PFR conversion for a volume (second order, liquid), `chemeng_pfr_conversion_second_order`

Exit conversion and concentration of an isothermal plug-flow reactor of volume V when -r_A = k C_A^2 (constant density).

$$
X = \frac{k\tau C_{A0}}{1 + k\tau C_{A0}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `V` | in | reactor volume | m^3 |
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `X` | out | conversion of A | - |
| `CA` | out | exit concentration of A | mol/m^3 |

**Applies when:** plug flow, isothermal, constant density.

```shell
$ chemical-engineering solve chemeng_pfr_conversion_second_order "v0=0.01 m^3/s" "V=2 m^3" "k=1e-05 m^3/(mol*s)" "CA0=1000 mol/m^3"
X = 0.6666666667
CA = 333.3333333 mol/m^3
  by: PFR conversion for a volume (second order, liquid)  (1/C_A = 1/C_A0 + k tau)
  v0 = 0.01 m^3/s
  V = 2 m^3
  k = 1e-05 m^3/(mol*s)
  CA0 = 1000 mol/m^3
  assumes: plug flow, isothermal, constant density
```

**What the worked example means:** This calculation determines the performance of an isothermal plug-flow reactor handling a second-order liquid reaction. Given the feed flow rate 0.01 m^3/s, reactor volume 2 m^3, rate constant 1e-05 m^3/(mol*s), and initial concentration 1000 mol/m^3, the model computes the final conversion 0.666667 and remaining reactant concentration 333.333 mol/m^3. These values indicate how effectively the reactant is consumed within the specified vessel size under ideal flow conditions.

<a id="chemeng_pfr_volume_first_order"></a>

#### PFR volume for a conversion (first order, liquid), `chemeng_pfr_volume_first_order`

Volume of an isothermal plug-flow reactor that reaches conversion X for a first-order constant-density reaction.

$$
V = \frac{v_0}{k}\ln\frac{1}{1-X}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `k` | in | first-order rate constant | 1/s |
| `X` | in | conversion of A wanted | - |
| `V` | out | reactor volume | m^3 |

**Applies when:** 0 <= X < 1; plug flow, isothermal, constant density.

```shell
$ chemical-engineering solve chemeng_pfr_volume_first_order "v0=0.01 m^3/s" "k=0.001 1/s" X=0.9
V = 23.02585093 m^3
  by: PFR volume for a conversion (first order, liquid)  (V = F_A0 * integral of dX/(-r_A) = (v0/k) ln(1/(1 - X)))
  v0 = 0.01 m^3/s
  k = 0.001 1/s
  X = 0.9
  assumes: 0 <= X < 1; plug flow, isothermal, constant density
```

**What the worked example means:** This calculation determines the volume of an isothermal plug-flow reactor needed to achieve a specific conversion for a first-order liquid-phase reaction. Using the volumetric feed flow 0.01 m^3/s, the rate constant 0.001 1/s, and the target conversion 0.9, the resulting reactor volume is 23.0259 m^3. This value represents the physical size of the vessel required to ensure the reactant A reaches the desired level of transformation under steady-state conditions.

<a id="chemeng_pfr_volume_second_order"></a>

#### PFR volume for a conversion (second order, liquid), `chemeng_pfr_volume_second_order`

Volume of an isothermal plug-flow reactor that reaches conversion X when -r_A = k C_A^2 (constant density).

$$
V = \frac{v_0}{k C_{A0}}\,\frac{X}{1-X}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | volumetric feed flow | m^3/s |
| `k` | in | second-order rate constant (-r_A = k C_A^2) | m^3/(mol*s) |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `X` | in | conversion of A wanted | - |
| `V` | out | reactor volume | m^3 |

**Applies when:** 0 <= X < 1; plug flow, isothermal, constant density.

```shell
$ chemical-engineering solve chemeng_pfr_volume_second_order "v0=0.01 m^3/s" "k=1e-05 m^3/(mol*s)" "CA0=1000 mol/m^3" X=0.8
V = 4 m^3
  by: PFR volume for a conversion (second order, liquid)  (V = (v0/(k C_A0)) X/(1 - X))
  v0 = 0.01 m^3/s
  k = 1e-05 m^3/(mol*s)
  CA0 = 1000 mol/m^3
  X = 0.8
  assumes: 0 <= X < 1; plug flow, isothermal, constant density
```

**What the worked example means:** This calculation determines the plug-flow reactor volume needed to achieve a specific conversion for a second-order liquid reaction. Using the feed flow rate 0.01 m^3/s, the rate constant 1e-05 m^3/(mol*s), the initial concentration 1000 mol/m^3, and the target conversion 0.8, the model computes the volume as 4 m^3. This result indicates the physical size of the vessel required to ensure the reactant spends enough time in the system to reach the desired level of transformation under constant density conditions.

<a id="chemeng_recycle_reactor_first_order"></a>

#### Plug-flow reactor with recycle: conversion (first order), `chemeng_recycle_reactor_first_order`

Overall conversion of a plug-flow reactor with recycle ratio R (recycled flow / product flow) for a first-order constant-density reaction.

$$
\frac{k\tau}{R+1} = \ln\frac{1 + R(1-X)}{(R+1)(1-X)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v0` | in | fresh volumetric feed flow | m^3/s |
| `V` | in | reactor volume | m^3 |
| `k` | in | first-order rate constant | 1/s |
| `R` | in | recycle ratio: recycled flow divided by the flow leaving the system | - |
| `X` | out | overall conversion of A | - |

**Applies when:** R >= 0 (R = 0 is a plain PFR; large R approaches a CSTR); isothermal, constant density.

```shell
$ chemical-engineering solve chemeng_recycle_reactor_first_order "v0=0.01 m^3/s" "V=20 m^3" "k=0.001 1/s" R=2
X = 0.7398003103
  by: Plug-flow reactor with recycle: conversion (first order)  (k tau/(R + 1) = ln[(1 + R(1 - X))/((R + 1)(1 - X))], tau = V/v0, solved for X)
  v0 = 0.01 m^3/s
  V = 20 m^3
  k = 0.001 1/s
  R = 2
  assumes: R >= 0 (R = 0 is a plain PFR; large R approaches a CSTR); isothermal, constant density
```

**What the worked example means:** This calculation determines the overall conversion of reactant A in a plug-flow reactor that includes a recycle stream. Using the fresh volumetric feed flow 0.01 m^3/s, reactor volume 20 m^3, first-order rate constant 0.001 1/s, and recycle ratio 2, the method solves the implicit equation relating these parameters to find the final conversion 0.7398. The result indicates how effectively the reaction proceeds given the specific operating conditions and the extent of recycling unreacted material back into the system.

<a id="chemeng_rtd_moments"></a>

#### Residence-time distribution: mean, variance and tanks-in-series number from tracer data, `chemeng_rtd_moments`

Mean residence time, variance and the equivalent number of tanks in series from a pulse-tracer response C(t) (trapezoidal rule, uneven spacing allowed).

$$
\bar t = \frac{\int t\,C\,dt}{\int C\,dt},\quad \sigma^2 = \frac{\int (t-\bar t)^2 C\,dt}{\int C\,dt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | sample times, in the unit t_unit | - |
| `C` | in | tracer concentrations at those times (any unit) | - |
| `t_unit` | in | unit of the times: s, min or h | - |
| `t_mean` | out | mean residence time | s |
| `variance` | out | variance of the RTD | s^2 |
| `N_tanks` | out | equivalent number of equal tanks in series, t_mean^2/variance | - |

**Applies when:** pulse tracer, closed vessel; times increasing, concentrations >= 0.

```shell
$ chemical-engineering solve chemeng_rtd_moments "t=[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14]" "C=[0, 1, 5, 8, 10, 8, 6, 4, 3, 2.2, 1.5, 0.6, 0]" t_unit=min
t_mean = 307.6406713 s
variance = 21424.34472 s^2
N_tanks = 4.417534531
  by: Residence-time distribution: mean, variance and tanks-in-series number from tracer data  (E(t) = C/integral C dt; t_mean = integral t E dt; variance = integral (t - t_mean)^2 E dt; trapezoidal rule)
  t_unit = min
  assumes: pulse tracer, closed vessel; times increasing, concentrations >= 0
```

**What the worked example means:** This calculation determines the statistical properties of the residence time distribution from pulse tracer data. The mean residence time, reported as 307.641 s, represents the average duration fluid elements spend within the vessel. The variance, given as 21424.3 s^2, quantifies the spread or dispersion of these times around the mean. From these moments, the equivalent number of ideal tanks in series is derived as 4.41753, providing a simple model to approximate the actual flow pattern observed in the reactor.

<a id="chemeng_reversible_first_order_batch"></a>

#### Reversible first-order reaction A <-> B in a batch reactor, `chemeng_reversible_first_order_batch`

Concentrations and conversion after time t, and the equilibrium conversion, for A <-> B with first-order forward and reverse steps.

$$
C_A = C_{Ae} + (C_{A0} - C_{Ae})e^{-(k_1+k_2)t},\ C_{Ae} = \frac{k_2(C_{A0}+C_{B0})}{k_1+k_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k1` | in | forward rate constant (A -> B) | 1/s |
| `k2` | in | reverse rate constant (B -> A) | 1/s |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `CB0` | in | initial concentration of B | mol/m^3 |
| `t` | in | reaction time | s |
| `CA` | out | concentration of A at t | mol/m^3 |
| `CB` | out | concentration of B at t | mol/m^3 |
| `X` | out | conversion of A at t | - |
| `X_eq` | out | equilibrium conversion of A | - |

**Applies when:** isothermal, constant volume.

```shell
$ chemical-engineering solve chemeng_reversible_first_order_batch "k1=0.002 1/s" "k2=0.001 1/s" "CA0=1000 mol/m^3" "CB0=0 mol/m^3" "t=300 s"
CA = 604.3797732 mol/m^3
CB = 395.6202268 mol/m^3
X = 0.3956202268
X_eq = 0.6666666667
  by: Reversible first-order reaction A <-> B in a batch reactor  (C_A = C_Ae + (C_A0 - C_Ae) exp(-(k1 + k2) t), C_Ae = k2 (C_A0 + C_B0)/(k1 + k2))
  k1 = 0.002 1/s
  k2 = 0.001 1/s
  CA0 = 1000 mol/m^3
  CB0 = 0 mol/m^3
  t = 300 s
  assumes: isothermal, constant volume
```

**What the worked example means:** This calculation determines the concentration of species A at time t, denoted as 604.38 mol/m^3, using the initial concentrations 1000 mol/m^3 for A and 0 mol/m^3 for B, along with rate constants 0.002 1/s for the forward reaction and 0.001 1/s for the reverse. The result shows how far the system has progressed toward equilibrium, where the final concentration of A would be determined by the ratio of the rate constants. The corresponding concentration of B is 395.62 mol/m^3, and the fractional conversion of A is 0.39562. This model assumes constant volume and temperature, meaning these values reflect kinetic progress rather than physical changes in the reactor size or thermal conditions.

<a id="chemeng_series_reaction_maximum"></a>

#### Series reactions A -> B -> C: maximum of the intermediate (batch or PFR), `chemeng_series_reaction_maximum`

Time (or plug-flow space time) at which the intermediate B of first-order series reactions A -> B -> C peaks, starting from pure A, and the concentrations there.

$$
t_{max} = \frac{\ln(k_2/k_1)}{k_2 - k_1},\quad C_{B,max} = C_{A0}\left(\frac{k_1}{k_2}\right)^{k_2/(k_2-k_1)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k1` | in | rate constant of A -> B | 1/s |
| `k2` | in | rate constant of B -> C | 1/s |
| `CA0` | in | feed (initial) concentration of A | mol/m^3 |
| `t_max` | out | time (or space time) of the maximum of B | s |
| `CB_max` | out | maximum concentration of B | mol/m^3 |
| `CA` | out | concentration of A at the maximum | mol/m^3 |
| `CC` | out | concentration of C at the maximum | mol/m^3 |

**Applies when:** starting from pure A (no B or C); isothermal, constant volume, first-order steps.

```shell
$ chemical-engineering solve chemeng_series_reaction_maximum "k1=0.002 1/s" "k2=0.001 1/s" "CA0=1000 mol/m^3"
t_max = 693.1471806 s
CB_max = 500 mol/m^3
CA = 250 mol/m^3
CC = 250 mol/m^3
  by: Series reactions A -> B -> C: maximum of the intermediate (batch or PFR)  (t_max = ln(k2/k1)/(k2 - k1), C_B,max = C_A0 (k1/k2)^(k2/(k2 - k1)) (k1 = k2: t = 1/k, C_B = C_A0/e))
  k1 = 0.002 1/s
  k2 = 0.001 1/s
  CA0 = 1000 mol/m^3
  assumes: starting from pure A (no B or C); isothermal, constant volume, first-order steps
```

**What the worked example means:** This calculation determines the optimal time to stop a batch or plug-flow reactor for first-order series reactions starting from pure A. Using the rate constants 0.002 1/s and 0.001 1/s, it finds that the intermediate B reaches its maximum concentration of 500 mol/m^3 at 693.147 s. At this moment, the remaining reactant A is 250 mol/m^3 and the final product C has accumulated to 250 mol/m^3.

### Reservoir engineering (6)

<a id="chemeng_gas_pz_material_balance"></a>

#### Gas initially in place from the p/z plot: `chemeng_gas_pz_material_balance`

Original gas in place of a volumetric gas reservoir from the straight line of p/z against cumulative gas production.

$$
\frac{p}{z} = \frac{p_i}{z_i}\left(1 - \frac{G_p}{G}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | average reservoir pressures, in p_unit | - |
| `z` | in | gas deviation factors at those pressures | - |
| `Gp` | in | cumulative gas production at standard conditions, in gp_unit | - |
| `p_unit` | in | unit of the pressures | - |
| `gp_unit` | in | unit of Gp: m3, MMscf or Bscf | - |
| `G` | out | original gas in place at standard conditions | m^3 |
| `pz_initial` | out | p/z extrapolated to no production | Pa |

**Applies when:** volumetric (no water drive) gas reservoir; p/z falls with production.

```shell
$ chemical-engineering solve chemeng_gas_pz_material_balance "p=[3200, 2925, 2525, 2125]" "z=[0.85, 0.84, 0.83, 0.82]" "Gp=[0, 15, 35, 55]" p_unit=psi gp_unit=Bscf
G = 4993507777 m^3
pz_initial = 26084853.14 Pa
  by: Gas initially in place from the p/z plot  (least squares of p/z = (p/z)_i (1 - Gp/G): G = -intercept/slope)
  p_unit = psi
  gp_unit = Bscf
  assumes: volumetric (no water drive) gas reservoir; p/z falls with production
```

**What the worked example means:** This calculation determines the original gas in place for a volumetric reservoir by analyzing the linear relationship between the ratio of average reservoir pressure to the gas deviation factor and cumulative gas production. By performing a least squares regression on the data points, the method identifies the y-intercept, which represents the initial p/z value at zero production, and the slope of the line. The result, 4.99351e+09 m^3, quantifies the total volume of gas originally present in the formation under standard conditions, providing a critical estimate for resource evaluation.

<a id="chemeng_havlena_odeh_gas_cap"></a>

#### Oil in place and gas-cap size from the material balance (Havlena-Odeh, gas cap), `chemeng_havlena_odeh_gas_cap`

Oil initially in place and the gas-cap ratio m from the straight line F/Eo = N + N m Eg/Eo of successive surveys (no water influx).

$$
\frac{F}{E_o} = N + Nm\,\frac{E_g}{E_o}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | underground withdrawals, in v_unit at reservoir conditions | - |
| `Eo` | in | oil and dissolved-gas expansion per unit of oil in place | - |
| `Eg` | in | gas-cap expansion term per unit of oil in place | - |
| `v_unit` | in | unit of F: m3 or bbl | - |
| `N` | out | oil initially in place (stock-tank volume) | m^3 |
| `m` | out | ratio of initial gas-cap to oil-zone reservoir volume | - |

**Applies when:** no water influx; consistent units.

```shell
$ chemical-engineering solve chemeng_havlena_odeh_gas_cap "F=[5000000.0, 9300000.0, 14300000.0, 19500000.0]" "Eo=[0.01, 0.018, 0.027, 0.036]" "Eg=[0.025, 0.05, 0.08, 0.11]" v_unit=bbl
N = 50598337.51 m^3
m = 0.2266832918
  by: Oil in place and gas-cap size from the material balance (Havlena-Odeh, gas cap)  (least squares of F/Eo against Eg/Eo: intercept N, slope N m)
  v_unit = bbl
  assumes: no water influx; consistent units
```

**What the worked example means:** This calculation determines the original oil in place, reported as 5.05983e+07 m^3, and the gas-cap size relative to the oil zone, given by 0.226683. These values are derived from linear regression of underground withdrawals against expansion terms, specifically fitting the relationship between F/Eo and Eg/Eo. The intercept of this fit provides the stock-tank volume estimate, while the slope indicates how much larger the initial gas cap is compared to the oil reservoir volume.

<a id="chemeng_havlena_odeh_oil_in_place"></a>

#### Oil initially in place from the material balance (Havlena-Odeh straight line), `chemeng_havlena_odeh_oil_in_place`

Stock-tank oil initially in place from the underground withdrawal F and the total expansion term Et of successive pressure surveys (no water influx), by the straight line F = N Et through the origin.

$$
F = N\,E_t,\quad N = \frac{\sum F E_t}{\sum E_t^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | underground withdrawals at each survey, in v_unit at reservoir conditions | - |
| `Et` | in | total expansion per unit of oil in place at each survey (reservoir volume per stock-tank volume) | - |
| `v_unit` | in | unit of F: m3 or bbl | - |
| `N` | out | oil initially in place (stock-tank volume) | m^3 |

**Applies when:** volumetric reservoir (no water influx); F and Et on one consistent basis.

```shell
$ chemical-engineering solve chemeng_havlena_odeh_oil_in_place "F=[580700, 1600000.0, 2800000.0, 4100000.0]" "Et=[0.0101, 0.0278, 0.0485, 0.0712]" v_unit=bbl
N = 9161201.209 m^3
  by: Oil initially in place from the material balance (Havlena-Odeh straight line)  (least squares through the origin: N = sum F Et / sum Et^2)
  v_unit = bbl
  assumes: volumetric reservoir (no water influx); F and Et on one consistent basis
```

**What the worked example means:** This calculation determines the stock-tank oil initially in place, denoted as N, by applying a least-squares regression through the origin to data pairs of underground withdrawal F and total expansion Et. The method assumes a volumetric reservoir with no water influx, meaning all production is balanced by fluid expansion. The resulting value 9.1612e+06 m^3 represents the original volume of oil at surface conditions that corresponds to the observed pressure decline and withdrawal history across successive surveys.

<a id="chemeng_horner_buildup"></a>

#### Pressure build-up test: Horner analysis for permeability and extrapolated pressure, `chemeng_horner_buildup`

Permeability and the extrapolated pressure p* from the semilog straight line of shut-in pressure against the Horner time ratio (tp + dt)/dt.

$$
p_{ws} = p^* - m\log_{10}\frac{t_p + \Delta t}{\Delta t},\quad k = \frac{q\mu\ln 10}{4\pi mh}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dt` | in | shut-in times, in t_unit | - |
| `pws` | in | shut-in bottom-hole pressures, in p_unit | - |
| `t_unit` | in | unit of the shut-in times | - |
| `p_unit` | in | unit of the pressures | - |
| `tp` | in | producing time before shut-in | s |
| `q` | in | flow rate before shut-in at reservoir conditions | m^3/s |
| `mu` | in | fluid viscosity | Pa*s |
| `h` | in | net pay thickness | m |
| `m` | out | semilog slope, pressure change per log cycle | Pa |
| `k` | out | permeability | m^2 |
| `p_star` | out | extrapolated pressure p* at (tp + dt)/dt = 1 | Pa |

**Applies when:** infinite-acting radial flow on the straight line; the points given lie on the semilog straight line.

```shell
$ chemical-engineering solve chemeng_horner_buildup "dt=[2, 4, 8, 16, 24]" "pws=[2960, 3015, 3069, 3122, 3150]" t_unit=h p_unit=psi "tp=1800000 s" "q=0.0005520392185 m^3/s" "mu=0.0008 Pa*s" "h=6.096 m"
m = 1239285.687 Pa
k = 1.071147681e-14 m^2
p_star = 23388029.28 Pa
  by: Pressure build-up test: Horner analysis for permeability and extrapolated pressure  (least squares of pws against log10((tp + dt)/dt): slope -m; k = q mu ln 10/(4 pi m h))
  tp = 1800000 s
  q = 0.0005520392185 m^3/s
  mu = 0.0008 Pa*s
  h = 6.096 m
  t_unit = h
  p_unit = psi
  assumes: infinite-acting radial flow on the straight line; the points given lie on the semilog straight line
```

**What the worked example means:** This calculation determines reservoir permeability and initial pressure using shut-in bottom-hole data. Performing a least-squares fit on pressure against the logarithm of the Horner time ratio gives the semilog slope 1.23929e+06 Pa. This slope, combined with flow rate 0.000552039 m^3/s, viscosity 0.0008 Pa*s, and thickness 6.096 m, gives the permeability 1.07115e-14 m^2. The intercept of this line provides the extrapolated pressure 2.3388e+07 Pa, which represents the estimated average reservoir pressure.

<a id="chemeng_radial_darcy_flow"></a>

#### Steady radial flow of oil to a well (Darcy) and productivity index, `chemeng_radial_darcy_flow`

Steady-state reservoir flow rate into a vertical well from a circular drainage area, with an optional skin factor, and the productivity index.

$$
q = \frac{2\pi kh\,(p_e - p_w)}{\mu\left(\ln\frac{r_e}{r_w} + S\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | permeability (md is converted here) | m^2 |
| `h` | in | net pay thickness | m |
| `mu` | in | fluid viscosity (cP is converted here) | Pa*s |
| `pe` | in | pressure at the drainage radius | Pa |
| `pw` | in | flowing bottom-hole pressure | Pa |
| `re` | in | drainage radius | m |
| `rw` | in | wellbore radius | m |
| `S` | in | skin factor (default 0: undamaged well) (optional) | - |
| `q` | out | flow rate at reservoir conditions | m^3/s |
| `PI` | out | productivity index q/(pe - pw) | m^3/(s*Pa) |

**Applies when:** steady state, single-phase incompressible flow; re > rw, pe > pw.

```shell
$ chemical-engineering solve chemeng_radial_darcy_flow "k=9.869233e-14 m^2" "h=9.144 m" "mu=0.0015 Pa*s" "pe=20684271.88 Pa" "pw=17236893.23 Pa" "re=304.8 m" "rw=0.09144 m"
q = 0.001606511477 m^3/s
PI = 4.660095801e-10 m^3/(s*Pa)
  by: Steady radial flow of oil to a well (Darcy) and productivity index  (q = 2 pi k h (pe - pw)/(mu (ln(re/rw) + S)))
  k = 9.869233e-14 m^2
  h = 9.144 m
  mu = 0.0015 Pa*s
  pe = 20684271.88 Pa
  pw = 17236893.23 Pa
  re = 304.8 m
  rw = 0.09144 m
  assumes: steady state, single-phase incompressible flow; re > rw, pe > pw
```

In the worked example, k = 9.869233e-14 m^2, h = 9.144 m, mu = 0.0015 Pa*s, pe = 20684271.88 Pa, pw = 17236893.23 Pa, re = 304.8 m, rw = 0.09144 m go through the equation above and give q = 0.001606511475 m^3/s, PI = 4.660095801e-10 m^3/(s*Pa). It applies only when steady state, single-phase incompressible flow; re > rw, pe > pw: outside that it says what is wrong instead of returning a number.

<a id="chemeng_line_source_drawdown"></a>

#### Transient drawdown around a well (line-source solution), `chemeng_line_source_drawdown`

Pressure drop and pressure at radius r and time t after a well starts producing at a constant rate in an infinite reservoir (line-source, exponential-integral solution).

$$
\Delta p = \frac{q\mu}{4\pi kh}\,E_1\!\left(\frac{\phi\mu c_t r^2}{4kt}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `q` | in | flow rate at reservoir conditions | m^3/s |
| `mu` | in | fluid viscosity | Pa*s |
| `k` | in | permeability | m^2 |
| `h` | in | net pay thickness | m |
| `phi` | in | porosity (fraction) | - |
| `ct` | in | total compressibility (1/psi is converted here) | 1/Pa |
| `r` | in | radius from the well | m |
| `t` | in | time since production started | s |
| `pi` | in | initial reservoir pressure | Pa |
| `dp` | out | pressure drop at r and t | Pa |
| `p` | out | pressure at r and t | Pa |

**Applies when:** infinite-acting, homogeneous, slightly compressible fluid; constant rate from t = 0.

```shell
$ chemical-engineering solve chemeng_line_source_drawdown "q=0.001 m^3/s" "mu=0.001 Pa*s" "k=1e-13 m^2" "h=10 m" phi=0.2 "ct=1e-09 1/Pa" "r=0.1 m" "t=3600 s" "pi=30000000 Pa"
dp = 1027328.623 Pa
p = 28972671.38 Pa
  by: Transient drawdown around a well (line-source solution)  (dp = (q mu/(4 pi k h)) E1(phi mu ct r^2/(4 k t)); E1 by series or continued fraction)
  q = 0.001 m^3/s
  mu = 0.001 Pa*s
  k = 1e-13 m^2
  h = 10 m
  phi = 0.2
  ct = 1e-09 1/Pa
  r = 0.1 m
  t = 3600 s
  pi = 30000000 Pa
  assumes: infinite-acting, homogeneous, slightly compressible fluid; constant rate from t = 0
```

**What the worked example means:** This calculation determines the transient pressure response in a reservoir. Using the line-source solution, it computes the pressure drop at a specific radius and time based on flow rate, viscosity, permeability, thickness, porosity, compressibility, and initial pressure. The computed pressure drop of 1.02733e+06 Pa indicates how much the reservoir pressure has decreased from its initial state due to production, while the final pressure of 2.89727e+07 Pa represents the current fluid potential at that location.

### Separations (22)

<a id="chemeng_kremser_absorber"></a>

#### Absorber with N equilibrium stages (Kremser), `chemeng_kremser_absorber`

Solute mole fraction in the gas leaving a countercurrent absorber of N ideal stages, and the fraction of solute absorbed, for dilute gas with linear equilibrium y = m x.

$$
\frac{y_{in} - y_{out}}{y_{in} - m x_{in}} = \frac{A^{N+1} - A}{A^{N+1} - 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of ideal stages | - |
| `A` | in | absorption factor A = L/(m V) | - |
| `m` | in | slope of the equilibrium line y = m x | - |
| `y_in` | in | solute mole fraction in the entering gas | - |
| `x_in` | in | solute mole fraction in the entering liquid (solvent) | - |
| `y_out` | out | solute mole fraction in the leaving gas | - |
| `recovery` | out | fraction of the entering solute absorbed | - |

**Applies when:** dilute, constant L and V, linear equilibrium; y_in > m x_in.

```shell
$ chemical-engineering solve chemeng_kremser_absorber N=5 A=1.4 m=2 y_in=0.05 x_in=0
y_out = 0.003063004783
recovery = 0.9387399043
  by: Absorber with N equilibrium stages (Kremser)  (Kremser: (y_in - y_out)/(y_in - m x_in) = (A^(N+1) - A)/(A^(N+1) - 1); N/(N + 1) when A = 1)
  A = 1.4
  m = 2
  y_in = 0.05
  x_in = 0
  assumes: dilute, constant L and V, linear equilibrium; y_in > m x_in
```

**What the worked example means:** This calculation determines the performance of a counter-current absorber by applying the Kremser equation. Given an absorption factor of 1.4, equilibrium slope of 2, and inlet concentrations of 0.05 for gas and 0 for liquid, the model computes the outlet gas concentration as 0.003063. The resulting recovery of 0.93874 represents the fraction of incoming solute transferred to the liquid phase across the 5 theoretical stages.

<a id="chemeng_nrtl_activity_coefficients"></a>

#### Activity coefficients from the NRTL equation, `chemeng_nrtl_activity_coefficients`

Activity coefficients and excess Gibbs energy of a liquid mixture from NRTL interaction parameters tau_ij and non-randomness parameters alpha_ij.

$$
\ln\gamma_i = \frac{\sum_j x_j\tau_{ji}G_{ji}}{\sum_k x_k G_{ki}} + \sum_j \frac{x_jG_{ij}}{\sum_k x_kG_{kj}}\left(\tau_{ij} - \frac{\sum_m x_m\tau_{mj}G_{mj}}{\sum_k x_kG_{kj}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | liquid mole fractions | - |
| `tau` | in | NRTL parameters tau_ij (dimensionless, tau_ii = 0) at the temperature of interest | - |
| `alpha` | in | non-randomness parameters alpha_ij (symmetric, alpha_ii = 0) | - |
| `gamma1` | out | activity coefficient of component 1 | - |
| `gamma` | out | activity coefficients of every component | - |
| `GE_RT` | out | excess Gibbs energy over RT | - |

**Applies when:** tau_ii = 0, alpha_ii = 0; all mole fractions positive.

```shell
$ chemical-engineering solve chemeng_nrtl_activity_coefficients "x=[0.3, 0.7]" "tau=[[0, 0.8], [1.2, 0]]" "alpha=[[0, 0.3], [0.3, 0]]"
gamma1 = 2.253463531
gamma = [2.25346353086,1.18777953647]
GE_RT = 0.3642004538
  by: Activity coefficients from the NRTL equation  (G_ij = exp(-alpha_ij tau_ij); ln gamma_i = sum_j x_j tau_ji G_ji/sum_k x_k G_ki + sum_j (x_j G_ij/sum_k x_k G_kj)(tau_ij - sum_m x_m tau_mj G_mj/sum_k x_k G_kj))
  assumes: tau_ii = 0, alpha_ii = 0; all mole fractions positive
```

**What the worked example means:** This calculation determines the activity coefficients and excess Gibbs energy for a liquid mixture using the Non-Random Two-Liquid model. By applying the specified interaction parameters and non-randomness factors to the mole fractions, the method yields an activity coefficient of 2.25346 for the first component and a dimensionless excess Gibbs energy value of 0.3642. These results quantify how much the components deviate from ideal solution behavior due to molecular interactions, which is critical for predicting phase equilibria and separation processes.

<a id="chemeng_uniquac_activity_coefficients"></a>

#### Activity coefficients from the UNIQUAC equation, `chemeng_uniquac_activity_coefficients`

Activity coefficients of a liquid mixture from UNIQUAC volume and area parameters r, q and interaction parameters tau_ij (coordination number 10).

$$
\ln\gamma_i = \ln\frac{\Phi_i}{x_i} + \frac{z}{2}q_i\ln\frac{\theta_i}{\Phi_i} + l_i - \frac{\Phi_i}{x_i}\sum_j x_jl_j + q_i\Big[1 - \ln\sum_j\theta_j\tau_{ji} - \sum_j\frac{\theta_j\tau_{ij}}{\sum_k\theta_k\tau_{kj}}\Big]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | liquid mole fractions | - |
| `r` | in | UNIQUAC volume parameters r_i | - |
| `q` | in | UNIQUAC area parameters q_i | - |
| `tau` | in | UNIQUAC parameters tau_ij = exp(-a_ij/T) at the temperature of interest (tau_ii = 1) | - |
| `gamma1` | out | activity coefficient of component 1 | - |
| `gamma` | out | activity coefficients of every component | - |

**Applies when:** tau_ii = 1, tau_ij > 0; all mole fractions positive; coordination number z = 10.

```shell
$ chemical-engineering solve chemeng_uniquac_activity_coefficients "x=[0.3, 0.7]" "r=[2.1055, 0.92]" "q=[1.972, 1.4]" "tau=[[1, 0.9], [1.1, 1]]"
gamma1 = 1.298685685
gamma = [1.29868568479,1.10358464339]
  by: Activity coefficients from the UNIQUAC equation  (combinatorial part ln(Phi_i/x_i) + 5 q_i ln(theta_i/Phi_i) + l_i - (Phi_i/x_i) sum x_j l_j plus residual part q_i [1 - ln(sum_j theta_j tau_ji) - sum_j theta_j tau_ij/sum_k theta_k tau_kj])
  assumes: tau_ii = 1, tau_ij > 0; all mole fractions positive; coordination number z = 10
```

**What the worked example means:** This calculation determines the activity coefficients for components in a liquid mixture using the UNIQUAC model. It combines combinatorial terms that account for molecular size differences with residual terms that capture energetic interactions between molecules. The result, such as [1.29869, 1.10358], indicates how much each component deviates from ideal solution behavior based on the provided volume and area parameters.

<a id="chemeng_wilson_activity_coefficients"></a>

#### Activity coefficients from the Wilson equation, `chemeng_wilson_activity_coefficients`

Activity coefficients of every component and the excess Gibbs energy of a liquid mixture from Wilson parameters Lambda_ij.

$$
\ln\gamma_i = 1 - \ln\Big(\sum_j x_j\Lambda_{ij}\Big) - \sum_k \frac{x_k\Lambda_{ki}}{\sum_j x_j\Lambda_{kj}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | liquid mole fractions | - |
| `Lam` | in | Wilson parameters Lambda_ij (dimensionless, Lambda_ii = 1) at the temperature of interest | - |
| `gamma1` | out | activity coefficient of component 1 | - |
| `gamma` | out | activity coefficients of every component | - |
| `GE_RT` | out | excess Gibbs energy over RT | - |

**Applies when:** Lambda_ii = 1, Lambda_ij > 0; all mole fractions positive.

```shell
$ chemical-engineering solve chemeng_wilson_activity_coefficients "x=[0.3, 0.7]" "Lam=[[1, 0.5], [1.5, 1]]"
gamma1 = 1.057810257
gamma = [1.05781025716,1.02099043889]
GE_RT = 0.03140151517
  by: Activity coefficients from the Wilson equation  (ln gamma_i = 1 - ln(sum_j x_j Lambda_ij) - sum_k x_k Lambda_ki/(sum_j x_j Lambda_kj))
  assumes: Lambda_ii = 1, Lambda_ij > 0; all mole fractions positive
```

**What the worked example means:** This calculation determines the activity coefficients for all components in a liquid mixture using the Wilson equation, which accounts for local composition effects in non-ideal systems. Given the mole fractions [0.3, 0.7] and the Wilson parameters [1 0.5; 1.5 1], the model computes the activity coefficient for the first component as 1.05781 and provides the full set of coefficients [1.05781, 1.02099]. Additionally, it calculates the dimensionless excess Gibbs energy 0.0314015, which quantifies the deviation from ideal solution behavior. These values are critical for accurately predicting vapor-liquid equilibrium.

<a id="chemeng_antoine_boiling_temperature"></a>

#### Boiling (saturation) temperature from the Antoine equation, `chemeng_antoine_boiling_temperature`

Temperature at which a pure liquid boils at pressure P, from Antoine coefficients written in a stated convention.

$$
T^{sat} = \frac{B}{A - \log_{10} P} - C
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | Antoine coefficient A | - |
| `B` | in | Antoine coefficient B | - |
| `C` | in | Antoine coefficient C | - |
| `units` | in | units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K | - |
| `base` | in | logarithm of the Antoine equation: log10 or ln | - |
| `P` | in | pressure | Pa |
| `Tsat` | out | saturation temperature | K |

**Applies when:** the convention (units, base) is stated; P within the fitted range.

```shell
$ chemical-engineering solve chemeng_antoine_boiling_temperature A=8.07131 B=1730.63 C=233.426 units=mmHg_degC base=log10 "P=101325 Pa"
Tsat = 373.1468258 K
  by: Boiling (saturation) temperature from the Antoine equation  (T = B/(A - log P) - C in the stated units; returned in K)
  A = 8.07131
  B = 1730.63
  C = 233.426
  P = 101325 Pa
  units = mmHg_degC
  base = log10
  assumes: the convention (units, base) is stated; P within the fitted range
```

**What the worked example means:** This calculation determines the boiling point of a pure liquid by inverting the Antoine equation. Using coefficients A, B, and C with the specified units and logarithm base, the formula computes the temperature where vapor pressure equals the external pressure 101325 Pa. The result, 373.147 K, indicates the thermal condition required for phase change at this specific pressure.

<a id="chemeng_raoult_bubble_pressure"></a>

#### Bubble-point pressure (Raoult's law): `chemeng_raoult_bubble_pressure`

Pressure at which a liquid of composition x starts to boil at temperature T, and the composition of the first bubble, for an ideal solution (Raoult's law, Antoine vapour pressures).

$$
P_{bubble} = \sum_i x_i P_i^{sat}(T)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | liquid mole fractions | - |
| `ant` | in | Antoine coefficients, one row (A, B, C) per component, in the stated convention | - |
| `units` | in | units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K | - |
| `base` | in | logarithm of the Antoine equation: log10 or ln | - |
| `T` | in | temperature | K |
| `P` | out | bubble-point pressure | Pa |
| `y` | out | vapour mole fractions of the first bubble | - |

**Applies when:** ideal liquid and ideal gas (Raoult's law); fractions sum to 1.

```shell
$ chemical-engineering solve chemeng_raoult_bubble_pressure "ant=[[6.90565, 1211.03, 220.79], [6.95464, 1344.8, 219.482]]" units=mmHg_degC base=log10 "x=[0.4, 0.6]" "T=368.15 K"
P = 100907.7943 Pa
y = [0.621965676216,0.378034323784]
  by: Bubble-point pressure (Raoult's law)  (P = sum x_i Psat_i(T); y_i = x_i Psat_i/P)
  T = 368.15 K
  units = mmHg_degC
  base = log10
  assumes: ideal liquid and ideal gas (Raoult's law); fractions sum to 1
```

**What the worked example means:** This calculation determines the pressure at which a liquid mixture begins to boil at temperature 368.15 K, known as the bubble-point pressure, resulting in 100906 Pa. It also identifies the composition of the very first vapour bubble formed, given by [0.621961, 0.378039]. The method applies Raoult's law, summing the products of each component's liquid mole fraction [0.4, 0.6] and its saturation pressure derived from Antoine coefficients [6.90565 1211.03 220.79; 6.95464 1344.8 219.482] and base log10 under units mmHg_degC. This ideal solution approach assumes the vapour behaves as an ideal gas.

<a id="chemeng_modified_raoult_bubble_pressure"></a>

#### Bubble-point pressure by the modified Raoult's law (activity coefficients), `chemeng_modified_raoult_bubble_pressure`

The bubble-point pressure P = sum x_i gamma_i P_i^sat of a non-ideal liquid with stated activity coefficients and vapour pressures (low pressure, ideal vapour), and the vapour composition y_i = x_i gamma_i P_i^sat/P.

$$
P = \sum_i x_i\gamma_i P_i^{sat},\quad y_i = \frac{x_i\gamma_i P_i^{sat}}{P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | liquid mole fractions (sum to 1) | - |
| `gamma` | in | activity coefficient of each component at this composition | - |
| `Psat` | in | vapour pressure of each pure component at the temperature, in the unit p_unit | - |
| `p_unit` | in | unit of the vapour pressures: Pa, kPa, MPa, bar, atm or psi | - |
| `P` | out | bubble-point pressure | Pa |
| `y1` | out | vapour mole fraction of the first component | - |
| `y` | out | vapour mole fractions, in the order of x | - |

**Applies when:** low pressure (ideal vapour, Poynting factor 1); activity coefficients stated for this liquid composition and temperature.

```shell
$ chemical-engineering solve chemeng_modified_raoult_bubble_pressure "x=[0.4, 0.6]" "gamma=[1.8, 1.2]" "Psat=[55, 30]" p_unit=kPa
P = 61200 Pa
y1 = 0.6470588235
y = [0.647058823529,0.352941176471]
  by: Bubble-point pressure by the modified Raoult's law (activity coefficients)  (P = sum x_i gamma_i P_i^sat, y_i = x_i gamma_i P_i^sat/P)
  p_unit = kPa
  assumes: low pressure (ideal vapour, Poynting factor 1); activity coefficients stated for this liquid composition and temperature
```

**What the worked example means:** This calculation determines the bubble-point pressure 61200 Pa for a non-ideal liquid mixture using the modified Raoult's law. It sums the product of each component's liquid mole fraction, its activity coefficient, and its pure-component vapour pressure. The resulting total pressure allows the determination of the vapour composition [0.647059, 0.352941], where each component's share is proportional to its contribution to the total pressure.

<a id="chemeng_raoult_bubble_temperature"></a>

#### Bubble-point temperature (Raoult's law): `chemeng_raoult_bubble_temperature`

Temperature at which a liquid of composition x starts to boil at pressure P, and the first-bubble composition (Raoult's law, Antoine vapour pressures; iterative).

$$
\sum_i x_i P_i^{sat}(T_{bubble}) = P
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | liquid mole fractions | - |
| `ant` | in | Antoine coefficients, one row (A, B, C) per component, in the stated convention | - |
| `units` | in | units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K | - |
| `base` | in | logarithm of the Antoine equation: log10 or ln | - |
| `P` | in | pressure | Pa |
| `T` | out | bubble-point temperature | K |
| `y` | out | vapour mole fractions of the first bubble | - |

**Applies when:** ideal liquid and ideal gas; fractions sum to 1.

```shell
$ chemical-engineering solve chemeng_raoult_bubble_temperature "ant=[[6.90565, 1211.03, 220.79], [6.95464, 1344.8, 219.482]]" units=mmHg_degC base=log10 "x=[0.4, 0.6]" "P=101325 Pa"
T = 368.2912291 K
y = [0.621854561429,0.378145438571]
  by: Bubble-point temperature (Raoult's law)  (sum x_i Psat_i(T) = P solved by bisection between the pure-component boiling points)
  P = 101325 Pa
  units = mmHg_degC
  base = log10
  assumes: ideal liquid and ideal gas; fractions sum to 1
```

**What the worked example means:** This calculation determines the bubble-point temperature 368.292 K for a liquid mixture with composition [0.4, 0.6] at pressure 101325 Pa. Using Antoine coefficients [6.90565 1211.03 220.79; 6.95464 1344.8 219.482] in units mmHg_degC with base log10, the algorithm solves the Raoult's law equation iteratively. The result indicates that boiling begins at 368.292 K, producing a vapor phase with composition [0.621849, 0.378151]. This value represents the threshold temperature where the sum of partial pressures equals the system pressure.

<a id="chemeng_countercurrent_extraction"></a>

#### Countercurrent liquid-liquid extraction (N stages, immiscible solvents), `chemeng_countercurrent_extraction`

Raffinate and extract solute ratios and the fraction extracted for N ideal countercurrent stages with immiscible carrier and solvent and a constant distribution ratio Y = K X (solute-free mass-ratio basis).

$$
\frac{X_F - X_N}{X_F - Y_S/K} = \frac{E^{N+1} - E}{E^{N+1} - 1},\ E = \frac{KS}{F}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | solute-free carrier flow in the feed | kg/s |
| `S` | in | solute-free solvent flow | kg/s |
| `K` | in | distribution ratio Y = K X (mass ratios) | - |
| `N` | in | number of ideal stages | - |
| `XF` | in | solute ratio in the feed, kg solute per kg carrier | - |
| `YS` | in | solute ratio in the entering solvent, kg solute per kg solvent | - |
| `XN` | out | solute ratio in the final raffinate | - |
| `Y1` | out | solute ratio in the final extract | - |
| `recovery` | out | fraction of the solute extracted | - |

**Applies when:** immiscible carrier and solvent, constant K; XF > YS/K.

```shell
$ chemical-engineering solve chemeng_countercurrent_extraction "F=100 kg/s" "S=80 kg/s" K=1.5 N=4 XF=0.1 YS=0
XN = 0.01343797033
Y1 = 0.1082025371
recovery = 0.8656202967
  by: Countercurrent liquid-liquid extraction (N stages, immiscible solvents)  (Kremser form with the extraction factor E = K S/F: (XF - XN)/(XF - YS/K) = (E^(N+1) - E)/(E^(N+1) - 1))
  F = 100 kg/s
  S = 80 kg/s
  K = 1.5
  XF = 0.1
  YS = 0
  assumes: immiscible carrier and solvent, constant K; XF > YS/K
```

**What the worked example means:** This calculation determines the equilibrium compositions after N ideal countercurrent stages using immiscible solvents. Given a feed carrier flow of 100 kg/s and solvent flow of 80 kg/s, with a distribution ratio of 1.5, the system processes a feed with initial solute ratio 0.1 against an entering solvent with solute ratio 0. The resulting final raffinate solute ratio is 0.013438, while the final extract solute ratio is 0.108203, corresponding to a total fraction extracted of 0.86562.

<a id="chemeng_crosscurrent_extraction"></a>

#### Crosscurrent (multiple-contact) extraction with fresh solvent, `chemeng_crosscurrent_extraction`

Raffinate solute ratio after N ideal stages, each fed with the same amount of fresh solvent, for immiscible solvents and a constant distribution ratio.

$$
X_N = \frac{X_F}{(1 + KS/F)^N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | solute-free carrier flow (or mass) in the feed | kg/s |
| `S` | in | solute-free solvent flow (or mass) added to EACH stage | kg/s |
| `K` | in | distribution ratio Y = K X (mass ratios) | - |
| `N` | in | number of stages | - |
| `XF` | in | solute ratio in the feed, kg solute per kg carrier | - |
| `XN` | out | solute ratio in the final raffinate | - |
| `recovery` | out | fraction of the solute extracted | - |

**Applies when:** immiscible carrier and solvent, constant K; fresh (solute-free) solvent to every stage.

```shell
$ chemical-engineering solve chemeng_crosscurrent_extraction "F=100 kg/s" "S=20 kg/s" K=1.5 N=4 XF=0.1
XN = 0.03501277966
recovery = 0.6498722034
  by: Crosscurrent (multiple-contact) extraction with fresh solvent  (each stage divides the raffinate ratio by (1 + K S/F): X_N = X_F/(1 + K S/F)^N)
  F = 100 kg/s
  S = 20 kg/s
  K = 1.5
  XF = 0.1
  assumes: immiscible carrier and solvent, constant K; fresh (solute-free) solvent to every stage
```

**What the worked example means:** This calculation determines the solute concentration in the final raffinate phase after passing through multiple extraction stages. Each stage introduces a fresh amount of solvent, defined by the solvent-to-feed ratio, to remove solute from the carrier fluid. The result, 0.0350128, indicates how effectively the repeated contacts reduce the impurity level starting from the initial feed ratio 0.1. A lower value signifies more efficient separation and higher purity of the treated carrier stream.

<a id="chemeng_raoult_dew_pressure"></a>

#### Dew-point pressure (Raoult's law): `chemeng_raoult_dew_pressure`

Pressure at which a vapour of composition y starts to condense at temperature T, and the composition of the first drop (Raoult's law, Antoine vapour pressures).

$$
\frac{1}{P_{dew}} = \sum_i \frac{y_i}{P_i^{sat}(T)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `y` | in | vapour mole fractions | - |
| `ant` | in | Antoine coefficients, one row (A, B, C) per component, in the stated convention | - |
| `units` | in | units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K | - |
| `base` | in | logarithm of the Antoine equation: log10 or ln | - |
| `T` | in | temperature | K |
| `P` | out | dew-point pressure | Pa |
| `x` | out | liquid mole fractions of the first drop | - |

**Applies when:** ideal liquid and ideal gas; fractions sum to 1.

```shell
$ chemical-engineering solve chemeng_raoult_dew_pressure "ant=[[6.90565, 1211.03, 220.79], [6.95464, 1344.8, 219.482]]" units=mmHg_degC base=log10 "y=[0.4, 0.6]" "T=368.15 K"
P = 83426.35248 Pa
x = [0.212682671661,0.787317328339]
  by: Dew-point pressure (Raoult's law)  (1/P = sum y_i/Psat_i(T); x_i = y_i P/Psat_i)
  T = 368.15 K
  units = mmHg_degC
  base = log10
  assumes: ideal liquid and ideal gas; fractions sum to 1
```

**What the worked example means:** This calculation determines the pressure at which a vapour mixture of composition [0.4, 0.6] begins to condense at temperature 368.15 K. Using Raoult's law with Antoine coefficients [6.90565 1211.03 220.79; 6.95464 1344.8 219.482] in units mmHg_degC and base log10, the reciprocal sum of vapour mole fractions divided by saturation pressures yields the dew-point pressure 83426 Pa. The resulting liquid phase composition [0.212686, 0.787314] represents the very first drop formed, which is essential for designing condensers and understanding separation limits.

<a id="chemeng_raoult_dew_temperature"></a>

#### Dew-point temperature (Raoult's law): `chemeng_raoult_dew_temperature`

Temperature at which a vapour of composition y starts to condense at pressure P, and the first-drop composition (Raoult's law, Antoine vapour pressures; iterative).

$$
\sum_i \frac{y_i P}{P_i^{sat}(T_{dew})} = 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `y` | in | vapour mole fractions | - |
| `ant` | in | Antoine coefficients, one row (A, B, C) per component, in the stated convention | - |
| `units` | in | units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K | - |
| `base` | in | logarithm of the Antoine equation: log10 or ln | - |
| `P` | in | pressure | Pa |
| `T` | out | dew-point temperature | K |
| `x` | out | liquid mole fractions of the first drop | - |

**Applies when:** ideal liquid and ideal gas; fractions sum to 1.

```shell
$ chemical-engineering solve chemeng_raoult_dew_temperature "ant=[[6.90565, 1211.03, 220.79], [6.95464, 1344.8, 219.482]]" units=mmHg_degC base=log10 "y=[0.4, 0.6]" "P=101325 Pa"
T = 374.6344972 K
x = [0.216263315226,0.783736684774]
  by: Dew-point temperature (Raoult's law)  (sum y_i P/Psat_i(T) = 1 solved by bisection between the pure-component boiling points)
  P = 101325 Pa
  units = mmHg_degC
  base = log10
  assumes: ideal liquid and ideal gas; fractions sum to 1
```

**What the worked example means:** This calculation determines the temperature at which a vapour mixture begins to condense into a liquid at a specified pressure. By solving the Raoult's law equation iteratively, the tool finds that the dew point occurs at 374.635 K. At this condition, the composition of the initial condensate droplet is given by the liquid mole fractions [0.216267, 0.783733], providing critical data for designing condensers and handling vapour streams.

<a id="chemeng_wilson_k_flash"></a>

#### Flash with Wilson K-values (hydrocarbon mixtures), `chemeng_wilson_k_flash`

Vapour fraction and phase compositions of a hydrocarbon feed at T and P with K-values estimated by Wilson's correlation from critical constants and acentric factors.

$$
K_i = \frac{P_{c,i}}{P}\exp\!\left[5.37(1+\omega_i)\left(1 - \frac{T_{c,i}}{T}\right)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | feed mole fractions | - |
| `Tc` | in | critical temperatures, K | - |
| `Pc` | in | critical pressures, in the unit p_unit | - |
| `omega` | in | acentric factors | - |
| `p_unit` | in | unit of the critical pressures: Pa, kPa, MPa, bar, atm or psi | - |
| `T` | in | flash temperature | K |
| `P` | in | flash pressure | Pa |
| `VF` | out | vapour fraction V/F | - |
| `x` | out | liquid mole fractions | - |
| `y` | out | vapour mole fractions | - |
| `K` | out | Wilson K-values | - |

**Applies when:** two-phase at the Wilson K-values; some K-value differs from 1 by more than 1 %; Wilson's correlation suits light hydrocarbons at low to moderate pressure.

```shell
$ chemical-engineering solve chemeng_wilson_k_flash "z=[0.3, 0.4, 0.3]" "Tc=[190.564, 369.83, 507.6]" "Pc=[45.99, 42.48, 30.25]" "omega=[0.011, 0.152, 0.3]" p_unit=bar "T=300 K" "P=2000000 Pa"
VF = 0.3623319151
x = [0.0449443371671,0.487795919806,0.467259743027]
y = [0.748872564861,0.245487924989,0.00563951014995]
K = [16.6622229198,0.503259488285,0.0120693259672]
  by: Flash with Wilson K-values (hydrocarbon mixtures)  (K_i = (Pc_i/P) exp[5.37 (1 + omega_i)(1 - Tc_i/T)], then the Rachford-Rice equation by bisection)
  T = 300 K
  P = 2000000 Pa
  p_unit = bar
  assumes: two-phase at the Wilson K-values; some K-value differs from 1 by more than 1 %; Wilson's correlation suits light hydrocarbons at low to moderate pressure
```

**What the worked example means:** This calculation determines the phase split for a hydrocarbon mixture at temperature 300 K and pressure 2e+06 Pa. Wilson's correlation estimates equilibrium ratios K using critical properties and acentric factors, yielding values such as [16.6622, 0.503259, 0.0120693]. The Rachford-Rice equation is then solved to find the vapour fraction 0.362332, along with the corresponding liquid mole fractions [0.0449443, 0.487796, 0.46726] and vapour mole fractions [0.748873, 0.245488, 0.00563951]. This result indicates how much of the feed remains in the vapour phase versus the liquid phase under these specific conditions.

<a id="chemeng_raoult_isothermal_flash"></a>

#### Isothermal flash of an ideal mixture (Raoult's law), `chemeng_raoult_isothermal_flash`

Vapour fraction and phase compositions of a feed flashed at temperature T and pressure P, with Raoult's-law K-values from Antoine vapour pressures.

$$
K_i = \frac{P_i^{sat}(T)}{P},\quad \sum_i \frac{z_i (K_i - 1)}{1 + \frac{V}{F}(K_i - 1)} = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | feed mole fractions | - |
| `ant` | in | Antoine coefficients, one row (A, B, C) per component, in the stated convention | - |
| `units` | in | units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K | - |
| `base` | in | logarithm of the Antoine equation: log10 or ln | - |
| `T` | in | flash temperature | K |
| `P` | in | flash pressure | Pa |
| `VF` | out | vapour fraction V/F | - |
| `x` | out | liquid mole fractions | - |
| `y` | out | vapour mole fractions | - |

**Applies when:** P between the dew- and bubble-point pressures at T (two phases); ideal liquid and gas; some K-value differs from 1 by more than 1 %.

```shell
$ chemical-engineering solve chemeng_raoult_isothermal_flash "ant=[[6.90565, 1211.03, 220.79], [6.95464, 1344.8, 219.482]]" units=mmHg_degC base=log10 "z=[0.5, 0.5]" "T=368.15 K" "P=101325 Pa"
VF = 0.4305910027
x = [0.404470447066,0.595529552934]
y = [0.626327272537,0.373672727463]
  by: Isothermal flash of an ideal mixture (Raoult's law)  (K_i = Psat_i(T)/P, then the Rachford-Rice equation solved for V/F by bisection)
  T = 368.15 K
  P = 101325 Pa
  units = mmHg_degC
  base = log10
  assumes: P between the dew- and bubble-point pressures at T (two phases); ideal liquid and gas; some K-value differs from 1 by more than 1 %
```

**What the worked example means:** This calculation determines the phase distribution of an ideal mixture flashed at temperature 368.15 K and pressure 101325 Pa. Using Raoult's law, equilibrium constants are derived from Antoine vapour pressures. The Rachford-Rice equation is then solved to find the vapour fraction 0.430535, which allows for the calculation of the resulting liquid composition [0.404485, 0.595515] and vapour composition [0.626337, 0.373663]. These values represent the split of the feed components between the two phases under the specified conditions.

<a id="chemeng_rachford_rice_flash"></a>

#### Isothermal flash with given K-values (Rachford-Rice), `chemeng_rachford_rice_flash`

Vapour fraction and the liquid and vapour compositions of a feed flashed at conditions where the K-values are known.

$$
\sum_i \frac{z_i (K_i - 1)}{1 + \frac{V}{F}(K_i - 1)} = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | feed mole fractions | - |
| `K` | in | equilibrium ratios K_i = y_i/x_i | - |
| `VF` | out | vapour fraction V/F | - |
| `x` | out | liquid mole fractions | - |
| `y` | out | vapour mole fractions | - |

**Applies when:** two-phase: sum z K > 1 and sum z/K > 1; K-values independent of composition; some K-value differs from 1 by more than 1 % (well-conditioned).

```shell
$ chemical-engineering solve chemeng_rachford_rice_flash "z=[0.5, 0.3, 0.2]" "K=[1.685, 0.742, 0.532]"
VF = 0.6907302628
x = [0.339408696966,0.365056059037,0.295535243996]
y = [0.571903654388,0.270871595806,0.157224749806]
  by: Isothermal flash with given K-values (Rachford-Rice)  (Rachford-Rice equation sum z_i (K_i - 1)/(1 + V (K_i - 1)) = 0 solved for V by bisection; x_i = z_i/(1 + V (K_i - 1)), y_i = K_i x_i)
  assumes: two-phase: sum z K > 1 and sum z/K > 1; K-values independent of composition; some K-value differs from 1 by more than 1 % (well-conditioned)
```

**What the worked example means:** This calculation determines the equilibrium split of a feed mixture given known K-values. By solving the Rachford-Rice equation, it finds the vapour fraction 0.69073 that balances the material requirements. The resulting liquid mole fractions are [0.339409, 0.365056, 0.295535] and the vapour mole fractions are [0.571904, 0.270872, 0.157225], representing the distribution of each component between the two phases at the specified conditions.

<a id="chemeng_mccabe_thiele_stages"></a>

#### McCabe-Thiele number of theoretical stages (binary), `chemeng_mccabe_thiele_stages`

Number of theoretical stages (the partial reboiler counted as one) of a binary column at reflux ratio R, by stepping between the operating lines and a constant-alpha equilibrium curve, with the feed switched at the intersection of the operating lines.

$$
y_{n+1} = \frac{R}{R+1}x_n + \frac{x_D}{R+1},\quad x_n = \frac{y_n}{\alpha - (\alpha-1)y_n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | relative volatility of the light component (constant) | - |
| `zF` | in | mole fraction of the light component in the feed | - |
| `xD` | in | mole fraction of the light component in the distillate | - |
| `q` | in | feed quality q: 1 saturated liquid, 0 saturated vapour, > 1 subcooled, < 0 superheated | - |
| `xB` | in | mole fraction of the light component in the bottoms | - |
| `R` | in | reflux ratio L/D | - |
| `N` | out | theoretical stages including the partial reboiler | - |
| `Rmin` | out | minimum reflux ratio | - |

**Applies when:** binary, constant alpha, constant molar overflow, total condenser; R > R_min; 0 < xB < zF < xD < 1.

```shell
$ chemical-engineering solve chemeng_mccabe_thiele_stages alpha=2.5 zF=0.5 xD=0.95 q=1 xB=0.05 R=2
N = 11
Rmin = 1.1
  by: McCabe-Thiele number of theoretical stages (binary)  (stepping from the top: equilibrium x from y, then y from the rectifying line above the intersection and the stripping line below it, until x <= xB)
  alpha = 2.5
  zF = 0.5
  xD = 0.95
  q = 1
  xB = 0.05
  R = 2
  assumes: binary, constant alpha, constant molar overflow, total condenser; R > R_min; 0 < xB < zF < xD < 1
```

**What the worked example means:** This calculation determines the number of theoretical stages needed for binary distillation by stepping between operating lines and the equilibrium curve. Using a relative volatility of 2.5, feed composition of 0.5, distillate purity of 0.95, bottoms purity of 0.05, and feed quality of 1, the method finds that 11 stages are required. The minimum reflux ratio for this separation is calculated as 1.1.

<a id="chemeng_fenske_minimum_stages"></a>

#### Minimum number of stages at total reflux (Fenske), `chemeng_fenske_minimum_stages`

The minimum number of equilibrium stages of a binary distillation at total reflux by the Fenske equation (constant relative volatility), counting the partial reboiler as a stage.

$$
N_{min} = \frac{\ln\left[\frac{x_D}{1 - x_D}\,\frac{1 - x_B}{x_B}\right]}{\ln\alpha}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | relative volatility of the light component (constant) | - |
| `xD` | in | mole fraction of the light component in the distillate | - |
| `xB` | in | mole fraction of the light component in the bottoms | - |
| `Nmin` | out | minimum number of equilibrium stages (partial reboiler included) | - |
| `Nmin_trays` | out | minimum number of trays (Nmin - 1, the reboiler excluded) | - |

**Applies when:** binary, constant alpha; 0 < xB < xD < 1; total reflux, total condenser.

```shell
$ chemical-engineering solve chemeng_fenske_minimum_stages alpha=2.5 xD=0.95 xB=0.05
Nmin = 6.426866226
Nmin_trays = 5.426866226
  by: Minimum number of stages at total reflux (Fenske)  (N_min = ln[(xD/(1 - xD)) ((1 - xB)/xB)]/ln(alpha))
  alpha = 2.5
  xD = 0.95
  xB = 0.05
  assumes: binary, constant alpha; 0 < xB < xD < 1; total reflux, total condenser
```

**What the worked example means:** This calculation determines the theoretical minimum number of equilibrium stages for a binary distillation column operating at total reflux using the Fenske equation. Given a relative volatility of 2.5, a distillate purity of 0.95, and a bottoms purity of 0.05, the result indicates that 6.42687 stages are required in total. Since the partial reboiler acts as one equilibrium stage, the actual number of physical trays needed inside the column is 5.42687. This value serves as a lower bound for the column height; real columns require more stages due to finite reflux ratios and efficiency losses.

<a id="chemeng_multicomponent_underwood"></a>

#### Minimum reflux of a multicomponent column (Underwood), `chemeng_multicomponent_underwood`

The minimum reflux ratio of a multicomponent distillation with constant relative volatilities by Underwood's equations: the root theta between the key volatilities of sum alpha_i z_i/(alpha_i - theta) = 1 - q, then R_min + 1 = sum alpha_i x_D,i/(alpha_i - theta).

$$
\sum_i \frac{\alpha_i z_i}{\alpha_i - \theta} = 1 - q,\quad R_{min} + 1 = \sum_i \frac{\alpha_i x_{D,i}}{\alpha_i - \theta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | relative volatility of each component (any common reference, constant) | - |
| `z` | in | feed mole fractions (sum to 1) | - |
| `xD` | in | distillate mole fractions at minimum reflux (sum to 1) | - |
| `q` | in | feed quality q (1 saturated liquid, 0 saturated vapour) | - |
| `lk` | in | light key: its position in the lists (1 = first) | - |
| `hk` | in | heavy key: its position in the lists | - |
| `theta` | out | Underwood root between the key volatilities | - |
| `Rmin` | out | minimum reflux ratio L/D | - |

**Applies when:** constant relative volatilities and molar overflow; adjacent keys (no component between them in volatility); the distillate composition at minimum reflux is stated.

```shell
$ chemical-engineering solve chemeng_multicomponent_underwood "alpha=[2.5, 1, 0.5]" "z=[0.4, 0.3, 0.3]" "xD=[0.9, 0.1, 0]" q=1 lk=1 hk=2
theta = 1.402634306
Rmin = 0.8020004774
  by: Minimum reflux of a multicomponent column (Underwood)  (sum alpha_i z_i/(alpha_i - theta) = 1 - q, theta between alpha_HK and alpha_LK (bisection); R_min = sum alpha_i x_Di/(alpha_i - theta) - 1)
  q = 1
  assumes: constant relative volatilities and molar overflow; adjacent keys (no component between them in volatility); the distillate composition at minimum reflux is stated
```

**What the worked example means:** This calculation determines the minimum reflux ratio for a multicomponent distillation column using Underwood's equations. It finds the parameter theta, which lies between the relative volatilities of the light and heavy keys, by solving the feed equation involving the feed composition z and quality q. Once theta is established as 1.40263, it is substituted into the second equation along with the specified distillate composition xD to yield the minimum reflux ratio Rmin of 0.802. This value represents the theoretical lower limit on energy consumption required to achieve the desired separation under constant molar overflow assumptions.

<a id="chemeng_binary_minimum_reflux"></a>

#### Minimum reflux ratio of a binary column (Underwood), `chemeng_binary_minimum_reflux`

Minimum reflux ratio of a binary distillation with constant relative volatility, for any feed condition q (Underwood's equations).

$$
\sum_i \frac{\alpha_i z_{F,i}}{\alpha_i - \theta} = 1 - q,\quad R_{min} + 1 = \sum_i \frac{\alpha_i x_{D,i}}{\alpha_i - \theta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | relative volatility of the light component (constant) | - |
| `zF` | in | mole fraction of the light component in the feed | - |
| `xD` | in | mole fraction of the light component in the distillate | - |
| `q` | in | feed quality q: 1 saturated liquid, 0 saturated vapour, > 1 subcooled, < 0 superheated | - |
| `Rmin` | out | minimum reflux ratio L/D | - |

**Applies when:** binary, constant alpha, constant molar overflow; 0 < zF < xD < 1.

```shell
$ chemical-engineering solve chemeng_binary_minimum_reflux alpha=2.5 zF=0.5 xD=0.95 q=1
Rmin = 1.1
  by: Minimum reflux ratio of a binary column (Underwood)  (Underwood: sum alpha_i z_i/(alpha_i - theta) = 1 - q for theta between 1 and alpha (bisection); R_min + 1 = sum alpha_i x_Di/(alpha_i - theta))
  alpha = 2.5
  zF = 0.5
  xD = 0.95
  q = 1
  assumes: binary, constant alpha, constant molar overflow; 0 < zF < xD < 1
```

**What the worked example means:** This calculation finds the minimum reflux ratio required for a binary distillation column using Underwood's method. It solves for the root theta between one and the relative volatility alpha, then applies this to the distillate composition xD and feed condition q. The resulting value 1.1 represents the lowest liquid-to-distillate flow ratio that can achieve the specified separation under constant molar overflow assumptions.

<a id="chemeng_absorber_transfer_units"></a>

#### Packed absorber: number of transfer units and height (Colburn), `chemeng_absorber_transfer_units`

Number of overall gas-phase transfer units and packed height of a dilute countercurrent absorber with linear equilibrium (Colburn's equation).

$$
N_{OG} = \frac{\ln\!\left[\left(1-\frac{1}{A}\right)\frac{y_1 - m x_2}{y_2 - m x_2} + \frac{1}{A}\right]}{1 - 1/A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `y1` | in | solute mole fraction in the gas entering (bottom) | - |
| `y2` | in | solute mole fraction in the gas leaving (top) | - |
| `x2` | in | solute mole fraction in the liquid entering (top) | - |
| `m` | in | slope of the equilibrium line y = m x | - |
| `A` | in | absorption factor A = L/(m V) | - |
| `HOG` | in | height of an overall gas-phase transfer unit | m |
| `NOG` | out | number of overall gas-phase transfer units | - |
| `Z` | out | packed height | m |

**Applies when:** dilute, constant L and V, linear equilibrium; y1 > y2 > m x2; no pinch.

```shell
$ chemical-engineering solve chemeng_absorber_transfer_units y1=0.02 y2=0.001 x2=0 m=1.2 A=1.5 "HOG=0.6 m"
NOG = 5.977290494
Z = 3.586374296 m
  by: Packed absorber: number of transfer units and height (Colburn)  (N_OG = ln[(1 - 1/A)(y1 - m x2)/(y2 - m x2) + 1/A]/(1 - 1/A); Z = H_OG N_OG)
  y1 = 0.02
  y2 = 0.001
  x2 = 0
  m = 1.2
  A = 1.5
  HOG = 0.6 m
  assumes: dilute, constant L and V, linear equilibrium; y1 > y2 > m x2; no pinch
```

**What the worked example means:** This computation determines the required packed column height for a dilute countercurrent absorber using Colburn's equation. It calculates the number of overall gas-phase transfer units from the inlet and outlet gas compositions, liquid inlet composition, equilibrium slope, and absorption factor. The final height is obtained by multiplying this unit count by the height of one transfer unit, resulting in 3.58637 m.

<a id="chemeng_kremser_stripper"></a>

#### Stripper with N equilibrium stages (Kremser), `chemeng_kremser_stripper`

Solute mole fraction in the liquid leaving a countercurrent stripper of N ideal stages, and the fraction stripped, for dilute liquid with linear equilibrium y = m x.

$$
\frac{x_{in} - x_{out}}{x_{in} - y_{in}/m} = \frac{S^{N+1} - S}{S^{N+1} - 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of ideal stages | - |
| `S` | in | stripping factor S = m V/L | - |
| `m` | in | slope of the equilibrium line y = m x | - |
| `x_in` | in | solute mole fraction in the entering liquid | - |
| `y_in` | in | solute mole fraction in the entering stripping gas | - |
| `x_out` | out | solute mole fraction in the leaving liquid | - |
| `recovery` | out | fraction of the entering solute stripped | - |

**Applies when:** dilute, constant L and V, linear equilibrium; x_in > y_in/m.

```shell
$ chemical-engineering solve chemeng_kremser_stripper N=6 S=1.5 m=3 x_in=0.01 y_in=0
x_out = 0.0003108305002
recovery = 0.96891695
  by: Stripper with N equilibrium stages (Kremser)  (Kremser: (x_in - x_out)/(x_in - y_in/m) = (S^(N+1) - S)/(S^(N+1) - 1); N/(N + 1) when S = 1)
  S = 1.5
  m = 3
  x_in = 0.01
  y_in = 0
  assumes: dilute, constant L and V, linear equilibrium; x_in > y_in/m
```

**What the worked example means:** This calculation determines the final concentration of solute in the liquid exiting a countercurrent stripper. Using the Kremser equation, it relates the inlet and outlet concentrations to the number of equilibrium stages and the stripping factor. The result shows that with 6 ideal stages and a stripping factor of 1.5, the solute mole fraction drops from 0.01 to 0.000310831, achieving a removal efficiency of 0.968917. This indicates how effectively the gas stream removes the dissolved component under dilute conditions.

<a id="chemeng_antoine_vapour_pressure"></a>

#### Vapour pressure from the Antoine equation: `chemeng_antoine_vapour_pressure`

Vapour pressure of a pure liquid at temperature T from Antoine coefficients written in a stated convention (logarithm and units).

$$
\log_{10} P^{sat} = A - \frac{B}{T + C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | Antoine coefficient A | - |
| `B` | in | Antoine coefficient B | - |
| `C` | in | Antoine coefficient C | - |
| `units` | in | units the Antoine coefficients are written for: pressure_temperature, e.g. mmHg_degC (P in mmHg, T in degC) or kPa_K | - |
| `base` | in | logarithm of the Antoine equation: log10 or ln | - |
| `T` | in | temperature | K |
| `Psat` | out | vapour pressure | Pa |

**Applies when:** the convention (units, base) is stated; T within the range the coefficients were fitted for.

```shell
$ chemical-engineering solve chemeng_antoine_vapour_pressure A=8.07131 B=1730.63 C=233.426 units=mmHg_degC base=log10 "T=373.15 K"
Psat = 101336.5294 Pa
  by: Vapour pressure from the Antoine equation  (log P = A - B/(T + C) in the stated units and logarithm; P returned in Pa)
  A = 8.07131
  B = 1730.63
  C = 233.426
  T = 373.15 K
  units = mmHg_degC
  base = log10
  assumes: the convention (units, base) is stated; T within the range the coefficients were fitted for
```

**What the worked example means:** This calculation determines the saturation vapour pressure of a pure substance at temperature 373.15 K. It applies the Antoine equation, which relates the logarithm of pressure to temperature through empirical constants A, B, and C. The result, 101337 Pa, indicates the pressure at which the liquid and vapour phases are in equilibrium. This value is critical for assessing boiling conditions and designing separation processes such as distillation.

### Transport (7)

<a id="chemeng_cake_filtration_volume"></a>

#### Constant-pressure cake filtration: filtrate volume after a time, `chemeng_cake_filtration_volume`

Volume of filtrate collected after time t in constant-pressure cake filtration (Ruth equation with cake and medium resistances).

$$
t = \frac{\mu\alpha c}{2A^2\Delta P}V^2 + \frac{\mu R_m}{A\,\Delta P}V
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | filter area | m^2 |
| `dP` | in | pressure difference across cake and medium | Pa |
| `mu` | in | filtrate viscosity | Pa*s |
| `alpha` | in | specific cake resistance | m/kg |
| `c` | in | mass of dry cake deposited per volume of filtrate | kg/m^3 |
| `Rm` | in | filter medium resistance | 1/m |
| `t` | in | filtration time | s |
| `V` | out | filtrate volume collected | m^3 |

**Applies when:** constant pressure, incompressible cake; Rm > 0.

```shell
$ chemical-engineering solve chemeng_cake_filtration_volume "A=1 m^2" "dP=100000 Pa" "mu=0.001 Pa*s" "alpha=1e+11 m/kg" "c=20 kg/m^3" "Rm=1e+10 1/m" "t=600 s"
V = 0.24 m^3
  by: Constant-pressure cake filtration: filtrate volume after a time  (dV/dt = A^2 dP/(mu (alpha c V + A Rm)), integrated: t = mu alpha c V^2/(2 A^2 dP) + mu Rm V/(A dP), solved for V)
  A = 1 m^2
  dP = 100000 Pa
  mu = 0.001 Pa*s
  alpha = 1e+11 m/kg
  c = 20 kg/m^3
  Rm = 1e+10 1/m
  t = 600 s
  assumes: constant pressure, incompressible cake; Rm > 0
```

**What the worked example means:** This calculation determines the total volume of liquid that has passed through the filter, which is 0.24 m^3. The result comes from integrating the filtration rate equation over the specified time of 600 s, using a filter area of 1 m^2 and a pressure difference of 100000 Pa. It accounts for resistance from both the accumulating cake, defined by specific cake resistance 1e+11 m/kg and concentration 20 kg/m^3, and the initial medium resistance 1e+10 1/m. Essentially, it tells you how much clear liquid you have successfully separated from the slurry up to this moment.

<a id="chemeng_stefan_diffusion_flux"></a>

#### Diffusion of A through stagnant B (Stefan film), `chemeng_stefan_diffusion_flux`

Steady molar flux of A diffusing through a stagnant film of non-diffusing B in an ideal gas (evaporation, absorption through an inert).

$$
N_A = \frac{cD_{AB}}{L}\ln\frac{1 - y_{A2}}{1 - y_{A1}},\quad c = \frac{P}{RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusivity of A in B | m^2/s |
| `P` | in | total pressure | Pa |
| `T` | in | temperature | K |
| `L` | in | film thickness (diffusion path length) | m |
| `yA1` | in | mole fraction of A at the start of the path | - |
| `yA2` | in | mole fraction of A at the end of the path | - |
| `NA` | out | molar flux of A | mol/(m^2*s) |

**Applies when:** steady state, ideal gas, constant D; B stagnant (no net flux of B).

```shell
$ chemical-engineering solve chemeng_stefan_diffusion_flux "D=2.6e-05 m^2/s" "P=101325 Pa" "T=298.15 K" "L=0.1 m" yA1=0.0313 yA2=0
NA = 0.0003379499217 mol/(m^2*s)
  by: Diffusion of A through stagnant B (Stefan film)  (N_A = (c D/L) ln[(1 - yA2)/(1 - yA1)], c = P/(R T))
  D = 2.6e-05 m^2/s
  P = 101325 Pa
  T = 298.15 K
  L = 0.1 m
  yA1 = 0.0313
  yA2 = 0
  assumes: steady state, ideal gas, constant D; B stagnant (no net flux of B)
```

**What the worked example means:** This calculation determines the steady molar flux of species A diffusing through a stagnant film of species B. It uses the ideal gas law to find total concentration from pressure 101325 Pa and temperature 298.15 K, then applies Stefan's law with mole fractions 0.0313 and 0 across path length 0.1 m using diffusivity 2.6e-05 m^2/s. The resulting flux 0.00033795 mol/(m^2*s) represents the rate at which component A moves per unit area under these specific conditions.

<a id="chemeng_equimolar_counterdiffusion"></a>

#### Equimolar counterdiffusion flux: `chemeng_equimolar_counterdiffusion`

The molar flux of A through a film of thickness L in equimolar counterdiffusion (N_B = -N_A, ideal gas): N_A = c D (y_A1 - y_A2)/L with c = P/(R T).

$$
N_A = \frac{P}{RT}\,\frac{D_{AB}\,(y_{A1} - y_{A2})}{L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusivity of A in B | m^2/s |
| `P` | in | total pressure | Pa |
| `T` | in | temperature | K |
| `L` | in | film (path) thickness | m |
| `yA1` | in | mole fraction of A at z = 0 | - |
| `yA2` | in | mole fraction of A at z = L | - |
| `NA` | out | molar flux of A | mol/(m^2*s) |
| `NB` | out | molar flux of B (= -N_A) | mol/(m^2*s) |

**Applies when:** steady state, ideal gas, constant D; equimolar counterdiffusion (N_B = -N_A); yA1 > yA2.

```shell
$ chemical-engineering solve chemeng_equimolar_counterdiffusion "D=2.3e-05 m^2/s" "P=101325 Pa" "T=298.15 K" "L=0.1 m" yA1=0.2 yA2=0.05
NA = 0.001410154536 mol/(m^2*s)
NB = -0.001410154536 mol/(m^2*s)
  by: Equimolar counterdiffusion flux  (N_A = (P/(R T)) D (y_A1 - y_A2)/L)
  D = 2.3e-05 m^2/s
  P = 101325 Pa
  T = 298.15 K
  L = 0.1 m
  yA1 = 0.2
  yA2 = 0.05
  assumes: steady state, ideal gas, constant D; equimolar counterdiffusion (N_B = -N_A); yA1 > yA2
```

**What the worked example means:** This calculation determines the molar flux of species A, resulting in 0.00141015 mol/(m^2*s), under equimolar counterdiffusion conditions. The flux depends on the total pressure 101325 Pa, temperature 298.15 K, diffusivity 2.3e-05 m^2/s, film thickness 0.1 m, and the mole fraction difference between the boundaries defined by 0.2 and 0.05. Because the process assumes ideal gas behavior and steady state, the flux of component B is equal in magnitude but opposite in direction to that of A, yielding -0.00141015 mol/(m^2*s). This result indicates the rate at which mass transfers through the stagnant film per unit area.

<a id="chemeng_mass_transfer_coefficient"></a>

#### Mass-transfer coefficient from a stated Sherwood correlation, `chemeng_mass_transfer_coefficient`

The convective mass-transfer coefficient k_c = Sh D/L from the Reynolds and Schmidt numbers by a stated correlation: a sphere (Ranz-Marshall), a flat plate in laminar flow (average) or turbulent flow in a pipe (Linton-Sherwood).

$$
Sh = \frac{k_c L}{D},\quad Sh = 2 + 0.6\,Re^{1/2}Sc^{1/3}\ \text{(sphere)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `geometry` | in | correlation: sphere (Ranz-Marshall), flat_plate (laminar, average) or pipe (Linton-Sherwood, turbulent) | - |
| `u` | in | fluid velocity | m/s |
| `L` | in | characteristic length: sphere or pipe diameter, plate length | m |
| `D` | in | diffusivity of the solute in the fluid | m^2/s |
| `rho` | in | fluid density | kg/m^3 |
| `mu` | in | fluid viscosity | Pa*s |
| `Re` | out | Reynolds number rho u L/mu | - |
| `Sc` | out | Schmidt number mu/(rho D) | - |
| `Sh` | out | Sherwood number | - |
| `kc` | out | mass-transfer coefficient Sh D/L | m/s |

**Applies when:** within each correlation's stated range of Re and Sc; dilute solute.

```shell
$ chemical-engineering solve chemeng_mass_transfer_coefficient geometry=sphere "u=0.1 m/s" "L=0.001 m" "D=5e-09 m^2/s" "rho=1000 kg/m^3" "mu=0.001 Pa*s"
Re = 100
Sc = 200
Sh = 37.08821286
kc = 0.0001854410643 m/s
  by: Mass-transfer coefficient from a stated Sherwood correlation  (Re = rho u L/mu, Sc = mu/(rho D); sphere Sh = 2 + 0.6 Re^1/2 Sc^1/3; plate Sh = 0.664 Re^1/2 Sc^1/3; pipe Sh = 0.023 Re^0.83 Sc^1/3; k_c = Sh D/L)
  u = 0.1 m/s
  L = 0.001 m
  D = 5e-09 m^2/s
  rho = 1000 kg/m^3
  mu = 0.001 Pa*s
  geometry = sphere
  assumes: within each correlation's stated range of Re and Sc; dilute solute
```

**What the worked example means:** This calculation determines the convective mass-transfer coefficient, denoted as kc, using the specified geometry and fluid properties. First, the Reynolds number (Re) and Schmidt number (Sc) are computed from the velocity, characteristic length, density, viscosity, and diffusivity. These dimensionless groups are then substituted into the appropriate correlation to yield the Sherwood number (Sh). Finally, the mass-transfer coefficient is derived by rearranging the definition of the Sherwood number, resulting in a value of 0.000185441 m/s for the given conditions.

<a id="chemeng_carman_pressure_drop"></a>

#### Packed-bed pressure drop (Carman correlation), `chemeng_carman_pressure_drop`

Pressure drop through a packed bed from Carman's correlation of the bed friction factor (180/Re' + 2.87/Re'^0.1 form).

$$
f_p = \frac{\left(180 + 2.871\left(\frac{Re}{1-\varepsilon}\right)^{0.9}\right)(1-\varepsilon)^2}{\varepsilon^3 Re},\quad \Delta P = \frac{f_p\,\rho v_s^2 L}{d_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dp` | in | particle diameter | m |
| `eps` | in | bed voidage (porosity) | - |
| `vs` | in | superficial velocity | m/s |
| `rho` | in | fluid density | kg/m^3 |
| `mu` | in | fluid viscosity | Pa*s |
| `L` | in | bed depth | m |
| `dP` | out | pressure drop across the bed | Pa |

**Applies when:** 0 < eps < 1; incompressible flow.

```shell
$ chemical-engineering solve chemeng_carman_pressure_drop "dp=0.0008 m" eps=0.4 "vs=0.05 m/s" "rho=1000 kg/m^3" "mu=0.001 Pa*s" "L=1 m"
dP = 134368.2731 Pa
  by: Packed-bed pressure drop (Carman correlation)  (f_p = (180 + 2.871 (Re/(1 - eps))^0.9)(1 - eps)^2/(eps^3 Re); dP = f_p rho vs^2 L/dp, Re = dp rho vs/mu)
  dp = 0.0008 m
  eps = 0.4
  vs = 0.05 m/s
  rho = 1000 kg/m^3
  mu = 0.001 Pa*s
  L = 1 m
  assumes: 0 < eps < 1; incompressible flow
```

**What the worked example means:** This calculation determines the pressure drop through a packed bed using Carman's correlation. It derives the friction factor from the particle diameter, bed voidage, superficial velocity, fluid density, and viscosity, then applies it to the bed depth. The resulting pressure drop of 134368 Pa indicates the energy loss required to drive the fluid through the porous medium under these specific operating conditions.

<a id="chemeng_ergun_pressure_drop"></a>

#### Packed-bed pressure drop (Ergun equation): `chemeng_ergun_pressure_drop`

Pressure drop of a fluid flowing through a packed bed of spheres from the particle size, voidage, superficial velocity and fluid properties (Ergun equation).

$$
\frac{\Delta P}{L} = \frac{150\,\mu(1-\varepsilon)^2 v_s}{\varepsilon^3 d_p^2} + \frac{1.75\,(1-\varepsilon)\rho v_s^2}{\varepsilon^3 d_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dp` | in | particle diameter | m |
| `eps` | in | bed voidage (porosity) | - |
| `vs` | in | superficial velocity | m/s |
| `rho` | in | fluid density | kg/m^3 |
| `mu` | in | fluid viscosity | Pa*s |
| `L` | in | bed depth | m |
| `dP` | out | pressure drop across the bed | Pa |

**Applies when:** 0 < eps < 1; incompressible flow (or small pressure drop for a gas); spherical or equivalent particles.

```shell
$ chemical-engineering solve chemeng_ergun_pressure_drop "dp=0.0008 m" eps=0.4 "vs=0.05 m/s" "rho=1000 kg/m^3" "mu=0.001 Pa*s" "L=1 m"
dP = 117187.5 Pa
  by: Packed-bed pressure drop (Ergun equation)  (dP/L = 150 mu (1 - eps)^2 vs/(eps^3 dp^2) + 1.75 (1 - eps) rho vs^2/(eps^3 dp))
  dp = 0.0008 m
  eps = 0.4
  vs = 0.05 m/s
  rho = 1000 kg/m^3
  mu = 0.001 Pa*s
  L = 1 m
  assumes: 0 < eps < 1; incompressible flow (or small pressure drop for a gas); spherical or equivalent particles
```

**What the worked example means:** This calculation determines the pressure drop across a packed bed using the Ergun equation, which accounts for both viscous and inertial resistance to fluid flow. Given the bed depth of 1 m, particle diameter of 0.0008 m, voidage of 0.4, superficial velocity of 0.05 m/s, fluid density of 1000 kg/m^3, and fluid viscosity of 0.001 Pa*s, the resulting pressure drop is 117188 Pa. This value represents the energy loss per unit area required to push the fluid through the porous medium.

<a id="chemeng_filtration_constants_fit"></a>

#### Specific cake and medium resistance from constant-pressure filtration data, `chemeng_filtration_constants_fit`

Specific cake resistance and filter-medium resistance from a constant-pressure filtration test (times and filtrate volumes), by the straight line of t/V against V.

$$
\frac{t}{V} = \frac{\mu\alpha c}{2A^2\Delta P}V + \frac{\mu R_m}{A\Delta P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | times at which the filtrate volumes were read, in t_unit | - |
| `V` | in | cumulative filtrate volumes, in v_unit | - |
| `t_unit` | in | unit of the times | - |
| `v_unit` | in | unit of the volumes | - |
| `A` | in | filter area | m^2 |
| `dP` | in | constant pressure difference | Pa |
| `mu` | in | filtrate viscosity | Pa*s |
| `c` | in | dry cake mass per volume of filtrate | kg/m^3 |
| `alpha` | out | specific cake resistance | m/kg |
| `Rm` | out | filter medium resistance | 1/m |
| `slope` | out | slope of t/V against V, s/m^6 | - |
| `intercept` | out | intercept of t/V against V, s/m^3 | - |

**Applies when:** constant pressure, incompressible cake; a positive slope and intercept.

```shell
$ chemical-engineering solve chemeng_filtration_constants_fit "t=[17.3, 41.3, 72, 108.3, 152.1, 201.7]" "V=[0.5, 1, 1.5, 2, 2.5, 3]" t_unit=s v_unit=L "A=0.0439 m^2" "dP=338000 Pa" "mu=0.000895 Pa*s" "c=23.47 kg/m^3"
alpha = 8.078221474e+11 m/kg
Rm = 4.679734332e+11 1/m
slope = 13024952.38
intercept = 28226.88889
  by: Specific cake and medium resistance from constant-pressure filtration data  (least squares of t/V = (mu alpha c/(2 A^2 dP)) V + mu Rm/(A dP))
  A = 0.0439 m^2
  dP = 338000 Pa
  mu = 0.000895 Pa*s
  c = 23.47 kg/m^3
  t_unit = s
  v_unit = L
  assumes: constant pressure, incompressible cake; a positive slope and intercept
```

**What the worked example means:** This calculation determines the specific cake resistance, denoted as alpha, and the filter medium resistance, denoted as Rm, from constant-pressure filtration data. By plotting the ratio of time to filtrate volume against the cumulative filtrate volume, a straight line is obtained whose slope and intercept relate directly to these physical properties. The resulting value for specific cake resistance is 8.07822e+11 m/kg, while the filter medium resistance is 4.67973e+11 1/m. These parameters are essential for scaling up filtration processes and predicting performance under varying operational conditions.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
