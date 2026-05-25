from imm import i as construct

CHANNEL_COUNT = 4
OUTPUT_FILE = "speech.craw"

DEFAULT_MS = 350
BREAK_MS = 10
VOICE_FREQ = 120
WAVE = 1  # sine if supported, use 0 for square

VOICED = {
  "AH", "AA", "AE", "EH", "EE", "IH", "UH", "OO", "OH", "ER",
  "AI", "AY", "OW", "OY", "AW",
  "M", "N", "NG",
  "L", "R", "W", "Y",
  "Z", "ZH", "V", "DH",
  "B", "D", "G", "J"
}

VOWELS = {
  "AH", "AA", "AE", "EH", "EE", "IH", "UH", "OO", "OH", "ER",
  "AI", "AY", "OW", "OY", "AW"
}

STOPS = {"P", "B", "T", "D", "K", "G"}
PAUSES = {"SIL", "PAUSE"}

PHONEMES = {
  # Vowels
  "AH": {"frames": [
    {"ms": 40, "channels": [(0, 124, 8), (1, 600, 5), (2, 1000, 4)]},
    {"ms": 80, "channels": [(0, 122, 24), (1, 680, 14), (2, 1080, 9)]},
    {"ms": 180, "channels": [(0, 118, 35), (1, 720, 18), (2, 1150, 12)]},
    {"ms": 60, "channels": [(0, 114, 18), (1, 660, 10), (2, 1050, 6)]},
  ]},

  "AA": {"frames": [
    {"ms": 45, "channels": [(0, 124, 8), (1, 650, 5), (2, 1050, 4)]},
    {"ms": 90, "channels": [(0, 121, 25), (1, 730, 15), (2, 1160, 9)]},
    {"ms": 170, "channels": [(0, 117, 35), (1, 780, 18), (2, 1240, 12)]},
    {"ms": 55, "channels": [(0, 113, 18), (1, 720, 10), (2, 1160, 6)]},
  ]},

  "AE": {"frames": [
    {"ms": 40, "channels": [(0, 126, 8), (1, 560, 5), (2, 1400, 4)]},
    {"ms": 90, "channels": [(0, 123, 25), (1, 620, 15), (2, 1600, 8)]},
    {"ms": 170, "channels": [(0, 119, 34), (1, 680, 18), (2, 1750, 10)]},
    {"ms": 60, "channels": [(0, 115, 18), (1, 620, 10), (2, 1550, 6)]},
  ]},

  "EH": {"frames": [
    {"ms": 40, "channels": [(0, 126, 8), (1, 420, 5), (2, 1450, 4)]},
    {"ms": 80, "channels": [(0, 123, 25), (1, 500, 15), (2, 1650, 8)]},
    {"ms": 170, "channels": [(0, 119, 35), (1, 540, 18), (2, 1750, 10)]},
    {"ms": 60, "channels": [(0, 115, 18), (1, 480, 10), (2, 1550, 6)]},
  ]},

  "EE": {"frames": [
    {"ms": 50, "channels": [(0, 128, 8), (1, 260, 4), (2, 1900, 3)]},
    {"ms": 90, "channels": [(0, 124, 24), (1, 300, 13), (2, 2150, 7)]},
    {"ms": 200, "channels": [(0, 120, 35), (1, 330, 16), (2, 2300, 8)]},
    {"ms": 60, "channels": [(0, 116, 16), (1, 290, 8), (2, 2050, 4)]},
  ]},

  "IH": {"frames": [
    {"ms": 35, "channels": [(0, 128, 8), (1, 320, 4), (2, 1650, 3)]},
    {"ms": 75, "channels": [(0, 124, 24), (1, 370, 13), (2, 1850, 7)]},
    {"ms": 160, "channels": [(0, 120, 34), (1, 410, 16), (2, 1950, 9)]},
    {"ms": 50, "channels": [(0, 116, 16), (1, 360, 8), (2, 1750, 5)]},
  ]},

  "UH": {"frames": [
    {"ms": 35, "channels": [(0, 124, 8), (1, 340, 4), (2, 850, 3)]},
    {"ms": 75, "channels": [(0, 121, 24), (1, 400, 12), (2, 980, 8)]},
    {"ms": 160, "channels": [(0, 117, 34), (1, 430, 15), (2, 1080, 11)]},
    {"ms": 50, "channels": [(0, 113, 16), (1, 360, 8), (2, 920, 5)]},
  ]},

  "OO": {"frames": [
    {"ms": 45, "channels": [(0, 123, 8), (1, 260, 4), (2, 650, 3)]},
    {"ms": 85, "channels": [(0, 120, 24), (1, 300, 13), (2, 760, 8)]},
    {"ms": 170, "channels": [(0, 116, 35), (1, 330, 16), (2, 840, 12)]},
    {"ms": 60, "channels": [(0, 112, 17), (1, 280, 8), (2, 700, 5)]},
  ]},

  "OH": {"frames": [
    {"ms": 45, "channels": [(0, 123, 8), (1, 420, 4), (2, 800, 3)]},
    {"ms": 85, "channels": [(0, 120, 24), (1, 500, 13), (2, 900, 8)]},
    {"ms": 170, "channels": [(0, 116, 35), (1, 540, 16), (2, 980, 12)]},
    {"ms": 60, "channels": [(0, 112, 17), (1, 460, 8), (2, 820, 5)]},
  ]},

  "ER": {"frames": [
    {"ms": 45, "channels": [(0, 122, 8), (1, 360, 4), (2, 1100, 3)]},
    {"ms": 85, "channels": [(0, 119, 23), (1, 420, 12), (2, 1250, 8)]},
    {"ms": 170, "channels": [(0, 115, 33), (1, 460, 15), (2, 1350, 10)]},
    {"ms": 60, "channels": [(0, 111, 16), (1, 390, 8), (2, 1180, 5)]},
  ]},

  # Diphthongs
  "AI": {"frames": [
    {"ms": 70, "channels": [(0, 124, 22), (1, 700, 14), (2, 1100, 9)]},
    {"ms": 110, "channels": [(0, 121, 34), (1, 620, 18), (2, 1350, 10)]},
    {"ms": 150, "channels": [(0, 118, 35), (1, 450, 16), (2, 1750, 9)]},
    {"ms": 120, "channels": [(0, 115, 26), (1, 310, 12), (2, 2200, 7)]},
    {"ms": 70, "channels": [(0, 112, 12), (1, 270, 6), (2, 2000, 4)]},
  ]},

  "AY": {"frames": [
    {"ms": 90, "channels": [(0, 124, 25), (1, 700, 15), (2, 1100, 9)]},
    {"ms": 140, "channels": [(0, 121, 35), (1, 560, 17), (2, 1450, 9)]},
    {"ms": 170, "channels": [(0, 117, 32), (1, 360, 14), (2, 2100, 8)]},
    {"ms": 120, "channels": [(0, 113, 15), (1, 280, 7), (2, 2200, 4)]},
  ]},

  "OW": {"frames": [
    {"ms": 90, "channels": [(0, 123, 25), (1, 520, 14), (2, 960, 9)]},
    {"ms": 150, "channels": [(0, 119, 35), (1, 450, 16), (2, 880, 11)]},
    {"ms": 160, "channels": [(0, 115, 30), (1, 330, 14), (2, 780, 12)]},
    {"ms": 100, "channels": [(0, 111, 14), (1, 280, 7), (2, 680, 5)]},
  ]},

  "OY": {"frames": [
    {"ms": 90, "channels": [(0, 123, 25), (1, 500, 14), (2, 900, 9)]},
    {"ms": 150, "channels": [(0, 120, 35), (1, 430, 16), (2, 1300, 9)]},
    {"ms": 170, "channels": [(0, 116, 30), (1, 320, 13), (2, 2150, 8)]},
    {"ms": 110, "channels": [(0, 112, 14), (1, 280, 7), (2, 2200, 4)]},
  ]},

  "AW": {"frames": [
    {"ms": 90, "channels": [(0, 124, 25), (1, 760, 15), (2, 1200, 9)]},
    {"ms": 150, "channels": [(0, 121, 35), (1, 650, 17), (2, 1100, 10)]},
    {"ms": 170, "channels": [(0, 117, 31), (1, 390, 14), (2, 820, 11)]},
    {"ms": 110, "channels": [(0, 113, 14), (1, 280, 7), (2, 700, 5)]},
  ]},

  # Nasals
  "M": {"frames": [
    {"ms": 30, "channels": [(0, 116, 8), (1, 220, 4), (2, 750, 2)]},
    {"ms": 90, "channels": [(0, 114, 28), (1, 250, 14), (2, 900, 8)]},
    {"ms": 40, "channels": [(0, 112, 12), (1, 230, 6), (2, 780, 3)]},
  ]},

  "N": {"frames": [
    {"ms": 25, "channels": [(0, 122, 8), (1, 240, 4), (2, 850, 2)]},
    {"ms": 80, "channels": [(0, 120, 28), (1, 280, 14), (2, 1100, 8)]},
    {"ms": 35, "channels": [(0, 117, 12), (1, 250, 6), (2, 900, 3)]},
  ]},

  "NG": {"frames": [
    {"ms": 35, "channels": [(0, 116, 8), (1, 260, 4), (2, 760, 2)]},
    {"ms": 105, "channels": [(0, 114, 28), (1, 300, 14), (2, 850, 8)]},
    {"ms": 40, "channels": [(0, 111, 12), (1, 270, 6), (2, 760, 3)]},
  ]},

  # Liquids / approximants
  "L": {"frames": [
    {"ms": 35, "channels": [(0, 124, 8), (1, 320, 4), (2, 1100, 3)]},
    {"ms": 70, "channels": [(0, 121, 26), (1, 390, 12), (2, 1350, 7)]},
    {"ms": 85, "channels": [(0, 118, 30), (1, 430, 14), (2, 1450, 8)]},
    {"ms": 30, "channels": [(0, 114, 12), (1, 360, 6), (2, 1200, 3)]},
  ]},

  "R": {"frames": [
    {"ms": 35, "channels": [(0, 122, 8), (1, 300, 4), (2, 950, 3)]},
    {"ms": 70, "channels": [(0, 119, 26), (1, 350, 12), (2, 1150, 8)]},
    {"ms": 85, "channels": [(0, 116, 30), (1, 390, 14), (2, 1250, 9)]},
    {"ms": 30, "channels": [(0, 112, 12), (1, 320, 6), (2, 1050, 4)]},
  ]},

  "W": {"frames": [
    {"ms": 35, "channels": [(0, 122, 8), (1, 250, 4), (2, 600, 3)]},
    {"ms": 65, "channels": [(0, 119, 26), (1, 300, 13), (2, 700, 9)]},
    {"ms": 55, "channels": [(0, 116, 30), (1, 420, 12), (2, 950, 7)]},
    {"ms": 25, "channels": [(0, 112, 12), (1, 360, 6), (2, 760, 3)]},
  ]},

  "Y": {"frames": [
    {"ms": 35, "channels": [(0, 126, 8), (1, 250, 4), (2, 2000, 3)]},
    {"ms": 65, "channels": [(0, 123, 26), (1, 300, 12), (2, 2300, 8)]},
    {"ms": 55, "channels": [(0, 120, 30), (1, 430, 12), (2, 2000, 7)]},
    {"ms": 25, "channels": [(0, 116, 12), (1, 360, 6), (2, 1750, 3)]},
  ]},

  # Voiceless fricatives
  "S": {"frames": [
    {"ms": 20, "channels": [(3, 3900, 4)]},
    {"ms": 20, "channels": [(3, 4700, 10)]},
    {"ms": 20, "channels": [(3, 4300, 9)]},
    {"ms": 20, "channels": [(3, 5100, 10)]},
    {"ms": 20, "channels": [(3, 4500, 8)]},
    {"ms": 30, "channels": [(3, 4100, 4)]},
  ]},

  "SH": {"frames": [
    {"ms": 25, "channels": [(3, 2300, 5)]},
    {"ms": 25, "channels": [(3, 3000, 12)]},
    {"ms": 25, "channels": [(3, 2600, 11)]},
    {"ms": 25, "channels": [(3, 3200, 12)]},
    {"ms": 25, "channels": [(3, 2800, 9)]},
    {"ms": 25, "channels": [(3, 2400, 5)]},
  ]},

  "F": {"frames": [
    {"ms": 20, "channels": [(3, 1300, 3)]},
    {"ms": 20, "channels": [(3, 1900, 8)]},
    {"ms": 20, "channels": [(3, 1500, 7)]},
    {"ms": 20, "channels": [(3, 2200, 8)]},
    {"ms": 20, "channels": [(3, 1700, 6)]},
    {"ms": 20, "channels": [(3, 1400, 3)]},
  ]},

  "TH": {"frames": [
    {"ms": 20, "channels": [(3, 2500, 3)]},
    {"ms": 25, "channels": [(3, 3300, 8)]},
    {"ms": 25, "channels": [(3, 2900, 7)]},
    {"ms": 25, "channels": [(3, 3500, 8)]},
    {"ms": 20, "channels": [(3, 3000, 4)]},
  ]},

  "H": {"frames": [
    {"ms": 25, "channels": [(3, 700, 2)]},
    {"ms": 35, "channels": [(3, 950, 8)]},
    {"ms": 25, "channels": [(3, 850, 6)]},
    {"ms": 15, "channels": [(3, 600, 2)]},
  ]},

  # Voiced fricatives
  "Z": {"frames": [
    {"ms": 30, "channels": [(0, 122, 8), (3, 3300, 3)]},
    {"ms": 35, "channels": [(0, 120, 22), (3, 3800, 8)]},
    {"ms": 35, "channels": [(0, 118, 22), (3, 3400, 7)]},
    {"ms": 35, "channels": [(0, 116, 20), (3, 4000, 8)]},
    {"ms": 25, "channels": [(0, 113, 8), (3, 3200, 3)]},
  ]},

  "ZH": {"frames": [
    {"ms": 30, "channels": [(0, 122, 8), (3, 2200, 3)]},
    {"ms": 35, "channels": [(0, 120, 22), (3, 2700, 9)]},
    {"ms": 35, "channels": [(0, 118, 22), (3, 2500, 8)]},
    {"ms": 35, "channels": [(0, 116, 20), (3, 2900, 8)]},
    {"ms": 25, "channels": [(0, 113, 8), (3, 2200, 3)]},
  ]},

  "V": {"frames": [
    {"ms": 25, "channels": [(0, 122, 8), (3, 1200, 2)]},
    {"ms": 35, "channels": [(0, 120, 22), (3, 1550, 7)]},
    {"ms": 35, "channels": [(0, 118, 22), (3, 1350, 6)]},
    {"ms": 30, "channels": [(0, 115, 18), (3, 1700, 7)]},
    {"ms": 15, "channels": [(0, 112, 8), (3, 1200, 2)]},
  ]},

  "DH": {"frames": [
    {"ms": 25, "channels": [(0, 122, 8), (3, 2300, 2)]},
    {"ms": 35, "channels": [(0, 120, 22), (3, 2800, 7)]},
    {"ms": 35, "channels": [(0, 118, 22), (3, 2500, 6)]},
    {"ms": 30, "channels": [(0, 115, 18), (3, 3000, 7)]},
    {"ms": 15, "channels": [(0, 112, 8), (3, 2200, 2)]},
  ]},

  # Stops / plosives
  "P": {"frames": [
    {"ms": 35, "channels": []},
    {"ms": 15, "channels": [(3, 90, 24)]},
    {"ms": 20, "channels": [(3, 1800, 10)]},
    {"ms": 10, "channels": []},
  ]},

  "B": {"frames": [
    {"ms": 25, "channels": [(0, 115, 8)]},
    {"ms": 20, "channels": [(0, 115, 20), (3, 120, 24)]},
    {"ms": 25, "channels": [(0, 120, 16), (3, 700, 8)]},
    {"ms": 20, "channels": []},
  ]},

  "T": {"frames": [
    {"ms": 30, "channels": []},
    {"ms": 15, "channels": [(3, 3200, 14)]},
    {"ms": 20, "channels": [(3, 4200, 9)]},
    {"ms": 10, "channels": []},
  ]},

  "D": {"frames": [
    {"ms": 25, "channels": [(0, 118, 8)]},
    {"ms": 15, "channels": [(0, 120, 18), (3, 1700, 12)]},
    {"ms": 25, "channels": [(0, 122, 14), (3, 2200, 7)]},
    {"ms": 15, "channels": []},
  ]},

  "K": {"frames": [
    {"ms": 35, "channels": []},
    {"ms": 15, "channels": [(3, 1600, 15)]},
    {"ms": 20, "channels": [(3, 2600, 10)]},
    {"ms": 15, "channels": []},
  ]},

  "G": {"frames": [
    {"ms": 25, "channels": [(0, 115, 8)]},
    {"ms": 20, "channels": [(0, 116, 20), (3, 100, 23)]},
    {"ms": 25, "channels": [(0, 120, 15), (3, 900, 8)]},
    {"ms": 20, "channels": []},
  ]},

  # Affricates
  "CH": {"frames": [
    {"ms": 30, "channels": []},
    {"ms": 20, "channels": [(3, 2200, 16)]},
    {"ms": 40, "channels": [(3, 3000, 12)]},
    {"ms": 40, "channels": [(3, 2600, 10)]},
    {"ms": 40, "channels": [(3, 2200, 5)]},
  ]},

  "J": {"frames": [
    {"ms": 25, "channels": [(0, 118, 8)]},
    {"ms": 25, "channels": [(0, 120, 20), (3, 1600, 12)]},
    {"ms": 45, "channels": [(0, 121, 20), (3, 2600, 9)]},
    {"ms": 45, "channels": [(0, 118, 16), (3, 2200, 6)]},
    {"ms": 40, "channels": [(0, 114, 8)]},
  ]},

  # Utility
  "SIL": {"frames": [
    {"ms": 220, "channels": []},
  ]},

  "PAUSE": {"frames": [
    {"ms": 350, "channels": []},
  ]},
}

