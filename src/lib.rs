#![forbid(unsafe_code)]

fn sasum(x: &Vec<f32>) -> f32 {
    let mut s = 0.0f32;
    for i in 0..x.len() {
        s += x[i].abs();
    }
    s
}

fn dasum(x: &Vec<f64>) -> f64 {
    let mut s = 0.0f64;
    for i in 0..x.len() {
        s += x[i].abs();
    }
    s
}

fn saxpy(a: f32, x: &Vec<f32>, y: &mut Vec<f32>) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

fn daxpy(a: f64, x: &Vec<f64>, y: &mut Vec<f64>) {
    assert!(x.len() == y.len());
    for i in 0..x.len() {
        y[i] += a * x[i];
    }
}

fn copy() {
    todo!()
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
}
