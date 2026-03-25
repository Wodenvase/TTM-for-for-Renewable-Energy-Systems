// src/analysis/convergence.rs
// Transition metrics and convergence analysis

use crate::models::Technology;
use crate::models::lcoe::LcoeCalculator;

/// Key metrics describing energy transition dynamics
#[derive(Clone, Debug)]
pub struct TransitionMetrics {
    /// Year when LCOE crossover occurs (renewable < fossil)
    pub lcoe_crossover_year: Option<f64>,

    /// Cumulative capacity at crossover
    pub crossover_capacity: Option<f64>,

    /// Years to reach 50% renewable penetration
    pub time_to_50_percent: Option<f64>,

    /// Years to reach 80% renewable penetration
    pub time_to_80_percent: Option<f64>,

    /// Initial LCOE of technology ($/MWh)
    pub initial_lcoe: f64,

    /// LCOE at 100 GW cumulative capacity
    pub lcoe_at_100gw: f64,

    /// Cost reduction percentage from 1 GW to 100 GW
    pub cost_reduction_pct: f64,
}

/// Compute transition metrics for a single technology vs. baseline
pub fn compute_transition_metrics(
    tech: &Technology,
    baseline: &Technology,
) -> TransitionMetrics {
    let initial_lcoe = LcoeCalculator::static_lcoe(tech);
    let lcoe_at_100gw = LcoeCalculator::dynamic_lcoe(tech, 100.0);
    let lcoe_initial_2 = LcoeCalculator::dynamic_lcoe(tech, 1.0);

    let cost_reduction_pct = if lcoe_initial_2 > 0.0 {
        100.0 * (1.0 - lcoe_at_100gw / lcoe_initial_2)
    } else {
        0.0
    };

    let lcoe_crossover = LcoeCalculator::find_lcoe_crossover(tech, baseline, tech.max_capacity, 0.1);

    TransitionMetrics {
        lcoe_crossover_year: lcoe_crossover.map(|(_, years)| years),
        crossover_capacity: lcoe_crossover.map(|(cap, _)| cap),
        initial_lcoe,
        lcoe_at_100gw,
        cost_reduction_pct,
        time_to_50_percent: None,
        time_to_80_percent: None,
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

    fn baseline_tech() -> Technology {
        Technology {
            name: "Gas CCGT".to_string(),
            initial_capex: 800.0,
            learning_exponent: 0.05,
            reference_capacity: 1.0,
            capacity_factor: 0.5,
            om_percentage: 0.03,
            fuel_cost: 40.0,
            max_capacity: 300.0,
            depreciation_rate: 0.05,
            initial_capacity: 100.0,
            discount_rate: 0.05,
            project_lifetime: 20,
        }
    }

    #[test]
    fn test_transition_metrics() {
        let solar = sample_tech();
        let gas = baseline_tech();
        let metrics = compute_transition_metrics(&solar, &gas);

        assert!(metrics.initial_lcoe > 0.0);
        assert!(metrics.lcoe_at_100gw > 0.0);
        assert!(metrics.cost_reduction_pct > 0.0);
    }
}
