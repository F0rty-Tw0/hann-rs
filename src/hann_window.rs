use lazy_static::lazy_static;
use std::{ collections::HashMap, error::Error, f32::consts::PI, fmt };

/// Error type for the Hann window function.
#[derive(Debug, PartialEq)]
pub enum HannWindowError {
  WindowLengthTooSmall,
  WindowLengthTooLarge,
  MemoryAllocationError,
}

// Implement the Error trait for the HannWindowError struct
impl Error for HannWindowError {}

// Implement the Display trait for the HannWindowError struct
impl fmt::Display for HannWindowError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write the error message to the Formatter
    match self {
      HannWindowError::WindowLengthTooSmall => {
        write!(f, "HannWindowError: Window length must be greater than 1.")
      }
      HannWindowError::WindowLengthTooLarge => {
        write!(f, "HannWindowError: Window length is too large.")
      }
      HannWindowError::MemoryAllocationError => {
        write!(f, "HannWindowError: Window length is too large to allocate memory.")
      }
    }
  }
}

// Defining a lazy_static block for the HANN_LOOKUP_TABLE
lazy_static! {
  // A lookup table for pre-computed Hann windows.
  pub static ref HANN_WINDOW_LOOKUP_TABLE: HashMap<usize, Vec<f32>> = {
    // Defining an array of pre-computed window lengths
    const HANN_WINDOW_PRECOMPUTED_LENGTHS: [usize; 5] = [256, 512, 1024, 2048, 4096];
    // Initialize an empty HashMap for the lookup table
    let mut table = HashMap::new();
    // Iterate over the pre-computed lengths and calculate the Hann windows
    for &length in &HANN_WINDOW_PRECOMPUTED_LENGTHS {
        let hann_window = calculate_hann_window(length).expect("Failed to compute the Hann window");
        // Insert the computed Hann window into the lookup table with the corresponding length
        table.insert(length, hann_window);
    }
    // Return the populated lookup table
    table
  };
}

/// Compute a Hann window of the given length.
///
/// This function takes an integer `window_length` representing the desired length of the Hann window,
/// and returns an `Vec<f32>` containing the Hann window values. If the `window_length` is less
/// than or equal to 1, or greater than the allowed maximum, an error is returned. If the `window_length`
/// is in the precomputed lookup table, the precomputed values are returned. Otherwise, the Hann window
/// values are computed using the formula `w(n) = 0.5 - 0.5 * cos(2π * n / (N - 1))`, where `n` is the
/// index of the current sample and `N` is the length of the window.
pub fn get_hann_window(window_length: usize) -> Result<Vec<f32>, HannWindowError> {
  // If the window length is less than or equal to 1, return an array with a single element of 0.0
  if window_length <= 1 {
    return Err(HannWindowError::WindowLengthTooSmall);
  }
  // Check if the window length exceeds the maximum allowed
  if window_length > usize::MAX / 2 {
    return Err(HannWindowError::MemoryAllocationError);
  }
  // Check if the window length exceeds the allowed maximum
  if window_length > 1 << 24 {
    return Err(HannWindowError::WindowLengthTooLarge);
  }
  // Check if the window length is in the lookup table.
  if let Some(hann_window) = HANN_WINDOW_LOOKUP_TABLE.get(&window_length) {
    Ok(hann_window.clone())
  } else {
    // If the window length is not in the lookup table, compute the Hann window values.
    calculate_hann_window(window_length)
  }
}

