# MATHEMATICAL FORMULATIONS

## Core LCOE Model

### Levelized Cost of Energy (LCOE)

The levelized cost is the average cost per unit of electricity produced over a technology's lifetime, discounted to present value:

$$\text{LCOE} = \frac{\text{CapEx Recovery} + \text{O\&M} + \text{Fuel}}{\text{Annual Generation}}$$

**Expanded form:**

$$\text{LCOE} = \frac{C(t) \cdot CRF(r, n) + O\&M(t) + F}{\eta \cdot 8760 \text{ hours}}$$

Where:
- $C(t)$ = Capital cost at time $t$ ($/kW)
- $CRF(r, n)$ = Capital recovery factor
- $O\&M(t)$ = Annual O&M cost ($/kW/year)
- $F$ = Fuel cost ($/MWh)
- $\eta$ = Capacity factor (0–1)

### Capital Recovery Factor

For a given discount rate and project lifetime:

$$CRF(r, n) = \frac{r(1+r)^n}{(1+r)^n - 1}$$

Where:
- $r$ = Discount rate (annual)
- $n$ = Project lifetime (years)

**Economic interpretation:** Converts a lump-sum capital investment into an equivalent annuity.

### Learning Curve (Wright's Law)

Cost reduction with cumulative deployment follows:

$$C(t) = C_0 \left(\frac{Q(t)}{Q_0}\right)^{-\lambda}$$

Where:
- $C_0$: Initial cost per unit ($/kW)
- $Q(t)$: Cumulative deployed capacity (GW)
- $Q_0$: Reference capacity (1 GW)
- $\lambda$: Learning exponent (progress ratio = $2^{-\lambda}$)

**Key property:** Learning exponent $\lambda = \log_2(1 - d) \approx -d$ for small $d$

For $\lambda = 0.25$ (solar):
- Progress ratio = $2^{-0.25} \approx 0.84$
- Each doubling of capacity reduces cost by 16%

### Economic Dynamics: Capacity Evolution

Installed capacity evolves according to:

$$\frac{dQ}{dt} = \alpha \cdot \max(0, \text{economic signal}) - \delta Q(t)$$

**Simplified form** (linear deployment model):

$$Q(t) = Q_0 e^{-\delta t} + \frac{\alpha \cdot E}{1 + \delta} \left(1 - e^{-(\delta + \beta t)}\right)$$

Where:
- $\alpha$ = Investment response coefficient (GW/year/$B)
- $E(t)$ = Economic affordability (budget / unit cost)
- $\delta$ = Asset retirement rate (annual)
- $\beta$ = Acceleration of investment response

### LCOE Crossover

Technology transition occurs when LCOE curves intersect. Solve:

$$\text{LCOE}_{\text{renewable}}(Q^*) = \text{LCOE}_{\text{fossil}}(Q^*)$$

$$C_{\text{REN}}(Q^*) \cdot CRF + O\&M_{\text{REN}} = C_{\text{FOSSIL}} \cdot CRF + O\&M_{\text{FOSSIL}} + F_{\text{FOSSIL}}$$

Substituting learning curve:

$$C_{R,0} \left(\frac{Q^*}{Q_0}\right)^{-\lambda_R} \cdot CRF + O\&M_R = C_F \cdot CRF + O\&M_F + F_F$$

**Qualitative result:** Crossover occurs earlier if:
- $\lambda_R$ is larger (aggressive learning)
- $C_{R,0}$ is smaller (low initial cost)
- Discount rate is lower (favors long-term returns)

---

## Sensitivity Analysis

### Parameter Perturbation Method

For a parameter $p$, compute elasticity:

$$\epsilon = \frac{\partial \ln(\text{LCOE})}{\partial \ln(p)} = \frac{p}{\text{LCOE}} \cdot \frac{\partial \text{LCOE}}{\partial p}$$

**Interpretation:** 1% change in $p$ → $\epsilon$% change in LCOE.

### Discount Rate Sensitivity

For fixed CapEx, O&M, and lifetime:

$$\frac{\partial \text{LCOE}}{\partial r} = C \cdot \frac{\partial CRF}{\partial r}$$

$$\frac{\partial CRF}{\partial r}\bigg|_{r \approx 0} \approx \frac{n^2}{12}$$

