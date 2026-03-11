use pivot_saw::lattice::Tetrahedral;
use tqdm::tqdm;
use pivot_saw::lattice::Lattice;
use pivot_saw::lattice::BaseLattice;

fn to_f64_coords(sol : &[[i32;3]], scale : f64) -> Vec<[f64;3]> {
    sol.iter().map(|p| p.map(|x| (x as f64)*scale)).collect()
}

// Calculate the displacement of a walk
fn displacement(walk : Vec<[i32;3]>) -> f64 {
    (walk[walk.len() -1][0] as f64).powf(2.) + (walk[walk.len() -1][0] as f64).powf(2.)
}

fn main() {
    let lat = Tetrahedral::new(1); // A body centered cubic grid of arg length 1.

    // The pivot algorithm for SAW of size 1000.
    // The algorithm will warmup with 20*1000 = 20000 accepted pivots during initialization.
    // Each time next() is called, 10*acceptation_rate pivots are attempted.
    let mut pivot = lat.get_pivot(10, rand::rng(), 10, 10); 

    let mean : Vec<Vec<[f64;3]>> = tqdm(0..10000) // for 10000 iterations
        .map(|_|to_f64_coords(&pivot.next().unwrap(), 3.8)).collect();

    println!("{:?}", mean);
}