LETTER_NAMES = {
  "A": ["AY"],
  "B": ["B", "EE"],
  "C": ["S", "EE"],
  "D": ["D", "EE"],
  "E": ["EE"],
  "F": ["EH", "F"],
  "G": ["J", "EE"],
  "H": ["AY", "CH"],
  "I": ["AI"],
  "J": ["J", "AY"],
  "K": ["K", "AY"],
  "L": ["EH", "L"],
  "M": ["EH", "M"],
  "N": ["EH", "N"],
  "O": ["OH"],
  "P": ["P", "EE"],
  "Q": ["K", "Y", "OO"],
  "R": ["AA", "R"],
  "S": ["EH", "S"],
  "T": ["T", "EE"],
  "U": ["Y", "OO"],
  "V": ["V", "EE"],
  "W": ["D", "UH", "B", "L", "Y", "OO"],
  "X": ["EH", "K", "S"],
  "Y": ["W", "AI"],
  "Z": ["Z", "EH", "D"],
}

def insert_large(value, out):
  if -128 <= value <= 127:
    out.append(f"sav {value} r01")
    return out

  out.append("sav 7 r02")
  blocks = construct(value, 0, False)

  for index, block in enumerate(blocks):
    if index + 1 == len(blocks):
      out.append(f"cal add {block} r01")
    else:
      out.extend([
        f"sav {block} r01",
        "cal shl r01 r02",
      ])

  return out

