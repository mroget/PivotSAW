
use std::cmp::max;
use crate::algebra::vec_add;
use crate::algebra::transform;
use crate::algebra::vec_minus;
use crate::boundingbox::BBox;
use crate::symmetry_group::DIHEDRAL4;
use crate::boundingbox::NewBoundingBox;
use rand::rngs::ThreadRng;
use crate::algebra::Vector;
use crate::symmetry_group::SymmetryGroup;
use crate::algebra::transpose;
use crate::algebra::dot;
use crate::math::ArithmeticOps;
use crate::algebra::Matrix;
use crate::boundingbox::BoundingBox;

#[derive(Debug, Clone, Copy)]
pub struct Node<T : ArithmeticOps, const D : usize> {
	pub id : usize,
	pub parent : Option<usize>,
	pub left : Option<usize>,
	pub right : Option<usize>,
	pub matrix : Matrix<T,D,D>,
	pub bounding_box : BoundingBox<T, D>,
	pub tail : Vector<T,D>,
}
impl<T : ArithmeticOps, const D : usize> Node<T,D> {
	pub fn leaf(&self) -> bool {
		match (self.left,self.right) {
			(None,None) => {true},
			_ => {false}
		}
	}

	pub fn root(&self) -> bool {
		match self.parent {
			None => {true},
			_ => {false}
		}
	}
}


#[derive(Debug)]
pub struct SAWTree<T : ArithmeticOps, const D : usize, const N : usize> {
	pub arena : Vec<Node<T, D>>,
	pub symmetries : SymmetryGroup<T, D, N>,
	pub root : usize,
	pub leaves : Vec<usize>,
	pub nodes : Vec<usize>,
	pub rng : ThreadRng,
}

impl<T : ArithmeticOps, const D : usize, const N : usize> SAWTree<T,D,N> {
	pub fn new(arena : Vec<Node<T, D>>, symmetries : SymmetryGroup<T, D, N>, rng : ThreadRng) -> Self {
		let mut root = 0;
		let mut leaves = Vec::new();
		let mut nodes = Vec::new();

		for i in 0..arena.len() {
			if arena[i].leaf() {
				leaves.push(i);
			}
			if !arena[i].leaf() {
				nodes.push(i);
			}
			if arena[i].root() {
				root = i;
			}
		}

		SAWTree {
			arena : arena,
			symmetries : symmetries,
			root : root,
			leaves : leaves,
			nodes : nodes,
			rng : rng
		}
	}

	pub fn from_walk(walk : &Vec<Vector<T,D>>, symmetries : SymmetryGroup<T, D, N>, rng : ThreadRng) -> Self {
		let n = walk.len();
		let mut depth = vec![0; 2*n-1];
		let mut arena = vec![
			Node {
	            id : 0,
	            parent : None,
	            left : None,
	            right : None,
	            matrix : symmetries[0],
	            bounding_box : BoundingBox::new([T::zero(); D]),
	            tail : [T::zero(); D]
	        }
		; 2*n-1];

		for i in 0..arena.len() {arena[i].id = i;}

		let mut queue : Vec<(i32, i32, i32, bool)> = vec![(-1,0,(n-2) as i32, true)];
		while queue.len() > 0 {
			let (i,a,b,right) = queue.pop().unwrap();
			if b < a {
				continue;
			}
			let c = (a+b)/2;
			if i != -1 {
				arena[c as usize].parent = Some(i as usize);
				depth[c as usize] = depth[i as usize]+1 as usize;
				if right {
					arena[i as usize].right = Some(c as usize);
				}
				else {
					arena[i as usize].left = Some(c as usize);
				}
				
			}
			queue.push((c,c+1,b,true));
			queue.push((c,a,c-1,false));
		}

		let mut root = 0;
		for i in 0..(n-1) {
			if arena[i].root() {
				root = i;
			}
		}

		let mut stack = vec![root];
		let mut k = n-1;
		while stack.len() > 0 {
			let i = stack.pop().unwrap();
			match arena[i].left {
				None => {
					arena[i].left = Some(k);
					arena[k].parent = Some(i);
					depth[k as usize] = depth[i as usize]+1 as usize;
					k+=1;
				},
				_ => {}
			}
			match arena[i].right {
				None => {
					arena[i].right = Some(k);
					arena[k].parent = Some(i);
					depth[k as usize] = depth[i as usize]+1 as usize;
					k+=1;
				},
				_ => {}
			}
			match arena[i].right {
				Some(x) => {
					if x < n-1 {stack.push(x)}
				}
				_ => {}
			}
			match arena[i].left {
				Some(x) => {
					if x < n-1 {stack.push(x)}
				}
				_ => {}
			}
		}

		for i in (n-1)..(2*n-1) {
			arena[i].bounding_box = BoundingBox::new(walk[i+1-n]);
			arena[i].tail = walk[i+1-n];
		}

		let mut tree = SAWTree::new(arena, symmetries, rng);

		let mut max_depth = depth.iter().fold(0, |acc, x| max(*x,acc)) as i32;
		while max_depth > -1 {
			for i in 0..tree.arena.len() {
				if depth[i] as i32 == max_depth {
					tree.update_bounds(i);
				}
			}
			max_depth-=1;
		}

		tree

	}


