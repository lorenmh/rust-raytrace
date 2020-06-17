use std::ffi::CString;
use nalgebra as na;
use gl::types::{GLfloat, GLsizeiptr, GLuint, GLint, GLboolean, GLvoid};
use std::convert::TryFrom;

pub struct Params {
    pub program: crate::gfx::shader::Program,
    pub camera: na::Matrix4<f32>,
    pub clock: f32,
    pub width: i32,
    pub height: i32,
}

pub struct Renderer {
    pub scale: f32,
    pub mesh: crate::gfx::Mesh,
    pub color: crate::gfx::ColorFn,
    vao: u32,
    vbo: u32,
}

impl Renderer {
    pub fn render(&mut self, phys: &crate::physics::Physics, params: &Params) -> Result<(), std::string::String> {
        let m = self.transformation(phys);

        unsafe {
            gl::UseProgram(params.program);

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            let uniform_model_id = CString::new("model").expect("CString::new failed");
            let uniform_model = gl::GetUniformLocation(params.program, uniform_model_id.as_ptr());
            gl::UniformMatrix4fv(uniform_model, 1, gl::FALSE, std::mem::transmute(&m[0]));

            let uniform_camera_id = CString::new("camera").expect("CString::new failed");
            let uniform_camera = gl::GetUniformLocation(params.program, uniform_camera_id.as_ptr());
            gl::UniformMatrix4fv(uniform_camera, 1, gl::FALSE, std::mem::transmute(&params.camera[0]));

            // Use shader program
            let uniform_clock_id = CString::new("clock").expect("CString::new failed");
            let uniform_clock = gl::GetUniformLocation(params.program, uniform_clock_id.as_ptr());
            gl::Uniform1f(uniform_clock, params.clock);

            let uniform_dimensions_id = CString::new("dimensions").expect("CString::new failed");
            let uniform_dimensions = gl::GetUniformLocation(params.program, uniform_dimensions_id.as_ptr());
            gl::Uniform2i(uniform_dimensions, params.width as GLint, params.height as GLint);

            // Specify the layout of the vertex data
            let attrib_position_id = CString::new("attribPosition").expect("CString:new failed");
            let attrib_color_id = CString::new("attribColor").expect("CString:new failed");

            let attrib_position = gl::GetAttribLocation(params.program, attrib_position_id.as_ptr());
            let attrib_color = gl::GetAttribLocation(params.program, attrib_color_id.as_ptr());

            gl::VertexAttribPointer(
                attrib_position as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (6 * std::mem::size_of::<GLfloat>()) as GLint,
                (0 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
            );

            gl::VertexAttribPointer(
                attrib_color as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (6 * std::mem::size_of::<GLfloat>()) as GLint,
                (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
            );

            gl::EnableVertexAttribArray(attrib_position as GLuint);
            gl::EnableVertexAttribArray(attrib_color as GLuint);

            let frag_data_id = CString::new("FragColor").expect("CString:new failed");
            gl::BindFragDataLocation(params.program, 0, frag_data_id.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.len() as i32 * 3);

            gl::UseProgram(0);
        }

        return Ok(());
    }

    fn transformation(&self, phys: &crate::physics::Physics) -> std::vec::Vec<GLfloat> {
        let m = self.mat_model(phys);

        std::vec::Vec::from(m.as_slice())
    }

    fn mat_model(&self, phys: &crate::physics::Physics) -> na::Matrix4<f32> {
        phys.mat_translation() * phys.mat_rotation() * self.mat_scale()
    }

    fn vertices(&self) -> std::vec::Vec<GLfloat> {
        let mut v: Vec<GLfloat> = Vec::with_capacity(self.mesh.len());

        let color_fn = &self.color;

        // iterate over triangles
        let mut counter: i32 = 0;
        for  &t in self.mesh.iter() {

            // iterate over points in triangles
            for &p in &t {
                v.extend(vec![
                    p.x as GLfloat,
                    p.y as GLfloat,
                    p.z as GLfloat,
                ]);

                let color = color_fn(counter);
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
        let s = self.scale;

        // column major format
        na::Matrix4::from_vec(vec![
            s, 0.0, 0.0, 0.0,
            0.0,   s, 0.0, 0.0,
            0.0, 0.0,   s, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    fn init(&mut self) {
        let v = self.vertices();
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                std::mem::transmute(&v[0]),
                gl::STATIC_DRAW,
            );
        }

        self.vao = vao;
        self.vbo = vbo;
    }
}

pub fn new(scale: f32, mesh: crate::gfx::Mesh, color: crate::gfx::ColorFn) -> Renderer {
    let mut r = Renderer{
        scale,
        mesh,
        color,
        vao: 0,
        vbo: 0,
    };

    r.init();

    r
}
