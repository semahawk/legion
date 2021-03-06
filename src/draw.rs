//
// draw.rs
// Copyright (C) 2016 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use std::path::Path;

use sdl2::render::{Renderer, Texture};
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};
use sdl2::pixels::{Color};

use position::*;

pub struct Drawer<'a> {
  font_texture: Texture,
  renderer: Renderer<'a>,
  curr_x: i32,
  curr_y: i32,
}

impl<'a> Drawer<'a> {
  pub fn new(renderer: Renderer<'a>) -> Drawer<'a> {
    let mut font_surface = match Surface::load_bmp(&Path::new("font.bmp")) {
      Ok(surface) => surface,
      Err(err) => panic!("failed to load surface: {}", err)
    };

    font_surface.set_color_key(true, Color::RGB(0xff, 0x00, 0xff)).expect("Couldn't set color key");

    let font_texture = match renderer.create_texture_from_surface(&font_surface) {
      Ok(texture) => texture,
      Err(err) => panic!("failed to convert surface: {:?}", err)
    };

    Drawer {
      font_texture: font_texture,
      renderer: renderer,
      curr_x: 0, curr_y: 0,
    }
  }

  pub fn put_at(&mut self, ch: u8, pos: &Position) {
    let source_rect = Rect::new((ch as i32) % 16 * 8, (ch as i32) / 16 * 8, 8, 8);
    let dest_rect = Rect::new(pos.x() * 8, pos.y() * 8, 8, 8);

    let _ = self.renderer.copy(&self.font_texture, Some(source_rect), Some(dest_rect));
    self.renderer.present();
  }

  pub fn put(&mut self, ch: u8) {
    let x = self.curr_x;
    let y = self.curr_y;

    self.put_at(ch, &Position::new(x, y));

    self.curr_x = self.curr_x + 1;
  }

  pub fn refresh(&mut self) {
    self.renderer.clear();
    self.renderer.present();
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

