use nalgebra as na;

struct Object {
    pos: na::Point3<f32>,
    vel: na::Vector3<f32>,
    rot: na::Vector3<f32>,
    scale: f32,

    // TODO: move the mesh / color data to own modules
    mesh: crate::gfx::Mesh,
    color: crate::gfx::Color,
}