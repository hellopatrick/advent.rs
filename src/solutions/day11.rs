use crate::intcode::*;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
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

fn sendable(c: Option<Color>) -> isize {
  match c {
    None => 0,
    Some(c) => c.into(),
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
  fn turn(self, dir: isize) -> Direction {
    match self {
      Self::Up => {
        if dir == 0 {
          Self::Left
        } else {
          Self::Right
        }
      }
      Self::Down => {
        if dir == 0 {
          Self::Right
        } else {
          Self::Left
        }
      }
      Self::Left => {
        if dir == 0 {
          Self::Down
        } else {
          Self::Up
        }
      }
      Self::Right => {
        if dir == 0 {
          Self::Up
        } else {
          Self::Down
        }
      }
    }
  }

  fn go(self, (x, y): Coordinate) -> Coordinate {
    match self {
      Self::Up => (x, y + 1),
      Self::Down => (x, y - 1),
      Self::Left => (x - 1, y),
      Self::Right => (x + 1, y),
    }
  }
}

fn solve_01(input: &str) -> usize {
  use std::thread;

  let mut vm = VM::from(input);

  let output = vm.output.clone();
  let input = vm.input.clone();

  let mut map: HashMap<Coordinate, Color> = HashMap::new();
  let mut loc = (0, 0);
  let mut facing = Direction::Up;
  thread::spawn(move || vm.run());

  input.send(sendable(map.get(&loc).copied()));

  output.iter().tuples().for_each(|(color, direction)| {
    let color = Color::from(color);
    map.insert(loc, color);
    facing = facing.turn(direction);
    loc = facing.go(loc);
    input.send(sendable(map.get(&loc).copied()));
  });

  map.len()
}

fn solve_02(input: &str) {
  use std::thread;

  let mut vm = VM::from(input);

  let output = vm.output.clone();
  let input = vm.input.clone();

  let mut map: HashMap<Coordinate, Color> = HashMap::new();
  let mut loc = (0, 0);
  map.insert(loc, Color::White);
  let mut facing = Direction::Up;
  thread::spawn(move || vm.run());

  input.send(sendable(map.get(&loc).copied()));

  output.iter().tuples().for_each(|(color, direction)| {
    let color = Color::from(color);

    map.insert(loc, color);

    facing = facing.turn(direction);
    loc = facing.go(loc);
    input.send(sendable(map.get(&loc).copied()));
  });

  let white_pixels = map
    .iter()
    .filter(|(_, &v)| v == Color::White)
    .map(|(k, _)| k)
    .collect_vec();

  let (min_x, _) = white_pixels.iter().min_by_key(|(x, _)| x).unwrap();
  let (_, min_y) = white_pixels.iter().min_by_key(|(_, y)| y).unwrap();

  let adjusted = white_pixels
    .iter()
    .map(|(x, y)| (x - min_x, y - min_y))
    .collect_vec();

  let (max_x, _) = *adjusted.iter().max_by_key(|(x, _)| x).unwrap();
  let (_, max_y) = *adjusted.iter().max_by_key(|(_, y)| y).unwrap();

  let img = ImageBuffer::from_fn((max_x + 1) as u32, (max_y + 1) as u32, |x, y| {
    let x = x as isize;
    let y = y as isize;

    if adjusted.contains(&(x, y)) {
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
