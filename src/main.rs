// Crawssembly Assembler (single-file, spec-driven)
// Reads:  assembly.craw
// Writes: program.bin  (u32 little-endian per instruction; only low 21 bits used)
//
// Flags:
//   --dump         Print assembled instructions alongside source lines
//   --dump-decoded Print core/mode/A/B decode in dump output too
//
// Supports:
// - Comments with # or ;
// - Empty/comment-only lines -> nop
// - Registers r00..rff (hex, case-insensitive)
// - Immediates: signed-magnitude encoding for imm8 (-128..127) used by sav/cal
// - ALU: not/and/or/xor/shl/shr/sar/add
// - Control/labels: label defs (name: or name alone), jmp/jmz/jmg/jml, ifz/ifg/ifl, rmv
// - fgo <line> (16-bit, fgo 0 special)
// - str <line_start> <block> (8-bit + 8-bit; block 0 => VM uses r01)
// - run <block> (16-bit; block 0 => VM uses r01)
// - Raw binary lines: "0b0101..." or 21-bit "0101..." accepted as-is

use std::collections::{HashMap, HashSet};
use std::fs;

mod vm;

type Instr = u32;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.iter().any(|a| a == "--help" || a == "-h") {
    print_help();
    return;
  }

  if args.iter().any(|a| a == "--decode") {
    decode_interactive();
    return;
  }

  let dump = args.iter().any(|a| a == "--dump");
  let dump_decoded = args.iter().any(|a| a == "--dump-decoded");
  let debug = args.iter().any(|a| a == "--debug");
  let plain = !args.iter().any(|a| a == "--tui");
  let audio = args.iter().any(|a| a == "--audio");
  let show_stats = debug || args.iter().any(|a| a == "--stats");

  let (screen_w, screen_h) = match parse_screen_size(&args) {
    Ok(size) => size,
    Err(e) => {
      eprintln!("{e}");
      std::process::exit(1);
    }
  };

  let file_input = match parse_input_file(&args) {
    Ok(input) => input,
    Err(e) => {
      eprintln!("{e}");
      std::process::exit(1);
    }
  };

  let input_list = file_input.unwrap_or_else(|| vec![0]);

  let vm_config = vm::VmConfig {
    screen_w,
    screen_h,
  };

  let command = first_positional(&args);

  match command.as_deref() {
    Some("compile") => {
      let Some(input_path) = positional_after(&args, "compile") else {
        eprintln!("Missing input file.\n");
        print_help();
        std::process::exit(1);
      };

      if let Err(e) = assemble_file(&input_path, "program.bin", dump, dump_decoded) {
        eprintln!("{e}");
        std::process::exit(1);
      }
    }

    Some("run") => {
      let input_path = positional_after(&args, "run");

      if let Some(path) = input_path {
        if let Err(e) = assemble_file(&path, "program.bin", dump, dump_decoded) {
          eprintln!("{e}");
          std::process::exit(1);
        }
      }

      if let Err(e) = vm::run_vm_with_options("program.bin", plain, audio, -1, input_list.clone(), true, show_stats, vm_config) {
        eprintln!("VM failed: {e}");
        std::process::exit(1);
      }
    }

    Some("debug") => {
      let Some(input_path) = positional_after(&args, "debug") else {
        eprintln!("Missing input file.\n");
        print_help();
        std::process::exit(1);
      };

      if let Err(e) = assemble_file(&input_path, "program.bin", dump, dump_decoded) {
        eprintln!("{e}");
        std::process::exit(1);
      }

      if let Err(e) = vm::run_vm_with_options("program.bin", plain, audio, -1, input_list.clone(), true, true, vm_config) {
        eprintln!("VM failed: {e}");
        std::process::exit(1);
      }
    }

    Some("check") => {
      let Some(input_path) = positional_after(&args, "check") else {
        eprintln!("Missing input file.\n");
        print_help();
        std::process::exit(1);
      };

      if let Err(e) = check_file(&input_path, dump, dump_decoded) {
        eprintln!("{e}");
        std::process::exit(1);
      }
    }

    Some(path) => {
      if let Err(e) = assemble_file(path, "program.bin", dump, dump_decoded) {
        eprintln!("{e}");
        std::process::exit(1);
      }

      if let Err(e) = vm::run_vm_with_options("program.bin", plain, audio, -1, input_list.clone(), true, show_stats, vm_config) {
        eprintln!("VM failed: {e}");
        std::process::exit(1);
      }
    }

    None => {
      if args.iter().any(|a| a == "--run") {
        if let Err(e) = vm::run_vm() {
          eprintln!("VM failed: {e}");
          std::process::exit(1);
        }
        return;
      }

      if let Err(e) = assemble_file("assembly.craw", "program.bin", dump, dump_decoded) {
        eprintln!("{e}");
        std::process::exit(1);
      }
    }
  }
}