def emit_channel(out, channel):
  out.extend([
    f"sav {channel} r01",
    "io speaker channel r01",
  ])

def emit_freq(out, freq):
  insert_large(freq, out)
  out.append("io speaker freq r01")

def emit_volume(out, volume):
  out.extend([
    f"sav {volume} r01",
    "io speaker volume r01",
  ])

def emit_wave(out, wave):
  out.extend([
    f"sav {wave} r01",
    "io speaker wave r01",
  ])

def emit_on(out):
  out.append("io speaker on rff")

def emit_off(out):
  out.append("io speaker off rff")

def emit_letters(out, chars):
  for char in chars:
    out.append(f"sav {ord(char)} ref")

def emit_sleep(out, ms):
  insert_large(ms, out)
  out.append("io time sleep r01")

def emit_stop_all(out):
  for channel in range(CHANNEL_COUNT):
    emit_channel(out, channel)
    emit_off(out)

def emit_channels(out, channels):
  emit_stop_all(out)

  for channel, freq, volume in channels:
    emit_channel(out, channel)
    emit_freq(out, freq)
    emit_volume(out, volume)
    emit_on(out)

def emit_frame(out, frame):
  emit_channels(out, frame["channels"])
  emit_sleep(out, frame["ms"])

def is_voiced(name):
  return name in VOICED

