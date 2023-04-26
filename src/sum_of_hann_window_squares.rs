use lazy_static::lazy_static;
use std::collections::HashMap;
use std::usize;

use crate::hann_window::HANN_WINDOW_LOOKUP_TABLE;

// Defining a lazy_static block for the HANN_LOOKUP_TABLE
lazy_static! {
  // A lookup table for pre-computed sum of squares.
  pub static ref HANN_WINDOW_SUM_OF_SQUARES: HashMap<usize, f32> = {
    // Defining an array of pre-computed window lengths
    const HANN_WINDOW_PRECOMPUTED_LENGTHS: [usize; 5] = [256, 512, 1024, 2048, 4096];

      // Initialize an empty HashMap for the lookup table
      let mut table = HashMap::new();

      // Iterate over the pre-computed lengths and calculate the Hann windows
      for &length in &HANN_WINDOW_PRECOMPUTED_LENGTHS {
          let hann_window = HANN_WINDOW_LOOKUP_TABLE.get(&length).expect("Failed to get the Hann window from the lookup table");
          let sum_of_squares = hann_window.iter().map(|&x| x.powi(2)).sum();

          // Insert the computed Hann window into the lookup table with the corresponding length
          table.insert(length, sum_of_squares);
      }

      // Return the populated lookup table
      table
  };
}

/// Compute the sum of squares of a Hann window.
///
/// This function takes a reference to a Vec `hann_window` representing a Hann window
/// and returns the sum of squares of the elements in the window. The sum of squares is computed
/// using a precomputed lookup table for Hann windows of length 512, 1024, 2048, and 4096. If the
/// length of the input `hann_window` is not in the lookup table, the sum of squares is computed
/// using `map` and `sum`.
pub fn get_hann_window_sum_squares(hann_window: &Vec<f32>) -> f32 {
  // Check if the sum-of-squares for the input Hann window length is in the lookup table
  if let Some(sum_squares) = HANN_WINDOW_SUM_OF_SQUARES.get(&hann_window.len()) {
    // If it is, return the precomputed value
    sum_squares.clone()
  } else {
    // Otherwise, compute the sum-of-squares using `map` and `sum`
    hann_window
      .iter()
      .map(|&x| x.powi(2))
      .sum()
  }
}

#[cfg(test)]
mod test_hann_window {
  use approx::relative_eq;

  use super::*;

  #[test]
  fn test_get_hann_window_sum_squares_256() {
    // Test a Hann window of length 256
    let hann_window = HANN_WINDOW_LOOKUP_TABLE.get(&256).clone().unwrap();
    let hann_window_sum_squares = get_hann_window_sum_squares(hann_window);

    let approx_eq = relative_eq!(hann_window_sum_squares, 95.625, epsilon = 1e-6);

    assert!(approx_eq);
  }

  #[test]
  fn test_get_hann_window_sum_squares_512() {
    // Test a Hann window of length 512
    let hann_window = HANN_WINDOW_LOOKUP_TABLE.get(&512).clone().unwrap();
    let hann_window_sum_squares = get_hann_window_sum_squares(hann_window);

    let approx_eq = relative_eq!(hann_window_sum_squares, 191.62506, epsilon = 1e-6);

    assert!(approx_eq);
  }

  #[test]
  fn test_get_hann_window_sum_squares_1024() {
    // Test a Hann window of length 1024
    let hann_window = HANN_WINDOW_LOOKUP_TABLE.get(&1024).clone().unwrap();
    let hann_window_sum_squares = get_hann_window_sum_squares(hann_window);

    let approx_eq = relative_eq!(hann_window_sum_squares, 383.62506, epsilon = 1e-6);

    assert!(approx_eq);
  }
}