	pub fn get_walk(&self) -> Vec<Vector<T,D>> {
		self.leaves.iter().map(|i| self.arena[*i].bounding_box.map(|x|x[0])).collect()
	}
}

impl<T : ArithmeticOps, const D : usize, const N : usize> SAWTree<T,D,N> {
	pub fn update_bounds(&mut self, target : usize) {
		match (self.arena[target].left, self.arena[target].right) {
			(None,None) => {},
			(Some(x), None) => {self.arena[target].bounding_box = self.arena[x].bounding_box;},
			(None, Some(y)) => {self.arena[target].bounding_box = self.arena[y].bounding_box;},
			(Some(left), Some(right)) => {
				println!("{:?}", target);
				println!("{:?} {:?}", left, right);
				println!("{:?}", self.arena[left].tail);
				println!("{:?}", self.arena[right].tail);
				let r = self.arena[right].bounding_box.transform(
						self.arena[target].matrix, 
						self.arena[left].tail
					);
				self.arena[target].bounding_box = self.arena[left].bounding_box.union(&r);
				

				self.arena[target].tail = 
					transform(
						self.arena[right].tail, 
						self.arena[target].matrix, 
						self.arena[left].tail
					)
				;
				println!("{:?}", transform(
						self.arena[right].tail, 
						self.arena[target].matrix, 
						self.arena[left].tail
					));
				println!("{:?}\n\n", self.arena[target].tail);
			},
		}
	}

	pub fn lr(&mut self, target : usize) {
		if target < self.arena.len() {
			let q1 = self.arena[target];
			if q1.right != None {
				let right = q1.right.unwrap();
				let q2 = self.arena[right];
				self.arena[target].right = q2.left;
				self.arena[target].parent = Some(q2.id);
				self.arena[right].parent = q1.parent;
				match q1.parent {
					None => {},
					Some(x) => {
						if self.arena[x].left == Some(q1.id) {
							self.arena[x].left = Some(q2.id);
						}
						else {
							self.arena[x].right = Some(q2.id);
						}
					},
				}
				self.arena[right].left = Some(q1.id);
				self.arena[right].matrix = dot(q1.matrix, q2.matrix);
				if self.root == q1.id {
					self.root = q2.id;
				}
			}
		}
	}

	pub fn rr(&mut self, target : usize) {
		if target < self.arena.len() {
			let q1 = self.arena[target];
			if q1.left != None {
				let left = q1.left.unwrap();
				let q2 = self.arena[left];
				self.arena[target].left = q2.right;
				self.arena[target].parent = Some(q2.id);
				self.arena[left].right = Some(q1.id);
				self.arena[left].parent = q1.parent;
				match q1.parent {
					None => {},
					Some(x) => {
						if self.arena[x].left == Some(q1.id) {
							self.arena[x].left = Some(q2.id);
						}
						else {
							self.arena[x].right = Some(q2.id);
						}
					},
				}
				self.arena[target].matrix = dot(transpose(q1.matrix), q2.matrix);
			}
		}
	}
}