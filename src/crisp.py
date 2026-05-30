##########  ##########  ##########  ##########  ##########
##          ##      ##      ##      ####        ##      ##
##          ##########      ##        ######    ##########
##          ##  ####        ##            ####  ##
##########  ##    ####  ##########  ##########  ##

# CRISP COMPILER (C) 2026 CRAW SYSTEMS Jonah Crawford

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

    self.calc_symbols = ["+", "-"]

    self.line_index = 0

    self.craw_mode = False

    self.reserved_keywords = []

  def int_to_hex(self, value): return f"{value:x}".zfill(2)

  def is_integer(self, text):
    try: int(text); return True
    except ValueError: return False

  def construct_int(self, value, out=1):
    craw = []

    if isinstance(out, str): out = int(out, 16)

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

  def assign_var(self, name, value, recursion=1, force_type=None):
    string = False
    array = False

    if isinstance(value, list):
      if value[0].startswith('"') and value[-1].endswith('"'): value = " ".join(value)

    if name in self.reserved_keywords: raise Exception(f"CRISP Error: Can not set variable to reserved keyword '{name}' (Line {self.line_index + 1})")

    print(f"assign_var()  name: {name}, value: {value}, force_type: {force_type}")

    if value in list(self.mem_addr.keys()):
      mem_ref = self.mem_addr[value][0]
      type = self.mem_addr[value][1]

      mem = []

      for addr in mem_ref:
        mem.append(self.empty_addrs.pop(0))
        self.full_addrs.append(mem[0])

      for idx, addr in enumerate(mem_ref):
        self.crawssembly.extend(self.construct_int(addr))
        self.crawssembly.extend(["io mem addr r01", "io mem read r03"])
        self.crawssembly.extend(self.construct_int(mem[idx]))
        self.crawssembly.extend(["io mem addr r01", "io mem write r03"])

      self.mem_addr.update({name : [mem, type]})

      return

    if name in list(self.mem_addr.keys()):
      print(f"assign_var()  name {name} defined")
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

    if force_type:
      if force_type == "str":
        string = True
        array = False
      elif force_type == "int":
        string = False
        array = False
      elif force_type == "array":
        string = False
        array = False
      else: raise Exception(f"CRISP Compile Error: Unknown forced variable type '{force_type}' (Compiling Line {self.line_index})")

    if string: final_value = ""
    elif array: final_value = []

    mem = self.empty_addrs[0]

    needs_solving = False

    if not isinstance(value, list): value = [value]

    print(f"assign_var()  name: {name}, value: {value}, string: {string}, array: {array}")

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

        print(f"assign_var() final_value: {final_value}")

        used_addr = []

        for i, c in enumerate(final_value):
          value = ord(c)

          if (i % 4) == 0:
            word_addr = self.empty_addrs.pop(0)
            self.full_addrs.append(word_addr)
            used_addr.append(word_addr)

            self.crawssembly.extend(self.construct_int(word_addr))
            self.crawssembly.extend([
              "io mem addr r01",
              f"sav {value} r01"
            ])

            if i != len(final_value) - 1:
              self.crawssembly.append("cal shl r01 r03")

          elif (i % 4) == 3:
            self.crawssembly.extend([
              f"cal add {value} r01",
              "io mem write r01"
            ])

          else:
            self.crawssembly.extend([
              f"cal add {value} r01"
            ])

            if i != len(final_value) - 1:
              self.crawssembly.append("cal shl r01 r03")

        if len(final_value) % 4 != 0:
          remaining = 4 - (len(final_value) % 4)

          for _ in range(remaining):
            self.crawssembly.append("cal shl r01 r03")

          self.crawssembly.append("io mem write r01")

        self.mem_addr.update({name : [used_addr, "str"]})

    if needs_solving:
      (data_type, mem, temp_name) = self.solve_expression(value, recursion + 1)
      print("Can't do this yet")

      if temp_name: self.free_temp(temp_name)

  def print_value(self, data_type, mem_list):

    if data_type == "str":
      self.crawssembly.extend([
        "sav 8 r03",

        "sav 127 r01",
        "cal add 127 r01",
        "cal add 1 r01",
        "sav r01 r07",

        "sav r07 r06",
        "cal shl r06 r03",
        "sav r01 r06",

        "sav r06 r05",
        "cal shl r05 r03",
        "sav r01 r05",

        "sav r05 r04",
        "cal shl r04 r03",
        "sav r01 r04"

      ])

      for mem_addr in mem_list:
        self.crawssembly.extend(self.construct_int(mem_addr))

        self.crawssembly.extend([
          "io mem addr r01",
          "io mem read r02",

          "sav r02 r01",
          "cal and r04 r01",
          "cal shr r01 r03",
          "cal shr r01 r03",
          "cal shr r01 r03",
          "io text char r01",

          "sav r02 r01",
          "cal and r05 r01",
          "cal shr r01 r03",
          "cal shr r01 r03",
          "io text char r01",

          "sav r02 r01",
          "cal and r06 r01",
          "cal shr r01 r03",
          "io text char r01",

          "sav r02 r01",
          "cal and r07 r01",
          "io text char r01"
        ])

      return

    elif data_type == "int":
      for mem_addr in mem_list:
        self.crawssembly.extend(self.construct_int(mem_addr))

        self.crawssembly.extend([
          "io mem addr r01",
          "io mem read r01",
          "io text int r01"
        ])

    elif data_type == "array": print("Can't print arrays yet")

  def free_temp(self, names):
    for name in names:

      if name not in list(self.mem_addr.keys()): continue

      addrs, data_type = self.mem_addr[name]

      for addr in addrs:
        self.crawssembly.extend(self.construct_int(addr))
        self.crawssembly.extend([
          "io mem addr r01",
          "io mem write ref" # if `io mem addr r01` works, error code reg will be 0. If it doesn't them memory update will show this \_0.0_/
        ])

        if addr not in self.empty_addrs: self.empty_addrs.append(addr)
        if addr in self.full_addrs: self.full_addrs.remove(addr)

      self.empty_addrs.sort()
      del self.mem_addr[name]

      if name in self.full_temp_vars: self.full_temp_vars.remove(name)
      if name not in self.empty_temp_vars:
        self.empty_temp_vars.append(name)
        self.empty_temp_vars.sort()

  def get_var(self, name, regstart, recursion=1):
    data_length = len(self.mem_addr[name][0])

    for i in range(data_length):
      self.crawssembly.extend(self.construct_int(self.mem_addr[name][0][i], self.int_to_hex(recursion)))
      self.crawssembly.extend([
        f"io mem addr r{self.int_to_hex(recursion)}",
        f"io mem read r{self.int_to_hex(regstart + i)}"
      ]) # loads each piece of data from memory into regs from regstart -> regstart + data_length

    return self.mem_addr[name] # returns variable data

  # debug memory is VERY hardcoded, might be stinky but oh well Please send any complaints following the form of "make ts dynamic" to tritech.corebench@gmail.com
  def debug_memory(self, mode, expression):  # debug runs *top-down* to preserve the most register values (construct_int still runs though...)
    if mode not in ("mem", "var", "regs"): raise Exception(f"CRISP Error: Unknown debug type '{mode}' (Line {self.line_index + 1})")

    elif mode == "mem":
      if len(expression) != 2: raise Exception(f"CRISP Error: Bad memory debug values; expected start and end, not '{expression}' (Line {self.line_index + 1})")

      start = expression[0]
      end = expression[1]

      self.crawssembly.extend(self.construct_int(start, "ec"))
      self.crawssembly.extend(self.construct_int(end, "eb"))

      self.crawssembly.extend([
        "sav 58 rea",
        "sav 32 re9",
        "65000",
        "io mem addr rec",
        "io mem read r01",
        "io text hex rec",
        "io text char rea",
        "io text char re9",
        "io text hex r01",
        "io text newline rff",
        "sav rec r01",
        "cal add 1 r01",
        "sav r01 red",
        "sav r01 rec",
        "cal not reb reb",
        "cal add 1 r01",
        "cal add r01 red",
        "jml 65000",
        "rmv 65000"
      ])

    elif mode == "var":
      if len(expression) != 1: raise Exception(f"CRISP Error: '{expression}' is not a singluar variable (Line {self.line_index + 1})")
      if expression[0] not in list(self.mem_addr.keys()): raise Exception(f"CRISP Error: '{expression}' is not defined (Line {self.line_index + 1})")

      mem, type = self.mem_addr[expression[0]]

      self.crawssembly.extend(["sav 58 rec", "sav 32 reb"])

      for mem_addr in mem:
        self.crawssembly.extend(self.construct_int(mem_addr))
        self.crawssembly.extend([
          "io mem addr r01",
          "io mem read red",
          "io text hex r01",
          "io text char rec",
          "io text char reb",
          "io text hex red",
          "io text newline rff",
        ])

    elif mode == "regs":
      if len(expression) != 2: raise Exception(f"CRISP Error: Expected 'start end', not '{expression}' (Line {self.line_index + 1})")

      try:

        if "0x" in expression[0]: start = int(expression[0], 16)
        else: start = int(expression[0])

        if "0x" in expression[1]: end = int(expression[1], 16)
        else: end = int(expression[1])

      except: raise Exception(f"CRISP Error: Bad register values '{expression}' (Line {self.line_index + 1})")

      self.crawssembly.extend(["sav 58 red", "sav 32 rec"])

      for reg in range(start, end):
        self.crawssembly.extend(self.construct_int(reg))
        self.crawssembly.extend([
          "io text hex r01",
          "io text char red",
          "io text char rec",
          f"io text hex r{self.int_to_hex(reg)}",
          "io text newline rff",
       ])

