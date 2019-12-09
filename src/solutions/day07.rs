use crate::intcode::*;
use itertools::*;
use std::thread;

fn connect(a: &mut VM, b: &mut VM) {
  use std::sync::mpsc::*;

  let (tx, rx) = channel();
  a.writer = tx.clone();
  b.input = tx.clone();
  b.reader = rx;
}

fn run_01(mem: &[isize], initial: &[isize]) -> isize {
  let mut vm_01 = VM::new(mem);
  let mut vm_02 = VM::new(mem);
  let mut vm_03 = VM::new(mem);
  let mut vm_04 = VM::new(mem);
  let mut vm_05 = VM::new(mem);

  connect(&mut vm_01, &mut vm_02);
  connect(&mut vm_02, &mut vm_03);
  connect(&mut vm_03, &mut vm_04);
  connect(&mut vm_04, &mut vm_05);

  vm_01.input.send(initial[0]).unwrap();
  vm_01.input.send(0).unwrap();

  vm_02.input.send(initial[1]).unwrap();
  vm_03.input.send(initial[2]).unwrap();
  vm_04.input.send(initial[3]).unwrap();
  vm_05.input.send(initial[4]).unwrap();

  thread::spawn(move || vm_01.run());

  thread::spawn(move || vm_02.run());

  thread::spawn(move || vm_03.run());

  thread::spawn(move || vm_04.run());

  let fifth = thread::spawn(move || vm_05.run());

  fifth.join().unwrap()
}

fn solve_01(mem: &[isize]) -> isize {
  (0..5)
    .permutations(5)
    .map(|permutation| run_01(mem, &permutation))
    .max()
    .expect("has a max")
}

fn run_02(mem: &[isize], initial: &[isize]) -> isize {
  let mut vm_01 = VM::new(mem);
  let mut vm_02 = VM::new(mem);
  let mut vm_03 = VM::new(mem);
  let mut vm_04 = VM::new(mem);
  let mut vm_05 = VM::new(mem);

  connect(&mut vm_05, &mut vm_01);
  connect(&mut vm_01, &mut vm_02);
  connect(&mut vm_02, &mut vm_03);
  connect(&mut vm_03, &mut vm_04);
  connect(&mut vm_04, &mut vm_05);

  vm_01.input.send(initial[0]).unwrap();
  vm_01.input.send(0).unwrap();

  vm_02.input.send(initial[1]).unwrap();
  vm_03.input.send(initial[2]).unwrap();
  vm_04.input.send(initial[3]).unwrap();
  vm_05.input.send(initial[4]).unwrap();

  thread::spawn(move || vm_01.run());

  thread::spawn(move || vm_02.run());

  thread::spawn(move || vm_03.run());

  thread::spawn(move || vm_04.run());

  let fifth = thread::spawn(move || vm_05.run());

  fifth.join().unwrap()
}

fn solve_02(mem: &[isize]) -> isize {
  (5..10)
    .permutations(5)
    .map(|permutation| run_02(mem, &permutation))
    .max()
    .expect("has a max")
}

fn load_initial_memory(input: &str) -> Vec<isize> {
  input.split(',').flat_map(|c| c.trim().parse()).collect()
}

pub fn solve(input: &str) {
  let mem = load_initial_memory(input);

  let res = solve_01(&mem);

  println!("part one: {}", res);

  let res = solve_02(&mem);

  println!("part two: {}", res);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one() {
    let mem = load_initial_memory("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    let inputs = vec![4, 3, 2, 1, 0];
    let last = run_01(&mem, &inputs);
    assert_eq!(last, 43_210);

    let mem = load_initial_memory(
      "3,23,3,24,1002,24,10,24,1002,23,
      -1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
    );
    let inputs = vec![0, 1, 2, 3, 4];
    let last = run_01(&mem, &inputs);

    assert_eq!(last, 54_321);

    let mem = load_initial_memory(
      "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,
      31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
    );
    let inputs = vec![1, 0, 4, 3, 2];
    let last = run_01(&mem, &inputs);

    assert_eq!(last, 65_210);
  }

  #[test]
  fn part_two() {
    let mem = load_initial_memory(
      "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
      -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
      53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
    );
    let inputs = vec![9, 7, 8, 5, 6];

    let res = run_02(&mem, &inputs);

    assert_eq!(res, 18_216);

    let mem = load_initial_memory(
      "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
      27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    );
    let inputs = vec![9, 8, 7, 6, 5];

    let res = run_02(&mem, &inputs);

    assert_eq!(res, 139_629_729);
  }
}
