// src/vm.rs

use std::fs;
use std::io::{self};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::thread;
use std::collections::VecDeque;
use std::sync::{
  Arc,
  Mutex,
  atomic::AtomicBool,
};

use crossterm::{
  cursor,
  event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
    MouseButton, MouseEventKind,
  },
  execute, queue,
  style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
  terminal::{self, ClearType},
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use sysinfo::System;

type Instr = u32;

const INSTR_MASK: u32 = 0x1F_FFFF; // 21 bits
const BLOCK_INSTRS: usize = 168;
const BLOCK_BYTES: usize = 441; // 168 * 21 = 3528 bits = 441 bytes
const STORAGE_BLOCKS: usize = 512;
const STORAGE_PATH: &str = "storage.bin";
const DISK_PATH: &str = "disk.img";
const DISK_CELLS: usize = 65536;

// IO status values
const REG_IO_STATUS: usize = 0xEE;
const IO_OK: i32 = 0;
const IO_INVALID_DEVICE: i32 = 1;
const IO_INVALID_COMMAND: i32 = 2;
const IO_BAD_VALUE: i32 = 3;
const IO_UNAVAILABLE: i32 = 4;

// screen
const IO_SCREEN_OUT_OF_BOUNDS: i32 = 0x10;

#[derive(Clone, Copy)]
pub struct VmConfig {
  pub screen_w: usize,
  pub screen_h: usize,
}

impl Default for VmConfig {
  fn default() -> Self {
    Self {
      screen_w: 64,
      screen_h: 64,
    }
  }
}

const SPEAKER_COUNT: usize = 8;

pub fn run_vm() -> Result<(), String> {
  let program = load_program("program.bin")?;
  let decoded = predecode(&program);

  let args: Vec<String> = std::env::args().collect();
  if let Some(iters) = parse_bench_iters(&args) {
    run_bench(&decoded, iters);
    return Ok(());
  }

  let (speed, input_list, looped) = startup()?;

  let mut cpu = Cpu::new(true, VmConfig::default());
  cpu.execute_interactive(&decoded, speed, &input_list, looped, false, true);

  Ok(())
}

pub fn run_vm_with_options(
  program_path: &str,
  plain: bool,
  audio: bool,
  speed: i32,
  input_list: Vec<i32>,
  looped: bool,
  show_stats: bool,
  config: VmConfig,
) -> Result<(), String> {
  let program = load_program(program_path)?;
  let decoded = predecode(&program);

  let args: Vec<String> = std::env::args().collect();
  if let Some(iters) = parse_bench_iters(&args) {
    run_bench(&decoded, iters);
    return Ok(());
  }

  let mut cpu = Cpu::new(audio, config);
  cpu.execute_interactive(&decoded, speed, &input_list, looped, plain, show_stats);

  Ok(())
}

fn parse_bench_iters(args: &[String]) -> Option<u64> {
  // Usage: --bench [iters]
  // If iters omitted, default to 200000
  for i in 0..args.len() {
    if args[i] == "--bench" {
      if i + 1 < args.len() {
        if let Ok(n) = args[i + 1].parse::<u64>() {
          return Some(n.max(1));
        }
      }
      return Some(200000);
    }
  }
  None
}

fn run_bench(program: &[Decoded], iters: u64) {
  let mut cpu = Cpu::new(false, VmConfig::default());

  let mut sys = System::new_all();
  sys.refresh_cpu_all();

  let cpu_mhz =
    sys.cpus()
    .first()
    .map(|cpu| cpu.frequency())
    .unwrap_or(0) as f64;

  let cpu_hz = cpu_mhz * 1_000_000.0;

  let mut total_ticks: u64 = 0;
  let start = Instant::now();

  for _ in 0..iters {
    let t = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap();
    let seed = (t.as_secs() as u32) ^ (t.subsec_nanos());
    let time_input_list: [i32; 1] = [seed as i32];

    total_ticks += cpu.execute_noio(program, &time_input_list);

  }

  let dt = start.elapsed().as_secs_f64();
  let hz = (total_ticks as f64) / dt;

  let cycles_per_instruction = cpu_hz / hz;

  println!("BENCH runs={} total_ticks={} time={:.6}s", iters, total_ticks, dt);
  println!("BENCH throughput:  {:.3} Hz = ~{:.3} MHz", hz, hz / 1_000_000.0);
  println!("~{:.3} ns / instruction", (dt * 1e9) / total_ticks as f64);
  println!("BENCH final rFF: {}", cpu.regs[0xFF]);
  println!("Estimated vm cost: {:.2} host CPU cycles / instruction", cycles_per_instruction);

}

fn load_program(path: &str) -> Result<Vec<Instr>, String> {
  let bytes = fs::read(path).map_err(|e| e.to_string())?;
  if bytes.len() % 4 != 0 {
    return Err("program.bin corrupted (not u32-aligned)".into());
  }

  let mut out = Vec::with_capacity(bytes.len() / 4);
  for c in bytes.chunks_exact(4) {
    out.push(u32::from_le_bytes([c[0], c[1], c[2], c[3]]) & INSTR_MASK);
  }
  Ok(out)
}

fn startup() -> Result<(i32, Vec<i32>, bool), String> {
  println!("Clock speed (0 = step, -1 = max):");
  let speed = read_i32()?.unwrap_or(-1);

  println!("Input values (space-separated):");
  let mut inputs = Vec::new();
  let line = read_line()?;

  if line.trim() == "file" {
    println!("Enter file name:");
    let name = read_line()?;
    let bytes = fs::read(name).map_err(|e| e.to_string())?;

    for byte in bytes {
      inputs.push(byte as i32);
    }
  } else {
    for t in line.split_whitespace() {
      if let Ok(v) = t.parse::<i32>() {
        inputs.push(v);
      }
    }
  }

  let mut looped = false;

  if inputs.is_empty() {
    inputs.push(0);
  } else {
    println!("Is this input looping:");
    let looping = read_line()?;
    if looping.trim() == "yes" { looped = true; }
  }

  Ok((speed, inputs, looped))
}

