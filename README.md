# PivotSAW
This crate provides an implementation of the pivot alorithm (see <https://en.wikipedia.org/wiki/Pivot_algorithm>) which generates random Salf Avoiding Walk (SAW) (see <https://en.wikipedia.org/wiki/Self-avoiding_walk#Further_reading>) in a pseudo uniform way.

## Overview of the algorithm
The Pivot algorithm works by applying a random symmetry (an operation that maps the lattice onto itself) on the tail of the walk, starting from a position at random. Doing this allows to create a new SAW from an existing one with remarkably high probability (compared to local methods). If the walk is still self avoiding after application of the pivot, we say that the pivot is accepted. If not, we reject the pivot and go back to the previous walk.

For the algorithm to provide a pseudo-uniform sampling of SAW (that can be used for estimating observables), it is important to define the thermalization time and autocorrelation time. The thermalization time is the number of accepted pivots that are accepted before the algorithm starts. This is a warmup that allows the algorithm to reach a random SAW from the straight line. In this package, it is defined as `size_of_the_walk` $\times$ `thermalization_factor`. The `thermalization_factor` is a parameter that you provide when initializing the Pivot algorithm. It is strongly advised to not set it to 0 and actually check the quality of the sampling regarding the observable you try to approximate. The autocorrelation time is the number of accepted pivots between each returned values. It descreases the correlation between successive SAW in the sampling. This is a value you must set when initializing the Pivot algorithm. A careful choice of this value is advised.

For more information about SAW (self avoiding walks), their uses and the Pivot algorithm, check this link: <https://clisby.net/projects/saw_feature/>.

## How it works
The pivot algorithm is an infinite Iterator which returns a new SAW at every iteration. You can create a new Pivot from a given lattice by specifying the length of the SAWs that will be generated, the autocorrelation time, the thermalization factor and a random seed (useful for reprucable results).

### Minimal example : Calculating average displacement in the body-centered cubic lattice
```rust
use tqdm::tqdm;
use pivot_saw::lattice::Lattice;
use pivot_saw::lattice::BaseLattice;

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
```

### Lattices
It is possible to define a `BaseLattice` (i.e. a lattice with a base) provided a base and a symmetry group.

It is also possible to define a personalized lattice (like a graph) trough the Lattice trait.

Lattice already implemented:
 - `BaseLattice::square_grid(a)` a square grid of arc length $a$.
 - `BaseLattice::cubic_grid(a)` a cubic grid of arc length $a$.
 - `BaseLattice::bcc(a)` a body centered cubic lattice of arc length $a$.
 - `BaseLattice::fcc(a)` a face centered cubic lattice of arc length $a$.
 - `Tetrahedral::new(a)` a tetrahedral lattice of arc length $a$.

