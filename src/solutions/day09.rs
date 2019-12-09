use crate::intcode::*;

fn load_initial_memory(input: &str) -> Vec<isize> {
  input.split(',').flat_map(|c| c.trim().parse()).collect()
}

pub fn solve(input: &str) {
  let mem = load_initial_memory(input);
  let mut vm = VM::new(&mem);
  vm.input.send(1).unwrap();

  let part_one = vm.run();

  dbg!(part_one);

  let mut vm = VM::new(&mem);
  vm.input.send(2).unwrap();

  let part_two = vm.run();

  dbg!(part_two);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn quine() {
    let mem = load_initial_memory("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut vm = VM::new(&mem);
    let res = vm.run();

    assert!(mem.iter().zip(vm.output.iter()).all(|(&a, b)| a == b));
    assert_eq!(res, 99);
  }

  #[test]
  fn big_numbers() {
    let mem = load_initial_memory("104,1125899906842624,99");
    let mut vm = VM::new(&mem);
    let res = vm.run();

    assert_eq!(res, 1_125_899_906_842_624);
  }

  #[test]
  fn big_numbers_two() {
    let mem = load_initial_memory("1102,34915192,34915192,7,4,7,99,0");
    let mut vm = VM::new(&mem);
    let res = vm.run();

    assert_eq!(res, 1_219_070_632_396_864);
  }
}