fn read_line() -> Result<String, String> {
  let mut s = String::new();
  io::stdin().read_line(&mut s).map_err(|e| e.to_string())?;
  Ok(s.trim().to_string())
}

fn read_i32() -> Result<Option<i32>, String> {
  Ok(read_line()?.parse::<i32>().ok())
}

// ---------- Storage (str/run) ----------

fn ensure_storage() -> Result<(), String> {
  if fs::metadata(STORAGE_PATH).is_ok() {
    return Ok(());
  }
  let f = OpenOptions::new()
    .create(true)
    .write(true)
    .open(STORAGE_PATH)
    .map_err(|e| e.to_string())?;
  let size = (BLOCK_BYTES * STORAGE_BLOCKS) as u64;
  f.set_len(size).map_err(|e| e.to_string())?;
  Ok(())
}

fn pack_21bit_block(instrs: &[Instr]) -> Vec<u8> {
  // instrs length <= 168. Pads with nop (0).
  let mut padded: Vec<u32> = Vec::with_capacity(BLOCK_INSTRS);
  for &i in instrs.iter().take(BLOCK_INSTRS) {
    padded.push(i & INSTR_MASK);
  }
  while padded.len() < BLOCK_INSTRS {
    padded.push(0);
  }

  let mut out = vec![0u8; BLOCK_BYTES];
  let mut bit_pos: usize = 0;

  for &v in &padded {
    for k in (0..21).rev() {
      let bit = ((v >> k) & 1) as u8;
      let byte_i = bit_pos / 8;
      let bit_i = 7 - (bit_pos % 8);
      if bit == 1 {
        out[byte_i] |= 1 << bit_i;
      }
      bit_pos += 1;
    }
  }

  out
}

fn unpack_21bit_block(buf: &[u8]) -> Vec<Instr> {
  let mut out: Vec<Instr> = Vec::with_capacity(BLOCK_INSTRS);
  let mut bit_pos: usize = 0;

  for _ in 0..BLOCK_INSTRS {
    let mut v: u32 = 0;
    for _ in 0..21 {
      let byte_i = bit_pos / 8;
      let bit_i = 7 - (bit_pos % 8);
      let bit = (buf[byte_i] >> bit_i) & 1;
      v = (v << 1) | (bit as u32);
      bit_pos += 1;
    }
    out.push(v & INSTR_MASK);
  }

  out
}

fn write_block(block: u16, instrs: &[Instr]) -> Result<(), String> {
  ensure_storage()?;
  if block as usize >= STORAGE_BLOCKS {
    return Err(format!("str: block out of range: {}", block));
  }

  let buf = pack_21bit_block(instrs);

  let mut f = OpenOptions::new()
    .write(true)
    .open(STORAGE_PATH)
    .map_err(|e| e.to_string())?;

  let off = (block as u64) * (BLOCK_BYTES as u64);
  f.seek(SeekFrom::Start(off)).map_err(|e| e.to_string())?;
  f.write_all(&buf).map_err(|e| e.to_string())?;
  Ok(())
}

fn read_block(block: u16) -> Result<Vec<Instr>, String> {
  ensure_storage()?;
  if block as usize >= STORAGE_BLOCKS {
    return Err(format!("run: block out of range: {}", block));
  }

  let mut f = OpenOptions::new()
    .read(true)
    .open(STORAGE_PATH)
    .map_err(|e| e.to_string())?;

  let off = (block as u64) * (BLOCK_BYTES as u64);
  f.seek(SeekFrom::Start(off)).map_err(|e| e.to_string())?;

  let mut buf = vec![0u8; BLOCK_BYTES];
  f.read_exact(&mut buf).map_err(|e| e.to_string())?;

  Ok(unpack_21bit_block(&buf))
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

    out.push(Decoded { core, mode, a, b, op5, imm16, raw21: v, is_nop, is_stp, is_inp });
  }

  out
}

// ---------- CPU ----------

#[derive(Copy, Clone)]
enum ProgRef {
  Main,
  Block(usize),
}

#[derive(Copy, Clone)]
struct Frame {
  prog: ProgRef,
  pc: i32,
}

#[derive(Copy, Clone)]
struct MouseState {
  x: i32,
  y: i32,
  buttons: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct Speaker {
  pub freq: f32,
  pub vol: f32,
  pub wave: i32,
  pub enabled: bool,
}

impl Default for Speaker {
  fn default() -> Self {
    Self {
      freq: 440.0,
      vol: 0.0,
      wave: 0,
      enabled: false,
    }
  }
}

pub struct AudioHandle { _stream: cpal::Stream }

pub fn start_audio(speakers: Arc<Mutex<[Speaker; SPEAKER_COUNT]>>, speech_samples: Arc<Mutex<VecDeque<f32>>>) -> Result<AudioHandle, String> {
  let host = cpal::default_host();

  let device = host
    .default_output_device()
    .ok_or("no output audio device found")?;

  let config = device
    .default_output_config()
    .map_err(|e| format!("failed to get audio config: {e}"))?;

  let sample_rate = config.sample_rate().0 as f32;
  let channels = config.channels() as usize;

  let mut phases = [0.0f32; SPEAKER_COUNT];

  let err_fn = |err| {  eprintln!("audio stream error: {err}"); };

  let stream = match config.sample_format() {
    cpal::SampleFormat::F32 => {
      device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _| {
          write_audio(data, channels, sample_rate, &speakers, &speech_samples, &mut phases);
        },
        err_fn,
        None
      )
    }

    _ => {
      return Err("only f32 audio output is currently supported".to_string());
    }
  }.map_err(|e| format!("failed to build audio stream: {e}"))?;

  stream
    .play()
    .map_err(|e| format!("failed to play audio stream: {e}"))?;

  Ok(AudioHandle {
    _stream: stream,
  })
}

