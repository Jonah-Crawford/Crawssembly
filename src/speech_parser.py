from imm import i as construct

OUTPUT_FILE = "speech.craw"

DEFAULT_MS = 350
BREAK_MS = 10

# frame = (ms, pitch, f1, f2, f3, noise, volume)
PHONEMES = {
  # Vowels
  "AH": [(45, 125, 650, 1150, 2600, 0, 45), (150, 118, 730, 1220, 2550, 0, 80), (60, 112, 680, 1120, 2450, 0, 45)],
  "AA": [(50, 124, 700, 1120, 2550, 0, 45), (170, 116, 820, 1200, 2500, 0, 82), (60, 111, 760, 1100, 2400, 0, 45)],
  "AE": [(45, 126, 580, 1500, 2600, 0, 45), (150, 119, 690, 1780, 2650, 0, 80), (60, 113, 620, 1550, 2500, 0, 45)],
  "EH": [(40, 126, 430, 1550, 2500, 0, 45), (145, 119, 530, 1840, 2600, 0, 78), (55, 113, 470, 1600, 2450, 0, 45)],
  "EE": [(45, 128, 260, 2100, 3050, 0, 42), (170, 121, 310, 2450, 3150, 0, 78), (60, 115, 280, 2200, 3000, 0, 42)],
  "IH": [(40, 128, 310, 1850, 2850, 0, 42), (140, 121, 390, 2100, 2950, 0, 76), (50, 115, 340, 1850, 2800, 0, 42)],
  "UH": [(40, 124, 330, 950, 2300, 0, 42), (140, 117, 430, 1150, 2350, 0, 76), (50, 112, 360, 980, 2200, 0, 42)],
  "OO": [(45, 123, 260, 700, 2200, 0, 42), (160, 116, 310, 850, 2250, 0, 78), (60, 111, 270, 720, 2100, 0, 42)],
  "OH": [(45, 123, 430, 850, 2400, 0, 42), (160, 116, 560, 1000, 2450, 0, 78), (60, 111, 460, 850, 2300, 0, 42)],
  "ER": [(45, 122, 360, 1100, 2200, 0, 42), (160, 115, 470, 1350, 2250, 0, 76), (60, 110, 390, 1150, 2150, 0, 42)],

  # Diphthongs
  "AI": [(80, 125, 720, 1150, 2600, 0, 65), (120, 121, 520, 1600, 2800, 0, 82), (140, 116, 310, 2250, 3100, 0, 65)],
  "AY": [(100, 125, 690, 1150, 2600, 0, 65), (150, 120, 480, 1650, 2850, 0, 82), (120, 115, 300, 2300, 3100, 0, 60)],
  "OW": [(90, 123, 520, 930, 2450, 0, 65), (150, 118, 430, 850, 2350, 0, 82), (120, 113, 300, 720, 2200, 0, 60)],
  "OY": [(90, 123, 500, 900, 2450, 0, 65), (150, 119, 430, 1350, 2700, 0, 82), (130, 114, 310, 2300, 3100, 0, 60)],
  "AW": [(90, 124, 780, 1200, 2600, 0, 65), (150, 119, 600, 1050, 2450, 0, 82), (130, 114, 310, 760, 2200, 0, 60)],

  # Nasals
  "M": [(40, 116, 250, 900, 1800, 0, 42), (90, 114, 270, 1050, 1900, 0, 65), (35, 111, 250, 850, 1800, 0, 35)],
  "N": [(35, 122, 270, 1100, 1950, 0, 42), (80, 119, 300, 1350, 2100, 0, 65), (35, 115, 270, 1050, 1950, 0, 35)],
  "NG": [(45, 116, 280, 850, 1900, 0, 42), (100, 114, 320, 950, 2000, 0, 65), (40, 110, 280, 850, 1900, 0, 35)],

  # Liquids / approximants
  "L": [(45, 124, 330, 1150, 2450, 0, 42), (110, 118, 430, 1500, 2600, 0, 68), (45, 113, 360, 1250, 2400, 0, 38)],
  "R": [(45, 122, 300, 950, 2200, 0, 42), (120, 116, 380, 1250, 2250, 0, 68), (45, 111, 320, 1050, 2150, 0, 38)],
  "W": [(45, 122, 260, 650, 2100, 0, 42), (90, 116, 420, 1000, 2300, 0, 65), (40, 112, 350, 780, 2150, 0, 35)],
  "Y": [(45, 126, 270, 2150, 3050, 0, 42), (90, 120, 390, 2200, 3000, 0, 65), (40, 116, 340, 1800, 2800, 0, 35)],

  # Fricatives
  "S": [(130, 0, 4500, 6200, 7600, 100, 55)],
  "SH": [(150, 0, 2600, 3800, 5600, 100, 60)],
  "F": [(120, 0, 1200, 2500, 4500, 100, 45)],
  "TH": [(130, 0, 2800, 4400, 6200, 100, 42)],
  "H": [(90, 0, 900, 1800, 2800, 80, 35)],

  # Voiced fricatives
  "Z": [(150, 120, 450, 3800, 5800, 45, 60)],
  "ZH": [(150, 120, 450, 2600, 4200, 45, 60)],
  "V": [(130, 120, 430, 1500, 3500, 45, 55)],
  "DH": [(130, 120, 430, 2800, 4500, 45, 55)],

  # Stops / plosives
  "P": [(35, 0, 0, 0, 0, 0, 0), (35, 0, 1300, 2800, 4800, 100, 70)],
  "B": [(30, 115, 250, 750, 1800, 0, 35), (45, 120, 550, 1250, 2500, 30, 65)],
  "T": [(30, 0, 0, 0, 0, 0, 0), (35, 0, 3300, 5000, 7000, 100, 68)],
  "D": [(30, 118, 300, 900, 2000, 0, 35), (45, 122, 700, 1800, 3000, 30, 65)],
  "K": [(35, 0, 0, 0, 0, 0, 0), (40, 0, 1500, 3000, 5200, 100, 72)],
  "G": [(35, 115, 300, 800, 1800, 0, 35), (50, 120, 650, 1400, 2500, 30, 65)],

  # Affricates
  "CH": [(30, 0, 0, 0, 0, 0, 0), (50, 0, 2300, 3500, 5400, 100, 70), (70, 0, 3000, 4500, 6200, 100, 55)],
  "J": [(35, 118, 400, 1000, 2300, 0, 35), (60, 120, 700, 1800, 3000, 40, 65), (60, 118, 500, 2600, 4200, 45, 55)],

  "SIL": [(220, 0, 0, 0, 0, 0, 0)],
  "PAUSE": [(350, 0, 0, 0, 0, 0, 0)],
}

