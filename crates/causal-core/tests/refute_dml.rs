// Parity for the causal worker's DML refuter block. The RNG probes are bitwise; fit-derived
// quantities carry the documented tied-split build tolerance of the forest port, which the
// z statistics of the light refits can amplify slightly, hence the looser p-value bound.
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use hirmos_causal_core::{
    placebo_refute, random_common_cause_refute, unobserved_refute, worker_fit, WorkerStudy,
};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64, tol: f64) {
    assert!(
        (got - want).abs() <= tol * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}"
    );
}

#[test]
fn generator_probes_match() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/refute_dml.json")).unwrap();
    let draws: Vec<f64> = serde_json::from_value(root["standard_normal_11"].clone()).unwrap();
    let mut rng = NpRng::seeded(11);
    for (i, &want) in draws.iter().enumerate() {
        let got = rng.standard_normal();
        assert!(
            got == want,
            "standard_normal draw {i}: got {got}, oracle {want}"
        );
    }
    let perm: Vec<f64> = serde_json::from_value(root["permutation_7"].clone()).unwrap();
    let vals: Vec<f64> = (0..12).map(|v| v as f64).collect();
    assert_eq!(
        NpRng::seeded(7).permutation_of(&vals),
        perm,
        "Generator permutation"
    );
    let rows: Vec<usize> = serde_json::from_value(root["sample_rows_7"].clone()).unwrap();
    let got: Vec<usize> = Mt19937::seeded(7)
        .permutation(1200)
        .into_iter()
        .take(20)
        .collect();
    assert_eq!(got, rows, "pandas sample row order");
}

#[test]
fn worker_refuters_match() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/refute_dml.json")).unwrap();
    let y: Vec<f64> = serde_json::from_value(root["data"]["y"].clone()).unwrap();
    let d: Vec<f64> = serde_json::from_value(root["data"]["d"].clone()).unwrap();
    let w1: Vec<f64> = serde_json::from_value(root["data"]["w1"].clone()).unwrap();
    let w2: Vec<f64> = serde_json::from_value(root["data"]["w2"].clone()).unwrap();
    let x: Vec<Vec<f64>> = w1.iter().zip(&w2).map(|(&a, &b)| vec![a, b]).collect();

    for kind in ["plr", "aipw"] {
        let case = &root[kind];
        let study = WorkerStudy {
            x: &x,
            y: &y,
            d: &d,
            aipw: kind == "aipw",
            att: false,
            treat_binary: true,
        };
        let mut fold_stream = Mt19937::seeded(7);
        let full = worker_fit(&study, &mut fold_stream);
        close(
            &format!("{kind} effect"),
            full.coef,
            case["effect"].as_f64().unwrap(),
            1e-3,
        );
        close(
            &format!("{kind} ci low"),
            full.ci_low,
            case["ciLow"].as_f64().unwrap(),
            1e-3,
        );
        close(
            &format!("{kind} ci high"),
            full.ci_high,
            case["ciHigh"].as_f64().unwrap(),
            1e-3,
        );

        let placebo = placebo_refute(&study, full.coef, &mut fold_stream);
        let want = &case["placebo"];
        close(
            &format!("{kind} placebo effect"),
            placebo.refuted_effect,
            want["refutedEffect"].as_f64().unwrap(),
            3e-3,
        );
        close(
            &format!("{kind} placebo p"),
            placebo.p_value,
            want["pValue"].as_f64().unwrap(),
            3e-2,
        );

        let rcc = random_common_cause_refute(&study, &mut fold_stream);
        let want = &case["randomCommonCause"];
        close(
            &format!("{kind} rcc baseline"),
            rcc.original_effect,
            want["baseline"].as_f64().unwrap(),
            3e-3,
        );
        close(
            &format!("{kind} rcc effect"),
            rcc.refuted_effect,
            want["refutedEffect"].as_f64().unwrap(),
            3e-3,
        );
        close(
            &format!("{kind} rcc p"),
            rcc.p_value,
            want["pValue"].as_f64().unwrap(),
            3e-2,
        );

        let sens = unobserved_refute(&full);
        let want = &case["sensitivity"];
        close(
            &format!("{kind} rv"),
            sens.robustness_value,
            want["robustnessValue"].as_f64().unwrap(),
            1e-3,
        );
        close(
            &format!("{kind} rva"),
            sens.robustness_value_ci,
            want["robustnessValueCi"].as_f64().unwrap(),
            1e-3,
        );
        for (got, want) in sens
            .scenarios
            .iter()
            .zip(want["scenarios"].as_array().unwrap())
        {
            let cf = want["confounding"].as_f64().unwrap();
            assert_eq!(got.confounding, cf);
            close(
                &format!("{kind} cf {cf} lower"),
                got.effect_lower,
                want["effectLower"].as_f64().unwrap(),
                1e-3,
            );
            close(
                &format!("{kind} cf {cf} upper"),
                got.effect_upper,
                want["effectUpper"].as_f64().unwrap(),
                1e-3,
            );
            close(
                &format!("{kind} cf {cf} ci lower"),
                got.ci_lower,
                want["ciLower"].as_f64().unwrap(),
                1e-3,
            );
            close(
                &format!("{kind} cf {cf} ci upper"),
                got.ci_upper,
                want["ciUpper"].as_f64().unwrap(),
                1e-3,
            );
        }
        println!("{kind}: full fit, placebo, random common cause and sensitivity match");
    }
}
