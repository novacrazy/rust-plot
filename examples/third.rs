extern crate nalgebra;
extern crate image;
extern crate rayon;
extern crate softrender;
extern crate rust_plot;

use std::sync::Arc;

use nalgebra::{Matrix4, Vector3, Vector4, Point3};

use rust_plot::plot::{LineStyle, Plotter};
use rust_plot::image_plot::ImagePlot;

use softrender::pixel::RGBAf32Pixel;
use softrender::mesh::{Mesh, Vertex};
use softrender::render::{FrameBuffer, ShadingPipeline, ClipVertex, ScreenVertex};
use softrender::image_compat::ImageFrameBuffer;

fn generate_mesh(theta: f32) -> Arc<Mesh<()>> {
    let f = |x: f32, y: f32| {
        let d = x.hypot(y);
        (d + theta * 2.0).sin() + rust_plot::stat::gaussian_dot_pdf_32(d, 10.0, 1.0)
    };

    let vertices = rust_plot::graph_3d::generation::plot_xz_function_to_3d_grid_mesh(-10.0..10.0, -10.0..10.0, 400, 400, f);

    let indices = vertices.iter().enumerate().map(|(i, _)| i as u32).collect();

    Arc::new(Mesh {
        vertices: vertices.into_iter().map(|vertex| Vertex { position: vertex, vertex_data: () }).collect(),
        indices: indices
    })
}

fn main() {
    //let frames: usize = 23;

    //(23..frames + 1).into_par_iter().for_each(|frame| {

    let theta = 45.0f32.to_radians(); //((frame as f32 / frames as f32) * 360.0).to_radians();

    let mut framebuffer = FrameBuffer::<RGBAf32Pixel>::new_with(1000, 1000, RGBAf32Pixel { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });

    framebuffer.set_blend_function(|a, b| {
        RGBAf32Pixel {
            r: a.r * a.a + b.r * (1.0 - a.a),
            g: a.g * a.a + b.g * (1.0 - a.a),
            b: a.b * a.a + b.b * (1.0 - a.a),
            a: a.a + b.a,
        }
    });

    let mesh = generate_mesh(theta);

    let view = nalgebra::Isometry3::look_at_rh(
        &Point3::new(12.0, 12.0, 12.0),
        &Point3::origin(),
        &Vector3::new(0.0, 1.0, 0.0)
    ).to_homogeneous();

    let projection = nalgebra::Perspective3::new(framebuffer.width() as f32 / framebuffer.height() as f32,
                                                 75.0f32.to_radians(), 0.001, 1000.0).to_homogeneous();

    let mut pipeline = ShadingPipeline::new(framebuffer, ());

    {
        let vertex_shader = pipeline.render_mesh(mesh.clone());

        let fragment_shader = vertex_shader.run(|vertex, _| {
            let position = projection * view * vertex.position.to_homogeneous();
            ClipVertex::new(position, 0.0)
        });

        fragment_shader.run(|screen, _| {
            // Determine the color of the pixel here
            RGBAf32Pixel { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
        });
    }

    let image = pipeline.framebuffer().copy_to_image().unwrap();

    image.save("examples/third.png").unwrap();
}