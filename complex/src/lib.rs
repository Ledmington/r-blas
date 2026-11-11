#![forbid(unsafe_code)]

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

pub trait Float {}

impl Float for f32 {}

impl Float for f64 {}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Complex<T: Float> {
    pub re: T,
    pub im: T,
}

impl Default for Complex<f32> {
    fn default() -> Self {
        Self {
            re: 0.0f32,
            im: 0.0f32,
        }
    }
}

impl Default for Complex<f64> {
    fn default() -> Self {
        Self {
            re: 0.0f64,
            im: 0.0f64,
        }
    }
}

impl Add<Complex<f32>> for Complex<f32> {
    type Output = Complex<f32>;

    fn add(self, rhs: Complex<f32>) -> Self::Output {
        Self::Output {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Add<Complex<f64>> for Complex<f64> {
    type Output = Complex<f64>;

    fn add(self, rhs: Complex<f64>) -> Self::Output {
        Self::Output {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub<Complex<f32>> for Complex<f32> {
    type Output = Complex<f32>;

    fn sub(self, rhs: Complex<f32>) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<Complex<f64>> for Complex<f64> {
    type Output = Complex<f64>;

    fn sub(self, rhs: Complex<f64>) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<&Complex<f32>> for &Complex<f32> {
    type Output = Complex<f32>;

    fn sub(self, rhs: &Complex<f32>) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Sub<&Complex<f64>> for &Complex<f64> {
    type Output = Complex<f64>;

    fn sub(self, rhs: &Complex<f64>) -> Self::Output {
        Self::Output {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl AddAssign<Complex<f32>> for Complex<f32> {
    fn add_assign(&mut self, rhs: Complex<f32>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl AddAssign<Complex<f64>> for Complex<f64> {
    fn add_assign(&mut self, rhs: Complex<f64>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Mul<Complex<f32>> for Complex<f32> {
    type Output = Complex<f32>;

    fn mul(self, rhs: Complex<f32>) -> Self::Output {
        Self::Output {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<Complex<f64>> for Complex<f64> {
    type Output = Complex<f64>;

    fn mul(self, rhs: Complex<f64>) -> Self::Output {
        Self::Output {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<f32> for Complex<f32> {
    type Output = Complex<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Mul<f64> for Complex<f64> {
    type Output = Complex<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Mul<Complex<f32>> for f32 {
    type Output = Complex<f32>;

    fn mul(self, rhs: Complex<f32>) -> Self::Output {
        Self::Output {
            re: self * rhs.re,
            im: self * rhs.im,
        }
    }
}

impl Mul<Complex<f64>> for f64 {
    type Output = Complex<f64>;

    fn mul(self, rhs: Complex<f64>) -> Self::Output {
        Self::Output {
            re: self * rhs.re,
            im: self * rhs.im,
        }
    }
}

impl MulAssign<f32> for Complex<f32> {
    fn mul_assign(&mut self, rhs: f32) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl MulAssign<f64> for Complex<f64> {
    fn mul_assign(&mut self, rhs: f64) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl MulAssign<Complex<f32>> for Complex<f32> {
    fn mul_assign(&mut self, rhs: Complex<f32>) {
        *self = *self * rhs;
    }
}

impl MulAssign<Complex<f64>> for Complex<f64> {
    fn mul_assign(&mut self, rhs: Complex<f64>) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Complex<f32> {
    type Output = Complex<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Div<f64> for Complex<f64> {
    type Output = Complex<f64>;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl<T: Float> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

impl Complex<f32> {
    pub fn conj(&self) -> Complex<f32> {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn norm_sqr(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(&self) -> f32 {
        self.re.hypot(self.im)
    }
}

impl Complex<f64> {
    pub fn conj(&self) -> Complex<f64> {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn norm(&self) -> f64 {
        self.re.hypot(self.im)
    }
}
