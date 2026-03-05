use crate::algebra::dot;
use crate::algebra::vec_minus;
use crate::algebra::Matrix;
use crate::algebra::Vector;
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



pub trait BBox<T, const D : usize> {
	fn union(&self, other : &Self) -> Self;
	fn intersection(&self, other : &Self) -> Self;
	fn is_empty(&self) -> bool;
	fn transform(&self, q : Matrix<T,D,D>, origin : Vector<T,D>) -> Self;
	fn shift(&self, s : Vector<T,D>) -> Self;
}


impl<T : ArithmeticOps, const D : usize> BBox<T,D> for BoundingBox<T,D> {
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
	fn shift(&self, s : Vector<T,D>) -> Self {
		let mut ret = self.clone();
		for i in 0..D {
			for j in 0..2 {
				ret[i][j] = self[i][j] + s[i];
			}
		}
		ret
	}

	fn transform(&self, q : Matrix<T,D,D>, origin : Vector<T,D>) -> Self {
		dot(q,self.shift(vec_minus(origin))).shift(origin).map(|x| if x[0] < x[1] {x} else {[x[1],x[0]]})
	}
}