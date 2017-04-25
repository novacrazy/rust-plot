use nalgebra::{Perspective3, Orthographic3, Matrix4, Point3, Vector4};

pub enum ProjectionMethod {
    Perspective(Perspective3<f32>),
    Orthographic(Orthographic3<f32>)
}

pub fn project_points_to_screen<I: IntoIterator<Item=Point3<f32>>>(view: Matrix4<f32>,
                                                                   projection: Matrix4<f32>,
                                                                   viewport: (f32, f32),
                                                                   points: I) -> Box<Iterator<Item=Vector4<f32>>> where I: 'static {
    Box::new(points.into_iter().map(move |p: Point3<f32>| {
        let mut p = projection * view * p.to_homogeneous();

        p.x = (p.x / p.w + 1.0) * (viewport.0 / 2.0);
        p.y = (p.y / p.w + 1.0) * (viewport.1 / 2.0);

        p
    }))
}