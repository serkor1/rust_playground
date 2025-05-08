use criterion::{criterion_group, criterion_main, Criterion};
use rust_playground::simple_approach::sma::simple_moving_average;

fn benchmark_simple_sma(c: &mut Criterion) {

    // generate a vector
    // for testing purposes
    // assumption: The benchmarking does NOT
    // include the creation of the numeric vector.
    let numeric_vector: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    // NOTE: If the values are passed
    // by reference no iteration claims ownership
    // if not passed by reference, it can only be used once,
    // because of ownership stuff. I think.

    c.bench_function("benchmark_simple_sma", |b| {
        b.iter(|| {
            std::hint::black_box(for i in 1..=100 {
                simple_moving_average(&numeric_vector, 2);
            });
        });
    });
}

criterion_group!(
    benches,
    benchmark_simple_sma,
);

criterion_main!(benches);