use nalgebra as na;

type Triangle = [na::Point3<f32>; 3];
type Mesh = std::vec::Vec<Triangle>;
type Color = [u8; 3];

struct Object {
    pos: na::Point3<f32>,
    vel: na::Vector3<f32>,
    rot: na::Vector3<f32>,
    scale: f32,

    // TODO: move the mesh / color data to own modules
    mesh: Mesh,
    color: Color,
}