# merges together strings to avoid "hello world" + "!" being made into ['"hello', 'world"', "+", '"!"'] and failing len=3 checks
  def split_expression(self, expression):
    inside_string = False

    for idx, token in enumerate(expression):
      if token.startswith('"'):
        inside_string = True

      if token in self.calc_symbols and not inside_string:
        left = expression[:idx]
        op = token
        right = expression[idx + 1:]

        return [
          self.join_value_tokens(left),
          op,
          self.join_value_tokens(right)
        ]

      if token.endswith('"'):
        inside_string = False

    if len(expression) == 1:
      return [self.join_value_tokens(expression)]

    raise Exception(f"CRISP Error: Could not split expression '{' '.join(expression)}'")

  def join_value_tokens(self, tokens):
    if not tokens:
      raise Exception("CRISP Error: Missing value in expression")

    text = " ".join(tokens)

    return text

  def solve_expression(self, expression, recursion=1, single_bypass=False): # solves varA X varB
    is_array = False
    is_int = False
    is_str = False

    temp_names = []

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

    inside_string = False

    realA = None
    realB = None

    typeA = None
    typeB = None

    symbol_idx = None

    print(f"Old expression: {expression}")

    expression = self.split_expression(expression)

    print(f"New expression: {expression}")

    if len(expression) != 3 and not single_bypass: raise Exception(f"CRISP Error: Expression lengths must be of the form 'value' 'operation' 'value' (Line {self.line_index + 1})")

    if expression[0].startswith('"') and expression[0].endswith('"'): typeA = "str"
    if expression[-1].startswith('"') and expression[-1].endswith('"'): typeB = "str"

    if expression[-1].startswith("[") and expression[-1].endswith("]"): typeA = "array"
    if expression[-1].startswith("[") and expression[-1].endswith("]"): typeB = "array"

    definedA = False
    definedB = False

    if expression[0] in self.mem_addr: definedA = True
    if expression[-1] in self.mem_addr: definedB = True

    print(expression)

    print(f"definedA: {definedA}, definedB: {definedB}")

    if definedA: typeA = self.mem_addr[expression[0]][1]
    if definedB: typeB = self.mem_addr[expression[-1]][1]

    print(f"typeA: {typeA}, typeB: {typeB}")

    if typeA == "str":
      is_str = True
      is_array = False
      is_int = False
    elif typeA == "int":
      is_str = False
      is_array = False
      is_int = True
    elif typeA == "array":
      is_str = False
      is_array = True
      is_int = False

    for idx, token in enumerate(expression):
      if self.is_integer(token): is_int = True
      elif "[" in token or "]" in token: is_array = True
      elif '"' in token: is_str = True

      if typeA != typeB: raise Exception(f"CRISP Error: Can not use '{expression[1]}' with '{typeA}' type and '{typeB}' type (Line {self.line_index + 1})")

      mem = None

      print(f"idx: {idx}, token: {token}")

      if is_str and is_int: raise Exception(f"CRISP Error: Cannot solve expression with int and string types (Line {self.line_index + 1})")
      if is_str and is_array: raise Exception(f"CRISP Error: Cannot solve expression with array and string types (Line {self.line_index + 1})")
      if is_int and is_array: raise Exception(f"CRISP Error: Cannot solve expression with array and integer types (Line {self.line_index + 1})")

      if token not in self.calc_symbols: # i.e. the token is a value
        if '"' in token: token = token.replace('"', "")
        if "[" in token: token = token.replace("[", "")
        if "]" in token: token = token.replace("]", "")

        if is_int: temp_type = "int"
        elif is_array: temp_type = "array"
        else: temp_type = "str"

        self.assign_var(self.empty_temp_vars[0], token, recursion + 1, force_type=temp_type)
        self.full_temp_vars.append(self.empty_temp_vars[0])

        try:
          name = self.empty_temp_vars.pop(0)
          mem = self.mem_addr[name][0] # values if expression logic falls through

          temp_names.append(name)
        except: pass

        if idx == 0 and not definedA: nameA = self.empty_temp_vars[0]
        elif idx == 0: nameA = token

        elif idx == 2 and not definedB: nameB = self.empty_temp_vars[0]
        elif idx == 2: nameB = token

        else: raise Exception(f"CRISP Error: Cannot solve expression of the form {' '.join(expression)} (Line {self.line_index + 1})")

        try:
          if idx == 0: memA = self.mem_addr[nameA][0]
          if idx == 2: memB = self.mem_addr[nameB][0]
        except: print(f"temp variable '{nameA}' or '{nameB}' has been deleted")

      else:

        # this bit is actually stinky, the compiler constructs the second value before actually looking at it properly
        # because calculation step needs the next value as the method of calculation depends on said value

        print("iterating over symbol character")

        if definedB:
          memB = self.mem_addr[expression[2]][0]
          nameB = expression[2]
        else:
          if expression[2].startswith('"') and expression[2].endswith('"'): typeB = "str"
          elif expression[2].startswith("[") and expression[2].endswith("]"): typeB = "array"
          else: typeB = "int"

          self.assign_var(self.empty_temp_vars[0], expression[2], recursion + 1, force_type=typeB)

          nameB = self.empty_temp_vars.pop(0)
          memB = self.mem_addr[nameB][0]

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
        ]) # <----------- ts only works for -2^31 < n < 2^31 - 1, UNDERFLOW WILL HAPPEN!

        self.free_temp(temp_names)

        return "int", [mem], None

      if token == "+":
        if is_array: raise Exception("Can't do ts yet")

        elif is_int:
          mem = self.empty_addrs.pop(0)
          self.full_addrs.append(mem)

          self.get_var(nameA, 3, recursion + 1)
          self.get_var(nameB, 3 + len(memA), recursion + 1)

          self.crawssembly.extend(self.construct_int(mem))
          self.crawssembly.extend([
            "io mem addr r01",
            "cal add r03 r04",
            "io mem write r01"
          ]) # <----------- ts only works for -2^31 < n < 2^31 - 1, OVERFLOW WILL HAPPEN!

          self.free_temp(temp_names)

          return "int", [mem], None

        elif is_str:
          print(f"string concat reached, memA: {memA}, memB: {memB}")

          name = self.empty_temp_vars.pop(0)
          self.full_temp_vars.append(name)

          mem = []

          mem_ref = memA + memB

          for _ in range(len(memA) + len(memB)):
            mem.append(self.empty_addrs.pop(0))
            self.full_addrs.append(mem[0])

          print(f"memA: {memA}, memB: {memB}, mem: {mem}, name: {name}, mem_ref: {mem_ref}")

          self.mem_addr.update({name : [mem, "str"]})

          self.free_temp(temp_names)

          self.debug_memory("mem", ["0", "2"])

          for mem_idx, addr in enumerate(mem_ref):
            self.crawssembly.extend(self.construct_int(addr))
            self.crawssembly.extend(["io mem addr r01", "io mem read r03"])
            self.crawssembly.extend(self.construct_int(mem[mem_idx]))
            self.crawssembly.extend(["io mem addr r01", "io mem write r03"])

          return "str", mem, [name]

    if is_str: type = "str"
    elif is_int: type = "int"
    elif is_array: type = "array"
    else: raise Exception(f"CRISP Compile Error: Could not assign a type to expression '{' '.join(expression)}' output (Compiling Line {self.line_index})")

    return type, [mem], [name]

  def compile_lines(self):
    for line_index, line in enumerate(self.crisp):

      if line in ("", None, "\n"): continue

      if line.strip() == self.crawssembly_str: self.craw_mode = not self.craw_mode; continue

      if self.craw_mode:
        self.crawssembly.append(line.strip())
        continue

      self.line_index = line_index

      self.line_indent = len(line) - len(line.lstrip(" "))

      print(f"\nline: {line}memory: {self.mem_addr}")

      try: tokens = line.split(self.comment_str)[0].split(" ")
      except: continue

      if len(tokens) == 0: continue
      else: command = tokens[0]

      for i, token in enumerate(tokens):
        if i == (len(tokens) - 1) and token.endswith("\n"): tokens[-1] = token[:-1]

      if self.line_indent != 0: tokens = tokens[self.line_indent:]


