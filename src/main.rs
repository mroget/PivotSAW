use tqdm::tqdm;
use saw::lattice::Lattice;
use saw::lattice::BaseLattice;

// Calculate the displacement of a walk
fn displacement(walk : Vec<[i32;3]>) -> f64 {
    (walk[walk.len() -1][0] as f64).powf(2.) + (walk[walk.len() -1][0] as f64).powf(2.)
}

fn main() {
    let lat = BaseLattice::bcc(1); // A body centered cubic grid of arg length 1.

    // The pivot algorithm for SAW of size 1000.
    // The algorithm will warmup with 20*1000 = 20000 accepted pivots during initialization.
    // Each time next() is called, 10 accepted pivots are calculated.
    let mut pivot = lat.get_pivot(1000, rand::rng(), 20, 10); 

    let mean = tqdm(0..10000) // for 10000 iterations
        .map(|_|displacement(pivot.next().unwrap())) // get the next SAW from the pivot and calculated the displacement
        .fold(0.,|acc, x|acc+x) / (10000.); // calculate the mean.

    println!("{}", mean);
}