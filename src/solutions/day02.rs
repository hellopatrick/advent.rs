fn run(opcodes: &mut [usize], noun: usize, verb: usize) -> usize {
  opcodes[1] = noun;
  opcodes[2] = verb;

  for i in (0..opcodes.len()).step_by(4) {
    match opcodes[i] {
      1 => {
        let a = opcodes[i + 1];
        let b = opcodes[i + 2];
        let addr = opcodes[i + 3];
        opcodes[addr] = opcodes[a] + opcodes[b]
      }
      2 => {
        let a = opcodes[i + 1];
        let b = opcodes[i + 2];
        let addr = opcodes[i + 3];
        opcodes[addr] = opcodes[a] * opcodes[b]
      }
      99 => break,
      n => panic!("unimplemented opcode: {}", n),
    }
  }

  opcodes[0]
}

fn solve_01(opcodes: &mut [usize]) -> usize {
  run(opcodes, 12, 2)
}

fn solve_02(opcodes: &[usize]) -> usize {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut op = opcodes.to_vec();
      let res = run(&mut op, noun, verb);

      if res == 1969_0720 {
        return 100 * noun + verb;
      }
    }
  }

  0
}

fn to_opcodes(input: &str) -> Vec<usize> {
  input.split(',').flat_map(|c| c.parse()).collect()
}

pub fn solve(input: &str) {
  let orig = to_opcodes(input);

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
    let mut ops = to_opcodes("1,9,10,3,2,3,11,0,99,30,40,50");
    let res = run(&mut ops, 9, 10);

    assert_eq!(res, 3500);

    let mut ops = to_opcodes("1,1,1,4,99,5,6,0,99");
    let res = run(&mut ops, 1, 1);

    assert_eq!(res, 30);

    let mut ops = to_opcodes("1,0,0,0,99");
    let res = run(&mut ops, 0, 0);

    assert_eq!(res, 2);
  }
}
