# CRISP COMPILER
# (C) 2026 CRAW SYSTEMS

from imm import i as construct

RAM_SIZE = 65536 # 2^16

class Compiler:
  def __init__(self, crisp):
    self.crisp = crisp
    self.crawssembly = []

    self.mem_addr = {}

    self.empty_addrs = [i for i in range(RAM_SIZE)]
    self.full_addrs = []

    self.empty_temp_vars = [f"_CRISP_TEMP_{i}" for i in range(256)]
    self.full_temp_vars = []

    self.comment_str = "/?/"
    self.crawssembly_str = "/c/"

    self.line_index = 0

    self.craw_mode = False

  def int_to_hex(self, value): return f"{value:x}".zfill(2)

  def is_integer(self, text):
    try: int(text); return True
    except ValueError: return False

  def construct_int(self, value, out=1):
    craw = []

    if isinstance(value, (int, str)): value = int(value)

    elif isinstance(value, list):
      if isinstance(value[0], (int, str)): value = int(value[0])
      else: raise Exception(f"CRISP Error: '{value}' is not a number (Line {self.line_index + 1})")

    else: raise Exception(f"CRISP Error: '{value}' is not a number (Line {self.line_index + 1})")

    if -128 <= value <= 127:
      craw.append(f"sav {value} r{self.int_to_hex(out)}")
      return craw

    craw.append("sav 7 r02")
    blocks = construct(value, 0, False)

    for i, block in enumerate(blocks):
      if i + 1 == len(blocks):
        craw.append(f"cal add {block} r01")
      else:
        craw.extend([
          f"sav {block} r01",
          "cal shl r01 r02",
        ])

    if out != 1: craw.append(f"sav r01 r{self.int_to_hex(out)}")

    return craw

  def reset_regs(self, regstart=1, regend=238):
    for i in range(regend - regstart + 1): self.crawssembly.append(f" sav 0 r{self.int_to_hex(regstart + i)}")

  def assign_var(self, name, value, recursion=1):
    string = False
    array = False

    if name in self.mem_addr:
      self.crawssembly.append("sav 0 r02")

      for addr in self.mem_addr[name][0]:
        self.crawssembly.extend(self.construct_int(addr))
        self.crawssembly.extend([
          "io mem addr r01",
          "io mem write r02"
        ])


        if isinstance(addr, list): self.empty_addrs.append(addr[0]); self.full_addrs.remove(addr[0])
        else: self.empty_addrs.append(addr); self.full_addrs.remove(addr)

        self.empty_addrs.sort()

      del self.mem_addr[name]

    if value and value[0].startswith('"') and value[-1].endswith('"'): string = True
    elif value and value[0].startswith("[") and value[-1].endswith("]"): array = True

    if string: final_value = ""
    elif array: final_value = []

    mem = self.empty_addrs[0]

    needs_solving = False

    for idx, v in enumerate(value):

      if string:
        if '"' not in v: final_value += str(v)
        else: final_value += str(v).replace('"', "")
        if idx != len(value) - 1: final_value += " " # restores space from line.split()

      elif array: pass # this will be complex for sure O_O

      else:
        if not self.is_integer(v):
          needs_solving = True
          break

        else:
          self.mem_addr.update({name : [[mem], "int"]})

          self.crawssembly.extend(self.construct_int(v, 1))
          self.crawssembly.extend(self.construct_int(mem, 2))
          self.crawssembly.extend([
            "io mem addr r02",
            "io mem write r01"
          ])

          self.empty_addrs.remove(mem)
          self.full_addrs.append(mem)

          break # singe reg only, skips string and array parsing

      if string:

        self.crawssembly.append("sav 8 r03")

        used_addr = []

        for i, c in enumerate(final_value): # compress 32-bit value into 4 chars for storage
          value = ord(c)

          if (i % 4) == 0: # first byte
            word_addr = self.empty_addrs.pop(0)
            self.full_addrs.append(word_addr)
            used_addr.append(word_addr)

            self.crawssembly.extend(self.construct_int(word_addr))
            self.crawssembly.extend([
             "io mem addr r01",
             f"sav {value} r02",
             "cal shl r02 r03"
            ])

          elif (i % 4) == 3: # last byte
            self.crawssembly.extend([
              f"cal add {value} r01",
              "io mem write r01"
            ])

          else: # middle two bytes
            self.crawssembly.extend([
              f"cal add {value} r01",
              "cal shl r01 r03"
            ])

        self.mem_addr.update({name : [used_addr, "str"]})

    if needs_solving:
      (data_type, memstart, memend) = self.solve_expression(value, recursion + 1)
      # something something...

  def print_value(self, data_type, mem_list):

    if data_type == "str":
      self.crawssembly.append("sav 8 r03")

      self.crawssembly.extend(self.construct_int(255)) # byte 4 mask
      self.crawssembly.append("sav r01 r07")

      self.crawssembly.extend(self.construct_int(65280)) # byte 3 mask
      self.crawssembly.append("sav r01 r06")

      self.crawssembly.extend(self.construct_int(16711680)) # byte 2 mask
      self.crawssembly.append("sav r01 r05")

      self.crawssembly.extend(self.construct_int(-16777216)) # byte 1 mask
      self.crawssembly.append("sav r01 r04")

      for mem in mem_list:
        for byte in range(4):

          if byte == 0: # first byte
            self.crawssembly.extend(self.construct_int(mem))

            self.crawssembly.extend([
              "io mem addr r01",
              "io mem read r02",
              "cal and r02 r07",
              "io text char r01"
            ])

          elif byte == 1: # second byte
            self.crawssembly.extend([
              "cal and r02 r06",
              "cal shr r01 r03",
              "io text char r01"
            ])

          elif byte == 2: # third byte
            self.crawssembly.extend([
              "cal and r02 r05",
              "cal shr r01 r03",
              "cal shr r01 r03",
              "io text char r01"
            ])

          else: # last byte
            self.crawssembly.extend([
              "cal and r02 r04",
              "cal shr r01 r03",
              "cal shr r01 r03",
              "cal shr r01 r03",
              "io text char r01"
            ])

      return

    elif data_type == "array": print("can't do ts yet"); return

    elif data_type == "int": # works for single-mem ints only atm
      self.crawssembly.extend(self.construct_int(mem_list[0]))

      self.crawssembly.extend([
        "io mem addr r01",
        "io mem read r01",
        "io text int r01"
      ])

  def get_var(self, name, regstart, recursion=1):
    data_length = len(self.mem_addr[name][0])

    for i in range(data_length):
      self.crawssembly.extend(self.construct_int(self.mem_addr[name][0][i], self.int_to_hex(recursion)))
      self.crawssembly.extend([
        f"io mem addr r{self.int_to_hex(recursion)}",
        f"io mem read r{self.int_to_hex(regstart + i)}"
      ]) # loads each piece of data from memory into regs from regstart -> regstart + data_length

    return self.mem_addr[name] # returns variable data

  def solve_expression(self, expression, recursion=1, single_bypass=False): # solves varA X varB
    is_array = False
    is_int = False
    is_str = False

    memA = None
    nameA = None
    memB = None
    nameB = None

    if not single_bypass:
      if len(expression) == 0: raise Exception(f"CRISP Error: Empty expressions can't be parsed. (Line {self.line_index + 1})")

      if len(expression) == 1:
        if '"' not in expression[0] and ("[" not in expression[0] and "]" not in expression[0]):
          if expression[0] not in self.mem_addr: raise Exception(f"CRISP Error: {expression[0]} is not defined (Line {self.line_index + 1})")
          else: return self.mem_addr[expression[0]][-1], self.mem_addr[expression[0]][0]

        elif '"' in expression[0] or self.is_integer(expression[0]): return self.solve_expression(expression, recursion, single_bypass=True)

        elif ("[" in expression[0] and "]" in expression[0]): raise Exception("Still can't do arrays yet fool")

        else: raise Exception(f"CRISP Error: I don't know how to parse '{expression[0]}' (Line {self.line_index + 1})")

    if len(expression) != 3 and not single_bypass: raise Exception(f"CRISP Error: Expression lengths must be of the form 'value' 'operation' 'value' (Line {self.line_index + 1})")

    for idx, token in enumerate(expression):
      if self.is_integer(token): is_int = True
      elif "[" in token or "]" in token: is_array = True
      elif '"' in token: is_str = True

      if is_str and is_int: raise Exception(f"CRISP Error: Cannot solve expression with int and string types (Line {self.line_index + 1})")
      if is_str and is_array: raise Exception(f"CRISP Error: Cannot solve expression with array and string types (Line {self.line_index + 1})")
      if is_int and is_array: raise Exception(f"CRISP Error: Cannot solve expression with array and integer types (Line {self.line_index + 1})")

      if token not in ("+", "-"):
        if '"' in token:
          token = token.replace('"', "")

          self.assign_var(self.empty_temp_vars[0], token, recursion + 1)
          self.full_temp_vars.append(self.empty_temp_vars[0])

          if idx == 0:
            nameA = self.empty_temp_vars[0]
            memA = self.mem_addr[nameA][0]
          elif idx == 2:
            nameB = self.empty_temp_vars[0]
            memB = self.mem_addr[nameB][0]
          else: raise Exception(f"CRISP Error: Cannot solve expression with >3 components (Line {self.line_index + 1})")

          self.empty_temp_vars.pop(0)

      if token == "-":
        if not is_int:
          if is_str: raise Exception("CRISP Error: Cannot subtract a str type")
          else: raise Exception("CRISP Error: Cannot subtract an array type")

        self.get_var(nameA, 3, recursion + 1)
        self.get_var(nameB, 4, recursion + 1)

        mem = self.empty_addrs.pop(0)
        self.full_addrs.append(mem)

        self.crawssembly.extend(self.construct_int(mem))
        self.crawssembly.extend([
          "io mem addr r01",
          "cal not r04 r04",
          "cal add 1 r01",
          "cal add r01 r03",
          "io mem write r01"
        ])

        return "int", [mem]

      if token == "+":
        data_length = len(memB) + len(memA) + recursion

        if data_length < 220: # 0x03-0xDF
          self.get_var(nameA, 3, recursion + 1)
          self.get_var(nameB, 3 + len(memA), recursion + 1)

          if is_array: raise Exception("Can't do ts yet")

          elif is_int:
            mem = self.empty_addrs.pop(0)
            self.full_addrs.append(mem)

            self.crawssembly.extend(self.construct_int(mem))
            self.crawssembly.extend([
              "io mem addr r01",
              "cal add r03 r04",
              "io mem write r01"
            ])

            return "int", [mem]

          elif is_str: raise Exception("Can't do ts yet")

        else: # data length is too large for register use
          raise Exception("Data too big, come back later when I've done ts")

    return None, None

  def compile_lines(self):
    for line_index, line in enumerate(self.crisp):

      if line in ("", None, "\n"): continue

      if line.strip() == self.crawssembly_str: self.craw_mode = not self.craw_mode; continue

      if self.craw_mode:
        self.crawssembly.append(line.strip())
        continue

      self.line_index = line_index

      self.line_indent = len(line) - len(line.lstrip(" "))

      print(f"line: {line}memory: {self.mem_addr}\n")

      try: tokens = line.split(self.comment_str)[0].split(" ")
      except: continue

      if len(tokens) == 0: continue
      else: command = tokens[0]

      for i, token in enumerate(tokens):
        if i == (len(tokens) - 1) and token.endswith("\n"): tokens[-1] = token[:-1]

      if self.line_indent != 0: tokens = tokens[self.line_indent:]

      if command == "let":
        var = tokens[1]
        value = tokens[tokens.index("=") + 1:]
        self.assign_var(var, value, recursion=1)

      elif command == "print":
        expression = tokens[1:]
        (data_type, mem) = self.solve_expression(expression)
        self.print_value(data_type, mem)

      else: raise Exception("CRISP Error: Unknown command '{command}' (Line {self.line_index})")

    print(f"memory: {self.mem_addr}")









def main():
  with open("code.crisp", "r") as f: crisp = f.readlines()
  compiler = Compiler(crisp)
  compiler.compile_lines()
  with open("assembly.craw", "w") as f:
    for line in compiler.crawssembly: f.write(line + "\n")

if __name__ == "__main__": main()









