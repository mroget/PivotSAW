
use crate::math::max;
use crate::math::min;
use crate::math::ArithmeticOps;

pub type BoundingBox2<T> = [[T;2];2];
pub type BoundingBox3<T> = [[T;2];3];
pub type BoundingBox4<T> = [[T;2];4];


pub trait New2d<T : ArithmeticOps> {
	fn new(point : [T; 2]) -> BoundingBox2<T> {
		point.map(|x| [x, x])
	}
}
impl<T : ArithmeticOps> New2d<T> for BoundingBox2<T> {}

pub trait New3d<T : ArithmeticOps> {
	fn new(point : [T; 3]) -> BoundingBox3<T> {
		point.map(|x| [x, x])
	}
}
impl<T : ArithmeticOps> New3d<T> for BoundingBox3<T> {}

pub trait New4d<T : ArithmeticOps> {
	fn new(point : [T; 4]) -> BoundingBox4<T> {
		point.map(|x| [x, x])
	}
}
impl<T : ArithmeticOps> New4d<T> for BoundingBox4<T> {}




pub trait BBox<T> {
	fn union(&self, other : &Self) -> Self;
	fn intersection(&self, other : &Self) -> Self;
	fn is_empty(&self) -> bool;
}

impl<T : ArithmeticOps> BBox<T> for BoundingBox2<T> {
	fn union(&self, other : &Self) -> Self {
		[0,1].map(|i| [min(self[i][0], other[i][0]), max(self[i][1], other[i][1])])
	}
	fn intersection(&self, other: &Self) -> Self {
		[0,1].map(|i| [max(self[i][0], other[i][0]), min(self[i][1], other[i][1])])
	}
	fn is_empty(&self) -> bool {
		for i in 0..2 {
			if self[i][0] > self[i][1] {
				return true;
			}
		}
		false
	}
}


impl<T : ArithmeticOps> BBox<T> for BoundingBox3<T> {
	fn union(&self, other : &Self) -> Self {
		[0,1,2].map(|i| [min(self[i][0], other[i][0]), max(self[i][1], other[i][1])])
	}
	fn intersection(&self, other: &Self) -> Self {
		[0,1,2].map(|i| [max(self[i][0], other[i][0]), min(self[i][1], other[i][1])])
	}
	fn is_empty(&self) -> bool {
		for i in 0..3 {
			if self[i][0] > self[i][1] {
				return true;
			}
		}
		false
	}
}


impl<T : ArithmeticOps> BBox<T> for BoundingBox4<T> {
	fn union(&self, other : &Self) -> Self {
		[0,1,2,3].map(|i| [min(self[i][0], other[i][0]), max(self[i][1], other[i][1])])
	}
	fn intersection(&self, other: &Self) -> Self {
		[0,1,2,3].map(|i| [max(self[i][0], other[i][0]), min(self[i][1], other[i][1])])
	}
	fn is_empty(&self) -> bool {
		for i in 0..4 {
			if self[i][0] > self[i][1] {
				return true;
			}
		}
		false
	}
}