fn first_positional(args: &[String]) -> Option<String> {
  let mut i = 1;

  while i < args.len() {
    let arg = &args[i];

    if is_option_with_value(arg) {
      i += 2;
      continue;
    }

    if arg.starts_with("--") {
      i += 1;
      continue;
    }

    return Some(arg.clone());
  }

  None
}

fn positional_after(args: &[String], command: &str) -> Option<String> {
  let mut seen_command = false;
  let mut i = 1;

  while i < args.len() {
    let arg = &args[i];

    if is_option_with_value(arg) {
      i += 2;
      continue;
    }

    if arg.starts_with("--") {
      i += 1;
      continue;
    }

    if arg == command {
      seen_command = true;
      i += 1;
      continue;
    }

    if seen_command {
      return Some(arg.clone());
    }

    i += 1;
  }

  None
}

fn is_option_with_value(arg: &str) -> bool {
  matches!(arg, "--screen" | "--file")
}

fn assemble_file(input_path: &str, output_path: &str, dump: bool, dump_decoded: bool) -> Result<(), String> {
  let src = fs::read_to_string(input_path)
    .map_err(|e| format!("Failed to read {input_path}: {e}"))?;

  let lines: Vec<String> = src.lines().map(|s| s.to_string()).collect();

  match assemble(&lines) {
    Ok(program) => {
      if dump {
        dump_program(&lines, &program, dump_decoded);
      }

      write_program_bin(output_path, &program)
        .map_err(|e| format!("Failed to write {output_path}: {e}"))?;

      Ok(())
    }

    Err(e) => Err(format!("Assembly failed:\n{e}"))
  }
}

fn check_file(input_path: &str, dump: bool, dump_decoded: bool) -> Result<(), String> {
  let src = fs::read_to_string(input_path)
    .map_err(|e| format!("Failed to read {input_path}: {e}"))?;

  let lines: Vec<String> = src.lines().map(|s| s.to_string()).collect();
  let program = assemble(&lines)?;

  if dump {
    dump_program(&lines, &program, dump_decoded);
  }

  println!("OK: {} instructions", program.len());
  Ok(())
}

fn print_help() {
  println!("craw");
  println!();
  println!("Usage:");
  println!("  craw <file.craw>              Assemble and run a file");
  println!("  craw check <file.craw>        Check that a file assembles");
  println!("  craw compile <file.craw>      Assemble to program.bin only");
  println!("  craw debug <file.craw>        Run with VM stats shown");
  println!("  craw run <file.craw>          Assemble and run a file");
  println!();
  println!("Options:");
  println!("  --audio                       Enable speaker/speech audio output");
  println!("  --decode                      Open the instruction decoder");
  println!("  --dump                        Show assembled instructions");
  println!("  --dump-decoded                Show decoded instruction fields with --dump");
  println!("  --file <file>                 Send file contents to register 0 during execution.");
  println!("  --help                        Show this help message");
  println!("  --screen <widthxheight>       Set virtual screen size, e.g. 128x128");
  println!("  --stats                       Show VM speed/tick statistics after running");
  println!("  --tui                         Use alternate-screen terminal mode");
  println!();
  println!("Examples:");
  println!("  craw hello.craw");
  println!("  craw debug hello.craw");
  println!("  craw compile hello.craw --dump");
  println!("  craw imageviewer.craw --file image.bmp");
  println!("  craw graphics.craw --screen 128x128 --tui");
}

fn parse_input_file(args: &[String]) -> Result<Option<Vec<i32>>, String> {
  for i in 0..args.len() {
    if args[i] == "--file" {
      let Some(path) = args.get(i + 1) else {
        return Err("--file expects a filename".to_string());
      };

      let bytes = fs::read(path)
        .map_err(|e| format!("Failed to read input file '{path}': {e}"))?;

      let input = bytes.into_iter()
        .map(|b| b as i32)
        .collect();

      return Ok(Some(input));
    }
  }

  Ok(None)
}

fn parse_screen_size(args: &[String]) -> Result<(usize, usize), String> {
  let mut width = 64;
  let mut height = 64;

  for i in 0..args.len() {
    if args[i] == "--screen" {
      let Some(size) = args.get(i + 1) else {
        return Err("--screen expects a value like 128x64".to_string());
      };

      let Some((w, h)) = size.split_once('x').or_else(|| size.split_once('X')) else {
        return Err("--screen expects a value like 128x64".to_string());
      };

      width = w.parse::<usize>()
        .map_err(|_| format!("Bad screen width: {w}"))?;

      height = h.parse::<usize>()
        .map_err(|_| format!("Bad screen height: {h}"))?;
    }
  }

  if width == 0 || height == 0 {
    return Err("Screen width and height must be greater than 0".to_string());
  }

  if width > u16::MAX as usize {
    return Err(format!("Screen width must be <= {} for terminal rendering", u16::MAX));
  }

  if height > u16::MAX as usize {
    return Err(format!("Screen height must be <= {} for terminal rendering", u16::MAX));
  }

  if height % 2 != 0 {
    return Err("Screen height must be even for terminal rendering".to_string());
  }

  Ok((width, height))
}

