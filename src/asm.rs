// src/asm.rs

use std::collections::{HashMap, HashSet};

pub type Instr = u32;

pub fn assemble(lines: &[String]) -> Result<Vec<Instr>, String> {
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
enum JumpKind {
    Jmp,
    Jmz,
    Jmg,
    Jml,
}

#[derive(Debug, Copy, Clone)]
enum IfKind {
    Ifz,
    Ifg,
    Ifl,
}

fn parse_lines(
    lines: &[String],
    defs: &mut HashSet<String>,
    refs: &mut HashSet<String>,
    numeric_ids_used: &mut HashSet<u16>,
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

        if head == "nop" {
            out.push(ParsedLine::Nop);
            continue;
        }
        if head == "inp" {
            out.push(ParsedLine::Inp);
            continue;
        }
        if head == "stp" {
            out.push(ParsedLine::Stp);
            continue;
        }

        if toks.len() == 1 {
            if toks[0].ends_with(':') {
                let name = toks[0].trim_end_matches(':').to_string();
                if name.is_empty() {
                    return Err(format!("Line {}: empty label name", ln + 1));
                }
                defs.insert(name.clone());
                out.push(ParsedLine::LabelDef {
                    label: LabelRef::Named(name),
                });
                continue;
            }

            if !is_register(&toks[0]) && !is_number_token(&toks[0]) && !is_known_mnemonic(&head) {
                let name = toks[0].to_string();
                defs.insert(name.clone());
                out.push(ParsedLine::LabelDef {
                    label: LabelRef::Named(name),
                });
                continue;
            }

            if is_number_token(&toks[0]) {
                let id = parse_u16(&toks[0]).map_err(|e| format!("Line {}: {e}", ln + 1))?;
                numeric_ids_used.insert(id);
                out.push(ParsedLine::LabelDef {
                    label: LabelRef::Numeric(id),
                });
                continue;
            }
        }

        if head == "fgo" {
            if toks.len() != 2 {
                return Err(format!("Line {}: fgo expects 1 operand", ln + 1));
            }
            let n = parse_u16(&toks[1]).map_err(|e| format!("Line {}: {e}", ln + 1))?;
            out.push(ParsedLine::Fgo { line: n });
            continue;
        }

        if head == "io" {
            if toks.len() != 4 {
                return Err(format!(
                    "Line {}: io expects 3 operands: io <device> <command> <register>",
                    ln + 1
                ));
            }

            let device = parse_io_device(&toks[1]).map_err(|e| format!("Line {}: {e}", ln + 1))?;

            let command =
                parse_io_command(device, &toks[2]).map_err(|e| format!("Line {}: {e}", ln + 1))?;

            let reg = parse_reg(&toks[3]).map_err(|e| format!("Line {}: {e}", ln + 1))?;

            out.push(ParsedLine::Io {
                device,
                command,
                reg,
            });
            continue;
        }

        if head == "run" {
            if toks.len() != 2 {
                return Err(format!("Line {}: run expects 1 operand", ln + 1));
            }
            let b = parse_u16(&toks[1]).map_err(|e| format!("Line {}: {e}", ln + 1))?;
            out.push(ParsedLine::Run { block: b });
            continue;
        }

        if head == "str" {
            if toks.len() != 3 {
                return Err(format!("Line {}: str expects 2 operands", ln + 1));
            }
            let start = if is_number_token(&toks[1]) {
                let v = parse_u8(&toks[1]).map_err(|e| format!("Line {}: {e}", ln + 1))?;
                StartRef::Numeric(v)
            } else {
                refs.insert(toks[1].clone());
                StartRef::Named(toks[1].clone())
            };

            let block = parse_u8(&toks[2]).map_err(|e| format!("Line {}: {e}", ln + 1))?;

            out.push(ParsedLine::Str { start, block });
            continue;
        }

        if matches!(
            head.as_str(),
            "jmp" | "jmz" | "jmg" | "jml" | "ifz" | "ifg" | "ifl" | "rmv"
        ) {
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
            let dst = parse_reg(&toks[2]).map_err(|e| format!("Line {}: {e}", ln + 1))?;

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
            let op = parse_alu(&toks[1]).map_err(|e| format!("Line {}: {e}", ln + 1))?;

            let a = if is_register(&toks[2]) {
                CalA::Reg(parse_reg(&toks[2]).map_err(|e| format!("Line {}: {e}", ln + 1))?)
            } else {
                let n = parse_i32(&toks[2]).map_err(|e| format!("Line {}: {e}", ln + 1))?;
                CalA::Imm(n)
            };

            let b = parse_reg(&toks[3]).map_err(|e| format!("Line {}: {e}", ln + 1))?;

            out.push(ParsedLine::Cal { op, a, b });
            continue;
        }

        return Err(format!(
            "Line {}: Unrecognised instruction: {:?}",
            ln + 1,
            toks
        ));
    }

    Ok(out)
}

fn allocate_label_ids(
    parsed: &[ParsedLine],
    defs: &HashSet<String>,
    refs: &HashSet<String>,
    numeric_ids_used: &HashSet<u16>,
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
            if next_id == 0 {
                return Err("Ran out of label IDs".into());
            }
        }
        name_to_id.insert(name.clone(), next_id);
        next_id = next_id.wrapping_add(1);
        if next_id == 0 {
            return Err("Ran out of label IDs".into());
        }
    }

    for pl in parsed {
        if let ParsedLine::LabelDef { label } = pl
            && let LabelRef::Named(n) = label
        {
            defined_names.insert(n.clone());
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
        _ => Err(format!("Unknown I/O device '{tok}'")),
    }
}

