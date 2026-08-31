# 0.2.0 - 2026-08-31

Breaking modernization release.

## Added

- `HannMode::{Symmetric, Periodic}`.
- Explicit in-place, allocating, and energy APIs for `f32` and `f64`.
- Caller-owned `HannCacheF32` and `HannCacheF64` for repeated window requests.
- Reproducible SVG plot generator backed by the public `hann_f64` API.
- Defined empty and one-element window behavior.

## Changed

- In-place generation performs no allocation.
- Energy is calculated from closed forms without global state.
- Minimum supported Rust version is 1.98; the crate uses Edition 2024.
- Benchmarks now separate generation, borrowed cache hits, owned clones, and the legacy workload.

## Removed

- `lazy_static`, process-wide lookup tables, and the runtime dependency.
- The 0.1 function names and `HannWindowError`.

## Migration

| 0.1 API | 0.2 replacement |
| --- | --- |
| `get_hann_window(length)` | `hann_f32(length, HannMode::Symmetric)` |
| `get_hann_window_sum_squares(&window)` | `hann_energy_f32(length, mode)` for an unmodified generated Hann window |
| `HannWindowError` | Removed; lengths 0 and 1 now have defined results |
| Window and energy lookup globals | Removed; generation is stateless |

`hann_energy_f32` and `hann_energy_f64` use only length and mode. For arbitrary
or modified coefficients, sum the samples directly:
`window.iter().map(|sample| sample * sample).sum::<f32>()`.

The new allocating functions return `Vec` directly rather than `Result`.

Version 0.2 removes the former `2^24` length cap and allocation-error variants.
Callers must validate untrusted lengths before `hann_f32`, `hann_f64`, or cache
`get`; impossible or failed allocations may panic or abort.

# 0.1.0

* Initial release.