use crate::intcode::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tile {
  Empty,
  Wall,
  Block,
  Paddle,
  Ball,
}

impl From<isize> for Tile {
  fn from(i: isize) -> Tile {
    match i {
      0 => Tile::Empty,
      1 => Tile::Wall,
      2 => Tile::Block,
      3 => Tile::Paddle,
      4 => Tile::Ball,
      _ => unreachable!(),
    }
  }
}

fn solve_01(input: &str) -> usize {
  let mut vm = VM::from(input);

  let output = vm.output.clone();

  thread::spawn(move || vm.run());

  let mut m = HashMap::new();

  output.iter().tuples().for_each(|(a, b, c)| {
    m.insert((a, b), Tile::from(c));
  });

  m.values().filter(|&c| *c == Tile::Block).count()
}

fn solve_02(input: &str) -> isize {
  let mut vm = VM::from(input);
  vm.memory[0] = 2;

  let output = vm.output.clone();
  let input = vm.input.clone();
  input.send(0);
  thread::spawn(move || vm.run());

  let mut score = 0;
  let mut ball = 0;
  let mut paddle = -1;

  output.iter().tuples().for_each(|(x, _, t)| {
    if x == -1 {
      score = t;
      dbg!(score);
    } else {
      match Tile::from(t) {
        Tile::Ball => {
          ball = x;
          if paddle > 0 {
            let joystick = (ball - paddle).signum();
            input.send(joystick);
          }
        }
        Tile::Paddle => paddle = x,
        _ => (),
      };
    }
  });

  score
}

pub fn solve(input: &str) {
  dbg!(solve_01(input));
  dbg!(solve_02(input));
}
