use crate::symmetry_group::DIHEDRAL4;
use crate::symmetry_group::SymmetryGroup;
use crate::algebra::vec_len;
use crate::algebra::Vector;
use crate::math::ArithmeticOps;
use rand::rngs::ThreadRng;
use rand::RngExt;

#[derive(Clone, Debug)]
pub struct Lattice<T, const D: usize, const N: usize> {
	pub turns : Vec<Vector<T,D>>,
	pub symmetries : SymmetryGroup<T,D,N>,
}
impl<T: ArithmeticOps, const D: usize, const N: usize> Lattice<T, D, N> {
	pub fn new(turns : Vec<Vector<T,D>>, symmetries : SymmetryGroup<T,D,N>) -> Self {
		Lattice {
			turns : turns,
			symmetries : symmetries,
		}
	}
}
impl<T: ArithmeticOps> Lattice<T, 2, 8> where [[[T; 2]; 2]; 8]: From<[[[i32; 2]; 2]; 8]> {
	pub fn square_grid(a : T) -> Self {
		let mut turns = vec![[T::zero(); 2]; 2];
		for i in 0..2 {
			turns[i][i] = a;
		}
		Lattice {
			turns : turns,
			symmetries : DIHEDRAL4.into(),
		}
	}
}

impl<T: ArithmeticOps, const D: usize, const N: usize> Lattice<T, D, N> {

	pub fn random_turn(&self, rng : &mut ThreadRng, pdf : Option<&[f64]>) -> Vector<T,D> {
		match pdf {
			None => {self.turns[rng.random_range(0..self.turns.len())]},
			Some(x) => {
				let r = rng.random_range((0.)..(1.));
				let mut p = 0.;
				let mut k = 0;
				while k < self.turns.len() {
					p = p + x[k];
					if p >= r {
						return self.turns[k]
					}
					k = k + 1;
				}
				panic!("Sum of probability less than 1 !");
			},
		}
	}
}















const POOL: [[f64; 2];4] = [
	[1.,0.],
	[-1.,0.],
	[0.,1.],
	[0.,-1.]
];

pub fn dist_50() -> f64 {
	let mut rng = rand::rng();
	let mut ret = [[0.,0.]; 50];

	for i in 1..50 {
		let k = rng.random_range(0..POOL.len());
		ret[i] = POOL[k].clone();
	}

	vec_len(ret[49])
}

pub fn dist_1000() -> f64 {
	let mut rng = rand::rng();
	let mut ret = [[0.,0.]; 1000];

	for i in 1..1000 {
		let k = rng.random_range(0..POOL.len());
		ret[i] = POOL[k].clone();
	}

	vec_len(ret[999])
}


pub fn dist_100000() -> f64 {
	let mut rng = rand::rng();
	let mut ret = [[0.,0.]; 100000];

	for i in 1..100000 {
		let k = rng.random_range(0..POOL.len());
		ret[i] = POOL[k].clone();
	}

	vec_len(ret[100000-1])
}