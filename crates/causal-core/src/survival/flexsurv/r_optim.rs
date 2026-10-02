//! Base R 4.4.1 `optim(method = "BFGS")` numerical recipe used by flexsurv.
//!
//! The variable-metric loop follows `src/appl/optim.c::vmmin`.  The numerical
//! gradient follows `src/library/stats/src/optim.c::fmingr`.  This is separate
//! from [`crate::bfgs`], which reproduces SciPy's BFGS recipe.

use std::num::NonZeroUsize;

const STEP_REDUCTION: f64 = 0.2;
const ACCEPTANCE_TOLERANCE: f64 = 0.0001;
const RELATIVE_CHANGE_TEST: f64 = 10.0;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ROptimArithmetic {
    Separate,
    PinnedArm,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ROptimControl {
    absolute_tolerance: f64,
    relative_tolerance: f64,
    maximum_iterations: usize,
    derivative_steps: Vec<f64>,
    parameter_scales: Vec<f64>,
    function_scale: f64,
    arithmetic: ROptimArithmetic,
}

impl ROptimControl {
    pub fn bfgs_defaults(parameter_count: NonZeroUsize) -> Self {
        let parameter_count = parameter_count.get();
        Self {
            absolute_tolerance: f64::NEG_INFINITY,
            relative_tolerance: f64::EPSILON.sqrt(),
            maximum_iterations: 100,
            derivative_steps: vec![1e-3; parameter_count],
            parameter_scales: vec![1.0; parameter_count],
            function_scale: 1.0,
            arithmetic: ROptimArithmetic::Separate,
        }
    }

    /// Same optimizer and defaults, with the contraction recipe verified
    /// against R 4.5.1's pinned ARM GCC build. Existing oracles keep their
    /// original arithmetic rather than silently changing their trajectories.
    pub fn bfgs_pinned_arm_defaults(parameter_count: NonZeroUsize) -> Self {
        let mut control = Self::bfgs_defaults(parameter_count);
        control.arithmetic = ROptimArithmetic::PinnedArm;
        control
    }

    pub fn new(
        absolute_tolerance: f64,
        relative_tolerance: f64,
        maximum_iterations: usize,
        derivative_steps: Vec<f64>,
        parameter_scales: Vec<f64>,
        function_scale: f64,
    ) -> Result<Self, ROptimError> {
        let control = Self {
            absolute_tolerance,
            relative_tolerance,
            maximum_iterations,
            derivative_steps,
            parameter_scales,
            function_scale,
            arithmetic: ROptimArithmetic::Separate,
        };
        validate_control(&control)?;
        Ok(control)
    }

    pub(crate) fn parameter_count(&self) -> usize {
        self.derivative_steps.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ROptimError {
    EmptyParameters,
    ControlLength {
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    InvalidControl(&'static str),
    NonFiniteParameter {
        index: usize,
    },
    NonFiniteInitialValue,
    NonFiniteFiniteDifference {
        index: usize,
    },
    InvalidGradientLength {
        expected: usize,
        actual: usize,
    },
    NonFiniteGradient {
        index: usize,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ROptimResult {
    pub parameters: Vec<f64>,
    pub value: f64,
    pub function_count: usize,
    pub gradient_count: usize,
    pub iterations: usize,
    /// Base R returns zero on convergence and one when `maxit` is reached.
    pub convergence: usize,
}

/// Optional diagnostics from the same BFGS loop used by production callers.
#[derive(Clone, Debug, PartialEq)]
pub enum ROptimEvent {
    Gradient {
        iteration: usize,
        parameters: Vec<f64>,
        values: Vec<f64>,
    },
    Trial {
        iteration: usize,
        step: f64,
        value: f64,
        threshold: f64,
        accepted: bool,
    },
    Curvature {
        iteration: usize,
        value: f64,
        update: bool,
    },
    Progress {
        iteration: usize,
        enough: bool,
    },
}

/// Base R `optimHess` with an analytic score function.
pub fn r_optim_hessian<G>(
    parameters: &[f64],
    control: &ROptimControl,
    mut gradient: G,
) -> Result<Vec<f64>, ROptimError>
where
    G: FnMut(&[f64]) -> Vec<f64>,
{
    optim_hessian(parameters, control, |scaled| {
        analytic_gradient(scaled, control, &mut gradient)
    })
}

/// Base R `optimHess` when `optim` also differentiated the objective.
pub fn r_optim_hessian_numeric<F>(
    parameters: &[f64],
    control: &ROptimControl,
    mut objective: F,
) -> Result<Vec<f64>, ROptimError>
where
    F: FnMut(&[f64]) -> f64,
{
    optim_hessian(parameters, control, |scaled| {
        numerical_gradient(scaled, control, &mut objective)
    })
}

fn optim_hessian<G>(
    parameters: &[f64],
    control: &ROptimControl,
    mut gradient: G,
) -> Result<Vec<f64>, ROptimError>
where
    G: FnMut(&[f64]) -> Result<Vec<f64>, ROptimError>,
{
    if parameters.len() != control.parameter_count() {
        return Err(ROptimError::ControlLength {
            field: "parameters",
            expected: control.parameter_count(),
            actual: parameters.len(),
        });
    }
    let count = parameters.len();
    let mut scaled = parameters
        .iter()
        .zip(&control.parameter_scales)
        .map(|(parameter, scale)| parameter / scale)
        .collect::<Vec<_>>();
    let mut hessian = vec![0.0; count * count];

    for column in 0..count {
        let step = control.derivative_steps[column] / control.parameter_scales[column];
        scaled[column] += step;
        let upper = gradient(&scaled)?;
        scaled[column] -= 2.0 * step;
        let lower = gradient(&scaled)?;
        scaled[column] += step;

        for row in 0..count {
            hessian[row * count + column] = control.function_scale * (upper[row] - lower[row])
                / (2.0 * step * control.parameter_scales[column] * control.parameter_scales[row]);
        }
    }
    for row in 0..count {
        for column in 0..row {
            let symmetric = 0.5 * (hessian[row * count + column] + hessian[column * count + row]);
            hessian[row * count + column] = symmetric;
            hessian[column * count + row] = symmetric;
        }
    }
    Ok(hessian)
}

fn validate_control(control: &ROptimControl) -> Result<(), ROptimError> {
    let parameter_count = control.derivative_steps.len();
    if parameter_count == 0 {
        return Err(ROptimError::EmptyParameters);
    }
    if control.parameter_scales.len() != parameter_count {
        return Err(ROptimError::ControlLength {
            field: "parameter_scales",
            expected: parameter_count,
            actual: control.parameter_scales.len(),
        });
    }
    if control.absolute_tolerance.is_nan()
        || !control.relative_tolerance.is_finite()
        || control.relative_tolerance < 0.0
    {
        return Err(ROptimError::InvalidControl("tolerances"));
    }

    if control.function_scale == 0.0 || !control.function_scale.is_finite() {
        return Err(ROptimError::InvalidControl("function_scale"));
    }
    if control
        .parameter_scales
        .iter()
        .any(|scale| *scale == 0.0 || !scale.is_finite())
    {
        return Err(ROptimError::InvalidControl("parameter_scales"));
    }
    if control
        .derivative_steps
        .iter()
        .any(|step| *step <= 0.0 || !step.is_finite())
    {
        return Err(ROptimError::InvalidControl("derivative_steps"));
    }
    Ok(())
}

fn scaled_value<F>(parameters: &[f64], control: &ROptimControl, objective: &mut F) -> f64
where
    F: FnMut(&[f64]) -> f64,
{
    let natural = parameters
        .iter()
        .zip(&control.parameter_scales)
        .map(|(parameter, scale)| parameter * scale)
        .collect::<Vec<_>>();
    objective(&natural) / control.function_scale
}

fn analytic_gradient<G>(
    parameters: &[f64],
    control: &ROptimControl,
    gradient: &mut G,
) -> Result<Vec<f64>, ROptimError>
where
    G: FnMut(&[f64]) -> Vec<f64>,
{
    let natural = parameters
        .iter()
        .zip(&control.parameter_scales)
        .map(|(parameter, scale)| parameter * scale)
        .collect::<Vec<_>>();
    let values = gradient(&natural);
    if values.len() != parameters.len() {
        return Err(ROptimError::InvalidGradientLength {
            expected: parameters.len(),
            actual: values.len(),
        });
    }

    values
        .iter()
        .zip(&control.parameter_scales)
        .enumerate()
        .map(|(index, (value, scale))| {
            let scaled = value * scale / control.function_scale;
            if scaled.is_finite() {
                Ok(scaled)
            } else {
                Err(ROptimError::NonFiniteGradient { index })
            }
        })
        .collect()
}

fn numerical_gradient<F>(
    parameters: &[f64],
    control: &ROptimControl,
    objective: &mut F,
) -> Result<Vec<f64>, ROptimError>
where
    F: FnMut(&[f64]) -> f64,
{
    let mut point = parameters.to_vec();
    let mut gradient = Vec::with_capacity(parameters.len());

    for index in 0..parameters.len() {
        let step = control.derivative_steps[index];
        point[index] = parameters[index] + step;
        let upper = scaled_value(&point, control, objective);
        point[index] = parameters[index] - step;
        let lower = scaled_value(&point, control, objective);
        point[index] = parameters[index];

        let value = (upper - lower) / (2.0 * step);
        if !value.is_finite() {
            return Err(ROptimError::NonFiniteFiniteDifference { index });
        }
        gradient.push(value);
    }

    Ok(gradient)
}

fn vmmin<F, G>(
    initial: &[f64],
    control: &ROptimControl,
    objective: &mut F,
    gradient: &mut G,
    mut trace: Option<&mut Vec<ROptimEvent>>,
) -> Result<ROptimResult, ROptimError>
where
    F: FnMut(&[f64]) -> f64,
    G: FnMut(&[f64], &ROptimControl, &mut F) -> Result<Vec<f64>, ROptimError>,
{
    if initial.is_empty() {
        return Err(ROptimError::EmptyParameters);
    }
    if initial.len() != control.derivative_steps.len() {
        return Err(ROptimError::ControlLength {
            field: "parameters",
            expected: control.derivative_steps.len(),
            actual: initial.len(),
        });
    }
    for (index, parameter) in initial.iter().enumerate() {
        if !parameter.is_finite() {
            return Err(ROptimError::NonFiniteParameter { index });
        }
    }

    let parameter_count = initial.len();
    let add_product = |acc: f64, left: f64, right: f64| match control.arithmetic {
        ROptimArithmetic::Separate => acc + left * right,
        ROptimArithmetic::PinnedArm => left.mul_add(right, acc),
    };
    let mut parameters = initial
        .iter()
        .zip(&control.parameter_scales)
        .map(|(parameter, scale)| parameter / scale)
        .collect::<Vec<_>>();

    if control.maximum_iterations == 0 {
        let value = scaled_value(&parameters, control, objective) * control.function_scale;
        return Ok(ROptimResult {
            parameters: initial.to_vec(),
            value,
            function_count: 0,
            gradient_count: 0,
            iterations: 0,
            convergence: 0,
        });
    }

    let mut minimum = scaled_value(&parameters, control, objective);
    if !minimum.is_finite() {
        return Err(ROptimError::NonFiniteInitialValue);
    }
    let mut function_count = 1;
    let mut gradient_count = 1;
    let mut gradient_values = gradient(&parameters, control, objective)?;
    if let Some(events) = trace.as_mut() {
        events.push(ROptimEvent::Gradient {
            iteration: 1,
            parameters: parameters.clone(),
            values: gradient_values.clone(),
        });
    }
    let mut iterations = 1;
    let mut last_restart = gradient_count;

    let mut inverse_hessian = vec![0.0; parameter_count * parameter_count];
    let mut old_parameters = vec![0.0; parameter_count];
    let mut direction = vec![0.0; parameter_count];
    let mut old_gradient = vec![0.0; parameter_count];
    let mut count;

    loop {
        if last_restart == gradient_count {
            inverse_hessian.fill(0.0);
            for index in 0..parameter_count {
                inverse_hessian[index * parameter_count + index] = 1.0;
            }
        }

        old_parameters.copy_from_slice(&parameters);
        old_gradient.copy_from_slice(&gradient_values);

        let mut projected_gradient = 0.0;
        for row in 0..parameter_count {
            let mut value = 0.0;
            for column in 0..=row {
                value = add_product(
                    value,
                    -inverse_hessian[row * parameter_count + column],
                    gradient_values[column],
                );
            }
            for column in (row + 1)..parameter_count {
                value = add_product(
                    value,
                    -inverse_hessian[column * parameter_count + row],
                    gradient_values[column],
                );
            }
            direction[row] = value;
            projected_gradient = add_product(projected_gradient, value, gradient_values[row]);
        }

        if projected_gradient < 0.0 {
            let mut step_length: f64 = 1.0;
            let mut accepted;
            let mut value = minimum;
            loop {
                count = 0;
                for index in 0..parameter_count {
                    parameters[index] =
                        add_product(old_parameters[index], step_length, direction[index]);
                    if RELATIVE_CHANGE_TEST + old_parameters[index]
                        == RELATIVE_CHANGE_TEST + parameters[index]
                    {
                        count += 1;
                    }
                }

                accepted = false;
                if count < parameter_count {
                    value = scaled_value(&parameters, control, objective);
                    function_count += 1;
                    let threshold = add_product(
                        minimum,
                        projected_gradient * step_length,
                        ACCEPTANCE_TOLERANCE,
                    );
                    accepted = value.is_finite() && value <= threshold;
                    if let Some(events) = trace.as_mut() {
                        events.push(ROptimEvent::Trial {
                            iteration: iterations,
                            step: step_length,
                            value,
                            threshold,
                            accepted,
                        });
                    }
                    if !accepted {
                        step_length *= STEP_REDUCTION;
                    }
                }
                if count == parameter_count || accepted {
                    break;
                }
            }

            let enough = value > control.absolute_tolerance
                && (value - minimum).abs()
                    > control.relative_tolerance * (minimum.abs() + control.relative_tolerance);
            if let Some(events) = trace.as_mut() {
                events.push(ROptimEvent::Progress {
                    iteration: iterations,
                    enough,
                });
            }
            if !enough {
                count = parameter_count;
                minimum = value;
            }

            if count < parameter_count {
                minimum = value;
                gradient_values = gradient(&parameters, control, objective)?;
                gradient_count += 1;
                iterations += 1;
                if let Some(events) = trace.as_mut() {
                    events.push(ROptimEvent::Gradient {
                        iteration: iterations,
                        parameters: parameters.clone(),
                        values: gradient_values.clone(),
                    });
                }

                let mut first_curvature = 0.0;
                for index in 0..parameter_count {
                    direction[index] *= step_length;
                    old_gradient[index] = gradient_values[index] - old_gradient[index];
                    first_curvature =
                        add_product(first_curvature, direction[index], old_gradient[index]);
                }

                if let Some(events) = trace.as_mut() {
                    events.push(ROptimEvent::Curvature {
                        iteration: iterations,
                        value: first_curvature,
                        update: first_curvature > 0.,
                    });
                }
                if first_curvature > 0.0 {
                    let mut transformed_gradient = vec![0.0; parameter_count];
                    let mut second_curvature = 0.0;
                    for row in 0..parameter_count {
                        let mut value = 0.0;
                        for column in 0..=row {
                            value = add_product(
                                value,
                                inverse_hessian[row * parameter_count + column],
                                old_gradient[column],
                            );
                        }
                        for column in (row + 1)..parameter_count {
                            value = add_product(
                                value,
                                inverse_hessian[column * parameter_count + row],
                                old_gradient[column],
                            );
                        }
                        transformed_gradient[row] = value;
                        second_curvature = add_product(second_curvature, value, old_gradient[row]);
                    }

                    let scale = 1.0 + second_curvature / first_curvature;
                    for row in 0..parameter_count {
                        for column in 0..=row {
                            let numerator = match control.arithmetic {
                                ROptimArithmetic::Separate => {
                                    scale * direction[row] * direction[column]
                                        - transformed_gradient[row] * direction[column]
                                        - direction[row] * transformed_gradient[column]
                                }
                                ROptimArithmetic::PinnedArm => {
                                    let first = (scale * direction[row]).mul_add(
                                        direction[column],
                                        -(transformed_gradient[row] * direction[column]),
                                    );
                                    (-direction[row]).mul_add(transformed_gradient[column], first)
                                }
                            };
                            inverse_hessian[row * parameter_count + column] +=
                                numerator / first_curvature;
                        }
                    }
                } else {
                    last_restart = gradient_count;
                }
            } else if last_restart < gradient_count {
                count = 0;
                last_restart = gradient_count;
            }
        } else {
            count = 0;
            if last_restart == gradient_count {
                count = parameter_count;
            } else {
                last_restart = gradient_count;
            }
        }

        if iterations >= control.maximum_iterations {
            break;
        }
        if gradient_count - last_restart > 2 * parameter_count {
            last_restart = gradient_count;
        }
        if count == parameter_count && last_restart == gradient_count {
            break;
        }
    }

    Ok(ROptimResult {
        parameters: parameters
            .iter()
            .zip(&control.parameter_scales)
            .map(|(parameter, scale)| parameter * scale)
            .collect(),
        value: minimum * control.function_scale,
        function_count,
        gradient_count,
        iterations,
        convergence: usize::from(iterations >= control.maximum_iterations),
    })
}

pub fn r_optim_bfgs<F, G>(
    initial: &[f64],
    control: &ROptimControl,
    mut objective: F,
    mut gradient: G,
) -> Result<ROptimResult, ROptimError>
where
    F: FnMut(&[f64]) -> f64,
    G: FnMut(&[f64]) -> Vec<f64>,
{
    vmmin(
        initial,
        control,
        &mut objective,
        &mut |parameters, control, _| analytic_gradient(parameters, control, &mut gradient),
        None,
    )
}

pub fn r_optim_bfgs_numeric<F>(
    initial: &[f64],
    control: &ROptimControl,
    mut objective: F,
) -> Result<ROptimResult, ROptimError>
where
    F: FnMut(&[f64]) -> f64,
{
    vmmin(
        initial,
        control,
        &mut objective,
        &mut |parameters, control, objective| numerical_gradient(parameters, control, objective),
        None,
    )
}

pub fn r_optim_bfgs_numeric_trace<F>(
    initial: &[f64],
    control: &ROptimControl,
    mut objective: F,
    trace: &mut Vec<ROptimEvent>,
) -> Result<ROptimResult, ROptimError>
where
    F: FnMut(&[f64]) -> f64,
{
    vmmin(
        initial,
        control,
        &mut objective,
        &mut |parameters, control, objective| numerical_gradient(parameters, control, objective),
        Some(trace),
    )
}
