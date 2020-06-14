use nalgebra as na;
use gl::types::GLfloat;

//fn position(&self) -> na::Vector3<f32>;
//fn rotation(&self) -> na::Vector3<f32>;
//fn scale(&self) -> f32;
//fn mesh(&self) -> Mesh;
//fn color(&self) -> Color;

pub struct Object {
    pub pos: na::Vector3<f32>,
    pub vel: na::Vector3<f32>,
    pub rot: na::Vector3<f32>,

    //pub scale: f32,
    //pub mesh: crate::gfx::Mesh,
    //pub color: fn(i32) -> crate::gfx::Color,
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
            p.x, p.y, p.x, 1.0,
        ])
    }

    pub fn mat_rotation(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_rotation(self.rot)
        //na::Matrix4::new_rotation(na::Vector3::new(0.0, 0.0, std::f32::consts::PI / 2.0))
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
