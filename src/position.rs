//
// position.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 09 Apr 2017 11:24:11 +0200 (CEST)
//

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
  x: i32,
  y: i32,
}

pub struct NeighbourIter {
  start_pos: Position,
  current: usize,
}

impl Iterator for NeighbourIter {
  type Item = Position;

  fn next(&mut self) -> Option<Position> {
    self.current += 1;

    match self.current {
      1 => Some(Position { x: self.start_pos.x - 1, y: self.start_pos.y - 1 }),
      2 => Some(Position { x: self.start_pos.x - 0, y: self.start_pos.y - 1 }),
      3 => Some(Position { x: self.start_pos.x + 1, y: self.start_pos.y - 1 }),
      4 => Some(Position { x: self.start_pos.x - 1, y: self.start_pos.y - 0 }),
      5 => Some(Position { x: self.start_pos.x + 1, y: self.start_pos.y - 0 }),
      6 => Some(Position { x: self.start_pos.x - 1, y: self.start_pos.y + 1 }),
      7 => Some(Position { x: self.start_pos.x - 0, y: self.start_pos.y + 1 }),
      8 => Some(Position { x: self.start_pos.x + 1, y: self.start_pos.y + 1 }),
      _ => None,
    }
  }
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

  pub fn neighbours(&self) -> NeighbourIter {
    NeighbourIter {
      start_pos: *self,
      current: 0,
    }
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

  pub fn find_path_to(&self, destination: &Position) -> Option<Position> {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    struct Tile {
      // identification
      pos: Position,
      // the cost
      distance_to_destination: usize,
    }

    impl Ord for Tile {
      fn cmp(&self, other: &Tile) -> Ordering {
        other.distance_to_destination.cmp(&self.distance_to_destination)
      }
    }

    impl PartialOrd for Tile {
      fn partial_cmp(&self, other: &Tile) -> Option<Ordering> {
        Some(self.cmp(other))
      }
    }

    let mut next_step: Option<Tile> = None;
    let mut came_from: HashMap<Position, Option<Tile>> = HashMap::new();
    let mut cost_so_far: HashMap<Position, usize> = HashMap::new();
    let mut frontier = BinaryHeap::new();

    came_from.insert(*self, None);
    cost_so_far.insert(*self, 0);
    frontier.push(Tile { pos: *self, distance_to_destination: 0 });

    'find_path: while let Some(current) = frontier.pop() {
      for next in current.pos.neighbours() {
        if next == *destination {
          next_step = Some(current);
          break 'find_path;
        }

        // TODO: check for 'passability' of the tiles (ie. characters shouldn't be able to exist in
        // the same physical space at the same time (ie. on the same tile))

        let new_cost = cost_so_far.get(&current.pos).unwrap() + current.pos.cost_to(&next);
        if !cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
          cost_so_far.insert(next, new_cost);
          frontier.push(Tile { pos: next, distance_to_destination: new_cost + destination.heuristic(&next) });
          came_from.insert(next, Some(current));
        }
      }
    }

    while let Some(parent) = came_from.get(&next_step.unwrap().pos) {
      if parent.is_none() { break }
      if parent.unwrap().pos == *self { break }
      next_step = *parent;
    }

    Some(next_step.unwrap().pos)
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

