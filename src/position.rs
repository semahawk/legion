//
// position.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 09 Apr 2017 11:24:11 +0200 (CEST)
//

pub struct Position {
  x: i32,
  y: i32,
}

impl Position {
  pub fn new(x: i32, y: i32) -> Position {
    Position {
      x: x,
      y: y,
    }
  }

  pub fn x(&self) -> i32 {
    self.x
  }

  pub fn y(&self) -> i32 {
    self.y
  }

  pub fn move_up(&mut self) {
    self.y -= 1;
  }

  pub fn move_down(&mut self) {
    self.y += 1;
  }

  pub fn move_left(&mut self) {
    self.x -= 1;
  }

  pub fn move_right(&mut self) {
    self.x += 1;
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

