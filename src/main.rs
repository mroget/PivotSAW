use nalgebra::Matrix2;
fn main() {
    let a = Matrix2::new(0,1,1,2);
    let b = Matrix2::new(0,2,3,4);  
    println!("{:?}",a+b);
}