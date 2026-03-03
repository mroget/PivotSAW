use saw::boundingbox::NewBoundingBox;
use saw::boundingbox::BoundingBox;
use criterion::{black_box, criterion_group, criterion_main, Criterion};



pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new_basic", |b| b.iter(|| {
        BoundingBox::new(black_box([2, 1]))
    }));

}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);