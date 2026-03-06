use saw::pivot::Pivot;
use saw::lattice::BaseLattice;
use saw::lattice::Lattice;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn get_pivot_grid(len : usize) -> Pivot<i32,2,8> {
    let lat = BaseLattice::square_grid(1);
    let rng = rand::rng();
    let walk = lat.straight(len);
    let pivot = Pivot::new(walk, lat.symmetries, rng.clone(), 10, 10);
    pivot
}


pub fn criterion_benchmark(c: &mut Criterion) {
    let mut pivot = get_pivot_grid(10);
    
    c.bench_function("pivot10", |b| b.iter(|| {
        pivot.next();
    }));


    let mut pivot = get_pivot_grid(100);
    c.bench_function("pivot100", |b| b.iter(|| {
        pivot.next();
    }));

    let mut pivot = get_pivot_grid(200);
    c.bench_function("pivot200", |b| b.iter(|| {
        pivot.next();
    }));


    let mut pivot = get_pivot_grid(1000);
    c.bench_function("pivot1000", |b| b.iter(|| {
        pivot.next();
    }));

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);