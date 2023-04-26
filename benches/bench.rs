use criterion::{ criterion_main, criterion_group };

mod hann_window;
mod sum_of_hann_window_squares;

criterion_group!(
  benches,
  hann_window::bench_get_hann_window,
  sum_of_hann_window_squares::bench_get_hann_window_sum_squares
);

criterion_main!(benches);