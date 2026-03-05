use rand::RngExt;
use rand::rngs::ThreadRng;
use crate::math::ArithmeticOps;
use crate::algebra::Matrix;


pub type SymmetryGroup<T : ArithmeticOps, const D : usize, const N : usize> = [Matrix<T,D,D>; N];

pub trait SymmetryGroupTrait<T : ArithmeticOps, const D : usize> {
	fn random(&self, rng : &mut ThreadRng) -> Matrix<T,D,D>;
}

impl<T : ArithmeticOps, const D : usize, const N : usize> SymmetryGroupTrait<T,D> for SymmetryGroup<T,D,N> {
	fn random(&self, rng : &mut ThreadRng) -> Matrix<T,D,D> {
		self[rng.random_range(0..self.len())]
	}
}



pub const DIHEDRAL6 : SymmetryGroup<i32, 2, 12> = [
	[[1,0],[0,1]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
	[[0,0],[0,0]],
];

pub const DIHEDRAL4 : SymmetryGroup<i32, 2, 8> = [
	[[1,0],[0,1]],
	[[-1,0],[0,-1]],
	[[0,-1],[1,0]],
	[[0,1],[-1,0]],
	[[-1,0],[0,1]],
	[[1,0],[0,-1]],
	[[0,1],[1,0]],
	[[0,-1],[-1,0]],
];


pub const DIHEDRAL2 : SymmetryGroup<i32, 2, 4> = [
	[[1,0],[0,1]],
	[[-1,0],[0,-1]],
	[[1,0],[0,-1]],
	[[-1,0],[0,1]],
];


pub const cyclic2 : SymmetryGroup<i32, 2, 2> = [
	[[1,0],[0,1]],
	[[-1,0],[0,-1]],
];


