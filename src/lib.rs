// src/lib.rs
// Core library for LCOE dynamics model

pub mod models;
pub mod analysis;
pub mod solvers;

pub use models::{Technology, System};
pub use analysis::{convergence, sensitivity};
pub use solvers::ode::RK4Solver;

/// Configuration for simulation
#[derive(Clone, Debug)]
pub struct SimulationConfig {
    pub start_year: i32,
    pub end_year: i32,
    pub timestep_years: f64,
    pub discount_rate: f64,
    pub project_lifetime: i32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            start_year: 2024,
            end_year: 2050,
            timestep_years: 0.5,
            discount_rate: 0.05,
            project_lifetime: 20,
        }
    }
}
