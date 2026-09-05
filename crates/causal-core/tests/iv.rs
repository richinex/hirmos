use hirmos_causal_core::nprandom::Mt19937;
use hirmos_causal_core::{
    identify_instrument_set, instrumental_variable, instrumental_variable_with_progress, Dag,
    DowhyBootstrap, IvEstimator, IvError, IvInput, IvOptions,
};
use nalgebra::DMatrix;
use serde_json::Value;

fn close(label: &str, got: f64, want: f64, tolerance: f64) {
    assert!(
        (got - want).abs() <= tolerance,
        "{label}: got {got:.16e}, oracle {want:.16e}, deviation {:.3e}",
        (got - want).abs(),
    );
}

fn floats(value: &Value) -> Vec<f64> {
    serde_json::from_value(value.clone()).unwrap()
}

/// NumPy's legacy global `RandomState` as `dowhy.datasets` consumes it: 53-bit doubles from two
/// 32-bit words, polar Gaussians with the cached second value, `binomial(1, p)` by inversion, and
/// `choice([0, 1], p=[1 - p, p])` through a cumulative-distribution search.
struct LegacyStream {
    rng: Mt19937,
    gauss: Option<f64>,
}

impl LegacyStream {
    fn seeded(seed: u32) -> Self {
        Self {
            rng: Mt19937::seeded(seed),
            gauss: None,
        }
    }

    fn random_sample(&mut self) -> f64 {
        self.rng.next_f64()
    }

    fn uniform(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.random_sample()
    }

    fn normal(&mut self, loc: f64, scale: f64) -> f64 {
        loc + scale * self.rng.standard_normal(&mut self.gauss)
    }

    fn binomial_inversion(&mut self, n: u64, p: f64) -> u64 {
        let q = 1.0 - p;
        let qn = (n as f64 * q.ln()).exp();
        let np = n as f64 * p;
        let bound = (n as f64).min(np + 10.0 * (np * q + 1.0).sqrt()) as u64;
        let mut x = 0;
        let mut px = qn;
        let mut u = self.random_sample();
        while u > px {
            x += 1;
            if x > bound {
                x = 0;
                px = qn;
                u = self.random_sample();
            } else {
                u -= px;
                px = ((n - x + 1) as f64 * p * px) / (x as f64 * q);
            }
        }
        x
    }

    /// `legacy_random_binomial` for the small-mean case the datasets use.
    fn binomial(&mut self, n: u64, p: f64) -> u64 {
        if n == 0 || p == 0.0 {
            return 0;
        }
        if p <= 0.5 {
            assert!(p * n as f64 <= 30.0, "BTPE branch is not ported");
            self.binomial_inversion(n, p)
        } else {
            let q = 1.0 - p;
            assert!(q * n as f64 <= 30.0, "BTPE branch is not ported");
            n - self.binomial_inversion(n, q)
        }
    }

    /// `dowhy.datasets.convert_to_binary` on one value: `choice([0, 1], 1, p=[1 - p, p])`.
    fn convert_to_binary(&mut self, value: f64) -> f64 {
        let p = 1.0 / (1.0 + (-value).exp());
        let total = (1.0 - p) + p;
        let threshold = (1.0 - p) / total;
        let draw = self.random_sample();
        if draw >= threshold {
            1.0
        } else {
            0.0
        }
    }

    /// `np.vectorize(convert_to_binary)` evaluates the first element once more to learn the
    /// output type before it sweeps the array, so one draw is spent and discarded.
    fn vectorize_binary(&mut self, values: &[f64]) -> Vec<f64> {
        self.convert_to_binary(values[0]);
        values
            .iter()
            .map(|&value| self.convert_to_binary(value))
            .collect()
    }
}

struct Dataset {
    columns: Vec<String>,
    /// Row-major values, `rows` by `columns.len()`.
    data: Vec<Vec<f64>>,
    treatments: Vec<usize>,
    instruments: Vec<usize>,
    outcome: usize,
    ate: f64,
    graph: Dag,
}

