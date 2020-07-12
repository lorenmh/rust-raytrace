use nalgebra as na;

use nphysics3d::object::{BodyDesc, ColliderDesc, RigidBodyDesc, BodyPartHandle};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use ncollide3d::transformation::ToTriMesh;

pub struct Cube {
    pub body: BodyDesc<f32>,
    pub collider: ColliderDesc<f32>,
    pub shape: ShapeHandle<f32>,
    pub gfx: crate::gfx::render::Renderer,
}

pub fn new(
    env: &mut crate::shapes::Environment,
    x: f32, y: f32, z: f32,
    width: f32, height: f32, depth: f32,
    color: crate::gfx::ColorFn
) -> Cube {
    let mut body = (
        RigidBodyDesc::<f32>::new()
            .translation(na::Vector3::new(x, y, z))
            .build()
    );

    let body_handle = env.bodies.insert(body);

    let cuboid = Cuboid::<f32>::new(
        na::Vector3::new(width/2.0, height/2.0, depth/2.0)
    );

    let shape = ShapeHandle::<f32>::new(cuboid);

    let mut collider = (
        ColliderDesc::<f32>::new(shape)
            .density(1.0)
            .build(BodyPartHandle(body_handle, 0))
    );

    env.colliders.insert(collider);

    let mesh = cuboid.to_trimesh();

    Cube{
        body, collider, shape,
        gfx: crate::gfx::render::new(
            mesh,
            color,
        ),
    }
}

impl Cube {
    pub fn render(&mut self, params: &crate::gfx::render::Params) { self.gfx.render(&self.phys, params).expect("err rendering") }
}
