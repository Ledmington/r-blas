#![forbid(unsafe_code)]

use crate::complex::Complex;

mod complex;

pub fn sasum(x: &[f32]) -> f32 {
    let mut s = 0.0f32;
    for xi in x {
        s += xi.abs();
    }
    s
}

pub fn dasum(x: &[f64]) -> f64 {
    let mut s = 0.0f64;
    for xi in x {
        s += xi.abs();
    }
    s
}

pub fn scasum(x: &[Complex<f32>]) -> f32 {
    let mut s = 0.0f32;
    for xi in x {
        s += xi.re.abs() + xi.im.abs();
    }
    s
}

pub fn dzasum(x: &[Complex<f64>]) -> f64 {
    let mut s = 0.0f64;
    for xi in x {
        s += xi.re.abs() + xi.im.abs();
    }
    s
}

pub fn saxpy(a: f32, x: &[f32], y: &mut [f32]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

pub fn daxpy(a: f64, x: &[f64], y: &mut [f64]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

pub fn caxpy(a: Complex<f32>, x: &[Complex<f32>], y: &mut [Complex<f32>]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

pub fn zaxpy(a: Complex<f64>, x: &[Complex<f64>], y: &mut [Complex<f64>]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

pub fn scopy(x: &[f32], y: &mut [f32]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

pub fn dcopy(x: &[f64], y: &mut [f64]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

pub fn ccopy(x: &[Complex<f32>], y: &mut [Complex<f32>]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

pub fn zcopy(x: &[Complex<f64>], y: &mut [Complex<f64>]) {
    assert!(x.len() == y.len());
    y[..x.len()].copy_from_slice(x);
}

pub fn sdot(x: &[f32], y: &[f32]) -> f32 {
    assert!(x.len() == y.len());
    let mut s = 0.0f32;
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

pub fn ddot(x: &[f64], y: &[f64]) -> f64 {
    assert!(x.len() == y.len());
    let mut s = 0.0f64;
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

pub fn cdotu(x: &[Complex<f32>], y: &[Complex<f32>]) -> Complex<f32> {
    assert!(x.len() == y.len());
    let mut s: Complex<f32> = Complex::default();
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

pub fn zdotu(x: &[Complex<f64>], y: &[Complex<f64>]) -> Complex<f64> {
    assert!(x.len() == y.len());
    let mut s: Complex<f64> = Complex::default();
    for i in 0..x.len() {
        s += x[i] * y[i];
    }
    s
}

pub fn cdotc(x: &[Complex<f32>], y: &[Complex<f32>]) -> Complex<f32> {
    assert!(x.len() == y.len());
    let mut s: Complex<f32> = Complex::default();
    for i in 0..x.len() {
        s += x[i].conj() * y[i];
    }
    s
}

pub fn zdotc(x: &[Complex<f64>], y: &[Complex<f64>]) -> Complex<f64> {
    assert!(x.len() == y.len());
    let mut s: Complex<f64> = Complex::default();
    for i in 0..x.len() {
        s += x[i].conj() * y[i];
    }
    s
}

pub fn snrm2(x: &[f32]) -> f32 {
    let mut s = 0.0f32;
    for xi in x {
        s += xi * xi;
    }
    s.sqrt()
}

pub fn dnrm2(x: &[f64]) -> f64 {
    let mut s = 0.0f64;
    for xi in x {
        s += xi * xi;
    }
    s.sqrt()
}

pub fn scnrm2(x: &[Complex<f32>]) -> f32 {
    let mut s = 0.0f32;
    for xi in x {
        s += xi.norm_sqr();
    }
    s.sqrt()
}

pub fn dznrm2(x: &[Complex<f64>]) -> f64 {
    let mut s = 0.0f64;
    for xi in x {
        s += xi.norm_sqr();
    }
    s.sqrt()
}

pub fn srot(x: &mut [f32], y: &mut [f32], c: f32, s: f32) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = c * xi + s * yi;
        y[i] = c * yi - s * xi;
    }
}

pub fn drot(x: &mut [f64], y: &mut [f64], c: f64, s: f64) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = c * xi + s * yi;
        y[i] = c * yi - s * xi;
    }
}

pub fn csrot(x: &mut [Complex<f32>], y: &mut [Complex<f32>], c: f32, s: f32) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = c * xi + s * yi;
        y[i] = c * yi - s * xi;
    }
}

pub fn zdrot(x: &mut [Complex<f64>], y: &mut [Complex<f64>], c: f64, s: f64) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = c * xi + s * yi;
        y[i] = c * yi - s * xi;
    }
}

pub fn crot(x: &mut [Complex<f32>], y: &mut [Complex<f32>], c: f32, s: Complex<f32>) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = c * xi + s * yi;
        y[i] = c * yi - s.conj() * xi;
    }
}

pub fn zrot(x: &mut [Complex<f64>], y: &mut [Complex<f64>], c: f64, s: Complex<f64>) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = c * xi + s * yi;
        y[i] = c * yi - s.conj() * xi;
    }
}

pub fn srotg(a: &mut f32, b: &mut f32, c: &mut f32, s: &mut f32) {
    let roe = if a.abs() > b.abs() { *a } else { *b };
    let scale = a.abs() + b.abs();
    if scale == 0.0f32 {
        *c = 1.0f32;
        *s = 0.0f32;
        *a = 0.0f32;
        *b = 0.0f32;
        return;
    }

    let r = (scale * ((*a / scale).powi(2) + (*b / scale).powi(2)).sqrt()).copysign(roe);
    *c = *a / r;
    *s = *b / r;
    let z = if a.abs() > b.abs() {
        *s
    } else if *c != 0.0f32 {
        1.0f32 / *c
    } else {
        1.0f32
    };

    *a = r;
    *b = z;
}

pub fn drotg(a: &mut f64, b: &mut f64, c: &mut f64, s: &mut f64) {
    let roe = if a.abs() > b.abs() { *a } else { *b };
    let scale = a.abs() + b.abs();
    if scale == 0.0f64 {
        *c = 1.0f64;
        *s = 0.0f64;
        *a = 0.0f64;
        *b = 0.0f64;
        return;
    }

    let r = (scale * ((*a / scale).powi(2) + (*b / scale).powi(2)).sqrt()).copysign(roe);
    *c = *a / r;
    *s = *b / r;
    let z = if a.abs() > b.abs() {
        *s
    } else if *c != 0.0f64 {
        1.0f64 / *c
    } else {
        1.0f64
    };

    *a = r;
    *b = z;
}

pub fn crotg(a: &mut Complex<f32>, b: &Complex<f32>, c: &mut f32, s: &mut Complex<f32>) {
    let abs_a = a.norm();
    if abs_a == 0.0f32 {
        *c = 0.0f32;
        *s = Complex::new(1.0f32, 0.0f32);
        *a = *b;
        return;
    }

    let scale = abs_a + b.norm();
    let d = scale * ((*a / scale).norm_sqr() + (*b / scale).norm_sqr()).sqrt();
    let alpha = *a / abs_a;
    *c = abs_a / d;
    *s = alpha * b.conj() / d;
    *a = alpha * d;
}

pub fn zrotg(a: &mut Complex<f64>, b: &Complex<f64>, c: &mut f64, s: &mut Complex<f64>) {
    let abs_a = a.norm();
    if abs_a == 0.0f64 {
        *c = 0.0f64;
        *s = Complex::new(1.0f64, 0.0f64);
        *a = *b;
        return;
    }

    let scale = abs_a + b.norm();
    let d = scale * ((*a / scale).norm_sqr() + (*b / scale).norm_sqr()).sqrt();
    let alpha = *a / abs_a;
    *c = abs_a / d;
    *s = alpha * b.conj() / d;
    *a = alpha * d;
}

pub fn srotm(x: &mut [f32], y: &mut [f32], param: [f32; 5]) {
    // NOTE: param is in column-major order, so it's [flag, h11, h21, h12, h22]
    assert!(x.len() == y.len()); // FIXME: not correct but good enough for now
    // h is in row-major order, so: h11, h12, h21, h22
    let h: [f32; 4] = match param[0] {
        -2.0f32 => [1.0f32, 0.0f32, 0.0f32, 1.0f32],
        -1.0f32 => [param[1], param[3], param[2], param[4]],
        0.0f32 => [1.0f32, param[3], param[2], 1.0f32],
        1.0f32 => [param[1], 1.0f32, -1.0f32, param[4]],
        _ => panic!("Invalid value for flag: {}", param[0]),
    };
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = h[0] * xi + h[1] * yi;
        y[i] = h[2] * xi + h[3] * yi;
    }
}

