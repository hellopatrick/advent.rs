use crate::intcode::*;

fn load_initial_memory(input: &str) -> Vec<isize> {
  input.split(',').flat_map(|c| c.parse()).collect()
}

pub fn solve(input: &str) {
  let mem = load_initial_memory(input);

  let mut vm = VM::new(&mem);
  vm.run();
}
