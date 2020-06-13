use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use rand::Rng;
use sdl2::video::GLProfile;
use std::ffi::CString;
use gl;
use gl::types::{GLfloat, GLsizeiptr, GLuint, GLint, GLboolean, GLvoid};
use std::convert::TryFrom;

mod gfx;

// traits
use gfx::Render;
use std::string::ToString;

const WIDTH: i16 = 800;
const HEIGHT: i16 = 600;

enum Action {
    Quit,
    Continue
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: pixels::Color,
    vec: [f32; 2],
}

static VERTEX_DATA: [GLfloat; 15] = [
    0.0,  0.5,      1.0, 0.0, 0.0,
    0.5, -0.5,      0.0, 1.0, 0.0,
    -0.5, -0.5,     0.0, 0.0, 1.0,
];

fn handle_events(events: &mut sdl2::EventPump) -> Action {
    for event in events.poll_iter() {

        match event {

            Event::Quit {..} => return Action::Quit,

            Event::KeyDown {keycode: Some(keycode), ..} => {
                if keycode == Keycode::Escape {
                    return Action::Quit;
                }
            }

            _ => {}
        }
    }

    return Action::Continue
}

//fn render_scene

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsys = sdl_context.video()?;

    let gl_attr = video_subsys.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsys
        .window("gfx", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsys.gl_get_proc_address(name) as *const _);

    let mut events = sdl_context.event_pump()?;

    let mut timer = sdl_context.timer()?;
    let mut tick: u32 = 0;

    let mut _rng = rand::thread_rng();

    let mut objects: Vec<gfx::object::Object> = Vec::new();
    let rect = gfx::rectangle::new(0.0, 0.0, 0.7, 0.35, [255, 155, 155]);
    println!("{}", rect);
    println!("{:?}", rect.vertices());
    println!("{:?}", rect.transformation());

    let vs_src = include_str!("shaders/vertex.glsl");
    let fs_src = include_str!("shaders/fragment.glsl");

    let vs = gfx::shader::compile_shader(vs_src, gfx::shader::Type::Vertex)?;
    let fs = gfx::shader::compile_shader(fs_src, gfx::shader::Type::Fragment)?;

    let program = gfx::shader::link_program(vs, fs)?;

    let mut vao = 0;
    let mut vbo = 0;

    let (uw, uh) = window.drawable_size();
    let width: i32 = i32::try_from(uw).expect("cant cast width");
    let height: i32 = i32::try_from(uh).expect("cant cast height");
    let numPixels = width * height;

    let dataBuffer = vec![0; (numPixels as usize) * 4];

    let mut data = 0;

    let mut uniformClock: GLint;
    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW,
        );

        // r/w data
        gl::GenBuffers(1, &mut data);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, data);
        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (uw * uh * 4 * std::mem::size_of::<GLfloat>() as u32) as GLsizeiptr,
            std::mem::transmute(&dataBuffer[0]),
            gl::DYNAMIC_COPY,
        );

        gl::UseProgram(program);

        // Use shader program
        let uniformClockID = CString::new("clock").expect("CString::new failed");
        uniformClock = gl::GetUniformLocation(program, uniformClockID.as_ptr());

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
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (5 * std::mem::size_of::<GLfloat>()) as GLint,
            (0 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );

        //let offsetPtr: *mut std::ffi::c_void = &mut offset as *mut _ as *mut std::ffi::c_void;
        gl::VertexAttribPointer(
            attribColor as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (5 * std::mem::size_of::<GLfloat>()) as GLint,
            (2 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );

        gl::EnableVertexAttribArray(attribPosition as GLuint);
        gl::EnableVertexAttribArray(attribColor as GLuint);

        let fragDataID = CString::new("FragColor").expect("CString:new failed");
        gl::BindFragDataLocation(program, 0, fragDataID.as_ptr());

    }

    'main: loop {
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        match handle_events(&mut events) {
            Action::Quit => {
                break 'main;
            }
            Action::Continue => {}
        }

        let now = timer.ticks();
        let delta = now - tick;
        tick = now;

        unsafe {
            let clock = 0.8 + (timer.ticks() as f32 / 100.0).sin() / 10.0;
            gl::Uniform1f(uniformClock, clock);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // scene
        //render_scene(&mut canvas, &mut objects, delta);

        window.gl_swap_window();
    }

    Ok(())
}
