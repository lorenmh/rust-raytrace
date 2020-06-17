use gl;
use std::ffi::CString;
use gl::types::{GLchar, GLint, GLuint};

pub type ShaderProgram = GLuint;
pub type Program = GLuint;

#[repr(u32)]
pub enum Type {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

pub fn link_program(vs: ShaderProgram, fs: ShaderProgram) -> Result<Program, std::string::String> {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::new();
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            let s = std::str::from_utf8(buf.as_slice()).ok().expect("ProgramInfoLog not valid utf8");
            return Err(s.to_string())
        }
        Ok(program as Program)
    }
}


pub fn compile_shader(glsl: &str, typ: Type) -> Result<ShaderProgram, std::string::String> {
    let program;

    unsafe {
        program = gl::CreateShader(typ as GLuint);
        let c_str = CString::new(glsl).expect("couldnt convert to c-string");
        gl::ShaderSource(program, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(program);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(program, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::new();
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(program, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            let s = std::str::from_utf8(buf.as_slice()).ok().expect("ShaderInfoLog not valid utf8");
            return Err(s.to_string());
        }
    }
    Ok(program)
}

//pub fn compile_shader()