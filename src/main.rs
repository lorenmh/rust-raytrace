use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use rand::Rng;
use sdl2::video::GLProfile;

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

    'main: loop {
        unsafe {
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
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

        // scene
        //render_scene(&mut canvas, &mut objects, delta);

        window.gl_swap_window();
    }

    Ok(())
}
