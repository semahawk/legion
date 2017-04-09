//
// actor.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 06 Apr 2017 20:07:14 +0200 (CEST)
//

use draw::*;

pub struct Actor {
  x: i32,
  y: i32,
  face: u8,
}

impl Actor {
  pub fn new(face: u8) -> Actor {
    Actor {
      x: 0,
      y: 0,
      face: face,
    }
  }

  pub fn move_up(&mut self) {
    self.y -= 1
  }

  pub fn move_down(&mut self) {
    self.y += 1
  }

  pub fn move_left(&mut self) {
    self.x -= 1
  }

  pub fn move_right(&mut self) {
    self.x += 1
  }

  pub fn draw(&self, draw: &mut Drawer) {
    draw.put_at(self.face, self.x, self.y);
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

