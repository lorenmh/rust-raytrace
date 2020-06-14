use nalgebra as na;
use crate::gfx::render::Renderable;

pub struct Rectangle {
    pub obj: crate::gfx::object::Object,
    pub renderer: crate::gfx::render::Renderer,
}

pub fn new<'a>(x: f32, y: f32, z: f32, width: f32, height: f32, color: fn(i32) -> crate::gfx::Color) -> Rectangle {
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
        obj: crate::gfx::object::Object{
            pos: na::Vector3::new(x, y, z),
            vel: na::Vector3::new(0.0, 0.0, 0.0),
            rot: na::Vector3::new(0.0, 0.0, 0.0),
        },
        renderer: crate::gfx::render::Renderer{
            scale: 1.0,
            color,
            mesh,
        }
    }
}

impl Rectangle {
    pub fn render(&self, params: &crate::gfx::render::Params) { self.renderer.render(&self.obj, params).expect("err rendering") }
}
