use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Mod;

use sdl2::video::Window;

use neovide::window::WindowWrapper;

use std::thread::sleep;

use std::time::{Duration, Instant};

pub fn snap(b_window: &Window, s_window: &mut Window) {
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

pub fn frame_rate_sleep(frame_start: Instant, refresh_rate: f32) {
    let elapsed = frame_start.elapsed();

    let frame_length = Duration::from_secs_f32(1.0 / refresh_rate);
    if elapsed < frame_length {
        sleep(frame_length - elapsed);
    }
}

pub enum EventResult {
    Close(u32),
    TextInput(String),
    KeyDown { keycode: Keycode, keymod: Mod },
    Quit,
    Running,
}

pub fn process_neovide_events(
    event: Event,
    neo_outer: &mut WindowWrapper,
    mut neo_inner: &mut Window,
    parent: &Window,
) -> EventResult {
    match event {
        Event::Quit { .. } => {
            return EventResult::Quit;
        }
        Event::Window {
            window_id,
            win_event,
            ..
        } => {
            match win_event {
                WindowEvent::FocusGained => {
                    if window_id == parent.id() {
                        neo_inner.raise();
                    }
                }
                WindowEvent::Moved { .. } => {
                    snap(&parent, &mut neo_inner);
                }
                WindowEvent::Close => {
                    return EventResult::Close(window_id);
                }
                _ => {}
            }

            neovide::redraw_scheduler::REDRAW_SCHEDULER.queue_next_frame()
        }
        Event::KeyDown {
            keycode: Some(keycode),
            keymod: modifiers,
            ..
        } => {
            return EventResult::KeyDown {
                keycode,
                keymod: modifiers,
            };
        }
        Event::TextInput { text, .. } => return EventResult::TextInput(text),
        Event::MouseMotion { x, y, .. } => neo_outer.handle_pointer_motion(x, y),
        Event::MouseButtonDown { .. } => neo_outer.handle_pointer_down(),
        Event::MouseButtonUp { .. } => neo_outer.handle_pointer_up(),
        Event::MouseWheel { x, y, .. } => neo_outer.handle_mouse_wheel(x, y),

        _ => {}
    }

    return EventResult::Running;
}

pub fn process_main_events(event: Event, mut main_window: &mut Window) -> EventResult {
    match event {
        Event::Quit { .. } => {
            return EventResult::Quit;
        }
        Event::Window {
            window_id,
            win_event,
            ..
        } => match win_event {
            WindowEvent::Close => {
                return EventResult::Close(window_id);
            }
            _ => {}
        },
        Event::KeyDown {
            keycode: Some(keycode),
            keymod: modifiers,
            ..
        } => {}
        Event::TextInput { text, .. } => {}
        Event::MouseMotion { x, y, .. } => {}
        Event::MouseButtonDown { .. } => {}
        Event::MouseButtonUp { .. } => {}
        Event::MouseWheel { x, y, .. } => {}

        _ => {}
    }
    return EventResult::Running;
}
