# Chemical Engineering: the desktop app, user guide

**Chemical Engineering**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Chemical Engineering covers material and energy balances, reactor design, separations (distillation, absorption, extraction), transport phenomena, process control and reservoir and petroleum basics. Its desktop app holds all 92 calculations in 11 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemical-engineering-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemical-engineering-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-chemical-engineering-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-chemical-engineering.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemical-engineering-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-chemical-engineering-1.6.5-linux-x86_64.tar.gz
./ai999x-chemical-engineering-1.6.5-linux-x86_64/ai999x-chemical-engineering
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Chemical Engineering app, opened on CSTR volume for a conversion (first order) with its worked example.](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/hero.webp)

*The Chemical Engineering app, opened on CSTR volume for a conversion (first order) with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Reactor design › CSTR volume for a conversion (first order) (or type part of its name in the search box). It opens with a worked example filled in: “First-order CSTR volume: v0 = 0.01 m^3/s, k = 0.001 1/s, X = 0.9” The answer: V = 90 m^3, tau = 9000 s.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![CSTR volume for a conversion (first order): the live scene of the worked example.](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-1.webp)

*CSTR volume for a conversion (first order): the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “First-order CSTR volume: v0 = 36 m^3/h, k = 0.06 1/min, X = 90%”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: CSTR volume for a conversion (first order)
V = 90 m^3
tau = 9000 s
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Combustion with excess air: air requirement and flue-gas composition](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-2.webp)

*Combustion with excess air: air requirement and flue-gas composition*