pub fn drotm(x: &mut [f64], y: &mut [f64], param: [f64; 5]) {
    // NOTE: param is in column-major order, so it's [flag, h11, h21, h12, h22]
    assert!(x.len() == y.len()); // FIXME: not correct but good enough for now
    // h is in row-major order, so: h11, h12, h21, h22
    let h: [f64; 4] = match param[0] {
        -2.0f64 => [1.0f64, 0.0f64, 0.0f64, 1.0f64],
        -1.0f64 => [param[1], param[3], param[2], param[4]],
        0.0f64 => [1.0f64, param[3], param[2], 1.0f64],
        1.0f64 => [param[1], 1.0f64, -1.0f64, param[4]],
        _ => panic!("Invalid value for flag: {}", param[0]),
    };
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = h[0] * xi + h[1] * yi;
        y[i] = h[2] * xi + h[3] * yi;
    }
}

pub fn srotmg(d1: &mut f32, d2: &mut f32, x1: &mut f32, y1: &f32, param: &mut [f32; 5]) {
    // NOTE: param is in column-major order, so it's [flag, h11, h21, h12, h22]
    if *d1 >= 0.0f32 {
        let p2 = *d2 * *y1;
        if p2 == 0.0f32 {
            // H is the identity
            param[0] = -2.0f32;
            return;
        }
        let p1 = *d1 * *x1;
        let q1 = p1 * *x1;
        let q2 = p2 * *y1;

        if q1.abs() > q2.abs() {
            // H = [1, h12; h21, 1]
            let h21 = -*y1 / *x1;
            let h12 = p2 / p1;
            let u = 1.0f32 - h12 * h21;
            *d1 /= u;
            *d2 /= u;
            *x1 *= u;
            param[0] = 0.0f32;
            param[2] = h21;
            param[3] = h12;
            return;
        }
        if q2 >= 0.0f32 {
            // H = [h11, 1; -1, h22]
            let h11 = p1 / p2;
            let h22 = *x1 / *y1;
            let u = 1.0f32 + h11 * h22;
            let tmp = *d2 / u;
            *d2 = *d1 / u;
            *d1 = tmp;
            *x1 = *y1 * u;
            param[0] = 1.0f32;
            param[1] = h11;
            param[4] = h22;
            return;
        }
    }

    // d1 < 0 or q2 < 0: no valid rotation, H = 0
    *param = [-1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32];
    *d1 = 0.0f32;
    *d2 = 0.0f32;
    *x1 = 0.0f32;
}

pub fn drotmg(d1: &mut f64, d2: &mut f64, x1: &mut f64, y1: &f64, param: &mut [f64; 5]) {
    // NOTE: param is in column-major order, so it's [flag, h11, h21, h12, h22]
    if *d1 >= 0.0f64 {
        let p2 = *d2 * *y1;
        if p2 == 0.0f64 {
            // H is the identity
            param[0] = -2.0f64;
            return;
        }
        let p1 = *d1 * *x1;
        let q1 = p1 * *x1;
        let q2 = p2 * *y1;

        if q1.abs() > q2.abs() {
            // H = [1, h12; h21, 1]
            let h21 = -*y1 / *x1;
            let h12 = p2 / p1;
            let u = 1.0f64 - h12 * h21;
            *d1 /= u;
            *d2 /= u;
            *x1 *= u;
            param[0] = 0.0f64;
            param[2] = h21;
            param[3] = h12;
            return;
        }
        if q2 >= 0.0f64 {
            // H = [h11, 1; -1, h22]
            let h11 = p1 / p2;
            let h22 = *x1 / *y1;
            let u = 1.0f64 + h11 * h22;
            let tmp = *d2 / u;
            *d2 = *d1 / u;
            *d1 = tmp;
            *x1 = *y1 * u;
            param[0] = 1.0f64;
            param[1] = h11;
            param[4] = h22;
            return;
        }
    }

    // d1 < 0 or q2 < 0: no valid rotation, H = 0
    *param = [-1.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64];
    *d1 = 0.0f64;
    *d2 = 0.0f64;
    *x1 = 0.0f64;
}

pub fn sscal(a: f32, x: &mut [f32]) {
    for xi in x {
        *xi *= a;
    }
}

pub fn dscal(a: f64, x: &mut [f64]) {
    for xi in x {
        *xi *= a;
    }
}

pub fn cscal(a: Complex<f32>, x: &mut [Complex<f32>]) {
    for xi in x {
        *xi *= a;
    }
}

pub fn zscal(a: Complex<f64>, x: &mut [Complex<f64>]) {
    for xi in x {
        *xi *= a;
    }
}

pub fn csscal(a: f32, x: &mut [Complex<f32>]) {
    for xi in x {
        *xi *= a;
    }
}

pub fn zdscal(a: f64, x: &mut [Complex<f64>]) {
    for xi in x {
        *xi *= a;
    }
}

pub fn sswap(x: &mut [f32], y: &mut [f32]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = yi;
        y[i] = xi;
    }
}

pub fn dswap(x: &mut [f64], y: &mut [f64]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = yi;
        y[i] = xi;
    }
}

pub fn cswap(x: &mut [Complex<f32>], y: &mut [Complex<f32>]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = yi;
        y[i] = xi;
    }
}

pub fn zswap(x: &mut [Complex<f64>], y: &mut [Complex<f64>]) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        let xi = x[i];
        let yi = y[i];
        x[i] = yi;
        y[i] = xi;
    }
}

pub fn isamax(x: &[f32]) -> usize {
    let mut max_idx = 0;
    for i in 1..x.len() {
        if x[i].abs() > x[max_idx].abs() {
            max_idx = i;
        }
    }
    max_idx
}

pub fn idamax(x: &[f64]) -> usize {
    let mut max_idx = 0;
    for i in 1..x.len() {
        if x[i].abs() > x[max_idx].abs() {
            max_idx = i;
        }
    }
    max_idx
}

pub fn icamax(x: &[Complex<f32>]) -> usize {
    let mut max_idx = 0;
    for i in 1..x.len() {
        if (x[i].re.abs() + x[i].im.abs()) > (x[max_idx].re.abs() + x[max_idx].im.abs()) {
            max_idx = i;
        }
    }
    max_idx
}

pub fn izamax(x: &[Complex<f64>]) -> usize {
    let mut max_idx = 0;
    for i in 1..x.len() {
        if (x[i].re.abs() + x[i].im.abs()) > (x[max_idx].re.abs() + x[max_idx].im.abs()) {
            max_idx = i;
        }
    }
    max_idx
}

pub fn isamin(x: &[f32]) -> usize {
    let mut min_idx = 0;
    for i in 1..x.len() {
        if x[i].abs() < x[min_idx].abs() {
            min_idx = i;
        }
    }
    min_idx
}

pub fn idamin(x: &[f64]) -> usize {
    let mut min_idx = 0;
    for i in 1..x.len() {
        if x[i].abs() < x[min_idx].abs() {
            min_idx = i;
        }
    }
    min_idx
}

pub fn icamin(x: &[Complex<f32>]) -> usize {
    let mut min_idx = 0;
    for i in 1..x.len() {
        if (x[i].re.abs() + x[i].im.abs()) < (x[min_idx].re.abs() + x[min_idx].im.abs()) {
            min_idx = i;
        }
    }
    min_idx
}

