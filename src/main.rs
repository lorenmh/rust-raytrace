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
            _rng.gen_range(-1.0, 1.0),
            _rng.gen_range(-1.0, 1.0),
            _rng.gen_range(-1.0, 1.0),
        );

        cubes.push(c);
    }

    //let mut x = shapes::rectangle::new(
    //    0.0,
    //    0.0,
    //    0.0,
    //    1000.0,
    //    0.2,
    //    |i| { [1.0, 0.0, 0.0] },
    //);

    //let mut y = shapes::rectangle::new(
    //    0.0,
    //    0.0,
    //    0.0,
    //    0.2,
    //    1000.0,
    //    |i| { [0.0, 1.0, 0.0] },
    //);

    //let mut z = shapes::rectangle::new(
    //    0.0,
    //    0.0,
    //    0.0,
    //    0.2,
    //    1000.0,
    //    |i| { [0.0, 0.0, 1.0] },
    //);
    //z.phys.rot.x = std::f32::consts::PI / 2.0;

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

    //camera.look_at(&na::Point3::new(0.0, 0.0, 0.0));

    'main: loop {
        let now = timer.ticks();
        let delta = now - tick;
        tick = now;

        let delta_v = 1.0;
        let delta_a = 0.05;

        match input::handle_events(&mut events) {
            input::Action::Quit => break 'main,
            input::Action::PanUp => camera.phys.ang += na::Vector3::new(-delta_a, 0.0, 0.0),
            input::Action::PanDown => camera.phys.ang += na::Vector3::new(delta_a, 0.0, 0.0),
            input::Action::PanRight => camera.phys.ang += na::Vector3::new(0.0, delta_a, 0.0),
            input::Action::PanLeft => camera.phys.ang += na::Vector3::new(0.0, -delta_a, 0.0),
            input::Action::Forward => camera.phys.vel += na::Vector3::new(0.00, 0.0, -delta_v),
            input::Action::Backward => camera.phys.vel += na::Vector3::new(0.00, 0.0, delta_v),
            input::Action::Right => camera.phys.vel += na::Vector3::new(delta_v, 0.0, 0.0),
            input::Action::Left => camera.phys.vel += na::Vector3::new(-delta_v, 0.0, 0.0),
            _ => {}
        }

        unsafe {
            gl::ClearColor(0.05, 0.05, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let clock = delta as f32 / 1000.0;

            camera.phys.move_(clock);

            let params = gfx::render::Params{
                camera: camera.transformation(),
                program,
                clock,
                width,
                height
            };

            //x.render(&params);
            //y.render(&params);
            //z.render(&params);

            for mut c in cubes.iter_mut() {
                c.phys.vel += na::Vector3::new(
                        _rng.gen_range(-0.05, 0.05),
                        _rng.gen_range(-0.05,0.05),
                        _rng.gen_range(-0.05,0.05),
                    );
                c.phys.move_(clock);
                c.render(&params);
            }

            //let mut rects = vec![&mut red, &mut green, &mut blue];

        }

        window.gl_swap_window();
    }

    Ok(())
}