fn write_audio(
  output: &mut [f32],
  channels: usize,
  sample_rate: f32,
  speakers: &Arc<Mutex<[Speaker; SPEAKER_COUNT]>>,
  speech_samples: &Arc<Mutex<VecDeque<f32>>>,
  phases: &mut [f32; SPEAKER_COUNT],
) {
  let speaker_snapshot = match speakers.lock() {
    Ok(speakers) => *speakers,
    Err(_) => {
      for sample in output.iter_mut() {
        *sample = 0.0;
      }

      return;
    }
  };

  for frame in output.chunks_mut(channels) {
    let mut sample = 0.0f32;

    for i in 0..SPEAKER_COUNT {
      let speaker = speaker_snapshot[i];

      if !speaker.enabled || speaker.vol <= 0.0 {
        continue;
      }

      let wave_sample = make_wave(speaker.wave, phases[i]);
      sample += wave_sample * speaker.vol;

      phases[i] += speaker.freq / sample_rate;

      while phases[i] >= 1.0 {
        phases[i] -= 1.0;
      }
    }

    if let Ok(mut speech) = speech_samples.lock() {
      if let Some(speech_sample) = speech.pop_front() {
        sample += speech_sample;
      }
    }

    sample = sample.clamp(-1.0, 1.0);

    for channel_sample in frame.iter_mut() {
      *channel_sample = sample;
    }
  }
}

fn make_wave(wave: i32, phase: f32) -> f32 {
  match wave {
    // square
    0 => {
      if phase < 0.5 {
        1.0
      } else {
        -1.0
      }
    }

    // sine
    1 => {
      (phase * std::f32::consts::TAU).sin()
    }

    // triangle
    2 => {
      4.0 * (phase - 0.5).abs() - 1.0
    }

    // sawtooth
    3 => {
      2.0 * phase - 1.0
    }

    // noise-ish placeholder
    4 => {
      if phase < 0.5 {
        0.5
      } else {
        -0.5
      }
    }

    _ => 0.0,
  }
}

fn _mouse_button_bit(button: MouseButton) -> i32 {
  match button {
    MouseButton::Left => 1 << 0,
    MouseButton::Right => 1 << 1,
    MouseButton::Middle => 1 << 2,
  }
}

#[derive(Clone, Copy)]
struct Resonator {
  y1: f32,
  y2: f32,
}

impl Resonator {
  fn new() -> Self {
    Self {
      y1: 0.0,
      y2: 0.0,
    }
  }

  fn process(&mut self, input: f32, freq: f32, bandwidth: f32, sample_rate: f32) -> f32 {
    if freq <= 0.0 {
      return 0.0;
    }

    let r = (-std::f32::consts::PI * bandwidth / sample_rate).exp();
    let theta = std::f32::consts::TAU * freq / sample_rate;

    let a1 = 2.0 * r * theta.cos();
    let a2 = -(r * r);

    let y = input + a1 * self.y1 + a2 * self.y2;

    self.y2 = self.y1;
    self.y1 = y;

    y
  }
}

struct Cpu {
  regs: [i32; 256],

  // bench
  pub sleep_times: f64,

  // storage
  pub memory: Vec<i32>,
  pub mem_addr: usize,
  pub disk: Vec<i32>,
  pub disk_addr: usize,
  disk_dirty: bool,

  // Dense labels with generation stamp
  label_pos: Vec<i32>,
  label_epoch: Vec<u32>,
  epoch: u32,
  skip: Vec<u16>,

  // Latched input: only advances when `inp`
  input_pos: usize,

  // run() support
  call_stack: Vec<Frame>,
  blocks: Vec<Vec<Decoded>>, // cache of decoded blocks

  // screen
  screen: Vec<[u8; 3]>,
  last_screen: Vec<[u8; 3]>,
  screen_x: i32,
  screen_y: i32,
  screen_red: u8,
  screen_green: u8,
  screen_blue: u8,
  last_present: Instant,
  target_frame_ms: u128,
  screen_w: usize,
  screen_h: usize,

  // keyboard
  last_key: Arc<Mutex<i32>>,

  // mouse
  mouse: Arc<Mutex<MouseState>>,

  // speakers
  pub speakers: Arc<Mutex<[Speaker; SPEAKER_COUNT]>>,
  pub speaker_channel: usize,

  #[allow(dead_code)] // speakers throw a fit if this isn't here
  pub audio_handle: Option<AudioHandle>,

  // speech
  speech_pitch: f32,
  speech_f1: f32,
  speech_f2: f32,
  speech_f3: f32,
  speech_noise: f32,
  speech_volume: f32,
  speech_ms: u32,

  speech_samples: Arc<Mutex<VecDeque<f32>>>,

  speech_r1: Resonator,
  speech_r2: Resonator,
  speech_r3: Resonator,

}

impl Cpu {
  fn new(audio_enabled: bool, config: VmConfig) -> Self {
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

    let speakers = Arc::new(Mutex::new([Speaker::default(); SPEAKER_COUNT]));

    let speech_samples = Arc::new(Mutex::new(VecDeque::new()));

    let audio = if audio_enabled {
      match start_audio(Arc::clone(&speakers), Arc::clone(&speech_samples)) {
        Ok(handle) => Some(handle),
        Err(_err) => None,
      }
    } else {
      None
    };

    Self {
      regs,
      sleep_times: 0.0,
      memory: vec![0; 65536],
      mem_addr: 0,
      disk: vec![0; 65536],
      disk_addr: 0,
      disk_dirty: false,
      label_pos: vec![-1; 65536],
      label_epoch: vec![0; 65536],
      epoch: 1,
      skip: Vec::new(),
      input_pos: 0,
      call_stack: Vec::new(),
      blocks: Vec::new(),
      screen: vec![[0, 0, 0]; config.screen_w * config.screen_h],
      last_screen: vec![[255, 255, 255];  config.screen_w * config.screen_h],
      screen_x: 0,
      screen_y: 0,
      screen_red: 255,
      screen_green: 255,
      screen_blue: 255,
      screen_w: config.screen_w,
      screen_h: config.screen_h,
      last_present: Instant::now(),
      target_frame_ms: 20, // <-<-<-<-<-<-<- FRAME RATE -<-<-<-<-<-<-<
      last_key: Arc::new(Mutex::new(0)),
      mouse: Arc::new(Mutex::new(MouseState {
        x: 0,
        y: 0,
        buttons: 0,
      })),
      speakers,
      speaker_channel: 0,
      audio_handle: audio,
      speech_pitch: 0.0f32,
      speech_f1: 0.0f32,
      speech_f2: 0.0f32,
      speech_f3: 0.0f32,
      speech_noise: 0.0f32,
      speech_volume: 0.0f32,
      speech_ms: 0u32,
      speech_samples: speech_samples,
      speech_r1: Resonator::new(),
      speech_r2: Resonator::new(),
      speech_r3: Resonator::new(),

    }
  }

