// src/analysis/sensitivity.rs
// Sensitivity analysis for LCOE and transition metrics

use crate::models::Technology;
use crate::models::lcoe::LcoeCalculator;

/// Results of parameter sensitivity analysis
#[derive(Clone, Debug)]
pub struct SensitivityResult {
    pub parameter_name: String,
    pub base_value: f64,
    pub low_value: f64,
    pub high_value: f64,
    pub lcoe_at_low: f64,
    pub lcoe_at_high: f64,
    pub sensitivity_pct: f64, // (lcoe_high - lcoe_low) / lcoe_base * 100
}

/// Sensitivity analysis engine
pub struct SensitivityAnalysis;

impl SensitivityAnalysis {
    /// Analyze sensitivity of LCOE to discount rate
    pub fn sensitivity_discount_rate(tech: &Technology, cumulative_capacity: f64) -> SensitivityResult {
        let base_lcoe = LcoeCalculator::dynamic_lcoe(tech, cumulative_capacity);

        let mut tech_low = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex,
            learning_exponent: tech.learning_exponent,
            reference_capacity: tech.reference_capacity,
            capacity_factor: tech.capacity_factor,
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: 0.02,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_low = LcoeCalculator::dynamic_lcoe(&tech_low, cumulative_capacity);

        let mut tech_high = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex,
            learning_exponent: tech.learning_exponent,
            reference_capacity: tech.reference_capacity,
            capacity_factor: tech.capacity_factor,
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: 0.10,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_high = LcoeCalculator::dynamic_lcoe(&tech_high, cumulative_capacity);

        let sensitivity_pct = ((lcoe_high - lcoe_low) / base_lcoe) * 100.0;

        SensitivityResult {
            parameter_name: "Discount Rate".to_string(),
            base_value: tech.discount_rate,
            low_value: 0.02,
            high_value: 0.10,
            lcoe_at_low: lcoe_low,
            lcoe_at_high: lcoe_high,
            sensitivity_pct,
        }
    }

    /// Analyze sensitivity of LCOE to learning exponent
    pub fn sensitivity_learning_exponent(tech: &Technology, cumulative_capacity: f64) -> SensitivityResult {
        let base_lcoe = LcoeCalculator::dynamic_lcoe(tech, cumulative_capacity);

        let mut tech_low = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex,
            learning_exponent: tech.learning_exponent * 0.5,
            reference_capacity: tech.reference_capacity,
            capacity_factor: tech.capacity_factor,
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: tech.discount_rate,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_low = LcoeCalculator::dynamic_lcoe(&tech_low, cumulative_capacity);

        let mut tech_high = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex,
            learning_exponent: tech.learning_exponent * 1.5,
            reference_capacity: tech.reference_capacity,
            capacity_factor: tech.capacity_factor,
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: tech.discount_rate,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_high = LcoeCalculator::dynamic_lcoe(&tech_high, cumulative_capacity);

        let sensitivity_pct = ((lcoe_high - lcoe_low) / base_lcoe) * 100.0;

        SensitivityResult {
            parameter_name: "Learning Exponent".to_string(),
            base_value: tech.learning_exponent,
            low_value: tech.learning_exponent * 0.5,
            high_value: tech.learning_exponent * 1.5,
            lcoe_at_low: lcoe_low,
            lcoe_at_high: lcoe_high,
            sensitivity_pct,
        }
    }

    /// Analyze sensitivity to initial CapEx
    pub fn sensitivity_initial_capex(tech: &Technology, cumulative_capacity: f64) -> SensitivityResult {
        let base_lcoe = LcoeCalculator::dynamic_lcoe(tech, cumulative_capacity);

        let mut tech_low = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex * 0.8,
            learning_exponent: tech.learning_exponent,
            reference_capacity: tech.reference_capacity,
            capacity_factor: tech.capacity_factor,
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: tech.discount_rate,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_low = LcoeCalculator::dynamic_lcoe(&tech_low, cumulative_capacity);

        let mut tech_high = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex * 1.2,
            learning_exponent: tech.learning_exponent,
            reference_capacity: tech.reference_capacity,
            capacity_factor: tech.capacity_factor,
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: tech.discount_rate,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_high = LcoeCalculator::dynamic_lcoe(&tech_high, cumulative_capacity);

        let sensitivity_pct = ((lcoe_high - lcoe_low) / base_lcoe) * 100.0;

        SensitivityResult {
            parameter_name: "Initial CapEx".to_string(),
            base_value: tech.initial_capex,
            low_value: tech.initial_capex * 0.8,
            high_value: tech.initial_capex * 1.2,
            lcoe_at_low: lcoe_low,
            lcoe_at_high: lcoe_high,
            sensitivity_pct,
        }
    }

    /// Analyze sensitivity to capacity factor
    pub fn sensitivity_capacity_factor(tech: &Technology, cumulative_capacity: f64) -> SensitivityResult {
        let base_lcoe = LcoeCalculator::dynamic_lcoe(tech, cumulative_capacity);

        let mut tech_low = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex,
            learning_exponent: tech.learning_exponent,
            reference_capacity: tech.reference_capacity,
            capacity_factor: (tech.capacity_factor * 0.85).min(1.0),
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: tech.discount_rate,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_low = LcoeCalculator::dynamic_lcoe(&tech_low, cumulative_capacity);

        let mut tech_high = Technology {
            name: tech.name.clone(),
            initial_capex: tech.initial_capex,
            learning_exponent: tech.learning_exponent,
            reference_capacity: tech.reference_capacity,
            capacity_factor: (tech.capacity_factor * 1.15).min(1.0),
            om_percentage: tech.om_percentage,
            fuel_cost: tech.fuel_cost,
            max_capacity: tech.max_capacity,
            depreciation_rate: tech.depreciation_rate,
            initial_capacity: tech.initial_capacity,
            discount_rate: tech.discount_rate,
            project_lifetime: tech.project_lifetime,
        };
        let lcoe_high = LcoeCalculator::dynamic_lcoe(&tech_high, cumulative_capacity);

        let sensitivity_pct = ((lcoe_high - lcoe_low) / base_lcoe) * 100.0;

        SensitivityResult {
            parameter_name: "Capacity Factor".to_string(),
            base_value: tech.capacity_factor,
            low_value: (tech.capacity_factor * 0.85).min(1.0),
            high_value: (tech.capacity_factor * 1.15).min(1.0),
            lcoe_at_low: lcoe_low,
            lcoe_at_high: lcoe_high,
            sensitivity_pct,
        }
    }

    /// Run all standard sensitivity analyses
    pub fn run_all(tech: &Technology, cumulative_capacity: f64) -> Vec<SensitivityResult> {
        vec![
            Self::sensitivity_discount_rate(tech, cumulative_capacity),
            Self::sensitivity_learning_exponent(tech, cumulative_capacity),
            Self::sensitivity_initial_capex(tech, cumulative_capacity),
            Self::sensitivity_capacity_factor(tech, cumulative_capacity),
        ]
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
    fn test_sensitivity_discount_rate() {
        let tech = sample_tech();
        let result = SensitivityAnalysis::sensitivity_discount_rate(&tech, 10.0);
        assert!(result.lcoe_at_low > 0.0);
        assert!(result.lcoe_at_high > 0.0);
    }

    #[test]
    fn test_run_all_sensitivities() {
        let tech = sample_tech();
        let results = SensitivityAnalysis::run_all(&tech, 10.0);
        assert_eq!(results.len(), 4);
    }
}
