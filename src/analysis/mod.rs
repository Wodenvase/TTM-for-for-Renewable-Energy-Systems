// src/analysis/mod.rs
pub mod convergence;
pub mod sensitivity;

pub use convergence::{TransitionMetrics, compute_transition_metrics};
pub use sensitivity::SensitivityAnalysis;
