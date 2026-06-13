use crate::asm;

type Instr = u32;

const INSTR_MASK: u32 = 0x1F_FFFF;

const REG_IO_STATUS: usize = 0xEE;
const IO_OK: i32 = 0;
const IO_INVALID_DEVICE: i32 = 1;
const IO_INVALID_COMMAND: i32 = 2;
const IO_BAD_VALUE: i32 = 3;
const IO_UNAVAILABLE: i32 = 4;
const IO_SCREEN_OUT_OF_BOUNDS: i32 = 0x10;

pub fn run_craw(source: &str) -> String {
  let lines: Vec<String> = source
    .lines()
    .map(|s| s.to_string())
    .collect();

  let program = match asm::assemble(&lines) {
    Ok(program) => program,
    Err(err) => return format!("Assembly failed:\n{err}"),
  };

  let decoded = predecode(&program);

  let mut cpu = WebCpu::new();

  match cpu.execute(&decoded, &[0], true, 1_000_000) {
    Ok(()) => cpu.output,
    Err(err) => format!("Runtime error:\n{err}\n\n{}", cpu.output),
  }
}

#[derive(Clone, Copy)]
struct Decoded {
  core: u8,
  mode: u8,
  a: u8,
  b: u8,
  op5: u8,
  imm16: u16,
  raw21: u32,
  is_nop: bool,
  is_stp: bool,
  is_inp: bool,
}

fn predecode(program: &[Instr]) -> Vec<Decoded> {
  let mut out = Vec::with_capacity(program.len());

  for &instr in program {
    let v = instr & INSTR_MASK;

    let core = ((v >> 19) & 0b11) as u8;
    let mode = ((v >> 16) & 0b111) as u8;
    let a = ((v >> 8) & 0xFF) as u8;
    let b = (v & 0xFF) as u8;

    let op5 = ((v >> 16) & 0b1_1111) as u8;
    let imm16 = (v & 0xFFFF) as u16;

    let is_nop = v == 0;
    let is_inp = v == 0b011000000000000000000;
    let is_stp = v == 0b011111111111111111111;

    out.push(Decoded {
      core,
      mode,
      a,
      b,
      op5,
      imm16,
      raw21: v,
      is_nop,
      is_stp,
      is_inp,
    });
  }

  out
}

struct WebCpu {
  regs: [i32; 256],
  memory: Vec<i32>,
  mem_addr: usize,

  label_pos: Vec<i32>,
  label_epoch: Vec<u32>,
  epoch: u32,
  skip: Vec<u16>,

  input_pos: usize,
  output: String,
}

impl WebCpu {
  fn new() -> Self {
    let mut regs = [0i32; 256];

    regs[0xF0] = 314159265;
    regs[0xF1] = 271828182;
    regs[0xF2] = 30102999;
    regs[0xF3] = 47712125;
    regs[0xF4] = 69897000;
    regs[0xF5] = 69314718;
    regs[0xF6] = 109861228;
    regs[0xF7] = 160943791;
    regs[0xF8] = 141421356;
    regs[0xF9] = 173205080;
    regs[0xFA] = 223606797;
    regs[0xFB] = 125992105;
    regs[0xFC] = 144224957;
    regs[0xFD] = 170997594;
    regs[0xFE] = 2147483647;

    Self {
      regs,
      memory: vec![0; 65536],
      mem_addr: 0,

      label_pos: vec![-1; 65536],
      label_epoch: vec![0; 65536],
      epoch: 1,
      skip: Vec::new(),

      input_pos: 0,
      output: String::new(),
    }
  }

  fn begin_run(&mut self) {
    self.skip.clear();
    self.input_pos = 0;
    self.output.clear();

    self.epoch = self.epoch.wrapping_add(1);

    if self.epoch == 0 {
      self.label_epoch.fill(0);
      self.epoch = 1;
    }

    for i in 0..256 {
      if i >= 0xF0 && i <= 0xFE {
        continue;
      }

      self.regs[i] = 0;
    }
  }

  fn label_get(&self, id: u16) -> i32 {
    let idx = id as usize;

    if self.label_epoch[idx] == self.epoch {
      self.label_pos[idx]
    } else {
      -1
    }
  }

