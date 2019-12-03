use crate::solutions::intcode::*;

fn run(mem: &[usize], noun: usize, verb: usize) -> usize {
  let mut vm = VM::with(mem);
  vm.set(1, noun);
  vm.set(2, verb);

  vm.run();

  vm.at(0)
}

fn solve_01(memory: &[usize]) -> usize {
  run(memory, 12, 2)
}

fn solve_02(memory: &[usize]) -> usize {
  for noun in 0..100 {
    for verb in 0..100 {
      let res = run(&memory, noun, verb);

      if res == 19_690_720 {
        return 100 * noun + verb;
      }
    }
  }

  0
}

fn load_initial_memory(input: &str) -> Vec<usize> {
  input.split(',').flat_map(|c| c.parse()).collect()
}

pub fn solve(input: &str) {
  let mem = load_initial_memory(input);

  let res = solve_01(&mem);
  println!("part 01: {}", res);

  let res = solve_02(&mem);
  println!("part 02: {}", res);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn part_one() {
    let ops = load_initial_memory("1,9,10,3,2,3,11,0,99,30,40,50");
    let res = run(&ops, 9, 10);

    assert_eq!(res, 3500);

    let ops = load_initial_memory("1,1,1,4,99,5,6,0,99");
    let res = run(&ops, 1, 1);

    assert_eq!(res, 30);

    let ops = load_initial_memory("1,0,0,0,99");
    let res = run(&ops, 0, 0);

    assert_eq!(res, 2);
  }
}
