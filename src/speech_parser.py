from imm import i as construct

OUTPUT_FILE = "speech.craw"

DEFAULT_MS = 350
BREAK_MS = 10

PHONEMES = {
  "AH": [(60, 124, 650, 1100, 2600, 0, 70), (180, 118, 720, 1200, 2600, 0, 85), (60, 114, 650, 1050, 2400, 0, 55)],
  "AA": [(60, 124, 700, 1150, 2600, 0, 70), (180, 117, 780, 1250, 2600, 0, 85), (60, 113, 700, 1100, 2400, 0, 55)],
  "AE": [(60, 126, 580, 1500, 2600, 0, 70), (170, 119, 680, 1750, 2700, 0, 85), (60, 115, 600, 1500, 2500, 0, 55)],
  "EH": [(50, 126, 450, 1500, 2500, 0, 70), (170, 119, 540, 1750, 2600, 0, 85), (60, 115, 480, 1500, 2400, 0, 55)],
  "EE": [(60, 128, 280, 2000, 3000, 0, 70), (200, 120, 330, 2300, 3100, 0, 85), (60, 116, 290, 2050, 2900, 0, 50)],
  "IH": [(50, 128, 330, 1700, 2800, 0, 70), (160, 120, 410, 1950, 2900, 0, 82), (50, 116, 360, 1750, 2700, 0, 50)],
  "UH": [(50, 124, 360, 900, 2300, 0, 70), (160, 117, 430, 1080, 2400, 0, 82), (50, 113, 360, 920, 2200, 0, 50)],
  "OO": [(60, 123, 280, 700, 2200, 0, 70), (170, 116, 330, 840, 2300, 0, 85), (60, 112, 280, 700, 2100, 0, 50)],
  "OH": [(60, 123, 450, 850, 2400, 0, 70), (170, 116, 540, 980, 2500, 0, 85), (60, 112, 460, 820, 2300, 0, 50)],
  "ER": [(60, 122, 380, 1150, 2300, 0, 65), (170, 115, 460, 1350, 2400, 0, 80), (60, 111, 390, 1180, 2200, 0, 50)],

  "AI": [(90, 124, 700, 1100, 2600, 0, 75), (140, 120, 500, 1600, 2800, 0, 85), (160, 115, 300, 2200, 3100, 0, 70)],
  "AY": [(120, 124, 700, 1100, 2600, 0, 75), (180, 117, 360, 2100, 3000, 0, 80), (120, 113, 280, 2200, 3100, 0, 55)],
  "OW": [(100, 123, 520, 960, 2500, 0, 75), (170, 119, 450, 880, 2400, 0, 85), (130, 115, 330, 780, 2200, 0, 65)],
  "OY": [(100, 123, 500, 900, 2500, 0, 75), (170, 120, 430, 1300, 2700, 0, 85), (130, 116, 320, 2150, 3100, 0, 65)],
  "AW": [(100, 124, 760, 1200, 2600, 0, 75), (170, 121, 650, 1100, 2500, 0, 85), (130, 117, 390, 820, 2300, 0, 65)],

  "M": [(40, 116, 220, 750, 1800, 0, 50), (100, 114, 250, 900, 1900, 0, 70), (40, 112, 230, 780, 1800, 0, 45)],
  "N": [(35, 122, 240, 850, 1900, 0, 50), (90, 120, 280, 1100, 2100, 0, 70), (35, 117, 250, 900, 1900, 0, 45)],
  "NG": [(45, 116, 260, 760, 1900, 0, 50), (110, 114, 300, 850, 2000, 0, 70), (45, 111, 270, 760, 1900, 0, 45)],

  "L": [(50, 124, 320, 1100, 2400, 0, 55), (120, 118, 430, 1450, 2600, 0, 75), (50, 114, 360, 1200, 2400, 0, 45)],
  "R": [(50, 122, 300, 950, 2200, 0, 55), (120, 116, 390, 1250, 2300, 0, 75), (50, 112, 320, 1050, 2200, 0, 45)],
  "W": [(45, 122, 250, 600, 2000, 0, 55), (90, 116, 420, 950, 2300, 0, 72), (45, 112, 360, 760, 2100, 0, 45)],
  "Y": [(45, 126, 250, 2000, 3000, 0, 55), (90, 120, 430, 2000, 2900, 0, 72), (45, 116, 360, 1750, 2700, 0, 45)],

  "S": [(130, 0, 4500, 6200, 7600, 100, 60)],
  "SH": [(150, 0, 2800, 4200, 6000, 100, 65)],
  "F": [(120, 0, 1800, 3500, 5200, 100, 50)],
  "TH": [(130, 0, 3200, 4800, 6500, 100, 45)],
  "H": [(100, 0, 900, 1800, 3000, 80, 40)],

  "Z": [(160, 120, 420, 3600, 5000, 45, 65)],
  "ZH": [(160, 120, 450, 2600, 4200, 45, 65)],
  "V": [(140, 120, 420, 1500, 3500, 40, 60)],
  "DH": [(140, 120, 420, 2800, 4500, 40, 60)],

  "P": [(35, 0, 0, 0, 0, 0, 0), (45, 0, 1400, 2600, 4200, 100, 75)],
  "B": [(35, 115, 300, 700, 1800, 0, 45), (55, 120, 500, 1200, 2500, 35, 70)],
  "T": [(30, 0, 0, 0, 0, 0, 0), (45, 0, 3200, 4800, 6500, 100, 70)],
  "D": [(30, 118, 300, 900, 2000, 0, 45), (50, 122, 700, 1700, 2800, 35, 70)],
  "K": [(35, 0, 0, 0, 0, 0, 0), (50, 0, 1600, 3000, 5200, 100, 75)],
  "G": [(35, 115, 300, 700, 1800, 0, 45), (55, 120, 650, 1300, 2400, 30, 70)],

  "CH": [(30, 0, 0, 0, 0, 0, 0), (60, 0, 2200, 3500, 5400, 100, 75), (80, 0, 3000, 4500, 6200, 100, 60)],
  "J": [(40, 118, 400, 1000, 2300, 0, 45), (70, 120, 700, 1800, 3000, 40, 70), (70, 118, 500, 2600, 4200, 45, 60)],

  "SIL": [(220, 0, 0, 0, 0, 0, 0)],
  "PAUSE": [(350, 0, 0, 0, 0, 0, 0)],
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
    return

  out.append("sav 7 r02")
  blocks = construct(value, 0, False)

  for index, block in enumerate(blocks):
    if index + 1 == len(blocks): out.append(f"cal add {block} r01")
    else:
      out.append(f"sav {block} r01")
      out.append("cal shl r01 r02")

def emit_value(out, value): insert_large(value, out)

def emit_speech_param(out, name, value):
  emit_value(out, value)
  out.append(f"io speech {name} r01")

def emit_speech_frame(out, frame):
  ms, pitch, f1, f2, f3, noise, volume = frame

  emit_speech_param(out, "pitch", pitch)
  emit_speech_param(out, "f1", f1)
  emit_speech_param(out, "f2", f2)
  emit_speech_param(out, "f3", f3)
  emit_speech_param(out, "noise", noise)
  emit_speech_param(out, "volume", volume)
  emit_speech_param(out, "ms", ms)
  out.append("io speech speak rff")

  emit_sleep(out, ms)

def emit_letters(out, chars):
  for char in chars:
    emit_value(out, ord(char))
    out.append("sav r01 ref")

def emit_sleep(out, ms):
  emit_value(out, ms)
  out.append("io time sleep r01")

def parse_tokens(lines):
  tokens = []

  for line_number, raw_line in enumerate(lines, start=1):
    line = raw_line.split(";")[0].strip().upper()

    if line == "": continue

    if line == "-":
      tokens.append(("PAUSE", " "))
      continue

    if line.startswith('"') and line.endswith('"'):
      for char in line[1:-1]:
        if char == " ":
          tokens.append(("PAUSE", " "))
          continue

        if char not in LETTER_NAMES: raise Exception(f"Unknown letter '{char}' on line {line_number}")

        for phoneme in LETTER_NAMES[char]: tokens.append((phoneme, char))

      continue

    if line not in PHONEMES: raise Exception(f"Unknown phoneme '{line}' on line {line_number}")

    tokens.append((line, line))

  return tokens

def compile_speech(lines):
  out = [
    "; Generated by speech_parser.py (C) CRAW SYSTEMS",
    "; Speech chip format",
    "; Input: speech.txt",
    "",
  ]

  tokens = parse_tokens(lines)

  for phoneme, display in tokens:
    emit_letters(out, display)
    out.append(f"; phoneme {phoneme}")

    if phoneme in {"SIL", "PAUSE"}:
      emit_sleep(out, PHONEMES[phoneme][0][0])
    else:
      for frame in PHONEMES[phoneme]:
        emit_speech_frame(out, frame)

      emit_sleep(out, BREAK_MS)

    out.append("")

  emit_sleep(out, 300)

  emit_speech_param(out, "volume", 0)
  return out

def main():
  with open("speech.txt", "r") as f:
    lines = f.readlines()

  out = compile_speech(lines)

  with open(OUTPUT_FILE, "w") as f:
    f.write("\n".join(out))

  print(f"Wrote {len(out)} lines to {OUTPUT_FILE}")

if __name__ == "__main__": main()
