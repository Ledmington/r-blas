#![forbid(unsafe_code)]

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

fn dot() {
    todo!()
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
}
