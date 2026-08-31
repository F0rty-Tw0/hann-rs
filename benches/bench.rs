use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use hann_rs::{
    HannCacheF32, HannCacheF64, HannMode, hann_f32, hann_f64, hann_in_place_f32, hann_in_place_f64,
};

const WINDOW_LENGTH: usize = 4096;

fn benchmark_hann_f32(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("hann_f32");
    let mut window = vec![0.0; WINDOW_LENGTH];

    group.bench_function("in_place", |bencher| {
        bencher.iter(|| {
            hann_in_place_f32(black_box(window.as_mut_slice()), HannMode::Periodic);
            black_box(window.as_slice());
        });
    });
    group.bench_function("allocating", |bencher| {
        bencher.iter(|| black_box(hann_f32(black_box(WINDOW_LENGTH), HannMode::Periodic)));
    });
    let mut cache = HannCacheF32::new();
    cache.get(WINDOW_LENGTH, HannMode::Periodic);
    group.bench_function("cached_hit", |bencher| {
        bencher.iter(|| {
            let window = cache.get(black_box(WINDOW_LENGTH), HannMode::Periodic);
            black_box(window.as_ptr());
        });
    });
    group.finish();
}

fn benchmark_hann_f64(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("hann_f64");
    let mut window = vec![0.0; WINDOW_LENGTH];

    group.bench_function("in_place", |bencher| {
        bencher.iter(|| {
            hann_in_place_f64(black_box(window.as_mut_slice()), HannMode::Periodic);
            black_box(window.as_slice());
        });
    });
    group.bench_function("allocating", |bencher| {
        bencher.iter(|| black_box(hann_f64(black_box(WINDOW_LENGTH), HannMode::Periodic)));
    });
    let mut cache = HannCacheF64::new();
    cache.get(WINDOW_LENGTH, HannMode::Periodic);
    group.bench_function("cached_hit", |bencher| {
        bencher.iter(|| {
            let window = cache.get(black_box(WINDOW_LENGTH), HannMode::Periodic);
            black_box(window.as_ptr());
        });
    });
    group.finish();
}

fn benchmark_legacy_comparison(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("legacy_comparison");

    group.bench_function("uncached_symmetric", |bencher| {
        bencher.iter(|| {
            black_box(hann_f32(black_box(WINDOW_LENGTH), HannMode::Symmetric));
        });
    });
    let mut cache = HannCacheF32::new();
    cache.get(WINDOW_LENGTH, HannMode::Symmetric);
    group.bench_function("cached_symmetric_borrowed", |bencher| {
        bencher.iter(|| {
            let window = cache.get(black_box(WINDOW_LENGTH), HannMode::Symmetric);
            black_box(window.as_ptr());
        });
    });
    group.bench_function("cached_symmetric_owned_clone", |bencher| {
        bencher.iter(|| {
            let window = cache.get(black_box(WINDOW_LENGTH), HannMode::Symmetric);
            black_box(window.to_vec());
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    benchmark_hann_f32,
    benchmark_hann_f64,
    benchmark_legacy_comparison
);
criterion_main!(benches);
