// src/models/technology.rs
// Technology definition and learning curve model

use serde::{Deserialize, Serialize};

/// Represents an energy technology with cost and performance characteristics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Technology {
    /// Technology name
    pub name: String,

    /// Initial capital cost ($/kW) at baseline year
    pub initial_capex: f64,

    /// Learning exponent λ (typical: 0.15-0.40)
    /// Cost reduction factor per doubling of cumulative capacity
    pub learning_exponent: f64,

    /// Reference capacity for learning curve (GW)
    pub reference_capacity: f64,

    /// Capacity factor η (0-1)
    pub capacity_factor: f64,

    /// Operations & Maintenance cost as % of CapEx per year
    pub om_percentage: f64,

    /// Fuel cost ($/MWh) - 0 for renewables
    pub fuel_cost: f64,

    /// Maximum deployable capacity (GW) - resource/grid constraint
    pub max_capacity: f64,

    /// Retirement/depreciation rate (per year)
    pub depreciation_rate: f64,

    /// Initial deployed capacity (GW)
    pub initial_capacity: f64,

    /// Discount rate for amortization (default from SimConfig)
    pub discount_rate: f64,

    /// Project lifetime (years)
    pub project_lifetime: i32,
}

impl Technology {
    /// Compute capital cost at time t given cumulative capacity Q(t)
    /// C(t) = C_0 * (Q(t) / Q_0)^(-λ)
    pub fn capital_cost_at(&self, cumulative_capacity: f64) -> f64 {
        let ratio = cumulative_capacity / self.reference_capacity;
        self.initial_capex * ratio.powf(-self.learning_exponent)
    }

    /// Annual O&M cost ($/kW/year)
    pub fn om_cost_annual(&self, cumulative_capacity: f64) -> f64 {
        let capex = self.capital_cost_at(cumulative_capacity);
        capex * self.om_percentage
    }

    /// Compute CapEx recovery factor (amortization coefficient)
    /// r(1+r)^n / ((1+r)^n - 1)
    pub fn capex_recovery_factor(&self) -> f64 {
        let r = self.discount_rate;
        let n = self.project_lifetime as f64;
        let factor = (1.0 + r).powf(n);
        r * factor / (factor - 1.0)
    }

    /// Total annual fixed cost per kW (CapEx + O&M)
    pub fn fixed_cost_annual(&self, cumulative_capacity: f64) -> f64 {
        let capex = self.capital_cost_at(cumulative_capacity);
        let recovery = self.capex_recovery_factor();
        capex * recovery + self.om_cost_annual(cumulative_capacity)
    }

    /// Energy generation per kW per year (MWh/kW/year)
    pub fn annual_generation_per_kw(&self) -> f64 {
        self.capacity_factor * 8760.0 / 1000.0 // Convert MWh to GWh
    }

    /// Validate technology parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.capacity_factor < 0.0 || self.capacity_factor > 1.0 {
            return Err("Capacity factor must be in [0, 1]".to_string());
        }
        if self.learning_exponent < 0.0 {
            return Err("Learning exponent must be non-negative".to_string());
        }
        if self.initial_capex <= 0.0 {
            return Err("Initial CapEx must be positive".to_string());
        }
        if self.project_lifetime <= 0 {
            return Err("Project lifetime must be positive".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_curve() {
        let tech = Technology {
            name: "Solar PV".to_string(),
            initial_capex: 1000.0,
            learning_exponent: 0.25,
            reference_capacity: 1.0,
            capacity_factor: 0.25,
            om_percentage: 0.01,
            fuel_cost: 0.0,
            max_capacity: 500.0,
            depreciation_rate: 0.05,
            initial_capacity: 0.1,
            discount_rate: 0.05,
            project_lifetime: 20,
        };

        // At Q=1: cost should be C_0
        assert!((tech.capital_cost_at(1.0) - 1000.0).abs() < 1.0);

        // At Q=2: cost should be C_0 * 2^(-λ)
        let expected = 1000.0 * 2.0_f64.powf(-0.25);
        assert!((tech.capital_cost_at(2.0) - expected).abs() < 1.0);
    }

    #[test]
    fn test_capex_recovery_factor() {
        let tech = Technology {
            name: "Test".to_string(),
            initial_capex: 1000.0,
            learning_exponent: 0.0,
            reference_capacity: 1.0,
            capacity_factor: 0.5,
            om_percentage: 0.02,
            fuel_cost: 0.0,
            max_capacity: 100.0,
            depreciation_rate: 0.05,
            initial_capacity: 1.0,
            discount_rate: 0.05,
            project_lifetime: 20,
        };

        // Recovery factor should be positive and less than annual discount rate
        let rf = tech.capex_recovery_factor();
        assert!(rf > 0.0 && rf < 0.1);
    }
}