fn parse_io_command(device: u8, tok: &str) -> Result<u8, String> {
    let t = tok.to_ascii_lowercase();

    match device {
        // text
        0x0 => match t.as_str() {
            "char" => Ok(0x0),
            "int" | "value" => Ok(0x1),
            "newline" | "nl" => Ok(0x2),
            "hex" => Ok(0x3),
            "error" | "err" => Ok(0x4),
            "clear" => Ok(0x5),
            _ => Err(format!("Unknown text command '{tok}'")),
        },

        // time
        0x1 => match t.as_str() {
            "unix" => Ok(0x0),
            "low" => Ok(0x01),
            "sleep" | "slp" => Ok(0x2),
            "milli" | "millis" => Ok(0x3),
            _ => Err(format!("Unknown time command '{tok}'")),
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
            _ => Err(format!("Unknown screen command '{tok}'")),
        },

        // keyboard
        0x3 => match t.as_str() {
            "last" | "poll" => Ok(0x0),
            _ => Err(format!("Unknown keyboard command '{tok}'")),
        },

        // mouse
        0x4 => match t.as_str() {
            "x" => Ok(0x0),
            "y" => Ok(0x1),
            "button" | "btn" => Ok(0x2),
            _ => Err(format!("Unknown mouse command '{tok}'")),
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
            _ => Err(format!("Unknown speaker command '{tok}'")),
        },

        // mem
        0x6 => match t.as_str() {
            "addr" | "address" => Ok(0x0),
            "read" => Ok(0x1),
            "write" => Ok(0x2),
            _ => Err(format!("Unknown memory command '{tok}'")),
        },

        // disk
        0x7 => match t.as_str() {
            "addr" | "address" => Ok(0x0),
            "read" => Ok(0x1),
            "write" => Ok(0x2),
            "save" | "update" => Ok(0x3),
            _ => Err(format!("Unknown disk command '{tok}'")),
        },

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
            _ => Err(format!("Unknown speech command '{tok}'")),
        },

        _ => Err(format!("Unknown I/O device id {}", device)),
    }
}

fn encode_line(
    ln: usize,
    pline: &ParsedLine,
    label_id: &HashMap<String, u16>,
    parsed: &[ParsedLine],
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

        ParsedLine::Io {
            device,
            command,
            reg,
        } => Ok((0b01110u32 << 16)
            | ((*device as u32) << 12)
            | ((*command as u32) << 8)
            | (*reg as u32)),

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

        ParsedLine::Fgo { line } => Ok((0b01011u32 << 16) | (*line as u32)),

        ParsedLine::Run { block } => Ok((0b01010u32 << 16) | (*block as u32)),

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
        AluOp::Xor => 0b011,
        AluOp::Shl => 0b100,
        AluOp::Shr => 0b101,
        AluOp::Sar => 0b110,
        AluOp::Add => 0b111,
    }
}

fn resolve_label_id(lr: &LabelRef, label_id: &HashMap<String, u16>) -> Result<u16, String> {
    match lr {
        LabelRef::Numeric(n) => Ok(*n),
        LabelRef::Named(s) => label_id
            .get(s)
            .copied()
            .ok_or_else(|| format!("Unknown label '{s}'")),
    }
}

fn resolve_start_u8(
    sr: &StartRef,
    _label_id: &HashMap<String, u16>,
    parsed: &[ParsedLine],
) -> Result<u8, String> {
    match sr {
        StartRef::Numeric(n) => Ok(*n),
        StartRef::Named(name) => {
            for (i, pl) in parsed.iter().enumerate() {
                if let ParsedLine::LabelDef { label } = pl
                    && let LabelRef::Named(n) = label
                    && n == name
                {
                    if i > 255 {
                        return Err(format!(
                            "str start label '{name}' resolves to line {i}, exceeds 8-bit"
                        ));
                    }
                    return Ok(i as u8);
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

    if let Some(bits) = t.strip_prefix("0b") {
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
    if t.len() != 3 {
        return false;
    }
    if !t.starts_with('r') {
        return false;
    }
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
        "xor" => Ok(AluOp::Xor),
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
    numeric_ids_used: &mut HashSet<u16>,
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
    if t.is_empty() {
        return false;
    }
    let t = t.strip_prefix('+').unwrap_or(t);
    t.chars().all(|c| c.is_ascii_digit())
}

fn parse_i32(tok: &str) -> Result<i32, String> {
    tok.trim()
        .parse::<i32>()
        .map_err(|_| format!("Bad integer '{tok}'"))
}

fn parse_u8(tok: &str) -> Result<u8, String> {
    let v = tok
        .trim()
        .parse::<u16>()
        .map_err(|_| format!("Bad u8 '{tok}'"))?;
    if v > 255 {
        return Err(format!("Value '{tok}' out of range for u8"));
    }
    Ok(v as u8)
}

fn parse_u16(tok: &str) -> Result<u16, String> {
    let v = tok
        .trim()
        .parse::<u32>()
        .map_err(|_| format!("Bad u16 '{tok}'"))?;
    if v > 65535 {
        return Err(format!("Value '{tok}' out of range for u16"));
    }
    Ok(v as u16)
}

fn imm8(n: i32) -> Result<u8, String> {
    if !(-128..=127).contains(&n) {
        return Err(format!("imm8 out of range: {n} (range -128..127)"));
    }

    Ok((n as i8) as u8)
}

fn _imm8_decode(u8v: u8) -> i32 {
    (u8v as i8) as i32
}

fn is_known_mnemonic(head: &str) -> bool {
    matches!(
        head,
        "nop"
            | "inp"
            | "stp"
            | "sav"
            | "cal"
            | "io"
            | "jmp"
            | "jmz"
            | "jmg"
            | "jml"
            | "ifz"
            | "ifg"
            | "ifl"
            | "rmv"
            | "fgo"
            | "str"
            | "run"
    )
}
