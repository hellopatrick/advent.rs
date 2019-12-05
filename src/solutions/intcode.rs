#[derive(Debug)]
pub struct VM {
  memory: Vec<isize>,
  pub ip: usize,
}

#[derive(Debug)]
pub enum Parameter {
  Position(isize),
  Immediate(isize),
}

impl From<(isize, isize)> for Parameter {
  fn from((mode, value): (isize, isize)) -> Parameter {
    match mode {
      0 => Parameter::Position(value),
      1 => Parameter::Immediate(value),
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub enum Instruction {
  Halt,
  Add(Parameter, Parameter, Parameter),
  Multiply(Parameter, Parameter, Parameter),
  Input(Parameter),
  Output(Parameter),
  JumpNZ(Parameter, Parameter),
  JumpZ(Parameter, Parameter),
  IfLess(Parameter, Parameter, Parameter),
  IfEqual(Parameter, Parameter, Parameter),
  Unknown(isize),
}

impl Instruction {
  fn arity(&self) -> usize {
    match self {
      Instruction::Halt => 1,
      Instruction::Add(_, _, _) => 4,
      Instruction::Multiply(_, _, _) => 4,
      Instruction::Input(_) => 2,
      Instruction::Output(_) => 2,
      Instruction::JumpNZ(_, _) => 3,
      Instruction::JumpZ(_, _) => 3,
      Instruction::IfLess(_, _, _) => 4,
      Instruction::IfEqual(_, _, _) => 4,
      Instruction::Unknown(_) => 0,
    }
  }
}

impl VM {
  pub fn with(memory: &[isize]) -> Self {
    Self {
      memory: memory.to_vec(),
      ip: 0,
    }
  }

  pub fn op(&self) -> Instruction {
    let i = self.memory[self.ip];

    let op = i % 100;

    let mode_one = (i / 100) % 10;
    let mode_two = (i / 1000) % 10;
    let mode_three = (i / 10000) % 10;

    match op {
      1 => Instruction::Add(
        (mode_one, self.memory[self.ip + 1]).into(),
        (mode_two, self.memory[self.ip + 2]).into(),
        (mode_three, self.memory[self.ip + 3]).into(),
      ),
      2 => Instruction::Multiply(
        (mode_one, self.memory[self.ip + 1]).into(),
        (mode_two, self.memory[self.ip + 2]).into(),
        (mode_three, self.memory[self.ip + 3]).into(),
      ),
      3 => Instruction::Input((mode_one, self.memory[self.ip + 1]).into()),
      4 => Instruction::Output((mode_one, self.memory[self.ip + 1]).into()),
      5 => Instruction::JumpNZ(
        (mode_one, self.memory[self.ip + 1]).into(),
        (mode_two, self.memory[self.ip + 2]).into(),
      ),
      6 => Instruction::JumpZ(
        (mode_one, self.memory[self.ip + 1]).into(),
        (mode_two, self.memory[self.ip + 2]).into(),
      ),
      7 => Instruction::IfLess(
        (mode_one, self.memory[self.ip + 1]).into(),
        (mode_two, self.memory[self.ip + 2]).into(),
        (mode_three, self.memory[self.ip + 3]).into(),
      ),
      8 => Instruction::IfEqual(
        (mode_one, self.memory[self.ip + 1]).into(),
        (mode_two, self.memory[self.ip + 2]).into(),
        (mode_three, self.memory[self.ip + 3]).into(),
      ),
      99 => Instruction::Halt,
      n => Instruction::Unknown(n),
    }
  }

  pub fn at(&self, idx: Parameter) -> isize {
    match idx {
      Parameter::Immediate(v) => v,
      Parameter::Position(p) => self.memory[p as usize],
    }
  }

  pub fn set(&mut self, idx: Parameter, val: isize) {
    match idx {
      Parameter::Immediate(v) => self.memory[v as usize] = val,
      Parameter::Position(p) => self.memory[p as usize] = val,
    };
  }

  pub fn run(&mut self) {
    loop {
      let op = self.op();
      let arity = op.arity();
      match op {
        Instruction::Unknown(n) => panic!("unimplemented opcode: {}", n),
        Instruction::Halt => break,
        Instruction::Multiply(a, b, c) => {
          let a = self.at(a);
          let b = self.at(b);
          self.set(c, a * b);
        }
        Instruction::Add(a, b, c) => {
          let a = self.at(a);
          let b = self.at(b);
          self.set(c, a + b);
        }
        Instruction::Input(a) => {
          use std::io;
          use std::io::Write;

          let mut input = String::new();
          print!("> ");
          io::stdout().flush().expect("flushed");
          io::stdin().read_line(&mut input).unwrap();
          let n: isize = input.trim().parse().unwrap();

          self.set(a, n);
        }
        Instruction::Output(a) => println!("{}", self.at(a)),
        Instruction::JumpNZ(a, b) => {
          let a = self.at(a);
          let b = self.at(b);

          if a != 0 {
            self.ip = b as usize;
            continue;
          }
        }
        Instruction::JumpZ(a, b) => {
          let a = self.at(a);
          let b = self.at(b);

          if a == 0 {
            self.ip = b as usize;
            continue;
          }
        }
        Instruction::IfLess(a, b, c) => {
          let a = self.at(a);
          let b = self.at(b);
          self.set(c, if a < b { 1 } else { 0 });
        }
        Instruction::IfEqual(a, b, c) => {
          let a = self.at(a);
          let b = self.at(b);
          self.set(c, if a == b { 1 } else { 0 });
        }
      }
      self.ip += arity;
    }
  }
}
