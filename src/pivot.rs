use num_traits::PrimInt;
use crate::algebra::transpose;
use std::collections::HashSet;

use crate::algebra::transform;
use crate::algebra::Matrix;
use rand::RngExt;
use crate::symmetry_group::SymmetryGroupTrait;
use rand::rngs::ThreadRng;
use crate::algebra::Vector;
use crate::symmetry_group::SymmetryGroup;

/// An iterator which returns random SAW through the pivot algorithm.</br>
/// This iterator is infinite.</br>
/// </br></br><b>Example</b>
/// ```rust
/// let lat = BaseLattice::fcc(1);
/// let mut pivot = lat.get_pivot(100, rand::rng(), 10, 10); // SAW of length 10 in the FCC lattice.
/// ```
#[derive(Debug)]
pub struct Pivot<T : PrimInt + std::hash::Hash, const D : usize, const N : usize> {
	pub walk : Vec<Vector<T,D>>,
	pub symmetries : SymmetryGroup<T,D,N>,
	pub n : usize,
	rng : ThreadRng,
	autocorrelation_factor : usize,
}

impl<T : PrimInt + std::hash::Hash, const D : usize, const N : usize> Pivot<T,D,N> {
	pub fn new(walk : Vec<Vector<T,D>>, symmetries : SymmetryGroup<T,D,N>, rng : ThreadRng, thermalization_factor : usize, autocorrelation_factor : usize) -> Self {
		let n = walk.len();
		let mut p = Pivot {
			walk : walk,
			n : n,
			symmetries : symmetries,
			rng : rng,
			autocorrelation_factor : autocorrelation_factor,
		};
		p.pivot_multiple(thermalization_factor*n);
		p
	}

	fn get_walk(&self) -> Vec<Vector<T,D>> {
		self.walk.clone()
	}
}

impl<T : PrimInt + std::hash::Hash, const D : usize, const N : usize> Pivot<T,D,N> {
	fn apply_symmetry(&mut self, q : Matrix<T,D,D>, r : usize) {
		// r is not affected
		for i in (r+1)..self.walk.len() {
			self.walk[i] = transform(self.walk[i], q, self.walk[r]);
		}
	}

	fn check_collisions_naive(&mut self) -> bool {
		for i in 0..self.walk.len() {
			for j in (i+1)..self.walk.len() {
				if self.walk[i] == self.walk[j] {
					return true;
				}
			}
		}
		false
	}

	fn check_collisions(&mut self) -> bool {
		let mut set = HashSet::new();
		for i in 0..self.walk.len() {
			if set.contains(&self.walk[i]) {
				return true;
			}
			else {
				set.insert(self.walk[i]);
			}
		}
		false
	}

	fn pivot(&mut self) -> bool{
		let r = self.rng.random_range(0..(self.walk.len()-1));
		let q = self.symmetries.random(&mut self.rng);
		self.apply_symmetry(q,r);
		if if self.walk.len() < 100 {self.check_collisions_naive()} else {self.check_collisions()} {
			self.apply_symmetry(transpose(q),r);
			return false;
		}
		true
	} 


	fn pivot_multiple(&mut self, nb : usize) {
		for _i in 0..nb {
			while !self.pivot() {}
		}
	}
} 

impl<T : PrimInt + std::hash::Hash, const D : usize, const N : usize> Iterator for Pivot<T,D,N> {
	type Item = Vec<Vector<T,D>>;
	fn next(&mut self) -> Option<Self::Item> {
		self.pivot_multiple(self.autocorrelation_factor);
		Some(self.get_walk())
	}
}













#[cfg(test)]
mod tests {
	use crate::lattice::Tetrahedral;
use std::hash::Hash;
use crate::pivot::PrimInt;
use std::collections::HashSet;
use crate::algebra::Vector;
use crate::algebra::vec_square_len;
use crate::walk::SAWIterator;
use std::collections::HashMap;

use crate::lattice::Lattice;
use crate::lattice::BaseLattice;
use crate::symmetry_group::DIHEDRAL4;
	use crate::pivot::Pivot;
	
