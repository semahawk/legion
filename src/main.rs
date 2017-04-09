extern crate sdl2;

use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;

mod draw;
mod actor;

use actor::*;
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

  let mut actor_a = Actor::new(1u8);

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
          } else if keycode == Keycode::H {
            actor_a.move_left();
          } else if keycode == Keycode::J {
            actor_a.move_down();
          } else if keycode == Keycode::K {
            actor_a.move_up();
          } else if keycode == Keycode::L {
            actor_a.move_right();
          }
        }
        _ => (),
      }

      draw.refresh();
      actor_a.draw(&mut draw);
    }
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

