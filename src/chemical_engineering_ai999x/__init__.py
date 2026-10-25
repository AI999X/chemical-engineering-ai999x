"""chemical-engineering-ai999x — offline chemical engineering answers with their formula, steps and verification.

    import chemical_engineering_ai999x as m
    m.ask("First-order CSTR volume: v0 = 36 m^3/h, k = 0.06 1/min, X = 90%").show()      # V = 90 m^3, tau = 9000 s
    m.ask("Combustion flue gas: fuel = CH4, excess_air = 20%").show()                     # 11.43 mol air/mol, 9.59 % CO2 dry
    m.solve("chemeng_mccabe_thiele_stages", alpha=2.5, zF=0.5, xD=0.95, q=1, xB=0.05, R=2)   # N = 11 stages, R_min = 1.1
    m.capabilities()                   # every certified capability

83 certified capabilities: material balances (blending, two-product separators, recycle with purge, unsteady and
variable-volume tanks, combustion air and flue-gas analysis, mass to mole fractions), energy balances (sensible heat
and Kirchhoff heats of reaction with polynomial heat capacities, adiabatic reactor and flame temperatures, batch heating
and coil cooling), reactor design (batch, CSTR and PFR design for first- and second-order kinetics, gas-phase PFRs with
volume change, CSTRs in series, the recycle reactor, reversible and series reactions, residence-time distribution
moments), separations (Antoine vapour pressure and boiling point, Raoult bubble and dew points, Rachford-Rice, ideal and
Wilson K-value flashes, Underwood minimum reflux, McCabe-Thiele stages, Kremser absorbers and strippers, transfer units,
countercurrent and crosscurrent extraction, Wilson, NRTL and UNIQUAC activity coefficients), property estimation
(Peng-Robinson and SRK equations of state, Lee-Kesler vapour pressure, Watson, Riedel and Pitzer heats of vaporisation,
Rackett liquid volume, virial compressibility), transport and particle beds (diffusion through a stagnant film, Ergun
and Carman packed-bed pressure drop, cake filtration and filtration-test analysis), process control (first-order-plus-
dead-time response, P-only offset, frequency response of lags, discrete PID output, Ziegler-Nichols tuning, closed-loop
stability, PI-loop gain and phase margins, step-test fits) and reservoir and petroleum engineering (radial Darcy inflow
and productivity index, exponential, hyperbolic and harmonic decline, Havlena-Odeh oil in place with and without a gas
cap, the gas p/z plot, Horner build-up analysis, the line-source drawdown, gas-well back-pressure tests).

Questions use the form "<capability phrase>: name = value, ..." (see templates()); physical values carry their units
(psi, bar g, degF, BTU, lb, bbl, STB, md, cP and acre are converted), fractions may be written with %, lists in
brackets, and conventions (Antoine units, list units, controller type) are stated, never assumed. Built by the AI999X
model machine from the same engine as every domain; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("chemical_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