fn dump_program(src_lines: &[String], program: &[Instr], decoded: bool) {
  println!("LINE   BIN(21)              HEX       SOURCE");
  println!("-------------------------------------------------------------");

  let width = (program.len().max(1) as f64).log10().floor() as usize + 1;

  for (i, &instr) in program.iter().enumerate() {
    let bin = instr_to_bin21(instr);
    let hex = format!("0x{:06X}", instr & 0x1F_FFFF);
    let src = src_lines.get(i).map(|s| s.as_str()).unwrap_or("");

    if decoded {
      let (core, mode, a, b) = decode_fields(instr);
      let ops = format_operands(core, a, b);
      let d = format!(" core={:02b} mode={:03b} {} ", core, mode, ops);
      println!("{:0width$} | {} | {} |{}{}", i, bin, hex, d, src, width = width);
    } else {
      println!("{:0width$} | {} | {} | {}", i, bin, hex, src, width = width);
    }
  }
}

fn instr_to_bin21(instr: Instr) -> String {
  let v = instr & 0x1F_FFFF;
  format!("{:021b}", v)
}

fn decode_fields(instr: Instr) -> (u32, u32, u32, u32) {
  let v = instr & 0x1F_FFFF;
  let core = (v >> 19) & 0b11;
  let mode = (v >> 16) & 0b111;
  let a = (v >> 8) & 0xFF;
  let b = v & 0xFF;
  (core, mode, a, b)
}

fn write_program_bin(path: &str, program: &[Instr]) -> Result<(), String> {
  let mut bytes: Vec<u8> = Vec::with_capacity(program.len() * 4);
  for &i in program {
    bytes.extend_from_slice(&i.to_le_bytes());
  }
  fs::write(path, bytes).map_err(|e| e.to_string())
}

fn assemble(lines: &[String]) -> Result<Vec<Instr>, String> {
  let mut defs: HashSet<String> = HashSet::new();
  let mut refs: HashSet<String> = HashSet::new();
  let mut numeric_ids_used: HashSet<u16> = HashSet::new();

  let parsed = parse_lines(lines, &mut defs, &mut refs, &mut numeric_ids_used)?;

  let (label_id, defined_names) = allocate_label_ids(&parsed, &defs, &refs, &numeric_ids_used)?;

  for r in refs.iter() {
    if is_number_token(r) {
      continue;
    }
    if !defined_names.contains(r) {
      return Err(format!("Undefined label referenced: '{r}'"));
    }
  }

  let mut out: Vec<Instr> = Vec::with_capacity(parsed.len());
  for (ln, pline) in parsed.iter().enumerate() {
    let instr = encode_line(ln, pline, &label_id, &parsed)?;
    out.push(instr & 0x1F_FFFF);
  }

  Ok(out)
}

#[derive(Debug, Clone)]
enum ParsedLine {
  Raw(Instr),
  Nop,
  Inp,
  Stp,

  Sav { src: SavSrc, dst: u8 },
  Cal { op: AluOp, a: CalA, b: u8 },

  LabelDef { label: LabelRef },

  Io { device: u8, command: u8, reg: u8 },

  Jump { kind: JumpKind, label: LabelRef },
  If { kind: IfKind, label: LabelRef },
  Rmv { label: LabelRef },
  Fgo { line: u16 },
  Run { block: u16 },

  Str { start: StartRef, block: u8 },
}

#[derive(Debug, Clone)]
enum LabelRef {
  Named(String),
  Numeric(u16),
}

#[derive(Debug, Clone)]
enum StartRef {
  Named(String),
  Numeric(u8),
}

#[derive(Debug, Clone)]
enum SavSrc {
  Reg(u8),
  Imm(i32),
}

#[derive(Debug, Clone)]
enum CalA {
  Reg(u8),
  Imm(i32),
}

#[derive(Debug, Copy, Clone)]
enum AluOp {
  Not,
  And,
  Or,
  Xor,
  Shl,
  Shr,
  Sar,
  Add,
}

#[derive(Debug, Copy, Clone)]
enum JumpKind { Jmp, Jmz, Jmg, Jml }

#[derive(Debug, Copy, Clone)]
enum IfKind { Ifz, Ifg, Ifl }

