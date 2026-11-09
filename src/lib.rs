#![forbid(unsafe_code)]

use num_complex::Complex;

fn sasum(x: &[f32]) -> f32 {
    let mut s = 0.0f32;
    for xi in x {
        s += xi.abs();
    }
    s
}

fn dasum(x: &[f64]) -> f64 {
    let mut s = 0.0f64;
    for xi in x {
        s += xi.abs();
    }
    s
}

fn scasum(x: &[Complex<f32>]) -> f32 {
    let mut s = 0.0f32;
    for xi in x {
        s += xi.re.abs() + xi.im.abs();
    }
    s
}

fn dzasum(x: &[Complex<f64>]) -> f64 {
    let mut s = 0.0f64;
    for xi in x {
        s += xi.re.abs() + xi.im.abs();
    }
    s
}

fn saxpy(a: f32, x: &[f32], y: &mut [f32]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

fn daxpy(a: f64, x: &[f64], y: &mut [f64]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

fn caxpy(a: Complex<f32>, x: &[Complex<f32>], y: &mut [Complex<f32>]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

fn zaxpy(a: Complex<f64>, x: &[Complex<f64>], y: &mut [Complex<f64>]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

fn scopy(x: &[f32], y: &mut [f32]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

fn dcopy(x: &[f64], y: &mut [f64]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

fn ccopy(x: &[Complex<f32>], y: &mut [Complex<f32>]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

fn zcopy(x: &[Complex<f64>], y: &mut [Complex<f64>]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

fn sdot(x: &[f32], y: &[f32]) -> f32 {
    assert!(x.len() == y.len());
    let mut s = 0.0f32;
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

fn ddot(x: &[f64], y: &[f64]) -> f64 {
    assert!(x.len() == y.len());
    let mut s = 0.0f64;
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

fn cdotu(x: &[Complex<f32>], y: &[Complex<f32>]) -> Complex<f32> {
    assert!(x.len() == y.len());
    let mut s = Complex::default();
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

fn zdotu(x: &[Complex<f64>], y: &[Complex<f64>]) -> Complex<f64> {
    assert!(x.len() == y.len());
    let mut s = Complex::default();
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

fn cdotc(x: &[Complex<f32>], y: &[Complex<f32>]) -> Complex<f32> {
    assert!(x.len() == y.len());
    let mut s = Complex::default();
    for i in 0..x.len() {
        s += x[i].conj() * y[i];
    }
    s
}

fn zdotc(x: &[Complex<f64>], y: &[Complex<f64>]) -> Complex<f64> {
    assert!(x.len() == y.len());
    let mut s = Complex::default();
    for i in 0..x.len() {
        s += x[i].conj() * y[i];
    }
    s
}

fn nrm2() {
    todo!()
}

fn rot() {
    todo!()
}

fn rotg() {
    todo!()
}

fn rotm() {
    todo!()
}

fn rotmg() {
    todo!()
}

fn scal() {
    todo!()
}

fn swap() {
    todo!()
}

fn iamax() {
    todo!()
}

fn iamin() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sasum() {
        let x: Vec<f32> = vec![1.0, -2.0, 3.5, -4.5];
        assert_eq!(sasum(&x), 11.0);
    }

    #[test]
    fn test_dasum() {
        let x: Vec<f64> = vec![1.0, -2.0, 3.5, -4.5];
        assert_eq!(dasum(&x), 11.0);
    }

    // BLAS defines scasum/dzasum as sum(|Re(x_i)| + |Im(x_i)|), not the sum of moduli.
    #[test]
    fn test_scasum() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(3.0, 4.0),
            Complex::new(-1.0, -2.0),
            Complex::new(0.5, -0.5),
        ];
        assert_eq!(scasum(&x), 11.0);
    }

    #[test]
    fn test_scasum_empty() {
        let x: Vec<Complex<f32>> = vec![];
        assert_eq!(scasum(&x), 0.0);
    }

    #[test]
    fn test_dzasum() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(3.0, 4.0),
            Complex::new(-1.0, -2.0),
            Complex::new(0.5, -0.5),
        ];
        assert_eq!(dzasum(&x), 11.0);
    }

    #[test]
    fn test_dzasum_empty() {
        let x: Vec<Complex<f64>> = vec![];
        assert_eq!(dzasum(&x), 0.0);
    }

    #[test]
    fn test_saxpy() {
        let x: Vec<f32> = vec![1.0, -2.0, 3.5, 0.0];
        let mut y: Vec<f32> = vec![10.0, 20.0, -30.0, 5.0];
        saxpy(2.0, &x, &mut y);
        assert_eq!(y, vec![12.0, 16.0, -23.0, 5.0]);
        assert_eq!(x, vec![1.0, -2.0, 3.5, 0.0]);
    }

    #[test]
    fn test_daxpy() {
        let x: Vec<f64> = vec![1.0, -2.0, 3.5, 0.0];
        let mut y: Vec<f64> = vec![10.0, 20.0, -30.0, 5.0];
        daxpy(2.0, &x, &mut y);
        assert_eq!(y, vec![12.0, 16.0, -23.0, 5.0]);
        assert_eq!(x, vec![1.0, -2.0, 3.5, 0.0]);
    }

    #[test]
    fn test_caxpy() {
        let a = Complex::new(2.0f32, 1.0);
        let x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, 1.0),
        ];
        let mut y: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(2.0, -3.0),
            Complex::new(0.0, 0.0),
        ];
        caxpy(a, &x, &mut y);
        assert_eq!(
            y,
            vec![
                Complex::new(1.0, 6.0),
                Complex::new(0.0, -4.0),
                Complex::new(-1.0, 2.0),
            ]
        );
        assert_eq!(
            x,
            vec![
                Complex::new(1.0, 2.0),
                Complex::new(-1.0, 0.0),
                Complex::new(0.0, 1.0),
            ]
        );
    }

    #[test]
    fn test_caxpy_zero_alpha() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(-3.0, 4.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(5.0, -6.0), Complex::new(7.0, 8.0)];
        caxpy(Complex::new(0.0, 0.0), &x, &mut y);
        assert_eq!(y, vec![Complex::new(5.0, -6.0), Complex::new(7.0, 8.0)]);
    }

    #[test]
    #[should_panic]
    fn test_caxpy_length_mismatch() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); 2];
        caxpy(Complex::new(1.0, 0.0), &x, &mut y);
    }

    #[test]
    fn test_zaxpy() {
        let a = Complex::new(2.0f64, 1.0);
        let x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, 1.0),
        ];
        let mut y: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(2.0, -3.0),
            Complex::new(0.0, 0.0),
        ];
        zaxpy(a, &x, &mut y);
        assert_eq!(
            y,
            vec![
                Complex::new(1.0, 6.0),
                Complex::new(0.0, -4.0),
                Complex::new(-1.0, 2.0),
            ]
        );
        assert_eq!(
            x,
            vec![
                Complex::new(1.0, 2.0),
                Complex::new(-1.0, 0.0),
                Complex::new(0.0, 1.0),
            ]
        );
    }

    #[test]
    fn test_zaxpy_zero_alpha() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(-3.0, 4.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(5.0, -6.0), Complex::new(7.0, 8.0)];
        zaxpy(Complex::new(0.0, 0.0), &x, &mut y);
        assert_eq!(y, vec![Complex::new(5.0, -6.0), Complex::new(7.0, 8.0)]);
    }

    #[test]
    #[should_panic]
    fn test_zaxpy_length_mismatch() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); 2];
        zaxpy(Complex::new(1.0, 0.0), &x, &mut y);
    }

    #[test]
    fn test_scopy() {
        let x: Vec<f32> = vec![1.0, -2.0, 3.5, -4.5];
        let mut y: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0];
        scopy(&x, &mut y);
        assert_eq!(y, vec![1.0, -2.0, 3.5, -4.5]);
        assert_eq!(x, vec![1.0, -2.0, 3.5, -4.5]);
    }

    #[test]
    fn test_scopy_empty() {
        let x: Vec<f32> = vec![];
        let mut y: Vec<f32> = vec![];
        scopy(&x, &mut y);
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_scopy_length_mismatch() {
        let x: Vec<f32> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f32> = vec![0.0, 0.0];
        scopy(&x, &mut y);
    }

    #[test]
    fn test_dcopy() {
        let x: Vec<f64> = vec![1.0, -2.0, 3.5, -4.5];
        let mut y: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0];
        dcopy(&x, &mut y);
        assert_eq!(y, vec![1.0, -2.0, 3.5, -4.5]);
        assert_eq!(x, vec![1.0, -2.0, 3.5, -4.5]);
    }

    #[test]
    fn test_dcopy_empty() {
        let x: Vec<f64> = vec![];
        let mut y: Vec<f64> = vec![];
        dcopy(&x, &mut y);
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_dcopy_length_mismatch() {
        let x: Vec<f64> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f64> = vec![0.0, 0.0];
        dcopy(&x, &mut y);
    }

    #[test]
    fn test_ccopy() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, -2.0),
            Complex::new(-3.5, 0.0),
            Complex::new(0.0, 4.5),
        ];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(9.0, 9.0); 3];
        ccopy(&x, &mut y);
        assert_eq!(y, x);
        assert_eq!(
            x,
            vec![
                Complex::new(1.0, -2.0),
                Complex::new(-3.5, 0.0),
                Complex::new(0.0, 4.5),
            ]
        );
    }

    #[test]
    fn test_ccopy_empty() {
        let x: Vec<Complex<f32>> = vec![];
        let mut y: Vec<Complex<f32>> = vec![];
        ccopy(&x, &mut y);
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_ccopy_length_mismatch() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 1.0); 3];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); 2];
        ccopy(&x, &mut y);
    }

    #[test]
    fn test_zcopy() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, -2.0),
            Complex::new(-3.5, 0.0),
            Complex::new(0.0, 4.5),
        ];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(9.0, 9.0); 3];
        zcopy(&x, &mut y);
        assert_eq!(y, x);
        assert_eq!(
            x,
            vec![
                Complex::new(1.0, -2.0),
                Complex::new(-3.5, 0.0),
                Complex::new(0.0, 4.5),
            ]
        );
    }

    #[test]
    fn test_zcopy_empty() {
        let x: Vec<Complex<f64>> = vec![];
        let mut y: Vec<Complex<f64>> = vec![];
        zcopy(&x, &mut y);
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_zcopy_length_mismatch() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 1.0); 3];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); 2];
        zcopy(&x, &mut y);
    }

    #[test]
    fn test_sdot() {
        let x: Vec<f32> = vec![1.0, -2.0, 3.5, -4.5];
        let y: Vec<f32> = vec![2.0, 0.5, -2.0, 1.0];
        assert_eq!(sdot(&x, &y), -10.5);
        assert_eq!(x, vec![1.0, -2.0, 3.5, -4.5]);
        assert_eq!(y, vec![2.0, 0.5, -2.0, 1.0]);
    }

    #[test]
    fn test_sdot_orthogonal() {
        let x: Vec<f32> = vec![1.0, 0.0, 2.0];
        let y: Vec<f32> = vec![0.0, 5.0, 0.0];
        assert_eq!(sdot(&x, &y), 0.0);
    }

    #[test]
    fn test_sdot_empty() {
        let x: Vec<f32> = vec![];
        let y: Vec<f32> = vec![];
        assert_eq!(sdot(&x, &y), 0.0);
    }

    #[test]
    #[should_panic]
    fn test_sdot_length_mismatch() {
        let x: Vec<f32> = vec![1.0, 2.0, 3.0];
        let y: Vec<f32> = vec![1.0, 2.0];
        sdot(&x, &y);
    }

    #[test]
    fn test_ddot() {
        let x: Vec<f64> = vec![1.0, -2.0, 3.5, -4.5];
        let y: Vec<f64> = vec![2.0, 0.5, -2.0, 1.0];
        assert_eq!(ddot(&x, &y), -10.5);
        assert_eq!(x, vec![1.0, -2.0, 3.5, -4.5]);
        assert_eq!(y, vec![2.0, 0.5, -2.0, 1.0]);
    }

    #[test]
    fn test_ddot_orthogonal() {
        let x: Vec<f64> = vec![1.0, 0.0, 2.0];
        let y: Vec<f64> = vec![0.0, 5.0, 0.0];
        assert_eq!(ddot(&x, &y), 0.0);
    }

    #[test]
    fn test_ddot_empty() {
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        assert_eq!(ddot(&x, &y), 0.0);
    }

    #[test]
    #[should_panic]
    fn test_ddot_length_mismatch() {
        let x: Vec<f64> = vec![1.0, 2.0, 3.0];
        let y: Vec<f64> = vec![1.0, 2.0];
        ddot(&x, &y);
    }

    #[test]
    fn test_cdotu() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)];
        let y: Vec<Complex<f32>> = vec![Complex::new(2.0, -1.0), Complex::new(-1.0, 4.0)];
        assert_eq!(cdotu(&x, &y), Complex::new(5.0, 16.0));
    }

    #[test]
    fn test_cdotu_empty() {
        let x: Vec<Complex<f32>> = vec![];
        let y: Vec<Complex<f32>> = vec![];
        assert_eq!(cdotu(&x, &y), Complex::new(0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn test_cdotu_length_mismatch() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 3];
        let y: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 2];
        cdotu(&x, &y);
    }

    #[test]
    fn test_cdotc() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)];
        let y: Vec<Complex<f32>> = vec![Complex::new(2.0, -1.0), Complex::new(-1.0, 4.0)];
        assert_eq!(cdotc(&x, &y), Complex::new(-7.0, 6.0));
    }

    // conj(x) . x is the squared Euclidean norm: always real.
    #[test]
    fn test_cdotc_self_is_squared_norm() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)];
        assert_eq!(cdotc(&x, &x), Complex::new(15.0, 0.0));
    }

    #[test]
    fn test_cdotc_empty() {
        let x: Vec<Complex<f32>> = vec![];
        let y: Vec<Complex<f32>> = vec![];
        assert_eq!(cdotc(&x, &y), Complex::new(0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn test_cdotc_length_mismatch() {
        let x: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 3];
        let y: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 2];
        cdotc(&x, &y);
    }

    #[test]
    fn test_zdotu() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)];
        let y: Vec<Complex<f64>> = vec![Complex::new(2.0, -1.0), Complex::new(-1.0, 4.0)];
        assert_eq!(zdotu(&x, &y), Complex::new(5.0, 16.0));
    }

    #[test]
    fn test_zdotu_empty() {
        let x: Vec<Complex<f64>> = vec![];
        let y: Vec<Complex<f64>> = vec![];
        assert_eq!(zdotu(&x, &y), Complex::new(0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn test_zdotu_length_mismatch() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 3];
        let y: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 2];
        zdotu(&x, &y);
    }

    #[test]
    fn test_zdotc() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)];
        let y: Vec<Complex<f64>> = vec![Complex::new(2.0, -1.0), Complex::new(-1.0, 4.0)];
        assert_eq!(zdotc(&x, &y), Complex::new(-7.0, 6.0));
    }

    // conj(x) . x is the squared Euclidean norm: always real.
    #[test]
    fn test_zdotc_self_is_squared_norm() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(3.0, -1.0)];
        assert_eq!(zdotc(&x, &x), Complex::new(15.0, 0.0));
    }

    #[test]
    fn test_zdotc_empty() {
        let x: Vec<Complex<f64>> = vec![];
        let y: Vec<Complex<f64>> = vec![];
        assert_eq!(zdotc(&x, &y), Complex::new(0.0, 0.0));
    }

    #[test]
    #[should_panic]
    fn test_zdotc_length_mismatch() {
        let x: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 3];
        let y: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 2];
        zdotc(&x, &y);
    }
}
