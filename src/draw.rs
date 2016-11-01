//
// draw.rs
// Copyright (C) 2016 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//

use std::path::Path;

use sdl2::render::{Renderer, Texture};
use sdl2::surface::{Surface};
use sdl2::rect::{Rect};

pub struct Drawer<'a> {
  font_texture: Texture,
  renderer: Renderer<'a>,
}

impl<'a> Drawer<'a> {
  pub fn new(renderer: Renderer<'a>) -> Drawer<'a> {
    let font_surface = match Surface::load_bmp(&Path::new("font.bmp")) {
      Ok(surface) => surface,
      Err(err) => panic!("failed to load surface: {}", err)
    };

    let font_texture = match renderer.create_texture_from_surface(&font_surface) {
      Ok(texture) => texture,
      Err(err) => panic!("failed to convert surface: {:?}", err)
    };

    Drawer {
      font_texture: font_texture,
      renderer: renderer,
    }
  }

  pub fn put(&mut self, ch: u8) {
    let source_rect = Rect::new((ch as i32) % 16 * 8, (ch as i32) / 16 * 8, 8, 8);
    let dest_rect = Rect::new(0, 0, 16, 16);

    self.renderer.copy(&self.font_texture, Some(source_rect), Some(dest_rect));
    self.renderer.present();
  }

  pub fn refresh(&self) {
    println!("requesting refresh!")
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