pub fn izamin(x: &[Complex<f64>]) -> usize {
    let mut min_idx = 0;
    for i in 1..x.len() {
        if (x[i].re.abs() + x[i].im.abs()) < (x[min_idx].re.abs() + x[min_idx].im.abs()) {
            min_idx = i;
        }
    }
    min_idx
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

    #[test]
    fn test_snrm2() {
        let x: Vec<f32> = vec![3.0, -4.0, 0.0, 12.0];
        assert_eq!(snrm2(&x), 13.0);
    }

    #[test]
    fn test_snrm2_single_negative() {
        let x: Vec<f32> = vec![-7.0];
        assert_eq!(snrm2(&x), 7.0);
    }

    #[test]
    fn test_snrm2_empty() {
        let x: Vec<f32> = vec![];
        assert_eq!(snrm2(&x), 0.0);
    }

    #[test]
    fn test_dnrm2() {
        let x: Vec<f64> = vec![3.0, -4.0, 0.0, 12.0];
        assert_eq!(dnrm2(&x), 13.0);
    }

    #[test]
    fn test_dnrm2_single_negative() {
        let x: Vec<f64> = vec![-7.0];
        assert_eq!(dnrm2(&x), 7.0);
    }

    #[test]
    fn test_dnrm2_empty() {
        let x: Vec<f64> = vec![];
        assert_eq!(dnrm2(&x), 0.0);
    }

    // BLAS defines scnrm2/dznrm2 as sqrt(sum(|x_i|^2)), a real number.
    #[test]
    fn test_scnrm2() {
        let x: Vec<Complex<f32>> = vec![Complex::new(3.0, 4.0), Complex::new(0.0, -12.0)];
        assert_eq!(scnrm2(&x), 13.0);
    }

    #[test]
    fn test_scnrm2_empty() {
        let x: Vec<Complex<f32>> = vec![];
        assert_eq!(scnrm2(&x), 0.0);
    }

    #[test]
    fn test_dznrm2() {
        let x: Vec<Complex<f64>> = vec![Complex::new(3.0, 4.0), Complex::new(0.0, -12.0)];
        assert_eq!(dznrm2(&x), 13.0);
    }

    #[test]
    fn test_dznrm2_empty() {
        let x: Vec<Complex<f64>> = vec![];
        assert_eq!(dznrm2(&x), 0.0);
    }

    // Element-wise comparison with a tolerance, for results that are not exact in floating point.
    fn assert_close_f32(a: &[f32], b: &[f32]) {
        assert_eq!(a.len(), b.len());
        for (ai, bi) in a.iter().zip(b) {
            assert!((ai - bi).abs() < 1e-6, "{a:?} != {b:?}");
        }
    }

    fn assert_close_f64(a: &[f64], b: &[f64]) {
        assert_eq!(a.len(), b.len());
        for (ai, bi) in a.iter().zip(b) {
            assert!((ai - bi).abs() < 1e-12, "{a:?} != {b:?}");
        }
    }

    fn assert_close_c32(a: &[Complex<f32>], b: &[Complex<f32>]) {
        assert_eq!(a.len(), b.len());
        for (ai, bi) in a.iter().zip(b) {
            assert!((ai - bi).norm() < 1e-6, "{a:?} != {b:?}");
        }
    }

    fn assert_close_c64(a: &[Complex<f64>], b: &[Complex<f64>]) {
        assert_eq!(a.len(), b.len());
        for (ai, bi) in a.iter().zip(b) {
            assert!((ai - bi).norm() < 1e-12, "{a:?} != {b:?}");
        }
    }

    // BLAS defines rot as: x' = c*x + s*y, y' = c*y - s*x.
    #[test]
    fn test_srot() {
        let mut x: Vec<f32> = vec![1.0, 2.0];
        let mut y: Vec<f32> = vec![3.0, -1.0];
        srot(&mut x, &mut y, 0.6, 0.8);
        assert_close_f32(&x, &[3.0, 0.4]);
        assert_close_f32(&y, &[1.0, -2.2]);
    }

    #[test]
    fn test_srot_identity() {
        let mut x: Vec<f32> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f32> = vec![4.0, 5.0, -6.5];
        srot(&mut x, &mut y, 1.0, 0.0);
        assert_eq!(x, vec![1.0, -2.0, 3.5]);
        assert_eq!(y, vec![4.0, 5.0, -6.5]);
    }

    #[test]
    fn test_srot_quarter_turn() {
        let mut x: Vec<f32> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f32> = vec![4.0, 5.0, -6.5];
        srot(&mut x, &mut y, 0.0, 1.0);
        assert_eq!(x, vec![4.0, 5.0, -6.5]);
        assert_eq!(y, vec![-1.0, 2.0, -3.5]);
    }

    #[test]
    #[should_panic]
    fn test_srot_length_mismatch() {
        let mut x: Vec<f32> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f32> = vec![1.0, 2.0];
        srot(&mut x, &mut y, 1.0, 0.0);
    }

    #[test]
    fn test_drot() {
        let mut x: Vec<f64> = vec![1.0, 2.0];
        let mut y: Vec<f64> = vec![3.0, -1.0];
        drot(&mut x, &mut y, 0.6, 0.8);
        assert_close_f64(&x, &[3.0, 0.4]);
        assert_close_f64(&y, &[1.0, -2.2]);
    }

    #[test]
    fn test_drot_identity() {
        let mut x: Vec<f64> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f64> = vec![4.0, 5.0, -6.5];
        drot(&mut x, &mut y, 1.0, 0.0);
        assert_eq!(x, vec![1.0, -2.0, 3.5]);
        assert_eq!(y, vec![4.0, 5.0, -6.5]);
    }

    #[test]
    fn test_drot_quarter_turn() {
        let mut x: Vec<f64> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f64> = vec![4.0, 5.0, -6.5];
        drot(&mut x, &mut y, 0.0, 1.0);
        assert_eq!(x, vec![4.0, 5.0, -6.5]);
        assert_eq!(y, vec![-1.0, 2.0, -3.5]);
    }

    #[test]
    #[should_panic]
    fn test_drot_length_mismatch() {
        let mut x: Vec<f64> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f64> = vec![1.0, 2.0];
        drot(&mut x, &mut y, 1.0, 0.0);
    }

    #[test]
    fn test_csrot() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(3.0, -1.0)];
        csrot(&mut x, &mut y, 0.6, 0.8);
        assert_close_c32(&x, &[Complex::new(3.0, 0.4)]);
        assert_close_c32(&y, &[Complex::new(1.0, -2.2)]);
    }

    #[test]
    fn test_csrot_identity() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        csrot(&mut x, &mut y, 1.0, 0.0);
        assert_eq!(x, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
        assert_eq!(y, vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)]);
    }

    #[test]
    fn test_csrot_quarter_turn() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        csrot(&mut x, &mut y, 0.0, 1.0);
        assert_eq!(x, vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)]);
        assert_eq!(y, vec![Complex::new(-1.0, -2.0), Complex::new(0.0, 3.0)]);
    }

    #[test]
    #[should_panic]
    fn test_csrot_length_mismatch() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 2];
        csrot(&mut x, &mut y, 1.0, 0.0);
    }

    #[test]
    fn test_zdrot() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(3.0, -1.0)];
        zdrot(&mut x, &mut y, 0.6, 0.8);
        assert_close_c64(&x, &[Complex::new(3.0, 0.4)]);
        assert_close_c64(&y, &[Complex::new(1.0, -2.2)]);
    }

    #[test]
    fn test_zdrot_identity() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        zdrot(&mut x, &mut y, 1.0, 0.0);
        assert_eq!(x, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
        assert_eq!(y, vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)]);
    }

    #[test]
    fn test_zdrot_quarter_turn() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        zdrot(&mut x, &mut y, 0.0, 1.0);
        assert_eq!(x, vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)]);
        assert_eq!(y, vec![Complex::new(-1.0, -2.0), Complex::new(0.0, 3.0)]);
    }

    #[test]
    #[should_panic]
    fn test_zdrot_length_mismatch() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 2];
        zdrot(&mut x, &mut y, 1.0, 0.0);
    }

    // LAPACK defines crot/zrot (real c, complex s) as: x' = c*x + s*y, y' = c*y - conj(s)*x.
    #[test]
    fn test_crot() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(3.0, -1.0)];
        crot(&mut x, &mut y, 0.6, Complex::new(0.0, 0.8));
        assert_close_c32(&x, &[Complex::new(1.4, 3.6)]);
        assert_close_c32(&y, &[Complex::new(0.2, 0.2)]);
    }

    // With a purely imaginary s, conj(s) = -s: a missing conj() flips the sign of y.
    #[test]
    fn test_crot_imaginary_s() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(3.0, -1.0)];
        crot(&mut x, &mut y, 0.0, Complex::new(0.0, 1.0));
        assert_eq!(x, vec![Complex::new(1.0, 3.0)]);
        assert_eq!(y, vec![Complex::new(-2.0, 1.0)]);
    }

    #[test]
    fn test_crot_identity() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        crot(&mut x, &mut y, 1.0, Complex::new(0.0, 0.0));
        assert_eq!(x, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
        assert_eq!(y, vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)]);
    }

    #[test]
    fn test_crot_real_s_matches_csrot() {
        let x0: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let y0: Vec<Complex<f32>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        let (mut x1, mut y1) = (x0.clone(), y0.clone());
        let (mut x2, mut y2) = (x0.clone(), y0.clone());
        crot(&mut x1, &mut y1, 0.6, Complex::new(0.8, 0.0));
        csrot(&mut x2, &mut y2, 0.6, 0.8);
        assert_eq!(x1, x2);
        assert_eq!(y1, y2);
    }

    #[test]
    #[should_panic]
    fn test_crot_length_mismatch() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 2];
        crot(&mut x, &mut y, 1.0, Complex::new(0.0, 0.0));
    }

    #[test]
    fn test_zrot() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(3.0, -1.0)];
        zrot(&mut x, &mut y, 0.6, Complex::new(0.0, 0.8));
        assert_close_c64(&x, &[Complex::new(1.4, 3.6)]);
        assert_close_c64(&y, &[Complex::new(0.2, 0.2)]);
    }

    // With a purely imaginary s, conj(s) = -s: a missing conj() flips the sign of y.
    #[test]
    fn test_zrot_imaginary_s() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(3.0, -1.0)];
        zrot(&mut x, &mut y, 0.0, Complex::new(0.0, 1.0));
        assert_eq!(x, vec![Complex::new(1.0, 3.0)]);
        assert_eq!(y, vec![Complex::new(-2.0, 1.0)]);
    }

    #[test]
    fn test_zrot_identity() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        zrot(&mut x, &mut y, 1.0, Complex::new(0.0, 0.0));
        assert_eq!(x, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
        assert_eq!(y, vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)]);
    }

    #[test]
    fn test_zrot_real_s_matches_zdrot() {
        let x0: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let y0: Vec<Complex<f64>> = vec![Complex::new(4.0, -1.0), Complex::new(2.0, 2.0)];
        let (mut x1, mut y1) = (x0.clone(), y0.clone());
        let (mut x2, mut y2) = (x0.clone(), y0.clone());
        zrot(&mut x1, &mut y1, 0.6, Complex::new(0.8, 0.0));
        zdrot(&mut x2, &mut y2, 0.6, 0.8);
        assert_eq!(x1, x2);
        assert_eq!(y1, y2);
    }

    #[test]
    #[should_panic]
    fn test_zrot_length_mismatch() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 2];
        zrot(&mut x, &mut y, 1.0, Complex::new(0.0, 0.0));
    }

    // BLAS rotg: from (a, b) compute c, s, r with [c s; -s c] [a; b] = [r; 0].
    // r takes the sign of whichever of a, b has the larger magnitude.
    // On exit a holds r and b holds z (s if |a| > |b|, else 1/c, or 1 if c == 0).
    #[test]
    fn test_srotg_a_smaller() {
        let (mut a, mut b, mut c, mut s): (f32, f32, f32, f32) = (3.0, 4.0, 0.0, 0.0);
        srotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f32(&[a, b, c, s], &[5.0, 1.0 / 0.6, 0.6, 0.8]);
    }

    #[test]
    fn test_srotg_a_larger() {
        let (mut a, mut b, mut c, mut s): (f32, f32, f32, f32) = (4.0, 3.0, 0.0, 0.0);
        srotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f32(&[a, b, c, s], &[5.0, 0.6, 0.8, 0.6]);
    }

    #[test]
    fn test_srotg_negative_a_dominant() {
        let (mut a, mut b, mut c, mut s): (f32, f32, f32, f32) = (-4.0, 3.0, 0.0, 0.0);
        srotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f32(&[a, b, c, s], &[-5.0, -0.6, 0.8, -0.6]);
    }

    #[test]
    fn test_srotg_negative_b_dominant() {
        let (mut a, mut b, mut c, mut s): (f32, f32, f32, f32) = (3.0, -4.0, 0.0, 0.0);
        srotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f32(&[a, b, c, s], &[-5.0, -1.0 / 0.6, -0.6, 0.8]);
    }

    #[test]
    fn test_srotg_a_zero() {
        let (mut a, mut b, mut c, mut s): (f32, f32, f32, f32) = (0.0, 5.0, 0.0, 0.0);
        srotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f32(&[a, b, c, s], &[5.0, 1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_srotg_both_zero() {
        let (mut a, mut b, mut c, mut s): (f32, f32, f32, f32) = (0.0, 0.0, 0.0, 0.0);
        srotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f32(&[a, b, c, s], &[0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_srotg_then_srot_zeroes_b() {
        let (a0, b0): (f32, f32) = (1.5, -2.0);
        let (mut a, mut b, mut c, mut s) = (a0, b0, 0.0, 0.0);
        srotg(&mut a, &mut b, &mut c, &mut s);
        let mut x = vec![a0];
        let mut y = vec![b0];
        srot(&mut x, &mut y, c, s);
        assert_close_f32(&x, &[a]);
        assert_close_f32(&y, &[0.0]);
    }

    #[test]
    fn test_drotg_a_smaller() {
        let (mut a, mut b, mut c, mut s): (f64, f64, f64, f64) = (3.0, 4.0, 0.0, 0.0);
        drotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f64(&[a, b, c, s], &[5.0, 1.0 / 0.6, 0.6, 0.8]);
    }

    #[test]
    fn test_drotg_a_larger() {
        let (mut a, mut b, mut c, mut s): (f64, f64, f64, f64) = (4.0, 3.0, 0.0, 0.0);
        drotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f64(&[a, b, c, s], &[5.0, 0.6, 0.8, 0.6]);
    }

    #[test]
    fn test_drotg_negative_a_dominant() {
        let (mut a, mut b, mut c, mut s): (f64, f64, f64, f64) = (-4.0, 3.0, 0.0, 0.0);
        drotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f64(&[a, b, c, s], &[-5.0, -0.6, 0.8, -0.6]);
    }

    #[test]
    fn test_drotg_negative_b_dominant() {
        let (mut a, mut b, mut c, mut s): (f64, f64, f64, f64) = (3.0, -4.0, 0.0, 0.0);
        drotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f64(&[a, b, c, s], &[-5.0, -1.0 / 0.6, -0.6, 0.8]);
    }

    #[test]
    fn test_drotg_a_zero() {
        let (mut a, mut b, mut c, mut s): (f64, f64, f64, f64) = (0.0, 5.0, 0.0, 0.0);
        drotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f64(&[a, b, c, s], &[5.0, 1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_drotg_both_zero() {
        let (mut a, mut b, mut c, mut s): (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);
        drotg(&mut a, &mut b, &mut c, &mut s);
        // [r, z, c, s]
        assert_close_f64(&[a, b, c, s], &[0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_drotg_then_drot_zeroes_b() {
        let (a0, b0): (f64, f64) = (1.5, -2.0);
        let (mut a, mut b, mut c, mut s) = (a0, b0, 0.0, 0.0);
        drotg(&mut a, &mut b, &mut c, &mut s);
        let mut x = vec![a0];
        let mut y = vec![b0];
        drot(&mut x, &mut y, c, s);
        assert_close_f64(&x, &[a]);
        assert_close_f64(&y, &[0.0]);
    }

    // Complex rotg: c real, [c s; -conj(s) c] [a; b] = [r; 0], with
    // r = (a / |a|) * sqrt(|a|^2 + |b|^2). On exit a holds r, b is unchanged.
    // If a == 0: c = 0, s = 1, r = b.
    #[test]
    fn test_crotg_imaginary_a() {
        let mut a: Complex<f32> = Complex::new(0.0, 3.0);
        let b: Complex<f32> = Complex::new(4.0, 0.0);
        let mut c: f32 = 0.0;
        let mut s: Complex<f32> = Complex::new(0.0, 0.0);
        crotg(&mut a, &b, &mut c, &mut s);
        assert_close_c32(&[a], &[Complex::new(0.0, 5.0)]);
        assert_close_f32(&[c], &[0.6]);
        assert_close_c32(&[s], &[Complex::new(0.0, 0.8)]);
        assert_eq!(b, Complex::new(4.0, 0.0));
    }

    #[test]
    fn test_crotg_imaginary_b() {
        let mut a: Complex<f32> = Complex::new(3.0, 0.0);
        let b: Complex<f32> = Complex::new(0.0, 4.0);
        let mut c: f32 = 0.0;
        let mut s: Complex<f32> = Complex::new(0.0, 0.0);
        crotg(&mut a, &b, &mut c, &mut s);
        assert_close_c32(&[a], &[Complex::new(5.0, 0.0)]);
        assert_close_f32(&[c], &[0.6]);
        assert_close_c32(&[s], &[Complex::new(0.0, -0.8)]);
        assert_eq!(b, Complex::new(0.0, 4.0));
    }

    #[test]
    fn test_crotg_a_zero() {
        let mut a: Complex<f32> = Complex::new(0.0, 0.0);
        let b: Complex<f32> = Complex::new(2.0, 1.0);
        let mut c: f32 = 0.0;
        let mut s: Complex<f32> = Complex::new(0.0, 0.0);
        crotg(&mut a, &b, &mut c, &mut s);
        assert_close_c32(&[a], &[Complex::new(2.0, 1.0)]);
        assert_close_f32(&[c], &[0.0]);
        assert_close_c32(&[s], &[Complex::new(1.0, 0.0)]);
        assert_eq!(b, Complex::new(2.0, 1.0));
    }

    #[test]
    fn test_crotg_then_crot_zeroes_b() {
        let a0: Complex<f32> = Complex::new(1.0, 2.0);
        let b0: Complex<f32> = Complex::new(-2.0, 0.5);
        let mut a = a0;
        let mut c: f32 = 0.0;
        let mut s: Complex<f32> = Complex::new(0.0, 0.0);
        crotg(&mut a, &b0, &mut c, &mut s);
        let mut x = vec![a0];
        let mut y = vec![b0];
        crot(&mut x, &mut y, c, s);
        assert_close_c32(&x, &[a]);
        assert_close_c32(&y, &[Complex::new(0.0, 0.0)]);
    }

    #[test]
    fn test_zrotg_imaginary_a() {
        let mut a: Complex<f64> = Complex::new(0.0, 3.0);
        let b: Complex<f64> = Complex::new(4.0, 0.0);
        let mut c: f64 = 0.0;
        let mut s: Complex<f64> = Complex::new(0.0, 0.0);
        zrotg(&mut a, &b, &mut c, &mut s);
        assert_close_c64(&[a], &[Complex::new(0.0, 5.0)]);
        assert_close_f64(&[c], &[0.6]);
        assert_close_c64(&[s], &[Complex::new(0.0, 0.8)]);
        assert_eq!(b, Complex::new(4.0, 0.0));
    }

    #[test]
    fn test_zrotg_imaginary_b() {
        let mut a: Complex<f64> = Complex::new(3.0, 0.0);
        let b: Complex<f64> = Complex::new(0.0, 4.0);
        let mut c: f64 = 0.0;
        let mut s: Complex<f64> = Complex::new(0.0, 0.0);
        zrotg(&mut a, &b, &mut c, &mut s);
        assert_close_c64(&[a], &[Complex::new(5.0, 0.0)]);
        assert_close_f64(&[c], &[0.6]);
        assert_close_c64(&[s], &[Complex::new(0.0, -0.8)]);
        assert_eq!(b, Complex::new(0.0, 4.0));
    }

    #[test]
    fn test_zrotg_a_zero() {
        let mut a: Complex<f64> = Complex::new(0.0, 0.0);
        let b: Complex<f64> = Complex::new(2.0, 1.0);
        let mut c: f64 = 0.0;
        let mut s: Complex<f64> = Complex::new(0.0, 0.0);
        zrotg(&mut a, &b, &mut c, &mut s);
        assert_close_c64(&[a], &[Complex::new(2.0, 1.0)]);
        assert_close_f64(&[c], &[0.0]);
        assert_close_c64(&[s], &[Complex::new(1.0, 0.0)]);
        assert_eq!(b, Complex::new(2.0, 1.0));
    }

    #[test]
    fn test_zrotg_then_zrot_zeroes_b() {
        let a0: Complex<f64> = Complex::new(1.0, 2.0);
        let b0: Complex<f64> = Complex::new(-2.0, 0.5);
        let mut a = a0;
        let mut c: f64 = 0.0;
        let mut s: Complex<f64> = Complex::new(0.0, 0.0);
        zrotg(&mut a, &b0, &mut c, &mut s);
        let mut x = vec![a0];
        let mut y = vec![b0];
        zrot(&mut x, &mut y, c, s);
        assert_close_c64(&x, &[a]);
        assert_close_c64(&y, &[Complex::new(0.0, 0.0)]);
    }

    // BLAS rotm: param = [flag, h11, h21, h12, h22], applies
    //   x' = h11*x + h12*y,  y' = h21*x + h22*y
    // flag -1: full H; flag 0: h11 = h22 = 1; flag 1: h21 = -1, h12 = 1; flag -2: identity.
    // The 9.0 entries are not used for that flag, and must be ignored.
    #[test]
    fn test_srotm_full() {
        let mut x: Vec<f32> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f32> = vec![3.0, -1.0, 4.0];
        srotm(&mut x, &mut y, [-1.0, 2.0, 3.0, 0.5, -1.0]);
        assert_eq!(x, vec![3.5, 3.5, 0.0]);
        assert_eq!(y, vec![0.0, 7.0, -7.0]);
    }

    #[test]
    fn test_srotm_flag_zero() {
        let mut x: Vec<f32> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f32> = vec![3.0, -1.0, 4.0];
        srotm(&mut x, &mut y, [0.0, 9.0, 3.0, 0.5, 9.0]);
        assert_eq!(x, vec![2.5, 1.5, 1.0]);
        assert_eq!(y, vec![6.0, 5.0, 1.0]);
    }

    #[test]
    fn test_srotm_flag_one() {
        let mut x: Vec<f32> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f32> = vec![3.0, -1.0, 4.0];
        srotm(&mut x, &mut y, [1.0, 2.0, 9.0, 9.0, -1.0]);
        assert_eq!(x, vec![5.0, 3.0, 2.0]);
        assert_eq!(y, vec![-4.0, -1.0, -3.0]);
    }

    #[test]
    fn test_srotm_identity() {
        let mut x: Vec<f32> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f32> = vec![3.0, -1.0, 4.0];
        srotm(&mut x, &mut y, [-2.0, 9.0, 9.0, 9.0, 9.0]);
        assert_eq!(x, vec![1.0, 2.0, -1.0]);
        assert_eq!(y, vec![3.0, -1.0, 4.0]);
    }

    #[test]
    fn test_srotm_single_element() {
        let mut x: Vec<f32> = vec![1.0];
        let mut y: Vec<f32> = vec![3.0];
        srotm(&mut x, &mut y, [-1.0, 2.0, 3.0, 0.5, -1.0]);
        assert_eq!(x, vec![3.5]);
        assert_eq!(y, vec![0.0]);
    }

    #[test]
    #[should_panic]
    fn test_srotm_invalid_flag() {
        let mut x: Vec<f32> = vec![1.0];
        let mut y: Vec<f32> = vec![1.0];
        srotm(&mut x, &mut y, [0.5, 1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    #[should_panic]
    fn test_srotm_length_mismatch() {
        let mut x: Vec<f32> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f32> = vec![1.0, 2.0];
        srotm(&mut x, &mut y, [-2.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_drotm_full() {
        let mut x: Vec<f64> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f64> = vec![3.0, -1.0, 4.0];
        drotm(&mut x, &mut y, [-1.0, 2.0, 3.0, 0.5, -1.0]);
        assert_eq!(x, vec![3.5, 3.5, 0.0]);
        assert_eq!(y, vec![0.0, 7.0, -7.0]);
    }

    #[test]
    fn test_drotm_flag_zero() {
        let mut x: Vec<f64> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f64> = vec![3.0, -1.0, 4.0];
        drotm(&mut x, &mut y, [0.0, 9.0, 3.0, 0.5, 9.0]);
        assert_eq!(x, vec![2.5, 1.5, 1.0]);
        assert_eq!(y, vec![6.0, 5.0, 1.0]);
    }

    #[test]
    fn test_drotm_flag_one() {
        let mut x: Vec<f64> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f64> = vec![3.0, -1.0, 4.0];
        drotm(&mut x, &mut y, [1.0, 2.0, 9.0, 9.0, -1.0]);
        assert_eq!(x, vec![5.0, 3.0, 2.0]);
        assert_eq!(y, vec![-4.0, -1.0, -3.0]);
    }

    #[test]
    fn test_drotm_identity() {
        let mut x: Vec<f64> = vec![1.0, 2.0, -1.0];
        let mut y: Vec<f64> = vec![3.0, -1.0, 4.0];
        drotm(&mut x, &mut y, [-2.0, 9.0, 9.0, 9.0, 9.0]);
        assert_eq!(x, vec![1.0, 2.0, -1.0]);
        assert_eq!(y, vec![3.0, -1.0, 4.0]);
    }

    #[test]
    fn test_drotm_single_element() {
        let mut x: Vec<f64> = vec![1.0];
        let mut y: Vec<f64> = vec![3.0];
        drotm(&mut x, &mut y, [-1.0, 2.0, 3.0, 0.5, -1.0]);
        assert_eq!(x, vec![3.5]);
        assert_eq!(y, vec![0.0]);
    }

    #[test]
    #[should_panic]
    fn test_drotm_invalid_flag() {
        let mut x: Vec<f64> = vec![1.0];
        let mut y: Vec<f64> = vec![1.0];
        drotm(&mut x, &mut y, [0.5, 1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    #[should_panic]
    fn test_drotm_length_mismatch() {
        let mut x: Vec<f64> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f64> = vec![1.0, 2.0];
        drotm(&mut x, &mut y, [-2.0, 0.0, 0.0, 0.0, 0.0]);
    }

    // BLAS rotmg: builds H so that H * [x1; y1] = [x1'; 0], with the weighted norm
    // d1*x1^2 + d2*y1^2 preserved as d1'*x1'^2. Entries of param not used by the
    // returned flag are left untouched (the tests start them at 9.0 to check this).
    #[test]
    fn test_srotmg_flag_zero() {
        let (mut d1, mut d2, mut x1, y1): (f32, f32, f32, f32) = (1.0, 1.0, 2.0, 1.0);
        let mut param: [f32; 5] = [9.0; 5];
        srotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param[0], 0.0);
        assert_close_f32(&param[1..], &[9.0, -0.5, 0.5, 9.0]);
        assert_close_f32(&[d1, d2, x1], &[0.8, 0.8, 2.5]);
    }

    #[test]
    fn test_srotmg_flag_one() {
        let (mut d1, mut d2, mut x1, y1): (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 2.0);
        let mut param: [f32; 5] = [9.0; 5];
        srotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param[0], 1.0);
        assert_close_f32(&param[1..], &[0.5, 9.0, 9.0, 0.5]);
        assert_close_f32(&[d1, d2, x1], &[0.8, 0.8, 2.5]);
    }

    #[test]
    fn test_srotmg_y1_zero_is_identity() {
        let (mut d1, mut d2, mut x1, y1): (f32, f32, f32, f32) = (2.0, 3.0, 1.0, 0.0);
        let mut param: [f32; 5] = [9.0; 5];
        srotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param, [-2.0, 9.0, 9.0, 9.0, 9.0]);
        assert_eq!((d1, d2, x1), (2.0, 3.0, 1.0));
    }

    #[test]
    fn test_srotmg_negative_d1() {
        let (mut d1, mut d2, mut x1, y1): (f32, f32, f32, f32) = (-1.0, 2.0, 3.0, 4.0);
        let mut param: [f32; 5] = [9.0; 5];
        srotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param, [-1.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!((d1, d2, x1), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_srotmg_negative_q2() {
        // d2 < 0 and |d2*y1^2| >= |d1*x1^2|
        let (mut d1, mut d2, mut x1, y1): (f32, f32, f32, f32) = (1.0, -1.0, 1.0, 2.0);
        let mut param: [f32; 5] = [9.0; 5];
        srotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param, [-1.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!((d1, d2, x1), (0.0, 0.0, 0.0));
    }

    // Calls rotmg, then checks that rotm with the returned param zeroes y1,
    // turns x1 into the returned x1, and preserves the weighted norm.
    fn check_srotmg_with_srotm(d1: f32, d2: f32, x1: f32, y1: f32) -> f32 {
        let (mut d1n, mut d2n, mut x1n) = (d1, d2, x1);
        let mut param: [f32; 5] = [9.0; 5];
        srotmg(&mut d1n, &mut d2n, &mut x1n, &y1, &mut param);
        let mut x = vec![x1];
        let mut y = vec![y1];
        srotm(&mut x, &mut y, param);
        let tol: f32 = 1e-5;
        assert!(
            (x[0] - x1n).abs() <= tol * x1n.abs(),
            "x: {} != {}",
            x[0],
            x1n
        );
        assert!(y[0].abs() <= tol * x1n.abs(), "y not zeroed: {}", y[0]);
        let before = d1 * x1 * x1 + d2 * y1 * y1;
        let after = d1n * x1n * x1n;
        assert!(
            (before - after).abs() <= tol * before.abs(),
            "norm: {before} != {after}"
        );
        param[0]
    }

    #[test]
    fn test_srotmg_then_srotm() {
        assert_eq!(check_srotmg_with_srotm(1.0, 1.0, 2.0, 1.0), 0.0);
        assert_eq!(check_srotmg_with_srotm(1.0, 1.0, 1.0, 2.0), 1.0);
        assert_eq!(check_srotmg_with_srotm(2.0, 0.5, -3.0, 1.5), 0.0);
        assert_eq!(check_srotmg_with_srotm(0.5, 3.0, 1.0, -4.0), 1.0);
    }

    #[test]
    fn test_drotmg_flag_zero() {
        let (mut d1, mut d2, mut x1, y1): (f64, f64, f64, f64) = (1.0, 1.0, 2.0, 1.0);
        let mut param: [f64; 5] = [9.0; 5];
        drotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param[0], 0.0);
        assert_close_f64(&param[1..], &[9.0, -0.5, 0.5, 9.0]);
        assert_close_f64(&[d1, d2, x1], &[0.8, 0.8, 2.5]);
    }

    #[test]
    fn test_drotmg_flag_one() {
        let (mut d1, mut d2, mut x1, y1): (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 2.0);
        let mut param: [f64; 5] = [9.0; 5];
        drotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param[0], 1.0);
        assert_close_f64(&param[1..], &[0.5, 9.0, 9.0, 0.5]);
        assert_close_f64(&[d1, d2, x1], &[0.8, 0.8, 2.5]);
    }

    #[test]
    fn test_drotmg_y1_zero_is_identity() {
        let (mut d1, mut d2, mut x1, y1): (f64, f64, f64, f64) = (2.0, 3.0, 1.0, 0.0);
        let mut param: [f64; 5] = [9.0; 5];
        drotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param, [-2.0, 9.0, 9.0, 9.0, 9.0]);
        assert_eq!((d1, d2, x1), (2.0, 3.0, 1.0));
    }

    #[test]
    fn test_drotmg_negative_d1() {
        let (mut d1, mut d2, mut x1, y1): (f64, f64, f64, f64) = (-1.0, 2.0, 3.0, 4.0);
        let mut param: [f64; 5] = [9.0; 5];
        drotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param, [-1.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!((d1, d2, x1), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_drotmg_negative_q2() {
        // d2 < 0 and |d2*y1^2| >= |d1*x1^2|
        let (mut d1, mut d2, mut x1, y1): (f64, f64, f64, f64) = (1.0, -1.0, 1.0, 2.0);
        let mut param: [f64; 5] = [9.0; 5];
        drotmg(&mut d1, &mut d2, &mut x1, &y1, &mut param);
        assert_eq!(param, [-1.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!((d1, d2, x1), (0.0, 0.0, 0.0));
    }

    // Calls rotmg, then checks that rotm with the returned param zeroes y1,
    // turns x1 into the returned x1, and preserves the weighted norm.
    fn check_drotmg_with_drotm(d1: f64, d2: f64, x1: f64, y1: f64) -> f64 {
        let (mut d1n, mut d2n, mut x1n) = (d1, d2, x1);
        let mut param: [f64; 5] = [9.0; 5];
        drotmg(&mut d1n, &mut d2n, &mut x1n, &y1, &mut param);
        let mut x = vec![x1];
        let mut y = vec![y1];
        drotm(&mut x, &mut y, param);
        let tol: f64 = 1e-12;
        assert!(
            (x[0] - x1n).abs() <= tol * x1n.abs(),
            "x: {} != {}",
            x[0],
            x1n
        );
        assert!(y[0].abs() <= tol * x1n.abs(), "y not zeroed: {}", y[0]);
        let before = d1 * x1 * x1 + d2 * y1 * y1;
        let after = d1n * x1n * x1n;
        assert!(
            (before - after).abs() <= tol * before.abs(),
            "norm: {before} != {after}"
        );
        param[0]
    }

    #[test]
    fn test_drotmg_then_drotm() {
        assert_eq!(check_drotmg_with_drotm(1.0, 1.0, 2.0, 1.0), 0.0);
        assert_eq!(check_drotmg_with_drotm(1.0, 1.0, 1.0, 2.0), 1.0);
        assert_eq!(check_drotmg_with_drotm(2.0, 0.5, -3.0, 1.5), 0.0);
        assert_eq!(check_drotmg_with_drotm(0.5, 3.0, 1.0, -4.0), 1.0);
    }

    #[test]
    fn test_sscal() {
        let mut x: Vec<f32> = vec![1.0, -2.0, 3.5, 0.0];
        sscal(2.5, &mut x);
        assert_eq!(x, vec![2.5, -5.0, 8.75, 0.0]);
    }

    #[test]
    fn test_sscal_negative() {
        let mut x: Vec<f32> = vec![1.0, -2.0, 3.5];
        sscal(-1.0, &mut x);
        assert_eq!(x, vec![-1.0, 2.0, -3.5]);
    }

    #[test]
    fn test_sscal_zero() {
        let mut x: Vec<f32> = vec![1.0, -2.0, 3.5];
        sscal(0.0, &mut x);
        assert!(x.iter().all(|&xi| xi == 0.0));
    }

    #[test]
    fn test_sscal_empty() {
        let mut x: Vec<f32> = vec![];
        sscal(2.0, &mut x);
        assert!(x.is_empty());
    }

    #[test]
    fn test_dscal() {
        let mut x: Vec<f64> = vec![1.0, -2.0, 3.5, 0.0];
        dscal(2.5, &mut x);
        assert_eq!(x, vec![2.5, -5.0, 8.75, 0.0]);
    }

    #[test]
    fn test_dscal_negative() {
        let mut x: Vec<f64> = vec![1.0, -2.0, 3.5];
        dscal(-1.0, &mut x);
        assert_eq!(x, vec![-1.0, 2.0, -3.5]);
    }

    #[test]
    fn test_dscal_zero() {
        let mut x: Vec<f64> = vec![1.0, -2.0, 3.5];
        dscal(0.0, &mut x);
        assert!(x.iter().all(|&xi| xi == 0.0));
    }

    #[test]
    fn test_dscal_empty() {
        let mut x: Vec<f64> = vec![];
        dscal(2.0, &mut x);
        assert!(x.is_empty());
    }

    #[test]
    fn test_cscal() {
        let mut x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        cscal(Complex::new(2.0, -1.0), &mut x);
        assert_eq!(
            x,
            vec![
                Complex::new(4.0, 3.0),
                Complex::new(-3.0, -6.0),
                Complex::new(8.0, -4.0)
            ]
        );
    }

    // Multiplying by i rotates each element by 90 degrees: (re, im) -> (-im, re).
    #[test]
    fn test_cscal_by_i() {
        let mut x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        cscal(Complex::new(0.0, 1.0), &mut x);
        assert_eq!(
            x,
            vec![
                Complex::new(-2.0, 1.0),
                Complex::new(3.0, 0.0),
                Complex::new(0.0, 4.0)
            ]
        );
    }

    #[test]
    fn test_cscal_zero() {
        let mut x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        cscal(Complex::new(0.0, 0.0), &mut x);
        assert!(x.iter().all(|xi| xi.re == 0.0 && xi.im == 0.0));
    }

    #[test]
    fn test_cscal_empty() {
        let mut x: Vec<Complex<f32>> = vec![];
        cscal(Complex::new(2.0, 1.0), &mut x);
        assert!(x.is_empty());
    }

    #[test]
    fn test_zscal() {
        let mut x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        zscal(Complex::new(2.0, -1.0), &mut x);
        assert_eq!(
            x,
            vec![
                Complex::new(4.0, 3.0),
                Complex::new(-3.0, -6.0),
                Complex::new(8.0, -4.0)
            ]
        );
    }

    // Multiplying by i rotates each element by 90 degrees: (re, im) -> (-im, re).
    #[test]
    fn test_zscal_by_i() {
        let mut x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        zscal(Complex::new(0.0, 1.0), &mut x);
        assert_eq!(
            x,
            vec![
                Complex::new(-2.0, 1.0),
                Complex::new(3.0, 0.0),
                Complex::new(0.0, 4.0)
            ]
        );
    }

    #[test]
    fn test_zscal_zero() {
        let mut x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        zscal(Complex::new(0.0, 0.0), &mut x);
        assert!(x.iter().all(|xi| xi.re == 0.0 && xi.im == 0.0));
    }

    #[test]
    fn test_zscal_empty() {
        let mut x: Vec<Complex<f64>> = vec![];
        zscal(Complex::new(2.0, 1.0), &mut x);
        assert!(x.is_empty());
    }

    #[test]
    fn test_csscal() {
        let mut x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        csscal(2.5, &mut x);
        assert_eq!(
            x,
            vec![
                Complex::new(2.5, 5.0),
                Complex::new(0.0, -7.5),
                Complex::new(10.0, 0.0)
            ]
        );
    }

    #[test]
    fn test_csscal_matches_cscal_with_real_a() {
        let mut x1: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        let mut x2 = x1.clone();
        csscal(-1.5, &mut x1);
        cscal(Complex::new(-1.5, 0.0), &mut x2);
        assert_eq!(x1, x2);
    }

    #[test]
    fn test_csscal_empty() {
        let mut x: Vec<Complex<f32>> = vec![];
        csscal(2.0, &mut x);
        assert!(x.is_empty());
    }

    #[test]
    fn test_zdscal() {
        let mut x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        zdscal(2.5, &mut x);
        assert_eq!(
            x,
            vec![
                Complex::new(2.5, 5.0),
                Complex::new(0.0, -7.5),
                Complex::new(10.0, 0.0)
            ]
        );
    }

    #[test]
    fn test_zdscal_matches_zscal_with_real_a() {
        let mut x1: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(0.0, -3.0),
            Complex::new(4.0, 0.0),
        ];
        let mut x2 = x1.clone();
        zdscal(-1.5, &mut x1);
        zscal(Complex::new(-1.5, 0.0), &mut x2);
        assert_eq!(x1, x2);
    }

    #[test]
    fn test_zdscal_empty() {
        let mut x: Vec<Complex<f64>> = vec![];
        zdscal(2.0, &mut x);
        assert!(x.is_empty());
    }

    #[test]
    fn test_sswap() {
        let mut x: Vec<f32> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f32> = vec![4.0, 5.0, -6.5];
        sswap(&mut x, &mut y);
        assert_eq!(x, vec![4.0, 5.0, -6.5]);
        assert_eq!(y, vec![1.0, -2.0, 3.5]);
    }

    #[test]
    fn test_sswap_twice_is_identity() {
        let mut x: Vec<f32> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f32> = vec![4.0, 5.0, -6.5];
        sswap(&mut x, &mut y);
        sswap(&mut x, &mut y);
        assert_eq!(x, vec![1.0, -2.0, 3.5]);
        assert_eq!(y, vec![4.0, 5.0, -6.5]);
    }

    #[test]
    fn test_sswap_empty() {
        let mut x: Vec<f32> = vec![];
        let mut y: Vec<f32> = vec![];
        sswap(&mut x, &mut y);
        assert!(x.is_empty());
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_sswap_length_mismatch() {
        let mut x: Vec<f32> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f32> = vec![1.0, 2.0];
        sswap(&mut x, &mut y);
    }

    #[test]
    fn test_dswap() {
        let mut x: Vec<f64> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f64> = vec![4.0, 5.0, -6.5];
        dswap(&mut x, &mut y);
        assert_eq!(x, vec![4.0, 5.0, -6.5]);
        assert_eq!(y, vec![1.0, -2.0, 3.5]);
    }

    #[test]
    fn test_dswap_twice_is_identity() {
        let mut x: Vec<f64> = vec![1.0, -2.0, 3.5];
        let mut y: Vec<f64> = vec![4.0, 5.0, -6.5];
        dswap(&mut x, &mut y);
        dswap(&mut x, &mut y);
        assert_eq!(x, vec![1.0, -2.0, 3.5]);
        assert_eq!(y, vec![4.0, 5.0, -6.5]);
    }

    #[test]
    fn test_dswap_empty() {
        let mut x: Vec<f64> = vec![];
        let mut y: Vec<f64> = vec![];
        dswap(&mut x, &mut y);
        assert!(x.is_empty());
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_dswap_length_mismatch() {
        let mut x: Vec<f64> = vec![1.0, 2.0, 3.0];
        let mut y: Vec<f64> = vec![1.0, 2.0];
        dswap(&mut x, &mut y);
    }

    #[test]
    fn test_cswap() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)];
        cswap(&mut x, &mut y);
        assert_eq!(x, vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)]);
        assert_eq!(y, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
    }

    #[test]
    fn test_cswap_twice_is_identity() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)];
        cswap(&mut x, &mut y);
        cswap(&mut x, &mut y);
        assert_eq!(x, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
        assert_eq!(y, vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)]);
    }

    #[test]
    fn test_cswap_empty() {
        let mut x: Vec<Complex<f32>> = vec![];
        let mut y: Vec<Complex<f32>> = vec![];
        cswap(&mut x, &mut y);
        assert!(x.is_empty());
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_cswap_length_mismatch() {
        let mut x: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f32>> = vec![Complex::new(1.0, 0.0); 2];
        cswap(&mut x, &mut y);
    }

    #[test]
    fn test_zswap() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)];
        zswap(&mut x, &mut y);
        assert_eq!(x, vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)]);
        assert_eq!(y, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
    }

    #[test]
    fn test_zswap_twice_is_identity() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)];
        zswap(&mut x, &mut y);
        zswap(&mut x, &mut y);
        assert_eq!(x, vec![Complex::new(1.0, 2.0), Complex::new(0.0, -3.0)]);
        assert_eq!(y, vec![Complex::new(4.0, -1.0), Complex::new(-2.5, 0.0)]);
    }

    #[test]
    fn test_zswap_empty() {
        let mut x: Vec<Complex<f64>> = vec![];
        let mut y: Vec<Complex<f64>> = vec![];
        zswap(&mut x, &mut y);
        assert!(x.is_empty());
        assert!(y.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_zswap_length_mismatch() {
        let mut x: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 3];
        let mut y: Vec<Complex<f64>> = vec![Complex::new(1.0, 0.0); 2];
        zswap(&mut x, &mut y);
    }

    // i?amax / i?amin return the 0-based index of the first element with the
    // largest / smallest |x_i|. For complex vectors BLAS uses |Re(x_i)| + |Im(x_i)|.
    #[test]
    fn test_isamax() {
        let x: Vec<f32> = vec![1.0, -2.0, 5.0, -3.0];
        assert_eq!(isamax(&x), 2);
    }

    #[test]
    fn test_isamax_negative() {
        let x: Vec<f32> = vec![1.0, -9.0, 5.0];
        assert_eq!(isamax(&x), 1);
    }

    #[test]
    fn test_isamax_tie_returns_first() {
        let x: Vec<f32> = vec![3.0, -7.0, 7.0, 2.0];
        assert_eq!(isamax(&x), 1);
    }

    #[test]
    fn test_isamax_single() {
        let x: Vec<f32> = vec![42.0];
        assert_eq!(isamax(&x), 0);
    }

    #[test]
    fn test_idamax() {
        let x: Vec<f64> = vec![1.0, -2.0, 5.0, -3.0];
        assert_eq!(idamax(&x), 2);
    }

    #[test]
    fn test_idamax_negative() {
        let x: Vec<f64> = vec![1.0, -9.0, 5.0];
        assert_eq!(idamax(&x), 1);
    }

    #[test]
    fn test_idamax_tie_returns_first() {
        let x: Vec<f64> = vec![3.0, -7.0, 7.0, 2.0];
        assert_eq!(idamax(&x), 1);
    }

    #[test]
    fn test_idamax_single() {
        let x: Vec<f64> = vec![42.0];
        assert_eq!(idamax(&x), 0);
    }

    #[test]
    fn test_isamin() {
        let x: Vec<f32> = vec![4.0, -2.0, 0.5, -3.0];
        assert_eq!(isamin(&x), 2);
    }

    #[test]
    fn test_isamin_negative() {
        let x: Vec<f32> = vec![4.0, -0.25, 1.0];
        assert_eq!(isamin(&x), 1);
    }

    #[test]
    fn test_isamin_tie_returns_first() {
        let x: Vec<f32> = vec![3.0, -1.0, 1.0, 2.0];
        assert_eq!(isamin(&x), 1);
    }

    #[test]
    fn test_isamin_single() {
        let x: Vec<f32> = vec![42.0];
        assert_eq!(isamin(&x), 0);
    }

    #[test]
    fn test_idamin() {
        let x: Vec<f64> = vec![4.0, -2.0, 0.5, -3.0];
        assert_eq!(idamin(&x), 2);
    }

    #[test]
    fn test_idamin_negative() {
        let x: Vec<f64> = vec![4.0, -0.25, 1.0];
        assert_eq!(idamin(&x), 1);
    }

    #[test]
    fn test_idamin_tie_returns_first() {
        let x: Vec<f64> = vec![3.0, -1.0, 1.0, 2.0];
        assert_eq!(idamin(&x), 1);
    }

    #[test]
    fn test_idamin_single() {
        let x: Vec<f64> = vec![42.0];
        assert_eq!(idamin(&x), 0);
    }

    #[test]
    fn test_icamax() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(-2.0, 3.0),
            Complex::new(0.0, -4.0),
            Complex::new(1.0, 0.5),
        ];
        assert_eq!(icamax(&x), 1);
    }

    #[test]
    fn test_icamax_uses_abs_sum_not_modulus() {
        // |re| + |im|: 6 vs 5, modulus: 4.24 vs 5
        let x: Vec<Complex<f32>> = vec![Complex::new(3.0, 3.0), Complex::new(0.0, 5.0)];
        assert_eq!(icamax(&x), 0);
    }

    #[test]
    fn test_icamax_tie_returns_first() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(-3.0, 0.0),
            Complex::new(0.0, -3.0),
        ];
        assert_eq!(icamax(&x), 0);
    }

    #[test]
    fn test_icamax_single() {
        let x: Vec<Complex<f32>> = vec![Complex::new(-1.0, 2.0)];
        assert_eq!(icamax(&x), 0);
    }

    #[test]
    fn test_izamax() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(-2.0, 3.0),
            Complex::new(0.0, -4.0),
            Complex::new(1.0, 0.5),
        ];
        assert_eq!(izamax(&x), 1);
    }

    #[test]
    fn test_izamax_uses_abs_sum_not_modulus() {
        // |re| + |im|: 6 vs 5, modulus: 4.24 vs 5
        let x: Vec<Complex<f64>> = vec![Complex::new(3.0, 3.0), Complex::new(0.0, 5.0)];
        assert_eq!(izamax(&x), 0);
    }

    #[test]
    fn test_izamax_tie_returns_first() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 2.0),
            Complex::new(-3.0, 0.0),
            Complex::new(0.0, -3.0),
        ];
        assert_eq!(izamax(&x), 0);
    }

    #[test]
    fn test_izamax_single() {
        let x: Vec<Complex<f64>> = vec![Complex::new(-1.0, 2.0)];
        assert_eq!(izamax(&x), 0);
    }

    #[test]
    fn test_icamin() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(-2.0, 3.0),
            Complex::new(0.0, -0.5),
            Complex::new(1.0, 0.5),
        ];
        assert_eq!(icamin(&x), 2);
    }

    #[test]
    fn test_icamin_uses_abs_sum_not_modulus() {
        // |re| + |im|: 6 vs 5, modulus: 4.24 vs 5
        let x: Vec<Complex<f32>> = vec![Complex::new(3.0, 3.0), Complex::new(0.0, 5.0)];
        assert_eq!(icamin(&x), 1);
    }

    #[test]
    fn test_icamin_tie_returns_first() {
        let x: Vec<Complex<f32>> = vec![
            Complex::new(4.0, 0.0),
            Complex::new(1.0, -2.0),
            Complex::new(0.0, 3.0),
        ];
        assert_eq!(icamin(&x), 1);
    }

    #[test]
    fn test_icamin_single() {
        let x: Vec<Complex<f32>> = vec![Complex::new(-1.0, 2.0)];
        assert_eq!(icamin(&x), 0);
    }

    #[test]
    fn test_izamin() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(1.0, 1.0),
            Complex::new(-2.0, 3.0),
            Complex::new(0.0, -0.5),
            Complex::new(1.0, 0.5),
        ];
        assert_eq!(izamin(&x), 2);
    }

    #[test]
    fn test_izamin_uses_abs_sum_not_modulus() {
        // |re| + |im|: 6 vs 5, modulus: 4.24 vs 5
        let x: Vec<Complex<f64>> = vec![Complex::new(3.0, 3.0), Complex::new(0.0, 5.0)];
        assert_eq!(izamin(&x), 1);
    }

    #[test]
    fn test_izamin_tie_returns_first() {
        let x: Vec<Complex<f64>> = vec![
            Complex::new(4.0, 0.0),
            Complex::new(1.0, -2.0),
            Complex::new(0.0, 3.0),
        ];
        assert_eq!(izamin(&x), 1);
    }

    #[test]
    fn test_izamin_single() {
        let x: Vec<Complex<f64>> = vec![Complex::new(-1.0, 2.0)];
        assert_eq!(izamin(&x), 0);
    }
}
