// src/main.rs

////////////////////////////////////////////////////////////////////////////////////////////////
// ██████╗██████╗  █████╗ ██╗    ██╗███████╗███████╗███████╗███╗   ███╗██████╗ ██╗  ██╗   ██╗ //
//██╔════╝██╔══██╗██╔══██╗██║    ██║██╔════╝██╔════╝██╔════╝████╗ ████║██╔══██╗██║  ╚██╗ ██╔╝ //
//██║     ██████╔╝███████║██║ █╗ ██║███████╗███████╗█████╗  ██╔████╔██║██████╔╝██║   ╚████╔╝  //
//██║     ██╔══██╗██╔══██║██║███╗██║╚════██║╚════██║██╔══╝  ██║╚██╔╝██║██╔══██╗██║    ╚██╔╝   //
//╚██████╗██║  ██║██║  ██║╚███╔███╔╝███████║███████║███████╗██║ ╚═╝ ██║██████╔╝███████╗██║    //
// ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚══════╝╚══════╝╚══════╝╚═╝     ╚═╝╚═════╝ ╚══════╝╚═╝    //
////////////////////////////////////////////////////////////////////////////////////////////////

use std::fs;
use std::env;
use std::path::{Path, PathBuf};

mod asm;
mod c_backend;
mod vm;

const CRAW_NANO: &str = r#"
syntax "crawssembly" "\.craw$"

color brightcyan "\<(sav|cal|io|inp)\>"

color cyan "\<(nop)\>"

color white "\<(not|and|or|xor|shl|shr|sar|add)\>"

color magenta "\<(jmp|jmz|jmg|jml|ifz|ifg|ifl|rmv|fgo|str|run|stp)\>"

color brightmagenta "\<(execute|executestd)\>"

color brightblue "\<(screen|keyboard|speaker|mem|disk|text|time)\>"

color blue "\<(char|hex|newline|pixel|read|write|erase|erasecell)\>"
color blue "\<(x|y|int|clear|unix|low|sleep|milli|dump|present)\>"
color blue "\<(red|green|blue|poll|btn|channel|freq|volume|wave|on|off)\>"
color blue "\<(toggle|addr|save|iso)\>"

color yellow "[-+]?[0-9]+"
color yellow "0x[0-9A-Fa-f]+"

color brightgreen "\<(r[0-9A-Fa-f]{2})\>"
color cyan "\<(r00|r01|ref|rff)\>"

color brightblack ";.*$"
"#;

type Instr = asm::Instr;

fn install_nano() {
    let home = match env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        Ok(home) => PathBuf::from(home),
        Err(_) => {
            eprintln!("Could not find your home directory.");
            return;
        }
    };

    let nano_dir = home.join(".nano");
    let syntax_path = nano_dir.join("craw.nanorc");
    let nanorc_path = home.join(".nanorc");

    if let Err(err) = fs::create_dir_all(&nano_dir) {
        eprintln!("Could not create ~/.nano: {err}");
        return;
    }

    if let Err(err) = fs::write(&syntax_path, CRAW_NANO) {
        eprintln!("Could not write Crawssembly Nano syntax file: {err}");
        return;
    }

    let include_line = "include ~/.nano/craw.nanorc";

    let existing = fs::read_to_string(&nanorc_path).unwrap_or_default();

    if !existing.lines().any(|line| line.trim() == include_line) {
        let mut new_contents = existing;

        if !new_contents.is_empty() && !new_contents.ends_with('\n') {
            new_contents.push('\n');
        }

        new_contents.push_str(include_line);
        new_contents.push('\n');

        if let Err(err) = fs::write(&nanorc_path, new_contents) {
            eprintln!("Could not update ~/.nanorc: {err}");
            return;
        }
    }

    println!("Installed Crawssembly syntax highlighting for GNU Nano.");
}

