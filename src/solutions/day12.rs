use itertools::Itertools;
use regex::*;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector {
  x: i32,
  y: i32,
  z: i32,
}

impl Vector {
  fn energy(self) -> i32 {
    self.x.abs() + self.y.abs() + self.z.abs()
  }

  fn diff(self, other: Vector) -> (i32, i32, i32) {
    use std::cmp::Ordering;

    let x = match self.x.cmp(&other.x) {
      Ordering::Equal => 0,
      Ordering::Less => 1,
      Ordering::Greater => -1,
    };

    let y = match self.y.cmp(&other.y) {
      Ordering::Equal => 0,
      Ordering::Less => 1,
      Ordering::Greater => -1,
    };

    let z = match self.z.cmp(&other.z) {
      Ordering::Equal => 0,
      Ordering::Less => 1,
      Ordering::Greater => -1,
    };

    (x, y, z)
  }
}

impl std::ops::Add<Vector> for Vector {
  type Output = Vector;
  fn add(self, other: Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl From<&str> for Vector {
  fn from(input: &str) -> Vector {
    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();

    let caps = re.captures(input).unwrap();

    let x = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
    let y = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
    let z = caps.get(3).map_or("", |m| m.as_str()).parse().unwrap();

    Vector { x, y, z }
  }
}

#[derive(Debug, Copy, Clone)]
struct Moon {
  p: Vector,
  v: Vector,
}

impl Moon {
  fn energy(&self) -> i32 {
    self.p.energy() * self.v.energy()
  }
}

#[derive(Clone, Debug)]
struct Simulation {
  moons: Vec<Moon>,
}

impl Simulation {
  fn new(moons: &[Moon]) -> Self {
    Self {
      moons: moons.to_vec(),
    }
  }
}

impl<'a> Iterator for Simulation {
  type Item = Vec<Moon>;

  fn next(&mut self) -> Option<Vec<Moon>> {
    use std::cmp::Ordering;
    let new_moons = self
      .moons
      .iter()
      .map(|moon| {
        let v = self.moons.iter().fold(moon.v, |acc, next| {
          let (dx, dy, dz) = moon.p.diff(next.p);

          Vector {
            x: acc.x + dx,
            y: acc.y + dy,
            z: acc.z + dz,
          }
        });
        Moon { p: moon.p, v }
      })
      .map(|moon| Moon {
        p: moon.p + moon.v,
        v: moon.v,
      })
      .collect();

    self.moons = new_moons;
    Some(self.moons.clone())
  }
}

fn solve_01(moons: &[Moon], steps: usize) -> i32 {
  let mut sim = Simulation::new(moons);

  let state = sim.nth(steps).unwrap();

  state.iter().map(|m| m.energy()).sum()
}

fn gcd(mut m: usize, mut n: usize) -> usize {
  while m != 0 {
    let old_m = m;
    m = n % m;
    n = old_m;
  }

  n
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

fn solve_02(moons: &[Moon]) -> usize {
  let sim = Simulation::new(moons);

  let mut set = HashSet::new();

  for state in sim {
    let i = state.iter().map(|m| (m.p.x, m.v.x)).collect_vec();

    if set.contains(&i) {
      break;
    }

    set.insert(i);
  }

  let x = set.len();

  let sim = Simulation::new(moons);

  set.clear();

  for state in sim {
    let i = state.iter().map(|m| (m.p.y, m.v.y)).collect_vec();

    if set.contains(&i) {
      break;
    }
    set.insert(i);
  }

  let y = set.len();

  let sim = Simulation::new(moons);

  set.clear();

  for state in sim {
    let i = state.iter().map(|m| (m.p.z, m.v.z)).collect_vec();

    if set.contains(&i) {
      break;
    }
    set.insert(i);
  }

  let z = set.len();
  dbg!(x, y, z);
  lcm(x, lcm(y, z))
}

pub fn solve(input: &str) {
  let moons = input
    .lines()
    .map(|line| {
      let p = line.trim().into();
      let v = Vector { x: 0, y: 0, z: 0 };

      Moon { p, v }
    })
    .collect_vec();

  dbg!(solve_01(&moons, 999));

  dbg!(solve_02(&moons));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one() {
    let input = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    let moons = input
      .lines()
      .map(|line| {
        let p = line.trim().into();
        let v = Vector { x: 0, y: 0, z: 0 };

        Moon { p, v }
      })
      .collect_vec();

    assert_eq!(solve_01(&moons, 99), 1940);
  }

  #[test]
  fn part_two() {
    let input = "<x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>";

    let moons = input
      .lines()
      .map(|line| {
        let p = line.trim().into();
        let v = Vector { x: 0, y: 0, z: 0 };

        Moon { p, v }
      })
      .collect_vec();

    assert_eq!(solve_02(&moons), 4_686_774_924);
  }
}