impl Dataset {
    fn column(&self, index: usize) -> Vec<f64> {
        self.data.iter().map(|row| row[index]).collect()
    }

    fn matrix(&self, columns: &[usize]) -> DMatrix<f64> {
        DMatrix::from_fn(self.data.len(), columns.len(), |row, column| {
            self.data[row][columns[column]]
        })
    }

    fn column_index(&self, name: &str) -> usize {
        self.columns.iter().position(|column| column == name).unwrap()
    }
}

fn mean(values: &[f64]) -> f64 {
    hirmos_causal_core::numpy_mean(values)
}

/// A BLAS inner product over at most two terms, as Accelerate's `dgemv`/`dgemm` accumulate it:
/// the second product fused into the first.
fn blas_dot(terms: &[(f64, f64)]) -> f64 {
    match terms {
        [] => 0.0,
        [(a, b)] => a * b,
        [(a0, b0), (a1, b1)] => a1.mul_add(*b1, a0 * b0),
        _ => panic!("only one- and two-term products are ported"),
    }
}

/// `dowhy.datasets.create_gml_graph`: every treatment into the outcome, every common cause into
/// each treatment and the outcome, every instrument into each treatment.
fn dowhy_graph(columns: &[String]) -> Dag {
    let index = |prefix: &str| -> Vec<usize> {
        columns
            .iter()
            .enumerate()
            .filter(|(_, name)| name.starts_with(prefix))
            .map(|(index, _)| index)
            .collect()
    };
    let treatments = index("v");
    let outcome = columns.iter().position(|name| name == "y").unwrap();
    let mut edges = Vec::new();
    for &treatment in &treatments {
        edges.push((treatment, outcome));
        for &cause in &index("W") {
            edges.push((cause, treatment));
        }
        for &instrument in &index("Z") {
            edges.push((instrument, treatment));
        }
    }
    for &cause in &index("W") {
        edges.push((cause, outcome));
    }
    Dag::new(columns.len(), &edges)
}

#[derive(Clone, Copy)]
struct LinearConfig {
    common_causes: usize,
    instruments: usize,
    treatments: usize,
    binary_treatment: bool,
}

