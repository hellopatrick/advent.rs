#[derive(Debug)]
pub struct VM {
  memory: Vec<usize>,
  pub ip: usize,
}

#[derive(Debug)]
pub enum Instruction {
  Halt,
  Add,
  Multiply,
  Unknown(usize),
}

impl VM {
  pub fn with(memory: &[usize]) -> Self {
    Self {
      memory: memory.to_vec(),
      ip: 0,
    }
  }

  pub fn op(&self) -> Instruction {
    match self.current() {
      1 => Instruction::Add,
      2 => Instruction::Multiply,
      99 => Instruction::Halt,
      n => Instruction::Unknown(n),
    }
  }

  pub fn current(&self) -> usize {
    self.memory[self.ip]
  }

  pub fn params(&self) -> (usize, usize, usize) {
    (
      self.at(self.ip + 1),
      self.at(self.ip + 2),
      self.at(self.ip + 3),
    )
  }

  pub fn at(&self, idx: usize) -> usize {
    self.memory[idx]
  }

  pub fn set(&mut self, idx: usize, val: usize) {
    self.memory[idx] = val;
  }

  pub fn run(&mut self) {
    loop {
      let op = self.op();
      match op {
        Instruction::Unknown(n) => panic!("unimplemented opcode: {}", n),
        Instruction::Halt => break,
        Instruction::Multiply => {
          let (loc_a, loc_b, loc_c) = self.params();
          let a = self.at(loc_a);
          let b = self.at(loc_b);
          self.set(loc_c, a * b);
        }
        Instruction::Add => {
          let (loc_a, loc_b, loc_c) = self.params();
          let a = self.at(loc_a);
          let b = self.at(loc_b);
          self.set(loc_c, a + b);
        }
      }
      self.ip += 4;
    }
  }
}
