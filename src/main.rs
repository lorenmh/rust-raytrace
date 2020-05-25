use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::time::Duration;
use std::thread::sleep;
use std::iter::Sum;
use ::bounded_vec_deque::BoundedVecDeque;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MARGIN: i16 = 20;

const MARGIN_COLOR: pixels::Color = pixels::Color::RGB(255,100,100);
const BG_COLOR: pixels::Color = pixels::Color::RGB(15,15,15);

const TARGET_FPS: u32 = 60;
const TARGET_INTERVAL: u32 = 1000 / TARGET_FPS;

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

fn render_scene(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    delta: u32
) {

}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsys = sdl_context.video()?;
    let window = video_subsys.window("rust-sdl2_gfx: draw line & FPSManager", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;

    let mut timer = sdl_context.timer()?;
    let mut tick: u32 = 0;
    let mut counter = 0;
    let mut sleep_micro = (TARGET_INTERVAL as u64) * 1000;

    let mut deltas: BoundedVecDeque<u32> = BoundedVecDeque::new(10);

    'main: loop {
        counter += 1;

        match handle_events(&mut events) {
            Action::Quit => {
                break 'main;
            }
            Action::Continue => {}
        }

        canvas.set_draw_color(MARGIN_COLOR);
        canvas.clear();

        // background
        canvas.box_(
            MARGIN,
            MARGIN,
            (WIDTH as i16) - MARGIN,
            (HEIGHT as i16) - MARGIN,
            BG_COLOR
        );

        let now = timer.ticks();
        let delta = now - tick;
        deltas.push_back(delta);
        tick = now;

        // scene
        render_scene(&mut canvas, delta);

        canvas.present();

        //let sleep_millis = match TARGET_INTERVAL.checked_sub(delta) {
        //    Some(x) => x,
        //    None => 0
        //};

        sleep(Duration::from_micros(sleep_micro));

        if counter % 20 == 0 {
            let len = deltas.len() as f32;

            if len == 0.0 {
                continue;
            }

            let sum: u32 = deltas.iter().sum();
            let actual: f32 = sum as f32 / len;

            if actual > TARGET_INTERVAL as f32 {
                sleep_micro = match sleep_micro.checked_sub(100) {
                    Some(x) => x,
                    None => 0
                };
            } else {
                sleep_micro += 100;
            }

            println!(
                "fps: {}\nsleep: {}\n",
                1000.0 / actual,
                (sleep_micro as f32) / 1000.0
            );
        }

    }

    Ok(())
}
