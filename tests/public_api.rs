use hann_rs::{
    HannMode, hann_energy_f32, hann_energy_f64, hann_f32, hann_f64, hann_in_place_f32,
    hann_in_place_f64,
};

fn assert_close_f32(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 1.0e-6,
        "expected {expected}, got {actual}"
    );
}

fn assert_close_f64(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn allocating_f32_supports_empty_and_unity_windows() {
    assert!(hann_f32(0, HannMode::Symmetric).is_empty());
    assert_eq!(hann_f32(1, HannMode::Periodic), vec![1.0]);
}

#[test]
fn allocating_f64_supports_empty_and_unity_windows() {
    assert!(hann_f64(0, HannMode::Periodic).is_empty());
    assert_eq!(hann_f64(1, HannMode::Symmetric), vec![1.0]);
}

#[test]
fn symmetric_mode_uses_n_minus_one_denominator() {
    assert_eq!(
        hann_f32(5, HannMode::Symmetric),
        vec![0.0, 0.5, 1.0, 0.5, 0.0]
    );
    let window = hann_f64(4, HannMode::Symmetric);
    for (actual, expected) in window.into_iter().zip([0.0, 0.75, 0.75, 0.0]) {
        assert_close_f64(actual, expected);
    }
}

#[test]
fn periodic_mode_uses_n_denominator() {
    assert_eq!(hann_f32(4, HannMode::Periodic), vec![0.0, 0.5, 1.0, 0.5]);
    let window = hann_f64(4, HannMode::Periodic);
    for (actual, expected) in window.into_iter().zip([0.0, 0.5, 1.0, 0.5]) {
        assert_close_f64(actual, expected);
    }
}

#[test]
fn in_place_f32_overwrites_the_entire_buffer() {
    let mut window = [42.0; 5];
    hann_in_place_f32(&mut window, HannMode::Symmetric);
    assert_eq!(window, [0.0, 0.5, 1.0, 0.5, 0.0]);
}

#[test]
fn in_place_f64_overwrites_the_entire_buffer() {
    let mut window = [42.0; 4];
    hann_in_place_f64(&mut window, HannMode::Periodic);
    for (actual, expected) in window.into_iter().zip([0.0, 0.5, 1.0, 0.5]) {
        assert_close_f64(actual, expected);
    }
}

#[test]
fn symmetric_samples_are_exact_mirrors_for_even_and_odd_lengths() {
    for length in [6, 7] {
        let window_f32 = hann_f32(length, HannMode::Symmetric);
        let window_f64 = hann_f64(length, HannMode::Symmetric);
        for index in 0..length {
            assert_eq!(window_f32[index], window_f32[length - 1 - index]);
            assert_eq!(window_f64[index], window_f64[length - 1 - index]);
        }
    }
}

#[test]
fn periodic_window_does_not_duplicate_the_zero_endpoint() {
    let symmetric = hann_f64(8, HannMode::Symmetric);
    let periodic = hann_f64(8, HannMode::Periodic);
    assert_eq!(symmetric[7], 0.0);
    assert!(periodic[7] > 0.0);
}

#[test]
fn f32_energy_has_exact_small_length_values() {
    for (mode, expected) in [
        (HannMode::Symmetric, [0.0, 1.0, 0.0, 1.0]),
        (HannMode::Periodic, [0.0, 1.0, 1.0, 1.125]),
    ] {
        for (length, expected) in expected.into_iter().enumerate() {
            assert_eq!(hann_energy_f32(length, mode), expected);
        }
    }
}

#[test]
fn f64_energy_has_exact_small_length_values() {
    for (mode, expected) in [
        (HannMode::Symmetric, [0.0, 1.0, 0.0, 1.0]),
        (HannMode::Periodic, [0.0, 1.0, 1.0, 1.125]),
    ] {
        for (length, expected) in expected.into_iter().enumerate() {
            assert_eq!(hann_energy_f64(length, mode), expected);
        }
    }
}

#[test]
fn f32_energy_matches_generated_even_and_odd_windows() {
    for mode in [HannMode::Symmetric, HannMode::Periodic] {
        for length in [4, 5, 16, 17] {
            let generated: f32 = hann_f32(length, mode)
                .iter()
                .map(|sample| sample * sample)
                .sum();
            assert_close_f32(hann_energy_f32(length, mode), generated);
        }
    }
}

#[test]
fn f64_energy_matches_generated_even_and_odd_windows() {
    for mode in [HannMode::Symmetric, HannMode::Periodic] {
        for length in [4, 5, 16, 17] {
            let generated: f64 = hann_f64(length, mode)
                .iter()
                .map(|sample| sample * sample)
                .sum();
            assert_close_f64(hann_energy_f64(length, mode), generated);
        }
    }
}