![Bubble-point temperature (Raoult's law)](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-3.webp)

*Bubble-point temperature (Raoult's law)*

![Steady radial flow of oil to a well (Darcy) and productivity index](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-4.webp)

*Steady radial flow of oil to a well (Darcy) and productivity index*

![Net positive suction head available](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-5.webp)

*Net positive suction head available*

![Closed-loop stability of a P-controlled process (closed-loop poles)](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-6.webp)

*Closed-loop stability of a P-controlled process (closed-loop poles)*

![Equipment cost scaling with a capacity exponent and cost indices](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-7.webp)

*Equipment cost scaling with a capacity exponent and cost indices*

![Gas compressibility from the virial equation (Pitzer-Abbott correlation)](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/scene-8.webp)

*Gas compressibility from the virial equation (Pitzer-Abbott correlation)*

![CSTR volume for a conversion (first order): a chart, enlarged.](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/chart-1.webp)

*CSTR volume for a conversion (first order): a chart, enlarged.*

![Combustion with excess air: air requirement and flue-gas composition: a chart, enlarged.](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/chart-2.webp)

*Combustion with excess air: air requirement and flue-gas composition: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Chemical Engineering app in the dark theme.](https://www.ai999x.com/?ax_shot=chemical-engineering-ai999x/hero-dark.webp)

*The Chemical Engineering app in the dark theme.*

## 7. Every calculation

92 calculations in 11 topics; each has its own entry on the model's page (https://www.ai999x.com/models/chemical-engineering-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Energy balances | Adiabatic flame temperature (complete combustion, NASA polynomials), Adiabatic reactor outlet temperature, Batch cooling (or heating) with a non-isothermal coolant in a coil, Batch heating of a stirred tank: temperature after a time, Batch heating of a stirred tank: time to reach a temperature, Heat of reaction at another temperature (Kirchhoff), Sensible heat with a polynomial heat capacity |
| Fluid transport | Net positive suction head available, Pump total head and power |
| Material balances | Blending: feed flows that make a product of given composition, Combustion with excess air: air requirement and flue-gas composition, Filling or draining well-mixed tank (variable volume), Mass fractions to mole fractions and mean molar mass, Reactor with recycle and purge, Two-product separator balance (binary distillation column), Unsteady well-mixed tank (concentration change) |
| Petroleum engineering | Exponential decline curve: rate and cumulative production, Gas-well deliverability from a back-pressure test (Rawlins-Schellhardt), Harmonic decline curve: rate and cumulative production, Hyperbolic decline curve: rate and cumulative production |
| Process control | Closed-loop stability of a P-controlled process (closed-loop poles), Discrete PID controller output for an error history, First-order-plus-dead-time process: response to a step, Frequency response of first-order lags in series (amplitude ratio and phase), Gain and phase margins of a PI loop around lags in series, Process gain and time constant from step-test data (least squares), Proportional control: steady-state offset after a set-point step, Ziegler-Nichols closed-loop tuning of lags in series |
| Process design | Equipment cost scaling with a capacity exponent and cost indices |
| Property estimation | Gas compressibility from the virial equation (Pitzer-Abbott correlation), Heat of vaporisation at another temperature (Watson), Heat of vaporisation at the normal boiling point (Riedel), Heat of vaporisation from the acentric factor (Pitzer), Peng-Robinson equation of state: compressibility and fugacity coefficient, Saturated liquid molar volume (Rackett), Soave-Redlich-Kwong equation of state: compressibility and fugacity coefficient, Vapour pressure by the Lee-Kesler correlation |
| Reactor design | Batch reactor conversion after a time (first order), Batch reactor conversion after a time (second order), Batch reactor time for a conversion (first order), Batch reactor time for a conversion (second order), CSTR conversion for a volume (first order), CSTR conversion for a volume (second order), CSTR volume for a conversion (first order), CSTR volume for a conversion (second order), Equal CSTRs in series: conversion (first order), Equal CSTRs in series: conversion (second order), Gas-phase PFR volume with volume change (first order), Gas-phase PFR volume with volume change (second order), PFR conversion for a volume (first order, liquid), PFR conversion for a volume (second order, liquid), PFR volume for a conversion (first order, liquid), PFR volume for a conversion (second order, liquid), Plug-flow reactor with recycle: conversion (first order), Residence-time distribution: mean, variance and tanks-in-series number from tracer data, Reversible first-order reaction A <-> B in a batch reactor, Series reactions A -> B -> C: maximum of the intermediate (batch or PFR) |
| Reservoir engineering | Gas initially in place from the p/z plot, Oil in place and gas-cap size from the material balance (Havlena-Odeh, gas cap), Oil initially in place from the material balance (Havlena-Odeh straight line), Pressure build-up test: Horner analysis for permeability and extrapolated pressure, Steady radial flow of oil to a well (Darcy) and productivity index, Transient drawdown around a well (line-source solution) |
| Separations | Absorber with N equilibrium stages (Kremser), Activity coefficients from the NRTL equation, Activity coefficients from the UNIQUAC equation, Activity coefficients from the Wilson equation, Boiling (saturation) temperature from the Antoine equation, Bubble-point pressure (Raoult's law), Bubble-point pressure by the modified Raoult's law (activity coefficients), Bubble-point temperature (Raoult's law), Countercurrent liquid-liquid extraction (N stages, immiscible solvents), Crosscurrent (multiple-contact) extraction with fresh solvent, Dew-point pressure (Raoult's law), Dew-point temperature (Raoult's law), Flash with Wilson K-values (hydrocarbon mixtures), Isothermal flash of an ideal mixture (Raoult's law), Isothermal flash with given K-values (Rachford-Rice), McCabe-Thiele number of theoretical stages (binary), Minimum number of stages at total reflux (Fenske), Minimum reflux of a multicomponent column (Underwood), Minimum reflux ratio of a binary column (Underwood), Packed absorber: number of transfer units and height (Colburn), Stripper with N equilibrium stages (Kremser), Vapour pressure from the Antoine equation |
| Transport | Constant-pressure cake filtration: filtrate volume after a time, Diffusion of A through stagnant B (Stefan film), Equimolar counterdiffusion flux, Mass-transfer coefficient from a stated Sherwood correlation, Packed-bed pressure drop (Carman correlation), Packed-bed pressure drop (Ergun equation), Specific cake and medium resistance from constant-pressure filtration data |

## 8. Other ways to use it

The same engine runs on the command line (`chemical-engineering`, its own guide), in Python (`pip install chemical-engineering-ai999x`, `import chemical_engineering_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/chemical-engineering-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/chemical-engineering-ai999x/