fn decode_interactive() {
  use std::io::{self, Write};

  loop {
    print!("Enter 21-bit binary (or 'exit'): ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    if s == "exit" { break; }

    if s.len() != 21 || !s.chars().all(|c| c == '0' || c == '1') {
      println!("Invalid input (must be 21-bit binary)");
      continue;
    }

    let instr = u32::from_str_radix(s, 2).unwrap();
    println!("{}", describe_instruction(instr));
  }
}

fn describe_instruction(instr: Instr) -> String {
  let v = instr & 0x1F_FFFF;

  let core = (v >> 19) & 0b11;
  let mode = (v >> 16) & 0b111;
  let a = (v >> 8) & 0xFF;
  let b = v & 0xFF;

  let op5 = (v >> 16) & 0b1_1111;
  let imm16 = v & 0xFFFF;

  // Special cases first
  if v == 0 {
    return "NOP".into();
  }

  if v == 0b011000000000000000000 {
    return "INP (advance input)".into();
  }

  if v == 0b011111111111111111111 {
    return "STP (halt / return)".into();
  }

  // IO
  if op5 == 0b01110 {
    let device = (imm16 >> 12) & 0xF;
    let command = (imm16 >> 8) & 0xF;
    let reg = imm16 & 0xFF;

    return format!(
      "IO device={} command={} reg=r{:02X}",
      device, command, reg
    );
  }

  // SAV
  if mode == 0b000 {
    if core == 0b00 {
      return format!("SAV r{:02X} -> r{:02X}", a, b);
    }
    if core == 0b01 {
      return format!("SAV #{} -> r{:02X}", imm8_decode(a as u8), b);
    }
  }

  // CAL
  if core == 0b10 || core == 0b11 {
    let op = match mode {
      0b000 => "NOT",
      0b001 => "AND",
      0b010 => "OR",
      0b011 => "XOR",
      0b100 => "SHL",
      0b101 => "SHR",
      0b110 => "SAR",
      _ => "ADD",
    };

    if core == 0b10 {
      return format!("CAL {} r{:02X}, r{:02X}", op, a, b);
    } else {
      return format!("CAL {} #{}, r{:02X}", op, imm8_decode(a as u8), b);
    }
  }

  // Control ops
  match op5 {
    0b01111 => return format!("LABEL {}", imm16),
    0b00111 => return format!("JMP {}", imm16),
    0b00001 => return format!("JMZ {}", imm16),
    0b00010 => return format!("JMG {}", imm16),
    0b00100 => return format!("JML {}", imm16),
    0b00110 => return format!("IFZ {}", imm16),
    0b00101 => return format!("IFG {}", imm16),
    0b00011 => return format!("IFL {}", imm16),
    0b01101 => return format!("RMV {}", imm16),
    0b01011 => return format!("FGO {}", imm16),
    0b01010 => return format!("RUN {}", imm16),
    _ => {}
  }

  format!("UNKNOWN: {}", instr_to_bin21(instr))
}

fn parse_lines(
  lines: &[String],
  defs: &mut HashSet<String>,
  refs: &mut HashSet<String>,
  numeric_ids_used: &mut HashSet<u16>
) -> Result<Vec<ParsedLine>, String> {
  let mut out: Vec<ParsedLine> = Vec::with_capacity(lines.len());

  for (ln, raw) in lines.iter().enumerate() {
    let toks = tokenize(raw);

    if toks.is_empty() {
      out.push(ParsedLine::Nop);
      continue;
    }

    if let Some(instr) = try_parse_raw_binary(&toks[0])? {
      out.push(ParsedLine::Raw(instr));
      continue;
    }

    let head = toks[0].to_ascii_lowercase();

    if head == "nop" { out.push(ParsedLine::Nop); continue; }
    if head == "inp" { out.push(ParsedLine::Inp); continue; }
    if head == "stp" { out.push(ParsedLine::Stp); continue; }

    if toks.len() == 1 {
      if toks[0].ends_with(':') {
        let name = toks[0].trim_end_matches(':').to_string();
        if name.is_empty() {
          return Err(format!("Line {}: empty label name", ln + 1));
        }
        defs.insert(name.clone());
        out.push(ParsedLine::LabelDef { label: LabelRef::Named(name) });
        continue;
      }

      if !is_register(&toks[0]) && !is_number_token(&toks[0]) && !is_known_mnemonic(&head) {
        let name = toks[0].to_string();
        defs.insert(name.clone());
        out.push(ParsedLine::LabelDef { label: LabelRef::Named(name) });
        continue;
      }

      if is_number_token(&toks[0]) {
        let id = parse_u16(&toks[0])
          .map_err(|e| format!("Line {}: {e}", ln + 1))?;
        numeric_ids_used.insert(id);
        out.push(ParsedLine::LabelDef { label: LabelRef::Numeric(id) });
        continue;
      }
    }

    if head == "fgo" {
      if toks.len() != 2 {
        return Err(format!("Line {}: fgo expects 1 operand", ln + 1));
      }
      let n = parse_u16(&toks[1])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;
      out.push(ParsedLine::Fgo { line: n });
      continue;
    }

    if head == "io" {
      if toks.len() != 4 {
        return Err(format!("Line {}: io expects 3 operands: io <device> <command> <register>", ln + 1));
      }

      let device = parse_io_device(&toks[1])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;

      let command = parse_io_command(device, &toks[2])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;

      let reg = parse_reg(&toks[3])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;

      out.push(ParsedLine::Io { device, command, reg });
      continue;
    }

    if head == "run" {
      if toks.len() != 2 {
        return Err(format!("Line {}: run expects 1 operand", ln + 1));
      }
      let b = parse_u16(&toks[1])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;
      out.push(ParsedLine::Run { block: b });
      continue;
    }

    if head == "str" {
      if toks.len() != 3 {
        return Err(format!("Line {}: str expects 2 operands", ln + 1));
      }
      let start = if is_number_token(&toks[1]) {
        let v = parse_u8(&toks[1])
          .map_err(|e| format!("Line {}: {e}", ln + 1))?;
        StartRef::Numeric(v)
      } else {
        refs.insert(toks[1].clone());
        StartRef::Named(toks[1].clone())
      };

      let block = parse_u8(&toks[2])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;

      out.push(ParsedLine::Str { start, block });
      continue;
    }

    if matches!(head.as_str(), "jmp" | "jmz" | "jmg" | "jml" | "ifz" | "ifg" | "ifl" | "rmv") {
      if toks.len() != 2 {
        return Err(format!("Line {}: {} expects 1 operand", ln + 1, head));
      }
      let lbl = parse_label_ref(&toks[1], refs, numeric_ids_used)?;

      if head == "rmv" {
        out.push(ParsedLine::Rmv { label: lbl });
      } else if head.starts_with("jm") {
        let kind = match head.as_str() {
          "jmp" => JumpKind::Jmp,
          "jmz" => JumpKind::Jmz,
          "jmg" => JumpKind::Jmg,
          "jml" => JumpKind::Jml,
          _ => return Err(format!("Line {}: unknown jump {}", ln + 1, head)),
        };
        out.push(ParsedLine::Jump { kind, label: lbl });
      } else {
        let kind = match head.as_str() {
          "ifz" => IfKind::Ifz,
          "ifg" => IfKind::Ifg,
          "ifl" => IfKind::Ifl,
          _ => return Err(format!("Line {}: unknown if {}", ln + 1, head)),
        };
        out.push(ParsedLine::If { kind, label: lbl });
      }
      continue;
    }

    if head == "sav" {
      if toks.len() != 3 {
        return Err(format!("Line {}: sav expects 2 operands", ln + 1));
      }
      let dst = parse_reg(&toks[2])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;

      let src = if is_register(&toks[1]) {
        SavSrc::Reg(parse_reg(&toks[1]).map_err(|e| format!("Line {}: {e}", ln + 1))?)
      } else {
        let n = parse_i32(&toks[1]).map_err(|e| format!("Line {}: {e}", ln + 1))?;
        SavSrc::Imm(n)
      };

      out.push(ParsedLine::Sav { src, dst });
      continue;
    }

    if head == "cal" {
      if toks.len() != 4 {
        return Err(format!("Line {}: cal expects 3 operands", ln + 1));
      }
      let op = parse_alu(&toks[1])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;

      let a = if is_register(&toks[2]) {
        CalA::Reg(parse_reg(&toks[2]).map_err(|e| format!("Line {}: {e}", ln + 1))?)
      } else {
        let n = parse_i32(&toks[2]).map_err(|e| format!("Line {}: {e}", ln + 1))?;
        CalA::Imm(n)
      };

      let b = parse_reg(&toks[3])
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;

      out.push(ParsedLine::Cal { op, a, b });
      continue;
    }

    return Err(format!("Line {}: Unrecognised instruction: {:?}", ln + 1, toks));
  }

  Ok(out)
}

fn allocate_label_ids(
  parsed: &[ParsedLine],
  defs: &HashSet<String>,
  refs: &HashSet<String>,
  numeric_ids_used: &HashSet<u16>
) -> Result<(HashMap<String, u16>, HashSet<String>), String> {
  let mut name_to_id: HashMap<String, u16> = HashMap::new();
  let mut defined_names: HashSet<String> = HashSet::new();

  let mut all_names: Vec<String> = Vec::new();
  for s in defs.iter() {
    if !is_number_token(s) {
      all_names.push(s.clone());
    }
  }
  for s in refs.iter() {
    if !is_number_token(s) {
      all_names.push(s.clone());
    }
  }

  all_names.sort();
  all_names.dedup();

  let mut next_id: u16 = 1;
  for name in all_names {
    while numeric_ids_used.contains(&next_id) {
      next_id = next_id.wrapping_add(1);
      if next_id == 0 { return Err("Ran out of label IDs".into()); }
    }
    name_to_id.insert(name.clone(), next_id);
    next_id = next_id.wrapping_add(1);
    if next_id == 0 { return Err("Ran out of label IDs".into()); }
  }

  for pl in parsed {
    if let ParsedLine::LabelDef { label } = pl {
      if let LabelRef::Named(n) = label {
        defined_names.insert(n.clone());
      }
    }
  }

  Ok((name_to_id, defined_names))
}

fn parse_io_device(tok: &str) -> Result<u8, String> {
  match tok.to_ascii_lowercase().as_str() {
    "text" | "console" => Ok(0x0),
    "time" => Ok(0x1),
    "screen" => Ok(0x2),
    "keyboard" | "key" => Ok(0x3),
    "mouse" => Ok(0x4),
    "speaker" | "audio" => Ok(0x5),
    "mem" | "memory" => Ok(0x6),
    "disk" | "disc" => Ok(0x7),
    "speech" => Ok(0x8),
    _ => Err(format!("Unknown I/O device '{tok}'"))
  }
}

fn parse_io_command(device: u8, tok: &str) -> Result<u8, String> {
  let t = tok.to_ascii_lowercase();

  match device {
    // text
    0x0 => match t.as_str() {
      "char" => Ok(0x0),
      "int" => Ok(0x1),
      "newline" | "nl" => Ok(0x2),
      "hex" => Ok(0x3),
      _ => Err(format!("Unknown text command '{tok}'"))
    },

    // time
    0x1 => match t.as_str() {
      "unix" => Ok(0x0),
      "low" => Ok(0x01),
      "sleep" | "slp" => Ok(0x2),
      _ => Err(format!("Unknown time command '{tok}'"))
    },

    // screen
    0x2 => match t.as_str() {
      "x" | "setx" => Ok(0x0),
      "y" | "sety" => Ok(0x1),
      "pixel" | "draw" => Ok(0x2),
      "clear" => Ok(0x3),
      "dump" => Ok(0x4),
      "present" | "render" => Ok(0x5),
      "red" | "r" => Ok(0x6),
      "green" | "g" => Ok(0x7),
      "blue" | "b" => Ok(0x8),
      "erase" => Ok(0x9),
      "erasecell" => Ok(0xA),
      _ => Err(format!("Unknown screen command '{tok}'"))
    },

    // keyboard
    0x3 => match t.as_str() {
      "last" | "poll" => Ok(0x0),
      _ => Err(format!("Unknown keyboard command '{tok}'"))
    },

    // mouse
    0x4 => match t.as_str() {
      "x" => Ok(0x0),
      "y" => Ok(0x1),
      "button" | "btn" => Ok(0x2),
      _ => Err(format!("Unknown mouse command '{tok}'"))
    },

    // speaker
    0x5 => match t.as_str() {
      "channel" => Ok(0x0),
      "freq" => Ok(0x1),
      "volume" => Ok(0x2),
      "wave" => Ok(0x3),
      "on" => Ok(0x4),
      "off" => Ok(0x5),
      "toggle" => Ok(0x6),
      _ => Err(format!("Unknown speaker command '{tok}'"))
    },


    // mem
    0x6 => match t.as_str() {
      "addr" | "address" => Ok(0x0),
      "read" => Ok(0x1),
      "write"=> Ok(0x2),
      _ => Err(format!("Unknown memory command '{tok}'"))
    }

    // disk
    0x7 => match t.as_str() {
      "addr" | "address" => Ok(0x0),
      "read" => Ok(0x1),
      "write" => Ok(0x2),
      "save" | "update" => Ok(0x3),
      _ => Err(format!("Unknown disk command '{tok}'"))
    }

    // speak
    0x8 => match t.as_str() {
      "pitch" => Ok(0x0),
      "f1" | "freq1" | "fone" | "freqone" => Ok(0x1),
      "f2" | "freq2" | "ftwo" | "freqtwo" => Ok(0x2),
      "f3" | "freq3" | "fthree" | "freqthree" => Ok(0x3),
      "noise" => Ok(0x4),
      "vol" | "volume" => Ok(0x5),
      "ms" | "dur" | "duration" => Ok(0x6),
      "speak" | "say" => Ok(0x7),
      "reset" | "clear" => Ok(0x8),
      _ => Err(format!("Unknown speech command '{tok}'"))
    }

    _ => Err(format!("Unknown I/O device id {}", device))
  }
}

fn encode_line(
  ln: usize,
  pline: &ParsedLine,
  label_id: &HashMap<String, u16>,
  parsed: &[ParsedLine]
) -> Result<Instr, String> {
  match pline {
    ParsedLine::Raw(i) => Ok(*i),

    ParsedLine::Nop => Ok(0b000000000000000000000),
    ParsedLine::Inp => Ok(0b011000000000000000000),
    ParsedLine::Stp => Ok(0b011111111111111111111),

    ParsedLine::Sav { src, dst } => {
      let (core, a8) = match src {
        SavSrc::Reg(r) => (0b00u32, *r as u32),
        SavSrc::Imm(n) => (0b01u32, imm8(*n)? as u32),
      };
      Ok(pack(core, 0b000, a8, *dst as u32))
    }

    ParsedLine::Cal { op, a, b } => {
      let mode = alu_mode(*op);
      let (core, a8) = match a {
        CalA::Reg(r) => (0b10u32, *r as u32),
        CalA::Imm(n) => (0b11u32, imm8(*n)? as u32),
      };
      Ok(pack(core, mode, a8, *b as u32))
    }

    ParsedLine::Io { device, command, reg } => {
      Ok((0b01110u32 << 16) | ((*device as u32) << 12) | ((*command as u32) << 8) | (*reg as u32))
    }

    ParsedLine::LabelDef { label } => {
      let id = resolve_label_id(label, label_id)?;
      Ok((0b01111u32 << 16) | (id as u32))
    }

    ParsedLine::Jump { kind, label } => {
      let id = resolve_label_id(label, label_id)?;
      let op5 = match kind {
        JumpKind::Jmp => 0b00111,
        JumpKind::Jmz => 0b00001,
        JumpKind::Jmg => 0b00010,
        JumpKind::Jml => 0b00100,
      };
      Ok((op5 << 16) | (id as u32))
    }

    ParsedLine::If { kind, label } => {
      let id = resolve_label_id(label, label_id)?;
      let op5 = match kind {
        IfKind::Ifz => 0b00110,
        IfKind::Ifg => 0b00101,
        IfKind::Ifl => 0b00011,
      };
      Ok((op5 << 16) | (id as u32))
    }

    ParsedLine::Rmv { label } => {
      let id = resolve_label_id(label, label_id)?;
      Ok((0b01101u32 << 16) | (id as u32))
    }

    ParsedLine::Fgo { line } => {
      Ok((0b01011u32 << 16) | (*line as u32))
    }

    ParsedLine::Run { block } => {
      Ok((0b01010u32 << 16) | (*block as u32))
    }

    ParsedLine::Str { start, block } => {
      let start_u8 = resolve_start_u8(start, label_id, parsed)
        .map_err(|e| format!("Line {}: {e}", ln + 1))?;
      Ok(pack(0b01, 0b001, start_u8 as u32, *block as u32))
    }
  }
}

fn pack(core: u32, mode: u32, a: u32, b: u32) -> u32 {
  (core << 19) | (mode << 16) | (a << 8) | b
}

fn alu_mode(op: AluOp) -> u32 {
  match op {
    AluOp::Not => 0b000,
    AluOp::And => 0b001,
    AluOp::Or => 0b010,
    AluOp::Xor  => 0b011,
    AluOp::Shl => 0b100,
    AluOp::Shr => 0b101,
    AluOp::Sar => 0b110,
    AluOp::Add => 0b111,
  }
}

fn resolve_label_id(lr: &LabelRef, label_id: &HashMap<String, u16>) -> Result<u16, String> {
  match lr {
    LabelRef::Numeric(n) => Ok(*n),
    LabelRef::Named(s) => label_id.get(s)
      .copied()
      .ok_or_else(|| format!("Unknown label '{s}'")),
  }
}

fn resolve_start_u8(
  sr: &StartRef,
  _label_id: &HashMap<String, u16>,
  parsed: &[ParsedLine]
) -> Result<u8, String> {
  match sr {
    StartRef::Numeric(n) => Ok(*n),
    StartRef::Named(name) => {
      for (i, pl) in parsed.iter().enumerate() {
        if let ParsedLine::LabelDef { label } = pl {
          if let LabelRef::Named(n) = label {
            if n == name {
              if i > 255 {
                return Err(format!("str start label '{name}' resolves to line {i}, exceeds 8-bit"));
              }
              return Ok(i as u8);
            }
          }
        }
      }
      Err(format!("Label '{name}' not defined (needed for str start)"))
    }
  }
}

fn tokenize(line: &str) -> Vec<String> {
  let s = strip_comment(line).trim();
  if s.is_empty() {
    return Vec::new();
  }
  s.split_whitespace().map(|t| t.to_string()).collect()
}

fn strip_comment(line: &str) -> &str {
  let mut end = line.len();
  for (i, c) in line.char_indices() {
    if c == '#' || c == ';' {
      end = i;
      break;
    }
  }
  &line[..end]
}

fn try_parse_raw_binary(tok: &str) -> Result<Option<Instr>, String> {
  let t = tok.trim();

  if t.starts_with("0x") || t.starts_with("0X") {
    let hex = &t[2..];

    if hex.is_empty() || hex.len() > 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
      return Err(format!("Raw hex must be 1-6 hex digits: {tok}"));
    }

    let v = u32::from_str_radix(hex, 16).map_err(|e| e.to_string())?;

    if v > 0x1F_FFFF {
      return Err(format!("Raw hex exceeds 21-bit instruction range: {tok}"));
    }

    return Ok(Some(v));
  }

  if t.starts_with("0b") {
    let bits = &t[2..];
    if bits.len() != 21 || !bits.chars().all(|c| c == '0' || c == '1') {
      return Err(format!("Raw binary must be exactly 21 bits: {tok}"));
    }
    let v = u32::from_str_radix(bits, 2).map_err(|e| e.to_string())?;
    return Ok(Some(v));
  }

  if t.len() == 21 && t.chars().all(|c| c == '0' || c == '1') {
    let v = u32::from_str_radix(t, 2).map_err(|e| e.to_string())?;
    return Ok(Some(v));
  }

  Ok(None)
}
fn is_register(tok: &str) -> bool {
  let t = tok.to_ascii_lowercase();
  if t.len() != 3 { return false; }
  if !t.starts_with('r') { return false; }
  u8::from_str_radix(&t[1..], 16).is_ok()
}

