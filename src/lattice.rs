use crate::math::ArithmeticOps;
use vecmath::vec2_add;
use rand::rngs::ThreadRng;
use vecmath::Vector2;
use vecmath::vec2_len;
use vecmath::vec2_square_len;
use rand::RngExt;


pub trait Lattice2<T: ArithmeticOps> {
	 fn degree(&self) -> usize;
	 fn get_turn(&self, k : usize) -> Vector2<T>;
	 fn symmetry_count(&self) -> usize;
	 fn get_symmetry(&self, k : usize) -> Vector2<T>;
	 

	 fn turns(&self) -> Vec<Vector2<T>> {
	 	(0..self.degree()).map(|i| self.get_turn(i)).collect()
	 }

	 fn symmetries(&self) -> Vec<Vector2<T>> {
	 	(0..self.symmetry_count()).map(|i| self.get_symmetry(i)).collect()
	 }

	 fn random_turn(&self, rng : &mut ThreadRng, pdf : Option<&[f64]>) -> Vector2<T> {
		match pdf {
			None => {self.get_turn(rng.random_range(0..self.degree()))},
			Some(x) => {
				let r = rng.random_range((0.)..(1.));
				let mut p = 0.;
				let mut k = 0;
				while k < self.degree() {
					p = p + x[k];
					if p >= r {
						return self.get_turn(k);
					}
					k = k + 1;
				}
				panic!("Sum of probability less than 1 !");
			},
		}
	}

	fn turns_to_coords(&self, turns : Vec<Vector2<T>>) -> Vec<Vector2<T>> {
		let mut ret = vec![[T::zero(),T::zero()]; turns.len()+1];
		for i in 0..turns.len() {
			ret[i+1] = vec2_add(ret[i], turns[i]);
		}
		ret
	}

	fn random_walk(&self, length : usize, pdf : Option<&[f64]>, rng : &mut ThreadRng) -> Vec<Vector2<T>> {
		if length == 0 {
			panic!("A walk of 0 points does not have turns !");
		}
		self.turns_to_coords((0..length).map(|_i| self.random_turn(rng, pdf)).collect())
	}

	fn collision(&self, a : Vector2<T>, b : Vector2<T>) -> bool {
	 	let dist = vec2_square_len([
	 		a[0] - b[0],
	 		a[1] - b[1],
	 	]);

	 	dist.is_zero()
	 }

	fn is_saw(&self, walk : &Vec<Vector2<T>>) -> bool {
		for i in 0..walk.len() {
			for j in (i+1)..walk.len() {
				if self.collision(walk[i], walk[j]) {
					return false;
				}
			}
		}
		true
	}

	fn random_saw_naive(&self, length : usize, rng : &mut ThreadRng) -> Vec<Vector2<T>> {
		loop {
			let walk = self.random_walk(length, None, rng);
			if self.is_saw(&walk) {
				return walk;
			}
		}
	}
}


#[derive(Clone, Debug)]
pub struct Basic2dLattice<T> {
	turns : Vec<Vector2<T>>,
	symmetries : Vec<Vector2<T>>,
}
impl<T: ArithmeticOps> Basic2dLattice<T> {
	pub fn new(turns : Vec<Vector2<T>>, symmetries : Vec<Vector2<T>>) -> Self {
		Basic2dLattice {
			turns : turns,
			symmetries : symmetries,
		}
	}
}
impl<T: ArithmeticOps> Lattice2<T> for Basic2dLattice<T> {

	fn degree(&self) -> usize { self.turns.len() }

	fn symmetry_count(&self) -> usize { self.symmetries.len() }

	fn get_turn(&self, k: usize) -> [T; 2] { 
		if k >= self.turns.len() {
			panic!("Turn number {k} does not exists !");
		}
		self.turns[k] 
	}

	fn get_symmetry(&self, k: usize) -> [T; 2] {
		if k >= self.symmetries.len() {
			panic!("Turn number {k} does not exists !");
		}
		self.turns[k] 
	}
}

#[derive(Clone, Debug)]
pub struct SquareGrid<T> {
	lat : Basic2dLattice<T>,
}
impl<T: ArithmeticOps> SquareGrid<T> {
	pub fn new(a : T) -> Self {
		SquareGrid {
			lat : Basic2dLattice::new(vec![[a,T::zero()],[T::zero(),a]], vec![]),
		}
	}
}
impl<T: ArithmeticOps> Lattice2<T> for SquareGrid<T> {

	fn degree(&self) -> usize { self.lat.turns.len() }

	fn symmetry_count(&self) -> usize { self.lat.symmetries.len() }

	fn get_turn(&self, k: usize) -> [T; 2] { 
		if k >= self.degree() {
			panic!("Turn number {k} does not exists !");
		}
		self.get_turn(k) 
	}

	fn get_symmetry(&self, k: usize) -> [T; 2] {
		if k >= self.symmetry_count() {
			panic!("Turn number {k} does not exists !");
		}
		self.get_symmetry(k) 
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

	vec2_len(ret[49])
}

pub fn dist_1000() -> f64 {
	let mut rng = rand::rng();
	let mut ret = [[0.,0.]; 1000];

	for i in 1..1000 {
		let k = rng.random_range(0..POOL.len());
		ret[i] = POOL[k].clone();
	}

	vec2_len(ret[999])
}


pub fn dist_100000() -> f64 {
	let mut rng = rand::rng();
	let mut ret = [[0.,0.]; 100000];

	for i in 1..100000 {
		let k = rng.random_range(0..POOL.len());
		ret[i] = POOL[k].clone();
	}

	vec2_len(ret[100000-1])
}