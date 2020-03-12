extern crate sdl2;
extern crate sdl2_window;
extern crate window;

#[macro_use]
extern crate log;
use crate::window::AdvancedWindow;
use sdl2::event::Event;
use sdl2::event::WindowEvent;

use sdl2::video::Window;
use sdl2_window::Sdl2Window;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::{Duration, Instant};
use window::WindowSettings;

fn snap(b_window: &sdl2::video::Window, s_window: &mut sdl2::video::Window) {
    let b_pos = b_window.position();
    let s_pos = s_window.position();
    let b_size = b_window.size();
    let s_size = s_window.size();
    let mut snap_pos = s_pos;
    if s_pos.0 < b_pos.0 {
        snap_pos.0 = b_pos.0;
    }
    if s_pos.1 < b_pos.1 {
        snap_pos.1 = b_pos.1;
    }
    if b_pos.0 + (b_size.0 as i32) < s_pos.0 + (s_size.0 as i32) {
        snap_pos.0 = b_pos.0 + (b_size.0 as i32) - (s_size.0 as i32);
    }
    if b_pos.1 + (b_size.1 as i32) < s_pos.1 + (s_size.1 as i32) {
        snap_pos.1 = b_pos.1 + (b_size.1 as i32) - (s_size.1 as i32);
    }
    s_window.set_position(
        sdl2::video::WindowPos::Positioned(snap_pos.0),
        sdl2::video::WindowPos::Positioned(snap_pos.1),
    );
}

fn frame_rate_sleep(frame_start: Instant, refresh_rate: f32) {
    let elapsed = frame_start.elapsed();

    let frame_length = Duration::from_secs_f32(1.0 / refresh_rate);
    if elapsed < frame_length {
        sleep(frame_length - elapsed);
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let display_mode = video_subsystem.current_display_mode(0).unwrap();
    let (d_width, d_height) = (display_mode.w as u32, display_mode.h as u32);

    let neo_dimensions = ((d_width / 20) as u64, (d_height / 50) as u64);

    neovide::init_neovide(neo_dimensions);

    let mut parent = Sdl2Window::with_subsystem(
        video_subsystem,
        &WindowSettings::new("SDL Window", (d_width, d_height))
            .fullscreen(false)
            .vsync(true), // etc
    )
    .unwrap();

    parent.set_automatic_close(false);

    let mut event_pump = sdl.event_pump().expect("Could not create sdl event pump");
    let mut window = neovide::window::WindowWrapper::new(Some(sdl));

    info!("Starting window event loop");

    let refresh_rate = neovide::settings::SETTINGS
        .get::<neovide::window::WindowSettings>()
        .refresh_rate as f32;

    'running: loop {
        let frame_start = Instant::now();

        window.synchronize_settings();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Window break");

                    break 'running;
                }
                Event::Window {
                    window_id,
                    win_event,
                    ..
                } => {
                    match win_event {
                        WindowEvent::FocusGained => {
                            if window_id == parent.window.id() {
                                if let Some(mut neo_win) = window.window.take() {
                                    neo_win.raise();
                                    window.window = Some(neo_win);
                                }
                            }
                        }
                        WindowEvent::Moved { .. } => {
                            let mut neo_win = window.window.take().unwrap();
                            snap(&parent.window, &mut neo_win);
                            window.window = Some(neo_win);
                        }
                        WindowEvent::Close => {
                            window.window.take();
                            println!("Window Close");
                        }
                        _ => {}
                    }

                    neovide::redraw_scheduler::REDRAW_SCHEDULER.queue_next_frame()
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    keymod: modifiers,
                    ..
                } => window.handle_key_down(keycode, modifiers),
                Event::TextInput { text, .. } => window.handle_text_input(text),
                Event::MouseMotion { x, y, .. } => window.handle_pointer_motion(x, y),
                Event::MouseButtonDown { .. } => window.handle_pointer_down(),
                Event::MouseButtonUp { .. } => window.handle_pointer_up(),
                Event::MouseWheel { x, y, .. } => window.handle_mouse_wheel(x, y),

                _ => {}
            }
        }

        if window.window.is_some() {
            window.draw_frame();
        }
        frame_rate_sleep(frame_start, refresh_rate);
    }
    std::process::exit(0);
}
