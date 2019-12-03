use std::collections::{HashMap, HashSet};

type Coordinate = (isize, isize);

#[derive(Debug)]
enum Dir {
  Up,
  Down,
  Left,
  Right,
}

impl Dir {
  pub fn delta(&self) -> Coordinate {
    match self {
      Dir::Up => (0, 1),
      Dir::Down => (0, -1),
      Dir::Left => (-1, 0),
      Dir::Right => (1, 0),
    }
  }
}

// in real world, we'd use TryFrom, but no malformed inputs here.
impl From<char> for Dir {
  fn from(c: char) -> Self {
    match c {
      'U' => Self::Up,
      'D' => Self::Down,
      'L' => Self::Left,
      'R' => Self::Right,
      _ => panic!("non-supported direction"),
    }
  }
}

#[derive(Debug)]
struct Step {
  dir: Dir,
  len: isize,
}

impl From<&str> for Step {
  fn from(s: &str) -> Self {
    let dir: Dir = s.chars().take(1).next().unwrap().into();
    let len = s.get(1..).unwrap().parse().unwrap();

    Self { dir, len }
  }
}

fn path(route: &[Step]) -> HashMap<Coordinate, isize> {
  let mut map = HashMap::new();
  let mut len = 1;

  let (mut x, mut y) = (0, 0);

  for step in route {
    let (dx, dy) = step.dir.delta();

    for _ in 0..step.len {
      x += dx;
      y += dy;

      *map.entry((x, y)).or_insert(0) = len;
      len += 1;
    }
  }

  map
}

fn manhattan_distance(coord: Coordinate) -> isize {
  coord.0.abs() + coord.1.abs()
}

fn solve_01(routes: &[Vec<Step>]) -> isize {
  let path_one = path(&routes[0]);
  let path_two = path(&routes[1]);

  let set_one: HashSet<_> = path_one.keys().collect();
  let set_two: HashSet<_> = path_two.keys().collect();
  let intersections = set_one.intersection(&set_two);

  intersections
    .map(|x| manhattan_distance(**x))
    .min()
    .unwrap()
}

fn solve_02(routes: &[Vec<Step>]) -> isize {
  let path_one = path(&routes[0]);
  let path_two = path(&routes[1]);

  let set_one: HashSet<_> = path_one.keys().collect();
  let set_two: HashSet<_> = path_two.keys().collect();
  let intersections = set_one.intersection(&set_two);

  intersections
    .map(|x| path_one.get(*x).unwrap() + path_two.get(*x).unwrap())
    .min()
    .unwrap()
}

fn parse(input: &str) -> Vec<Vec<Step>> {
  input
    .lines()
    .map(|line| line.trim().split(',').map(Step::from).collect())
    .collect()
}

pub fn solve(input: &str) {
  let instructions = parse(input);

  println!("part one: {}", solve_01(&instructions));
  println!("part two: {}", solve_02(&instructions));
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn part_one() {
    let input = parse(
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
      U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );

    let res = solve_01(&input);

    assert_eq!(res, 135);
  }

  #[test]
  fn part_two() {
    let input = parse(
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
      U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );

    let res = solve_02(&input);

    assert_eq!(res, 410);
  }
}
