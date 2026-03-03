use saw::boundingbox::New2d;
use saw::boundingbox::BoundingBox2;
use saw::boundingbox::new_bounding_box_2;
use saw::boundingbox::new_bounding_box;
use saw::lattice::dist_1000;
use saw::lattice::dist_100000;
use saw::lattice::dist_50;
use criterion::{black_box, criterion_group, criterion_main, Criterion};



pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new_basic", |b| b.iter(|| {
        new_bounding_box(black_box([2, 1]))
    }));

    c.bench_function("new_map", |b| b.iter(|| {
        new_bounding_box_2(black_box([2, 1]))
    }));

    c.bench_function("new_struct", |b| b.iter(|| {
        BoundingBox2::new(black_box([2 as i32, 1 as i32]))
    }));

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);