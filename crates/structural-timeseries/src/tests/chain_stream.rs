use super::*;

#[test]
fn application_chain_replays_boom_without_state_resets() {
    let f: serde_json::Value =
        serde_json::from_str(include_str!("../../fixtures/chain-stream.json")).unwrap();
    let y: Vec<f64> = serde_json::from_value(f["y"].clone()).unwrap();
    let observed: Vec<_> = y.iter().copied().map(Some).collect();
    let x = DMatrix::from_fn(y.len(), 2, |i, j| f["x"][i][j].as_f64().unwrap());
    let slab = crate::defaults::bsts_regression(&x, &observed).unwrap();
    let prior = Prior::new(
        Scale::new(0.01).unwrap(),
        0.01,
        crate::prior::Limit::At(Scale::new(1.).unwrap()),
    )
    .unwrap();
    let update = Update::Sample {
        prior,
        initial: Scale::new(0.01).unwrap(),
    };
    let variance = |v| Variance::new(v).unwrap();
    let mut maximum = 0_f64;
    for saved in f["chains"].as_array().unwrap() {
        let seed = saved["seed"].as_u64().unwrap();
        let terms = vec![
            Term::level(update, Normal::new(y[0], variance(1.)).unwrap()),
            Term::seasonal(
                Season::new(12, 1).unwrap(),
                update,
                Normal::new(0., variance(1.)).unwrap(),
            ),
        ];
        let mut chain = Chain::with_regression(
            terms,
            slab.clone(),
            x.clone(),
            observed.clone(),
            seed as u32,
        )
        .unwrap();
        // The probe supplies explicit child seeds. Set them once before sampling.
        chain.terms[0].1 = Random::new(seed + 1);
        chain.terms[1].1 = Random::new(seed + 2);
        chain.state_rng = Random::new(seed + 3);
        let system = System::new(chain.terms.iter().map(Term::component).collect()).unwrap();
        chain.states =
            gaussian::draw_states(&system, variance(1.), &observed, &mut chain.state_rng).unwrap();
        let Observation::Regression { model, .. } = &mut chain.observation_model else {
            unreachable!()
        };
        *model = crate::regression::Regression::new(
            x.clone(),
            DVector::from_vec(y.clone()),
            slab.clone(),
            seed as u32,
        )
        .unwrap();
        for (i, expected) in saved["draws"].as_array().unwrap().iter().enumerate() {
            let draw = chain.step().unwrap();
            let mut parameters = vec![
                draw.observation_variance,
                draw.component_variances[0][0],
                draw.component_variances[1][0],
            ];
            parameters.extend(&draw.regression.as_ref().unwrap().coefficients);
            for (j, actual) in parameters.iter().enumerate() {
                let reference = expected["parameters"][j].as_f64().unwrap();
                assert!(
                    (actual - reference).abs() < 1e-8 * reference.abs().max(1.),
                    "seed {seed}, sweep {i}, parameter {j}"
                );
            }
            for (t, state) in draw.states.iter().enumerate() {
                for (j, actual) in state.iter().enumerate() {
                    let reference = expected["states"][t][j].as_f64().unwrap();
                    maximum = maximum.max((actual - reference).abs());
                    assert!(
                        (actual - reference).abs() < 1e-8 * reference.abs().max(1.),
                        "seed {seed}, sweep {i}, state {t}/{j}"
                    );
                }
            }
        }
    }
    eprintln!("800 application sweeps: maximum state error {maximum:e}");
}
