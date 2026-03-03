pub use vecmath::traits::*;
pub use vecmath::*;
pub use float_ord::FloatOrd;
use std::ops::{Add, Mul, Sub};

pub trait Zero {
	fn zero() -> Self;
	fn is_zero(self) -> bool; 
}

impl Zero for i32 {
	fn zero() -> i32 {
		0
	}

	fn is_zero(self) -> bool {
		self == 0
	}
}

impl Zero for i64 {
	fn zero() -> i64 {
		0
	}

	fn is_zero(self) -> bool {
		self == 0
	}
}

impl Zero for i128 {
	fn zero() -> i128 {
		0
	}

	fn is_zero(self) -> bool {
		self == 0
	}
}


impl Zero for f64 {
	fn zero() -> f64 {
		0.0
	}

	fn is_zero(self) -> bool {
		self.abs() < 1e-10
	}
}


pub fn min<T: std::cmp::PartialOrd + Copy>(a : T, b : T) -> T {
	if a < b {
		a
	}
	else {
		b
	}
}


pub fn max<T: std::cmp::PartialOrd + Copy>(a : T, b : T) -> T {
	if a < b {
		b
	}
	else {
		a
	}
}



pub trait ArithmeticOps: 
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + std::cmp::PartialOrd + Zero + Copy
    where Self: std::marker::Sized {
}

impl<T> ArithmeticOps for T 
    where T: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + std::cmp::PartialOrd + Zero + Copy {
}