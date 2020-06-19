use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub enum Action {
    Quit,
    Continue,
    PanUp,
    PanDown,
    PanRight,
    PanLeft,
    Up,
    Down,
    Right,
    Left,
    Forward,
    Backward,
    Stop,
    YawLeft,
    YawRight,
    MouseMotion{x: i32, y: i32, dx: i32, dy: i32},
    TimeFaster,
    TimeSlower,
    BoxFaster,
    BoxSlower,
}

pub type Events = std::vec::Vec<Action>;

pub fn handle_events(events: &mut sdl2::EventPump) -> Events {
    let mut e = std::vec::Vec::<Action>::new();

    for event in events.poll_iter() {

        match event {

            Event::Quit {..} => e.push(Action::Quit),

            Event::MouseMotion {x, y, xrel, yrel, ..} => {
                e.push(Action::MouseMotion {x, y, dx: xrel, dy: yrel})
            },

            Event::KeyDown {keycode: Some(keycode), ..} => {
                e.push(match keycode {
                    Keycode::Escape => Action::Quit,
                    Keycode::W => Action::Forward,
                    Keycode::S => Action::Backward,
                    Keycode::D => Action::Right,
                    Keycode::A => Action::Left,

                    Keycode::Down => Action::PanDown,
                    Keycode::Up => Action::PanUp,
                    Keycode::Right => Action::PanRight,
                    Keycode::Left => Action::PanLeft,
                    Keycode::Space => Action::Stop,

                    Keycode::Q => Action::YawLeft,
                    Keycode::E => Action::YawRight,

                    Keycode::RightBracket => Action::TimeFaster,
                    Keycode::LeftBracket => Action::TimeSlower,
                    Keycode::Semicolon => Action::BoxFaster,
                    Keycode::Quote => Action::BoxSlower,


                    _ => Action::Continue,
                });
            }

            _ => {}
        }
    }

    return e;
}

