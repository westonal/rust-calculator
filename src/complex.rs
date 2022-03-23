use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;
use crate::FromStrValue;
use crate::math::{CommonMath, Math, Pow};

pub struct Complex<T> {
    real: T,
    imaginary: T,
}

trait Zero {
    fn zero() -> Self;
}

trait One {
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

impl CommonMath<Complex<f64>> for Complex<f64> {}

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

        println!("{} + {}i / {} + {}i\n\n", a, b, c, d);

        // (ac + bd)/(c^2 + d^2) + (bc-ad)/(c^2+d^2)i
        let c2d2 = c * c + d * d;

        Self {
            real: (a * c + b * d) / c2d2,
            imaginary: (b * c - a * d) / c2d2,
        }
    }
}

impl Pow for Complex<f64> {
    type Output = ();

    fn pow(self, rhs: Self) -> Self {
        todo!()
    }
}

impl<T: FromStr + Default> FromStr for Complex<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            real: s.parse()?,
            imaginary: Default::default(),
        })
    }
}

impl<T: FromStr + Default + One> FromStrValue for Complex<T> {
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
        let has_real = self.real != <T as Zero>::zero();
        let has_imaginary = self.imaginary != <T as Zero>::zero();
        if has_real {
            f.write_fmt(format_args!("{}", self.real))?;
        }
        if has_imaginary {
            if self.imaginary > <T as Zero>::zero() {
                if has_real {
                    f.write_char('+')?;
                }
            } else {
                f.write_char('-')?;
            }
            let mut img = self.imaginary;
            if img < <T as Zero>::zero() {
                img = -img;
            }
            if img != <T as One>::one() {
                f.write_fmt(format_args!("{}", self.imaginary))?
            }
            f.write_char('i')
        } else {
            f.write_str("")
        }
    }
}
