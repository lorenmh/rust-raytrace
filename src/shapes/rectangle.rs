use nalgebra as na;
use core::convert::From;

pub struct Rectangle {
    pub phys: crate::physics::Physics,
    pub gfx: crate::gfx::render::Renderer,
}

pub fn new<'a>(x: f32, y: f32, z: f32, width: f32, height: f32, color: crate::gfx::ColorFn) -> Rectangle {
    let mesh: std::vec::Vec<crate::gfx::Triangle> = vec![
        [
            na::Point3::new(-width/2.0,  height/2.0,  0.0), // top left corner
            na::Point3::new( width/2.0,  height/2.0,  0.0), // top right corner
            na::Point3::new( width/2.0, -height/2.0,  0.0), // bottom right corner
        ],
        [
            na::Point3::new(-width/2.0,  height/2.0,  0.0), // top left corner
            na::Point3::new(-width/2.0, -height/2.0,  0.0), // bottom left corner
            na::Point3::new( width/2.0, -height/2.0,  0.0), // bottom right corner
        ]
    ];

    Rectangle{
        phys: crate::physics::new(x, y, z),
        gfx: crate::gfx::render::new(
            1.0,
            mesh,
            color,
        ),
    }
}

fn translate_mesh(&mat: &na::Matrix4<f32>, triangle: &crate::gfx::Triangle) -> crate::gfx::Triangle {
    let mut points: std::vec::Vec<na::Point3<f32>> = std::vec::Vec::with_capacity(3);
    for &point in triangle.into_iter() {
        let p = point.to_homogeneous();
        let mut t = mat * p;
        points.push(na::Point3::from_homogeneous(t).unwrap());
    }
    return [points[0], points[1], points[2]];
}

impl Rectangle {
    pub fn render(&mut self, params: &crate::gfx::render::Params) { self.gfx.render(&self.phys, params).expect("err rendering") }
    pub fn vertices(&self) -> std::vec::Vec<crate::gfx::Triangle> {
        let mat = self.phys.mat_model().to_homogeneous();
        return self.gfx.mesh.iter().map(|t| translate_mesh(&mat, &t)).rev().collect();
    }
}