# -----------------------------------   A D D   C O M M A N D S   H E R E   -----------------------------------

      if command in ("", " ", None, "\n"): raise Exception(f"CRISP Error: Malformed line start. (Line {self.line_index + 1})")

      elif command == "let":
        var = tokens[1]
        value = tokens[tokens.index("=") + 1:]
        self.assign_var(var, value, recursion=1)

      elif command == "print":
        expression = tokens[1:]

        (data_type, mem, temp_names) = self.solve_expression(expression)
        self.print_value(data_type, mem)

        if temp_names: self.free_temp(temp_names)

      elif command == "println":
        expression = tokens[1:]

        (data_type, mem, temp_names) = self.solve_expression(expression)

        print(f"mem: {mem}, temp_names: {temp_names}")

        self.debug_memory("mem", ["0", "7"])

        self.print_value(data_type, mem)

        self.crawssembly.append("io text newline rff")

        if temp_names: self.free_temp(temp_names)

      elif command == "debug":
        self.debug_memory(tokens[1], tokens[2:])








      else: raise Exception(f"CRISP Error: Unknown command '{command}' (Line {self.line_index + 1})")



def main():
  with open("code.crisp", "r") as f: crisp = f.readlines()
  compiler = Compiler(crisp)
  compiler.compile_lines()
  with open("assembly.craw", "w") as f:
    for line in compiler.crawssembly: f.write(line + "\n")

if __name__ == "__main__": main()









