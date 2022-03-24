use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use num::pow::Pow as NumPow;

use crate::FromStrValue;
use crate::math::{Inv, Percent, Pow};

#[cfg(test)]
mod complex_number_tests {
    use super::*;

    #[test]
    fn zero() {
        let complex = <Complex<f64> as Zero>::zero();
        assert_eq!((0f64, 0f64), (complex.real, complex.imaginary));
        assert!(!complex.has_real());
        assert!(!complex.has_imaginary());
    }

    #[test]
    fn one_real() {
        let complex = Complex::new(1f64, 0f64);
        assert_eq!((1f64, 0f64), (complex.real, complex.imaginary));
        assert!(complex.has_real());
        assert!(!complex.has_imaginary());
    }

    #[test]
    fn one_imaginary() {
        let complex = Complex::new(0f64, 1f64);
        assert_eq!((0f64, 1f64), (complex.real, complex.imaginary));
        assert!(!complex.has_real());
        assert!(complex.has_imaginary());
    }
}

pub struct Complex<T> {
    real: T,
    imaginary: T,
}

impl<T> Complex<T> {
    pub fn new(real: T, imaginary: T) -> Self {
        Self {
            real,
            imaginary,
        }
    }

    pub fn real(real: T) -> Self where T: Zero {
        Self {
            real,
            imaginary: T::zero(),
        }
    }
}

impl<T: Zero + PartialEq> Complex<T> {
    pub fn has_real(&self) -> bool {
        T::zero() != self.real
    }

    pub fn has_imaginary(&self) -> bool {
        T::zero() != self.imaginary
    }
}

impl<T: Zero> Zero for Complex<T> {
    fn zero() -> Self {
        Self {
            real: T::zero(),
            imaginary: T::zero(),
        }
    }
}

impl<T> Complex<T> {
    fn has_complex() {}
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

impl Zero for f64 {
    fn zero() -> Self {
        0f64
    }
}

impl One for f64 {
    fn one() -> Self {
        1f64
    }
}

impl Add for Complex<f64> {
    type Output = Complex<f64>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Sub for Complex<f64> {
    type Output = Complex<f64>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl Mul for Complex<f64> {
    type Output = Complex<f64>;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.real;
        let b = self.imaginary;
        let c = rhs.real;
        let d = rhs.imaginary;
        Self {
            // (a + bi) * (c + di)
            // ac - bd + (bc + ad)i
            real: a * c - b * d,
            imaginary: b * c + a * d,
        }
    }
}

impl Div for Complex<f64> {
    type Output = Complex<f64>;

    fn div(self, rhs: Self) -> Self::Output {
        let a = self.real;
        let b = self.imaginary;
        let c = rhs.real;
        let d = rhs.imaginary;
        // (ac + bd)/(c^2 + d^2) + (bc-ad)/(c^2+d^2)i
        let c2d2 = c * c + d * d;
        Self {
            real: (a * c + b * d) / c2d2,
            imaginary: (b * c - a * d) / c2d2,
        }
    }
}

impl Percent for Complex<f64> {
    fn percent(self) -> Self {
        self / Complex::real(100f64)
    }
}

impl Pow for Complex<f64> {
    fn pow(self, rhs: Self) -> Self {
        // TODO write into if we use this elsewhere
        let lhs = num::Complex { im: self.imaginary, re: self.real };
        let rhs = num::Complex { im: rhs.imaginary, re: rhs.real };
        let complex = lhs.pow(rhs);
        Complex::new(complex.re, complex.im)
    }
}

impl Inv for Complex<f64> {
    fn inv(self) -> Self {
        Complex::real(1f64).div(self)
    }
}

impl<T: FromStrValue + Default> FromStr for Complex<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            real: <T as FromStrValue>::from_str(s)?,
            imaginary: Default::default(),
        })
    }
}

impl<T: FromStrValue + Default + One> FromStrValue for Complex<T> {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "i" {
            Ok(Self {
                real: Default::default(),
                imaginary: <T as One>::one(),
            })
        } else {
            s.parse()
        }
    }
}

impl<T: Display + PartialEq + Zero + One + PartialOrd + Neg<Output=T> + Copy> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let has_real = self.has_real();
        let has_imaginary = self.has_imaginary();
        if has_real {
            f.write_fmt(format_args!("{}", self.real))?;
        }
        if has_imaginary {
            if self.imaginary > T::zero() {
                if has_real {
                    f.write_str(" + ")?;
                }
            } else {
                if has_real {
                    f.write_char(' ')?;
                }
                f.write_char('-')?;
            }
            let mut img = self.imaginary;
            if img < T::zero() {
                img = -img;
            }
            if img != <T as One>::one() {
                f.write_fmt(format_args!("{}", self.imaginary))?
            }
            f.write_char('i')
        } else {
            if !has_real {
                f.write_fmt(format_args!("{}", self.real))?;
            }
            f.write_str("")
        }
    }
}
