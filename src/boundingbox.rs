use crate::math::max;
use crate::math::min;
use crate::math::ArithmeticOps;

pub type BoundingBox<T, const D : usize> = [[T;2];D];

pub trait NewBoundingBox<T : ArithmeticOps, const D : usize> {
	fn new(point : [T; D]) -> BoundingBox<T,D> {
		point.map(|x| [x, x])
	}
}

impl<T : ArithmeticOps, const D : usize> NewBoundingBox<T,D> for BoundingBox<T,D> {}



pub trait BBox<T> {
	fn union(&self, other : &Self) -> Self;
	fn intersection(&self, other : &Self) -> Self;
	fn is_empty(&self) -> bool;
}


impl<T : ArithmeticOps, const D : usize> BBox<T> for BoundingBox<T,D> {
	fn union(&self, other : &Self) -> Self {
		let mut ret = [[T::zero(), T::zero()];D];
		for i in 0..D {
			ret[i] = [min(self[i][0], other[i][0]), max(self[i][1], other[i][1])];
		}
		ret
	}
	fn intersection(&self, other: &Self) -> Self {
		let mut ret = [[T::zero(), T::zero()];D];
		for i in 0..D {
			ret[i] = [max(self[i][0], other[i][0]), min(self[i][1], other[i][1])];
		}
		ret
	}
	fn is_empty(&self) -> bool {
		for i in 0..D {
			if self[i][0] > self[i][1] {
				return true;
			}
		}
		false
	}
}