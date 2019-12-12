use crate::intcode::*;
use image::ImageBuffer;
use itertools::Itertools;
use std::collections::HashMap;

type Coordinate = (isize, isize);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
  Black,
  White,
}

impl Into<isize> for Color {
  fn into(self) -> isize {
    match self {
      Color::Black => 0,
      Color::White => 1,
    }
  }
}

impl From<isize> for Color {
  fn from(n: isize) -> Color {
    match n {
      0 => Color::Black,
      1 => Color::White,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Turn {
  Left,
  Right,
}

impl From<isize> for Turn {
  fn from(n: isize) -> Turn {
    match n {
      0 => Turn::Left,
      1 => Turn::Right,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
  Up,
  Left,
  Down,
  Right,
}

impl Direction {
  fn turn(self, turn: Turn) -> Direction {
    match self {
      Self::Up => match turn {
        Turn::Left => Self::Left,
        Turn::Right => Self::Right,
      },
      Self::Down => match turn {
        Turn::Left => Self::Right,
        Turn::Right => Self::Left,
      },
      Self::Left => match turn {
        Turn::Left => Self::Down,
        Turn::Right => Self::Up,
      },
      Self::Right => match turn {
        Turn::Left => Self::Up,
        Turn::Right => Self::Down,
      },
    }
  }
}

trait Motion {
  fn go(self, dir: Direction) -> Self;
}

impl Motion for Coordinate {
  fn go(self, dir: Direction) -> Self {
    let (x, y) = self;
    match dir {
      Direction::Up => (x, y + 1),
      Direction::Down => (x, y - 1),
      Direction::Left => (x - 1, y),
      Direction::Right => (x + 1, y),
    }
  }
}

fn solve_01(input: &str) -> usize {
  use std::thread;

  let mut vm = VM::from(input);

  let output = vm.output.clone();
  let input = vm.input.clone();

  let mut map = HashMap::new();
  let mut loc = (0, 0);
  let mut facing = Direction::Up;
  thread::spawn(move || vm.run());

  input
    .send(map.get(&loc).copied().map(Color::into).unwrap_or(0))
    .expect("initial send successful");

  output.iter().tuples().for_each(|(color, turn)| {
    let color = Color::from(color);
    let turn = Turn::from(turn);

    map.insert(loc, color);
    facing = facing.turn(turn);
    loc = loc.go(facing);
    input.send(map.get(&loc).copied().map(Color::into).unwrap_or(0));
  });

  map.len()
}

fn solve_02(input: &str) {
  use std::thread;

  let mut vm = VM::from(input);

  let output = vm.output.clone();
  let input = vm.input.clone();

  let mut map = HashMap::new();
  let mut loc = (0, 0);
  map.insert(loc, Color::White);
  let mut facing = Direction::Up;
  thread::spawn(move || vm.run());

  input
    .send(map.get(&loc).copied().map(Color::into).unwrap_or(0))
    .expect("initial send successful");

  output.iter().tuples().for_each(|(color, turn)| {
    let color = Color::from(color);
    let turn = Turn::from(turn);

    map.insert(loc, color);
    facing = facing.turn(turn);
    loc = loc.go(facing);
    input.send(map.get(&loc).copied().map(Color::into).unwrap_or(0));
  });

  let white_pixels = map
    .iter()
    .filter(|(_, &v)| v == Color::White)
    .map(|(k, _)| k)
    .collect_vec();

  let (min_x, _) = white_pixels.iter().min_by_key(|(x, _)| x).unwrap();
  let (_, min_y) = white_pixels.iter().min_by_key(|(_, y)| y).unwrap();

  let white_pixels = white_pixels
    .iter()
    .map(|(x, y)| (x - min_x, y - min_y))
    .collect_vec();

  let (max_x, _) = *white_pixels.iter().max_by_key(|(x, _)| x).unwrap();
  let (_, max_y) = *white_pixels.iter().max_by_key(|(_, y)| y).unwrap();

  let img = ImageBuffer::from_fn((max_x + 1) as u32, (max_y + 1) as u32, |x, y| {
    let x = x as isize;
    let y = y as isize;

    if white_pixels.contains(&(x, y)) {
      image::Luma([255u8])
    } else {
      image::Luma([0u8])
    }
  });

  image::imageops::flip_vertical(&img)
    .save("./out.png")
    .expect("able to write answer");
}

pub fn solve(input: &str) {
  dbg!(solve_01(input));
  dbg!(solve_02(input));
}
