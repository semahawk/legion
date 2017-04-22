//
// position.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 09 Apr 2017 11:24:11 +0200 (CEST)
//

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use pathfinding;

use actor::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

  pub fn neighbours(&self) -> Vec<(Position, usize)> {
    let &Position{x: x, y: y} = self;

    vec![
      Position::new(x-1,y-1), Position::new(x+0,y-1), Position::new(x+1,y-1),
      Position::new(x-1,y+0),                         Position::new(x+1,y+0),
      Position::new(x-1,y+1), Position::new(x+0,y+1), Position::new(x+1,y+1)
    ].into_iter().map(|p| (p, 10)).collect()
  }

  pub fn cost_to(&self, other: &Position) -> usize {
    let delta_x = self.x - other.x;
    let delta_y = self.y - other.y;

    if delta_x == 0 || delta_y == 0 {
      10
    } else {
      14
    }
  }

  pub fn heuristic(&self, other: &Position) -> usize {
    ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
  }

  pub fn find_next_step_to(&self, destination: &Position) -> Option<Position> {
    let result = pathfinding::astar(self, |p| p.neighbours(), |p| p.cost_to(destination), |p| *p == *destination);

    match result {
      Some((steps, cost)) => {
        if steps[1] == *destination { None }
        else { Some(steps[1]) }
      },
      None => None,
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

