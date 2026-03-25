# Dynamic LCOE and Transition Time Modelling for Renewable Energy Systems

## Executive Summary

This project develops a computational model of renewable energy economics that goes beyond static LCOE analysis. By integrating **learning curves, capital constraints, and time-dependent dynamics**, the model answers a critical question:

**How fast can an energy system economically transition from fossil fuels to renewables?**

The model is implemented in **Rust** for numerical stability and performance, with Python visualization layers. All assumptions, sensitivity analyses, and limitations are explicitly documented.

---

## 1. Problem Statement

### Static LCOE Limitations

Conventional LCOE analysis treats technology costs as fixed:

$$\text{LCOE}_{\text{static}} = \frac{\sum_{t=0}^{T} \frac{I_t + O_M_t + F_t}{(1+r)^t}}{\sum_{t=0}^{T} \frac{E_t}{(1+r)^t}}$$

**Issues:**
- Does not capture technological learning
- Cannot model feedback between deployment and cost reduction
- Provides no insight into transition dynamics
- Assumes deployment is unconstrained

### Our Approach: Dynamic Modelling

We extend LCOE to capture:

1. **Cost learning curves** – capital costs decline with cumulative deployment
2. **Capacity dynamics** – deployment responds to economic affordability
3. **System constraints** – grid stability, capital availability
4. **Convergence behavior** – can the system sustain transition?

---

## 2. Mathematical Framework

### 2.1 Dynamic LCOE with Learning Curves

Capital cost trajectory follows a learning curve:

$$C(t) = C_0 \left(\frac{Q(t)}{Q_0}\right)^{-\lambda}$$

Where:
- $C_0$: initial capital cost ($/kW)
- $Q(t)$: cumulative installed capacity at time $t$ (GW)
- $Q_0$: reference capacity
- $\lambda$: learning exponent (typical: 0.2–0.4)

**Interpretation:** Each doubling of cumulative capacity reduces costs by factor $2^{-\lambda}$.

### 2.2 Levelized Cost of Energy (Dynamic)

At each time step $t$, LCOE is computed as:

$$\text{LCOE}(t) = \frac{C(t) \cdot \text{CapEx recovery factor} + O\&M(t) + F(t)}{\text{Capacity Factor} \cdot 8760}$$

Where:
- CapEx recovery factor: $\frac{r(1+r)^n}{(1+r)^n - 1}$ (amortization)
- $O\&M(t)$: operations & maintenance costs ($/kW/year)
- $F(t)$: fuel costs (≈0 for renewables)
- Capacity Factor: $\eta$ (efficiency parameter)

### 2.3 Capacity Evolution Dynamics

Deployment follows a capacity diffusion model:

$$\frac{dQ}{dt} = \alpha \cdot \max(0, \text{CapEx}_{\text{budget}} / C(t)) - \delta Q(t)$$

Where:
- $\alpha$: sensitivity to affordability (investment rate constant)
- $\text{CapEx}_{\text{budget}}$: annual capital available for deployment
- $\delta$: retirement/depreciation rate

**Economic interpretation:** Deployment accelerates when LCOE is attractive relative to baseline and capital is available.

### 2.4 Technology Comparison Framework

For each technology $i \in \{\text{PV, Wind, Gas, Nuclear}\}$:

$$\text{LCOE}_i(t) = f_i(C_i(t), Q_i(t), \lambda_i, \eta_i, r)$$

Technologies compete on:
1. **Cost convergence time** $t_{i}^*$: when LCOE_i < LCOE_baseline
2. **Learning aggressiveness** $\lambda_i$
3. **Space/grid constraints** embodied in max $Q_i$

### 2.5 System Transition Metric

Define penetration as:

$$\text{Penetration}(t) = \frac{\sum_{i \in \text{renewables}} Q_i(t)}{\sum_{j} Q_j(t)}$$

**Key variable:** Time $T_{80\%}$ to reach 80% renewable penetration.

Transition stability:

$$\forall t > T_{80\%}, \quad \frac{d(\text{Penetration})}{dt} > 0 \quad \text{(or stays plateaued)}$$

---

## 3. Implementation Architecture

### 3.1 Core Modules

