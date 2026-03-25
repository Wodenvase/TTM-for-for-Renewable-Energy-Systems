# RESEARCH DATASET & CALIBRATION

## Technology Parameters (2024 Baseline)

Source: **NREL Annual Technology Baseline (ATB) 2024**, BloombergNEF 2025

### Solar Photovoltaic (UTILITY-SCALE)

| Parameter | Value | Unit | Source |
|-----------|-------|------|--------|
| **Cost** | | | |
| Initial CapEx (2024) | 900 | $/kW | NREL ATB'24 Mid scenario |
| Learning exponent | 0.25 | dimensionless | Historical 2010–2024 |
| Reference capacity | 1.0 | GW | Normalization |
| | | | |
| **Performance** | | | |
| Capacity factor (US avg) | 0.25 | — | NREL PVWatts |
| Range by region | 0.20–0.30 | — | Best: SW; Worst: NE |
| | | | |
| **Cost breakdown** | | | |
| Module (50%) | 450 | $/kW | BNEF module pricing |
| BOP—Balance of Plant (30%) | 270 | $/kW | Inverter, wiring, structure |
| Soft costs (20%) | 180 | $/kW | Engineering, permitting, grid |
| | | | |
| **Operating** | | | |
| O&M (% of CapEx/year) | 1.0 | %/yr | NREL avg. fixed |
| Fuel cost | 0 | $/MWh | Renewable |
| Lifespan | 30 | years | Typical warranty |
| Degradation | 0.5 | %/yr | Field data |

**Historical learning rate:**
- 2010–2024: cost declined from $3,800 → $900/kW (~68% reduction)
- Progress ratio (16% cost reduction per doubling) → λ ≈ 0.25

---

### Wind (Onshore)

| Parameter | Value | Unit | Source |
|-----------|-------|------|--------|
| **Cost** | | | |
| Initial CapEx (2024) | 1,300 | $/kW | NREL ATB'24 Mid |
| Learning exponent | 0.15 | dimensionless | Slower learning than solar |
| | | | |
| **Performance** | | | |
| Capacity factor (US avg) | 0.35 | — | NREL WIND toolkit |
| Range by region | 0.25–0.45 | — | High plains > coastal |
| | | | |
| **Operating** | | | |
| O&M (% of CapEx/year) | 2.0 | %/yr | Higher than solar: blades, gearbox |
| Fuel cost | 0 | $/MWh | Renewable |
| Lifespan | 25 | years | Typical warranty |

**Learning plateau:** Wind costs stabilized ~2015 (λ reduced from 0.20 to 0.15)

---

### Wind (Offshore)

| Parameter | Value | Unit | Source |
|-----------|-------|------|--------|
| **Cost** | | | |
| Initial CapEx (2024) | 2,800 | $/kW | NREL ATB'24; European avg |
| Learning exponent | 0.20 | dimensionless | Still learning phase |
| | | | |
| **Performance** | | | |
| Capacity factor (avg) | 0.45 | — | Higher wind resource |
| Range | 0.40–0.50 | — | Shallow vs. deep water |
| | | | |
| **Operating** | | | |
| O&M (% of CapEx/year) | 3.0 | %/yr | Marine environment, specialized ships |
| Lifespan | 25 | years | Same as onshore |

**Development status:** 50 GW installed globally; learning curves still steep.

---

### Natural Gas (CCGT—Combined Cycle Gas Turbine)

| Parameter | Value | Unit | Source |
|-----------|-------|------|--------|
| **Cost** | | | |
| Initial CapEx (2024) | 800 | $/kW | NREL ATB'24 |
| Learning exponent | 0.05 | dimensionless | Mature technology; minimal learning |
| | | | |
| **Performance** | | | |
| Capacity factor (dispatch) | 0.50 | — | Typical average dispatch |
| | | | |
| **Operating** | | | |
| Fuel cost (US Henry Hub) | 40 | $/MWh | 2024 avg ~$2.50/MMBtu |
| Fuel efficiency (heat rate) | 7.5 | MMBtu/MWh | Modern CCGT |
| O&M (% of CapEx/year) | 3.0 | %/yr | Labor, maintenance |
| Lifespan | 30 | years | Durable; often refurbished |
| | | | |
| **Emissions** | | | |
| CO₂ intensity | 0.490 | tCO₂/MWh | Scope 1 only |
| NOx / particulates | regulated | — | Varies by region |

