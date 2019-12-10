use itertools::Itertools;

trait Distance<T> {
  fn manhattan_distance(self, t: Self) -> T;
}

type Coordinate = (i32, i32);

impl Distance<i32> for Coordinate {
  fn manhattan_distance(self, to: Coordinate) -> i32 {
    (self.0 - to.0).abs() + (self.1 - to.1).abs()
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Slope {
  dx: i32,
  dy: i32,
}

fn parse_input(input: &str) -> Vec<Coordinate> {
  input
    .lines()
    .enumerate()
    .flat_map(|(row, line)| {
      line
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(col, c)| {
          if c == '#' {
            Some((col as i32, row as i32))
          } else {
            None
          }
        })
        .collect_vec()
    })
    .collect_vec()
}

fn gcd(mut m: i32, mut n: i32) -> i32 {
  while m != 0 {
    let old_m = m;
    m = n % m;
    n = old_m;
  }

  n.abs()
}

impl Slope {
  fn from(from: Coordinate, to: Coordinate) -> Slope {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    if dx == 0 && dy == 0 {
      return Slope { dx, dy };
    }
    let div = gcd(dx, dy);
    Slope {
      dx: dx / div,
      dy: dy / div,
    }
  }

  fn angle(self) -> f32 {
    use std::f32;
    if self.dx == 0 && self.dy == 0 {
      return f32::INFINITY;
    }
    let dx = self.dx as f32;
    let dy = self.dy as f32;
    let ang = dy.atan2(dx).to_degrees();
    if ang < -90. {
      360. + ang
    } else {
      ang
    }
  }
}

fn solve_01(asteroids: &[Coordinate]) -> (usize, Coordinate) {
  asteroids
    .iter()
    .map(|from| {
      (
        asteroids
          .iter()
          .map(|to| Slope::from(*from, *to))
          .unique()
          .count()
          - 1,
        *from,
      )
    })
    .max()
    .unwrap()
}

fn solve_02(coord: Coordinate, asteroids: &[Coordinate]) -> Vec<(f32, Coordinate)> {
  let mut map = asteroids
    .iter()
    .map(|ast| {
      let sl = Slope::from(coord, *ast);
      (sl, *ast)
    })
    .into_group_map();

  let mut res = map
    .iter_mut()
    .flat_map(|(s, points)| {
      let ang = s.angle();
      points.sort_by_key(|k| k.manhattan_distance(coord));

      points
        .iter()
        .enumerate()
        .map(|(i, k)| (ang + (i as f32) * 360., *k))
        .collect_vec()
    })
    .collect_vec();

  res.sort_by(|a, b| a.partial_cmp(b).unwrap());

  res
}

pub fn solve(input: &str) {
  let asteroids = parse_input(input);

  let (part_01, location) = solve_01(&asteroids);
  dbg!(part_01);

  let res = solve_02(location, &asteroids);
  let (_, part_02) = res[199];
  dbg!(part_02);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn small() {
    let asteroids = parse_input(
      ".#..#
    .....
    #####
    ....#
    ...##",
    );

    let (count, coord) = solve_01(&asteroids);

    assert_eq!(count, 8);
    assert_eq!(coord, (3, 4));
  }

  #[test]
  fn medium() {
    let asteroids = parse_input(
      "......#.#.
      #..#.#....
      ..#######.
      .#.#.###..
      .#..#.....
      ..#....#.#
      #..#....#.
      .##.#..###
      ##...#..#.
      .#....####",
    );

    let (count, coord) = solve_01(&asteroids);

    assert_eq!(count, 33);
    assert_eq!(coord, (5, 8));
  }

  #[test]
  fn large() {
    let asteroids = parse_input(
      ".#..##.###...#######
      ##.############..##.
      .#.######.########.#
      .###.#######.####.#.
      #####.##.#.##.###.##
      ..#####..#.#########
      ####################
      #.####....###.#.#.##
      ##.#################
      #####.##.###..####..
      ..######..##.#######
      ####.##.####...##..#
      .#####..#.######.###
      ##...#.##########...
      #.##########.#######
      .####.#.###.###.#.##
      ....##.##.###..#####
      .#.#.###########.###
      #.#.#.#####.####.###
      ###.##.####.##.#..##",
    );

    let (count, coord) = solve_01(&asteroids);

    assert_eq!(count, 210);
    assert_eq!(coord, (11, 13));

    let res = solve_02(coord, &asteroids);
    res
      .iter()
      .enumerate()
      .for_each(|(i, c)| println!("{}, {:?}", i, c));
    assert_eq!(res[199].1, (8, 2));
  }
}
