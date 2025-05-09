use criterion::{criterion_group, criterion_main, Criterion};
use rust_playground::simple_approach::sma::simple_moving_average;
use rust_playground::unsafe_approach::sma::simple_moving_average_pointer;

fn benchmark_simple_sma(c: &mut Criterion) {

    // generate a vector
    // for testing purposes
    // assumption: The benchmarking does NOT
    // include the creation of the numeric vector.
    let numeric_vector: Vec<f64> = vec![0.5; 1000];

    // NOTE: If the values are passed
    // by reference no iteration claims ownership
    // if not passed by reference, it can only be used once,
    // because of ownership stuff. I think.

    c.bench_function("benchmark_simple_sma", |b| {
        b.iter(|| {
            std::hint::black_box(simple_moving_average(&numeric_vector, 2));
        });
    });
}


fn benchmark_pointer_sma(c: &mut Criterion) {

    // generate a vector
    // for testing purposes
    // assumption: The benchmarking does NOT
    // include the creation of the numeric vector.
    let numeric_vector: Vec<f64> = vec![0.5; 1000];

    // NOTE: If the values are passed
    // by reference no iteration claims ownership
    // if not passed by reference, it can only be used once,
    // because of ownership stuff. I think.
    c.bench_function("benchmark_pointer_sma", |b| {
        b.iter(|| {
            std::hint::black_box(simple_moving_average_pointer(&numeric_vector, 2));
        });
    });
}

criterion_group!(
    benches,
    benchmark_simple_sma,
    benchmark_pointer_sma
);

criterion_main!(benches);