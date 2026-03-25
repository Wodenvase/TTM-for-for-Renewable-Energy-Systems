// src/main.rs
// CLI entry point for LCOE dynamics model

use lcoe_dynamics::models::{Technology, System};
use lcoe_dynamics::models::lcoe::LcoeCalculator;
use lcoe_dynamics::analysis::convergence::compute_transition_metrics;
use lcoe_dynamics::analysis::sensitivity::SensitivityAnalysis;

fn main() {
    println!("LCOE Dynamics Model - Research Grade");
    println!("====================================\n");

    // Define baseline technologies (2024 data)
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
        initial_capacity: 0.1,
        discount_rate: 0.05,
        project_lifetime: 20,
    };

    let wind_onshore = Technology {
        name: "Wind Onshore".to_string(),
        initial_capex: 1300.0,
        learning_exponent: 0.15,
        reference_capacity: 1.0,
        capacity_factor: 0.35,
        om_percentage: 0.02,
        fuel_cost: 0.0,
        max_capacity: 500.0,
        depreciation_rate: 0.05,
        initial_capacity: 0.1,
        discount_rate: 0.05,
        project_lifetime: 20,
    };

    let gas = Technology {
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
    };

    // Analysis 1: Static LCOE
    println!("1. STATIC LCOE COMPARISON (2024 baseline)");
    println!("-----------------------------------------");
    println!("Solar PV:        ${:.2}/MWh", LcoeCalculator::static_lcoe(&solar));
    println!("Wind Onshore:    ${:.2}/MWh", LcoeCalculator::static_lcoe(&wind_onshore));
    println!("Gas CCGT:        ${:.2}/MWh", LcoeCalculator::static_lcoe(&gas));
    println!();

    // Analysis 2: Dynamic LCOE at different cumulative capacities
    println!("2. DYNAMIC LCOE WITH LEARNING CURVES");
    println!("-----------------------------------");
    for q in [1.0, 10.0, 50.0, 100.0, 200.0].iter() {
        println!("\nAt Q = {:.0} GW cumulative capacity:", q);
        println!("  Solar PV:      ${:.2}/MWh", LcoeCalculator::dynamic_lcoe(&solar, *q));
        println!("  Wind Onshore:  ${:.2}/MWh", LcoeCalculator::dynamic_lcoe(&wind_onshore, *q));
        println!("  Gas CCGT:      ${:.2}/MWh", LcoeCalculator::dynamic_lcoe(&gas, *q));
    }
    println!();

    // Analysis 3: Transition metrics
    println!("3. TECHNOLOGY TRANSITION METRICS");
    println!("--------------------------------");
    let solar_metrics = compute_transition_metrics(&solar, &gas);
    println!("\nSolar PV vs Gas baseline:");
    println!("  Initial LCOE:      ${:.2}/MWh", solar_metrics.initial_lcoe);
    println!("  LCOE at 100 GW:    ${:.2}/MWh", solar_metrics.lcoe_at_100gw);
    println!("  Cost reduction:    {:.1}%", solar_metrics.cost_reduction_pct);
    if let Some(years) = solar_metrics.lcoe_crossover_year {
        println!("  LCOE crossover:    {:.1} years", years);
    }
    println!();

    // Analysis 4: Sensitivity analysis
    println!("4. SENSITIVITY ANALYSIS - Solar PV");
    println!("----------------------------------");
    let sensitivities = SensitivityAnalysis::run_all(&solar, 10.0);
    for result in sensitivities {
        println!("\n{}:", result.parameter_name);
        println!("  LCOE (low):      ${:.2}/MWh", result.lcoe_at_low);
        println!("  LCOE (high):     ${:.2}/MWh", result.lcoe_at_high);
        println!("  Sensitivity:     {:.1}%", result.sensitivity_pct);
    }
    println!();

    // Analysis 5: System penetration
    println!("5. SYSTEM CONFIGURATION");
    println!("---------------------");
    let mut system = System::new(50e9); // $50B annual
    system.add_technology(solar.clone());
    system.add_technology(wind_onshore.clone());
    system.add_technology(gas.clone());

    println!("Total system capacity:   {:.1} GW", system.total_capacity());
    println!("Renewable penetration:   {:.1}%", system.renewable_penetration());
    println!();

    println!("✓ Analysis complete. See README.md for methodology and references.");
}
