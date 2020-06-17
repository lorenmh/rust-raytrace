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
}



pub fn handle_events(events: &mut sdl2::EventPump) -> Action {
    for event in events.poll_iter() {

        match event {

            Event::Quit {..} => return Action::Quit,

            Event::KeyDown {keycode: Some(keycode), ..} => {
                return match keycode {
                    Keycode::Escape => Action::Quit,
                    Keycode::W => Action::Forward,
                    Keycode::S => Action::Backward,
                    Keycode::D => Action::Right,
                    Keycode::A => Action::Left,
                    Keycode::Down => Action::PanDown,
                    Keycode::Up => Action::PanUp,
                    Keycode::Right => Action::PanRight,
                    Keycode::Left => Action::PanLeft,
                    _ => Action::Continue,
                };
            }

            _ => {}
        }
    }

    return Action::Continue
}