/// `dowhy.datasets.linear_dataset` for the settings the upstream test sweeps: no effect
/// modifiers, no front-door variables, no discrete variables, `beta` recycled to every treatment.
fn linear_dataset(stream: &mut LegacyStream, config: LinearConfig, rows: usize, beta: f64) -> Dataset {
    let k = config.treatments;
    let betas = vec![beta; k];
    let mut common = Vec::new();
    let mut c1 = Vec::new();
    let mut c2 = Vec::new();
    if config.common_causes > 0 {
        assert_eq!(config.common_causes, 1, "identity SVD only holds for one common cause");
        let range = 0.5 + beta.abs() * 0.5;
        let means: Vec<f64> = (0..config.common_causes)
            .map(|_| stream.uniform(-1.0, 1.0))
            .collect();
        common = (0..rows)
            .map(|_| {
                (0..config.common_causes)
                    .map(|column| stream.normal(0.0, 1.0) * 1.0 + means[column])
                    .collect::<Vec<f64>>()
            })
            .collect();
        c1 = (0..config.common_causes * k)
            .map(|_| stream.uniform(0.0, range))
            .collect();
        c2 = (0..config.common_causes)
            .map(|_| stream.uniform(0.0, range))
            .collect();
    }
    let mut instruments = vec![Vec::new(); rows];
    let mut cz = Vec::new();
    if config.instruments > 0 {
        let range_cz = 1.0 + beta.abs();
        let p: Vec<f64> = (0..config.instruments)
            .map(|_| stream.uniform(0.0, 1.0))
            .collect();
        instruments = vec![vec![0.0; config.instruments]; rows];
        for i in 0..config.instruments {
            for row in instruments.iter_mut() {
                row[i] = if i % 2 == 0 {
                    stream.binomial(1, p[i]) as f64
                } else {
                    stream.uniform(0.0, 1.0)
                };
            }
        }
        cz = (0..config.instruments * k)
            .map(|_| stream.uniform(range_cz - range_cz * 0.05, range_cz + range_cz * 0.05))
            .collect();
    }
    let mut treatment: Vec<Vec<f64>> = (0..rows)
        .map(|_| (0..k).map(|_| stream.normal(0.0, 1.0)).collect())
        .collect();
    if config.common_causes > 0 {
        for (row, w) in treatment.iter_mut().zip(&common) {
            for (j, value) in row.iter_mut().enumerate() {
                let terms: Vec<(f64, f64)> = (0..config.common_causes).map(|c| (w[c], c1[c * k + j])).collect();
                *value += blas_dot(&terms);
            }
        }
    }
    if config.instruments > 0 {
        for (row, z) in treatment.iter_mut().zip(&instruments) {
            for (j, value) in row.iter_mut().enumerate() {
                let terms: Vec<(f64, f64)> = (0..config.instruments).map(|i| (z[i], cz[i * k + j])).collect();
                *value += blas_dot(&terms);
            }
        }
    }
    if config.binary_treatment {
        let flat: Vec<f64> = treatment.iter().flatten().copied().collect();
        let converted = stream.vectorize_binary(&flat);
        treatment = converted.chunks(k).map(<[f64]>::to_vec).collect();
    }
    let compute_y = |stream: &mut LegacyStream, treatment: &[Vec<f64>]| -> Vec<f64> {
        let mut y: Vec<f64> = (0..rows).map(|_| stream.normal(0.0, 0.01)).collect();
        for (row, t) in y.iter_mut().zip(treatment) {
            let terms: Vec<(f64, f64)> = t.iter().copied().zip(betas.iter().copied()).collect();
            *row += blas_dot(&terms);
        }
        if config.common_causes > 0 {
            for (row, w) in y.iter_mut().zip(&common) {
                let terms: Vec<(f64, f64)> = (0..config.common_causes).map(|c| (w[c], c2[c])).collect();
                *row += blas_dot(&terms);
            }
        }
        y
    };
    let outcome = compute_y(stream, &treatment);
    let ones = vec![vec![1.0; k]; rows];
    let zeros = vec![vec![0.0; k]; rows];
    let treated = compute_y(stream, &ones);
    let untreated = compute_y(stream, &zeros);
    let differences: Vec<f64> = treated
        .iter()
        .zip(&untreated)
        .map(|(treated, untreated)| treated - untreated)
        .collect();
    let ate = mean(&differences);

    let mut columns = Vec::new();
    columns.extend((0..config.instruments).map(|i| format!("Z{i}")));
    columns.extend((0..config.common_causes).map(|i| format!("W{i}")));
    columns.extend((0..k).map(|i| format!("v{i}")));
    columns.push("y".to_owned());
    let data: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            let mut values = instruments[row].clone();
            if config.common_causes > 0 {
                values.extend(&common[row]);
            }
            values.extend(&treatment[row]);
            values.push(outcome[row]);
            values
        })
        .collect();
    let graph = dowhy_graph(&columns);
    Dataset {
        treatments: (0..k).map(|i| config.instruments + config.common_causes + i).collect(),
        instruments: (0..config.instruments).collect(),
        outcome: columns.len() - 1,
        columns,
        data,
        ate,
        graph,
    }
}