def is_vowel(name):
  return name in VOWELS

def is_stop(name):
  return name in STOPS

def is_pause(name):
  return name in PAUSES

def should_break_after(name, next_name):
  if next_name is None:
    return True

  if is_pause(name) or is_pause(next_name):
    return True

  if is_voiced(name) and is_voiced(next_name):
    return False

  if is_stop(name):
    return False

  return True

def should_shorten(name, prev_name, next_name):
  if is_pause(name):
    return False

  if is_vowel(name) and next_name is not None and not is_pause(next_name):
    return True

  if name in {"L", "R", "W", "Y"} and next_name is not None:
    return True

  return False

def scaled_frame(frame, factor):
  return {
    "ms": max(10, round(frame["ms"] * factor)),
    "channels": frame["channels"],
  }

def maybe_shorten_frames(name, frames, prev_name, next_name):
  if should_shorten(name, prev_name, next_name):
    return [scaled_frame(frame, 0.72) for frame in frames]

  return frames

def emit_phoneme(out, name, phoneme, prev_name=None, next_name=None):
  out.append(f"; phoneme {name}")

  frames = maybe_shorten_frames(name, phoneme["frames"], prev_name, next_name)

  for frame in frames:
    emit_frame(out, frame)

  if should_break_after(name, next_name):
    emit_stop_all(out)
    emit_sleep(out, BREAK_MS)

