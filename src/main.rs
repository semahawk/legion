extern crate sdl2;
extern crate pathfinding;
extern crate goap;

use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;
use std::time::{Instant, Duration};
use std::thread;

mod draw;
mod actor;
mod position;

use actor::*;
use draw::*;

const GAME_SPEED: u64    = 30;
const FPS_CAP: u64       = 60;
const MAX_FRAMESKIP: u64 = 10;

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

  let mut actors = vec!(Actor::new(1u8, 16, 16), Actor::new(2u8, 20, 32));

  actors[0].action_planner.add_action(
    Action::Scout,
    vec!((Condition::TargetLocked, false)),
    vec!((Condition::TargetLocked, true)),
    10
  );

  actors[0].action_planner.add_action(
    Action::Approach,
    vec!((Condition::TargetLocked, true), (Condition::NearEnemy, false)),
    vec!((Condition::NearEnemy, true)),
    0
  );

  actors[0].action_planner.set_state((Condition::NearEnemy, false));
  actors[0].action_planner.set_state((Condition::TargetLocked, true));
  actors[0].action_planner.set_goal((Condition::NearEnemy, true));

  actors[1].action_planner.add_action(
    Action::Scout,
    vec!((Condition::TargetLocked, false)),
    vec!((Condition::TargetLocked, true)),
    10
  );

  actors[1].action_planner.add_action(
    Action::Approach,
    vec!((Condition::TargetLocked, true), (Condition::NearEnemy, false)),
    vec!((Condition::NearEnemy, true)),
    0
  );

  actors[1].action_planner.set_state((Condition::NearEnemy, false));
  actors[1].action_planner.set_state((Condition::TargetLocked, true));
  actors[1].action_planner.set_goal((Condition::NearEnemy, true));

  let mut events = ctx.event_pump().unwrap();

  let frame_start_tick = Duration::new(0, 0);
  let mut next_game_tick = Instant::now();
  let mut loops = 0;

  'main: loop {
    loops = 0;

    if let Some(event) = events.poll_event() {
      match event {
        Event::Quit{..} => break 'main,
        Event::Window{win_event_id, ..} => {
          match win_event_id {
            WindowEventId::Exposed => draw.refresh(),
            _ => (),
          }
        },
        Event::KeyDown{keycode: Some(keycode), ..} => {
          if keycode == Keycode::Escape {
            break 'main
          } else if keycode == Keycode::H {
            actors[0].move_left();
          } else if keycode == Keycode::J {
            actors[0].move_down();
          } else if keycode == Keycode::K {
            actors[0].move_up();
          } else if keycode == Keycode::L {
            actors[0].move_right();
          }
        }
        _ => (),
      }
    }

    while Instant::now() > next_game_tick && loops < MAX_FRAMESKIP {
      let mut actors_iter = actors.iter_mut();

      while let Some(actor) = actors_iter.next() {
        actor.update(&mut actors_iter);
      }

      next_game_tick += Duration::from_millis((1000f64 / GAME_SPEED as f64) as u64);
      loops += 1;
    }

    draw.refresh();

    for actor in actors.iter() {
      actor.draw(&mut draw);
    }

    {
      // Cap the frame rate
      let remaining = Instant::now().elapsed() - frame_start_tick;
      let delay_time = Duration::from_millis((1000f64 / FPS_CAP as f64) as u64) - remaining;

      if delay_time > Duration::from_secs(0) {
        thread::sleep(delay_time);
      }
    }
  }
}

/*
 * vi: ts=2:sw=2 expandtab
 */

