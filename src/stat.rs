//! Probability Distribution Functions

use num_traits::Float;

pub fn gaussian_dot_pdf(x: f64, width: f64, hardness: f64) -> f64 {
    let exponent = (x / (width * 0.5)).powf(2.0f64.powf(hardness));

    ::std::f64::consts::E.powf(-exponent)
}

#[inline]
pub fn gaussian_dot_pdf_32(x: f32, width: f32, hardness: f32) -> f32 {
    gaussian_dot_pdf(x as f64, width as f64, hardness as f64) as f32
}