/// `dowhy.datasets.simple_iv_dataset`: one standard-normal instrument, one uniform common cause.
fn simple_iv_dataset(stream: &mut LegacyStream, rows: usize, beta: f64, binary_treatment: bool) -> Dataset {
    let c1 = stream.uniform(0.0, 1.0);
    let c2 = stream.uniform(0.0, 1.0);
    let range_cz = 1.0 + beta.abs();
    let cz = stream.uniform(range_cz - range_cz * 0.05, range_cz + range_cz * 0.05);
    let common: Vec<f64> = (0..rows).map(|_| stream.uniform(0.0, 1.0)).collect();
    let instrument: Vec<f64> = (0..rows).map(|_| stream.normal(0.0, 1.0)).collect();
    let mut treatment: Vec<f64> = (0..rows)
        .map(|row| (stream.normal(0.0, 1.0) + instrument[row] * cz) + common[row] * c1)
        .collect();
    if binary_treatment {
        treatment = stream.vectorize_binary(&treatment);
    }
    let outcome: Vec<f64> = (0..rows)
        .map(|row| treatment[row] * beta + common[row] * c2)
        .collect();
    let differences: Vec<f64> = (0..rows)
        .map(|row| (1.0 * beta + common[row] * c2) - (0.0 * beta + common[row] * c2))
        .collect();
    let columns: Vec<String> = ["Z0", "W0", "v0", "y"].map(str::to_owned).to_vec();
    let graph = dowhy_graph(&columns);
    Dataset {
        columns,
        data: (0..rows)
            .map(|row| vec![instrument[row], common[row], treatment[row], outcome[row]])
            .collect(),
        treatments: vec![2],
        instruments: vec![0],
        outcome: 3,
        ate: mean(&differences),
        graph,
    }
}

fn dataset_matches_snapshot(dataset: &Dataset, record: &Value, label: &str) {
    let columns: Vec<String> = serde_json::from_value(record["columns"].clone()).unwrap();
    assert_eq!(dataset.columns, columns, "{label}: columns");
    let head: Vec<Vec<f64>> = serde_json::from_value(record["head"].clone()).unwrap();
    let tail: Vec<Vec<f64>> = serde_json::from_value(record["tail"].clone()).unwrap();
    let rows = dataset.data.len();
    for (index, row) in head.iter().enumerate() {
        assert_eq!(dataset.data[index], *row, "{label}: head row {index}");
    }
    for (index, row) in tail.iter().enumerate() {
        let source = rows - tail.len() + index;
        assert_eq!(dataset.data[source], *row, "{label}: tail row {index}");
    }
    let sums = floats(&record["column_sums"]);
    for (column, want) in sums.iter().enumerate() {
        let got = dataset.column(column).iter().fold(0.0, |total, value| total + value);
        close(&format!("{label}: sum of column {column}"), got, *want, 1e-9 * want.abs().max(1.0));
    }
    close(&format!("{label}: true ATE"), dataset.ate, record["true_ate"].as_f64().unwrap(), 1e-12);
    let instruments: Vec<String> = serde_json::from_value(record["instruments"].clone()).unwrap();
    let want: Vec<usize> = instruments.iter().map(|name| dataset.column_index(name)).collect();
    let got = identify_instrument_set(&dataset.graph, &dataset.treatments, &[dataset.outcome]).unwrap();
    assert_eq!(got, want, "{label}: instruments");
}

fn estimator_named(name: &str) -> IvEstimator {
    match name {
        "wald-ratio" => IvEstimator::WaldRatio,
        "covariance-ratio" => IvEstimator::CovarianceRatio,
        "two-stage-least-squares" => IvEstimator::TwoStageLeastSquares,
        other => panic!("unknown estimator {other}"),
    }
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/iv_dowhy.json")).unwrap()
}

