#![allow(dead_code)]

pub struct Load;

struct LoadHlD16 {
  param: u16,
}

trait Instruction {
  fn execute(&self, cpu: &mut CPU);
}

trait InstructionMagic : Instruction {
  fn new(rom: &mut Box<MemoryUnit>) -> Box<Instruction> { panic!("invalid opcode, {}", rom.read_word()); }
}

impl Instruction for LoadHlD16 {
  fn execute(&self, cpu: &mut CPU) {
    cpu.reg_h = ((self.param >> 8) & 0xff) as u8;
    cpu.reg_l = ((self.param >> 0) & 0xff) as u8;
  }
}

impl InstructionMagic for LoadHlD16 {
  fn new(rom: &mut Box<MemoryUnit>) -> Box<Instruction> {
    Box::new(LoadHlD16 {
      param: rom.read_dword(),
    })
  }
}

trait MemoryUnit {
  extern fn read_word(&mut self) -> u8;
  extern fn read_dword(&mut self) -> u16;
}

struct ROM {
  memory: Vec<u8>,
  read_head: usize,
}

impl ROM {
  pub fn new() -> ROM {
    ROM {
      memory: Vec::new(),
      read_head: 0,
    }
  }
}

impl MemoryUnit for ROM {
  extern fn read_word(&mut self) -> u8 {
    self.read_head += 1;
    return self.memory[self.read_head - 1];
  }

 extern fn read_dword(&mut self) -> u16 {
    let a16 = self.read_word() as u16;
    let b16 = self.read_word() as u16;

    (b16 << 8) | a16
  }
}

pub struct CPU {
  reg_h : u8,
  reg_l : u8,
}

fn main() { }
