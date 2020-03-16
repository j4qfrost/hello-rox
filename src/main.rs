extern crate neovide;
extern crate sdl2;

#[macro_use]
extern crate log;

use std::env;
use std::path::Path;
use std::time::Instant;
use sdl2::image::{InitFlag, LoadTexture};

mod util;
use crate::util::frame_rate_sleep;
use crate::util::process_main_events;
use crate::util::process_neovide_events;
use crate::util::EventResult;

fn main() {
    let sdl = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let video_subsystem = sdl.video().unwrap();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let (d_width, d_height) = (display_mode.w as u32, display_mode.h as u32);

    let neo_dimensions = ((d_width / 20) as u64, (d_height / 50) as u64);

    neovide::init_neovide(neo_dimensions);

    let mut main_window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut event_pump = sdl.event_pump().expect("Could not create sdl event pump");
    let mut neovide_window = neovide::window::WindowWrapper::new(Some(sdl));

    // info!("Starting window event loop");

    let refresh_rate = neovide::settings::SETTINGS
        .get::<neovide::window::WindowSettings>()
        .refresh_rate as f32;
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    let mut canvas = main_window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let png = Path::join(&path, "assets/rust.png");
    println!("{:?}", png);

    let texture_creator = canvas.texture_creator();
    let texture = match texture_creator.load_texture("assets/rust.png") {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    canvas.copy(&texture, None, None);
    canvas.present();

    main_window = canvas.into_window();

    'running: loop {
        let frame_start = Instant::now();
        let mut event_result: EventResult;
        neovide_window.synchronize_settings();

        let mut keycode = None;
        // let mut keymod = None;
        let mut keytext = None;

        for event in event_pump.poll_iter() {
            if let Some(mut neo_inner) = neovide_window.window.take() {
                event_result = process_neovide_events(
                    event,
                    &mut neovide_window,
                    &mut neo_inner,
                    &main_window,
                );
                match event_result {
                    EventResult::Quit => break 'running,
                    EventResult::Close(win_id) => {
                        if win_id == main_window.id() {
                            break 'running;
                        }
                        continue 'running;
                    }
                    EventResult::KeyDown {
                        keycode: code,
                        keymod: _,
                    } => {
                        keycode = Some(code);
                        // keymod = Some(modifiers);
                    }
                    EventResult::TextInput(text) => keytext = Some(text),
                    _ => {
                        neovide_window.window = Some(neo_inner);
                    }
                }
            } else {
                event_result = process_main_events(event, &mut main_window);
                match event_result {
                    EventResult::Quit | EventResult::Close(..) => break 'running,
                    _ => {}
                }
            }
        }

        neovide_window.handle_keyboard_input(keycode, keytext);
        neovide_window.draw_frame();

        frame_rate_sleep(frame_start, refresh_rate);
    }
    std::process::exit(0);
}
