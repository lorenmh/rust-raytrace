use nalgebra as na;
use gl::types::GLfloat;
use nphysics3d::object::{RigidBody, RigidBodyDesc};

mod world;

pub struct Physics {
    pub body: BodyDesc<f32>,
    pub collider: ColliderDesc<f32>,
}

pub fn new(x: f32, y: f32, z: f32) -> Physics {
    let body = (
        RigidBodyDesc::new()
            .translation(na::Vector3::new(x, y, z))
            .build()
    );

   Physics {
       body,
   }
}

impl std::fmt::Display for Physics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.body,
        )
    }
}