**Example:** For 20-year project at $r = 5\%$:
- 1% increase in discount rate → ~1.8% increase in LCOE

### Learning Exponent Sensitivity

Cost at cumulative $Q$:

$$\frac{\partial C}{\partial \lambda} = -C_0 \left(\frac{Q}{Q_0}\right)^{-\lambda} \ln\left(\frac{Q}{Q_0}\right)$$

At $Q = 10 \text{ GW}$, $\lambda = 0.25$ (solar):

$$\frac{\partial \ln(C)}{\partial \lambda} \approx 2.3 \text{ (large sensitivity)}$$

**Interpretation:** Learning curves are the dominant parameter in technology cost projections.

---

## System Integration

### Penetration Metric

Renewable energy penetration:

$$P(t) = \frac{\sum_i^{\text{renewables}} Q_i(t)}{\sum_j^{\text{all}} Q_j(t)} \times 100\%$$

### Transition Speed Analysis

Define transition timescales:
- $T_{50}$: Time to 50% penetration
- $T_{80}$: Time to 80% penetration

Acceleration indicator:

$$\text{convexity} = \frac{T_{80} - T_{50}}{T_{50}} < 1 \text{ (accelerating)}$$

### Carbon Cost Integration

Effective LCOE with carbon pricing:

$$\text{LCOE}_{\text{eff}} = \text{LCOE} + P_{\text{carbon}} \times I_{\text{carbon}}$$

Where:
- $P_{\text{carbon}}$ = Carbon price ($/tCO₂e)
- $I_{\text{carbon}}$ = Carbon intensity (tCO₂e/MWh)

**Example** (Solar at $P_c = \$100$/tCO₂e):
- Carbon penalty: $0.048 \times 100 = \$4.80/\text{MWh}$ (negligible)

**Example** (Gas at $P_c = \$100$/tCO₂e):
- Carbon penalty: $0.490 \times 100 = \$49.0/\text{MWh}$ (dominant)

---

## Numerical Solutions

### Runge-Kutta 4th Order (RK4)

For ODE $\frac{dQ}{dt} = f(t, Q)$:

$$Q_{n+1} = Q_n + \frac{\Delta t}{6}(k_1 + 2k_2 + 2k_3 + k_4)$$

Where:
- $k_1 = f(t_n, Q_n)$
- $k_2 = f(t_n + \frac{\Delta t}{2}, Q_n + \frac{\Delta t}{2}k_1)$
- $k_3 = f(t_n + \frac{\Delta t}{2}, Q_n + \frac{\Delta t}{2}k_2)$
- $k_4 = f(t_n + \Delta t, Q_n + \Delta t k_3)$

**Accuracy:** $O(\Delta t^5)$ local error, $O(\Delta t^4)$ global error.

### Stability Condition

For capacity diffusion with constant source:

$$\frac{dQ}{dt} = \alpha E - \delta Q$$

**Solution:** $Q(t) = \frac{\alpha E}{\delta} (1 - e^{-\delta t}) + Q_0 e^{-\delta t}$

**Steady state:** $Q_\infty = \frac{\alpha E}{\delta}$ (independent of initial condition).

---

## References

1. Wright, T. P. (1936). *Factors Affecting the Cost of Airplanes.* Journal of the Aeronautical Sciences, 3(4), 122–128.
   - Original learning curve formulation

2. Nordhaus, W. D. (2007). *Two Centuries of Productivity Growth in Computing.* Journal of Economic History, 67(1), 128–159.
   - Application to information technology

3. Junginger, M., & Faaij, A. (2005). *Technological Learning in the Energy Sector.* International Journal of Energy Technology & Policy, 3(5–6), 487–512.
   - Energy-specific learning rates

4. van Sark, G. J. M., et al. (2012). *Accuracy of Progress Ratio and Learning Curve Assumptions for Renewable Energy Technologies.* Renewable Energy, 49, 90–95.
   - Empirical validation of learning exponents

5. Schmidt, O., Melchior, S., Hawkes, A., & Staffell, I. (2019). *Projecting the Future Levelized Cost of Electricity Storage Technologies.* Joule, 3(1), 81–100.
   - Extended LCOE framework for storage

