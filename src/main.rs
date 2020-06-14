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
    Continue,
    Up,
    Down,
}

fn handle_events(events: &mut sdl2::EventPump) -> Action {
    for event in events.poll_iter() {

        match event {

            Event::Quit {..} => return Action::Quit,

            Event::KeyDown {keycode: Some(keycode), ..} => {
                if keycode == Keycode::Escape {
                    return Action::Quit;
                }

                if keycode == Keycode::W {
                    return Action::Up;
                }

                if keycode == Keycode::S {
                    return Action::Down;
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
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(16);

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

    let red: fn(i32) -> gfx::Color = |i| { [1.0, 0.2, 0.2] };
    let green: fn(i32) -> gfx::Color = |i| { [0.2, 0.8, 0.2] };
    let blue: fn(i32) -> gfx::Color = |i| { [0.2, 0.2, 1.0] };

    let mut f1 = gfx::rectangle::new(
        0.0,
        0.0,
        5.0,
        10.0,
        10.0,
        red,
    );

    let mut f2 = gfx::rectangle::new(
        -5.0,
        0.0,
        0.0,
        10.0,
        10.0,
        green,
    );
    f2.obj.rot.y = std::f32::consts::PI / 2.0;

    let mut f3 = gfx::rectangle::new(
        0.0,
        5.0,
        0.0,
        10.0,
        10.0,
        blue,
    );
    f3.obj.rot.x = std::f32::consts::PI / 2.0;

    let mut f4 = gfx::rectangle::new(
        0.0,
        0.0,
        -5.0,
        10.0,
        10.0,
        red,
    );

    let mut f5 = gfx::rectangle::new(
        5.0,
        0.0,
        0.0,
        10.0,
        10.0,
        green,
    );
    f5.obj.rot.y = std::f32::consts::PI / 2.0;

    let mut f6 = gfx::rectangle::new(
        0.0,
        -5.0,
        0.0,
        10.0,
        10.0,
        blue,
    );
    f6.obj.rot.x = std::f32::consts::PI / 2.0;

    let x = gfx::rectangle::new(
        0.0,
        0.0,
        0.0,
        1000.0,
        0.2,
        |i| { [1.0, 0.0, 0.0] },
    );

    let y = gfx::rectangle::new(
        0.0,
        0.0,
        0.0,
        0.2,
        1000.0,
        |i| { [0.0, 1.0, 0.0] },
    );

    let mut z = gfx::rectangle::new(
        0.0,
        0.0,
        0.0,
        0.2,
        1000.0,
        |i| { [0.0, 0.0, 1.0] },
    );
    z.obj.rot.x = std::f32::consts::PI / 2.0;

    let vs_src = include_str!("shaders/vertex.glsl");
    let fs_src = include_str!("shaders/fragment.glsl");

    let vs = gfx::shader::compile_shader(vs_src, gfx::shader::Type::Vertex)?;
    let fs = gfx::shader::compile_shader(fs_src, gfx::shader::Type::Fragment)?;

    let program = gfx::shader::link_program(vs, fs)?;

    let (uw, uh) = window.drawable_size();

    let width: i32 = i32::try_from(uw).expect("cant cast width");
    let height: i32 = i32::try_from(uh).expect("cant cast height");

    let aspect = width as f32 / height as f32;
    let fov = std::f32::consts::PI / 4.0;
    let mut camera = gfx::camera::new(0.0, 5.0, 50.0, aspect, fov);


    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let clock = timer.ticks() as f32 / 1000.0;

            camera.obj.pos.x = clock.sin() * 20.0;
            camera.look_at(&na::Point3::new(0.0, 0.0, 0.0));

            let params = gfx::render::Params{
                camera: camera.transformation(),
                program,
                clock,
                width,
                height
            };

            x.render(&params);
            y.render(&params);
            z.render(&params);

            f1.render(&params);
            f2.render(&params);
            f3.render(&params);
            f4.render(&params);
            f5.render(&params);
            f6.render(&params);

            //let mut rects = vec![&mut red, &mut green, &mut blue];

        }

        window.gl_swap_window();
    }

    Ok(())
}
