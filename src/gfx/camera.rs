use nalgebra as na;

pub struct Camera {
    pub phys: crate::physics::Physics,
    pub target: na::Point3<f32>,
    pub perspective: na::Matrix4<f32>,
}

pub fn new(x: f32, y: f32, z: f32, aspect: f32, fov: f32) -> Camera {
    Camera{
        phys: crate::physics::new(x, y, z),
        target: na::Point3::new(0.0, 0.0, 0.0),
        perspective: na::Matrix4::new_perspective(aspect, fov, 1.0, -1.0),
    }
}

impl Camera {
    pub fn look_at(&mut self, point: &na::Point3<f32>) {
        self.target = *point;
    }

    pub fn transformation(&self) -> na::Matrix4<f32> {
        let rot = na::Matrix4::new_rotation(self.phys.rot);
        self.perspective * rot * na::Matrix4::look_at_rh(
            &na::Point3::new(self.phys.pos.x, self.phys.pos.y, self.phys.pos.z),
            &self.target,
            &na::Vector3::new(0.0, 1.0, 0.0),
        )
    }
}
