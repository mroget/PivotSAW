use saw::boundingbox::New4d;
use saw::boundingbox::BoundingBox4;
use saw::boundingbox::New3d;
use saw::boundingbox::BoundingBox3;
use saw::boundingbox::BBox;
use saw::boundingbox::New2d;
use saw::boundingbox::BoundingBox2;

// 2D
#[test]
fn d2_union_simple() {
    let pts = [[0.,1.],[4.,5.]];
    let mut result = BoundingBox2::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox2::new(pts[i]));
    }
    assert_eq!(result, [[0.,4.],[1.,5.]]);
}

#[test]
fn d2_union_multiple() {
    let pts = [[0,1],[1,1],[1,2],[0,2],[1,3]];
    let mut result = BoundingBox2::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox2::new(pts[i]));
    }
    assert_eq!(result, [[0,1],[1,3]]);
}


#[test]
fn d2_intersection() {
    let pts1 = [[0,1],[1,1],[1,2],[0,2],[1,3]];
    let mut box1 = BoundingBox2::new(pts1[0]);
    for i in 1..pts1.len() {
        box1 = box1.union(&BoundingBox2::new(pts1[i]));
    }

    let pts2 = [[0,1],[1,1]];
    let mut box2 = BoundingBox2::new(pts2[0]);
    for i in 1..pts2.len() {
        box2 = box2.union(&BoundingBox2::new(pts2[i]));
    }

    let result = box1.intersection(&box2);
    assert_eq!(result, [[0,1],[1,1]]);
}

#[test]
fn d2_empty_1() {
    let pts = [[0,1],[1,1],[1,2],[0,2],[1,3]];
    let mut result = BoundingBox2::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox2::new(pts[i]));
    }
    assert_eq!(result.is_empty(), false);
}

#[test]
fn d2_empty_2() {
    let pts = [[1,1],[1,2],[1,3]];
    let mut result = BoundingBox2::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox2::new(pts[i]));
    }
    assert_eq!(result.is_empty(), false);
}

#[test]
fn d2_empty_3() {
    let pts1 = [[0,1],[1,1],[1,2],[0,2],[1,3]];
    let mut box1 = BoundingBox2::new(pts1[0]);
    for i in 1..pts1.len() {
        box1 = box1.union(&BoundingBox2::new(pts1[i]));
    }

    let pts2 = [[5,6],[6,6]];
    let mut box2 = BoundingBox2::new(pts2[0]);
    for i in 1..pts2.len() {
        box2 = box2.union(&BoundingBox2::new(pts2[i]));
    }

    let result = box1.intersection(&box2);
    assert_eq!(result.is_empty(), true);
}







// 3D

#[test]
fn d3_union_simple() {
    let pts = [[0.,1.,-2.],[4.,5., 3.]];
    let mut result = BoundingBox3::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox3::new(pts[i]));
    }
    assert_eq!(result, [[0.,4.],[1.,5.],[-2.,3.]]);
}

#[test]
fn d3_union_multiple() {
    let pts = [[0,1,0],[1,1,0],[1,2,1],[0,2,1],[1,3,-1]];
    let mut result = BoundingBox3::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox3::new(pts[i]));
    }
    assert_eq!(result, [[0,1],[1,3],[-1,1]]);
}


#[test]
fn d3_intersection() {
    let pts1 = [[0,1,0],[1,1,0],[1,2,1],[0,2,1],[1,3,-1]];
    let mut box1 = BoundingBox3::new(pts1[0]);
    for i in 1..pts1.len() {
        box1 = box1.union(&BoundingBox3::new(pts1[i]));
    }

    let pts2 = [[0,1,-2],[4,2, 0]];
    let mut box2 = BoundingBox3::new(pts2[0]);
    for i in 1..pts2.len() {
        box2 = box2.union(&BoundingBox3::new(pts2[i]));
    }

    let result = box1.intersection(&box2);
    assert_eq!(result, [[0,1],[1,2],[-1,0]]);
}

