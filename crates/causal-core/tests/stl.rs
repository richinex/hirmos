// Parity for statsmodels' STL, the seasonal trend decomposition 805_dag runs at period 52.
use hirmos_causal_core::stl::*;
use serde_json::Value;

fn floats(v: &Value) -> Vec<f64> {
    serde_json::from_value(v.clone()).unwrap()
}

fn maxdev(got: &[f64], want: &[f64]) -> f64 {
    assert_eq!(got.len(), want.len(), "length mismatch");
    got.iter()
        .zip(want)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max)
}

#[test]
fn stl_matches() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/stl.json")).unwrap();
    for case in root["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let y = floats(&case["y"]);
        let mut cfg = StlConfig::new(
            case["period"].as_u64().unwrap() as usize,
            case["robust"].as_bool().unwrap(),
        );
        if let Some(j) = case.get("jumps") {
            cfg.seasonal_jump = j["seasonal"].as_u64().unwrap() as usize;
            cfg.trend_jump = j["trend"].as_u64().unwrap() as usize;
            cfg.low_pass_jump = j["low_pass"].as_u64().unwrap() as usize;
        }

        // The derived window lengths must agree before the numbers can.
        let want_cfg = &case["config"];
        assert_eq!(
            cfg.seasonal,
            want_cfg["seasonal"].as_u64().unwrap() as usize,
            "{name}: seasonal"
        );
        assert_eq!(
            cfg.trend,
            want_cfg["trend"].as_u64().unwrap() as usize,
            "{name}: trend"
        );
        assert_eq!(
            cfg.low_pass,
            want_cfg["low_pass"].as_u64().unwrap() as usize,
            "{name}: low_pass"
        );

        let fit = stl(&y, &cfg, None, None);
        let td = maxdev(&fit.trend, &floats(&case["trend"]));
        let sd = maxdev(&fit.seasonal, &floats(&case["seasonal"]));
        let rd = maxdev(&fit.resid, &floats(&case["resid"]));
        let wd = maxdev(&fit.weights, &floats(&case["weights"]));
        println!(
            "{name} (seasonal={} trend={} low_pass={})",
            cfg.seasonal, cfg.trend, cfg.low_pass
        );
        println!("  trend {td:.3e}, seasonal {sd:.3e}, resid {rd:.3e}, weights {wd:.3e}");

        let ts = strength(&fit.trend, &fit.resid);
        let ss = strength(&fit.seasonal, &fit.resid);
        println!(
            "  trend strength {ts:.6} against {:.6}, seasonal strength {ss:.6} against {:.6}",
            case["trend_strength"].as_f64().unwrap(),
            case["seasonal_strength"].as_f64().unwrap()
        );
        assert!(td <= 1e-10, "{name}: trend deviation {td}");
        assert!(sd <= 1e-10, "{name}: seasonal deviation {sd}");
        assert!(rd <= 1e-10, "{name}: residual deviation {rd}");
        assert!(wd <= 1e-10, "{name}: robustness weight deviation {wd}");
        assert!((ts - case["trend_strength"].as_f64().unwrap()).abs() <= 1e-12);
        assert!((ss - case["seasonal_strength"].as_f64().unwrap()).abs() <= 1e-12);

        // The decomposition is additive by construction.
        let recon = (0..y.len())
            .map(|i| (fit.trend[i] + fit.seasonal[i] + fit.resid[i] - y[i]).abs())
            .fold(0.0f64, f64::max);
        assert!(
            recon <= 1e-12,
            "{name}: components do not sum back to the series"
        );
    }
}