  fn label_set(&mut self, id: u16, pc: i32) {
    let idx = id as usize;
    self.label_epoch[idx] = self.epoch;
    self.label_pos[idx] = pc;
  }

  fn label_rmv(&mut self, id: u16) {
    let idx = id as usize;

    if self.label_epoch[idx] == self.epoch {
      self.label_epoch[idx] = 0;
      self.label_pos[idx] = -1;
    }
  }

  fn write_reg(&mut self, dst: usize, value: i32) {
    if dst == 0 {
      return;
    }

    self.regs[dst] = value;

    if dst == 0xEF {
      if let Some(c) = char::from_u32(value as u32) {
        self.output.push(c);
      }
    }

    if dst == 0xFF {
      self.output.push_str(&format!("[out:{:02X}]", value & 0xFF));
    }
  }

  fn handle_io(&mut self, device: u8, command: u8, reg: u8) {
    let r = reg as usize;
    let value = self.regs[r];

    self.regs[REG_IO_STATUS] = IO_OK;

    match device {
      // text
      0x0 => match command {
        // char
        0x0 => {
          if let Some(c) = char::from_u32(value as u32) {
            self.output.push(c);
          }
        }

        // int
        0x1 => {
          self.output.push_str(&value.to_string());
        }

        // newline
        0x2 => {
          self.output.push('\n');
        }

        // hex
        0x3 => {
          self.output.push_str(&format!("{:08X}", value));
        }

        // error
        0x4 => {
          if self.regs[238] != 0 {
            self.output.push_str(&format!("Crawssembly Error Code {}", self.regs[238]));
          }
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // time
      0x1 => match command {
        // unix / low
        0x0 | 0x1 => {
          self.write_reg(r, 0);
          self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
        }

        // sleep
        0x2 => {
          // ignored in browser demo
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // screen
      0x2 => {
        self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
      }

      // keyboard
      0x3 => match command {
        0x0 => {
          self.write_reg(r, 0);
          self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // mouse
      0x4 => {
        self.write_reg(r, 0);
        self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
      }

      // speaker
      0x5 => {
        self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
      }

      // mem
      0x6 => match command {
        // addr
        0x0 => {
          if value < 0 || value > 65535 {
            self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
          } else {
            self.mem_addr = value as usize;
          }
        }

        // read
        0x1 => {
          self.write_reg(r, self.memory[self.mem_addr]);
        }

        // write
        0x2 => {
          self.memory[self.mem_addr] = value;
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // disk
      0x7 => {
        self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
      }

      // speech
      0x8 => {
        self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
      }

      _ => {
        self.regs[REG_IO_STATUS] = IO_INVALID_DEVICE;
      }
    }
  }

  fn execute(
    &mut self,
    program: &[Decoded],
    input: &[i32],
    looped: bool,
    max_ticks: u64,
  ) -> Result<(), String> {
    self.begin_run();

    if !input.is_empty() {
      self.input_pos = 0;
      self.regs[0] = input[0];
    }

    let mut pc: i32 = 0;
    let mut tick: u64 = 0;

    loop {
      if tick >= max_ticks {
        return Err(format!("Stopped after {max_ticks} ticks. Possible infinite loop. Consider downloading Crawssembly for infinite runtime."));
      }

      if pc < 0 || pc >= program.len() as i32 {
        break;
      }

      let d = program[pc as usize];
      tick += 1;

      let (next_pc, did_stop) = self.step(d, pc, input, looped);
      pc = next_pc;

      if did_stop {
        break;
      }
    }

    Ok(())
  }

  fn step(
    &mut self,
    d: Decoded,
    pc: i32,
    input: &[i32],
    looped: bool,
  ) -> (i32, bool) {
    if !self.skip.is_empty() {
      if d.op5 == 0b01101 {
        let id = d.imm16;

        if let Some(pos) = self.skip.iter().position(|&x| x == id) {
          self.skip.remove(pos);
        }
      }

      return (pc + 1, false);
    }

    if d.is_stp {
      return (pc, true);
    }

    if d.is_nop {
      return (pc + 1, false);
    }

    if d.is_inp {
      if !input.is_empty() {
        self.input_pos += 1;

        if self.input_pos >= input.len() {
          if looped {
            self.input_pos = 0;
            self.regs[0] = input[self.input_pos];
          } else {
            self.regs[0] = 0;
          }
        } else {
          self.regs[0] = input[self.input_pos];
        }
      }

      return (pc + 1, false);
    }

    // ALU
    if d.core == 0b10 || d.core == 0b11 {
      let va = if d.core == 0b10 {
        self.regs[d.a as usize]
      } else {
        imm8(d.a)
      };

      let vb = self.regs[d.b as usize];

      self.regs[1] = alu(d.mode, va, vb);

      return (pc + 1, false);
    }

    // IO
    if d.op5 == 0b01110 {
      let device = (d.imm16 >> 12) as u8;
      let command = ((d.imm16 >> 8) & 0x0F) as u8;
      let reg = (d.imm16 & 0xFF) as u8;

      self.handle_io(device, command, reg);

      return (pc + 1, false);
    }

    // SAV
    if d.mode == 0b000 {
      if d.core == 0b00 {
        let src = self.regs[d.a as usize];
        let dst = d.b as usize;
        self.write_reg(dst, src);

        return (pc + 1, false);
      }

      if d.core == 0b01 {
        let imm = imm8(d.a);
        let dst = d.b as usize;
        self.write_reg(dst, imm);

        return (pc + 1, false);
      }
    }

    // For first web demo: STR/RUN unsupported
    if d.core == 0b01 && d.mode == 0b001 {
      self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
      return (pc + 1, false);
    }

    match d.op5 {
      // label
      0b01111 => {
        self.label_set(d.imm16, pc);
        (pc + 1, false)
      }

      // jmp
      0b00111 => {
        let t = self.label_get(d.imm16);

        if t >= 0 {
          (t, false)
        } else {
          (pc + 1, false)
        }
      }

      // jmz
      0b00001 => {
        if self.regs[1] == 0 {
          let t = self.label_get(d.imm16);

          if t >= 0 {
            (t, false)
          } else {
            (pc + 1, false)
          }
        } else {
          (pc + 1, false)
        }
      }

      // jmg
      0b00010 => {
        if self.regs[1] > 0 {
          let t = self.label_get(d.imm16);

          if t >= 0 {
            (t, false)
          } else {
            (pc + 1, false)
          }
        } else {
          (pc + 1, false)
        }
      }

      // jml
      0b00100 => {
        if self.regs[1] < 0 {
          let t = self.label_get(d.imm16);

          if t >= 0 {
            (t, false)
          } else {
            (pc + 1, false)
          }
        } else {
          (pc + 1, false)
        }
      }

      // ifz
      0b00110 => {
        if self.regs[1] != 0 {
          self.skip.push(d.imm16);
        }

        (pc + 1, false)
      }

      // ifg
      0b00101 => {
        if self.regs[1] <= 0 {
          self.skip.push(d.imm16);
        }

        (pc + 1, false)
      }

      // ifl
      0b00011 => {
        if self.regs[1] >= 0 {
          self.skip.push(d.imm16);
        }

        (pc + 1, false)
      }

      // rmv
      0b01101 => {
        self.label_rmv(d.imm16);
        (pc + 1, false)
      }

      // fgo
      0b01011 => {
        let target_1_based = if d.imm16 == 0 {
          self.regs[1]
        } else {
          d.imm16 as i32
        };

        if target_1_based >= 1 {
          (target_1_based - 1, false)
        } else {
          (pc + 1, false)
        }
      }

      // run unsupported
      0b01010 => {
        self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
        (pc + 1, false)
      }

      _ => (pc + 1, false),
    }
  }
}

fn imm8(v: u8) -> i32 {
  (v as i8) as i32
}

fn alu(mode: u8, a: i32, b: i32) -> i32 {
  match mode {
    0b000 => !a,
    0b001 => a & b,
    0b010 => a | b,
    0b011 => a ^ b,
    0b100 => a.wrapping_shl((b & 31) as u32),
    0b101 => ((a as u32) >> (b & 31)) as i32,
    0b110 => a >> (b & 31),
    0b111 => a.wrapping_add(b),
    _ => unreachable!(),
  }
}
