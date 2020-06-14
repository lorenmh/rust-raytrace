use nalgebra as na;
use gl::types::GLfloat;

pub struct Object {
    pub pos: na::Vector3<f32>,
    pub vel: na::Vector3<f32>,
    pub rot: na::Vector3<f32>,
}

pub fn new(x: f32, y: f32, z: f32) -> Object {
   Object{
       pos: na::Vector3::new(x, y, z),
       vel: na::Vector3::<f32>::zeros(),
       rot: na::Vector3::<f32>::zeros(),
   }
}

impl Object {
    pub fn to_string(&self) -> std::string::String {
       format!(
           "pos: {:?}\nvel: {:?}\nrot: {:?}",
           self.pos,
           self.vel,
           self.rot,
       )
    }

    pub fn transformation(&self) -> std::vec::Vec<GLfloat> {
        let m = self.mat_model();

        std::vec::Vec::from(m.as_slice())
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

impl std::fmt::Display for Object {
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
