//! Generic numeric utilities

use num_traits::Num;

/// Generic min-max function for any `PartialOrd`
///
/// ```
/// use rust_plot::utils::min_max;
///
/// assert_eq!(min_max(2, 1), (1, 2));
/// ```
#[inline(always)]
pub fn min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}

/// Clamp a value to the given range
///
/// ```
/// use rust_plot::utils::clamp;
///
/// assert_eq!(clamp(15u32, 0, 5), 5);
/// ```
pub fn clamp<T>(value: T, min: T, max: T) -> T where T: PartialOrd {
    if value < min { min } else if value > max { max } else { value }
}

/// Linear interpolation for numeric types
///
/// ```
/// use rust_plot::utils::lerp;
///
/// assert_eq!(lerp(0.5f32, 0.0, 0.0, 1.0, 3.0), 1.5);
/// ```
pub fn lerp<T: Num + Copy>(x: T, x0: T, y0: T, x1: T, y1: T) -> T {
    y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
}

/// Scales a value between the range `in_min` and `in_max` to the range of `out_min` to `out_max`
///
/// ```
/// use rust_plot::utils::scale;
///
/// assert_eq!(scale(0.5f32, 0.0, 1.0, 0.0, 2.0), 1.0);
/// ```
#[inline]
pub fn scale<T: Num + Copy>(x: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}