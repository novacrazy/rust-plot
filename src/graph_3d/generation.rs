use std::ops::Range;

use nalgebra::Point3;

pub fn plot_xz_function_to_3d_grid_mesh<F>(domain_x: Range<f32>, domain_z: Range<f32>, x_steps: usize, z_steps: usize, f: F) -> Vec<Point3<f32>> where F: Fn(f32, f32) -> f32 {
    let mut points: Vec<Point3<f32>> = Vec::new();

    let x0 = domain_x.start;
    let z0 = domain_z.start;

    let x1 = domain_x.end;
    let z1 = domain_z.end;

    let dx = (x1 - x0) / x_steps as f32;
    let dz = (z1 - z0) / z_steps as f32;

    let mut z2 = z0;

    while z2 <= z1 {
        let mut x2 = x0;

        while x2 <= x1 {
            //bottom left
            let a = (x2, z2);
            //bottom right
            let b = (x2 + dx, z2);
            //top left
            let c = (x2, z2 + dz);
            //top right
            let d = (x2 + dx, z2 + dz);

            let a_y = f(a.0, a.1);
            let b_y = f(b.0, b.1);
            let c_y = f(c.0, c.1);
            let d_y = f(d.0, d.1);

            points.push(Point3::new(a.0, a_y, a.1));
            points.push(Point3::new(c.0, c_y, c.1));
            points.push(Point3::new(b.0, b_y, b.1));

            points.push(Point3::new(d.0, d_y, d.1));
            points.push(Point3::new(b.0, b_y, b.1));
            points.push(Point3::new(c.0, c_y, c.1));

            x2 += dx;
        }

        z2 += dz;
    }

    points
}