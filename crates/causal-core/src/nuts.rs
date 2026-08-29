//! Multinomial No-U-Turn sampling with Pyro-style biased progressive sampling,
//! dual-averaged step size, and windowed diagonal metric adaptation.
//!
//! The integrator and recursive tree structure were adapted from the existing Recursis
//! slice-NUTS skeleton, but the proposal weights, generalized U-turn test, and warmup
//! schedule follow `pyro.infer.mcmc.NUTS` and `WarmupAdapter`.

use crate::nprandom::NpRng;

const MAX_SLICED_ENERGY: f64 = 1000.0;
const MIN_STEP_SIZE: f64 = 1e-10;
const MAX_STEP_SIZE: f64 = 1e10;

#[derive(Clone, Copy, Debug)]
pub struct NutsOptions {
    pub warmup: usize,
    pub samples: usize,
    pub target_accept: f64,
    pub max_tree_depth: usize,
    pub adapt_step_size: bool,
    pub adapt_mass_matrix: bool,
}

impl Default for NutsOptions {
    fn default() -> Self {
        Self {
            warmup: 500,
            samples: 1000,
            target_accept: 0.8,
            max_tree_depth: 10,
            adapt_step_size: true,
            adapt_mass_matrix: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NutsResult {
    pub samples: Vec<Vec<f64>>,
    pub mean_accept_probability: f64,
    pub acceptance_rate: f64,
    pub divergences: usize,
    pub step_size: f64,
    /// Pyro's diagonal `inverse_mass_matrix`, estimated as regularized position variance.
    pub inverse_mass_matrix: Vec<f64>,
}

#[derive(Clone)]
struct PhasePoint {
    position: Vec<f64>,
    momentum: Vec<f64>,
    gradient: Vec<f64>,
    log_prob: f64,
}

#[derive(Clone)]
struct DiagMetric {
    inverse_mass: Vec<f64>,
}

impl DiagMetric {
    fn identity(dim: usize) -> Self {
        Self {
            inverse_mass: vec![1.0; dim],
        }
    }

    /// Pyro samples unscaled momentum from N(0,I), then applies M^(-1/2), where the
    /// adapted diagonal `inverse_mass` is the position covariance estimate.
    fn sample_momentum(&self, rng: &mut NpRng) -> Vec<f64> {
        self.inverse_mass
            .iter()
            .map(|variance| rng.standard_normal() / variance.sqrt())
            .collect()
    }

    fn unscaled_momentum(&self, momentum: &[f64]) -> Vec<f64> {
        momentum
            .iter()
            .zip(&self.inverse_mass)
            .map(|(value, variance)| value * variance.sqrt())
            .collect()
    }

    fn kinetic_energy(&self, momentum: &[f64]) -> f64 {
        momentum
            .iter()
            .zip(&self.inverse_mass)
            .map(|(value, variance)| value * value * variance)
            .sum::<f64>()
            * 0.5
    }
}

fn leapfrog(
    point: &PhasePoint,
    step_size: f64,
    log_prob_grad: &dyn Fn(&[f64]) -> (f64, Vec<f64>),
    metric: &DiagMetric,
) -> PhasePoint {
    let mut momentum = point.momentum.clone();
    for (value, gradient) in momentum.iter_mut().zip(&point.gradient) {
        *value += 0.5 * step_size * gradient;
    }
    let position: Vec<f64> = point
        .position
        .iter()
        .zip(&momentum)
        .zip(&metric.inverse_mass)
        .map(|((position, momentum), inverse_mass)| position + step_size * inverse_mass * momentum)
        .collect();
    let (log_prob, gradient) = log_prob_grad(&position);
    for (value, gradient) in momentum.iter_mut().zip(&gradient) {
        *value += 0.5 * step_size * gradient;
    }
    PhasePoint {
        position,
        momentum,
        gradient,
        log_prob,
    }
}

fn energy(point: &PhasePoint, metric: &DiagMetric) -> f64 {
    -point.log_prob + metric.kinetic_energy(&point.momentum)
}

fn logaddexp(left: f64, right: f64) -> f64 {
    if left == f64::NEG_INFINITY {
        return right;
    }
    if right == f64::NEG_INFINITY {
        return left;
    }
    let maximum = left.max(right);
    maximum + ((left - maximum).exp() + (right - maximum).exp()).ln()
}

/// Betancourt's generalized criterion used by Pyro, based on the sum of unscaled momenta.
fn is_turning(left: &[f64], right: &[f64], momentum_sum: &[f64]) -> bool {
    let mut left_angle = 0.0;
    let mut right_angle = 0.0;
    for index in 0..momentum_sum.len() {
        let rho = momentum_sum[index] - (left[index] + right[index]) / 2.0;
        left_angle += left[index] * rho;
        right_angle += right[index] * rho;
    }
    left_angle <= 0.0 || right_angle <= 0.0
}

struct Tree {
    left: PhasePoint,
    left_unscaled: Vec<f64>,
    right: PhasePoint,
    right_unscaled: Vec<f64>,
    proposal: PhasePoint,
    momentum_sum: Vec<f64>,
    log_weight: f64,
    turning: bool,
    diverging: bool,
    sum_accept_probabilities: f64,
    proposals: usize,
}

#[allow(clippy::too_many_arguments)]
fn build_tree(
    point: &PhasePoint,
    direction: i8,
    depth: usize,
    step_size: f64,
    log_slice: f64,
    initial_energy: f64,
    log_prob_grad: &dyn Fn(&[f64]) -> (f64, Vec<f64>),
    metric: &DiagMetric,
    rng: &mut NpRng,
) -> Tree {
    if depth == 0 {
        let next = leapfrog(point, direction as f64 * step_size, log_prob_grad, metric);
        let next_energy = {
            let value = energy(&next, metric);
            if value.is_nan() {
                f64::INFINITY
            } else {
                value
            }
        };
        let sliced_energy = next_energy + log_slice;
        let delta_energy = next_energy - initial_energy;
        let accept_probability = (-delta_energy).exp().min(1.0);
        let unscaled = metric.unscaled_momentum(&next.momentum);
        return Tree {
            left: next.clone(),
            left_unscaled: unscaled.clone(),
            right: next.clone(),
            right_unscaled: unscaled.clone(),
            proposal: next,
            momentum_sum: unscaled,
            log_weight: -sliced_energy,
            turning: false,
            diverging: sliced_energy > MAX_SLICED_ENERGY,
            sum_accept_probabilities: accept_probability,
            proposals: 1,
        };
    }

    let first = build_tree(
        point,
        direction,
        depth - 1,
        step_size,
        log_slice,
        initial_energy,
        log_prob_grad,
        metric,
        rng,
    );
    if first.turning || first.diverging {
        return first;
    }

    let second_start = if direction == 1 {
        &first.right
    } else {
        &first.left
    };
    let second = build_tree(
        second_start,
        direction,
        depth - 1,
        step_size,
        log_slice,
        initial_energy,
        log_prob_grad,
        metric,
        rng,
    );

    let log_weight = logaddexp(first.log_weight, second.log_weight);
    let choose_second_probability = (second.log_weight - log_weight).exp();
    let proposal = if rng.next_f64() < choose_second_probability {
        second.proposal.clone()
    } else {
        first.proposal.clone()
    };
    let momentum_sum: Vec<f64> = first
        .momentum_sum
        .iter()
        .zip(&second.momentum_sum)
        .map(|(left, right)| left + right)
        .collect();

    let (left, left_unscaled, right, right_unscaled) = if direction == 1 {
        (
            first.left,
            first.left_unscaled,
            second.right,
            second.right_unscaled,
        )
    } else {
        (
            second.left,
            second.left_unscaled,
            first.right,
            first.right_unscaled,
        )
    };
    let turning = second.turning || is_turning(&left_unscaled, &right_unscaled, &momentum_sum);

    Tree {
        left,
        left_unscaled,
        right,
        right_unscaled,
        proposal,
        momentum_sum,
        log_weight,
        turning,
        diverging: second.diverging,
        sum_accept_probabilities: first.sum_accept_probabilities + second.sum_accept_probabilities,
        proposals: first.proposals + second.proposals,
    }
}

fn find_reasonable_step_size(
    position: &[f64],
    log_prob: f64,
    gradient: &[f64],
    initial_step_size: f64,
    log_prob_grad: &dyn Fn(&[f64]) -> (f64, Vec<f64>),
    metric: &DiagMetric,
    rng: &mut NpRng,
) -> f64 {
    let delta_energy_at = |step_size: f64, rng: &mut NpRng| {
        let point = PhasePoint {
            position: position.to_vec(),
            momentum: metric.sample_momentum(rng),
            gradient: gradient.to_vec(),
            log_prob,
        };
        let initial_energy = energy(&point, metric);
        let proposal = leapfrog(&point, step_size, log_prob_grad, metric);
        energy(&proposal, metric) - initial_energy
    };

    let mut step_size = initial_step_size;
    let mut delta_energy = delta_energy_at(step_size, rng);
    let threshold = 0.8_f64.ln();
    let direction: i32 = if threshold < -delta_energy { 1 } else { -1 };
    let scale = 2.0_f64.powi(direction);
    let mut direction_new = direction;
    while direction_new == direction && step_size > MIN_STEP_SIZE && step_size < MAX_STEP_SIZE {
        step_size *= scale;
        delta_energy = delta_energy_at(step_size, rng);
        direction_new = if threshold < -delta_energy { 1 } else { -1 };
    }
    step_size.clamp(MIN_STEP_SIZE, MAX_STEP_SIZE)
}

struct DualAveraging {
    prox_center: f64,
    x_average: f64,
    gradient_average: f64,
    iteration: usize,
    x: f64,
}

impl DualAveraging {
    fn new(prox_center: f64) -> Self {
        Self {
            prox_center,
            x_average: 0.0,
            gradient_average: 0.0,
            iteration: 0,
            x: 0.0,
        }
    }

    fn reset(&mut self, prox_center: f64) {
        *self = Self::new(prox_center);
    }

    fn step(&mut self, gradient: f64) {
        self.iteration += 1;
        let t = self.iteration as f64;
        self.gradient_average =
            (1.0 - 1.0 / (t + 10.0)) * self.gradient_average + gradient / (t + 10.0);
        self.x = self.prox_center - t.sqrt() / 0.05 * self.gradient_average;
        let weight = t.powf(-0.75);
        self.x_average = (1.0 - weight) * self.x_average + weight * self.x;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AdaptWindow {
    start: usize,
    end: usize,
}

fn adaptation_schedule(warmup: usize) -> Vec<AdaptWindow> {
    if warmup == 0 {
        return Vec::new();
    }
    if warmup < 20 {
        return vec![AdaptWindow {
            start: 0,
            end: warmup - 1,
        }];
    }
    let (mut start_buffer, mut end_buffer, mut initial_window) = (75, 50, 25);
    if start_buffer + end_buffer + initial_window > warmup {
        start_buffer = (0.15 * warmup as f64) as usize;
        end_buffer = (0.1 * warmup as f64) as usize;
        initial_window = warmup - start_buffer - end_buffer;
    }
    let mut schedule = vec![AdaptWindow {
        start: 0,
        end: start_buffer - 1,
    }];
    let end_window_start = warmup - end_buffer;
    let mut next_size = initial_window;
    let mut next_start = start_buffer;
    while next_start < end_window_start {
        let current_start = next_start;
        let mut current_size = next_size;
        if 3 * current_size <= end_window_start - current_start {
            next_size = 2 * current_size;
        } else {
            current_size = end_window_start - current_start;
        }
        next_start = current_start + current_size;
        schedule.push(AdaptWindow {
            start: current_start,
            end: next_start - 1,
        });
    }
    schedule.push(AdaptWindow {
        start: end_window_start,
        end: warmup - 1,
    });
    schedule
}

struct WelfordDiagonal {
    mean: Vec<f64>,
    m2: Vec<f64>,
    samples: usize,
}

impl WelfordDiagonal {
    fn new(dim: usize) -> Self {
        Self {
            mean: vec![0.0; dim],
            m2: vec![0.0; dim],
            samples: 0,
        }
    }

    fn reset(&mut self) {
        self.mean.fill(0.0);
        self.m2.fill(0.0);
        self.samples = 0;
    }

    fn update(&mut self, sample: &[f64]) {
        self.samples += 1;
        for index in 0..sample.len() {
            let before = sample[index] - self.mean[index];
            self.mean[index] += before / self.samples as f64;
            let after = sample[index] - self.mean[index];
            self.m2[index] += before * after;
        }
    }

    fn regularized_covariance(&self) -> Vec<f64> {
        assert!(self.samples >= 2, "insufficient metric-adaptation samples");
        let n = self.samples as f64;
        self.m2
            .iter()
            .map(|m2| {
                let covariance = m2 / (n - 1.0);
                n / (n + 5.0) * covariance + 1e-3 * 5.0 / (n + 5.0)
            })
            .collect()
    }
}

/// Run Pyro-style multinomial NUTS from an explicit unconstrained starting point.
pub fn multinomial_nuts(
    log_prob_grad: &dyn Fn(&[f64]) -> (f64, Vec<f64>),
    initial: &[f64],
    options: NutsOptions,
    rng: &mut NpRng,
) -> NutsResult {
    assert!(!initial.is_empty());
    assert!(options.samples > 0);
    assert!((0.0..1.0).contains(&options.target_accept));
    let (mut log_prob, mut gradient) = log_prob_grad(initial);
    assert!(log_prob.is_finite() && gradient.iter().all(|value| value.is_finite()));
    let mut position = initial.to_vec();
    let mut metric = DiagMetric::identity(initial.len());
    let mut step_size = if options.adapt_step_size {
        find_reasonable_step_size(
            &position,
            log_prob,
            &gradient,
            1.0,
            log_prob_grad,
            &metric,
            rng,
        )
    } else {
        1.0
    };
    let mut dual = DualAveraging::new((10.0 * step_size).ln());
    let schedule = adaptation_schedule(options.warmup);
    let mut current_window = 0usize;
    let mut welford = WelfordDiagonal::new(initial.len());

    let mut samples = Vec::with_capacity(options.samples);
    let mut divergences = 0usize;
    let mut accepted = 0usize;
    let mut post_warmup_accept_total = 0.0;

    for iteration in 0..options.warmup + options.samples {
        let momentum = metric.sample_momentum(rng);
        let initial_point = PhasePoint {
            position: position.clone(),
            momentum,
            gradient: gradient.clone(),
            log_prob,
        };
        let initial_energy = energy(&initial_point, &metric);
        let log_slice = -initial_energy;
        let initial_unscaled = metric.unscaled_momentum(&initial_point.momentum);
        let mut left = initial_point.clone();
        let mut right = initial_point;
        let mut left_unscaled = initial_unscaled.clone();
        let mut right_unscaled = initial_unscaled.clone();
        let mut momentum_sum = initial_unscaled;
        let mut tree_log_weight = 0.0;
        let mut sum_accept_probabilities = 0.0;
        let mut proposals = 0usize;
        let mut moved = false;

        for depth in 0..options.max_tree_depth {
            let direction = if rng.next_f64() < 0.5 { -1 } else { 1 };
            let tree = if direction == 1 {
                let tree = build_tree(
                    &right,
                    direction,
                    depth,
                    step_size,
                    log_slice,
                    initial_energy,
                    log_prob_grad,
                    &metric,
                    rng,
                );
                right = tree.right.clone();
                right_unscaled = tree.right_unscaled.clone();
                tree
            } else {
                let tree = build_tree(
                    &left,
                    direction,
                    depth,
                    step_size,
                    log_slice,
                    initial_energy,
                    log_prob_grad,
                    &metric,
                    rng,
                );
                left = tree.left.clone();
                left_unscaled = tree.left_unscaled.clone();
                tree
            };
            sum_accept_probabilities += tree.sum_accept_probabilities;
            proposals += tree.proposals;
            if tree.diverging {
                if iteration >= options.warmup {
                    divergences += 1;
                }
                break;
            }
            if tree.turning {
                break;
            }

            let proposal_probability = (tree.log_weight - tree_log_weight).exp();
            if rng.next_f64() < proposal_probability {
                moved = true;
                position = tree.proposal.position;
                log_prob = tree.proposal.log_prob;
                gradient = tree.proposal.gradient;
            }
            for index in 0..momentum_sum.len() {
                momentum_sum[index] += tree.momentum_sum[index];
            }
            if is_turning(&left_unscaled, &right_unscaled, &momentum_sum) {
                break;
            }
            tree_log_weight = logaddexp(tree_log_weight, tree.log_weight);
        }

        let accept_probability = sum_accept_probabilities / proposals.max(1) as f64;
        let adaptation_time = iteration + 1;
        if adaptation_time < options.warmup {
            if options.adapt_step_size {
                dual.step(options.target_accept - accept_probability);
                step_size = dual.x.exp();
            }
            if !schedule.is_empty() {
                let window = schedule[current_window];
                let mass_phase = options.adapt_mass_matrix
                    && current_window > 0
                    && current_window + 1 < schedule.len();
                if mass_phase {
                    welford.update(&position);
                }
                if adaptation_time == window.end {
                    if current_window + 1 == schedule.len() {
                        if options.adapt_step_size {
                            step_size = dual.x_average.exp();
                        }
                        current_window += 1;
                    } else if current_window == 0 {
                        current_window += 1;
                    } else {
                        if mass_phase {
                            metric.inverse_mass = welford.regularized_covariance();
                            welford.reset();
                            if options.adapt_step_size {
                                step_size = find_reasonable_step_size(
                                    &position,
                                    log_prob,
                                    &gradient,
                                    step_size,
                                    log_prob_grad,
                                    &metric,
                                    rng,
                                );
                                dual.reset((10.0 * step_size).ln());
                            }
                        }
                        current_window += 1;
                    }
                }
            }
        } else if iteration >= options.warmup {
            if moved {
                accepted += 1;
            }
            post_warmup_accept_total += accept_probability;
            samples.push(position.clone());
        }
    }

    NutsResult {
        samples,
        mean_accept_probability: post_warmup_accept_total / options.samples as f64,
        acceptance_rate: accepted as f64 / options.samples as f64,
        divergences,
        step_size,
        inverse_mass_matrix: metric.inverse_mass,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pyro_500_step_schedule_matches() {
        assert_eq!(
            adaptation_schedule(500),
            vec![
                AdaptWindow { start: 0, end: 74 },
                AdaptWindow { start: 75, end: 99 },
                AdaptWindow {
                    start: 100,
                    end: 149
                },
                AdaptWindow {
                    start: 150,
                    end: 249
                },
                AdaptWindow {
                    start: 250,
                    end: 449
                },
                AdaptWindow {
                    start: 450,
                    end: 499
                },
            ]
        );
    }

    #[test]
    fn welford_regularization_matches_pyro_formula() {
        let mut state = WelfordDiagonal::new(2);
        state.update(&[1.0, 4.0]);
        state.update(&[3.0, 8.0]);
        state.update(&[5.0, 6.0]);
        let got = state.regularized_covariance();
        let shrinkage = 1e-3 * 5.0 / 8.0;
        assert!((got[0] - (3.0 / 8.0 * 4.0 + shrinkage)).abs() < 1e-15);
        assert!((got[1] - (3.0 / 8.0 * 4.0 + shrinkage)).abs() < 1e-15);
    }
}
