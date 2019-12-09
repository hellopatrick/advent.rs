use crate::intcode::*;

fn load_initial_memory(input: &str) -> Vec<isize> {
  input.split(',').flat_map(|c| c.trim().parse()).collect()
}

pub fn solve(input: &str) {
  let mem = load_initial_memory(input);

  let mut vm = VM::new(&mem);
  vm.input.send(1).unwrap();
  let res = vm.run();

  println!("part one: {}", res);

  let mut vm = VM::new(&mem);
  vm.input.send(5).unwrap();
  let res = vm.run();

  println!("part two: {}", res);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn part_two() {
    let mem = load_initial_memory(
      "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,
      20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,
      1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,
      1,46,98,99",
    );
    let mut vm = VM::new(&mem);
    vm.input.send(5).unwrap();
    let res = vm.run();

    assert_eq!(res, 999);

    let mut vm = VM::new(&mem);
    vm.input.send(8).unwrap();
    let res = vm.run();

    assert_eq!(res, 1000);

    let mut vm = VM::new(&mem);
    vm.input.send(13).unwrap();
    let res = vm.run();

    assert_eq!(res, 1001);
  }
}
