use saw::boundingbox::NewBoundingBox;
use saw::saw_tree::Node;
use saw::symmetry_group::DIHEDRAL4;
use saw::boundingbox::BoundingBox;
use saw::saw_tree::SAWTree;
use saw::math::ArithmeticOps;

fn infix<T : ArithmeticOps, const D : usize, const N : usize>(tree : &SAWTree<T,N,D>) -> Vec<usize> {
    infix_aux(tree, tree.root)
}

fn infix_aux<T : ArithmeticOps, const D : usize, const N : usize>(tree : &SAWTree<T,N,D>, node : usize) -> Vec<usize> {
    let mut ret = vec![node];
    match tree.arena[node].left {
        None => {},
        Some(x) => {ret.append(&mut infix_aux(tree, x));},
    }
    match tree.arena[node].right {
        None => {},
        Some(x) => {ret.append(&mut infix_aux(tree, x));},
    }
    ret
}

fn example_tree() -> SAWTree<i32, 2, 8> {
    let arena = vec![
        Node {
            id : 0,
            parent : None,
            left : Some(1),
            right : Some(7),
            matrix : DIHEDRAL4[1],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 1,
            parent : Some(0),
            left : Some(2),
            right : Some(5),
            matrix : DIHEDRAL4[2],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 2,
            parent : Some(1),
            left : Some(3),
            right : Some(4),
            matrix : DIHEDRAL4[3],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 3,
            parent : Some(2),
            left : None,
            right : None,
            matrix : DIHEDRAL4[0],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 4,
            parent : Some(2),
            left : None,
            right : None,
            matrix : DIHEDRAL4[0],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 5,
            parent : Some(1),
            left : Some(6),
            right : None,
            matrix : DIHEDRAL4[7],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 6,
            parent : Some(5),
            left : None,
            right : None,
            matrix : DIHEDRAL4[0],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 7,
            parent : Some(0),
            left : None,
            right : Some(8),
            matrix : DIHEDRAL4[6],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 8,
            parent : Some(7),
            left : None,
            right : Some(9),
            matrix : DIHEDRAL4[4],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
        Node {
            id : 9,
            parent : Some(8),
            left : None,
            right : None,
            matrix : DIHEDRAL4[0],
            bounding_box : BoundingBox::new([1,2]),
            tail : [0,0]
        },
    ];
    let rng = rand::rng();
    SAWTree::new(arena, DIHEDRAL4, rng)
}

#[test]
fn new() {
    let tree = example_tree();
    let l = infix(&tree);
    assert_eq!(l, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn lr_1() {
    let mut tree = example_tree();
    tree.lr(3);
    let l = infix(&tree);
    assert_eq!(l, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn lr_2() {
    let mut tree = example_tree();
    tree.lr(8);
    let l = infix(&tree);
    assert_eq!(l, vec![0,1,2,3,4,5,6,7,9,8]);
    assert_eq!(tree.arena[9].left, Some(8));
}

#[test]
fn lr_3() {
    let mut tree = example_tree();
    tree.lr(5);
    let l = infix(&tree);
    assert_eq!(l, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn lr_4() {
    let mut tree = example_tree();
    tree.lr(1);
    let l = infix(&tree);
    assert_eq!(l, vec![0,5,1,2,3,4,6,7,8,9]);
    assert_eq!(tree.arena[5].right, None);
    assert_eq!(tree.arena[5].matrix, DIHEDRAL4[5]);
}


#[test]
fn rr_1() {
    let mut tree = example_tree();
    tree.rr(3);
    let l = infix(&tree);
    assert_eq!(l, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn rr_2() {
    let mut tree = example_tree();
    tree.rr(8);
    let l = infix(&tree);
    assert_eq!(l, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn rr_3() {
    let mut tree = example_tree();
    tree.rr(5);
    let l = infix(&tree);
    assert_eq!(l, vec![0,1,2,3,4,6,5,7,8,9]);
    assert_eq!(tree.arena[6].right, Some(5));
}

#[test]
fn rr_4() {
    let mut tree = example_tree();
    tree.rr(1);
    let l = infix(&tree);
    assert_eq!(l, vec![0,2,3,1,4,5,6,7,8,9]);
    assert_eq!(tree.arena[1].left, Some(4));
    assert_eq!(tree.arena[1].matrix, DIHEDRAL4[1]);
}


#[test]
fn from_walk() {
    let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2]];
    let rng = rand::rng();
    let tree = SAWTree::from_walk(&walk, DIHEDRAL4, rng);    
    let l = infix(&tree);
    assert_eq!(tree.get_walk(), walk);
    assert_eq!(tree.arena[l[2]].leaf(), true);
    assert_eq!(tree.arena[l[4]].leaf(), true);
    assert_eq!(tree.arena[l[5]].leaf(), true);
    assert_eq!(tree.arena[l[8]].leaf(), true);
    assert_eq!(tree.arena[l[9]].leaf(), true);
    assert_eq!(tree.arena[l[11]].leaf(), true);
    assert_eq!(tree.arena[l[12]].leaf(), true);
    assert_eq!(tree.arena[tree.root].bounding_box, [[0,2],[0,3]]);
}


#[test]
fn from_walk_bound() {
    let walk = vec![[0,0], [0,1], [0,2], [1,2], [1,3], [2,3], [2,2]];
    let rng = rand::rng();
    let mut tree = SAWTree::from_walk(&walk, DIHEDRAL4, rng);    
    let l = infix(&tree);
    for i in l.iter() {
        if true || tree.arena[*i].leaf() {
            println!("{:?}", tree.arena[*i]);
        }
    }
    assert_eq!(tree.get_walk(), vec![[0,0], [0,1], [0,2], [0,3], [0,4], [-1,4], [-1,3]]);
    assert_eq!(tree.arena[l[2]].leaf(), true);
    assert_eq!(tree.arena[l[4]].leaf(), true);
    assert_eq!(tree.arena[l[5]].leaf(), true);
    assert_eq!(tree.arena[l[8]].leaf(), true);
    assert_eq!(tree.arena[l[9]].leaf(), true);
    assert_eq!(tree.arena[l[11]].leaf(), true);
    assert_eq!(tree.arena[l[12]].leaf(), true);
    tree.arena[tree.root].matrix = DIHEDRAL4[2];
    tree.update_bounds(tree.root);
    assert_eq!(tree.arena[tree.root].bounding_box, [[-1,0],[0,4]]);
}