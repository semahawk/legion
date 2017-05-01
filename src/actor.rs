//
// actor.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 06 Apr 2017 20:07:14 +0200 (CEST)
//

use std::slice;

use draw::*;
use position::*;
use goap::*;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Action {
  Scout,
  Approach,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Condition {
  NearEnemy,
  TargetLocked,
}

#[derive(Debug)]
pub struct Actor {
  pub pos: Position,
  face: u8,
  pub action_planner: ActionPlanner<Action, Condition>,
  target: Option<Position>,
}

impl Actor {
  pub fn new(face: u8, x: i32, y: i32) -> Actor {
    Actor {
      pos: Position::new(x, y),
      face: face,
      action_planner: ActionPlanner::new(),
      target: None,
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

  pub fn plan_next_action(&mut self) -> Option<Action> {
    let plan = self.action_planner.plan();

    if plan.len() > 0 {
      Some(self.action_planner.plan()[0])
    } else {
      None
    }
  }

  pub fn update(&mut self, actors: &mut slice::IterMut<Actor>) {
    match self.plan_next_action() {
      a => {
        println!("planned: {:?}", a);
      },
      Some(Action::Scout) => {
        println!("scouting!");
        let target_actor = actors.find(|a| a.face != self.face);
        println!("found target: actor: {:?}", target_actor);
      },
      Some(Action::Approach) => {
        println!("approaching!");
        self.pos = self.pos.find_next_step_to(&Position::new(16, 16)).unwrap_or(self.pos);
      },
      None => (),
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

