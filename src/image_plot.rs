use image::{ImageBuffer, Rgba, RgbaImage};

use ::utils::clamp;
use ::plot::Plotter;

pub type RgbaFloatImage = ImageBuffer<Rgba<f32>, Vec<f32>>;

pub struct ImagePlot {
    image: RgbaFloatImage,
    foreground: Rgba<f32>,
}

impl ImagePlot {
    pub fn new(width: u32, height: u32) -> ImagePlot {
        ImagePlot {
            image: RgbaFloatImage::from_pixel(width, height, Rgba {
                data: [1.0, 1.0, 1.0, 1.0]
            }),
            foreground: Rgba { data: [0.0, 0.0, 0.0, 1.0] },
        }
    }

    pub fn with_background(width: u32, height: u32, background: Rgba<f32>) -> ImagePlot {
        ImagePlot {
            image: RgbaFloatImage::from_pixel(width, height, background),
            foreground: Rgba { data: [0.0, 0.0, 0.0, 1.0] },
        }
    }

    pub fn set_foreground(&mut self, foreground: Rgba<f32>) -> Rgba<f32> {
        ::std::mem::replace(&mut self.foreground, foreground)
    }

    pub fn foreground(&self) -> Rgba<f32> {
        self.foreground
    }

    pub fn image(&self) -> &RgbaFloatImage { &self.image }

    pub fn image_mut(&mut self) -> &mut RgbaFloatImage { &mut self.image }

    pub fn into_image(self) -> RgbaFloatImage {
        self.image
    }

    pub fn into_u8_component_image(self) -> RgbaImage {
        let (w, h) = self.image.dimensions();
        RgbaImage::from_raw(w, h, self.image.into_raw().into_iter().map(|subpixel| (subpixel * 255.0) as u8).collect()).unwrap()
    }
}

fn blend_over(top: Rgba<f32>, bottom: Rgba<f32>) -> Rgba<f32> {
    fn over_component(x: f32, y: f32, a: f32, b: f32) -> f32 {
        let a1 = 1.0 - a;
        (x * a + y * b * a1) / (a + b * a1)
    }

    let top_alpha = top.data[3];
    let bottom_alpha = bottom.data[3];

    Rgba {
        data: [
            over_component(top.data[0], bottom.data[0], top_alpha, bottom_alpha),
            over_component(top.data[1], bottom.data[1], top_alpha, bottom_alpha),
            over_component(top.data[2], bottom.data[2], top_alpha, bottom_alpha),
            top_alpha + bottom_alpha * (1.0 - top_alpha)
        ]
    }
}

impl Plotter for ImagePlot {
    fn width(&self) -> u32 { self.image.width() }

    fn height(&self) -> u32 { self.image.height() }

    fn draw_pixel(&mut self, x: i64, y: i64, alpha: f64) {
        let (w, h) = (self.width() as i64, self.height() as i64);

        if 0 <= x && x <= w && 0 <= y && y <= h {
            let x = x as u32;
            let y = (h - y - 1) as u32;

            let mut p = self.image.get_pixel_mut(x, y);

            *p = blend_over(Rgba {
                data: [
                    self.foreground.data[0], self.foreground.data[1], self.foreground.data[2],
                    clamp(alpha, 0.0, 1.0) as f32 * self.foreground[3]
                ]
            }, p.clone());
        }
    }
}