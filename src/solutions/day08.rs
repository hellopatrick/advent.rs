fn solve_01(input: &[u32], width: usize, height: usize) -> usize {
  let m = input
    .chunks_exact(width * height)
    .min_by_key(|&c| c.iter().filter(|&&d| d == 0).count())
    .unwrap();

  m.iter().filter(|&&d| d == 1).count() * m.iter().filter(|&&d| d == 2).count()
}

fn solve_02(input: &[u32], width: usize, height: usize) -> Vec<u32> {
  input
    .chunks_exact(width * height)
    .fold(vec![2; width * height], |acc, layer| {
      acc
        .iter()
        .zip(layer)
        .map(|(curr, layer)| if *curr == 2 { *layer } else { *curr })
        .collect()
    })
}

fn input_gen(input: &str) -> Vec<u32> {
  input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn solve(input: &str) {
  let r = input_gen(input);

  dbg!(solve_01(&r, 25, 6));

  let img = solve_02(&r, 25, 6);

  img.chunks(25).for_each(|row| {
    for &d in row {
      print!("{}", if d == 0 { " " } else { "*" })
    }
    println!();
  })
}
