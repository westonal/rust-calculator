use std::ops::{Add, Div, Mul, Sub};

pub trait CommonMath<T>: Sized + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> {}

pub trait Pow {
    type Output;
    fn pow(self, rhs: Self) -> Self;
}

impl Pow for f64 {
    type Output = f64;

    fn pow(self, rhs: Self) -> Self {
        self.powf(rhs)
    }
}

impl Pow for i32 {
    type Output = i32;

    fn pow(self, rhs: Self) -> Self {
        self.pow(rhs.try_into::<>().expect(""))
    }
}

pub trait Math<T>: CommonMath<T> + Pow {}

impl<T: Sized + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Pow<Output=T>> CommonMath<T> for T {}

impl<T: CommonMath<T> + Pow> Math<T> for T {}