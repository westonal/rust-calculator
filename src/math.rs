use std::ops::{Add, Div, Mul, Sub};

pub trait CommonMath<T>: Sized + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> {}

pub trait Inv {
    fn inv(self) -> Self;
}

pub trait Pow {
    fn pow(self, rhs: Self) -> Self;
}

pub trait Percent {
    fn percent(self) -> Self;
}

pub trait Root {
    /// Find the [self] root of [rhs]
    fn root(self, rhs: Self) -> Self;
}

impl Pow for f64 {
    fn pow(self, rhs: Self) -> Self {
        self.powf(rhs)
    }
}

impl Pow for i32 {
    fn pow(self, rhs: Self) -> Self {
        self.pow(rhs.try_into::<>().expect(""))
    }
}

impl Inv for i32 {
    fn inv(self) -> Self {
        panic!()
    }
}

impl Inv for f64 {
    fn inv(self) -> Self {
        1f64 / self
    }
}

impl Percent for i32 {
    fn percent(self) -> Self {
        self / 100i32
    }
}

impl Percent for f64 {
    fn percent(self) -> Self {
        self / 100f64
    }
}

impl<T: Pow + Inv> Root for T {
    fn root(self, rhs: Self) -> Self {
        rhs.pow(self.inv())
    }
}

pub trait Math<T>: CommonMath<T> + Pow + Root + Percent {}

impl<T: Sized + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Pow + Root> CommonMath<T> for T {}

impl<T: CommonMath<T> + Pow + Root + Percent> Math<T> for T {}