/// Computes a Hann window of length `window_length`.
///
/// A Hann window is a function that smoothly tapers the edges of a signal window to reduce spectral leakage.
/// This function computes the Hann window values for a given window length and returns them as a vector.
/// https://en.wikipedia.org/wiki/Window_function#Hann_and_Hamming_windows
/// Formula used: w(n) = 0.5 - 0.5 * cos(2π * n / (N - 1))
///
/// # Arguments
/// `window_length` The length of the Hann window.
///
/// # Returns
/// `Result<Vec<Complex<f32>>, HannWindowError>` A Vec containing the Hann window values.
/// or an error if the window length is less than or equal to 1 or if the window length is too large.
pub fn calculate_hann_window(window_length: usize) -> Result<Vec<f32>, HannWindowError> {
  // If the window length is less than or equal to 1, return an array with a single element of 0.0
  if window_length <= 1 {
    return Err(HannWindowError::WindowLengthTooSmall);
  }

  // Check if the window length exceeds the maximum allowed
  if window_length > usize::MAX / 2 {
    return Err(HannWindowError::MemoryAllocationError);
  }

  // Check if the window length exceeds the allowed maximum
  if window_length > 1 << 24 {
    return Err(HannWindowError::WindowLengthTooLarge);
  }

  // Since the Hann window is symmetric, we can compute only half of the values and mirror them to the other half.
  // This reduces the number of cosine computations by half.
  // Calculate the half-length of the window, accounting for odd window lengths.
  let half_length = (window_length + (window_length % 2)) / 2;

  // Compute the scaling factor for the Hann window: 2π / (N - 1)
  // The scaling factor adjusts the window values based on the length of the window
  // and is used in the formula to calculate the Hann window values for each sample.
  let scaling_factor = (PI * 2.0) / ((window_length - 1) as f32);

  // Initialize the window array with zeros and a length equal to the window_length
  let mut window = vec![0.0; window_length];

  // Compute the first half of the Hann window values
  // Formula used: w(n) = 0.5 - 0.5 * cos(2π * n / (N - 1))
  for i in 0..half_length {
    window[i] = 0.5 - 0.5 * ((scaling_factor * (i as f32)).cos() as f32);
    window[window_length - 1 - i] = window[i];
  }

  // Return the Hann window values.
  Ok(window)
}

#[cfg(test)]
mod test_hann_window {
  use approx::{ assert_abs_diff_eq, relative_eq };

  use super::*;

  const WINDOW_LENGTH_5: usize = 5;
  const WINDOW_LENGTH_10: usize = 10;

  #[test]
  fn test_hann_window_length() {
    let hann_window = calculate_hann_window(WINDOW_LENGTH_10).unwrap();

    assert_eq!(hann_window.len(), WINDOW_LENGTH_10);
  }

  #[test]
  fn test_hann_window_properties() {
    let hann_window = get_hann_window(WINDOW_LENGTH_10).unwrap();
    assert_abs_diff_eq!(hann_window[0], 0.0, epsilon = 1e-6);
    assert_abs_diff_eq!(hann_window[WINDOW_LENGTH_10 - 1], 0.0, epsilon = 1e-6);
    assert!(hann_window.iter().all(|&value| value >= 0.0));
  }

  #[test]
  fn test_even_hann_window_values() {
    let expected_window_value = vec![
      0.0,
      0.11697778,
      0.41317594,
      0.75,
      0.96984637,
      0.96984637,
      0.75,
      0.41317594,
      0.11697778,
      0.0
    ];

    let hann_window = calculate_hann_window(WINDOW_LENGTH_10).unwrap();

    for i in 0..WINDOW_LENGTH_10 {
      assert_eq!(hann_window[i], expected_window_value[i]);
    }
  }

  #[test]
  fn test_odd_hann_window_values() {
    let expected_window_value = vec![0.0, 0.5, 1.0, 0.5, 0.0];

    let hann_window = calculate_hann_window(WINDOW_LENGTH_5).unwrap();

    for i in 0..WINDOW_LENGTH_5 {
      assert_eq!(hann_window[i], expected_window_value[i]);
    }
  }

  #[test]
  fn test_hann_window_scaling_factor() {
    let hann_window = calculate_hann_window(WINDOW_LENGTH_10).unwrap();
    let scaling_factor = (PI * 2.0) / ((WINDOW_LENGTH_10 - 1) as f32);

    for i in 0..WINDOW_LENGTH_10 {
      let expected_value = 0.5 - 0.5 * (scaling_factor * (i as f32)).cos();

      let relative_eq = relative_eq!(hann_window[i], expected_value, epsilon = 1e-4);

      assert!(relative_eq);
    }
  }

  #[test]
  fn test_hann_window_length_too_small() {
    let window_length: usize = 1;

    let result = get_hann_window(window_length);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), HannWindowError::WindowLengthTooSmall);
  }

  #[test]
  fn test_hann_window_length_too_large() {
    let window_length: usize = 1 << 25; // Larger than the allowed maximum (1 << 24)

    let result = get_hann_window(window_length);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), HannWindowError::WindowLengthTooLarge);
  }

  #[test]
  fn test_hann_window_length_too_large_to_allocate_memory() {
    let window_length: usize = usize::MAX / 2 + 1; // Larger than the allowed maximum (usize::MAX / 2)

    let result = get_hann_window(window_length);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), HannWindowError::MemoryAllocationError);
  }
}