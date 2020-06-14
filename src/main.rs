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
use nalgebra as na;

mod gfx;

// traits
use gfx::render::Renderable;
use std::string::ToString;

const WIDTH: i16 = 800;
const HEIGHT: i16 = 600;

enum Action {
    Quit,
    Continue
}

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

    let mut rect0 = gfx::rectangle::new(
        -0.5,
        0.5,
        0.5,
        1.0,
        1.5,
        |i| { if (i / 3) == 0 { [0.0, 0.1, 0.0] } else { [0.4, 0.3, 0.2] } }
    );

    let mut rect1 = gfx::rectangle::new(
        0.0,
        0.0,
        0.0,
        0.7,
        0.35,
        |i| { if (i / 3) == 0 { [0.1, 0.3, 0.2] } else { [0.4, 0.8, 0.64] } }
    );
    rect1.renderer.scale = 1.5;

    let mut rect2 = gfx::rectangle::new(
        0.0,
        0.0,
        0.25,
        0.7,
        0.35,
        |i| { if (i / 3) == 0 { [0.6, 0.7, 0.8] } else { [0.5, 0.6, 0.7] } }
    );


    let vs_src = include_str!("shaders/vertex.glsl");
    let fs_src = include_str!("shaders/fragment.glsl");

    let vs = gfx::shader::compile_shader(vs_src, gfx::shader::Type::Vertex)?;
    let fs = gfx::shader::compile_shader(fs_src, gfx::shader::Type::Fragment)?;

    let program = gfx::shader::link_program(vs, fs)?;

    let (uw, uh) = window.drawable_size();

    let width: i32 = i32::try_from(uw).expect("cant cast width");
    let height: i32 = i32::try_from(uh).expect("cant cast height");

    unsafe {
        //gl::Enable(gl::DEPTH_TEST);
        //gl::DepthFunc(gl::ALWAYS);
    }

    'main: loop {
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
            gl::ClearColor(0.05, 0.05, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);// | gl::DEPTH_BUFFER_BIT);

            let clock = 0.8 + (timer.ticks() as f32 / 100.0).sin() / 10.0;

            let params = gfx::render::Params{
                program, clock, width, height
            };

            rect0.render(&params);
            rect1.render(&params);
            rect2.render(&params);

            rect0.obj.rot.z += std::f32::consts::PI / 200.0;
            rect0.renderer.scale = (f32::sin(timer.ticks() as f32 / 500.0) / 4.0) + 0.75;

            rect1.obj.pos = na::Vector3::new(
                f32::sin(timer.ticks() as f32 / 1000.0),
                f32::sin(timer.ticks() as f32 / 1000.0),
                0.0
            );
            rect1.obj.rot.z -= std::f32::consts::PI / 300.0;

            rect2.obj.pos.x = f32::sin(timer.ticks() as f32 / 800.0);
            rect2.obj.rot.x -= delta as f32 / 500.0;
        }

        window.gl_swap_window();
    }

    Ok(())
}
