use crate::intcode::*;

fn load_initial_memory(input: &str) -> Vec<isize> {
  input.split(',').flat_map(|c| c.trim().parse()).collect()
}

pub fn solve(input: &str) {
  let mem = load_initial_memory(input);

  let mut vm = VM::new(&mem, "1\n".as_bytes());
  vm.run();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one() {
    let input = include_str!("../../inputs/day05.txt");
    let mem = load_initial_memory(input);

    let mut vm = VM::new(&mem, "1\n".as_bytes());
    vm.run();
  }
}
