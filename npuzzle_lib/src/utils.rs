//! Utilities and helpers
//!

pub mod status;
/// Checks if given coordinates x, y are within the limits (inclusive)
///
/// Example:
/// ```
///     assert!(is_in_bounds(3, 3, (0,0), (5, 5)))
///
/// ```
#[inline]
pub fn is_in_bounds<T: PartialOrd>(x: T, y: T, min: (T, T), max: (T, T)) -> bool {
	x >= min.0 && x <= max.0 && y >= min.1 && y <= max.1
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn bounds() {
		assert!(is_in_bounds(3, 3, (0, 0), (5, 5)));
		assert!(is_in_bounds(3.0, 3.0, (0.0, 0.1), (5.3, 5.2)));
		assert!(!is_in_bounds(-3.0, 3.0, (0.0, 0.1), (5.3, 5.2)));
	}
}