fn copy_dir(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;

    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;

        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            let _ = copy_dir(&src_path, &dst_path);
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn install_std() {
    if let Err(e) = try_install_std() { eprintln!("Warning: Could not install standard library: {e}"); }
}

fn try_install_std() -> Result<(), String> {
    let home = std::env::var("HOME")
        .map_err(|_| "HOME not set")?;

    let dst = Path::new(&home)
        .join(".crawssembly")
        .join("std");

    if dst.exists() { return Ok(()); }

    let _ = copy_dir(Path::new("std"), &dst);

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    if args.len() >= 2 && (args[1] == "--install-nano" || args[1] == "install-nano") {
        install_nano();
        return;
    }

    if args.len() >= 2 && (args[1] == "--install-std" || args[1] == "install-std") {
        install_std();
        return;
    }

    if args.iter().any(|a| a == "--decode") {
        decode_interactive();
        return;
    }

    let disk_config = match parse_disk_config(&args) {
        Ok(disk) => disk,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let dump = args.iter().any(|a| a == "--dump");
    let dump_decoded = args.iter().any(|a| a == "--dump-decoded");
    let debug = args.iter().any(|a| a == "--debug");
    let plain = !args.iter().any(|a| a == "--tui");
    let audio = args.iter().any(|a| a == "--audio");
    let show_stats = debug || args.iter().any(|a| a == "--stats");
    let looping = args.iter().any(|a| a == "--loop");

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

    if disk_config.raw && !disk_config.readonly && !args.iter().any(|a| a == "--force") {
        eprintln!("WARNING: Crawssembly is about to write to a real disk/device:");
        eprintln!("  {}", disk_config.path);
        eprintln!("Use --readonly for safe inspection, or --force to allow writes.");
        std::process::exit(1);
    }

    let vm_config = vm::VmConfig { screen_w, screen_h, disk: disk_config, };

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

            if let Some(path) = input_path
                && let Err(e) = assemble_file(&path, "program.bin", dump, dump_decoded)
            {
                eprintln!("{e}");
                std::process::exit(1);
            }

            if let Err(e) = vm::run_vm_with_options(
                "program.bin",
                plain,
                audio,
                -1,
                input_list.clone(),
                looping,
                show_stats,
                vm_config,
            ) {
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

            if let Err(e) = vm::run_vm_with_options(
                "program.bin",
                plain,
                audio,
                -1,
                input_list.clone(),
                looping,
                true,
                vm_config,
            ) {
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

        Some("emit-c") => {
            let Some(input_path) = positional_after(&args, "emit-c") else {
                eprintln!("Missing input file.\n");
                print_help();
                std::process::exit(1);
            };

            let output_path = parse_output_file(&args).unwrap_or_else(|| "out.c".to_string());

            if let Err(e) = emit_c_file(&input_path, &output_path) {
                eprintln!("{e}");
                std::process::exit(1);
            }

            println!("Wrote C output to {output_path}");
        }

        Some(path) => {
            if let Err(e) = assemble_file(path, "program.bin", dump, dump_decoded) {
                eprintln!("{e}");
                std::process::exit(1);
            }

            if let Err(e) = vm::run_vm_with_options(
                "program.bin",
                plain,
                audio,
                -1,
                input_list.clone(),
                looping,
                show_stats,
                vm_config,
            ) {
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
    matches!(arg, "--screen" | "--file" | "-o" | "--output" | "--disk" | "--raw-disk")
}

fn parse_disk_config(args: &[String]) -> Result<vm::DiskConfig, String> {
  let mut disk = vm::DiskConfig::default();

  for i in 0..args.len() {
    match args[i].as_str() {
      "--disk" => {
        let Some(path) = args.get(i + 1) else {
          return Err("--disk expects a filename".to_string());
        };

        disk.path = path.clone();
        disk.raw = false;
      }

      "--raw-disk" => {
        let Some(path) = args.get(i + 1) else {
          return Err("--raw-disk expects a device path".to_string());
        };

        disk.path = path.clone();
        disk.raw = true;
      }

      "--readonly" => {
        disk.readonly = true;
      }

      _ => {}
    }
  }

  Ok(disk)
}

fn parse_output_file(args: &[String]) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == "-o" || args[i] == "--output" {
            return args.get(i + 1).cloned();
        }
    }

    None
}

fn emit_c_file(input_path: &str, output_path: &str) -> Result<(), String> {
    let src =
        fs::read_to_string(input_path).map_err(|e| format!("Failed to read {input_path}: {e}"))?;

    let lines: Vec<String> = src.lines().map(|s| s.to_string()).collect();
    let c_code = c_backend::emit_c(lines);

    fs::write(output_path, c_code).map_err(|e| format!("Failed to write {output_path}: {e}"))?;

    Ok(())
}

fn assemble_file(
    input_path: &str,
    output_path: &str,
    dump: bool,
    dump_decoded: bool,
) -> Result<(), String> {
    let lines = asm::expand_execute_file(std::path::Path::new(input_path))?;

    match asm::assemble(&lines) {
        Ok(program) => {
            if dump {
                dump_program(&lines, &program, dump_decoded);
            }

            write_program_bin(output_path, &program)
                .map_err(|e| format!("Failed to write {output_path}: {e}"))?;

            Ok(())
        }

        Err(e) => Err(format!("Assembly failed:\n{e}")),
    }
}

fn check_file(input_path: &str, dump: bool, dump_decoded: bool) -> Result<(), String> {
    let src =
        fs::read_to_string(input_path).map_err(|e| format!("Failed to read {input_path}: {e}"))?;

    let lines: Vec<String> = src.lines().map(|s| s.to_string()).collect();
    let program = asm::assemble(&lines)?;

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
    println!("  craw emit-c <file.craw>       Convert Crawssembly to C");
    println!("  craw install-nano             Install nano syntax highlighting");
    println!("  craw run <file.craw>          Assemble and run a file");
    println!();
    println!("Options:");
    println!("  --audio                       Enable speaker/speech audio output");
    println!("  --decode                      Open the instruction decoder");
    println!("  --dump                        Show assembled instructions");
    println!("  --dump-decoded                Show decoded instruction fields with --dump");
    println!("  --file <file>                 Send file contents to r00 during execution.");
    println!("  --help                        Show this help message");
    println!("  --o, --output <file>          Set output filename");
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

            let bytes =
                fs::read(path).map_err(|e| format!("Failed to read input file '{path}': {e}"))?;

            let input = bytes.into_iter().map(|b| b as i32).collect();

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

            width = w
                .parse::<usize>()
                .map_err(|_| format!("Bad screen width: {w}"))?;

            height = h
                .parse::<usize>()
                .map_err(|_| format!("Bad screen height: {h}"))?;
        }
    }

    if width == 0 || height == 0 {
        return Err("Screen width and height must be greater than 0".to_string());
    }

    if width > u16::MAX as usize {
        return Err(format!(
            "Screen width must be <= {} for terminal rendering",
            u16::MAX
        ));
    }

    if height > u16::MAX as usize {
        return Err(format!(
            "Screen height must be <= {} for terminal rendering",
            u16::MAX
        ));
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
            println!(
                "{:0width$} | {} | {} |{}{}",
                i,
                bin,
                hex,
                d,
                src,
                width = width
            );
        } else {
            println!("{:0width$} | {} | {} | {}", i, bin, hex, src, width = width);
        }
    }
}

fn imm8_decode(u8v: u8) -> i32 {
    (u8v as i8) as i32
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

fn decode_interactive() {
    use std::io::{self, Write};

    loop {
        print!("Enter 21-bit binary (or 'exit'): ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();

        if s == "exit" {
            break;
        }

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

        return format!("IO device={} command={} reg=r{:02X}", device, command, reg);
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

fn format_operands(core: u32, a: u32, b: u32) -> String {
    match core {
        0b00 => format!("A=r{:02X} B=r{:02X}", a, b),
        0b01 => format!("A=#{} B=r{:02X}", imm8_decode(a as u8), b),
        0b10 => format!("A=r{:02X} B=r{:02X}", a, b),
        0b11 => format!("A=#{} B=r{:02X}", imm8_decode(a as u8), b),
        _ => "A=? B=?".to_string(),
    }
}
