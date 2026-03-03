use crate::boundingbox::BoundingBox2;

struct Node2<T> {
	id : usize,
	parent : Option<usize>,
	left : Option<usize>,
	right : Option<usize>,
	matrix : [[T; 2]; 2],
	bounding_box : BoundingBox2<T>,
}


struct SAWTree2<T> {
	arena : Vec<Node2<T>>,
}