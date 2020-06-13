use nalgebra as na;
use gl::types::GLfloat;

pub mod camera;
pub mod object;
pub mod shader;
pub mod rectangle;

pub type Triangle = [na::Point3<f32>; 3];
pub type Mesh = std::vec::Vec<Triangle>;
pub type Color = [u8; 3];

pub trait Render {
    fn position(&self) -> na::Vector3<f32>;
    fn rotation(&self) -> na::Vector3<f32>;
    fn scale(&self) -> f32;
    fn mesh(&self) -> &Mesh;
    fn color(&self) -> Color;

    fn vertices(&self) -> std::vec::Vec<GLfloat> {
        let mut v: Vec<GLfloat> = Vec::with_capacity(self.mesh().len());

        let a = self.mesh();

        let color = self.color();
        let c: Vec<GLfloat> = vec![
            (color[0] as GLfloat) / 255.0,
            (color[1] as GLfloat) / 255.0,
            (color[2] as GLfloat) / 255.0,
        ];

        // iterate over triangles
        for &t in self.mesh() {

            // iterate over points in triangles
            for &p in &t {
                v.extend(vec![
                    p.x as GLfloat,
                    p.y as GLfloat,
                    p.z as GLfloat,
                ]);

                // interleaved vertex and color
                v.extend(&c);
            }
        }

        v
    }

    fn transformation(&self) -> std::vec::Vec<GLfloat> {
        let m = self.mat_model();

        std::vec::Vec::from(m.as_slice())
    }

    fn mat_model(&self) -> na::Matrix4<f32> {
        self.mat_translation() * self.mat_rotation()
    }

    fn mat_translation(&self) -> na::Matrix4<f32> {
        let p = self.position();
        let s = self.scale();

        na::Matrix4::from_vec(vec![
              s, 0.0, 0.0, p.x,
            0.0,   s, 0.0, p.y,
            0.0, 0.0,   s, p.z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    fn mat_rotation(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_rotation(self.rotation())
    }
}

trait Velocity {
    fn velocity() -> na::Vector3<f32>;
}
