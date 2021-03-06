extern crate nalgebra;
extern crate num_traits;
extern crate rand;

#[cfg(feature = "image_compat")]
extern crate image;

pub mod utils;
pub mod stat;
pub mod geometry;
pub mod bezier;
pub mod plot;
pub mod sampling;
pub mod graph;
pub mod graph_3d;

#[cfg(feature = "image_compat")]
pub mod image_plot;