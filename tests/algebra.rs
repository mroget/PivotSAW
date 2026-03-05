use saw::algebra::transpose;
use saw::algebra::IntoVector;
use saw::algebra::dot;
use saw::algebra::vec_len;
use saw::algebra::vec_square_len;
use saw::algebra::vec_sub;
use saw::algebra::vec_add;

#[test]
fn test_vec_add() {
    let a = [0,-2,5,7];
    let b = [4,2,-3,6];
    let result = vec_add(a,b);
    assert_eq!(result, [4, 0, 2, 13]);
}


#[test]
fn test_vec_sub() {
    let a = [0,-2,5,7];
    let b = [4,2,-3,6];
    let result = vec_sub(a,b);
    assert_eq!(result, [-4, -4, 8, 1]);
}

#[test]
fn test_vec_len_square() {
    let a = [0,3,4];
    let result = vec_square_len(a);
    assert_eq!(result, 25);
}

#[test]
fn test_vec_len() {
    let a = [0,3,4];
    let result = vec_len(a);
    assert_eq!(result, 5.);
}



#[test]
fn test_mat_mul() {
    let a = [[0,1],[1,2]];
    let b = [[0,2],[3,4]];
    let result = dot(a,b);
    assert_eq!(result, [[3,4],[6,10]]);
}

#[test]
fn test_mat_apply() {
    let a = [[4,1,3],[1,2,-1]];
    let b = [1,2,3];
    let result : [i32; 2] = dot(a,b).from_mat();
    assert_eq!(result, [15,2]);
}

#[test]
fn test_mat_apply_row() {
    let a = [[2,1,3]];
    let b = [[0,2],[3,4],[1,2]];
    let result : [i32;2] = dot(a,b).from_mat();
    assert_eq!(result, [6,14]);
}


#[test]
fn test_transpose_mat() {
    let a = [[0,1],[2,3]];
    let result = transpose(a);
    assert_eq!(result, [[0,2],[1,3]]);
}

#[test]
fn test_transpose_vec() {
    let a = [1,2,3];
    let result = transpose(a);
    assert_eq!(result, [[1,2,3]]);
}