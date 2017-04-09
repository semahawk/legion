//
// actor.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 06 Apr 2017 20:07:14 +0200 (CEST)
//

use draw::*;
use position::*;

pub struct Actor {
  pub pos: Position,
  face: u8,
}

impl Actor {
  pub fn new(face: u8, x: i32, y: i32) -> Actor {
    Actor {
      pos: Position::new(x, y),
      face: face,
    }
  }

  pub fn move_up(&mut self) {
    self.pos.move_up();
  }

  pub fn move_down(&mut self) {
    self.pos.move_down();
  }

  pub fn move_left(&mut self) {
    self.pos.move_left();
  }

  pub fn move_right(&mut self) {
    self.pos.move_right();
  }

  pub fn draw(&self, draw: &mut Drawer) {
    draw.put_at(self.face, &self.pos);
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

