extern crate rust_plot;
extern crate image;

use rust_plot::plot::{LineStyle, Plotter};
use rust_plot::image_plot::ImagePlot;

fn main() {
    let mut plotter = ImagePlot::new(1000, 1000);

    plotter.draw_line(20.0, 20.0, 400.0, 900.0, LineStyle::thick(20.0, 2.0).aa());
    plotter.draw_circle(500, 500, 200, LineStyle::thick(5.0, 1.0).aa());

    plotter.into_u8_component_image().save("test.png").unwrap();
}