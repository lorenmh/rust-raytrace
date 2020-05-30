use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use rand::Rng;
use sdl2::video::GLProfile;
use std::ffi::CString;
use gl;
use gl::types::{GLfloat, GLsizeiptr, GLuint, GLboolean};

mod gfx;

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

static VERTEX_DATA: [GLfloat; 6] = [
    0.0,  0.5,
    0.5, -0.5,
    -0.5, -0.5
];

static VS_SRC: &'static str =
    "#version 150\n\
    in vec2 position;\n\
    void main() {\n\
    gl_Position = vec4(position, 0.0, 1.0);\n\
    }";

static FS_SRC: &'static str =
    "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
       out_color = vec4(1.0, 1.0, 1.0, 1.0);\n\
    }";

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

    let vs = gfx::shader::compile_shader(VS_SRC, gfx::shader::Type::Vertex)?;
    let fs = gfx::shader::compile_shader(FS_SRC, gfx::shader::Type::Fragment)?;

    let program = gfx::shader::link_program(vs, fs)?;

    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                       std::mem::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        let id1 = CString::new("out_color").expect("foop");
        gl::BindFragDataLocation(program, 0, id1.as_ptr());

        // Specify the layout of the vertex data
        let id2 = CString::new("position").expect("boop");
        let pos_attr = gl::GetAttribLocation(program, id2.as_ptr());

        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());
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
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // scene
        //render_scene(&mut canvas, &mut objects, delta);

        window.gl_swap_window();
    }

    Ok(())
}
