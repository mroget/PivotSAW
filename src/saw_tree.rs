use crate::boundingbox::BoundingBox;

struct Node<T, const D : usize> {
	id : usize,
	parent : Option<usize>,
	left : Option<usize>,
	right : Option<usize>,
	matrix : [[T; D]; D],
	bounding_box : BoundingBox<T, D>,
}


struct SAWTree2<T, const D : usize> {
	arena : Vec<Node<T, D>>,
}