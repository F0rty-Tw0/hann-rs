#![doc = r#"
Hann windows for digital signal processing.

Use [`HannMode::Symmetric`] for filter design and [`HannMode::Periodic`] for
spectral analysis.

```
use hann_rs::{
    HannMode, hann_energy_f32, hann_energy_f64, hann_f32, hann_f64,
    hann_in_place_f32, hann_in_place_f64,
};

let mut buffer_f32 = [0.0; 4];
hann_in_place_f32(&mut buffer_f32, HannMode::Periodic);
assert_eq!(buffer_f32, [0.0, 0.5, 1.0, 0.5]);

let mut buffer_f64 = [0.0; 5];
hann_in_place_f64(&mut buffer_f64, HannMode::Symmetric);

let window_f32 = hann_f32(5, HannMode::Symmetric);
let window_f64 = hann_f64(4, HannMode::Periodic);
assert_eq!(hann_energy_f32(5, HannMode::Symmetric), 1.5);
assert_eq!(hann_energy_f64(4, HannMode::Periodic), 1.5);
assert_eq!(window_f32.len(), 5);
assert_eq!(window_f64.len(), 4);
```
"#]

use std::collections::HashMap;

/// Cache of generated `f32` Hann windows owned by the caller.
///
/// Each distinct `(length, mode)` key is retained until [`Self::clear`] or
/// drop. [`Self::get`] requires a mutable borrow and returns a shared slice, so
/// finish using one returned slice before requesting another window.
#[derive(Debug, Default)]
pub struct HannCacheF32 {
    windows: HashMap<(usize, HannMode), Vec<f32>>,
}

/// Cache of generated `f64` Hann windows owned by the caller.
///
/// Each distinct `(length, mode)` key is retained until [`Self::clear`] or
/// drop. [`Self::get`] requires a mutable borrow and returns a shared slice, so
/// finish using one returned slice before requesting another window.
#[derive(Debug, Default)]
pub struct HannCacheF64 {
    windows: HashMap<(usize, HannMode), Vec<f64>>,
}

macro_rules! impl_hann_cache {
    ($cache:ident, $float:ty, $generator:ident) => {
        impl $cache {
            /// Creates an empty cache.
            pub fn new() -> Self {
                Self::default()
            }

            /// Returns the retained window for `(length, mode)`, generating it
            /// on the first request.
            ///
            /// The returned slice borrows this cache. The cache retains each
            /// generated allocation until [`Self::clear`] or drop, and only
            /// one mutable cache borrow can be active at a time.
            ///
            /// # Panics
            ///
            /// On a cache miss, may panic if `length` causes an allocation-size
            /// calculation to overflow. Allocation failure may abort the process.
            /// Callers must validate untrusted lengths before calling this method.
            pub fn get(&mut self, length: usize, mode: HannMode) -> &[$float] {
                self.windows
                    .entry((length, mode))
                    .or_insert_with(|| $generator(length, mode))
                    .as_slice()
            }

            /// Returns the number of distinct retained `(length, mode)` keys.
            pub fn len(&self) -> usize {
                self.windows.len()
            }

            /// Returns whether this cache retains no windows.
            pub fn is_empty(&self) -> bool {
                self.windows.is_empty()
            }

            /// Drops all retained windows and resets this cache to empty.
            pub fn clear(&mut self) {
                self.windows.clear();
            }
        }
    };
}

impl_hann_cache!(HannCacheF32, f32, hann_f32);
impl_hann_cache!(HannCacheF64, f64, hann_f64);

/// Selects the denominator used to generate a Hann window.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HannMode {
    /// Uses `N - 1`, producing equal zero-valued endpoints.
    Symmetric,
    /// Uses `N`, producing one period without duplicating the zero endpoint.
    Periodic,
}

macro_rules! impl_hann {
    ($float:ty, $in_place:ident, $allocating:ident, $energy:ident, $tau:expr) => {
        /// Overwrites a buffer with Hann coefficients without allocating.
        ///
        /// Empty buffers are unchanged. A one-element buffer becomes `[1.0]`.
        pub fn $in_place(window: &mut [$float], mode: HannMode) {
            let length = window.len();
            match length {
                0 => return,
                1 => {
                    window[0] = 1.0;
                    return;
                }
                _ => {}
            }

            let denominator = match mode {
                HannMode::Symmetric => (length - 1) as $float,
                HannMode::Periodic => length as $float,
            };
            let scale = $tau / denominator;

            match mode {
                HannMode::Symmetric => {
                    for index in 0..length.div_ceil(2) {
                        let sample = 0.5 - 0.5 * (scale * index as $float).cos();
                        window[index] = sample;
                        window[length - 1 - index] = sample;
                    }
                }
                HannMode::Periodic => {
                    for (index, sample) in window.iter_mut().enumerate() {
                        *sample = 0.5 - 0.5 * (scale * index as $float).cos();
                    }
                }
            }
        }

        /// Allocates and returns a Hann window.
        ///
        /// Length zero returns an empty vector; length one returns `[1.0]`.
        ///
        /// # Panics
        ///
        /// May panic if `length` causes an allocation-size calculation to
        /// overflow. Allocation failure may abort the process. Callers must
        /// validate untrusted lengths before calling this function.
        pub fn $allocating(length: usize, mode: HannMode) -> Vec<$float> {
            let mut window = vec![0.0; length];
            $in_place(&mut window, mode);
            window
        }

        /// Returns the closed-form sum of squared Hann coefficients.
        pub fn $energy(length: usize, mode: HannMode) -> $float {
            match mode {
                HannMode::Symmetric => match length {
                    0 => 0.0,
                    1 => 1.0,
                    2 => 0.0,
                    3 => 1.0,
                    _ => 3.0 * (length - 1) as $float / 8.0,
                },
                HannMode::Periodic => match length {
                    0 => 0.0,
                    1 | 2 => 1.0,
                    _ => 3.0 * length as $float / 8.0,
                },
            }
        }
    };
}

impl_hann!(
    f32,
    hann_in_place_f32,
    hann_f32,
    hann_energy_f32,
    std::f32::consts::TAU
);
impl_hann!(
    f64,
    hann_in_place_f64,
    hann_f64,
    hann_energy_f64,
    std::f64::consts::TAU
);
