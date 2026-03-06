use crate::symmetry_group::TD43M;
use crate::algebra::vec_sub;
use crate::symmetry_group::PM3M;
use rand::RngExt;
use rand::rngs::ThreadRng;
use crate::pivot::Pivot;
use crate::algebra::vec_add;
use num_traits::PrimInt;
use crate::symmetry_group::DIHEDRAL4;
use crate::symmetry_group::SymmetryGroup;
use crate::algebra::Vector;

pub trait Lattice<T : PrimInt + std::hash::Hash, const D: usize, const N: usize> {
	//! This is the trait for lattices. You can implement it youself or use the already implemented structs.</br>
    //! Note that it requires the symmetry group for the Pivot algorithm to work.
    /// This function must return the symmetry group of the lattice.
	fn symmetries(&self) -> SymmetryGroup<T,D,N>;
	/// This function must return a SAW. It does not have to be random and can be a straight line.
	fn straight(&self, n : usize) -> Vec<Vector<T,D>>;
	/// This function returns, given a point of the lattice, all the neighbors of that point.
	fn neighbors(&self, point : Vector<T,D>) -> Vec<Vector<T,D>>;

	fn get_pivot(&self, n : usize, rng : ThreadRng, thermalization_factor : usize, autocorrelation_factor : usize) -> Pivot<T,D,N> {
		//! This is the main way to define a Pivot iterator.</br>
		//! This function returns an iterator that runs the pivot algorithm and generate SAW of a given size.</br>
		//! </br><b>Args:</b>
		//!  - n: The size of the SAW to be generated.
		//!  - rng: A ThreadRng oject. Can be used to obtain reproducable results.
		//!  - thermalization_factor: Define how many pivots must be accepted before returning the iterator.
		//!  - autocorrelation_factor: Define how many pivots must be accepted before returning a value. If equal to 0, the Pivot algorithm does nothing.
		//!</br></br>
		//!This function returns a Pivot algorithm that has been initialized. </br>
		//!This means that several iterations of the Pivot algorithm are calculated and ignored before the Pivot is returned.
		//! This is necessary to ensure that the results are as random as possible.</br>
		//!</br>
		//! Specifically, n*thermalization_factor accepted pivots are calculated.
		//! Between each next() of the iterator, autocorrelation_factor pivots are accepted.

		let walk = self.straight(n);
		let symmetries = self.symmetries();
		Pivot::new(walk, symmetries, rng, thermalization_factor, autocorrelation_factor)
	}

	fn random_walk(&self, n : usize, rng : &mut ThreadRng) -> Vec<Vector<T,D>> {
		let mut ret = vec![[T::zero(); D]; n];
		for i in 1..n {
			let ng = self.neighbors(ret[i-1]);
			ret[i] = ng[rng.random_range(0..ng.len())];
		}
		ret
	}
}

/// A lattice with a base. Includes most lattices you can think of. </br>
/// Some basis and symmetry groups have been implemented for common lattices.
#[derive(Clone, Debug)]
pub struct BaseLattice<T : PrimInt, const D: usize, const N: usize, const K : usize> {
	pub turns : [Vector<T,D>; K],
	pub symmetries : SymmetryGroup<T,D,N>,
}
impl<T: PrimInt, const D: usize, const N: usize, const K : usize> BaseLattice<T,D,N,K> {
	/// This function creates a lattice from its basis and symmetry group.</br>
	/// Check out square and cubic grids for examples of implementation.
	pub fn new(turns : [Vector<T,D>; K], symmetries : SymmetryGroup<T,D,N>) -> Self {
		BaseLattice {
			turns : turns,
			symmetries : symmetries,
		}
	}
}
impl<T: PrimInt + std::ops::Neg<Output = T>> BaseLattice<T, 2, 8, 4> where T: From<i32> {
	/// This function creates a square grid with arc length `a`.
	pub fn square_grid(a : T) -> Self {
		let mut turns = [[T::zero(); 2]; 4];
		for i in 0..2 {
			turns[i][i] = a;
			turns[2+i][i] = -a;
		}
		BaseLattice {
			turns : turns,
			symmetries : DIHEDRAL4.map(|x| x.map(|y| y.map(|z| z.into()))),
		}
	}
}
impl<T: PrimInt + std::ops::Neg<Output = T>> BaseLattice<T, 3, 48, 6> where T: From<i32> {
	/// This function creates a cubic 3d grid with arc length `a`.
	pub fn cubic_grid(a : T) -> Self {
		let turns = [
			[a, T::zero(), T::zero()],
			[T::zero(), a, T::zero()],
			[T::zero(), T::zero(), a],
			[-a, T::zero(), T::zero()],
			[T::zero(), -a, T::zero()],
			[T::zero(), T::zero(), -a],
		];

		BaseLattice {
			turns : turns,
			symmetries : PM3M.map(|x| x.map(|y| y.map(|z| z.into()))),
		}
	}
}
impl<T: PrimInt + std::ops::Neg<Output = T>> BaseLattice<T, 3, 48, 8> where T: From<i32> {
	/// This function creates a body centered cubic lattice with arc length `a`.
	pub fn bcc(a : T) -> Self {
		let turns = [
			[a, a, a],
			[-a, a, a],
			[a, -a, a],
			[a, a, -a],
			[a, -a, -a],
			[-a, a, -a],
			[-a, -a, a],
			[-a, -a, -a],
		];

		BaseLattice {
			turns : turns,
			symmetries : PM3M.map(|x| x.map(|y| y.map(|z| z.into()))),
		}
	}
}

