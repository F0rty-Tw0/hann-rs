use hann_rs::{HannCacheF32, HannCacheF64, HannMode};

fn assert_close_f64(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn f32_cache_new_and_default_start_empty() {
    assert!(HannCacheF32::new().is_empty());
    assert!(HannCacheF32::default().is_empty());
}

#[test]
fn f32_cache_generates_and_reuses_same_key() {
    let mut cache = HannCacheF32::new();

    let first = cache.get(5, HannMode::Symmetric);
    assert_eq!(first, &[0.0, 0.5, 1.0, 0.5, 0.0]);
    let first_pointer = first.as_ptr();

    let second_pointer = cache.get(5, HannMode::Symmetric).as_ptr();
    assert_eq!(first_pointer, second_pointer);
    assert_eq!(cache.len(), 1);
}

#[test]
fn f32_cache_keeps_distinct_length_and_mode_entries() {
    let mut cache = HannCacheF32::new();

    assert_eq!(cache.get(4, HannMode::Periodic), &[0.0, 0.5, 1.0, 0.5]);
    assert_eq!(cache.get(5, HannMode::Periodic).len(), 5);
    assert_eq!(cache.get(4, HannMode::Symmetric).len(), 4);
    assert_eq!(cache.len(), 3);
    assert!(!cache.is_empty());
}

#[test]
fn f32_cache_clear_releases_entries() {
    let mut cache = HannCacheF32::new();
    cache.get(4, HannMode::Periodic);
    cache.get(5, HannMode::Symmetric);
    assert_eq!(cache.len(), 2);

    cache.clear();

    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

#[test]
fn f64_cache_new_and_default_start_empty() {
    assert!(HannCacheF64::new().is_empty());
    assert!(HannCacheF64::default().is_empty());
}

#[test]
fn f64_cache_generates_and_reuses_same_key() {
    let mut cache = HannCacheF64::new();

    let first = cache.get(4, HannMode::Periodic);
    for (actual, expected) in first.iter().zip([0.0, 0.5, 1.0, 0.5]) {
        assert_close_f64(*actual, expected);
    }
    let first_pointer = first.as_ptr();

    let second_pointer = cache.get(4, HannMode::Periodic).as_ptr();
    assert_eq!(first_pointer, second_pointer);
    assert_eq!(cache.len(), 1);
}

#[test]
fn f64_cache_keeps_distinct_length_and_mode_entries() {
    let mut cache = HannCacheF64::new();

    assert_eq!(cache.get(3, HannMode::Symmetric).len(), 3);
    assert_eq!(cache.get(4, HannMode::Symmetric).len(), 4);
    assert_eq!(cache.get(4, HannMode::Periodic).len(), 4);
    assert_eq!(cache.len(), 3);
    assert!(!cache.is_empty());
}

#[test]
fn f64_cache_clear_releases_entries() {
    let mut cache = HannCacheF64::new();
    cache.get(4, HannMode::Periodic);
    cache.get(5, HannMode::Symmetric);
    assert_eq!(cache.len(), 2);

    cache.clear();

    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}
