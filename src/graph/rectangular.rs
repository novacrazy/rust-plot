use std::ops::Range;

use ::plot::{LineStyle, Plotter};

pub trait RectangularGraph {
    /// Graphs a set of samples from a linear or parametric equation, where a linear equation is
    /// simply a subset of parametric equations where the x term is known ahead of time.
    ///
    /// The samples are composed of two-element tuples, where the first element is the x-axis coordinate,
    /// and the second element is the y-axis coordinate.
    fn parametric_samples(&mut self, samples: Vec<(f64, f64)>, style: LineStyle);
    fn linear_equation<F>(&mut self, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> f64;
    fn parametric_equation<F>(&mut self, t_domain: Range<f64>, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> (f64, f64);
    fn draw_axis(&mut self, style: LineStyle);
}

impl<P: Plotter> RectangularGraph for super::Graph<P> {
    fn parametric_samples(&mut self, samples: Vec<(f64, f64)>, style: LineStyle) {

    }

    fn linear_equation<F>(&mut self, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> f64 {

    }

    fn parametric_equation<F>(&mut self, t_domain: Range<f64>, samples: usize, style: LineStyle, f: F) where F: Fn(f64) -> (f64, f64) {

    }

    fn draw_axis(&mut self, style: LineStyle) {

    }
}
