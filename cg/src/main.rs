//! Conjugate gradient on a large random SPD system with a known solution.
//!
//! Every vector operation of the solver (including the matrix-vector product,
//! done as one dot product per row) goes through the BLAS level 1 routines of
//! `blas1`.
//!
//! Run `cg --help` for the available options.

use clap::Parser;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use rblas1::{dasum, daxpy, dcopy, ddot, dnrm2, dscal};
use std::process::ExitCode;
use std::time::Instant;

#[derive(Parser)]
#[command(about = "Conjugate gradient on a random SPD system, using only BLAS level 1")]
struct Args {
    /// Size of the system (the matrix is N x N)
    #[arg(
        short = 'n',
        long = "size",
        value_name = "N",
        default_value_t = 2000,
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    n: u64,

    /// Seed of the random generator, for reproducible problems
    #[arg(short, long, default_value_t = 42)]
    seed: u64,

    /// Relative residual ||b - A x|| / ||b|| at which CG stops
    #[arg(short, long, default_value = "1e-12")]
    tolerance: f64,

    /// Maximum accepted relative error ||x - x_true|| / ||x_true||
    #[arg(short, long, default_value = "1e-9")]
    max_error: f64,
}

/// Dense n x n (row-major) symmetric matrix with random off-diagonal entries in
/// [-1, 1) and a diagonal larger than the sum of the absolute values of the rest
/// of its row. Strictly diagonally dominant with a positive diagonal => SPD.
///
/// The margin by which each diagonal entry exceeds its row sum grows
/// geometrically from 1 to 1e5 along the diagonal, spreading the spectrum so
/// that the system is not trivially well conditioned.
fn random_spd(n: usize, rng: &mut StdRng) -> Vec<f64> {
    let mut a = vec![0.0f64; n * n];
    for i in 0..n {
        for j in (i + 1)..n {
            let v = rng.random_range(-1.0f64..1.0f64);
            a[i * n + j] = v;
            a[j * n + i] = v;
        }
    }
    for i in 0..n {
        // The diagonal entry is still 0 here, so this is the off-diagonal sum.
        let off_diag = dasum(&a[i * n..(i + 1) * n]);
        let margin = 10.0f64.powf(5.0f64 * i as f64 / n as f64);
        a[i * n + i] = off_diag + margin;
    }
    a
}

/// y = A x, one level 1 dot product per row.
fn matvec(a: &[f64], x: &[f64], y: &mut [f64]) {
    let n = x.len();
    for (i, yi) in y.iter_mut().enumerate() {
        *yi = ddot(&a[i * n..(i + 1) * n], x);
    }
}

/// Solves A x = b starting from x = 0. Returns the number of iterations, or
/// None if it did not converge within `max_iter` iterations.
fn conjugate_gradient(
    a: &[f64],
    b: &[f64],
    x: &mut [f64],
    tolerance: f64,
    max_iter: usize,
) -> Option<usize> {
    let n = b.len();
    let mut r = vec![0.0f64; n];
    let mut p = vec![0.0f64; n];
    let mut ap = vec![0.0f64; n];

    // x = 0, so r = b - A x = b, and p = r.
    dscal(0.0f64, x);
    dcopy(b, &mut r);
    dcopy(&r, &mut p);

    let threshold = tolerance * dnrm2(b);
    let mut rs_old = ddot(&r, &r);

    for k in 0..max_iter {
        if rs_old.sqrt() <= threshold {
            return Some(k);
        }
        matvec(a, &p, &mut ap);
        let alpha = rs_old / ddot(&p, &ap);
        daxpy(alpha, &p, x); // x += alpha p
        daxpy(-alpha, &ap, &mut r); // r -= alpha A p
        let rs_new = ddot(&r, &r);
        dscal(rs_new / rs_old, &mut p); // p = r + beta p
        daxpy(1.0f64, &r, &mut p);
        rs_old = rs_new;
    }
    if rs_old.sqrt() <= threshold {
        Some(max_iter)
    } else {
        None
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let n = usize::try_from(args.n).expect("N does not fit in usize");

    println!("Conjugate gradient: N = {n}, seed = {}", args.seed);
    let mut rng = StdRng::seed_from_u64(args.seed);

    let start = Instant::now();
    let a = random_spd(n, &mut rng);
    let x_true: Vec<f64> = (0..n).map(|_| rng.random_range(-1.0f64..1.0f64)).collect();
    let mut b = vec![0.0f64; n];
    matvec(&a, &x_true, &mut b);
    println!("  setup:    {:.3?}", start.elapsed());

    let start = Instant::now();
    let mut x = vec![0.0f64; n];
    let iterations = conjugate_gradient(&a, &b, &mut x, args.tolerance, n);
    println!("  solve:    {:.3?}", start.elapsed());

    // Relative error against the known solution: ||x - x_true|| / ||x_true||.
    let mut err = vec![0.0f64; n];
    dcopy(&x, &mut err);
    daxpy(-1.0f64, &x_true, &mut err);
    let rel_error = dnrm2(&err) / dnrm2(&x_true);

    // True relative residual: ||A x - b|| / ||b||.
    let mut res = vec![0.0f64; n];
    matvec(&a, &x, &mut res);
    daxpy(-1.0f64, &b, &mut res);
    let rel_residual = dnrm2(&res) / dnrm2(&b);

    println!("  relative residual: {rel_residual:.3e}");
    println!("  relative error:    {rel_error:.3e}");

    match iterations {
        None => {
            eprintln!("FAILED: CG did not converge in {n} iterations");
            ExitCode::FAILURE
        }
        Some(_) if rel_error > args.max_error => {
            eprintln!(
                "FAILED: relative error {rel_error:.3e} > {:.0e}",
                args.max_error
            );
            ExitCode::FAILURE
        }
        Some(k) => {
            println!("OK: converged in {k} iterations");
            ExitCode::SUCCESS
        }
    }
}
