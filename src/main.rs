extern crate sdl2;

use std::path::Path;

use sdl2::event::{Event, WindowEventId};
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};
use sdl2::keyboard::Keycode;

fn main() {
  let ctx = sdl2::init().unwrap();
  let video_ctx = ctx.video().unwrap();

  let window = match video_ctx.window("hello", 640, 480).position_centered().opengl().build() {
    Ok(window) => window,
    Err(err) => panic!("failed to create window: {}", err)
  };

  let mut renderer = match window.renderer().build() {
    Ok(renderer) => renderer,
    Err(err) => panic!("failed to create renderer: {}", err)
  };

  let surface = match Surface::load_bmp(&Path::new("font.bmp")) {
    Ok(surface) => surface,
    Err(err) => panic!("failed to load surface: {}", err)
  };

  let texture = match renderer.create_texture_from_surface(&surface) {
    Ok(texture) => texture,
    Err(err) => panic!("failed to convert surface: {:?}", err)
  };

  let _ = renderer.clear();

  // display the texture
  // omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer
  // try passing Some(surface.rect()) for src & dst instead of None to see how things change
  let source_rect = Rect::new(16, 0, 8, 8);
  let dest_rect = Rect::new(0, 0, 16, 16);
  let _ = renderer.copy(&texture, Some(source_rect), Some(dest_rect));
  let _ = renderer.present();

  let mut events = ctx.event_pump().unwrap();

  'event: loop {
    for event in events.wait_iter() {
      match event {
        Event::Quit{..} => break 'event,
        Event::Window{win_event_id, ..} => {
          match win_event_id {
            WindowEventId::Exposed => renderer.present(),
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