def parse_tokens(lines):
  tokens = []

  for line_number, raw_line in enumerate(lines, start=1):
    line = raw_line.split(";")[0].strip().upper()

    if line == "":
      continue

    if line == "-":
      tokens.append(("PAUSE", line_number, " "))
      continue

    if line.startswith('"') and line.endswith('"'):
      text = line[1:-1]

      for char in text:
        if char == " ":
          tokens.append(("PAUSE", line_number, " "))
          continue

        if char not in LETTER_NAMES:
          raise Exception(f"Unknown letter '{char}' on line {line_number}")

        for name in LETTER_NAMES[char]:
          tokens.append((name, line_number, char))

      continue

    if line in LETTER_NAMES and line not in PHONEMES:
      for name in LETTER_NAMES[line]:
        tokens.append((name, line_number, line))

      continue

    if line not in PHONEMES:
      raise Exception(f"Unknown phoneme '{line}' on line {line_number}")

    if line in PAUSES:
      tokens.append((line, line_number, " "))
    else:
      tokens.append((line, line_number, line))

  return tokens

def compile_speech(lines):
  out = [
    "; Generated by speech_parser.py (C) CRAW SYSTEMS",
    "; Input: speech.txt",
    "",
  ]

  for channel in range(CHANNEL_COUNT):
    emit_channel(out, channel)
    emit_wave(out, WAVE)
    emit_volume(out, 0)
    emit_off(out)
    out.append("")

  tokens = parse_tokens(lines)

  for index, (name, line_number, display) in enumerate(tokens):
    prev_name = tokens[index - 1][0] if index > 0 else None
    next_name = tokens[index + 1][0] if index + 1 < len(tokens) else None

    emit_letters(out, display)
    emit_phoneme(out, name, PHONEMES[name], prev_name, next_name)

    out.append("")

  emit_stop_all(out)
  return out

def main():
  with open("speech.txt", "r") as f:
    lines = f.readlines()

  out = compile_speech(lines)

  with open(OUTPUT_FILE, "w") as f:
    f.write("\n".join(out))

  print(f"Wrote {len(out)} lines to {OUTPUT_FILE}")

if __name__ == "__main__": main()
