use criterion::{criterion_group, criterion_main, Criterion};
use rust_playground::simple_approach::sma::simple_moving_average;

fn benc_simple_moving_average(c: &mut Criterion) {

    // generate a vector
    // for testing purposes
    // assumption: The benchmarking does NOT
    // include the creation of the numeric vector.
    let numeric_vector: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];


    c.bench_function("benc_simple_moving_average", |b| {
        b.iter(|| {
            std::hint::black_box(for i in 1..=100 {
                simple_moving_average(numeric_vector.clone(), 2);
            });
        });
    });
}

criterion_group!(
    benches,
    benc_simple_moving_average,
);

criterion_main!(benches);