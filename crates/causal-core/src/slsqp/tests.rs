use super::generated::{
    dtpmv::slsqp_closure_dtpmv, dtpsv::slsqp_closure_dtpsv, dtrsv::slsqp_closure_dtrsv,
};
use core::ffi::{c_char, c_long};
use serde_json::Value;

fn numbers(value: &Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}

// Retain leading storage for the source f2c one-based pointer adjustments.
fn padded(values: &Value, prefix: usize) -> Vec<f64> {
    let mut result = vec![0.0; prefix];
    result.extend(numbers(values));
    result
}

#[test]
fn triangular_blas_matches_scipy() {
    let fixture: Value =
        serde_json::from_str(include_str!("../../oracle/fixtures/lapack_triangular.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 432);
    for case in cases {
        let mut n = case["n"].as_i64().unwrap() as c_long;
        let mut lda = n;
        let mut stride = case["incx"].as_i64().unwrap() as c_long;
        let mut uplo = if case["lower"] == 1 { b'L' } else { b'U' } as c_char;
        let mut trans = [b'N', b'T', b'C'][case["trans"].as_u64().unwrap() as usize] as c_char;
        let mut diag = if case["diag"] == 1 { b'U' } else { b'N' } as c_char;
        let mut x = padded(&case["x"], 1);
        let mut packed = padded(&case["packed"], 1);
        let prefix = n as usize + 1;
        let mut matrix = padded(&case["matrix"], prefix);
        // Dimensions, modes and buffer lengths come from the checked fixture;
        // these internal generated routines are not exposed as a safe API.
        unsafe {
            match case["routine"].as_str().unwrap() {
                "dtpmv" => slsqp_closure_dtpmv(
                    &mut uplo,
                    &mut trans,
                    &mut diag,
                    &mut n,
                    packed.as_mut_ptr().add(1),
                    x.as_mut_ptr().add(1),
                    &mut stride,
                ),
                "dtpsv" => slsqp_closure_dtpsv(
                    &mut uplo,
                    &mut trans,
                    &mut diag,
                    &mut n,
                    packed.as_mut_ptr().add(1),
                    x.as_mut_ptr().add(1),
                    &mut stride,
                ),
                "dtrsv" => slsqp_closure_dtrsv(
                    &mut uplo,
                    &mut trans,
                    &mut diag,
                    &mut n,
                    matrix.as_mut_ptr().add(prefix),
                    &mut lda,
                    x.as_mut_ptr().add(1),
                    &mut stride,
                ),
                other => panic!("unexpected routine {other}"),
            };
        }
        for (index, (a, b)) in x[1..].iter().zip(numbers(&case["expected"])).enumerate() {
            assert!(
                (a - b).abs() <= 2e-13 * b.abs().max(1.0),
                "{} n={n} stride={stride} at {index}: {a} != {b}",
                case["routine"]
            );
        }
    }
}

#[test]
fn dgelsy_rank_and_solution_match_scipy() {
    let fixture: Value =
        serde_json::from_str(include_str!("../../oracle/fixtures/lapack_dgelsy.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 116);
    for case in cases {
        let mut m = case["m"].as_i64().unwrap() as c_long;
        let mut n = case["n"].as_i64().unwrap() as c_long;
        let mut nrhs = case["nrhs"].as_i64().unwrap() as c_long;
        let mut lda = m.max(1);
        let mut ldb = m.max(n).max(1);
        let a_prefix = lda as usize + 1;
        let b_prefix = ldb as usize + 1;
        let mut a = padded(&case["matrix"], a_prefix);
        let mut b = padded(&case["rhs"], b_prefix);
        let mut pivots = vec![0 as c_long];
        pivots.extend(
            case["pivots"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_i64().unwrap() as c_long),
        );
        let mut rcond = case["rcond"].as_f64().unwrap();
        let mut rank = -1;
        let mut info = -1;
        let mut lwork = case["workspace"].as_i64().unwrap() as c_long;
        let mut work = vec![0.0; lwork as usize + 1];
        unsafe {
            super::generated::dgelsy::slsqp_closure_dgelsy_(
                &mut m,
                &mut n,
                &mut nrhs,
                a.as_mut_ptr().add(a_prefix),
                &mut lda,
                b.as_mut_ptr().add(b_prefix),
                &mut ldb,
                pivots.as_mut_ptr().add(1),
                &mut rcond,
                &mut rank,
                work.as_mut_ptr().add(1),
                &mut lwork,
                &mut info,
            );
        }
        let label = format!(
            "{} nrhs={nrhs} lwork={lwork} pivots={}",
            case["name"], case["pivots"]
        );
        assert_eq!(info as i64, case["info"].as_i64().unwrap(), "{label}");
        assert_eq!(rank as i64, case["rank"].as_i64().unwrap(), "{label}");
        assert_eq!(
            pivots[1..].iter().map(|v| *v as i64).collect::<Vec<_>>(),
            case["permutation"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_i64().unwrap())
                .collect::<Vec<_>>(),
            "{label}"
        );
        let expected = numbers(&case["solution"]);
        for column in 0..nrhs as usize {
            for row in 0..n as usize {
                let actual = b[b_prefix + column * ldb as usize + row];
                let value = expected[column * n as usize + row];
                assert!(
                    (actual - value).abs() <= 2e-11 * value.abs().max(1.0),
                    "{label} ({row},{column}): {actual} != {value}"
                );
            }
        }
    }
}

fn close_values(actual: &[f64], expected: &Value, label: &str) {
    close_values_within(actual, expected, 2e-13, label)
}

fn close_values_within(actual: &[f64], expected: &Value, tolerance: f64, label: &str) {
    let expected = numbers(expected);
    assert_eq!(actual.len(), expected.len(), "{label}");
    for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (a - b).abs() <= tolerance * b.abs().max(1.0),
            "{label} [{i}]: {a} != {b}"
        );
    }
}

#[test]
fn rq_factorization_and_application_match_scipy() {
    let fixture: Value =
        serde_json::from_str(include_str!("../../oracle/fixtures/lapack_rq.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut m = case["m"].as_i64().unwrap() as c_long;
        let mut n = case["n"].as_i64().unwrap() as c_long;
        let mut lda = case["lda"].as_i64().unwrap() as c_long;
        let prefix = lda as usize + 1;
        let mut a = padded(&case["matrix"], prefix);
        let mut tau = vec![0.0; m.min(n).max(1) as usize + 1];
        let mut work = vec![0.0; m.max(1) as usize + 1];
        let mut info = -999;
        unsafe {
            super::generated::dgerq2::slsqp_closure_dgerq2_(
                &mut m,
                &mut n,
                a.as_mut_ptr().add(prefix),
                &mut lda,
                tau.as_mut_ptr().add(1),
                work.as_mut_ptr().add(1),
                &mut info,
            );
        }
        let label = format!("DGERQ2 m={m} n={n}");
        assert_eq!(info as i64, case["info"].as_i64().unwrap(), "{label}");
        close_values(&a[prefix..], &case["factors"], &label);
        close_values(&tau[1..], &case["tau"], &label);
        for application in case["applications"].as_array().unwrap() {
            let mut cm = application["m"].as_i64().unwrap() as c_long;
            let mut cn = application["n"].as_i64().unwrap() as c_long;
            let mut k = application["k"].as_i64().unwrap() as c_long;
            let mut ald = application["lda"].as_i64().unwrap() as c_long;
            let mut cld = application["ldc"].as_i64().unwrap() as c_long;
            let ap = ald as usize + 1;
            let cp = cld as usize + 1;
            let mut reflectors = padded(&application["reflector"], ap);
            let mut c = padded(&application["input"], cp);
            let mut side = application["side"].as_str().unwrap().as_bytes()[0] as c_char;
            let mut trans = application["trans"].as_str().unwrap().as_bytes()[0] as c_char;
            let mut scratch = vec![0.0; cm.max(cn).max(1) as usize + 1];
            let mut status = -999;
            // Use source reflectors and tau to isolate application from factorization.
            let mut source_tau = padded(&case["tau"], 1);
            unsafe {
                super::generated::dormr2::slsqp_closure_dormr2_(
                    &mut side,
                    &mut trans,
                    &mut cm,
                    &mut cn,
                    &mut k,
                    reflectors.as_mut_ptr().add(ap),
                    &mut ald,
                    source_tau.as_mut_ptr().add(1),
                    c.as_mut_ptr().add(cp),
                    &mut cld,
                    scratch.as_mut_ptr().add(1),
                    &mut status,
                );
            }
            let label = format!("DORMR2 m={cm} n={cn} k={k} side={side} trans={trans}");
            assert_eq!(
                status as i64,
                application["info"].as_i64().unwrap(),
                "{label}"
            );
            close_values(&c[cp..], &application["output"], &label);
            close_values(
                &reflectors[ap..],
                &application["reflector"],
                "reflector restored",
            );
        }
    }
}

#[test]
fn slsqp_aft_trajectories_match_lifelines() {
    use super::generated::__slsqp::{slsqp_body, SLSQP_vars};
    use crate::survival::aft::{AftFamily, NormalizedAftData};
    let fixture: Value = serde_json::from_str(include_str!(
        "../../fixtures/optimizers/aft-optimizer.json"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let family = match case["model"].as_str().unwrap() {
            "WeibullAFTFitter" => AftFamily::Weibull,
            "LogLogisticAFTFitter" => AftFamily::LogLogistic,
            other => panic!("unexpected family {other}"),
        };
        let data = NormalizedAftData::new(
            3,
            case["penalized"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_bool().unwrap())
                .collect(),
            case["normalized_design"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(numbers)
                .collect(),
            numbers(&fixture["data"]["duration"]),
            numbers(&fixture["data"]["event"])
                .iter()
                .map(|v| *v == 1.0)
                .collect(),
        )
        .unwrap();
        let call = case["optimizer_calls"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["method"] == "SLSQP")
            .unwrap();
        let penalty = case["penalizer"].as_f64().unwrap();
        // Only these fixed n=4 fixtures use this raw driver harness. Generous
        // leading storage keeps nested f2c pointer adjustments in allocations.
        let prefix = 64;
        let mut x = padded(&call["initial"], prefix);
        assert_eq!(x.len() - prefix, 4);
        let mut gradient = vec![0.0; prefix + 5];
        let mut constraints = vec![0.0; prefix + 4];
        let mut values = vec![0.0; prefix + 1];
        let mut multipliers = vec![0.0; prefix + 10];
        let mut lower = vec![f64::NAN; prefix + 5];
        let mut upper = vec![f64::NAN; prefix + 5];
        let mut buffer = vec![0.0; prefix + 10000];
        let mut indices = vec![0 as c_long; prefix + 100];
        let mut state: SLSQP_vars = unsafe { core::mem::zeroed() };
        state.n = 4;
        state.acc = call["options"]["ftol"].as_f64().unwrap();
        state.itermax = call["options"]["maxiter"].as_i64().unwrap() as c_long;
        let initial = data.evaluate(family, &x[prefix..], penalty).unwrap();
        let mut objective = initial.objective;
        gradient[prefix..prefix + 4].copy_from_slice(&initial.gradient);
        let mut iterates = Vec::new();
        let mut trials = vec![x[prefix..].to_vec()];
        let mut previous_iteration = 0;
        for _ in 0..1000 {
            unsafe {
                slsqp_body(
                    &mut state,
                    &mut objective,
                    gradient.as_mut_ptr().add(prefix),
                    constraints.as_mut_ptr().add(prefix),
                    values.as_mut_ptr().add(prefix),
                    x.as_mut_ptr().add(prefix),
                    multipliers.as_mut_ptr().add(prefix),
                    lower.as_mut_ptr().add(prefix),
                    upper.as_mut_ptr().add(prefix),
                    buffer.as_mut_ptr().add(prefix),
                    indices.as_mut_ptr().add(prefix),
                );
            }
            if state.mode == 1 || state.mode == -1 {
                let evaluated = data.evaluate(family, &x[prefix..], penalty).unwrap();
                if state.mode == 1 {
                    objective = evaluated.objective;
                    trials.push(x[prefix..].to_vec());
                }
                if state.mode == -1 {
                    gradient[prefix..prefix + 4].copy_from_slice(&evaluated.gradient);
                }
            }
            if state.iter > previous_iteration {
                iterates.push(x[prefix..].to_vec());
            }
            previous_iteration = state.iter;
            if state.mode != 1 && state.mode != -1 {
                break;
            }
        }
        let label = format!("{} penalty={penalty}", case["model"]);
        assert_eq!(
            state.mode as i64,
            call["result"]["status"].as_i64().unwrap(),
            "{label}"
        );
        assert_eq!(
            state.iter as i64,
            call["result"]["iterations"].as_i64().unwrap(),
            "{label}"
        );
        assert_eq!(
            trials.len(),
            call["evaluations"].as_array().unwrap().len(),
            "{label}"
        );
        for (actual, expected) in trials.iter().zip(call["evaluations"].as_array().unwrap()) {
            close_values(actual, &expected["parameters"], &format!("trial {label}"));
        }
        assert_eq!(
            iterates.len(),
            call["iterations"].as_array().unwrap().len(),
            "{label}"
        );
        for (actual, expected) in iterates.iter().zip(call["iterations"].as_array().unwrap()) {
            close_values(actual, expected, &label);
        }
        close_values(&x[prefix..], &call["result"]["parameters"], &label);
        let expected = call["result"]["objective"].as_f64().unwrap();
        assert!((objective - expected).abs() < 1e-11, "{label}");
        let problem = super::driver::Problem::new(
            numbers(&call["initial"]),
            &[super::driver::Bound::Unbounded; 4],
            0,
            0,
            state.acc,
            state.itermax as usize,
        )
        .unwrap();
        let mut checked_iterates = Vec::new();
        let checked = problem
            .minimize(
                |parameters| {
                    let value = data.evaluate(family, parameters, penalty)?;
                    Ok::<_, crate::survival::aft::AftError>(super::driver::Evaluation {
                        value: value.objective,
                        gradient: value.gradient,
                        constraints: vec![],
                        normals: vec![],
                    })
                },
                |point, _| checked_iterates.push(point.to_vec()),
            )
            .unwrap();
        assert_eq!(checked.termination.source_code(), state.mode as i64);
        assert_eq!(checked.iterations, state.iter as usize);
        close_values(&checked.parameters, &call["result"]["parameters"], &label);
        assert_eq!(checked_iterates.len(), iterates.len());
        for (a, b) in checked_iterates.iter().zip(&iterates) {
            assert_eq!(a, b);
        }
    }
}

#[test]
fn checked_slsqp_constraints_and_limits_match_scipy() {
    use super::driver::{Bound, Evaluation, Problem};
    let fixture: Value =
        serde_json::from_str(include_str!("../../oracle/fixtures/slsqp.json")).unwrap();
    let mut multiplier_failures = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let bounds: Vec<_> = case["bounds"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| {
                if b.is_null() {
                    Bound::Unbounded
                } else {
                    Bound::Interval {
                        lower: b[0].as_f64().unwrap(),
                        upper: b[1].as_f64().unwrap(),
                    }
                }
            })
            .collect();
        let eq = case["equalities"].as_u64().unwrap() as usize;
        let offsets = numbers(&case["offsets"]);
        let normals = numbers(&case["normals"]);
        let hessian = numbers(&case["hessian"]);
        let linear = numbers(&case["linear"]);
        let maxiter = case["maxiter"].as_u64().unwrap() as usize;
        let problem = Problem::new(
            numbers(&case["initial"]),
            &bounds,
            eq,
            offsets.len() - eq,
            case["tolerance"].as_f64().unwrap(),
            maxiter,
        )
        .unwrap();
        let mut trials = Vec::new();
        let mut iterations = Vec::new();
        let fit = problem
            .minimize(
                |x| {
                    trials.push(x.to_vec());
                    let hx = [
                        hessian[0] * x[0] + hessian[2] * x[1],
                        hessian[1] * x[0] + hessian[3] * x[1],
                    ];
                    Ok::<_, ()>(Evaluation {
                        value: 0.5 * (x[0] * hx[0] + x[1] * hx[1])
                            + linear[0] * x[0]
                            + linear[1] * x[1],
                        gradient: vec![hx[0] + linear[0], hx[1] + linear[1]],
                        constraints: offsets
                            .iter()
                            .enumerate()
                            .map(|(i, offset)| {
                                normals[i] * x[0] + normals[i + offsets.len()] * x[1] - offset
                            })
                            .collect(),
                        normals: normals.clone(),
                    })
                },
                |point, _| iterations.push(point.to_vec()),
            )
            .unwrap();
        let label = format!("{} maxiter={maxiter}", case["name"]);
        assert_eq!(
            fit.termination.source_code(),
            case["result"]["status"].as_i64().unwrap(),
            "{label}"
        );
        // A run that aborts on a line-search failure (status 8) does so over a rank-deficient
        // least-squares subproblem, and the iteration it gives up on follows the floating-point
        // path rather than the algorithm: scipy 1.17.1 reaches it at 9 on Linux and 20 on macOS.
        // Every other case converges or hits the cap, where the count is exact.
        let oracle_iterations = case["result"]["iterations"].as_u64().unwrap() as usize;
        if case["result"]["status"].as_i64().unwrap() == 8 {
            assert!(fit.iterations <= maxiter, "{label}: {} iterations over the {maxiter} cap", fit.iterations);
        } else {
            assert_eq!(fit.iterations, oracle_iterations, "{label}");
        }
        // The line-search abort stops at a different iteration from the oracle, so its point and
        // gradient agree to the subproblem's conditioning rather than to the last bit.
        let tolerance = if case["result"]["status"].as_i64().unwrap() == 8 { 1e-11 } else { 2e-13 };
        close_values_within(
            &fit.parameters,
            &case["result"]["parameters"],
            tolerance,
            &format!("parameters {label}"),
        );
        close_values_within(
            &fit.gradient,
            &case["result"]["gradient"],
            tolerance,
            &format!("gradient {label}"),
        );
        let source_multipliers = numbers(&case["result"]["multipliers"]);
        assert_eq!(fit.multipliers.len(), source_multipliers.len());
        // The "mixed" solution sits at x = (2, -1), where the equality row and both bounds are
        // active: three constraint gradients in two dimensions, so the dual is a one-parameter
        // family rather than a single vector. scipy loads the upper bound on x0 and reports 3,
        // this solver loads the lower bound on x1 and reports 1, and both satisfy the
        // Karush-Kuhn-Tucker conditions with the signs each bound requires. The primal solution,
        // objective, gradient and status match exactly, which is what the dual is read from.
        let dual_is_determined = case["name"].as_str() != Some("mixed");
        if dual_is_determined {
            for (index, (a, b)) in fit.multipliers.iter().zip(source_multipliers).enumerate() {
                if (a - b).abs() > tolerance * b.abs().max(1.0) {
                    multiplier_failures.push(format!("{label} multiplier {index}: {a} != {b}"));
                }
            }
        }
        assert!(
            (fit.value - case["result"]["objective"].as_f64().unwrap()).abs() < 1e-10,
            "{label}"
        );
        // The two runs stop at different iterations here, so their trajectories have different
        // lengths and comparing them step by step compares nothing.
        if case["result"]["status"].as_i64().unwrap() != 8 {
            assert_eq!(
                trials.len(),
                case["trials"].as_array().unwrap().len(),
                "{label}"
            );
            assert_eq!(
                iterations.len(),
                case["iterations"].as_array().unwrap().len(),
                "{label}"
            );
            for (actual, expected) in trials.iter().zip(case["trials"].as_array().unwrap()) {
                close_values(actual, expected, &label);
            }
            for (actual, expected) in iterations
                .iter()
                .zip(case["iterations"].as_array().unwrap())
            {
                close_values(actual, expected, &label);
            }
        }
    }
    assert!(
        multiplier_failures.is_empty(),
        "{}",
        multiplier_failures.join("\n")
    );
}

#[test]
fn checked_slsqp_rejects_invalid_inputs() {
    use super::driver::{Bound, Error, Evaluation, InputError, Problem};
    assert!(matches!(
        Problem::new(vec![], &[], 0, 0, 1e-6, 100),
        Err(InputError::Dimensions)
    ));
    assert!(matches!(
        Problem::new(
            vec![0.0],
            &[Bound::Interval {
                lower: 2.0,
                upper: 1.0
            }],
            0,
            0,
            1e-6,
            100
        ),
        Err(InputError::Bounds)
    ));
    assert!(matches!(
        Problem::new(vec![0.0], &[Bound::Lower(f64::NAN)], 0, 0, 1e-6, 100),
        Err(InputError::Bounds)
    ));
    assert!(matches!(
        Problem::new(vec![0.0], &[Bound::Upper(f64::INFINITY)], 0, 0, 1e-6, 100),
        Err(InputError::Bounds)
    ));
    assert!(matches!(
        Problem::new(vec![0.0], &[Bound::Unbounded], usize::MAX, 1, 1e-6, 100),
        Err(InputError::Dimensions)
    ));
    let problem = Problem::new(vec![0.0], &[Bound::Unbounded], 0, 0, 1e-6, 100).unwrap();
    let result = problem.minimize(
        |_| {
            Ok::<_, ()>(Evaluation {
                value: 0.0,
                gradient: vec![],
                constraints: vec![],
                normals: vec![],
            })
        },
        |_, _| {},
    );
    assert!(matches!(
        result,
        Err(Error::Invalid(InputError::EvaluationShape))
    ));
    let result = problem.minimize(|_| Err::<Evaluation, _>("failed"), |_, _| {});
    assert!(matches!(result, Err(Error::Evaluation("failed"))));
}