#[test]
fn instrument_search_matches_get_instruments_on_hand_built_graphs() {
    // Z -> X -> Y with a latent U into X and Y: Z is the instrument, U is not.
    let classic = Dag::new(4, &[(0, 1), (1, 2), (3, 1), (3, 2)]);
    assert_eq!(identify_instrument_set(&classic, &[1], &[2]).unwrap(), vec![0]);

    // A parent of X that also reaches Y directly fails exclusion.
    let direct = Dag::new(3, &[(0, 1), (1, 2), (0, 2)]);
    assert_eq!(identify_instrument_set(&direct, &[1], &[2]).unwrap(), Vec::<usize>::new());

    // A parent of X that descends from a cause of Y fails as-if-random.
    let confounded = Dag::new(4, &[(3, 0), (3, 2), (0, 1), (1, 2)]);
    assert_eq!(identify_instrument_set(&confounded, &[1], &[2]).unwrap(), Vec::<usize>::new());

    // The reference returns every qualifying parent, observed or not.
    let latent_instrument = Dag::new(4, &[(0, 1), (3, 1), (1, 2)]);
    assert_eq!(identify_instrument_set(&latent_instrument, &[1], &[2]).unwrap(), vec![0, 3]);

    assert!(identify_instrument_set(&classic, &[], &[2]).is_err());
    assert!(identify_instrument_set(&classic, &[1], &[1]).is_err());
    assert!(identify_instrument_set(&classic, &[9], &[2]).is_err());
}

#[test]
fn upstream_dowhy_test_replays_at_its_seed_and_configuration_order() {
    let fixture = fixture();
    let upstream = &fixture["upstream_test"];
    let rows = upstream["rows"].as_u64().unwrap() as usize;
    let beta = upstream["beta"].as_f64().unwrap();
    let tolerance = upstream["error_tolerance"].as_f64().unwrap();
    let mut stream = LegacyStream::seeded(upstream["seed"].as_u64().unwrap() as u32);
    let mut max_estimate_deviation = 0.0_f64;

    for (index, record) in upstream["configs"].as_array().unwrap().iter().enumerate() {
        let config = &record["config"];
        let settings = LinearConfig {
            common_causes: config["num_common_causes"].as_u64().unwrap() as usize,
            instruments: config["num_instruments"].as_u64().unwrap() as usize,
            treatments: config["num_treatments"].as_u64().unwrap() as usize,
            binary_treatment: config["treatment_is_binary"].as_bool().unwrap(),
        };
        let label = format!("configuration {index}");
        let dataset = linear_dataset(&mut stream, settings, rows, beta);
        dataset_matches_snapshot(&dataset, record, &label);

        let treatments = dataset.matrix(&dataset.treatments);
        let instruments = dataset.matrix(&dataset.instruments);
        let outcome = dataset.column(dataset.outcome);
        let result = instrumental_variable(
            IvInput {
                treatments: &treatments,
                instruments: &instruments,
                outcome: &outcome,
            },
            IvOptions::default(),
        );
        let expected = &record["result"];
        match expected["kind"].as_str().unwrap() {
            "refusal" => {
                assert_eq!(record["expected"], "refusal");
                let error = result.expect_err(&label);
                let want = if settings.instruments == 0 {
                    IvError::NoInstruments
                } else {
                    IvError::FewerInstrumentsThanTreatments {
                        instruments: settings.instruments,
                        treatments: settings.treatments,
                    }
                };
                assert_eq!(error, want, "{label}");
            }
            "estimate" => {
                assert_eq!(record["expected"], "estimate");
                let got = result.expect(&label);
                assert_eq!(got.estimator, estimator_named(expected["estimator"].as_str().unwrap()), "{label}");
                let want = expected["value"].as_f64().unwrap();
                max_estimate_deviation = max_estimate_deviation.max((got.estimate - want).abs());
                // Two-stage least squares accumulates its 100,000-term normal equations in
                // BLAS order, which the port cannot reproduce. The summed effect the upstream
                // test asserts moves by at most 2.5e-9. With two treatments driven by the same
                // instruments the individual coefficients are nearly collinear, and the binary
                // case amplifies that order noise to 1.5e-5 per coefficient. The
                // single-instrument routes sum in NumPy's order and agree to the last few bits.
                let (estimate_tolerance, parameter_tolerance) = match got.estimator {
                    IvEstimator::TwoStageLeastSquares if settings.treatments > 1 => (1e-8, 5e-5),
                    IvEstimator::TwoStageLeastSquares => (1e-9, 1e-9),
                    IvEstimator::WaldRatio | IvEstimator::CovarianceRatio => (1e-12, 1e-12),
                };
                close(&format!("{label}: estimate"), got.estimate, want, estimate_tolerance);
                for (i, (param, want)) in got.params.iter().zip(floats(&expected["params"])).enumerate() {
                    close(&format!("{label}: parameter {i}"), *param, want, parameter_tolerance);
                }
                let parameter_deviation = got
                    .params
                    .iter()
                    .zip(floats(&expected["params"]))
                    .map(|(param, want)| (param - want).abs())
                    .fold(0.0_f64, f64::max);
                println!(
                    "{label}: {} estimate dev={:.3e}, parameter max dev={parameter_deviation:.3e}",
                    got.estimator,
                    (got.estimate - want).abs()
                );
                let within = (got.estimate - dataset.ate).abs() < dataset.ate.abs() * tolerance;
                assert!(within, "{label}: outside the upstream 40% tolerance");
                assert_eq!(within, expected["within_tolerance"].as_bool().unwrap());
            }
            other => panic!("unknown result kind {other}"),
        }
    }
    println!("upstream test: estimate max dev={max_estimate_deviation:.3e}");
}

