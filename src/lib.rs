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

fn axpy() {
    todo!()
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
}
