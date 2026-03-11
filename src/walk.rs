
use crate::algebra::Vector;
use num_traits::PrimInt;
use crate::lattice::Lattice;

#[derive(Debug, Clone, Copy)]
enum Arc<T: PrimInt, const D : usize> {
	Forward(Vector<T,D>),
	Backward,
}
/// An iterator that lists all SAW of a given length. This is going to be a very large iterator for walk length above 10 for most lattices.
pub struct SAWIterator<T : PrimInt + std::hash::Hash, const D : usize, const N : usize, L : Lattice<T, D, N>> {
	lat : L,
	walk : Vec<Vector<T,D>>,
	stack : Vec<Arc<T,D>>,
	over : bool,
	n : usize,
}

impl<T : PrimInt + std::hash::Hash, const D : usize, const N : usize, L : Lattice<T, D, N>> SAWIterator<T,D,N,L> {
	/// Create an iterator for a given lattice and walk length.
	pub fn new(lat : L, n :usize, prefix : Vec<[T;D]>) -> Self {
		let mut prefix = prefix;
		let origin = match prefix.pop() {
			Some(x) => {x},
			None => {[T::zero(); D]},
		};
		SAWIterator {
			lat : lat,
			walk : prefix,
			stack : vec![Arc::Forward(origin)],
			over : false,
			n : n,
		}
	}

	fn collision_with_tail(&self) -> bool {
		for i in 0..(self.walk.len()-1) {
			if self.walk[i] == self.walk[self.walk.len()-1] {
				return true;
			}
		}
		false
	}

	fn visit(&mut self) -> Option<Vec<Vector<T,D>>>{
		match self.stack.pop() {
			None => {self.over = true; return None;},
			Some(Arc::Backward) => {
				self.walk.pop();
			},
			Some(Arc::Forward(x)) => {
				self.walk.push(x);
				self.stack.push(Arc::Backward);
				if self.collision_with_tail() {
					return None;
				}
				if self.walk.len() == self.n {
					return Some(self.walk.clone());
				}
				for ng in self.lat.neighbors(x).into_iter() {
					self.stack.push(Arc::Forward(ng));
				}
			}
		}
		None
	}
}

impl<T : PrimInt + std::hash::Hash, const D : usize, const N : usize, L : Lattice<T, D, N>> Iterator for SAWIterator<T,D,N,L> {
	type Item = Vec<Vector<T,D>>;
	fn next(&mut self) -> Option<Self::Item> {
		while !self.over {
			match self.visit() {
				None => {},
				Some(x) => {return Some(x);}
			}
		}
		None
	}
}


#[cfg(test)]
mod tests {
	use crate::lattice::BaseLattice;
	use crate::walk::SAWIterator;

	#[test]
	fn iter_1() {
    	let lat = BaseLattice::square_grid(1);
    	let it = SAWIterator::new(lat, 2, vec![]);
    	let v: Vec<_> = it.map(|x| x).collect();
    	assert_eq!(v.len(), 4);
	}
	#[test]
	fn iter_2() {
    	let lat = BaseLattice::square_grid(1);
    	let it = SAWIterator::new(lat, 3, vec![]);
    	let v: Vec<_> = it.map(|x| x).collect();
    	assert_eq!(v.len(), 12);
	}
	#[test]
	fn iter_3() {
    	let lat = BaseLattice::square_grid(1);
    	let it = SAWIterator::new(lat, 4, vec![]);
    	let v: Vec<_> = it.map(|x| x).collect();
    	assert_eq!(v.len(), 36);
	}
	#[test]
	fn iter_4() {
    	let lat = BaseLattice::square_grid(1);
    	let it = SAWIterator::new(lat, 5, vec![]);
    	let v: Vec<_> = it.map(|x| x).collect();
    	assert_eq!(v.len(), 100);
	}
	#[test]
	fn iter_5() {
    	let lat = BaseLattice::square_grid(1);
    	let it = SAWIterator::new(lat, 6, vec![]);
    	let v: Vec<_> = it.map(|x| x).collect();
    	assert_eq!(v.len(), 284);
	}
}