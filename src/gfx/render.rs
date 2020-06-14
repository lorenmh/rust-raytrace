use std::ffi::CString;
use nalgebra as na;
use gl::types::{GLfloat, GLsizeiptr, GLuint, GLint, GLboolean, GLvoid};

pub struct Params {
    pub program: crate::gfx::shader::Program,
    pub camera: na::Matrix4<f32>,
    pub clock: f32,
    pub width: i32,
    pub height: i32,
}

pub trait Renderable {
    fn scale(&self) -> f32;
    fn mesh(&self) -> &crate::gfx::Mesh;
    fn color(&self) -> fn(i32) -> crate::gfx::Color;

    fn render(&self, obj: &crate::gfx::object::Object, params: &Params) -> Result<(), std::string::String> {
        let v = self.vertices();
        let m = self.transformation(obj);

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

            gl::UseProgram(params.program);

            let uniformModelID = CString::new("model").expect("CString::new failed");
            let uniformModel = gl::GetUniformLocation(params.program, uniformModelID.as_ptr());
            gl::UniformMatrix4fv(uniformModel, 1, gl::FALSE, std::mem::transmute(&m[0]));

            let uniformCameraID = CString::new("camera").expect("CString::new failed");
            let uniformCamera = gl::GetUniformLocation(params.program, uniformCameraID.as_ptr());
            gl::UniformMatrix4fv(uniformCamera, 1, gl::FALSE, std::mem::transmute(&params.camera[0]));

            // Use shader program
            let uniformClockID = CString::new("clock").expect("CString::new failed");
            let uniformClock = gl::GetUniformLocation(params.program, uniformClockID.as_ptr());
            gl::Uniform1f(uniformClock, params.clock);

            let uniformDimensionsID= CString::new("dimensions").expect("CString::new failed");
            let uniformDimensions = gl::GetUniformLocation(params.program, uniformDimensionsID.as_ptr());
            gl::Uniform2i(uniformDimensions, params.width as GLint, params.height as GLint);

            // Specify the layout of the vertex data
            let attribPositionID = CString::new("attribPosition").expect("CString:new failed");
            let attribColorID = CString::new("attribColor").expect("CString:new failed");

            let attribPosition = gl::GetAttribLocation(params.program, attribPositionID.as_ptr());
            let attribColor = gl::GetAttribLocation(params.program, attribColorID.as_ptr());

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
            gl::BindFragDataLocation(params.program, 0, fragDataID.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::UseProgram(0);
        }

        return Ok(());
    }

    fn transformation(&self, obj: &crate::gfx::object::Object) -> std::vec::Vec<GLfloat> {
        let m = self.mat_model(obj);

        std::vec::Vec::from(m.as_slice())
    }

    fn mat_model(&self, obj: &crate::gfx::object::Object) -> na::Matrix4<f32> {
        obj.mat_translation() * obj.mat_rotation() * self.mat_scale()
    }

    fn vertices(&self) -> std::vec::Vec<GLfloat> {
        let mut v: Vec<GLfloat> = Vec::with_capacity(self.mesh().len());

        let a = self.mesh();

        let colorFn = self.color();

        // iterate over triangles
        let mut counter: i32 = 0;
        for  &t in self.mesh() {

            // iterate over points in triangles
            for &p in &t {
                v.extend(vec![
                    p.x as GLfloat,
                    p.y as GLfloat,
                    p.z as GLfloat,
                ]);

                let color = colorFn(counter);
                // interleaved vertex and color
                v.extend(vec![
                    color[0] as GLfloat,
                    color[1] as GLfloat,
                    color[2] as GLfloat,
                ]);

                counter += 1;
            }
        }

        v
    }

    fn mat_scale(&self) -> na::Matrix4<f32> {
        let s = self.scale();

        // column major format
        na::Matrix4::from_vec(vec![
            s, 0.0, 0.0, 0.0,
            0.0,   s, 0.0, 0.0,
            0.0, 0.0,   s, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }
}

pub struct Renderer {
    pub scale: f32,
    pub mesh: crate::gfx::Mesh,
    pub color: fn(i32) -> crate::gfx::Color,
}

impl Renderable for Renderer {
    fn scale(&self) -> f32 { self.scale }
    fn mesh(&self) -> &crate::gfx::Mesh { &self.mesh }
    fn color(&self) -> fn(i32) -> crate::gfx::Color { self.color }
}
