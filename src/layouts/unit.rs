use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Sizing units.
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub enum SizeUnit {
    Pixel(f64),
}

impl SizeUnit {
    pub fn abs(self) -> Self {
        let a: f64 = self.into();
        Self::Pixel(a.abs())
    }
}

impl Into<f64> for SizeUnit {
    fn into(self) -> f64 {
        match self {
            Self::Pixel(f) => f,
        }
    }
}

impl From<f64> for SizeUnit {
    fn from(value: f64) -> Self {
        Self::Pixel(value)
    }
}

impl Add for SizeUnit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let a: f64 = self.into();
        let b: f64 = rhs.into();
        Self::Pixel(a + b)
    }
}

impl AddAssign for SizeUnit {
    fn add_assign(&mut self, rhs: Self) {
        let a: f64 = self.clone().into();
        let b: f64 = rhs.into();
        *self = Self::Pixel(a + b)
    }
}

impl Div for SizeUnit {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let a: f64 = self.into();
        let b: f64 = rhs.into();
        Self::Pixel(a / b)
    }
}

impl DivAssign for SizeUnit {
    fn div_assign(&mut self, rhs: Self) {
        let a: f64 = self.clone().into();
        let b: f64 = rhs.into();
        *self = Self::Pixel(a / b)
    }
}

impl Sub for SizeUnit {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let a: f64 = self.into();
        let b: f64 = rhs.into();
        Self::Pixel(a - b)
    }
}

impl SubAssign for SizeUnit {
    fn sub_assign(&mut self, rhs: Self) {
        let a: f64 = self.clone().into();
        let b: f64 = rhs.into();
        *self = Self::Pixel(a - b)
    }
}

impl Mul for SizeUnit {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let a: f64 = self.into();
        let b: f64 = rhs.into();
        Self::Pixel(a * b)
    }
}

impl MulAssign for SizeUnit {
    fn mul_assign(&mut self, rhs: Self) {
        let a: f64 = self.clone().into();
        let b: f64 = rhs.into();
        *self = Self::Pixel(a * b)
    }
}

impl Add<f64> for SizeUnit {
    type Output = f64;
    fn add(self, rhs: f64) -> Self::Output {
        let a: f64 = self.into();
        a + rhs
    }
}

impl AddAssign<f64> for SizeUnit {
    fn add_assign(&mut self, rhs: f64) {
        let a: f64 = self.clone().into();
        *self = Self::Pixel(a + rhs)
    }
}

impl Div<f64> for SizeUnit {
    type Output = f64;
    fn div(self, rhs: f64) -> Self::Output {
        let a: f64 = self.into();
        a / rhs
    }
}

impl DivAssign<f64> for SizeUnit {
    fn div_assign(&mut self, rhs: f64) {
        let a: f64 = self.clone().into();
        *self = Self::Pixel(a / rhs)
    }
}

impl Sub<f64> for SizeUnit {
    type Output = f64;
    fn sub(self, rhs: f64) -> Self::Output {
        let a: f64 = self.into();
        a - rhs
    }
}

impl SubAssign<f64> for SizeUnit {
    fn sub_assign(&mut self, rhs: f64) {
        let a: f64 = self.clone().into();
        *self = Self::Pixel(a - rhs)
    }
}

impl Mul<f64> for SizeUnit {
    type Output = f64;
    fn mul(self, rhs: f64) -> Self::Output {
        let a: f64 = self.into();
        a * rhs
    }
}

impl MulAssign<f64> for SizeUnit {
    fn mul_assign(&mut self, rhs: f64) {
        let a: f64 = self.clone().into();
        *self = Self::Pixel(a * rhs)
    }
}

impl Display for SizeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a: f64 = self.clone().into();
        f.write_str(&a.to_string())
    }
}

impl PartialEq<f64> for SizeUnit {
    fn eq(&self, other: &f64) -> bool {
        let a: f64 = self.clone().into();
        &a == other
    }
}

#[macro_export]
macro_rules! unitf {
    ($x:expr) => {
        <SizeUnit as Into<f64>>::into($x)
    };
}

#[macro_export]
macro_rules! unit {
    ($x:expr) => {
        <SizeUnit as From<f64>>::from($x)
    };
}
