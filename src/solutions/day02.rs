use crate::solutions::intcode::*;

fn run(opcodes: &mut [usize], noun: usize, verb: usize) -> usize {
  let mut vm = VM::with(opcodes);
  vm.set(1, noun);
  vm.set(2, verb);

  vm.run();

  vm.at(0)
}

fn solve_01(opcodes: &mut [usize]) -> usize {
  run(opcodes, 12, 2)
}

fn solve_02(opcodes: &[usize]) -> usize {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut op = opcodes.to_vec();
      let res = run(&mut op, noun, verb);

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
  let orig = load_initial_memory(input);

  let mut o = orig.clone();
  let res = solve_01(&mut o);
  println!("part 01: {}", res);

  let o = orig.clone();
  let res = solve_02(&o);
  println!("part 02: {}", res);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn part_one() {
    let mut ops = load_initial_memory("1,9,10,3,2,3,11,0,99,30,40,50");
    let res = run(&mut ops, 9, 10);

    assert_eq!(res, 3500);

    let mut ops = load_initial_memory("1,1,1,4,99,5,6,0,99");
    let res = run(&mut ops, 1, 1);

    assert_eq!(res, 30);

    let mut ops = load_initial_memory("1,0,0,0,99");
    let res = run(&mut ops, 0, 0);

    assert_eq!(res, 2);
  }
}
