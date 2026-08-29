use hirmos_causal_core::nprandom::NpRng;
use hirmos_causal_core::nuts::{multinomial_nuts, NutsOptions};

#[test]
fn multinomial_nuts_adapts_anisotropic_gaussian() {
    let target = |position: &[f64]| {
        let delta_0 = position[0] - 1.0;
        let delta_1 = position[1] + 2.0;
        (
            -0.5 * (delta_0 * delta_0 / 0.25 + delta_1 * delta_1 / 4.0),
            vec![-delta_0 / 0.25, -delta_1 / 4.0],
        )
    };
    let mut rng = NpRng::seeded(73);
    let result = multinomial_nuts(
        &target,
        &[0.0, 0.0],
        NutsOptions {
            warmup: 400,
            samples: 1200,
            ..NutsOptions::default()
        },
        &mut rng,
    );
    let means: Vec<f64> = (0..2)
        .map(|column| {
            result
                .samples
                .iter()
                .map(|sample| sample[column])
                .sum::<f64>()
                / result.samples.len() as f64
        })
        .collect();
    let variances: Vec<f64> = (0..2)
        .map(|column| {
            result
                .samples
                .iter()
                .map(|sample| (sample[column] - means[column]).powi(2))
                .sum::<f64>()
                / (result.samples.len() - 1) as f64
        })
        .collect();
    println!(
        "Gaussian NUTS: mean {means:?}, variance {variances:?}, inverse mass {:?}, step {:.4}, accept {:.3}, divergences {}",
        result.inverse_mass_matrix,
        result.step_size,
        result.mean_accept_probability,
        result.divergences
    );
    assert!((means[0] - 1.0).abs() < 0.12);
    assert!((means[1] + 2.0).abs() < 0.24);
    assert!((variances[0] - 0.25).abs() < 0.08);
    assert!((variances[1] - 4.0).abs() < 0.7);
    assert!(result.inverse_mass_matrix[0] < 0.6);
    assert!(result.inverse_mass_matrix[1] > 1.5);
    assert!(result.mean_accept_probability > 0.65);
    assert_eq!(result.divergences, 0);
}
