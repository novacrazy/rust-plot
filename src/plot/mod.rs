//! Methods and data structures for plotting lines and shapes on pixel-oriented structures, such as images or a screen.

pub mod dot;
pub mod line;
pub mod shape;

/// Different styles of lines that can be drawn
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    /// A thin line
    Thin,
    /// A thin line with anti-aliasing. Antialiased lines can be drawn at fractional pixels,
    /// making them more suitable for detailed drawings.
    ThinAA,
    /// A thick line drawn by placing many dots on top of each other
    Thick { width: f64, hardness: f64 },
    /// A thick line with anti-aliasing. Antialiased lines can be drawn at fractional pixels,
    /// making them more suitable for detailed drawings.
    ///
    /// A thick line is drawn by placing many dots on top of each other.
    ThickAA { width: f64, hardness: f64 }
}

impl LineStyle {
    /// Convenience method for `LineStyle::Thick`
    pub fn thick(width: f64, hardness: f64) -> LineStyle {
        LineStyle::Thick { width: width, hardness: hardness }
    }

    /// Add anti-aliasing to the line style
    pub fn aa(self) -> LineStyle {
        match self {
            LineStyle::Thin => LineStyle::ThinAA,
            LineStyle::Thick { width, hardness } => LineStyle::ThickAA { width: width, hardness: hardness },
            _ => self
        }
    }
}

/// Common methods for plotters
pub trait Plotter {
    /// Get the width (in pixels) of the plotter
    fn width(&self) -> u32;

    /// Get the height (in pixels) of the plotter
    fn height(&self) -> u32;

    /// Draw a single pixel at the given x and y coordinates
    ///
    /// Signed coordinates are allowed for custom handling of overflow.
    /// Usually negative coordinates would just be ignored.
    fn draw_pixel(&mut self, x: i64, y: i64, alpha: f64);

    /// Draw a dot at the given coordinates
    ///
    /// By default, this draws a dot using a Gaussian function distribution, giving it a nice smooth falloff.
    ///
    /// This is used to draw thick lines by placing dots along the line.
    #[inline]
    fn draw_dot(&mut self, x: i64, y: i64, alpha: f64, width: f64, hardness: f64) {
        dot::plot_gaussian_dot(x, y, alpha, width, hardness, |x, y, alpha| self.draw_pixel(x, y, alpha))
    }

    /// Draw a line between the given coordinates with the given style.
    ///
    /// Note that antialiased lines accept fractional pixel values,
    /// while non-AA lines are rounded to the nearest whole pixel.
    #[inline]
    fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, style: LineStyle) {
        match style {
            LineStyle::Thin => {
                line::draw_line_bresenham(x0.round() as i64, y0.round() as i64, x1.round() as i64, y1.round() as i64,
                                          |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::ThinAA => {
                line::draw_line_xiaolin_wu(x0, y0, x1, y1,
                                           |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::Thick { width, hardness } => {
                line::draw_line_bresenham(x0.round() as i64, y0.round() as i64, x1.round() as i64, y1.round() as i64,
                                          |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
            LineStyle::ThickAA { width, hardness } => {
                line::draw_line_xiaolin_wu(x0, y0, x1, y1,
                                           |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
        }
    }

    /// Draw a circle with its center at the given coordinates.
    #[inline]
    fn draw_circle(&mut self, x: i64, y: i64, radius: i64, style: LineStyle) {
        match style {
            LineStyle::Thin => {
                shape::circle::draw_circle(x, y, radius, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::ThinAA => {
                shape::circle::draw_circle_aa(x, y, radius, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::Thick { width, hardness } => {
                shape::circle::draw_circle(x, y, radius, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
            LineStyle::ThickAA { width, hardness } => {
                shape::circle::draw_circle_aa(x, y, radius, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
        }
    }

    /// Draw an ellipse between the given bounding box.
    #[inline]
    fn draw_ellipse(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, style: LineStyle) {
        match style {
            LineStyle::Thin => {
                shape::ellipse::draw_ellipse(x0, y0, x1, y1, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::ThinAA => {
                shape::ellipse::draw_ellipse_aa(x0, y0, x1, y1, |x, y, alpha| self.draw_pixel(x, y, alpha))
            }
            LineStyle::Thick { width, hardness } => {
                shape::ellipse::draw_ellipse(x0, y0, x1, y1, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
            LineStyle::ThickAA { width, hardness } => {
                shape::ellipse::draw_ellipse_aa(x0, y0, x1, y1, |x, y, alpha| self.draw_dot(x, y, alpha, width, hardness))
            }
        }
    }
}

impl<'a, P: Plotter> Plotter for &'a mut P {
    #[inline(always)]
    fn width(&self) -> u32 {
        (**self).width()
    }

    #[inline(always)]
    fn height(&self) -> u32 {
        (**self).height()
    }

    #[inline(always)]
    fn draw_pixel(&mut self, x: i64, y: i64, alpha: f64) {
        (**self).draw_pixel(x, y, alpha)
    }
}

impl<P: Plotter> Plotter for Box<P> {
    #[inline(always)]
    fn width(&self) -> u32 {
        (**self).width()
    }

    #[inline(always)]
    fn height(&self) -> u32 {
        (**self).height()
    }

    #[inline(always)]
    fn draw_pixel(&mut self, x: i64, y: i64, alpha: f64) {
        (**self).draw_pixel(x, y, alpha)
    }
}
