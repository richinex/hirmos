use hirmos_causal_core::grf::rank::{self, Target};
use serde_json::Value;
fn num(v: &Value) -> f64 {
    v.as_str()
        .map(|s| s.parse().unwrap())
        .or_else(|| v.as_f64())
        .unwrap()
}
fn vals(v: &Value) -> Vec<f64> {
    if let Some(v) = v.as_array() {
        v.iter().map(num).collect()
    } else {
        vec![num(v)]
    }
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-9 * b.abs().max(1.0), "{a} != {b}");
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn rank_and_paired_bootstrap_match_r() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/rank.json")).unwrap();
    for c in fixture["cases"].as_array().unwrap() {
        let s = vals(&c["scores"]);
        let p: Vec<_> = c["priorities"]
            .as_array()
            .unwrap()
            .iter()
            .map(vals)
            .collect();
        let w = if c["weights"].is_null() {
            None
        } else {
            Some(vals(&c["weights"]))
        };
        let labels: Vec<_> = c["labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as usize)
            .collect();
        let q = vals(&c["q"]);
        let target = if c["target"] == "AUTOC" {
            Target::Autoc
        } else {
            Target::Qini
        };
        let a = rank::fit(
            &s,
            &p,
            w.as_deref(),
            &labels,
            &q,
            target,
            50,
            c["seed"].as_u64().unwrap() as u32,
        )
        .unwrap();
        let mut rules = a.rules;
        if let Some(d) = a.difference {
            rules.push(d);
        }
        let expected = vals(&c["estimate"]);
        let se = vals(&c["se"]);
        for (i, r) in rules.iter().enumerate() {
            close(r.estimate, expected[i]);
            close(r.standard_error, se[i]);
            for j in 0..q.len() {
                let row = &c["toc"][i * q.len() + j];
                close(r.toc[j], num(&row["estimate"]));
                close(r.toc_standard_error[j], num(&row["std.err"]));
            }
        }
    }
}
