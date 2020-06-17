use nalgebra as na;
use gl::types::GLfloat;

pub struct Physics {
    pub pos: na::Vector3<f32>,
    pub vel: na::Vector3<f32>,
    pub rot: na::Vector3<f32>,
    pub ang: na::Vector3<f32>,
}

pub fn new(x: f32, y: f32, z: f32) -> Physics {
   Physics {
       pos: na::Vector3::new(x, y, z),
       vel: na::Vector3::<f32>::zeros(),
       rot: na::Vector3::<f32>::zeros(),
       ang: na::Vector3::<f32>::zeros(),
   }
}

impl Physics {
    pub fn move_(&mut self, t: f32) {
        self.pos += na::Vector3::new(t * self.vel.x, t * self.vel.y, t * self.vel.z);
        self.rot += na::Vector3::new(t * self.ang.x, t * self.ang.y, t * self.ang.z);
    }

    pub fn mat_model(&self) -> na::Matrix4<f32> {
        self.mat_translation() * self.mat_rotation()
    }

    pub fn mat_translation(&self) -> na::Matrix4<f32> {
        let p = self.pos;

        // column major format
        na::Matrix4::from_vec(vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            p.x, p.y, p.z, 1.0,
        ])
    }

    pub fn mat_rotation(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_rotation(self.rot)
    }
}

impl std::fmt::Display for Physics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "pos: {:?}\nvel: {:?}\nrot: {:?}",
            self.pos,
            self.vel,
            self.rot,
        )
    }
}
