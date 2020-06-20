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

    let mut window = video_subsys
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

    let red: fn(i32) -> gfx::Color = |i| { if (i % 2) == 0 { [1.0, 0.2, 0.2] } else { [1.0, 0.4, 0.4] } };
    let green: fn(i32) -> gfx::Color = |i| { if (i % 2) == 0 { [0.3, 0.9, 0.4] } else { [0.5, 1.0, 0.6] } };
    let blue: fn(i32) -> gfx::Color = |i| { if (i % 2) == 0 { [0.0, 0.89, 0.91] } else { [0.2, 1.0, 1.0] } };

    let mut cubes: std::vec::Vec<shapes::cube::Cube> = vec![];
    for i in 0..3 {
        let c = if (i == 0) { red } else if (i == 1) { green } else { blue };
        for _ in 1..100 {
            let mut c = shapes::cube::new(
                i,
                _rng.gen_range(-50.0, 50.0),
                _rng.gen_range(-50.0, 50.0),
                _rng.gen_range(-50.0, 50.0),
                _rng.gen_range(0.30, 1.5),
                _rng.gen_range(0.30, 1.5),
                _rng.gen_range(0.30, 1.5),
                c,
            );

            c.phys.vel += na::Vector3::new(
                _rng.gen_range(-1.0, 1.0),
                _rng.gen_range(-1.0,1.0),
                _rng.gen_range(-1.0,1.0),
            );
            c.phys.ang = na::Vector3::new(
                _rng.gen_range(-0.5, 0.5),
                _rng.gen_range(-0.5, 0.5),
                _rng.gen_range(-0.5, 0.5),
            );

            cubes.push(c);
        }
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
    let mut camera = gfx::camera::new(0.0, 0.0, -150.0, aspect, fov);
    //camera.phys.vel = na::Vector3::z() * 20.0;

    let mut mouse = sdl_context.mouse();
    mouse.warp_mouse_in_window(&window, width / 2, height / 2);
    mouse.set_relative_mouse_mode(true);
    window.set_grab(true);

    let axes = shapes::axes::new();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::MULTISAMPLE);
    }

    let mut t_factor = 32.0;
    let mut speed_adjust = 0.0;

    let delta_v = 4.0;
    let delta_a = 0.05 * std::f32::consts::PI;
    let delta_m = 0.001 * std::f32::consts::PI;
    let delta_b = 0.2;
    let delta_g = 0.25;

    let mut t = 0.0;

    let mut t_save = 1.0;

    'main: loop {
        let now = timer.ticks();
        let delta = now - tick;
        tick = now;

        t += (delta as f32 / 100000.0) * t_factor;


        let (dir_x, dir_y, dir_z,) = camera.phys.direction();

        for event in input::handle_events(&mut events).iter() {
            match event {
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

                input::Action::TimeFaster => {
                    t_factor = if (t_factor == 0.0) { 1.0 } else { t_factor * 2.0 };
                },
                input::Action::TimeSlower => t_factor /= 2.0,

                input::Action::BoxFaster => speed_adjust += 1.0,
                input::Action::BoxSlower => speed_adjust -= 1.0,

                input::Action::Stop => {
                    camera.phys.vel = na::Vector3::zeros();
                    camera.phys.ang = na::Vector3::zeros();
                },

                input::Action::TimeStop => {
                    if (t_factor == 0.0) {
                        t_factor = t_save;
                    } else {
                        camera.phys.vel = na::Vector3::zeros();
                        camera.phys.ang = na::Vector3::zeros();
                        t_save = t_factor;
                        t_factor = 0.0;
                    }
                },

                input::Action::MouseMotion {dx, dy, ..} => {
                    camera.phys.rot *= na::Rotation3::new(na::Vector3::new(
                        *dy as f32 * delta_m,
                        -dx as f32 * delta_m,
                        0.0,
                    ));
                },

                _ => {}

            }
        }

        unsafe {
            gl::ClearColor(0.05, 0.05, 0.1, 1.0);
            //gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            let clock = (delta as f32 / 1000.0);

            camera.phys.move_(clock);

            let params = gfx::render::Params{
                program,
                camera: camera.transformation(),
                width,
                height,
                clock,
            };

            //axes.render(&params);

            let mut centroid0: na::Vector3<f32> = na::Vector3::zeros();
            let mut centroid1: na::Vector3<f32> = na::Vector3::zeros();
            let mut centroid2: na::Vector3<f32> = na::Vector3::zeros();

            let mut n0 = 0.0;
            let mut n1 = 0.0;
            let mut n2 = 0.0;

            for mut c in cubes.iter() {
                if c.id == 0 {
                    centroid0 += c.phys.pos;
                    n0 += 1.0;
                } else if c.id == 1 {
                    centroid1 += c.phys.pos;
                    n1 += 1.0;
                } else if c.id == 2 {
                    centroid2 += c.phys.pos;
                    n2 += 1.0;
                }
            }
            centroid0 /= n0;
            centroid1 /= n1;
            centroid2 /= n2;

            let amt = 20.0;

            let pi = 2.0 * std::f32::consts::FRAC_PI_3;
            let adj = |i: i32| {
                let r = ((t * 0.5) + (pi * i as f32));
                na::Vector3::new(
                    r.sin() * amt,
                    r.cos() * amt,
                    0.0,
                )
            };
            println!("{}", t);

            for mut c in cubes.iter_mut() {
                //let n = c.phys.pos + c.phys.vel;
                //let rot = na::Matrix4::from(na::Rotation3::rotation_between(&c.phys.pos, &centroid).unwrap());
                //let correction = na::Vector3::from_homogeneous(rot * c.phys.vel.to_homogeneous()).unwrap();
                //c.phys.vel += correction * 0.001;

                let mut adjust = na::Vector3::<f32>::zeros();
                let mut center = na::Vector3::<f32>::zeros();

                if c.id == 0 {
                    center = centroid0;
                }
                if c.id == 1 {
                    center = centroid1;
                }
                if c.id == 2 {
                    center = centroid2;
                }

                let adjust = adj(c.id);

                //let d = adjust - c.phys.pos;
                let mut b = (adjust - center) * 0.5;
                let mut d = center - c.phys.pos + b;
                let mut m = d.magnitude() as f32;
                if (m < 5.0) { m = 5.0 };
                let mut a = d * delta_g * 1.0 / (m * m);
                c.phys.vel += a * clock * t_factor;

                c.phys.vel -= c.phys.vel * speed_adjust * delta_b;

                c.phys.move_(clock * t_factor);
                c.render(&params);
            }
            //axes.render(&params);
        }


        speed_adjust = 0.0;

        window.gl_swap_window();
    }

    Ok(())
}
