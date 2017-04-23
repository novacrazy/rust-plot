use std::ops::{Range, Deref, DerefMut};

pub mod axis;
pub mod rectangular;
pub mod polar;

use ::plot::Plotter;

pub struct Graph<P: Plotter> {
    plotter: P,
    x_domain: Range<f64>,
    y_domain: Range<f64>,
    break_discontinuous: bool,
}

impl<P: Plotter> Graph<P> {
    pub fn with_plotter(plotter: P, x_domain: Range<f64>, y_domain: Range<f64>) -> Graph<P> {
        Graph {
            plotter: plotter,
            x_domain: x_domain,
            y_domain: y_domain,
            break_discontinuous: true,
        }
    }

    pub fn x_domain(&self) -> Range<f64> {
        self.x_domain.clone()
    }

    pub fn y_domain(&self) -> Range<f64> {
        self.y_domain.clone()
    }

    pub fn into_plotter(self) -> P {
        self.plotter
    }

    pub fn break_discontinuous(&mut self) {
        self.break_discontinuous = true;
    }

    pub fn bridge_discontinuous(&mut self) {
        self.break_discontinuous = false;
    }
}


impl<P: Plotter> Deref for Graph<P> {
    type Target = P;

    fn deref(&self) -> &P {
        &self.plotter
    }
}

impl<P: Plotter> DerefMut for Graph<P> {
    fn deref_mut(&mut self) -> &mut P {
        &mut self.plotter
    }
}