fn parse_reg(tok: &str) -> Result<u8, String> {
  if !is_register(tok) {
    return Err(format!("Bad register '{tok}' (expected r00..rff)"));
  }
  let t = tok.to_ascii_lowercase();
  u8::from_str_radix(&t[1..], 16).map_err(|_| format!("Bad register '{tok}'"))
}

fn parse_alu(tok: &str) -> Result<AluOp, String> {
  match tok.to_ascii_lowercase().as_str() {
    "not" => Ok(AluOp::Not),
    "and" => Ok(AluOp::And),
    "or" => Ok(AluOp::Or),
    "xor"  => Ok(AluOp::Xor),
    "shl" => Ok(AluOp::Shl),
    "shr" => Ok(AluOp::Shr),
    "sar" => Ok(AluOp::Sar),
    "add" => Ok(AluOp::Add),
    _ => Err(format!("Unknown ALU op '{tok}'")),
  }
}

fn parse_label_ref(
  tok: &str,
  refs: &mut HashSet<String>,
  numeric_ids_used: &mut HashSet<u16>
) -> Result<LabelRef, String> {
  if is_number_token(tok) {
    let id = parse_u16(tok)?;
    numeric_ids_used.insert(id);
    Ok(LabelRef::Numeric(id))
  } else {
    refs.insert(tok.to_string());
    Ok(LabelRef::Named(tok.to_string()))
  }
}

