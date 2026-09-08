//! Matrix exponential in the scipy manner: Pade approximants of orders 3-13 chosen by 1-norm
//! bounds, with scaling and squaring for the rest.

use nalgebra::DMatrix;

fn one_norm(m: &DMatrix<f64>) -> f64 {
    let mut best = 0.0f64;
    for j in 0..m.ncols() {
        let s: f64 = m.column(j).iter().map(|v| v.abs()).sum();
        best = best.max(s);
    }
    best
}

fn pade(a: &DMatrix<f64>, b: &[f64]) -> (DMatrix<f64>, DMatrix<f64>) {
    let n = a.nrows();
    let ident = DMatrix::<f64>::identity(n, n);
    let a2 = a * a;
    match b.len() {
        4 => {
            let u = a * (&a2 * b[3] + &ident * b[1]);
            let v = &a2 * b[2] + &ident * b[0];
            (u, v)
        }
        6 => {
            let a4 = &a2 * &a2;
            let u = a * (&a4 * b[5] + &a2 * b[3] + &ident * b[1]);
            let v = &a4 * b[4] + &a2 * b[2] + &ident * b[0];
            (u, v)
        }
        8 => {
            let a4 = &a2 * &a2;
            let a6 = &a4 * &a2;
            let u = a * (&a6 * b[7] + &a4 * b[5] + &a2 * b[3] + &ident * b[1]);
            let v = &a6 * b[6] + &a4 * b[4] + &a2 * b[2] + &ident * b[0];
            (u, v)
        }
        10 => {
            let a4 = &a2 * &a2;
            let a6 = &a4 * &a2;
            let a8 = &a6 * &a2;
            let u = a * (&a8 * b[9] + &a6 * b[7] + &a4 * b[5] + &a2 * b[3] + &ident * b[1]);
            let v = &a8 * b[8] + &a6 * b[6] + &a4 * b[4] + &a2 * b[2] + &ident * b[0];
            (u, v)
        }
        _ => {
            let a4 = &a2 * &a2;
            let a6 = &a4 * &a2;
            let inner_u = &a6 * b[13] + &a4 * b[11] + &a2 * b[9];
            let u = a * (&a6 * &inner_u + &a6 * b[7] + &a4 * b[5] + &a2 * b[3] + &ident * b[1]);
            let inner_v = &a6 * b[12] + &a4 * b[10] + &a2 * b[8];
            let v = &a6 * &inner_v + &a6 * b[6] + &a4 * b[4] + &a2 * b[2] + &ident * b[0];
            (u, v)
        }
    }
}

/// scipy.linalg.expm for a real square matrix.
pub fn expm(a: &DMatrix<f64>) -> DMatrix<f64> {
    const B3: [f64; 4] = [120.0, 60.0, 12.0, 1.0];
    const B5: [f64; 6] = [30240.0, 15120.0, 3360.0, 420.0, 30.0, 1.0];
    const B7: [f64; 8] = [
        17297280.0, 8648640.0, 1995840.0, 277200.0, 25200.0, 1512.0, 56.0, 1.0,
    ];
    const B9: [f64; 10] = [
        17643225600.0,
        8821612800.0,
        2075673600.0,
        302702400.0,
        30270240.0,
        2162160.0,
        110880.0,
        3960.0,
        90.0,
        1.0,
    ];
    const B13: [f64; 14] = [
        64764752532480000.0,
        32382376266240000.0,
        7771770303897600.0,
        1187353796428800.0,
        129060195264000.0,
        10559470521600.0,
        670442572800.0,
        33522128640.0,
        1323241920.0,
        40840800.0,
        960960.0,
        16380.0,
        182.0,
        1.0,
    ];
    const THETA3: f64 = 1.495585217958292e-2;
    const THETA5: f64 = 2.539398330063230e-1;
    const THETA7: f64 = 9.504178996162932e-1;
    const THETA9: f64 = 2.097847961257068;
    const THETA13: f64 = 4.25;

    let norm = one_norm(a);
    let (u, v, squarings) = if norm <= THETA3 {
        let (u, v) = pade(a, &B3);
        (u, v, 0)
    } else if norm <= THETA5 {
        let (u, v) = pade(a, &B5);
        (u, v, 0)
    } else if norm <= THETA7 {
        let (u, v) = pade(a, &B7);
        (u, v, 0)
    } else if norm <= THETA9 {
        let (u, v) = pade(a, &B9);
        (u, v, 0)
    } else {
        let s = ((norm / THETA13).log2().ceil()).max(0.0) as u32;
        let scaled = a / 2f64.powi(s as i32);
        let (u, v) = pade(&scaled, &B13);
        (u, v, s)
    };

    let p = &v + &u;
    let q = &v - &u;
    let mut r = crate::linalg::solve(&q, &p).expect("expm solve failed");
    for _ in 0..squarings {
        r = &r * &r;
    }
    r
}
