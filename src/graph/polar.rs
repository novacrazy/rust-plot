use std::ops::Range;

use ::plot::{LineStyle, Plotter};

pub trait PolarGraph {
    /// Graphs a set of samples from a polar function
    ///
    /// The samples are composed of two-element tuples, where the first element is the polar angle,
    /// and the second element is the radius given by the function.
    fn polar_samples(&mut self, samples: Vec<(f64, f64)>, style: LineStyle);
    fn polar_equation<F>(&mut self, a_domain: Range<f64>, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> f64;
    fn draw_polar_axis(&mut self, divisions: usize, style: LineStyle);
}

impl<P: Plotter> PolarGraph for super::Graph<P> {
    fn polar_samples(&mut self, samples: Vec<(f64, f64)>, style: LineStyle) {}

    fn polar_equation<F>(&mut self, a_domain: Range<f64>, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> f64 {}

    fn draw_polar_axis(&mut self, divisions: usize, style: LineStyle) {}
}