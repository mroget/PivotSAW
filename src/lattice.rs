use crate::algebra::vec_len;
use crate::algebra::Vector;
use crate::math::ArithmeticOps;
use rand::rngs::ThreadRng;
use rand::RngExt;

#[derive(Clone, Debug)]
pub struct Lattice<T, const D: usize> {
	turns : Vec<Vector<T,D>>,
	symmetries : Vec<Vector<T,D>>,
}
impl<T: ArithmeticOps, const D: usize> Lattice<T, D> {
	pub fn new(turns : Vec<Vector<T,D>>, symmetries : Vec<Vector<T,D>>) -> Self {
		Lattice {
			turns : turns,
			symmetries : symmetries,
		}
	}

	pub fn square_grid(a : T) -> Self {
		let mut turns = vec![[T::zero(); D]; D];
		for i in 0..D {
			turns[i][i] = a;
		}
		Lattice {
			turns : turns,
			symmetries : vec![],
		}
	}
}
impl<T: ArithmeticOps, const D: usize> Lattice<T, D> {

	pub fn degree(&self) -> usize { self.turns.len() }

	pub fn symmetry_count(&self) -> usize { self.symmetries.len() }

	pub fn get_turn(&self, k: usize) -> Vector<T,D> { 
		if k >= self.turns.len() {
			panic!("Turn number {k} does not exists !");
		}
		self.turns[k] 
	}

	pub fn get_symmetry(&self, k: usize) -> Vector<T,D> {
		if k >= self.symmetries.len() {
			panic!("Turn number {k} does not exists !");
		}
		self.turns[k] 
	}

	pub fn turns(&self) -> Vec<Vector<T,D>> {
	 	(0..self.turns.len()).map(|i| self.turns[i]).collect()
	 }

	pub fn symmetries(&self) -> Vec<Vector<T,D>> {
	 	(0..self.symmetries.len()).map(|i| self.symmetries[i]).collect()
	 }

	pub fn random_turn(&self, rng : &mut ThreadRng, pdf : Option<&[f64]>) -> Vector<T,D> {
		match pdf {
			None => {self.turns[rng.random_range(0..self.degree())]},
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