**Cost volatility:** CapEx stable; fuel cost is major driver (2024: $25–60/MWh)

---

### Nuclear (Benchmark—New Fleet)

| Parameter | Value | Unit | Source |
|-----------|-------|------|--------|
| **Cost** | | | |
| Initial CapEx (2024) | 8,000 | $/kW | NREL ATB'24 Conservative |
| Learning exponent | 0.10 | dimensionless | Very slow learning; regulatory burden |
| | | | |
| **Performance** | | | |
| Capacity factor | 0.92 | — | Highest of any technology |
| | | | |
| **Operating** | | | |
| Fuel cost | 8.5 | $/MWh | Uranium + enrichment |
| O&M (% of CapEx/year) | 2.0 | %/yr | Regulatory + security costs |
| Lifespan | 60 | years | Extended with license renewal |
| | | | |
| **Emissions** | | | |
| CO₂ intensity | 0.012 | tCO₂/MWh | Lowest of all options |

**Note:** New nuclear deployment is capital-constrained (high upfront); used as policy benchmark only.

---

## Economic Assumptions

### Discount Rate Analysis

| Scenario | Rate | Rationale | LCOE Impact |
|----------|------|-----------|------------|
| Low (Public/green) | 2% | Sovereign/utility debt | Favors capital-intensive |
| Base (Corporate) | 5% | Typical equity/debt blend | Industry standard |
| High (Private/speculative) | 8–10% | High-risk private investment | Penalizes renewable |

### Impact on Solar LCOE (Q = 10 GW):

- $r = 2\%$: LCOE = $16.44/MWh (CapEx dominates)
- $r = 5\%$: LCOE = $20.85/MWh (base case)
- $r = 10\%$: LCOE = $29.46/MWh (aggressive discounting)

---

## Validation Benchmarks

### 2024 Market LCOE (Actual Bids)

| Technology | Market LCOE | Model | Error |
|------------|-----------|-------|-------|
| Solar PV | $20–30 | $20.8 @10GW | ✓ 0–4% |
| Wind Onshore | $25–35 | $30.1 @10GW | ✓ 0–10% |
| Wind Offshore | $60–100 | $77.4 @10GW | ✓ –5–10% |
| Gas CCGT | $25–45 | $29.3 (static) | ✓ –5–15% (fuel-dependent) |

**Interpretation:** Model calibrated to mid-range; 2024 market includes regional subsidies, tax credits.

---

## Sensitivity Parameter Ranges

For robustness analysis:

| Parameter | Low | Base | High | Rationale |
|-----------|-----|------|------|-----------|
| **Discount rate (%)** | 2 | 5 | 10 | Financing profile |
| **Learning exponent (solar)** | 0.15 | 0.25 | 0.35 | Historical variance ±40% |
| **Learning exponent (wind)** | 0.08 | 0.15 | 0.22 | Manufacturing plateau |
| **Initial CapEx ±%** | –20 | 0 | +20 | Supply chain / scale |
| **Capacity factor ±%** | –15 | 0 | +15 | Regional siting variance |
| **Project lifetime (yrs)** | 15 | 20/25 | 30 | Derating / extension |

---

## Data Lineage & Updates

- **Last updated:** March 2024 (NREL ATB v12.1)
- **Next update:** December 2024 (Bloomberg NEO 2025)
- **Obsolescence risk:** Learning exponents may change if (a) supply chain disruptions, (b) manufacturing breakthroughs, (c) regulatory changes

---

