// src/models/lcoe.rs
// LCOE calculation (static and dynamic)

use crate::models::Technology;

/// Computes Levelized Cost of Energy
pub struct LcoeCalculator;

impl LcoeCalculator {
    /// Compute static LCOE (assumes fixed costs)
    /// LCOE = (CapEx recovery + O&M + Fuel) / (Capacity Factor * 8760)
    /// Result in $/MWh
    pub fn static_lcoe(tech: &Technology) -> f64 {
        let capex = tech.initial_capex;
        let recovery = tech.capex_recovery_factor();
        let fixed_annual = capex * recovery + tech.om_cost_annual(tech.reference_capacity);
        let generation = tech.capacity_factor * 8760.0; // MWh per kW per year

        (fixed_annual + tech.fuel_cost) / (generation / 1000.0) // Convert to $/MWh
    }

    /// Compute dynamic LCOE at cumulative capacity Q(t)
    /// Incorporates learning curve effects
    pub fn dynamic_lcoe(tech: &Technology, cumulative_capacity: f64) -> f64 {
        let capex = tech.capital_cost_at(cumulative_capacity);
        let recovery = tech.capex_recovery_factor();
        let om_annual = tech.om_cost_annual(cumulative_capacity);
        let fixed_annual = capex * recovery + om_annual;
        let generation = tech.capacity_factor * 8760.0; // MWh per kW per year

        (fixed_annual + tech.fuel_cost) / (generation / 1000.0) // $/MWh
    }

    /// Find time t when LCOE of tech1 equals LCOE of tech2
    /// Assumes linear interpolation of learning curves
    /// Returns (cumulative_capacity_at_crossover, years_to_crossover)
    pub fn find_lcoe_crossover(
        tech1: &Technology,
        tech2: &Technology,
        max_capacity: f64,
        timestep: f64,
    ) -> Option<(f64, f64)> {
        let mut q = 0.01; // Start with small cumulative capacity
        let mut time = 0.0;

        while q < max_capacity {
            let lcoe1 = Self::dynamic_lcoe(tech1, q);
            let lcoe2 = Self::dynamic_lcoe(tech2, q);

            if (lcoe1 - lcoe2).abs() < 0.5 {
                // Crossover found (within $0.50/MWh)
                return Some((q, time));
            }

            if lcoe1 < lcoe2 {
                // tech1 is cheaper, stop searching
                return Some((q, time));
            }

            q *= 1.1; // Exponential search
            time += timestep;

            if time > 100.0 {
                // Timeout: no crossover in 100 years
                return None;
            }
        }

        None
    }

    /// Levelized carbon intensity (tCO2/MWh)
    /// Placeholder: actual values depend on technology
    pub fn carbon_intensity(tech_name: &str) -> f64 {
        match tech_name.to_lowercase().as_str() {
            "solar pv" => 0.048,
            "wind onshore" => 0.011,
            "wind offshore" => 0.012,
            "gas ccgt" => 0.490,
            "nuclear" => 0.012,
            _ => 0.0,
        }
    }

    /// Effective cost including carbon price
    /// effective_cost = LCOE + carbon_price * carbon_intensity
    pub fn cost_with_carbon_price(tech: &Technology, cumulative_capacity: f64, carbon_price: f64) -> f64 {
        let lcoe = Self::dynamic_lcoe(tech, cumulative_capacity);
        let carbon_cost = carbon_price * Self::carbon_intensity(&tech.name);
        lcoe + carbon_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tech() -> Technology {
        Technology {
            name: "Solar PV".to_string(),
            initial_capex: 900.0,
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
        }
    }

    #[test]
    fn test_static_lcoe() {
        let tech = sample_tech();
        let lcoe = LcoeCalculator::static_lcoe(&tech);
        assert!(lcoe > 0.0);
        assert!(lcoe < 150.0); // Solar should be relatively cheap
    }

    #[test]
    fn test_dynamic_lcoe_decreases_with_learning() {
        let tech = sample_tech();
        let lcoe_early = LcoeCalculator::dynamic_lcoe(&tech, 1.0);
        let lcoe_late = LcoeCalculator::dynamic_lcoe(&tech, 100.0);

        assert!(lcoe_late < lcoe_early, "Learning should reduce LCOE");
    }

    #[test]
    fn test_carbon_intensity() {
        assert_eq!(LcoeCalculator::carbon_intensity("solar pv"), 0.048);
        assert_eq!(LcoeCalculator::carbon_intensity("gas ccgt"), 0.490);
    }
}
