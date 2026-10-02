//! Translation of base R `src/appl/optim.c::nmmin` (R 4.4.1, GPL-2-or-later).
//! R's simplex construction and stopping rules differ from SciPy's recipe.

#[derive(Debug, PartialEq)]
pub enum NmError {
    InvalidInput,
    NonFiniteInitialValue,
}
#[derive(Debug, PartialEq)]
pub enum NmTermination {
    Converged,
    EvaluationLimit,
    ShrinkFailure,
}
pub struct NmFit {
    pub parameters: Vec<f64>,
    pub value: f64,
    pub evaluations: usize,
    pub termination: NmTermination,
}

pub fn minimize<F: FnMut(&[f64]) -> f64>(
    start: &[f64],
    max_evaluations: usize,
    relative_tolerance: f64,
    absolute_tolerance: f64,
    mut objective: F,
) -> Result<NmFit, NmError> {
    let n = start.len();
    if n == 0
        || start.iter().any(|v| !v.is_finite())
        || !relative_tolerance.is_finite()
        || relative_tolerance < 0.
        || absolute_tolerance.is_nan()
    {
        return Err(NmError::InvalidInput);
    }
    let initial = objective(start);
    if !initial.is_finite() {
        return Err(NmError::NonFiniteInitialValue);
    }
    if max_evaluations == 0 {
        return Ok(NmFit {
            parameters: start.to_vec(),
            value: initial,
            evaluations: 0,
            termination: NmTermination::Converged,
        });
    }
    let tolerance = relative_tolerance * (initial.abs() + relative_tolerance);
    let mut vertices = vec![start.to_vec(); n + 1];
    let mut values = vec![0.; n + 1];
    values[0] = initial;
    let step = start.iter().map(|v| 0.1 * v.abs()).fold(0., f64::max);
    let step = if step == 0. { 0.1 } else { step };
    let mut size = 0.;
    for j in 1..=n {
        let mut trial = step;
        while vertices[j][j - 1] == start[j - 1] {
            vertices[j][j - 1] = start[j - 1] + trial;
            trial *= 10.;
        }
        size += trial;
    }
    let mut old_size = size;
    let mut best = 0;
    let mut evaluate_vertices = true;
    let mut evaluations = 1;
    let mut termination = NmTermination::Converged;
    let evaluate = |x: &[f64], f: &mut F| {
        let y = f(x);
        if y.is_finite() {
            y
        } else {
            1e35
        }
    };
    loop {
        if evaluate_vertices {
            for j in 0..=n {
                if j != best {
                    values[j] = evaluate(&vertices[j], &mut objective);
                    evaluations += 1;
                }
            }
            evaluate_vertices = false;
        }
        let mut low = values[best];
        let mut high = low;
        let mut worst = best;
        for j in 0..=n {
            if j != best {
                let value = values[j];
                if value < low {
                    best = j;
                    low = value;
                }
                if value > high {
                    worst = j;
                    high = value;
                }
            }
        }
        if high <= low + tolerance || low <= absolute_tolerance {
            break;
        }
        let centroid: Vec<f64> = (0..n)
            .map(|i| {
                let mut temp = -vertices[worst][i];
                for vertex in &vertices {
                    temp += vertex[i];
                }
                temp / n as f64
            })
            .collect();
        let reflected: Vec<f64> = (0..n)
            .map(|i| 2. * centroid[i] - vertices[worst][i])
            .collect();
        let reflected_value = evaluate(&reflected, &mut objective);
        evaluations += 1;
        if reflected_value < low {
            let expanded: Vec<f64> = (0..n).map(|i| 2. * reflected[i] - centroid[i]).collect();
            let expanded_value = evaluate(&expanded, &mut objective);
            evaluations += 1;
            if expanded_value < reflected_value {
                vertices[worst] = expanded;
                values[worst] = expanded_value;
            } else {
                vertices[worst] = reflected;
                values[worst] = reflected_value;
            }
        } else {
            if reflected_value < high {
                vertices[worst] = reflected;
                values[worst] = reflected_value;
            }
            let contracted: Vec<f64> = (0..n)
                .map(|i| 0.5 * vertices[worst][i] + 0.5 * centroid[i])
                .collect();
            let contracted_value = evaluate(&contracted, &mut objective);
            evaluations += 1;
            if contracted_value < values[worst] {
                vertices[worst] = contracted;
                values[worst] = contracted_value;
            } else if reflected_value >= high {
                evaluate_vertices = true;
                size = 0.;
                for j in 0..=n {
                    if j != best {
                        for i in 0..n {
                            vertices[j][i] =
                                0.5 * (vertices[j][i] - vertices[best][i]) + vertices[best][i];
                            size += (vertices[j][i] - vertices[best][i]).abs();
                        }
                    }
                }
                if size < old_size {
                    old_size = size;
                } else {
                    termination = NmTermination::ShrinkFailure;
                    break;
                }
            }
        }
        // Like nmmin, the limit is tested after the iteration. It counts
        // function evaluations, not simplex iterations.
        if evaluations > max_evaluations {
            break;
        }
    }
    if evaluations > max_evaluations {
        termination = NmTermination::EvaluationLimit;
    }
    Ok(NmFit {
        parameters: vertices[best].clone(),
        value: values[best],
        evaluations,
        termination,
    })
}