fn is_number_token(tok: &str) -> bool {
  let t = tok.trim();
  if t.is_empty() { return false; }
  let t = if t.starts_with('+') { &t[1..] } else { t };
  t.chars().all(|c| c.is_ascii_digit())
}

fn parse_i32(tok: &str) -> Result<i32, String> {
  tok.trim().parse::<i32>().map_err(|_| format!("Bad integer '{tok}'"))
}

fn parse_u8(tok: &str) -> Result<u8, String> {
  let v = tok.trim().parse::<u16>().map_err(|_| format!("Bad u8 '{tok}'"))?;
  if v > 255 { return Err(format!("Value '{tok}' out of range for u8")); }
  Ok(v as u8)
}

fn parse_u16(tok: &str) -> Result<u16, String> {
  let v = tok.trim().parse::<u32>().map_err(|_| format!("Bad u16 '{tok}'"))?;
  if v > 65535 { return Err(format!("Value '{tok}' out of range for u16")); }
  Ok(v as u16)
}

fn imm8(n: i32) -> Result<u8, String> {
  if n < -128 || n > 127 {
    return Err(format!("imm8 out of range: {n} (range -128..127)"));
  }

  Ok((n as i8) as u8)
}

fn imm8_decode(u8v: u8) -> i32 { (u8v as i8) as i32 }

fn format_operands(core: u32, a: u32, b: u32) -> String {
  match core {
    0b00 => format!("A=r{:02X} B=r{:02X}", a, b),
    0b01 => format!("A=#{} B=r{:02X}", imm8_decode(a as u8), b),
    0b10 => format!("A=r{:02X} B=r{:02X}", a, b),
    0b11 => format!("A=#{} B=r{:02X}", imm8_decode(a as u8), b),
    _ => "A=? B=?".to_string()
  }
}

fn is_known_mnemonic(head: &str) -> bool {
  matches!(
    head,
    "nop" | "inp" | "stp" |
    "sav" | "cal" |
    "io" |
    "jmp" | "jmz" | "jmg" | "jml" |
    "ifz" | "ifg" | "ifl" |
    "rmv" | "fgo" | "str" | "run"
  )
}
