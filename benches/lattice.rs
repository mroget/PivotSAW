use saw::lattice::dist_1000;
use saw::lattice::dist_100000;
use saw::lattice::dist_50;
use saw::lattice::Lattice2;
use saw::math::vec2_len;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use saw::lattice::Basic2dLattice;
use saw::lattice::SquareGrid;


pub fn dist_basic(length : usize) -> f64 {
    let mut rng = rand::rng();
    let lattice = Basic2dLattice::new(vec![[1,0],[0,1]], vec![]);
    let coords = lattice.random_walk(length, None, &mut rng);
    vec2_len(coords[coords.len()-1].map(|x| x as f64))
}

pub fn dist(length : usize) -> f64 {
    let mut rng = rand::rng();
    let lattice = SquareGrid::new(1);
    let coords = lattice.random_walk(length, None, &mut rng);
    vec2_len(coords[coords.len()-1].map(|x| x as f64))
}


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dist_basic_50", |b| b.iter(|| {
        dist_basic(black_box(50))
    }));

    c.bench_function("dist_50", |b| b.iter(|| {
        dist(black_box(50))
    }));

    c.bench_function("dist_efficient_50", |b| b.iter(|| {
        dist_50()
    }));




    c.bench_function("dist_basic_1000", |b| b.iter(|| {
        dist_basic(black_box(1000))
    }));

    c.bench_function("dist_1000", |b| b.iter(|| {
        dist(black_box(1000))
    }));

    c.bench_function("dist_efficient_1000", |b| b.iter(|| {
        dist_1000()
    }));




    c.bench_function("dist_basic_100000", |b| b.iter(|| {
        dist_basic(black_box(100000))
    }));

    c.bench_function("dist_100000", |b| b.iter(|| {
        dist(black_box(100000))
    }));

    c.bench_function("dist_efficient_100000", |b| b.iter(|| {
        dist_100000()
    }));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);