//use gfx::Device;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
//use gfx::format::{Rgba8, DepthStencil};
//
//const CLEAR_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
//
//pub fn main() {
//    let sdl_context = sdl2::init().unwrap();
//    let video = sdl_context.video().unwrap();
//    // Request opengl core 3.2 for example:
//    video.gl_attr().set_context_profile(sdl2::video::GLProfile::Core);
//    video.gl_attr().set_context_version(3, 2);
//    let builder = video.window("SDL Window", 1024, 768);
//    let (window, _gl_context, mut device, mut factory, main_color, _main_depth) =
//        gfx_window_sdl::init::<Rgba8, DepthStencil>(&video, builder).unwrap();
//
//    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
//
//    let mut events = sdl_context.event_pump().unwrap();
//
//    let mut running = true;
//    while running {
//        // handle events
//        for event in events.poll_iter() {
//            match event {
//                Event::Quit { .. } |
//                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => {
//                    running = false;
//                }
//                _ => {}
//            }
//        }
//
//        // draw a frame
//        encoder.clear(&main_color, CLEAR_COLOR);
//        // <- draw actual stuff here
//        encoder.flush(&mut device);
//        window.gl_swap_window();
//        device.cleanup();
//    }
//}

pub fn main() {}