impl<T: PrimInt + std::ops::Neg<Output = T>> BaseLattice<T, 3, 48, 12> where T: From<i32> {
	/// This function creates a face centered cubic lattice with arc length `a`.
	pub fn fcc(a : T) -> Self {
		let turns = [
			[a, a, T::zero()],
			[a, -a, T::zero()],
			[a, T::zero(), a],
			[a, T::zero(), -a],
			[-a, a, T::zero()],
			[-a, -a, T::zero()],
			[-a, T::zero(), a],
			[-a, T::zero(), -a],
			[T::zero(), a, a],
			[T::zero(), a, -a],
			[T::zero(), -a, a],
			[T::zero(), -a, -a],
		];

		BaseLattice {
			turns : turns,
			symmetries : PM3M.map(|x| x.map(|y| y.map(|z| z.into()))),
		}
	}
}


impl<T: PrimInt + std::hash::Hash, const D: usize, const N: usize, const K : usize> Lattice<T,D,N> for BaseLattice<T,D,N,K> {
	fn symmetries(&self) -> [[[T; D]; D]; N] { self.symmetries.clone() }
	fn neighbors(&self, point : [T; D]) -> Vec<[T; D]> { 
		self.turns.iter().map(|x| vec_add(*x,point)).collect()
	}
	fn straight(&self, n : usize) -> Vec<[T; D]> { 
		let mut ret = vec![[T::zero(); D]; n];
		for i in 1..n {
			ret[i] = self.neighbors(ret[i-1])[0];
		}
		ret
	}
}









/// This struct implement the tetrahedral lattice. </br>
/// The tetrahedral lattice is not a proper lattice with a base. Indeed, odd and even positions do not have the same turns' direction.</br>
#[derive(Clone, Debug)]
pub struct Tetrahedral<T : PrimInt> {
	pub turns : [Vector<T,3>; 4],
	pub symmetries : SymmetryGroup<T,3,24>,
}
impl<T: PrimInt + std::convert::From<i32> + std::ops::Neg<Output = T>> Tetrahedral<T> {
	/// This function creates a tetrahedral lattice with arc length `a`.
	pub fn new(a : T) -> Self {
		let turns = [
			[-a, a, a],
			[a, -a, a],
			[a, a, -a],
			[-a, -a, -a]
		];
		Tetrahedral {
			turns : turns,
			symmetries : TD43M.map(|x| x.map(|y| y.map(|z| z.into()))),
		}
	}
}

impl<T: PrimInt + std::hash::Hash + std::fmt::Debug> Lattice<T,3,24> for Tetrahedral<T> where i64: From<T> {
	fn symmetries(&self) -> [[[T; 3]; 3]; 24] { self.symmetries.clone() }
	fn neighbors(&self, point : [T; 3]) -> Vec<[T; 3]> { 
		let sum = <T as Into<i64>>::into(point.into_iter().fold(T::zero(),|acc,x| acc+x));

		if sum.abs() %2 == 0 {
			self.turns.iter().map(|x| vec_add(*x,point)).collect()
		}
		else {
			self.turns.iter().map(|x| vec_sub(point, *x)).collect()
		}
		
	}
	fn straight(&self, n : usize) -> Vec<[T; 3]> { 
		let mut ret = vec![[T::zero(); 3]; n];
		for i in 1..n {
			ret[i] = self.neighbors(ret[i-1])[i%2];
		}
		ret
	}
}


#[cfg(test)]
mod tests {

	use crate::lattice::Lattice;
use crate::lattice::Tetrahedral;

#[test]
	fn tet_straight() {
    	let lat = Tetrahedral::new(1);
    	let walk = lat.straight(7);
    	assert_eq!(walk[6], [6,-6,0]);
	}
}