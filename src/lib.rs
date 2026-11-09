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

fn scasum(x: &[Complex<f32>]) -> Complex<f32> {
    let mut s = Complex::default();
    for xi in x {
        s += xi.re.abs() + xi.im.abs();
    }
    s
}

fn dzasum(x: &[Complex<f64>]) -> Complex<f64> {
    let mut s = Complex::default();
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

fn scopy(x: &[f32], y: &mut [f32]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

fn dcopy(x: &[f64], y: &mut [f64]) {
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

fn dotc() {
    todo!()
}

fn dotu() {
    todo!()
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

    // BLAS defines ?casum as sum(|Re(x_i)| + |Im(x_i)|), not the sum of moduli.
    #[test]
    fn test_casum() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(3.0, 4.0),
            Complex::new(-1.0, -2.0),
            Complex::new(0.5, -0.5),
        ];
        assert_eq!(scasum(&x), Complex::new(11.0, 0.0));
    }

    #[test]
    fn test_casum_empty() {
        let x: Vec<Complex<f32>> = vec![];
        assert_eq!(scasum(&x), Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_zasum() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(3.0, 4.0),
            Complex::new(-1.0, -2.0),
            Complex::new(0.5, -0.5),
        ];
        assert_eq!(dzasum(&x), Complex::new(11.0, 0.0));
    }

    #[test]
    fn test_zasum_empty() {
        let x: Vec<Complex<f64>> = vec![];
        assert_eq!(dzasum(&x), Complex::new(0.0, 0.0));
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
}
