use nalgebra as na;
use gl;
use std::ffi::CString;
use gl::types::{GLfloat, GLsizeiptr, GLuint, GLint, GLboolean, GLvoid};

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

    fn render(&self, program: shader::Program, clock: f32, width: i32, height: i32) -> Result<(), std::string::String> {
        let v = self.vertices();
        let m = self.transformation();

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                std::mem::transmute(&v[0]),
                gl::STATIC_DRAW,
            );

            gl::UseProgram(program);

            let uniformModelID = CString::new("model").expect("CString::new failed");
            let uniformModel = gl::GetUniformLocation(program, uniformModelID.as_ptr());
            gl::UniformMatrix4fv(uniformModel, 1, gl::FALSE, std::mem::transmute(&m[0]));

            // Use shader program
            let uniformClockID = CString::new("clock").expect("CString::new failed");
            let uniformClock = gl::GetUniformLocation(program, uniformClockID.as_ptr());
            gl::Uniform1f(uniformClock, clock);

            let uniformDimensionsID= CString::new("dimensions").expect("CString::new failed");
            let uniformDimensions = gl::GetUniformLocation(program, uniformDimensionsID.as_ptr());
            gl::Uniform2i(uniformDimensions, width as GLint, height as GLint);

            // Specify the layout of the vertex data
            let attribPositionID = CString::new("attribPosition").expect("CString:new failed");
            let attribColorID = CString::new("attribColor").expect("CString:new failed");

            let attribPosition = gl::GetAttribLocation(program, attribPositionID.as_ptr());
            let attribColor = gl::GetAttribLocation(program, attribColorID.as_ptr());

            gl::VertexAttribPointer(
                attribPosition as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (6 * std::mem::size_of::<GLfloat>()) as GLint,
                (0 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
            );

            gl::VertexAttribPointer(
                attribColor as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (6 * std::mem::size_of::<GLfloat>()) as GLint,
                (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(attribPosition as GLuint);
            gl::EnableVertexAttribArray(attribColor as GLuint);

            let fragDataID = CString::new("FragColor").expect("CString:new failed");
            gl::BindFragDataLocation(program, 0, fragDataID.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::UseProgram(0);
        }

        return Ok(());
    }

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

        // column major format
        na::Matrix4::from_vec(vec![
              s, 0.0, 0.0, 0.0,
            0.0,   s, 0.0, 0.0,
            0.0, 0.0,   s, 0.0,
            p.x, p.y, p.x, 1.0,
        ])
    }

    fn mat_rotation(&self) -> na::Matrix4<f32> {
        na::Matrix4::new_rotation(self.rotation())
        //na::Matrix4::new_rotation(na::Vector3::new(0.0, 0.0, std::f32::consts::PI / 2.0))
    }
}

trait Velocity {
    fn velocity() -> na::Vector3<f32>;
}
