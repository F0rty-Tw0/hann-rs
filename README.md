# hann-rs

# BENCHMARK **APPROXIMATE** RESULTS

|Metric  | Size | Minimum Time  | Average Time  | Maximum Time  |
:-------:|-----:|------------------:|------------------:|------------------:|
get_hann_window | 2000 WL  | 7.0252 (µs) | 7.0657 (µs) | 7.1153 (µs) |
get_hann_window | 4000 WL  | 13.496 (µs) | 13.596 (µs)| 13.708 (µs) |
get_hann_window (Cached) | 4096 WL  | 363.84 (ns) | 369.98 (ns) | 377.30 (ns) | 


|Metric  | Size | Minimum Time  | Average Time  | Maximum Time  |
:-------:|-----:|------------------:|------------------:|------------------:|
get_hann_window_sum_squares | 2000 HW |  601.14 (ns) | 603.21 (ns) | 605.71 (ns) |
get_hann_window_sum_squares | 4000 HW |  1.1465 (µs)| 1.1520 (µs) | 1.1588 (µs) |
get_hann_window_sum_squares (Cached) | 4096 HW  | 10.583 (ns) | 10.628 (ns) | 10.680 (ns) |
