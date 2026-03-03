use saw::algebra::vec_len;
use saw::walk::random_walk;
use saw::lattice::dist_1000;
use saw::lattice::dist_100000;
use saw::lattice::dist_50;
use saw::lattice::Lattice;

use criterion::{black_box, criterion_group, criterion_main, Criterion};


pub fn dist_basic(length : usize) -> f64 {
    let mut rng = rand::rng();
    let lattice = Lattice::new(vec![[1,0],[0,1]], vec![]);
    let coords = random_walk(&lattice, length, None, &mut rng);
    vec_len(coords[coords.len()-1])
}

pub fn dist(length : usize) -> f64 {
    let mut rng = rand::rng();
    let lattice: Lattice<_, 2> = Lattice::square_grid(1);
    let coords = random_walk(&lattice, length, None, &mut rng);
    vec_len(coords[coords.len()-1])
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