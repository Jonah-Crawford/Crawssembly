from imm import i as construct

CHANNEL_COUNT = 4
OUTPUT_FILE = "speech.craw"

DEFAULT_MS = 350
SHORT_MS = 90
BREAK_MS = 10
VOICE_FREQ = 120
WAVE = 1  # sine if supported, use 0 for square

PHONEMES = {
  # Vowels
  "AH": {"duration": 360, "channels": [(0, VOICE_FREQ, 35), (1, 700, 18), (2, 1100, 12)]},
  "AA": {"duration": 360, "channels": [(0, VOICE_FREQ, 35), (1, 750, 18), (2, 1200, 12)]},
  "AE": {"duration": 360, "channels": [(0, VOICE_FREQ, 35), (1, 650, 18), (2, 1700, 10)]},
  "EH": {"duration": 350, "channels": [(0, VOICE_FREQ, 35), (1, 500, 18), (2, 1700, 10)]},
  "EE": {"duration": 400, "channels": [(0, VOICE_FREQ, 35), (1, 300, 16), (2, 2200, 8)]},
  "IH": {"duration": 320, "channels": [(0, VOICE_FREQ, 34), (1, 380, 16), (2, 1900, 9)]},
  "UH": {"duration": 320, "channels": [(0, VOICE_FREQ, 34), (1, 400, 15), (2, 1000, 11)]},
  "OO": {"duration": 360, "channels": [(0, VOICE_FREQ, 35), (1, 300, 16), (2, 800, 12)]},
  "OH": {"duration": 360, "channels": [(0, VOICE_FREQ, 35), (1, 500, 16), (2, 900, 12)]},
  "ER": {"duration": 360, "channels": [(0, VOICE_FREQ, 33), (1, 450, 15), (2, 1300, 10)]},

  # Diphthongs
  "AI": {
    "duration": 520,
    "steps": [
      [(0, VOICE_FREQ, 35), (1, 700, 18), (2, 1100, 12)],
      [(0, VOICE_FREQ, 35), (1, 500, 18), (2, 1600, 10)],
      [(0, VOICE_FREQ, 35), (1, 300, 16), (2, 2200, 8)],
    ],
  },
  "AY": {
    "duration": 520,
    "steps": [
      [(0, VOICE_FREQ, 35), (1, 700, 18), (2, 1100, 12)],
      [(0, VOICE_FREQ, 35), (1, 300, 16), (2, 2200, 8)],
    ],
  },
  "OW": {
    "duration": 500,
    "steps": [
      [(0, VOICE_FREQ, 35), (1, 500, 16), (2, 1000, 12)],
      [(0, VOICE_FREQ, 35), (1, 300, 16), (2, 800, 12)],
    ],
  },
  "OY": {
    "duration": 520,
    "steps": [
      [(0, VOICE_FREQ, 35), (1, 500, 16), (2, 900, 12)],
      [(0, VOICE_FREQ, 35), (1, 300, 16), (2, 2200, 8)],
    ],
  },
  "AW": {
    "duration": 520,
    "steps": [
      [(0, VOICE_FREQ, 35), (1, 750, 18), (2, 1200, 12)],
      [(0, VOICE_FREQ, 35), (1, 300, 16), (2, 800, 12)],
    ],
  },

  # Nasals
  "M": {"duration": 160, "channels": [(0, 115, 28), (1, 250, 14), (2, 900, 8)]},
  "N": {"duration": 140, "channels": [(0, 120, 28), (1, 280, 14), (2, 1100, 8)]},
  "NG": {"duration": 180, "channels": [(0, 115, 28), (1, 300, 14), (2, 850, 8)]},

  # Liquids / approximants
  "L": {"duration": 220, "channels": [(0, VOICE_FREQ, 30), (1, 400, 14), (2, 1400, 8)]},
  "R": {"duration": 220, "channels": [(0, VOICE_FREQ, 30), (1, 350, 14), (2, 1200, 9)]},
  "W": {"duration": 180, "channels": [(0, VOICE_FREQ, 30), (1, 300, 15), (2, 700, 10)]},
  "Y": {"duration": 180, "channels": [(0, VOICE_FREQ, 30), (1, 300, 14), (2, 2300, 8)]},

  # Voiceless fricatives
  "S": {"duration": 130, "channels": [(3, 4500, 10)]},
  "SH": {"duration": 150, "channels": [(3, 2800, 12)]},
  "F": {"duration": 120, "channels": [(3, 1800, 8)]},
  "TH": {"duration": 130, "channels": [(3, 3200, 8)]},
  "H": {"duration": 100, "channels": [(3, 900, 8)]},

  # Voiced fricatives
  "Z": {"duration": 160, "channels": [(0, VOICE_FREQ, 22), (3, 3600, 8)]},
  "ZH": {"duration": 160, "channels": [(0, VOICE_FREQ, 22), (3, 2600, 9)]},
  "V": {"duration": 140, "channels": [(0, VOICE_FREQ, 22), (3, 1500, 7)]},
  "DH": {"duration": 140, "channels": [(0, VOICE_FREQ, 22), (3, 2800, 7)]},

  # Stops / plosives
  "P": {"duration": 80, "channels": [(3, 90, 22)]},
  "B": {"duration": 90, "channels": [(3, 120, 24)]},
  "T": {"duration": 75, "channels": [(3, 3000, 13)]},
  "D": {"duration": 80, "channels": [(3, 1800, 12)]},
  "K": {"duration": 85, "channels": [(3, 1700, 15)]},
  "G": {"duration": 90, "channels": [(3, 100, 23)]},

  # Affricates
  "CH": {
    "duration": 170,
    "steps": [
      [(3, 2200, 16)],
      [(3, 3000, 11)],
    ],
  },
  "J": {
    "duration": 180,
    "steps": [
      [(0, VOICE_FREQ, 20), (3, 1600, 12)],
      [(0, VOICE_FREQ, 20), (3, 2600, 9)],
    ],
  },

  # Utility
  "SIL": {"duration": 220, "channels": []},
  "PAUSE": {"duration": 350, "channels": []},
}

