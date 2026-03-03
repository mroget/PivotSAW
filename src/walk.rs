use crate::lattice::Lattice;
use crate::math::ArithmeticOps;
use crate::algebra::vec_sub;
use crate::algebra::vec_square_len;
use rand::rngs::ThreadRng;
use crate::algebra::Vector;
use crate::algebra::vec_add;

pub fn turns_to_coords<T : ArithmeticOps, const D : usize>(turns : Vec<Vector<T,D>>) -> Vec<Vector<T,D>> {
		let mut ret = vec![[T::zero();D]; turns.len()+1];
		for i in 0..turns.len() {
			ret[i+1] = vec_add(ret[i], turns[i]);
		}
		ret
	}

pub fn random_walk<T : ArithmeticOps, const D : usize>(lat : &Lattice<T,D>, length : usize, pdf : Option<&[f64]>, rng : &mut ThreadRng) -> Vec<Vector<T,D>> {
	if length == 0 {
		panic!("A walk of 0 points does not have turns !");
	}
	turns_to_coords((0..length).map(|_i| lat.random_turn(rng, pdf)).collect())
}

pub fn collision<T : ArithmeticOps, const D : usize>(a : Vector<T,D>, b : Vector<T,D>) -> bool {
	let dist = vec_square_len(vec_sub(a,b));

	dist.is_zero()
}

pub fn is_saw<T : ArithmeticOps, const D : usize>(walk : &Vec<Vector<T,D>>) -> bool {
	for i in 0..walk.len() {
		for j in (i+1)..walk.len() {
			if collision(walk[i], walk[j]) {
				return false;
			}
		}
	}
	true
}

pub fn random_saw_naive<T : ArithmeticOps, const D : usize>(lat : &Lattice<T,D>, length : usize, rng : &mut ThreadRng) -> Vec<Vector<T,D>> {
	loop {
		let walk = random_walk(lat, length, None, rng);
		if is_saw(&walk) {
			return walk;
		}
	}
}