#[test]
fn seeded_bootstrap_matches_dowhy_on_every_estimator_route() {
    let fixture = fixture();
    let rows = fixture["upstream_test"]["rows"].as_u64().unwrap() as usize;
    let beta = fixture["upstream_test"]["beta"].as_f64().unwrap();
    let bootstrap = DowhyBootstrap {
        simulations: fixture["reference"]["bootstrap_simulations"].as_u64().unwrap() as usize,
        sample_size_fraction: fixture["reference"]["bootstrap_sample_fraction"].as_f64().unwrap(),
        confidence_level: fixture["reference"]["confidence_level"].as_f64().unwrap(),
        seed: fixture["reference"]["bootstrap_seed"].as_u64().unwrap() as u32,
    };

    for case in fixture["bootstrap_cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut stream = LegacyStream::seeded(case["seed"].as_u64().unwrap() as u32);
        let dataset = match case["generator"].as_str().unwrap() {
            "linear_dataset" => linear_dataset(
                &mut stream,
                LinearConfig {
                    common_causes: 1,
                    instruments: if name == "two-stage-least-squares" { 2 } else { 1 },
                    treatments: if name == "two-stage-least-squares" { 2 } else { 1 },
                    binary_treatment: name == "wald-ratio",
                },
                rows,
                beta,
            ),
            "simple_iv_dataset" => simple_iv_dataset(&mut stream, rows, beta, name.ends_with("binary-treatment")),
            other => panic!("unknown generator {other}"),
        };
        dataset_matches_snapshot(&dataset, case, name);

        let treatments = dataset.matrix(&dataset.treatments);
        let instruments = dataset.matrix(&dataset.instruments);
        let outcome = dataset.column(dataset.outcome);
        let mut progress = Vec::new();
        let got = instrumental_variable_with_progress(
            IvInput {
                treatments: &treatments,
                instruments: &instruments,
                outcome: &outcome,
            },
            IvOptions {
                bootstrap: Some(bootstrap),
            },
            |completed, total| progress.push((completed, total)),
        )
        .unwrap();
        let expected = &case["result"];
        assert_eq!(got.estimator, estimator_named(expected["estimator"].as_str().unwrap()), "{name}");
        close(&format!("{name}: estimate"), got.estimate, expected["value"].as_f64().unwrap(), 1e-9);

        let draws = floats(&expected["bootstrap_estimates"]);
        assert_eq!(got.bootstrap_estimates.len(), draws.len());
        let mut max_draw_deviation = 0.0_f64;
        for (index, (got, want)) in got.bootstrap_estimates.iter().zip(&draws).enumerate() {
            max_draw_deviation = max_draw_deviation.max((got - want).abs());
            close(&format!("{name}: bootstrap estimate {index}"), *got, *want, 1e-8);
        }
        let interval = got.confidence_interval.unwrap();
        let want_interval = floats(&expected["confidence_interval"]);
        close(&format!("{name}: interval lower"), interval[0], want_interval[0], 1e-8);
        close(&format!("{name}: interval upper"), interval[1], want_interval[1], 1e-8);
        close(
            &format!("{name}: standard error"),
            got.standard_error.unwrap(),
            expected["standard_error"].as_f64().unwrap(),
            1e-10,
        );
        assert_eq!(progress.first(), Some(&(0, bootstrap.simulations)));
        assert_eq!(progress.last(), Some(&(bootstrap.simulations, bootstrap.simulations)));
        assert_eq!(progress.len(), bootstrap.simulations + 1);
        println!(
            "{name}: estimate dev={:.3e}, bootstrap max dev={max_draw_deviation:.3e}, CI max dev={:.3e}, SE dev={:.3e}",
            (got.estimate - expected["value"].as_f64().unwrap()).abs(),
            (interval[0] - want_interval[0]).abs().max((interval[1] - want_interval[1]).abs()),
            (got.standard_error.unwrap() - expected["standard_error"].as_f64().unwrap()).abs(),
        );
    }
}

