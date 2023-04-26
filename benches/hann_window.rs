use criterion::{ black_box, Criterion };
use hann_rs::get_hann_window;

pub fn bench_get_hann_window(criterion: &mut Criterion) {
  const WINDOW_LENGTH: usize = 4096;

  criterion.bench_function("get_hann_window", |bencher| {
    bencher.iter(||
      black_box(
        get_hann_window(WINDOW_LENGTH).expect("Failed to get the Hann window from the lookup table")
      )
    );
  });
}