def insert_large(value, out):
  if -128 <= value <= 127:
    out.append(f"sav {value} r01")
    return out

  out.append("sav 7 r02")
  blocks = construct(value, 0, False)

  for i, block in enumerate(blocks):
    if i + 1 == len(blocks): out.append(f"cal add {block} r01")
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

def emit_on(out): out.append("io speaker on rff")

def emit_off(out): out.append("io speaker off rff")

def emit_letters(out, chars):
  for c in chars: out.append(f"sav {ord(c)} ref")

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

def emit_sound(out, channels, duration, trailing_break=True):
  emit_channels(out, channels)
  emit_sleep(out, duration)
  emit_stop_all(out)

  if trailing_break: emit_sleep(out, BREAK_MS)

def emit_phoneme(out, name, phoneme):
  out.append(f"; phoneme {name}")

  if "steps" in phoneme:
    step_duration = phoneme["duration"] // len(phoneme["steps"])

    for step in phoneme["steps"]:
      emit_channels(out, step)
      emit_sleep(out, step_duration)

    emit_stop_all(out)
    emit_sleep(out, BREAK_MS)
    return

  emit_sound(out, phoneme["channels"], phoneme["duration"])

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

  for line_number, raw_line in enumerate(lines, start=1):
    line = raw_line.split(";")[0].strip().upper()

    if line == "": continue

    if line == "-":
      out.append(f"; pause line {line_number}")
      emit_stop_all(out)
      emit_sleep(out, DEFAULT_MS)
      out.append("")
      continue

    if line not in PHONEMES: raise Exception(f"Unknown phoneme '{line}' on line {line_number}")

    if line not in ("PAUSE", "SIL"): emit_letters(out, line)
    else: emit_letters(out, " ")

    emit_phoneme(out, line, PHONEMES[line])

    out.append("")

  emit_stop_all(out)
  return out

def main():
  with open("speech.txt", "r") as f: lines = f.readlines()

  out = compile_speech(lines)

  with open(OUTPUT_FILE, "w") as f: f.write("\n".join(out))

  print(f"Wrote {len(out)} lines to {OUTPUT_FILE}")

if __name__ == "__main__": main()
