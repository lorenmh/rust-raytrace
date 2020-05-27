use nalgebra as na;

pub mod camera;
pub mod object;

pub type Triangle = [na::Point3<f32>; 3];
pub type Mesh = std::vec::Vec<Triangle>;
pub type Color = [u8; 3];

trait Render {
    fn position() -> na::Point3<f32>;
    fn rotation() -> na::Vector3<f32>;
    fn mesh() -> Mesh;
    fn color() -> Color;

    fn transform(&self) -> na::Matrix4<f32> {
        self.transform()
    }
}

trait Velocity {
    fn velocity() -> na::Vector3<f32>;
}
