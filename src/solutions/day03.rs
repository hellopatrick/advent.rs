use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Dir {
  Up,
  Down,
  Left,
  Right,
}

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

type Coordinate = (isize, isize);

impl Step {
  pub fn dp(&self) -> Coordinate {
    match self.dir {
      Dir::Up => (0, 1),
      Dir::Down => (0, -1),
      Dir::Left => (-1, 0),
      Dir::Right => (1, 0),
    }
  }
}

impl From<&str> for Step {
  fn from(s: &str) -> Self {
    let dir: Dir = s.chars().take(1).next().expect("must exist").into();
    let len = s
      .get(1..)
      .expect("must exist")
      .parse()
      .expect("must be number");

    Self { dir, len }
  }
}

fn path(route: &[Step]) -> HashMap<(isize, isize), isize> {
  let mut map = HashMap::new();
  let mut len = 1;

  let (mut x, mut y) = (0, 0);

  for dir in route {
    let (dx, dy) = dir.dp();

    for _ in 0..dir.len {
      x += dx;
      y += dy;

      *map.entry((x, y)).or_insert(0) = len;
      len += 1;
    }
  }

  map
}

fn manhattan_distance(coord: (isize, isize)) -> isize {
  coord.0.abs() + coord.1.abs()
}

fn solve_01(routes: Vec<Vec<Step>>) -> isize {
  let path_one = path(&routes[0]);
  let path_two = path(&routes[1]);

  let set_one: HashSet<_> = path_one.keys().collect();
  let set_two: HashSet<_> = path_two.keys().collect();
  let intersections = set_one.intersection(&set_two);

  intersections
    .map(|x| manhattan_distance(**x))
    .min()
    .expect("intersection")
}

fn solve_02(routes: Vec<Vec<Step>>) -> isize {
  let path_one = path(&routes[0]);
  let path_two = path(&routes[1]);

  let set_one: HashSet<_> = path_one.keys().collect();
  let set_two: HashSet<_> = path_two.keys().collect();
  let intersections = set_one.intersection(&set_two);

  let mut min = std::isize::MAX;

  for intersection in intersections {
    let len_one = path_one.get(*intersection).expect("must be here");
    let len_two = path_two.get(*intersection).expect("must be here");

    let total = len_one + len_two;

    if total < min {
      min = total;
    }
  }

  min
}

fn parse(input: &str) -> Vec<Vec<Step>> {
  input
    .lines()
    .map(|line| line.trim().split(',').map(Step::from).collect())
    .collect()
}

pub fn solve(input: &str) {
  let instructions = parse(input);

  let res = solve_01(instructions);

  println!("{}", res);

  let instructions = parse(input);

  let res = solve_02(instructions);

  println!("{}", res);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_part_one() {
    let input = parse(
      "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
      U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    );

    let res = solve_01(input);

    assert_eq!(res, 135);
  }
}