  fn begin_run(&mut self) {
    self.skip.clear();
    self.call_stack.clear();
    self.blocks.clear();
    self.input_pos = 0;

    self.screen.fill([0, 0, 0]);
    self.last_screen.fill([255, 255, 255]);
    self.screen_x = 0;
    self.screen_y = 0;
    self.screen_red = 255;
    self.screen_green = 255;
    self.screen_blue = 255;

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

    self.sleep_times = 0.0

  }

  fn create_blank_disk(&mut self, path: &str) -> Result<(), String> {
    let byte_count = DISK_CELLS * 4;
    let data = vec![0u8; byte_count];

    std::fs::write(path, data).map_err(|e| e.to_string())
  }

  fn load_disk(&mut self, path: &str) -> Result<(), String> {
    if !std::path::Path::new(path).exists() { self.create_blank_disk(path)?; }

    let bytes = match std::fs::read(path) {
      Ok(bytes) => bytes,
      Err(_) => return Ok(()),
    };

    for (i, chunk) in bytes.chunks_exact(4).enumerate() {
      if i >= self.disk.len() {
        break;
      }

      self.disk[i] = i32::from_le_bytes([
        chunk[0],
        chunk[1],
        chunk[2],
        chunk[3],
      ]);
    }

    Ok(())
  }

  fn save_disk(&self, path: &str) -> Result<(), String> {
    let mut bytes = Vec::with_capacity(self.disk.len() * 4);

    for value in &self.disk {
      let value = *value as i32;
      bytes.extend_from_slice(&value.to_le_bytes());
    }

    std::fs::write(path, bytes).map_err(|e| e.to_string())
  }

  fn prog_len(&self, main: &[Decoded], prog: ProgRef) -> i32 {
    match prog {
      ProgRef::Main => main.len() as i32,
      ProgRef::Block(i) => self.blocks[i].len() as i32,
    }
  }

  fn fetch_decoded(&self, main: &[Decoded], prog: ProgRef, pc: i32) -> Decoded {
    match prog {
      ProgRef::Main => main[pc as usize],
      ProgRef::Block(i) => self.blocks[i][pc as usize],
    }
  }

  fn fetch_raw21(&self, main: &[Decoded], prog: ProgRef, idx: usize) -> u32 {
    match prog {
      ProgRef::Main => main[idx].raw21,
      ProgRef::Block(i) => self.blocks[i][idx].raw21,
    }
  }

  fn screen_index(&self, x: i32, y: i32) -> Option<usize> {
    if x < 0 || y < 0 {
      return None;
    }

    let x = x as usize;
    let y = y as usize;

    if x >= self.screen_w || y >= self.screen_h {
      return None;
    }

    Some(y * self.screen_w + x)
  }

  fn signed_to_rgb(value: i32) -> Option<u8> {
    if value < -128 || value > 127 {
      return None;
    }

    Some((value + 128) as u8)
  }

  fn screen_colour_to_terminal(v: [u8; 3]) -> Color {
    Color::Rgb {
      r: v[0],
      g: v[1],
      b: v[2],
    }
  }

  fn screen_draw_pixel(&mut self) {
    if let Some(i) = self.screen_index(self.screen_x, self.screen_y) {
      self.screen[i] = [self.screen_red, self.screen_green, self.screen_blue];
    } else {
      self.regs[REG_IO_STATUS] = IO_SCREEN_OUT_OF_BOUNDS;
    }
  }

  fn screen_clear(&mut self) {
    self.screen.fill([0, 0, 0]);
  }


  fn screen_dump(&self) {
    let mut out = String::with_capacity((self.screen_w + 1) * self.screen_h + 1);

    out.push('\n');

    for y in 0..self.screen_h {
      for x in 0..self.screen_w {
        let [r, g, b] = self.screen[y * self.screen_w + x];

        let brightness = (r as u16 + g as u16 + b as u16) / 3;

        let c = match brightness {
          0..=20 => ' ',
          21..=80 => '░',
          81..=140 => '▒',
          141..=210 => '▓',
          _ => '█',
        };

        out.push(c);
      }

      out.push('\n');
    }

    print!("{}", out);
    let _ = io::stdout().flush();
  }

  fn screen_erase_pixel_at(&mut self, x: i32, y: i32) {
    if let Some(i) = self.screen_index(x, y) {
      self.screen[i] = [0, 0, 0];
    } else {
      self.regs[REG_IO_STATUS] = IO_SCREEN_OUT_OF_BOUNDS;
    }
  }

  fn screen_erase_half(&mut self, selector: i32) {
    let cell_y = self.screen_y / 2;
    let y = if selector < 0 {
      cell_y * 2
    } else {
      cell_y * 2 + 1
    };

    self.screen_erase_pixel_at(self.screen_x, y);
  }

