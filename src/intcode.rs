use std::sync::mpsc::*;

#[derive(Debug)]
pub struct VM {
  pub memory: Vec<isize>,
  pub ip: usize,
  pub sp: usize,
  pub input: Sender<isize>,
  pub reader: Receiver<isize>,
  pub output: Receiver<isize>,
  pub writer: Sender<isize>,
  pub last_output: isize,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Address {
  Position(isize),
  Immediate(isize),
  Relative(isize),
}

impl From<(isize, isize)> for Address {
  fn from((mode, value): (isize, isize)) -> Address {
    match mode {
      0 => Address::Position(value),
      1 => Address::Immediate(value),
      2 => Address::Relative(value),
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
  Halt,
  Add(Address, Address, Address),
  Multiply(Address, Address, Address),
  Input(Address),
  Output(Address),
  JumpNZ(Address, Address),
  JumpZ(Address, Address),
  IfLess(Address, Address, Address),
  IfEqual(Address, Address, Address),
  AdjustSP(Address),
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
      Instruction::AdjustSP(_) => 2,
      Instruction::Unknown(_) => 0,
    }
  }
}

impl VM {
  pub fn new(initial_memory: &[isize]) -> Self {
    let (input, reader) = channel();
    let (writer, output) = channel();

    let mut memory = initial_memory.to_vec();
    memory.resize(4000, 0);

    Self {
      memory,
      ip: 0,
      sp: 0,
      last_output: 0,
      input,
      reader,
      output,
      writer,
    }
  }

  pub fn op(&self) -> Instruction {
    let i = self.memory[self.ip];

    let op = i % 100;

    let mode_one = (i / 100) % 10;
    let mode_two = (i / 1_000) % 10;
    let mode_three = (i / 10_000) % 10;

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
      9 => Instruction::AdjustSP((mode_one, self.memory[self.ip + 1]).into()),
      99 => Instruction::Halt,
      n => Instruction::Unknown(n),
    }
  }

  pub fn at(&self, idx: Address) -> isize {
    match idx {
      Address::Immediate(v) => v,
      Address::Position(p) => self.memory[p as usize],
      Address::Relative(r) => self.memory[(self.sp as isize + r) as usize],
    }
  }

  pub fn set(&mut self, idx: Address, val: isize) {
    match idx {
      Address::Immediate(v) => self.memory[v as usize] = val,
      Address::Position(p) => self.memory[p as usize] = val,
      Address::Relative(r) => self.memory[(self.sp as isize + r) as usize] = val,
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
          let n = self.reader.recv().unwrap();
          self.set(a, n);
        }
        Instruction::Output(a) => {
          let a = self.at(a);
          self.last_output = a;
          self.writer.send(a).unwrap_or_default();
        }
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
        Instruction::AdjustSP(a) => {
          let a = self.at(a);

          self.sp = (self.sp as isize + a) as usize;
        }
      }
      self.ip += arity;
    }
  }
}