#[test]
fn d3_empty_1() {
    let pts = [[0,1,0],[1,1,0],[1,2,1],[0,2,1],[1,3,-1]];
    let mut result = BoundingBox3::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox3::new(pts[i]));
    }
    assert_eq!(result.is_empty(), false);
}

#[test]
fn d3_empty_2() {
    let pts = [[1,1,0],[1,2,0],[1,3,0]];
    let mut result = BoundingBox3::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox3::new(pts[i]));
    }
    assert_eq!(result.is_empty(), false);
}

#[test]
fn d3_empty_3() {
    let pts1 = [[0,1,0],[1,1,0],[1,2,1],[0,2,1],[1,3,-1]];
    let mut box1 = BoundingBox3::new(pts1[0]);
    for i in 1..pts1.len() {
        box1 = box1.union(&BoundingBox3::new(pts1[i]));
    }

    let pts2 = [[5,6,7],[6,6,8]];
    let mut box2 = BoundingBox3::new(pts2[0]);
    for i in 1..pts2.len() {
        box2 = box2.union(&BoundingBox3::new(pts2[i]));
    }

    let result = box1.intersection(&box2);
    assert_eq!(result.is_empty(), true);
}







// 4D

#[test]
fn d4_union_simple() {
    let pts = [[0.,1.,-2.,-3.],[4.,5., 3.,-4.]];
    let mut result = BoundingBox4::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox4::new(pts[i]));
    }
    assert_eq!(result, [[0.,4.],[1.,5.],[-2.,3.],[-4.,-3.]]);
}

#[test]
fn d4_union_multiple() {
    let pts = [[0,1,0,0],[1,1,0,4],[1,2,1,2],[0,2,1,3],[1,3,-1,0]];
    let mut result = BoundingBox4::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox4::new(pts[i]));
    }
    assert_eq!(result, [[0,1],[1,3],[-1,1],[0,4]]);
}


#[test]
fn d4_intersection() {
    let pts1 = [[0,1,0,0],[1,1,0,4],[1,2,1,2],[0,2,1,3],[1,3,-1,0]];
    let mut box1 = BoundingBox4::new(pts1[0]);
    for i in 1..pts1.len() {
        box1 = box1.union(&BoundingBox4::new(pts1[i]));
    }

    let pts2 = [[0,1,-2,2],[4,2,0,5]];
    let mut box2 = BoundingBox4::new(pts2[0]);
    for i in 1..pts2.len() {
        box2 = box2.union(&BoundingBox4::new(pts2[i]));
    }

    let result = box1.intersection(&box2);
    assert_eq!(result, [[0,1],[1,2],[-1,0],[2,4]]);
}

#[test]
fn d4_empty_1() {
    let pts = [[0,1,0,0],[1,1,0,4],[1,2,1,2],[0,2,1,3],[1,3,-1,0]];
    let mut result = BoundingBox4::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox4::new(pts[i]));
    }
    assert_eq!(result.is_empty(), false);
}

#[test]
fn d4_empty_2() {
    let pts = [[1,1,0,1],[1,2,0,1],[1,3,0,1]];
    let mut result = BoundingBox4::new(pts[0]);
    for i in 1..pts.len() {
        result = result.union(&BoundingBox4::new(pts[i]));
    }
    assert_eq!(result.is_empty(), false);
}

#[test]
fn d4_empty_3() {
    let pts1 = [[0,1,0,0],[1,1,0,4],[1,2,1,2],[0,2,1,3],[1,3,-1,0]];
    let mut box1 = BoundingBox4::new(pts1[0]);
    for i in 1..pts1.len() {
        box1 = box1.union(&BoundingBox4::new(pts1[i]));
    }

    let pts2 = [[5,6,7,10],[6,6,8,9]];
    let mut box2 = BoundingBox4::new(pts2[0]);
    for i in 1..pts2.len() {
        box2 = box2.union(&BoundingBox4::new(pts2[i]));
    }

    let result = box1.intersection(&box2);
    assert_eq!(result.is_empty(), true);
}