use nalgebra as na;
use gl::types::GLfloat;

mod environment;

pub struct Physics {
    pub pos: na::Vector3<f32>,
    pub vel: na::Vector3<f32>,
    pub rot: na::Vector3<f32>,
    //pub rot: na::Rotation3<f32>,
    pub ang: na::Vector3<f32>,
}

pub fn new(x: f32, y: f32, z: f32) -> Physics {

    Physics {
        pos: na::Vector3::new(x, y, z),
        vel: na::Vector3::zeros(),
        rot: na::Vector3::zeros(),
        //rot: na::Rotation3::new(na::Vector3::zeros()),
        ang: na::Vector3::zeros(),
    }
}

impl Physics {
    pub fn move_(&mut self, t: f32) {
        self.pos += self.vel * t;
        self.rot += self.ang * t;
        //self.rot *= na::Rotation3::new(self.ang * t);
        //self.rot.renormalize();
    }

    pub fn direction(&self) -> (na::Vector3<f32>, na::Vector3<f32>, na::Vector3<f32>) {
        let mat = self.mat_rotation();

        let hx = mat * na::Vector3::x().to_homogeneous();
        let hy = mat * na::Vector3::y().to_homogeneous();
        let hz = mat * na::Vector3::z().to_homogeneous();

        let x = na::Vector3::from_homogeneous(hx).unwrap();
        let y = na::Vector3::from_homogeneous(hy).unwrap();
        let z = na::Vector3::from_homogeneous(hz).unwrap();

        (x, y, z,)
    }

    pub fn mat_model(&self) -> na::Isometry3<f32> {
        //self.mat_translation() * self.mat_rotation()
        na::Isometry3::new(self.pos, self.rot)
    }

    pub fn mat_translation(&self) -> na::Matrix4<f32> {
        na::Translation::from(self.pos).to_homogeneous()
    }

    pub fn mat_rotation(&self) -> na::Matrix4<f32> {
        na::Matrix4::from(na::Rotation3::new(self.rot))
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
