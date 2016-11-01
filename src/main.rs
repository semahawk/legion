extern crate sdl2;

use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;

mod draw;
use draw::*;

fn main() {
  let ctx = sdl2::init().unwrap();
  let video_ctx = ctx.video().unwrap();

  let window = match video_ctx.window("hello", 640, 480).position_centered().opengl().build() {
    Ok(window) => window,
    Err(err) => panic!("failed to create window: {}", err)
  };

  let renderer = match window.renderer().build() {
    Ok(renderer) => renderer,
    Err(err) => panic!("failed to create renderer: {}", err)
  };

  let mut draw = Drawer::new(renderer);
  draw.put(1);
  draw.put(2);
  draw.put(3);

  let mut events = ctx.event_pump().unwrap();

  'event: loop {
    for event in events.wait_iter() {
      match event {
        Event::Quit{..} => break 'event,
        Event::Window{win_event_id, ..} => {
          match win_event_id {
            WindowEventId::Exposed => draw.refresh(),
            _ => (),
          }
        },
        Event::KeyDown{keycode: Some(keycode), ..} => {
          println!("registered a keydown event: {:?}", keycode);
          if keycode == Keycode::Escape {
            break 'event
          }
        }
        _ => continue
      }
    }
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

