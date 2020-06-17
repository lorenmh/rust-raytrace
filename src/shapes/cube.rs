use nalgebra as na;
use crate::gfx::render::Renderable;

pub struct Cube {
    pub phys: crate::physics::Physics,
    pub gfx: crate::gfx::render::Renderer,
}

pub fn new<'a>(x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32, color: crate::gfx::ColorFn) -> Cube {
    let mut front = crate::shapes::rectangle::new(
        0.0,
        0.0,
        depth / 2.0,
        width,
        height,
        color,
    );

    let mut back = crate::shapes::rectangle::new(
        0.0,
        0.0,
        -depth / 2.0,
        width,
        height,
        color,
    );

    let mut left = crate::shapes::rectangle::new(
        -width / 2.0,
        0.0,
        0.0,
        depth,
        height,
        color,
    );
    left.phys.rot.y = std::f32::consts::PI / 2.0;

    let mut right = crate::shapes::rectangle::new(
        width / 2.0,
        0.0,
        0.0,
        depth,
        height,
        color,
    );
    right.phys.rot.y = -std::f32::consts::PI / 2.0;

    let mut top = crate::shapes::rectangle::new(
        0.0,
        height / 2.0,
        0.0,
        width,
        depth,
        color,
    );
    top.phys.rot.x = std::f32::consts::PI / 2.0;

    let mut bottom = crate::shapes::rectangle::new(
        0.0,
        -height / 2.0,
        0.0,
        width,
        depth,
        color,
    );
    bottom.phys.rot.x = -std::f32::consts::PI / 2.0;


    let mut mesh: std::vec::Vec<crate::gfx::Triangle> = front.vertices();
    mesh.extend(back.vertices());
    mesh.extend(left.vertices());
    mesh.extend(right.vertices());
    mesh.extend(top.vertices());
    mesh.extend(bottom.vertices());

    Cube{
        phys: crate::physics::new(x, y, z),
        gfx: crate::gfx::render::Renderer{
            scale: 1.0,
            color,
            mesh,
        }
    }
}

impl Cube {
    pub fn render(&self, params: &crate::gfx::render::Params) { self.gfx.render(&self.phys, params).expect("err rendering") }
}