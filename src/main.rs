use sdl2::keyboard::Keycode;
use rand::Rng;
use sdl2::video::GLProfile;
use std::ffi::CString;
use gl;
use gl::types::{GLfloat, GLsizeiptr, GLuint, GLint, GLboolean, GLvoid};
use std::convert::TryFrom;
use nalgebra as na;

mod gfx;
mod physics;
mod input;
mod shapes;

// traits
use std::string::ToString;

const WIDTH: i16 = 800;
const HEIGHT: i16 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsys = sdl_context.video()?;

    let gl_attr = video_subsys.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(8);

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

    let mut cubes: std::vec::Vec<shapes::cube::Cube> = vec![];
    for _ in 1..500 {
        let mut c = shapes::cube::new(
        _rng.gen_range(-10.0, 10.0),
            _rng.gen_range(-10.0, 10.0),
            _rng.gen_range(-10.0, 10.0),
            _rng.gen_range(0.15, 0.75),
            _rng.gen_range(0.15, 0.75),
            _rng.gen_range(0.15, 0.75),
            |i| -> gfx::Color {
                if (i / 6) == 0 {
                    [1.0,0.0,0.0]
                } else if (i / 6) == 1 {
                    [0.0,1.0,0.0]
                } else if (i / 6) == 2 {
                    [0.0,0.0,1.0]
                } else if (i / 6) == 3 {
                    [1.0,1.0,0.0]
                } else if (i / 6) == 4 {
                    [0.0,1.0,1.0]
                } else {
                    [1.0,0.0,1.0]
                }
            },
        );

        c.phys.ang = na::Vector3::new(
            _rng.gen_range(-2.0, 2.0),
            _rng.gen_range(-2.0,2.0),
            _rng.gen_range(-2.0,2.0),
        );

        cubes.push(c);
    }


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
    let mut camera = gfx::camera::new(0.0, 5.0, -100.0, aspect, fov);

    let axes = shapes::axes::new();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
    }

    'main: loop {
        let now = timer.ticks();
        let delta = now - tick;
        tick = now;

        let delta_v = 4.0;
        let delta_a = 0.05 * std::f32::consts::PI;

        let (dir_x, dir_y, dir_z,) = camera.phys.direction();


        match input::handle_events(&mut events) {
            input::Action::Quit => break 'main,

            input::Action::PanUp => camera.phys.ang += na::Vector3::x() * -delta_a,
            input::Action::PanDown => camera.phys.ang += na::Vector3::x() * delta_a,
            input::Action::PanLeft => camera.phys.ang += na::Vector3::y() * delta_a,
            input::Action::PanRight => camera.phys.ang += na::Vector3::y() * -delta_a,
            input::Action::YawLeft => camera.phys.ang += na::Vector3::z() * -delta_a,
            input::Action::YawRight => camera.phys.ang += na::Vector3::z() * delta_a,

            input::Action::Forward => camera.phys.vel += dir_z * delta_v,
            input::Action::Backward => camera.phys.vel += dir_z * -delta_v,
            input::Action::Left => camera.phys.vel += dir_x * delta_v,
            input::Action::Right => camera.phys.vel += dir_x * -delta_v,

            input::Action::Stop => {
                camera.phys.vel = na::Vector3::zeros();
                camera.phys.ang = na::Vector3::zeros();
            },
            _ => {

            }
        }

        unsafe {
            gl::ClearColor(0.05, 0.05, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let clock = delta as f32 / 1000.0;

            camera.phys.move_(clock);

            let params = gfx::render::Params{
                program,
                camera: camera.transformation(),
                width,
                height,
                clock,
            };

            //axes.render(&params);

            let mut centroid: na::Vector3<f32> = na::Vector3::zeros();
            for mut c in cubes.iter() {
                centroid += c.phys.pos;
            }
            centroid /= cubes.len() as f32;

            for mut c in cubes.iter_mut() {
                c.phys.vel += na::Vector3::new(
                        _rng.gen_range(-0.1, 0.1),
                        _rng.gen_range(-0.1,0.1),
                        _rng.gen_range(-0.1,0.1),
                    );
                c.phys.vel += (centroid - c.phys.pos) * 0.0005;
                c.phys.move_(clock);
                c.render(&params);
            }
        }

        window.gl_swap_window();
    }

    Ok(())
}