WORDS = {
  "HELLO": ["H", "EH", "L", "OW"],
  "CRAW": ["K", "R", "AW"],
  "SYSTEMS": ["S", "IH", "S", "T", "EH", "M", "Z"],
  "MUM": ["M", "UH", "M"],
  "DAD": ["D", "AE", "D"],
  "CAT": ["K", "AE", "T"],
  "DOG": ["D", "OH", "G"],
  "SPEAK": ["S", "P", "EE", "K"],
  "SPELL": ["S", "P", "EH", "L"],
  # Wikipedia's top 20 words in English (https://en.wikipedia.org/wiki/Most_common_words_in_English)
  "THE": ["F", "UH"],
  "BE": ["B", "EE"],
  "TO": ["T", "OO"],
  "OF": ["OH", "F"],
  "AND": ["AH", "N", "D"],
  "A": ["AY"],
  "IN": ["IH", "N"],
  "THAT": ["TH", "AH", "T"],
  "HAVE": ["H", "AH", "V"],
  "I": ["AI"],
  "IT": ["IH", "T"],
  "FOR": ["F", "AW"],
  "NOT": ["N", "OH", "T"],
  "ON": ["OH", "N"],
  "WITH": ["W", "IH", "TH"],
  "HE": ["H", "EE"],
  "AS": ["AH", "S"],
  "YOU": ["Y", "OO"],
  "DO": ["D", "OO"],
  "AT": ["AH", "T"]
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

      word = ""

      for char in line[1:-1] + " ":
        if char.isalpha():
          word += char
          continue

        if word:
          if word in WORDS:
            for phoneme in WORDS[word]:
              tokens.append((phoneme, word))
          else:
            for letter in word:
              for phoneme in LETTER_NAMES[letter]:
                tokens.append((phoneme, letter))

          word = ""

        if char == " ": tokens.append(("PAUSE", " "))

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