	#[test]
	fn symmetry() {
		let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2]];
		let rng = rand::rng();
		let mut pivot = Pivot::new(walk.clone(), DIHEDRAL4, rng, 0, 0);
		pivot.walk = walk;
		pivot.apply_symmetry(DIHEDRAL4[1], 2);
		println!("{:?}", DIHEDRAL4[1]);
		assert_eq!(pivot.get_walk(), vec![[0,0], [0,1], [0,2], [-1,2], [-1,1], [-2,1], [-2,2]]);
	}

	#[test]
	fn symmetry2() {
		let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2]];
		let rng = rand::rng();
		let mut pivot = Pivot::new(walk.clone(), DIHEDRAL4, rng, 0, 0);
		pivot.walk = walk;
		pivot.apply_symmetry(DIHEDRAL4[2], 2);
		assert_eq!(pivot.get_walk(), vec![[0,0], [0,1], [0,2], [0,3], [-1,3], [-1,4], [0,4]]);
	}

	#[test]
	fn collision1() {
		let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2]];
		let rng = rand::rng();
		let mut pivot = Pivot::new(walk.clone(), DIHEDRAL4, rng, 0, 0);
		pivot.walk = walk;
		assert_eq!(pivot.check_collisions_naive(), false);
	}

	#[test]
	fn collision2() {
		let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2], [1,2]];
		let rng = rand::rng();
		let mut pivot = Pivot::new(walk.clone(), DIHEDRAL4, rng, 0, 0);
		pivot.walk = walk;
		assert_eq!(pivot.check_collisions_naive(), true);
	}

	#[test]
	fn collision3() {
		let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2], [1,2]];
		let rng = rand::rng();
		let mut pivot = Pivot::new(walk.clone(), DIHEDRAL4, rng, 0, 0);
		pivot.walk = walk;
		assert_eq!(pivot.check_collisions(), true);
	}

	#[test]
	fn collision4() {
		let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2]];
		let rng = rand::rng();
		let mut pivot = Pivot::new(walk.clone(), DIHEDRAL4, rng, 0, 0);
		pivot.walk = walk;
		assert_eq!(pivot.check_collisions(), false);
	}
	
	#[test]
	fn stats_grid() {
		let len = 10;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::square_grid(1);
    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut count = HashMap::new();
    	for w in SAWIterator::new(lat.clone(), len) {
    		count.insert(w, 0);
    	}
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		*count.get_mut(&w).unwrap() += 1;
    	}

    	let mut err = 0.;
    	for (_,&val) in count.iter() {
    		err = err + (1./(count.len() as f64) - (val as f64)/(repeat as f64)).powf(2.);
    	}
    	println!("{}", err);
		assert!(err <= 1e-3);
	}

	
	#[test]
	fn stats_cubic() {
		let len = 10;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::cubic_grid(1);
    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut count = HashMap::new();
    	for w in SAWIterator::new(lat.clone(), len) {
    		count.insert(w, 0);
    	}
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		*count.get_mut(&w).unwrap() += 1;
    	}

    	let mut err = 0.;
    	for (_,&val) in count.iter() {
    		err = err + (1./(count.len() as f64) - (val as f64)/(repeat as f64)).powf(2.);
    	}
    	println!("{}", err);
		assert!(err <= 1e-3);
	}

	#[test]
	fn stats_bcc() {
		let len = 7;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::bcc(1);
    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut count = HashMap::new();
    	for w in SAWIterator::new(lat.clone(), len) {
    		count.insert(w, 0);
    	}
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		*count.get_mut(&w).unwrap() += 1;
    	}

    	let mut err = 0.;
    	for (_,&val) in count.iter() {
    		err = err + (1./(count.len() as f64) - (val as f64)/(repeat as f64)).powf(2.);
    	}
    	println!("{}", err);
		assert!(err <= 1e-3);
	}

	#[test]
	fn stats_fcc() {
		let len = 7;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::fcc(1);
    	let mut pivot = lat.get_pivot(len, rng, 3, 3);
    	let mut count = HashMap::new();
    	for w in SAWIterator::new(lat.clone(), len) {
    		count.insert(w, 0);
    	}
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		*count.get_mut(&w).unwrap() += 1;
    	}

    	let mut err = 0.;
    	for (_,&val) in count.iter() {
    		err = err + (1./(count.len() as f64) - (val as f64)/(repeat as f64)).powf(2.);
    	}
    	println!("{}", err);
		assert!(err <= 1e-3);
	}

	#[test]
	fn stats_tet() {
		let len = 10;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = Tetrahedral::new(1);
    	let mut pivot = lat.get_pivot(len, rng, 3, 3);
    	let mut count = HashMap::new();
    	for w in SAWIterator::new(lat.clone(), len) {
    		count.insert(w, 0);
    	}
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		*count.get_mut(&w).unwrap() += 1;
    	}

    	let mut err = 0.;
    	for (_,&val) in count.iter() {
    		err = err + (1./(count.len() as f64) - (val as f64)/(repeat as f64)).powf(2.);
    	}
    	println!("{}", err);
		assert!(err <= 1e-3);
	}

	#[test]
	fn dist_square() {
		let len = 10;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::square_grid(1);

    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut dist1 = 0;
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		dist1 += vec_square_len(w[w.len()-1]);
    	}
    	let d1 = (dist1 as f64) / (repeat as f64);
    	let mut dist2 = 0;
    	let mut k = 0;
    	for w in SAWIterator::new(lat.clone(), len) {
    		dist2 += vec_square_len(w[w.len()-1]);
    		k+=1;
    	}
    	println!("{}",k);
    	let d2 = (dist2 as f64) / (k as f64);

    	let err = ((d1-d2) as f64).abs() / (d2 as f64);
    	println!("{} {} {}", d1, d2, err);
		assert!(err <= 5e-2);
	}
	#[test]
	fn dist_cubic() {
		let len = 10;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::cubic_grid(1);

    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut dist1 = 0;
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		dist1 += vec_square_len(w[w.len()-1]);
    	}
    	let d1 = (dist1 as f64) / (repeat as f64);
    	let mut dist2 = 0;
    	let mut k = 0;
    	for w in SAWIterator::new(lat.clone(), len) {
    		dist2 += vec_square_len(w[w.len()-1]);
    		k+=1;
    	}
    	println!("{}",k);
    	let d2 = (dist2 as f64) / (k as f64);

    	let err = ((d1-d2) as f64).abs() / (d2 as f64);
    	println!("{} {} {}", d1, d2, err);
		assert!(err <= 5e-2);
	}

	#[test]
	fn dist_bcc() {
		let len = 7;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::bcc(1);

    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut dist1 = 0;
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		dist1 += vec_square_len(w[w.len()-1]);
    	}
    	let d1 = (dist1 as f64) / (repeat as f64);
    	let mut dist2 = 0;
    	let mut k = 0;
    	for w in SAWIterator::new(lat.clone(), len) {
    		dist2 += vec_square_len(w[w.len()-1]);
    		k+=1;
    	}
    	println!("{}",k);
    	let d2 = (dist2 as f64) / (k as f64);

    	let err = ((d1-d2) as f64).abs() / (d2 as f64);
    	println!("{} {} {}", d1, d2, err);
		assert!(err <= 5e-2);
	}

	#[test]
	fn dist_fcc() {
		let len = 7;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = BaseLattice::fcc(1);

    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut dist1 = 0;
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		dist1 += vec_square_len(w[w.len()-1]);
    	}
    	let d1 = (dist1 as f64) / (repeat as f64);
    	let mut dist2 = 0;
    	let mut k = 0;
    	for w in SAWIterator::new(lat.clone(), len) {
    		dist2 += vec_square_len(w[w.len()-1]);
    		k+=1;
    	}
    	println!("{}",k);
    	let d2 = (dist2 as f64) / (k as f64);

    	let err = ((d1-d2) as f64).abs() / (d2 as f64);
    	println!("{} {} {}", d1, d2, err);
		assert!(err <= 5e-2);
	}

	#[test]
	fn dist_tet() {
		let len = 10;
		let repeat = 10000;
		let rng = rand::rng();
    	let lat = Tetrahedral::new(1);

    	let mut pivot = lat.get_pivot(len, rng, 10, 10);
    	let mut dist1 = 0;
    	for _i in 0..repeat {
    		let w = pivot.next().unwrap();
    		dist1 += vec_square_len(w[w.len()-1]);
    	}
    	let d1 = (dist1 as f64) / (repeat as f64);
    	let mut dist2 = 0;
    	let mut k = 0;
    	for w in SAWIterator::new(lat.clone(), len) {
    		dist2 += vec_square_len(w[w.len()-1]);
    		k+=1;
    	}
    	println!("{}",k);
    	let d2 = (dist2 as f64) / (k as f64);

    	let err = ((d1-d2) as f64).abs() / (d2 as f64);
    	println!("{} {} {}", d1, d2, err);
		assert!(err <= 5e-2);
	}
}