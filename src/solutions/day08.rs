use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum Pixel {
  Transparent,
  Black,
  White,
}

impl From<char> for Pixel {
  fn from(c: char) -> Pixel {
    match c {
      '0' => Pixel::Black,
      '1' => Pixel::White,
      '2' => Pixel::Transparent,
      _ => unreachable!(),
    }
  }
}

impl fmt::Display for Pixel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Pixel::White => write!(f, "▒"),
      _ => write!(f, " "),
    }
  }
}

impl Pixel {
  fn paint(self, layer: Pixel) -> Pixel {
    match self {
      Pixel::Transparent => layer,
      n => n,
    }
  }
}

fn solve_01(input: &[Pixel], width: usize, height: usize) -> usize {
  let m = input
    .chunks_exact(width * height)
    .min_by_key(|c| c.iter().filter(|&&d| d == Pixel::Black).count())
    .unwrap();

  m.iter().filter(|d| **d == Pixel::White).count()
    * m.iter().filter(|d| **d == Pixel::Black).count()
}

fn solve_02(input: &[Pixel], width: usize, height: usize) -> Vec<Pixel> {
  input
    .chunks_exact(width * height)
    .fold(vec![Pixel::Transparent; width * height], |acc, layer| {
      acc
        .iter()
        .zip(layer)
        .map(|(&curr, &layer)| curr.paint(layer))
        .collect()
    })
}

fn input_gen(input: &str) -> Vec<Pixel> {
  input.chars().map(|c| c.into()).collect()
}

pub fn solve(input: &str) {
  let r = input_gen(input);

  dbg!(solve_01(&r, 25, 6));

  let img = solve_02(&r, 25, 6);

  img.chunks(25).for_each(|row| {
    for &pixel in row {
      print!("{}", pixel);
    }
    println!();
  })
}
