use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use rand::Rng;

mod gfx;

const WIDTH: i16 = 800;
const HEIGHT: i16 = 600;
const MARGIN: i16 = 20;

const MARGIN_COLOR: pixels::Color = pixels::Color::RGB(15,15,15);
const BG_COLOR: pixels::Color = pixels::Color::RGB(15,15,15);

const RECT_SIZE: f32 = 5.;
const RECT_SPEED: f32 = 3.;

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

fn render_scene(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    rects: &mut Vec<Rect>,
    delta: u32
) {
    for rect in rects.iter_mut() {
        let mut _x: f32 = rect.x + rect.vec[0];
        let mut _y: f32 = rect.y + rect.vec[1];

        if _x + rect.width >= (WIDTH - MARGIN) as f32 {
            _x = ((WIDTH - MARGIN) as f32) - rect.width;
            rect.vec[0] = -rect.vec[0];
        }

        if _x <= MARGIN as f32 {
            _x = MARGIN as f32;
            rect.vec[0] = -rect.vec[0];
        }

        if _y + rect.height >= (HEIGHT - MARGIN) as f32 {
            _y = ((HEIGHT - MARGIN) as f32) - rect.height;
            rect.vec[1] = -rect.vec[1];
        }

        if _y <= MARGIN as f32 {
            _y = MARGIN as f32;
            rect.vec[1] = -rect.vec[1];
        }

        rect.x = _x;
        rect.y = _y;

        canvas.box_(
            rect.x as i16,
            rect.y as i16,
            (rect.x + rect.width) as i16,
            (rect.y + rect.height) as i16,
            rect.color,
        );
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window("ray trace", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;

    let mut timer = sdl_context.timer()?;
    let mut tick: u32 = 0;

    let mut rng = rand::thread_rng();

    let mut rects: Vec<Rect> = Vec::new();
    
    for _ in 0..1000 {
        let phi: f32 = rng.gen_range(0., 2. * std::f32::consts::PI);

        let rect = Rect{
            x: rng.gen_range(MARGIN as f32, (WIDTH - MARGIN) as f32 - 5.),
            y: rng.gen_range(MARGIN as f32, (HEIGHT - MARGIN) as f32 - 5.),
            width: 5.,
            height: 5.,
            color: pixels::Color::RGB(
                rng.gen_range(150, 250),
                rng.gen_range(130, 230),
                rng.gen_range(90, 190),
            ),
            vec: [
                RECT_SPEED * phi.cos(),
                RECT_SPEED * phi.sin(),
            ],
        };

        rects.push(rect);
    };

    gfx::camera::hello();

    'main: loop {
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
            WIDTH - MARGIN,
            HEIGHT - MARGIN,
            BG_COLOR
        )?;

        let now = timer.ticks();
        let delta = now - tick;
        tick = now;

        // scene
        render_scene(&mut canvas, &mut rects, delta);

        canvas.present();
    }

    Ok(())
}
