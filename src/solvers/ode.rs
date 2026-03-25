// src/solvers/ode.rs
// ODE solver for capacity evolution dynamics

/// RK4 (Runge-Kutta 4th order) solver for ODEs
/// dQ/dt = f(t, Q)
pub struct RK4Solver;

impl RK4Solver {
    /// Solve dQ/dt = f(t, Q) with initial condition Q(0) = Q0
    /// Returns vec of (time, capacity) tuples
    pub fn solve<F>(
        f: F,
        q0: f64,
        t_start: f64,
        t_end: f64,
        dt: f64,
    ) -> Vec<(f64, f64)>
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut result = vec![(t_start, q0)];
        let mut t = t_start;
        let mut q = q0;

        while t < t_end {
            let dt_step = (t_end - t).min(dt);

            // RK4 coefficients
            let k1 = f(t, q);
            let k2 = f(t + dt_step / 2.0, q + k1 * dt_step / 2.0);
            let k3 = f(t + dt_step / 2.0, q + k2 * dt_step / 2.0);
            let k4 = f(t + dt_step, q + k3 * dt_step);

            // Update
            q = q + (dt_step / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
            t = t + dt_step;

            result.push((t, q.max(0.0))); // Capacity cannot be negative
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rk4_exponential() {
        // Solve dQ/dt = 0.1 * Q with Q(0) = 1
        // Exact solution: Q(t) = exp(0.1 * t)
        let solution = RK4Solver::solve(|_, q| 0.1 * q, 1.0, 0.0, 10.0, 0.1);

        // Check final value
        let (_, q_final) = solution.last().unwrap();
        let expected = (0.1 * 10.0).exp();

        assert!((q_final - expected).abs() / expected < 0.01); // 1% error
    }

    #[test]
    fn test_rk4_linear() {
        // Solve dQ/dt = 0.5 with Q(0) = 0
        // Exact solution: Q(t) = 0.5 * t
        let solution = RK4Solver::solve(|_, _| 0.5, 0.0, 0.0, 10.0, 0.1);

        let (_, q_final) = solution.last().unwrap();
        let expected = 0.5 * 10.0;

        assert!((q_final - expected).abs() < 0.1);
    }

    #[test]
    fn test_rk4_non_negative() {
        // Solve dQ/dt = -0.5 * Q with Q(0) = 1
        // Solution should never go negative
        let solution = RK4Solver::solve(|_, q| -0.5 * q, 1.0, 0.0, 20.0, 0.1);

        for (_, q) in solution.iter() {
            assert!(*q >= 0.0);
        }
    }
}
