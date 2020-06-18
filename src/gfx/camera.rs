use nalgebra as na;

pub struct Camera {
    pub phys: crate::physics::Physics,
    pub perspective: na::Matrix4<f32>,
    pub orientation: na::Matrix4<f32>,
}

pub fn new(x: f32, y: f32, z: f32, aspect: f32, fov: f32) -> Camera {
    Camera{
        phys: crate::physics::new(x, y, z),
        perspective: na::Matrix4::new_perspective(aspect, fov, 1.0, -1.0),
        orientation: na::Matrix4::<f32>::identity(),
    }
}

impl Camera {
    pub fn look_at(&mut self, point: &na::Point3<f32>) {
        let (_, y, z,) = self.phys.direction();

        self.orientation = na::Matrix4::look_at_rh(
            &na::Point3::from(self.phys.pos),
            &na::Point3::from(self.phys.pos + z),
            &y,
        );
    }

    pub fn transformation(&self) -> na::Matrix4<f32> {
        let (_, y, z,) = self.phys.direction();
        self.perspective * na::Matrix4::look_at_rh(
            &na::Point3::from(self.phys.pos),
            &na::Point3::from(self.phys.pos + z),
            &y,
        )
    }
}
