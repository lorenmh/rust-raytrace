use nalgebra as na;

pub mod camera;
pub mod object;

pub type Triangle = [na::Point3<f32>; 3];
pub type Mesh = std::vec::Vec<Triangle>;
pub type Color = [u8; 3];

trait Render {
    fn position(&self) -> na::Vector3<f32>;
    fn rotation(&self) -> na::Vector3<f32>;
    fn mesh(&self) -> Mesh;
    fn color(&self) -> Color;

    fn mat_translation(&self) -> na::Matrix3<f32> {
        na::Matrix3::from_diagonal(&self.position())
    }

    fn mat_rotation(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_rotation(self.rotation())
    }
}

trait Velocity {
    fn velocity() -> na::Vector3<f32>;
}