```
src/
├── lib.rs                  # Library root
├── models/
│   ├── technology.rs       # Technology struct & learning curves
│   ├── lcoe.rs             # LCOE calculation (static & dynamic)
│   ├── dynamics.rs         # ODE solver for capacity evolution
│   └── system.rs           # Multi-technology system model
├── analysis/
│   ├── sensitivity.rs      # Sensitivity analysis engine
│   ├── convergence.rs      # Transition metrics
│   └── crossover.rs        # Technology crossover detection
├── solvers/
│   └── ode.rs              # RK4 / adaptive ODE integrator
└── main.rs                 # CLI entry point
```

### 3.2 Technology Struct

```rust
pub struct Technology {
    pub name: String,
    pub initial_cost: f64,        // $/kW
    pub learning_exponent: f64,   // λ
    pub capacity_factor: f64,     // η (0-1)
    pub om_cost: f64,             // $/kW/year
    pub fuel_cost: f64,           // $/MWh
    pub max_capacity: f64,        // GW (constraint)
    pub discount_rate: f64,       // r
    pub initial_capacity: f64,    // GW (starting point)
}
```

### 3.3 Solver Strategy

- **ODE Integration:** RK4 with adaptive stepping
- **LCOE Recalculation:** At each timestep, recompute costs from learning curve
- **Convergence Detection:** Stop when transition metrics saturate
- **Output:** Time series of (Q, LCOE, Penetration)

---

## 4. Key Analyses

### 4.1 Crossover Time Analysis

**Question:** When does renewable LCOE < fossil baseline?

**Method:** Solve $\text{LCOE}_{\text{renewable}}(t^*) = \text{LCOE}_{\text{fossil}}$ numerically.

**Output:** $t^* \in \mathbb{R}_+$ (years from baseline)

**Sensitivity:** Vary learning rate $\lambda$, discount rate $r$, initial capacity factor.

### 4.2 Sensitivity Analysis

Vary key parameters:
- Learning exponent: $\lambda \in [0.1, 0.5]$
- Discount rate: $r \in [2\%, 10\%]$
- Initial capital costs: $\pm 20\%$
- Capacity factor uncertainty: $\pm 15\%$

**Output:** Tornado plots showing dominating factors.

### 4.3 Transition Speed Analysis

Track cumulative penetration over time:
$$T_{p\%} = \min \{t : \text{Penetration}(t) \geq p\%\}$$

For $p \in \{50\%, 70\%, 80\%, 90\%\}$, compute $T_p$.

**Interpretation:** Convexity in $T_p$ signals transition acceleration or stalling.

### 4.4 Policy Control Loop (Optional)

Extend model to include carbon pricing:

$$\text{effective cost} = \text{LCOE} + \text{carbon price} \times \text{emissions intensity}$$

Measure how policy accelerates transition by reducing $T_{80\%}$.

---

## 5. Data & Parameters

### Technology Baseline (2024)

| Technology | Initial Cost | Learning Rate | Capacity Factor | O&M (% of CapEx) |
|------------|------|-------|--------|---------|
| Solar PV   | 900  | 0.25  | 0.25   | 1%      |
| Wind Onshore | 1300 | 0.15  | 0.35   | 2%      |
| Wind Offshore | 2800 | 0.20 | 0.45   | 3%      |
| Gas (CCGT) | 800  | 0.05  | 0.50   | 3%      |
| Nuclear    | 8000 | 0.10  | 0.92   | 2%      |

**Sources:**
- NREL 2024 ATB (Annual Technology Baseline)
- IPCC AR6 WG3
- BloombergNEF NEO 2025

### Economic Assumptions

- **Discount rate:** 5% (default)
- **Project lifetime:** 20 years
- **Reference capacity:** 1 GW
- **Annual CapEx budget:** adjustable (policy variable)

---

## 6. Expected Results & Inferences

### 6.1 Technology Crossover Dynamics

**Hypothesis:** Solar LCOE crosses gas LCOE first (most aggressive learning curve).

**Expected finding:**
- Solar-gas crossover: ~2-3 years (under 5% discount)
- Wind-gas crossover: ~5-7 years
- Offshore-gas crossover: >10 years

### 6.2 Transition Acceleration Effect

**Hypothesis:** Once renewable LCOE < fossil, deployment accelerates nonlinearly.

**Expected finding:**
- Slow phase (0-5 yrs): capacity doubles slowly, learning curve shallow
- Acceleration phase (5-15 yrs): exponential deployment feedback
- Saturation phase (15+ yrs): grid constraints bind, growth slows

