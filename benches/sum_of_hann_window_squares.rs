use criterion::{ black_box, Criterion };
use hann_rs::{ get_hann_window, get_hann_window_sum_squares };

pub fn bench_get_hann_window_sum_squares(criterion: &mut Criterion) {
  const WINDOW_LENGTH: usize = 4096;

  let hann_window = get_hann_window(WINDOW_LENGTH).expect(
    "Failed to get the Hann window from the lookup table"
  );

  criterion.bench_function("get_hann_window_sum_squares", |bencher| {
    bencher.iter(|| black_box(get_hann_window_sum_squares(&hann_window)));
  });
}