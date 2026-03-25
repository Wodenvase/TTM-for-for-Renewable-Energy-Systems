// src/models/system.rs
// Multi-technology energy system model

use crate::models::Technology;
use std::collections::HashMap;

/// Represents a system of multiple energy technologies
#[derive(Clone, Debug)]
pub struct System {
    pub technologies: HashMap<String, Technology>,
    pub annual_capex_budget: f64, // Dollars per year for total deployment
}

impl System {
    pub fn new(budget: f64) -> Self {
        System {
            technologies: HashMap::new(),
            annual_capex_budget: budget,
        }
    }

    pub fn add_technology(&mut self, tech: Technology) {
        tech.validate().expect("Invalid technology");
        self.technologies.insert(tech.name.clone(), tech);
    }

    /// Compute total installed capacity across all technologies
    pub fn total_capacity(&self) -> f64 {
        self.technologies
            .values()
            .map(|t| t.initial_capacity)
            .sum()
    }

    /// Compute renewable penetration (%)
    pub fn renewable_penetration(&self) -> f64 {
        let renewables = vec!["Solar PV", "Wind Onshore", "Wind Offshore"];
        let renewable_cap: f64 = self
            .technologies
            .values()
            .filter(|t| renewables.iter().any(|r| t.name.contains(r)))
            .map(|t| t.initial_capacity)
            .sum();

        let total = self.total_capacity();
        if total > 0.0 {
            100.0 * renewable_cap / total
        } else {
            0.0
        }
    }

    /// Validate all technologies in system
    pub fn validate(&self) -> Result<(), String> {
        if self.technologies.is_empty() {
            return Err("System has no technologies".to_string());
        }
        for tech in self.technologies.values() {
            tech.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_creation() {
        let mut system = System::new(50e9); // $50B annual budget

        let solar = Technology {
            name: "Solar PV".to_string(),
            initial_capex: 900.0,
            learning_exponent: 0.25,
            reference_capacity: 1.0,
            capacity_factor: 0.25,
            om_percentage: 0.01,
            fuel_cost: 0.0,
            max_capacity: 500.0,
            depreciation_rate: 0.05,
            initial_capacity: 10.0,
            discount_rate: 0.05,
            project_lifetime: 20,
        };

        system.add_technology(solar);
        assert_eq!(system.total_capacity(), 10.0);
    }

    #[test]
    fn test_renewable_penetration() {
        let mut system = System::new(50e9);

        system.add_technology(Technology {
            name: "Solar PV".to_string(),
            initial_capex: 900.0,
            learning_exponent: 0.25,
            reference_capacity: 1.0,
            capacity_factor: 0.25,
            om_percentage: 0.01,
            fuel_cost: 0.0,
            max_capacity: 500.0,
            depreciation_rate: 0.05,
            initial_capacity: 50.0,
            discount_rate: 0.05,
            project_lifetime: 20,
        });

        system.add_technology(Technology {
            name: "Gas CCGT".to_string(),
            initial_capex: 800.0,
            learning_exponent: 0.05,
            reference_capacity: 1.0,
            capacity_factor: 0.5,
            om_percentage: 0.03,
            fuel_cost: 40.0,
            max_capacity: 300.0,
            depreciation_rate: 0.05,
            initial_capacity: 50.0,
            discount_rate: 0.05,
            project_lifetime: 20,
        });

        let penetration = system.renewable_penetration();
        assert_eq!(penetration, 50.0); // 50 GW solar / 100 GW total
    }
}
