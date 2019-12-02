fn fuel_for_mass(mass: u32) -> u32 {
  (mass / 3).saturating_sub(2)
}

fn masses<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
  input.lines().flat_map(|line| line.parse())
}

fn solve_01(input: &str) -> u32 {
  masses(input).map(fuel_for_mass).sum()
}

fn total_fuel_for_mass(mass: u32) -> u32 {
  let mut total = 0;
  let mut new_mass = mass;

  while new_mass > 0 {
    new_mass = fuel_for_mass(new_mass);
    total += new_mass;
  }

  total
}

fn solve_02(input: &str) -> u32 {
  masses(input).map(total_fuel_for_mass).sum()
}

pub fn solve(input: &str) {
  let one = solve_01(input);

  println!("part 1: {}", one);

  let two = solve_02(input);

  println!("part 2: {}", two);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn part_one() {
    assert_eq!(fuel_for_mass(12), 2);
    assert_eq!(fuel_for_mass(14), 2);
    assert_eq!(fuel_for_mass(1_969), 654);
    assert_eq!(fuel_for_mass(100_756), 33_583);
  }

  #[test]
  fn part_two() {
    assert_eq!(total_fuel_for_mass(14), 2);
    assert_eq!(total_fuel_for_mass(1_969), 966);
    assert_eq!(total_fuel_for_mass(100_756), 50_346);
  }
}