  fn screen_erase_cell(&mut self) {
    let cell_y = self.screen_y / 2;
    let top_y = cell_y * 2;
    let bottom_y = top_y + 1;

    self.screen_erase_pixel_at(self.screen_x, top_y);
    self.screen_erase_pixel_at(self.screen_x, bottom_y);
  }

  fn screen_present_terminal(&mut self) -> Result<(), String> {
    let elapsed = self.last_present.elapsed().as_millis();

    if elapsed < self.target_frame_ms {
      return Ok(());
    }

    self.last_present = Instant::now();

    let mut stdout = io::stdout();

    for y_pair in 0..(self.screen_h / 2) {
      for x in 0..self.screen_w {
        let top_i = (y_pair * 2) * self.screen_w + x;
        let bottom_i = (y_pair * 2 + 1) * self.screen_w + x;

        let top = self.screen[top_i];
        let bottom = self.screen[bottom_i];

        let old_top = self.last_screen[top_i];
        let old_bottom = self.last_screen[bottom_i];

        if top == old_top && bottom == old_bottom {
          continue;
        }

        let fg = Self::screen_colour_to_terminal(top);
        let bg = Self::screen_colour_to_terminal(bottom);

        queue!(
          stdout,
          cursor::MoveTo(x as u16, y_pair as u16),
          SetForegroundColor(fg),
          SetBackgroundColor(bg),
          Print("▀")
        ).map_err(|e| e.to_string())?;

        self.last_screen[top_i] = top;
        self.last_screen[bottom_i] = bottom;
      }
    }

    queue!(stdout, ResetColor).map_err(|e| e.to_string())?;
    stdout.flush().map_err(|e| e.to_string())
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

  fn _cur_prog<'a>(&'a self, main: &'a [Decoded], prog: ProgRef) -> &'a [Decoded] {
    match prog {
      ProgRef::Main => main,
      ProgRef::Block(i) => &self.blocks[i],
    }
  }

  fn execute_interactive(&mut self, program: &[Decoded], speed: i32, input: &[i32], looped: bool, plain: bool, show_stats: bool) {
    use std::time::Duration;

    self.begin_run();

    if !input.is_empty() {
      self.input_pos = 0;
      self.regs[0] = input[0];
    }

    let mut prog = ProgRef::Main;
    let mut pc: i32 = 0;
    let mut tick: u64 = 0;

    let mut stdout = io::stdout();

    let _ = self.load_disk(DISK_PATH);

    let _ = terminal::enable_raw_mode();

    if !plain {
      let _ = execute!(
        stdout,
        terminal::EnterAlternateScreen,
        EnableMouseCapture,
        cursor::Hide,
        terminal::Clear(ClearType::All)
      );
    }

    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_thread = interrupted.clone();

    let key_state = self.last_key.clone();
    let mouse_state = self.mouse.clone();

    thread::spawn(move || {
      loop {
        match event::read() {
          Ok(Event::Key(k)) => {

            if tick % 10_000 == 0 {
              if k.code == KeyCode::Char('c')
                && k.modifiers.contains(event::KeyModifiers::CONTROL)
              {
                interrupted_thread.store(true, std::sync::atomic::Ordering::SeqCst);
                println!("\nExecution terminated by user.");
                continue;
              }
            }

            let code = match k.code {
              KeyCode::Up => -1,
              KeyCode::Down => -2,
              KeyCode::Left => -3,
              KeyCode::Right => -4,
              KeyCode::Esc => 27,
              KeyCode::Char(c) => c as i32,
              _ => 0,
            };

            if let Ok(mut last) = key_state.lock() {
              *last = code;
            }
          }

          Ok(Event::Mouse(m)) => {
            if let Ok(mut mouse) = mouse_state.lock() {
              mouse.x = m.column as i32;

              let base_y = (m.row as i32) * 2;

              match m.kind {
                MouseEventKind::Down(MouseButton::Left)
                | MouseEventKind::Drag(MouseButton::Left) => {
                  mouse.y = base_y; // top pixel
                  mouse.buttons |= 1 << 0;
                }

                MouseEventKind::Down(MouseButton::Right)
                | MouseEventKind::Drag(MouseButton::Right) => {
                  mouse.y = base_y + 1; // bottom pixel
                  mouse.buttons |= 1 << 1;
                }

                MouseEventKind::Down(MouseButton::Middle)
                | MouseEventKind::Drag(MouseButton::Middle) => {
                  mouse.y = base_y;
                  mouse.buttons |= 1 << 2;
                }

                MouseEventKind::Up(MouseButton::Left) => {
                  mouse.buttons &= !(1 << 0);
                }

                MouseEventKind::Up(MouseButton::Right) => {
                  mouse.buttons &= !(1 << 1);
                }

                MouseEventKind::Up(MouseButton::Middle) => {
                  mouse.buttons &= !(1 << 2);
                }

                MouseEventKind::Moved => {}

                _ => {}
              }
            }
          }

          Ok(_) => {}

          Err(_) => {}
        }
      }
    });

    let mut sys = System::new_all();
    sys.refresh_cpu_all();

    let cpu_mhz =
      sys.cpus()
      .first()
      .map(|cpu| cpu.frequency())
      .unwrap_or(0) as f64;

    let cpu_hz = cpu_mhz * 1_000_000.0;

    let tracing = std::env::args().any(|a| a == "--trace");

    let mut trace = if tracing {
      let trace_file = File::create("trace.txt").expect("failed to create trace file");
      let mut trace = BufWriter::new(trace_file);

      write!(trace, "tick,line").ok();
      for i in 0..256 {
        write!(trace, ",r{:02X}", i).ok();
      }
      writeln!(trace).ok();

      Some(trace)
    } else {
      None
    };

    let start = Instant::now();

    loop {
      let len = self.prog_len(program, prog);

      if tick % 10_000 == 0 {
        if interrupted.load(std::sync::atomic::Ordering::SeqCst) { break; }
      }

      if pc < 0 || pc >= len {
        break;
      }

      let d = self.fetch_decoded(program, prog, pc);
      tick += 1;

      if speed == 0 {
        let _ = read_line();
      } else if speed > 0 {
        std::thread::sleep(Duration::from_secs_f64(1.0 / speed as f64));
      }

      if let Some(trace) = trace.as_mut() {
        write!(trace, "{},{}", tick, pc + 1).ok();

        for val in self.regs.iter() {
          write!(trace, ",{}", val).ok();
        }

        writeln!(trace).ok();
      }

      let (next_prog, next_pc, did_stop) = self.step(d, prog, pc, input, program, looped);
      prog = next_prog;
      pc = next_pc;

      if did_stop {
        break;
      }
    }

    let dt = start.elapsed().as_secs_f64() - self.sleep_times;

    if let Some(trace) = trace.as_mut() { trace.flush().ok(); }

    let _ = terminal::disable_raw_mode();

    if plain {
      let _ = execute!(
        stdout,
        ResetColor,
        cursor::Show,
        DisableMouseCapture
      );
    } else {
      let _ = execute!(
        stdout,
        ResetColor,
        cursor::Show,
        DisableMouseCapture,
        terminal::LeaveAlternateScreen
      );
    }

    println!();

    if show_stats {
      let hz = if dt > 0.0 { (tick as f64) / dt } else { 0.0 };
      let ns_per = if tick > 0 { (dt * 1e9) / (tick as f64) } else { 0.0 };
      let cycles_per_instruction = if hz > 0.0 { cpu_hz / hz } else { 0.0 };

      println!("Done: ticks={} time={:.10}s 0xFF={}", tick, dt, self.regs[0xFF]);
      println!("Clock speed was revealed to be {:.3} Hz = ~{:.3} MHz", hz, hz / 1_000_000.0);
      println!("~{:.3} ns / instruction", ns_per);
      println!("Estimated vm cost: {:.2} host CPU cycles / instruction", cycles_per_instruction);
    }

    if self.disk_dirty { let _ = self.save_disk(DISK_PATH); }

  }

  fn execute_noio(&mut self, program: &[Decoded], input: &[i32]) -> u64 {
    self.begin_run();

    if !input.is_empty() {
      self.input_pos = 0;
      self.regs[0] = input[0];
    }

    let mut prog = ProgRef::Main;
    let mut pc: i32 = 0;
    let mut tick: u64 = 0;

    loop {
      let len = self.prog_len(program, prog);
      if pc < 0 || pc >= len {
        break;
      }

      let d = self.fetch_decoded(program, prog, pc);
      tick += 1;

      let (next_prog, next_pc, did_stop) = self.step(d, prog, pc, input, program, true);
      prog = next_prog;
      pc = next_pc;

      if did_stop {
        break;
      }
    }

    tick
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
          if let Some(c) = char::from_u32(self.regs[r] as u32) {
            print!("{}", c);
            let _ = io::stdout().flush();
          }
        }

        // int
        0x1 => {
          print!("{}", value);
          let _ = io::stdout().flush();
        }

        // newline
        0x2 => {
          print!("\r\n");
          let _ = io::stdout().flush();
        }

        // hex
        0x3 => {
          print!("{:08X}", value);
          let _ = io::stdout().flush();
        }

        // error
        0x4 => {
          if self.regs[238] != 0 {
            print!("Crawssembly Error Code {}", self.regs[238]);
            let _ = io::stdout().flush();
          }
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // time
      0x1 => match command {
        // raw time
        0x0 => {
          let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

          self.write_reg(r, now as i32);
        }

        // 2038-safe time
        0x1 => {
          let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

          self.write_reg(r, (now & 0x7FFF_FFFF) as i32);
        }

        // sleep
        0x2 => {
          thread::sleep(Duration::from_millis(value as u64));
          self.sleep_times += value as f64 / 1000.0
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // screen
      0x2 => match command {
        // set x
        0x0 => {
          self.screen_x = value;
        }

        // set y
        0x1 => {
          self.screen_y = value;
        }

        // draw pixel
        0x2 => {
          self.screen_draw_pixel();
        }

        // clear
        0x3 => {
          self.screen_clear();
        }

        // dump
        0x4 => {
          self.screen_dump();
        }

        // present
        0x5 => {
          if self.screen_present_terminal().is_err() {
            self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
          }
        }

        // red colour
        0x6 => {
          if let Some(v) = Self::signed_to_rgb(value) {
            self.screen_red = v;
          } else {
            self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
          }
        }

        // green colour
        0x7 => {
          if let Some(v) = Self::signed_to_rgb(value) {
            self.screen_green = v;
          } else {
            self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
          }
        }

        // blue colour
        0x8 => {
          if let Some(v) = Self::signed_to_rgb(value) {
            self.screen_blue = v;
          } else {
            self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
          }
        }

        // clear single pixel
        0x9 => {
          self.screen_erase_half(value);
        }

        // clear entire cell
        0xA => {
          self.screen_erase_cell();
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // keyboard
      0x3 => match command {
        // get last pressed key
        0x0 => {
          let key = if let Ok(mut last) = self.last_key.lock() {
            let key = *last;
            *last = 0;
            key
          } else {
            self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
            0
          };

          self.write_reg(r, key);
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      },

      // mouse
      0x4 => match command {

        // x value
        0x0 => {
          let x = if let Ok(mouse) = self.mouse.lock() {
            mouse.x
          } else {
            self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
            0
          };

          self.write_reg(r, x);
        }

        // y value
        0x1 => {
          let y = if let Ok(mouse) = self.mouse.lock() {
            mouse.y
          } else {
            self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
            0
          };

          self.write_reg(r, y);
        }

        // buttons
        0x2 => {
          let buttons = if let Ok(mut mouse) = self.mouse.lock() {
            let b = mouse.buttons;
            mouse.buttons &= 0b0000_0000_0000_0111;
            b
          } else {
            self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
            0
          };

          self.write_reg(r, buttons);
        }

        _ => { self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND; }
      },

      // speaker
      0x5 => {

        let mut speakers = match self.speakers.lock() {
          Ok(speakers) => speakers,
          Err(_) => {
            self.regs[REG_IO_STATUS] = IO_UNAVAILABLE;
            return;
          }
        };

        match command {

          // channel
          0x0 => {
            if value < 0 || value as usize >= SPEAKER_COUNT {
              self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
            } else {
              self.speaker_channel = value as usize;
            }
          }

          // freq
          0x1 => {
            if value <= 0 {
              self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
            } else {
              speakers[self.speaker_channel].freq = value as f32;
            }
          }

          // volume
          0x2 => {
            speakers[self.speaker_channel].vol = value.clamp(0, 100) as f32 / 100.0;
          }

          // wave
          0x3 => {
            if value < 0 || value > (SPEAKER_COUNT as i32) {
              self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
            } else {
              speakers[self.speaker_channel].wave = value; }
          }

          // on
          0x4 => {
            speakers[self.speaker_channel].enabled = true;
          }

          // off
          0x5 => {
            speakers[self.speaker_channel].enabled = false;
          }

          // toggle
          0x6 => {
            speakers[self.speaker_channel].enabled = !speakers[self.speaker_channel].enabled;
          }

          _ => { self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND; }

        }
      }

      // mem
      0x6 => match command {

        // addr
        0x0 => {
          if value < 0 || value > 65535 {
            self.regs[REG_IO_STATUS] = IO_BAD_VALUE;
          } else {
          self.mem_addr = value as usize;
          //println!("mem_addr now equals '{}'", self.mem_addr);
          }
        }

        // read
        0x1 => {
          self.write_reg(r, self.memory[self.mem_addr]);
          //println!("mem addr {} read to {}", self.mem_addr, r);
        }

        // write
        0x2 => {
          self.memory[self.mem_addr] = value;
        }

        _ => { self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND }

      }

      // disk ('k' used for magnetic-based value saving)
      0x7 => match command {

        // addr
        0x0 => {
          if value < 0 || value > 65535 { self.regs[REG_IO_STATUS] = IO_BAD_VALUE; return; }
          self.disk_addr = value as usize;
          //println!("disk_addr now equals '{}'", self.mem_addr)
        }

        // read
        0x1 => {
          self.write_reg(r, self.disk[self.disk_addr]);
          //println!("disk addr {} read to {}", self.disk_addr, r);
        }

        // write
        0x2 => {
          self.disk[self.disk_addr] = value;
          self.disk_dirty = true;
          //println!("disk addr {} written with {}", self.disk_addr, value);
        }

        // save
        0x3 => { let _ = self.save_disk(DISK_PATH); }

        _ => { self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND }

      }

      // speech
      0x8 => match command {

        // pitch
        0x0 => self.speech_pitch = value.max(0) as f32,

        // f1
        0x1 => self.speech_f1 = value.max(0) as f32,

        // f2
        0x2 => self.speech_f2 = value.max(0) as f32,

        // f3
        0x3 => self.speech_f3 = value.max(0) as f32,

        // noise 0..100
        0x4 => self.speech_noise = (value.clamp(0, 100) as f32) / 100.0,

        // volume 0..100
        0x5 => self.speech_volume = (value.clamp(0, 100) as f32) / 100.0,

        // ms
        0x6 => self.speech_ms = value.max(1) as u32,

        // speak
        0x7 => {
          self.speech_speak();
        }

        _ => {
          self.regs[REG_IO_STATUS] = IO_INVALID_COMMAND;
        }
      }

      _ => {
        self.regs[REG_IO_STATUS] = IO_INVALID_DEVICE;
      }
    }
  }

  fn envelope(&mut self, n: usize, total: usize) -> f32 {
    let attack = (total / 10).max(1);
    let release = (total / 8).max(1);

    if n < attack {
      n as f32 / attack as f32
    } else if n + release > total {
      (total - n) as f32 / release as f32
    } else {
     1.0
    }
  }

  fn speech_speak(&mut self) {
    let sample_rate = 44100.0f32;
    let samples = ((self.speech_ms as f32 / 1000.0f32) * sample_rate) as usize;

    let mut out = Vec::with_capacity(samples);
    let mut phase = 0.0f32;
    let mut rng = 1u32;

    for n in 0..samples {
      let env = self.envelope(n, samples);

      let voiced = if self.speech_pitch > 0.0 {
        phase += self.speech_pitch / sample_rate;

        while phase >= 1.0 {
          phase -= 1.0;
        }

        // sawtooth/glottal-ish buzz
        2.0f32 * phase - 1.0f32
      } else {
        0.0
      };

      rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
      let noise = ((rng >> 16) as f32 / 32768.0f32) - 1.0f32;

      let source =
        voiced * (1.0f32 - self.speech_noise)
        + noise * self.speech_noise;

      let y1 = self.speech_r1.process(source, self.speech_f1, 90.0f32, sample_rate) * 0.75f32;
      let y2 = self.speech_r2.process(source, self.speech_f2, 120.0f32, sample_rate) * 0.45f32;
      let y3 = self.speech_r3.process(source, self.speech_f3, 180.0f32, sample_rate) * 0.25f32;

      let sample = (y1 + y2 + y3) * self.speech_volume * env * 0.25f32;

      out.push(sample.clamp(-1.0, 1.0));
    }

    if let Ok(mut queue) = self.speech_samples.lock() {
      for sample in out {
        queue.push_back(sample);
      }
    }
  }

  fn write_reg(&mut self, dst: usize, value: i32) {
    if dst == 0 {
      return;
    }

    self.regs[dst] = value;

    if dst == 0xEF {
      if let Some(c) = char::from_u32(value as u32) {
        use std::io::{self, Write};
        print!("{}", c);
        let _ = io::stdout().flush();
      }
    }
  }

  fn step(
    &mut self,
    d: Decoded,
    prog: ProgRef,
    pc: i32,
    input: &[i32],
    main: &[Decoded],
    looped: bool,
  ) -> (ProgRef, i32, bool) {
    // returns (next_prog, next_pc, did_stop)
    if !self.skip.is_empty() {
      if d.op5 == 0b01101 {
        let id = d.imm16;
        if let Some(pos) = self.skip.iter().position(|&x| x == id) {
          self.skip.remove(pos);
        }
      }
      return (prog, pc + 1, false);
    }

    // stp: RETURN if inside run; HALT if in main
    if d.is_stp {
      if let Some(frame) = self.call_stack.pop() {
        return (frame.prog, frame.pc, false);
      }
      return (prog, pc, true);
    }

    if d.is_nop {
      return (prog, pc + 1, false);
    }

    if d.is_inp {
      if !input.is_empty() {
        self.input_pos = self.input_pos + 1;
        if looped { self.input_pos = (self.input_pos + 1) % input.len(); }
        if self.input_pos >= input.len() { self.regs[0] = 0; } else { self.regs[0] = input[self.input_pos]; }
      }
      return (prog, pc + 1, false);
    }

    // ALU
    if d.core == 0b10 || d.core == 0b11 {
      let va = if d.core == 0b10 { self.regs[d.a as usize] } else { imm8(d.a) };
      let vb = self.regs[d.b as usize];
      self.regs[1] = alu(d.mode, va, vb);
      return (prog, pc + 1, false);
    }

    // Seperated IO command block for hardware interaction
    if d.op5 == 0b01110 {
      let device = (d.imm16 >> 12) as u8;
      let command = ((d.imm16 >> 8) & 0x0F) as u8;
      let reg = (d.imm16 & 0xFF) as u8;

      self.handle_io(device, command, reg);

      return (prog, pc + 1, false);
    }

    // SAV
    if d.mode == 0b000 {
      if d.core == 0b00 {
        let src = self.regs[d.a as usize];
        let dst = d.b as usize;
        self.write_reg(dst, src);
        return (prog, pc + 1, false);
      }

      if d.core == 0b01 {
        let imm = imm8(d.a);
        let dst = d.b as usize;
        self.write_reg(dst, imm);
        return (prog, pc + 1, false);
      }
    }

    // STR: encoding used by assembler: core=01 mode=001 A=start_u8 B=block_u8
    if d.core == 0b01 && d.mode == 0b001 {
      let start = d.a as usize;
      let mut blk = d.b as u16;
      if blk == 0 {
        let r = self.regs[1];
        if r >= 0 {
          blk = r as u16;
        }
      }

      let end = pc as usize;

      // bounds check against CURRENT program length
      let cur_len = self.prog_len(main, prog) as usize;

      if start <= end && end <= cur_len {
        let mut instrs: Vec<Instr> = Vec::with_capacity(end - start);
        for i in start..end {
          instrs.push(self.fetch_raw21(main, prog, i) & INSTR_MASK);
        }
        let _ = write_block(blk, &instrs);
      }

      return (prog, pc + 1, false);
    }

    // Control ops via op5
    match d.op5 {
      0b01111 => {
        self.label_set(d.imm16, pc);
        (prog, pc + 1, false)
      }

      0b00111 => {
        let t = self.label_get(d.imm16);
        if t >= 0 { (prog, t, false) } else { (prog, pc + 1, false) }
      }

      0b00001 => {
        if self.regs[1] == 0 {
          let t = self.label_get(d.imm16);
          if t >= 0 { (prog, t, false) } else { (prog, pc + 1, false) }
        } else {
          (prog, pc + 1, false)
        }
      }

      0b00010 => {
        if self.regs[1] > 0 {
          let t = self.label_get(d.imm16);
          if t >= 0 { (prog, t, false) } else { (prog, pc + 1, false) }
        } else {
          (prog, pc + 1, false)
        }
      }

      0b00100 => {
        if self.regs[1] < 0 {
          let t = self.label_get(d.imm16);
          if t >= 0 { (prog, t, false) } else { (prog, pc + 1, false) }
        } else {
          (prog, pc + 1, false)
        }
      }

      0b00110 => {
        if self.regs[1] != 0 {
          self.skip.push(d.imm16);
        }
        (prog, pc + 1, false)
      }

      0b00101 => {
        if self.regs[1] <= 0 {
          self.skip.push(d.imm16);
        }
        (prog, pc + 1, false)
      }

      0b00011 => {
        if self.regs[1] >= 0 {
          self.skip.push(d.imm16);
        }
        (prog, pc + 1, false)
      }

      0b01101 => {
        self.label_rmv(d.imm16);
        (prog, pc + 1, false)
      }

      0b01011 => {
        let target_1_based = if d.imm16 == 0 { self.regs[1] } else { d.imm16 as i32 };
        if target_1_based >= 1 {
          (prog, target_1_based - 1, false)
        } else {
          (prog, pc + 1, false)
        }
      }

      // RUN: 01010 + u16 (run 0 uses r01)
      0b01010 => {
        let mut blk = d.imm16;
        if blk == 0 {
          let r = self.regs[1];
          if r >= 0 {
            blk = r as u16;
          }
        }

        if let Ok(instrs) = read_block(blk) {
          let decoded_block = predecode(&instrs);
          let idx = self.blocks.len();
          self.blocks.push(decoded_block);

          // return to instruction after run
          self.call_stack.push(Frame { prog, pc: pc + 1 });

          // enter block at pc=0
          return (ProgRef::Block(idx), 0, false);
        }

        (prog, pc + 1, false)
      }

      _ => (prog, pc + 1, false)
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

    // shift left
    0b100 => a.wrapping_shl((b & 31) as u32),

    // logical shift right (zero-fill)
    0b101 => ((a as u32) >> (b & 31)) as i32,

    // arithmetic shift right (sign-preserving)
    0b110 => a >> (b & 31),

    0b111 => a.wrapping_add(b),

    _ => unreachable!(),
  }
}