#[test]
fn invalid_inputs_are_reported_at_the_boundary() {
    let outcome = [1.0, 2.0, 3.0, 4.0];
    let treatment = DMatrix::from_column_slice(4, 1, &[0.0, 1.0, 1.0, 1.0]);
    let instrument = DMatrix::from_column_slice(4, 1, &[0.0, 0.0, 1.0, 1.0]);
    let none = DMatrix::<f64>::zeros(4, 0);
    let error = instrumental_variable(
        IvInput {
            treatments: &treatment,
            instruments: &none,
            outcome: &outcome,
        },
        IvOptions::default(),
    )
    .unwrap_err();
    assert_eq!(error, IvError::NoInstruments);

    let two_treatments = DMatrix::from_column_slice(4, 2, &[0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0]);
    let error = instrumental_variable(
        IvInput {
            treatments: &two_treatments,
            instruments: &instrument,
            outcome: &outcome,
        },
        IvOptions::default(),
    )
    .unwrap_err();
    assert_eq!(
        error,
        IvError::FewerInstrumentsThanTreatments {
            instruments: 1,
            treatments: 2,
        }
    );

    let short = DMatrix::from_column_slice(3, 1, &[0.0, 1.0, 0.0]);
    let error = instrumental_variable(
        IvInput {
            treatments: &short,
            instruments: &instrument,
            outcome: &outcome,
        },
        IvOptions::default(),
    )
    .unwrap_err();
    assert_eq!(
        error,
        IvError::LengthMismatch {
            field: "treatments",
            expected: 4,
            actual: 3,
        }
    );

    // A binary instrument coded 1/2 leaves the reference with empty groups and NaN.
    let miscoded = DMatrix::from_column_slice(4, 1, &[1.0, 1.0, 2.0, 2.0]);
    let error = instrumental_variable(
        IvInput {
            treatments: &treatment,
            instruments: &miscoded,
            outcome: &outcome,
        },
        IvOptions::default(),
    )
    .unwrap_err();
    assert_eq!(error, IvError::NonFiniteEstimate(IvEstimator::WaldRatio));

    // Groups: z = 0 has outcome mean 1.5 and treatment mean 0.5; z = 1 has 3.5 and 1.0.
    let got = instrumental_variable(
        IvInput {
            treatments: &treatment,
            instruments: &instrument,
            outcome: &outcome,
        },
        IvOptions::default(),
    )
    .unwrap();
    assert_eq!(got.estimator, IvEstimator::WaldRatio);
    close("planted Wald ratio", got.estimate, 4.0, 1e-15);
    assert!(got.confidence_interval.is_none());
    assert!(got.bootstrap_estimates.is_empty());
}