### 6.3 Policy Sensitivity

**Hypothesis:** Carbon pricing has stronger effect on low-learning technologies.

**Expected finding:**
- High-learning (solar): minimal policy needed
- Low-learning (nuclear): requires meaningful carbon price (>$100/tCO2e)

---

## 7. Usage

### Build

```bash
cargo build --release
```

### Run Base Case

```bash
cargo run --release -- --config base_case.json --output results/
```

### Output

```
results/
├── lcoe_trajectory.csv      # Time series of LCOE for each tech
├── penetration_trajectory.csv # Renewable penetration %
├── capacity_trajectory.csv   # Installed capacity GW
├── crossover_times.json      # Technology crossover moments
└── sensitivity_analysis.json # Parameter sweep results
```

### Visualize (Python)

```bash
python plots/visualize.py results/
```

Generates:
- LCOE convergence plots
- Capacity growth curves
- Penetration trajectories
- Sensitivity tornado plots
- Phase diagrams

---

## 8. Limitations & Honest Assessment

1. **Grid stability:** Model does not capture intra-hour matching or storage costs
2. **Policy exogeneity:** Carbon pricing treated as external parameter (not endogenous)
3. **Technology independence:** Does not model cannibalization (wind farm density limits)
4. **Financial realism:** Assumes perfect capital markets (no credit constraints for small players)
5. **Demand growth:** Assumes constant demand (electricity growth not modeled)

### Appropriate Cautions

This model is **illustrative of economically-driven transition**, not predictive of real-world timelines. Policy, geopolitics, supply chain, and social factors are not captured.

---

## 9. References

### Core Literature

[1] **Wright, T. P.** (1936). *Factors Affecting the Cost of Airplanes.* Journal of the Aeronautical Sciences, 3(4), 122-128.
   - Foundational learning curve theory (experience curve, progress ratio)

[2] **Nordhaus, W. D.** (2007). *Two centuries of productivity growth in computing.* The Journal of Economic History, 67(1), 128-159.
   - Application of learning curves to technology cost reduction

[3] **Lazard.** (2024). *Lazard's Levelized Cost of Energy Analysis – Version 18.0.*
   - Industry-standard LCOE benchmarks

[4] **NREL Annual Technology Baseline (ATB) 2024.** National Renewable Energy Laboratory.
   - Cost and performance projections for energy technologies

[5] **IPCC.** (2022). *Climate Change 2022: Mitigation of Climate Change.* Contribution of Working Group III.
   - Section on renewable energy transition economics and timelines

[6] **BloombergNEF.** (2025). *New Energy Outlook 2025.*
   - Market data on renewable deployment and cost trajectories

[7] **Sterner, E.** (2002). *The Economics of Climate Change.* American Economic Review.
   - Discount rate implications for long-horizon climate investments

[8] **van Sark, G., et al.** (2012). *Accuracy of progress ratio and learning curve assumptions for renewable energy technologies.* Renewable Energy, 49, 90-95.
   - Critical assessment of learning rate variability

[9] **Schmidt, O., Melchior, S., Hawkes, A., & Staffell, I.** (2019). *Projecting the future levelized cost of electricity storage technologies.* Joule, 3(1), 81-100.
   - Methods for extending LCOE to emerging technologies

### Complementary References

[10] Gagnon, P., et al. (2020). *Cost of Wind Energy Review.* NREL Technical Report.

[11] Fu, R., et al. (2018). *Photovoltaic Degradation Rates – An Analytical Review.* NREL Technical Report.

[12] Davis, S. J., et al. (2018). *Net-zero emissions energy systems.* Science, 360(6396), eaas9793.

---

## Project Structure & Reproducibility

All computations are deterministic. To reproduce:

1. Clone repository
2. Install Rust (1.70+)
3. Run `cargo build --release`
4. Run with bundled parameters: `cargo run --release -- --profile research`
5. Visualize: `python plots/visualize.py`

All data inputs and model configurations are version-controlled (JSON format).

---

## Author

Dipanta Bhattacharyya
Computational Energy Systems Research
2026

---

## License

MIT License – See LICENSE file

---

## Contact & Contributions

For questions or extensions:
- Open an issue on the repository
- Extensions: battery storage costs, demand growth, political constraints

---

