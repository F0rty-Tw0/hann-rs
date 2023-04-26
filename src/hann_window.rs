use std::{ error::Error, f32::consts::PI, fmt };

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
/// `Result<Array1<Complex<f32>>, HannWindowError>` A Array1 containing the Hann window values.
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