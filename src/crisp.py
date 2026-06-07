##########  ##########  ##########  ##########  ##########
##          ##      ##      ##      ####        ##      ##
##          ##########      ##        ######    ##########
##          ##  ####        ##            ####  ##
##########  ##    ####  ##########  ##########  ##

# Craw Rapid Instruction Set Programming

# CRISP COMPILER (C) 2026 CRAW SYSTEMS Written By Jonah Crawford

from imm import i as construct

RAM_SIZE = 65536 # 2^16

class Compiler:
  def __init__(self, crisp):
    self.crisp = crisp
    self.crawssembly = []

    self.mem_addr = {}

    self.empty_addrs = [i for i in range(RAM_SIZE)] # heavy on memory here, most programs won't touch anything above 0x0000FFFF
    self.full_addrs = []

    self.empty_temp_vars = [f"_CRISP_TEMP_{i}" for i in range(256)]
    self.full_temp_vars = []

    self.comment_str = "/?/"
    self.crawssembly_str = "/c/"

    self.calc_symbols = ["+", "-"]
    self.bool_symbols = ["==", "!=", ">", "<", ">=", "<="]
    self.bin_symbols = ["¬¬", "&&", "||", "^^"]

    self.line_index = 0

    self.craw_mode = False

    self.reserved_keywords = [
      "true",
      "false",
      "if",
      "endif",
    ]

    self.empty_labels = [i for i in range(2 ** 15, 2**16)]

    # Each entry is [label, temp_names_to_free_after_endif]
    self.branch_labels = []

  def int_to_hex(self, value): return f"{value:x}".zfill(2)

  def is_integer(self, text):
    try: int(text); return True
    except (ValueError, TypeError): return False

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

  def new_temp(self, data_type, mem):
    name = self.empty_temp_vars.pop(0)
    self.full_temp_vars.append(name)
    self.mem_addr[name] = [mem, data_type]
    return name

  def alloc_mem(self):
    mem = self.empty_addrs.pop(0)
    self.full_addrs.append(mem)
    return mem

  def write_reg_to_mem(self, reg, mem):
    self.crawssembly.extend(self.construct_int(mem, 1))
    self.crawssembly.extend([
      "io mem addr r01",
      f"io mem write r{reg}"
    ])

  def read_mem_to_reg(self, mem, reg):
    self.crawssembly.extend(self.construct_int(mem, 1))
    self.crawssembly.extend([
      "io mem addr r01",
      f"io mem read r{reg}"
    ])

  def emit_jump_then_execute_bool(self, jump_ops, source_reg="03", out_reg="05"):
    label = self.empty_labels.pop(0)
    start_line = len(self.crawssembly) + 1

    set_skip = ["sav 0 r02"]
    set_body = ["sav 0 r02"]

    while True:
      a = len(set_skip)

      body_line = start_line + a + 3
      set_body_new = self.construct_int(body_line, "02")
      b = len(set_body_new)

      end_line = start_line + a + b + len(jump_ops) + 8
      skip_line = start_line + a + 6
      set_skip_new = self.construct_int(skip_line, "02")

      if set_skip_new == set_skip and set_body_new == set_body:
        break

      set_skip = set_skip_new
      set_body = set_body_new

    self.crawssembly.extend(set_skip)

    self.crawssembly.extend([
      f"{label}",
      "sav r02 r01",
      "fgo 0",
      f"sav 1 r{out_reg}",
      f"fgo {end_line}",
      f"rmv {label}",
      f"sav 0 r{out_reg}",
    ])

    self.crawssembly.extend(set_body)

    self.crawssembly.append(f"sav r{source_reg} r01")

    for jump_op in jump_ops:
      self.crawssembly.append(f"{jump_op} {label}")

    self.crawssembly.append(f"rmv {label}")

  def emit_bool_from_diff(self, op, diff_reg="03", out_reg="05"):
    # r[diff_reg] = left - right
    # r[out_reg] becomes 0 or 1

    if op == "==":
      jump_ops = ["jmz"]

    elif op == "!=":
      jump_ops = ["jmg", "jml"]

    elif op == ">":
      jump_ops = ["jmg"]

    elif op == "<":
      jump_ops = ["jml"]

    elif op == ">=":
      jump_ops = ["jmz", "jmg"]

    elif op == "<=":
      jump_ops = ["jmz", "jml"]

    else:
      raise Exception(f"CRISP Error: Unknown comparison '{op}' (Line {self.line_index + 1})")

    self.emit_jump_then_execute_bool(jump_ops, diff_reg, out_reg)

  def solve_comparison(self, left, op, right):
    left_type = self.infer_type(left)
    right_type = self.infer_type(right)

    if left_type != right_type: raise Exception(f"CRISP Error: Can not compare '{left_type}' and '{right_type}' (Line {self.line_index + 1})")

    if left_type != "int": raise Exception(f"CRISP Error: Only int comparisons work currently (Line {self.line_index + 1})")

    ltype, lmem, ltemps = self.solve_expression_node([left])
    rtype, rmem, rtemps = self.solve_expression_node([right])

    out_mem = self.alloc_mem()

    self.read_mem_to_reg(lmem[0], "03")
    self.read_mem_to_reg(rmem[0], "04")

    self.crawssembly.extend([
      "sav r04 r01",
      "cal not r01 r01",
      "cal add 1 r01",
      "cal add r01 r03"
    ])

    self.emit_bool_from_diff(op, "03", "05")
    self.write_reg_to_mem("05", out_mem)

    temps = []
    if ltemps: temps.extend(ltemps)
    if rtemps: temps.extend(rtemps)

    name = self.new_temp("boolean", [out_mem])
    temps.append(name)

    return "boolean", [out_mem], temps

  def solve_boolean_not(self, token):
    data_type, mem, temp_names = self.solve_expression_node([token])

    if data_type != "boolean": raise Exception(f"CRISP Error: '¬¬' requires a boolean value (Line {self.line_index + 1})")

    out_mem = self.alloc_mem()

    self.read_mem_to_reg(mem[0], "03")

    self.emit_jump_then_execute_bool(["jmz"], "03", "05")

    self.write_reg_to_mem("05", out_mem)

    temps = []
    if temp_names: temps.extend(temp_names)

    name = self.new_temp("boolean", [out_mem])
    temps.append(name)

    return "boolean", [out_mem], temps

  def solve_boolean_binary(self, left, op, right):
    ltype, lmem, ltemps = self.solve_expression_node([left])
    rtype, rmem, rtemps = self.solve_expression_node([right])

    if ltype != "boolean" or rtype != "boolean": raise Exception(f"CRISP Error: '{op}' requires booleans (Line {self.line_index + 1})")

    out_mem = self.alloc_mem()

    self.read_mem_to_reg(lmem[0], "03")
    self.read_mem_to_reg(rmem[0], "04")

    if op == "&&": self.crawssembly.append("cal and r04 r03")

    elif op == "||": self.crawssembly.append("cal or r04 r03")

    elif op == "^^": self.crawssembly.append("cal xor r04 r03")

    else: raise Exception(f"CRISP Error: Unknown boolean operator '{op}' (Line {self.line_index + 1})")

    self.write_reg_to_mem("03", out_mem)

    temps = []
    if ltemps: temps.extend(ltemps)
    if rtemps: temps.extend(rtemps)

    name = self.new_temp("boolean", [out_mem])
    temps.append(name)

    return "boolean", [out_mem], temps

  def make_if_statement(self, result_type, mem, temp_names):
    if result_type != "boolean": raise Exception(f"CRISP Error: 'if' requires a boolean expression, not '{result_type}' (Line {self.line_index + 1})")

    label = self.empty_labels.pop(0)

    self.read_mem_to_reg(mem[0], "01")
    self.crawssembly.append(f"ifg {label}")

    self.branch_labels.append([label, temp_names or []])

  def assign_var(self, name, value, recursion=1, force_type=None):
    string = False
    array = False
    boolean = False

    if isinstance(value, list) and len(value) == 1 and isinstance(value[0], tuple): value = value[0]

    if isinstance(value, tuple):
      value_type, value_addrs, temp_vars = value

      if name in self.reserved_keywords: raise Exception(f"CRISP Error: Can not set variable to reserved keyword '{name}' (Line {self.line_index + 1})")

      if name in self.mem_addr:
        self.crawssembly.append("sav 0 r02")

        for addr in self.mem_addr[name][0]:
          self.crawssembly.extend(self.construct_int(addr))
          self.crawssembly.extend([
            "io mem addr r01",
            "io mem write r02"
          ])

          if addr not in self.empty_addrs: self.empty_addrs.append(addr)
          if addr in self.full_addrs: self.full_addrs.remove(addr)

        self.empty_addrs.sort()
        del self.mem_addr[name]

      self.mem_addr[name] = [value_addrs, value_type]

      for temp in temp_vars or []:
        if temp in self.mem_addr: del self.mem_addr[temp]
        if temp in self.full_temp_vars: self.full_temp_vars.remove(temp)
        if temp not in self.empty_temp_vars: self.empty_temp_vars.append(temp)

      self.empty_temp_vars.sort()
      return

    if isinstance(value, list):
      if value and isinstance(value[0], str) and value[0].startswith('"') and value[-1].endswith('"'): value = " ".join(value)

    if name in self.reserved_keywords: raise Exception(f"CRISP Error: Can not set variable to reserved keyword '{name}' (Line {self.line_index + 1})")

    print(f"assign_var()  name: {name}, value: {value}, force_type: {force_type}")

    if isinstance(value, str) and value in self.mem_addr:
      mem_ref = self.mem_addr[value][0]
      data_type = self.mem_addr[value][1]

      mem = []

      for _ in mem_ref:
        addr = self.empty_addrs.pop(0)
        mem.append(addr)
        self.full_addrs.append(addr)

      for idx, addr in enumerate(mem_ref):
        self.crawssembly.extend(self.construct_int(addr))
        self.crawssembly.extend([
          "io mem addr r01",
          "io mem read r03"
        ])

        self.crawssembly.extend(self.construct_int(mem[idx]))
        self.crawssembly.extend([
          "io mem addr r01",
          "io mem write r03"
        ])

      self.mem_addr[name] = [mem, data_type]
      return

    if name in self.mem_addr:
      print(f"assign_var()  name {name} defined")
      self.crawssembly.append("sav 0 r02")

      for addr in self.mem_addr[name][0]:
        self.crawssembly.extend(self.construct_int(addr))
        self.crawssembly.extend([
          "io mem addr r01",
          "io mem write r02"
        ])

        if addr not in self.empty_addrs: self.empty_addrs.append(addr)
        if addr in self.full_addrs: self.full_addrs.remove(addr)

      self.empty_addrs.sort()
      del self.mem_addr[name]

    if isinstance(value, str) and value.startswith('"') and value.endswith('"'): string = True
    elif isinstance(value, str) and value.startswith("[") and value.endswith("]"): array = True
    elif isinstance(value, str) and value in ("true", "false"): boolean = True

    if force_type:
      if force_type == "str":
        string = True
        array = False
        boolean = False
      elif force_type == "int":
        string = False
        array = False
        boolean = False
      elif force_type == "array":
        string = False
        array = True
        boolean = False
      elif force_type == "boolean":
        string = False
        array = False
        boolean = True
      else: raise Exception(f"CRISP Compile Error: Unknown forced variable type '{force_type}' (Compiling Line {self.line_index})")

    if string: final_value = ""
    elif array: final_value = []

    mem = self.empty_addrs[0]
    needs_solving = False

    if not isinstance(value, list): value = [value]

    print(f"assign_var()  name: {name}, value: {value}, string: {string}, array: {array}, boolean: {boolean}")

    if boolean:
      if len(value) != 1: raise Exception(f"CRISP Error: Bad boolean value '{value}' (Line {self.line_index + 1})")

      bool_value = value[0]

      if bool_value == "true": number = 1
      elif bool_value == "false": number = 0
      elif self.is_integer(bool_value): number = 1 if int(bool_value) != 0 else 0
      else: raise Exception(f"CRISP Error: Bad boolean value '{bool_value}' (Line {self.line_index + 1})")

      self.mem_addr[name] = [[mem], "boolean"]

      self.crawssembly.extend(self.construct_int(number, 1))
      self.crawssembly.extend(self.construct_int(mem, 2))
      self.crawssembly.extend([
        "io mem addr r02",
        "io mem write r01"
      ])

      self.empty_addrs.remove(mem)
      self.full_addrs.append(mem)
      return

    for idx, v in enumerate(value):
      if string:
        v = str(v)

        if v.startswith('"') and v.endswith('"'): final_value += v[1:-1]
        elif v.startswith('"'): final_value += v[1:]
        elif v.endswith('"'): final_value += v[:-1]
        else: final_value += v

        if idx != len(value) - 1: final_value += " "

      elif array: pass

      else:
        if not self.is_integer(v):
          needs_solving = True
          break

        self.mem_addr[name] = [[mem], "int"]

        self.crawssembly.extend(self.construct_int(v, 1))
        self.crawssembly.extend(self.construct_int(mem, 2))
        self.crawssembly.extend([
          "io mem addr r02",
          "io mem write r01"
        ])

        self.empty_addrs.remove(mem)
        self.full_addrs.append(mem)

        return

    if string:
      self.crawssembly.append("sav 8 r03")

      print(f"assign_var() final_value: {final_value}")

      used_addr = []

      for i, c in enumerate(final_value):
        char_value = ord(c)

        if (i % 4) == 0:
          word_addr = self.empty_addrs.pop(0)
          self.full_addrs.append(word_addr)
          used_addr.append(word_addr)

          self.crawssembly.extend(self.construct_int(word_addr))
          self.crawssembly.extend([
            "io mem addr r01",
            f"sav {char_value} r01"
          ])

          if i != len(final_value) - 1: self.crawssembly.append("cal shl r01 r03")

        elif (i % 4) == 3:
          self.crawssembly.extend([
            f"cal add {char_value} r01",
            "io mem write r01"
          ])

        else:
          self.crawssembly.append(f"cal add {char_value} r01")

          if i != len(final_value) - 1: self.crawssembly.append("cal shl r01 r03")

      if len(final_value) % 4 != 0:
        remaining = 4 - (len(final_value) % 4)

        for _ in range(remaining): self.crawssembly.append("cal shl r01 r03")

        self.crawssembly.append("io mem write r01")

      self.mem_addr[name] = [used_addr, "str"]
      return

    if needs_solving:
      data_type, mem, temp_names = self.solve_expression(value)

      self.assign_var(name, (data_type, mem, temp_names), recursion + 1)

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

    elif data_type in ("int", "boolean"):
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
          "io mem write ree" # ree should be 0 because of good io, address should never fail unless you're doing something insane
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
      ])

    return self.mem_addr[name]

  def debug_memory(self, mode, expression):
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

      except:
        raise Exception(f"CRISP Error: Bad register values '{expression}' (Line {self.line_index + 1})")

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

  def split_expression(self, expression):
    parts = []
    current = []
    inside_string = False
    operators = self.calc_symbols + self.bool_symbols + ["&&", "||", "^^"]

    for token in expression:
      if token == "": continue

      if token.startswith('"'): inside_string = True

      if token in operators and not inside_string:
        if not current: raise Exception(f"CRISP Error: Missing value before '{token}' (Line {self.line_index + 1})")

        parts.append(" ".join(current))
        parts.append(token)
        current = []
        continue

      if token == "¬¬" and not inside_string:
        if current:
          parts.append(" ".join(current))
          current = []

        parts.append(token)
        continue

      current.append(token)

      if token.endswith('"'): inside_string = False

    if inside_string: raise Exception(f"CRISP Error: Unterminated string in expression (Line {self.line_index + 1})")

    if current: parts.append(" ".join(current))

    return parts

  def infer_type(self, token):
    if token in self.mem_addr: return self.mem_addr[token][1]
    if token.startswith('"') and token.endswith('"'): return "str"
    if token.startswith("[") and token.endswith("]"): return "array"
    if token in ("true", "false"): return "boolean"
    if self.is_integer(token): return "int"
    raise Exception(f"CRISP Error: '{token}' is not defined (Line {self.line_index + 1})")

  def collapse_unary_nots(self, expression):
    expression = list(expression)
    temp_names = []

    while "¬¬" in expression:
      index = len(expression) - 1 - expression[::-1].index("¬¬")

      if index + 1 >= len(expression): raise Exception(f"CRISP Error: Missing value after '¬¬' (Line {self.line_index + 1})")

      operand = expression[index + 1]

      if operand in self.calc_symbols or operand in self.bool_symbols or operand in ("&&", "||", "^^", "¬¬"): raise Exception(f"CRISP Error: Bad value after '¬¬' (Line {self.line_index + 1})")

      _, _, temps = self.solve_boolean_not(operand)

      if not temps: raise Exception(f"CRISP Error: Internal boolean temp failure (Line {self.line_index + 1})")

      result_name = temps[-1]
      temp_names.extend(temps)

      expression = expression[:index] + [result_name] + expression[index + 2:]

    return expression, temp_names

  def solve_expression_node(self, expression, recursion=1, single_bypass=False):
    temp_names = []

    print(f"Old expression: {expression}")

    if isinstance(expression, str): expression = [expression]

    if not single_bypass:
      if len(expression) == 0: raise Exception(f"CRISP Error: Empty expressions can't be parsed. (Line {self.line_index + 1})")

      if len(expression) == 1:
        token = expression[0]

        if token in self.mem_addr: return self.mem_addr[token][1], self.mem_addr[token][0], None

        if not token.startswith('"') and not token.endswith('"') and "[" not in token and "]" not in token and not self.is_integer(token) and token not in ("true", "false"): raise Exception(f"CRISP Error: {token} is not defined (Line {self.line_index + 1})")

        if '"' in token or self.is_integer(token) or token in ("true", "false"):
          return self.solve_expression_node(expression, recursion, single_bypass=True)

        if "[" in token and "]" in token: raise Exception("Still can't do arrays yet fool")

        raise Exception(f"CRISP Error: I don't know how to parse '{token}' (Line {self.line_index + 1})")

    expression = self.split_expression(expression)

    if len(expression) == 2 and expression[0] == "¬¬": return self.solve_boolean_not(expression[1])

    if len(expression) == 1:
      token = expression[0]

      if token in self.mem_addr: return self.mem_addr[token][1], self.mem_addr[token][0], None

      data_type = self.infer_type(token)

      clean = token
      if clean.startswith('"') and clean.endswith('"'): clean = clean[1:-1]

      temp_name = self.empty_temp_vars.pop(0)
      self.full_temp_vars.append(temp_name)

      self.assign_var(temp_name, clean, recursion + 1, force_type=data_type)

      return data_type, self.mem_addr[temp_name][0], [temp_name]

    print(f"New expression: {expression}")

    if len(expression) != 3: raise Exception(f"CRISP Error: Expression lengths must be of the form 'value' 'operation' 'value' (Line {self.line_index + 1})")

    left, op, right = expression

    if op in self.bool_symbols: return self.solve_comparison(left, op, right)

    if op in ("&&", "||", "^^"): return self.solve_boolean_binary(left, op, right)

    typeA = self.infer_type(left)
    typeB = self.infer_type(right)

    if typeA != typeB: raise Exception(f"CRISP Error: Can not use '{op}' with '{typeA}' type and '{typeB}' type (Line {self.line_index + 1})")

    def materialise(token):
      if token in self.mem_addr:
        name = token
        mem = self.mem_addr[token][0]
        return name, mem

      clean = token

      if clean.startswith('"') and clean.endswith('"'): clean = clean[1:-1]
      elif clean.startswith("[") and clean.endswith("]"): clean = clean[1:-1]

      temp_name = self.empty_temp_vars.pop(0)
      self.full_temp_vars.append(temp_name)

      self.assign_var(temp_name, clean, recursion + 1, force_type=typeA)

      temp_names.append(temp_name)

      mem = self.mem_addr[temp_name][0]
      return temp_name, mem

    nameA, memA = materialise(left)
    nameB, memB = materialise(right)

    if op == "-":
      if typeA != "int": raise Exception(f"CRISP Error: Cannot subtract a {typeA} type (Line {self.line_index + 1})")

      self.get_var(nameA, 3, recursion + 1)
      self.get_var(nameB, 4, recursion + 1)

      mem = self.empty_addrs.pop(0)
      self.full_addrs.append(mem)

      self.crawssembly.extend(self.construct_int(mem))
      self.crawssembly.extend([
        "io mem addr r01",
        "cal not r04 r04",
        "sav 1 r01",
        "cal add r01 r04",
        "cal add r04 r03",
        "io mem write r03"
      ])

      self.free_temp(temp_names)

      name = self.empty_temp_vars.pop(0)
      self.full_temp_vars.append(name)
      self.mem_addr[name] = [[mem], "int"]

      return "int", [mem], [name]

    if op == "+":
      if typeA == "boolean": raise Exception(f"Can not use '+' on a boolean type (Line {self.line_index + 1})")
      if typeA == "array": raise Exception("Can't do ts yet")

      if typeA == "int":
        mem = self.empty_addrs.pop(0)
        self.full_addrs.append(mem)

        self.get_var(nameA, 3, recursion + 1)
        self.get_var(nameB, 3 + len(memA), recursion + 1)

        self.crawssembly.extend(self.construct_int(mem))
        self.crawssembly.extend([
          "io mem addr r01",
          "cal add r03 r04",
          "io mem write r04"
        ])

        self.free_temp(temp_names)

        name = self.empty_temp_vars.pop(0)
        self.full_temp_vars.append(name)
        self.mem_addr[name] = [[mem], "int"]

        return "int", [mem], [name]

      if typeA == "str":
        name = self.empty_temp_vars.pop(0)
        self.full_temp_vars.append(name)

        mem_ref = memA + memB
        mem = []

        for _ in range(len(mem_ref)):
          addr = self.empty_addrs.pop(0)
          mem.append(addr)
          self.full_addrs.append(addr)

        self.mem_addr.update({name: [mem, "str"]})

        for mem_idx, addr in enumerate(mem_ref):
          self.crawssembly.extend(self.construct_int(addr))
          self.crawssembly.extend([
            "io mem addr r01",
            "io mem read r03"
          ])

          self.crawssembly.extend(self.construct_int(mem[mem_idx]))
          self.crawssembly.extend([
            "io mem addr r01",
            "io mem write r03"
          ])

        self.free_temp(temp_names)

        return "str", mem, [name]

    raise Exception(f"CRISP Error: Unknown operator '{op}' (Line {self.line_index + 1})")

  def solve_expression(self, expression):
    print(f"old expression: {expression}")
    expression = self.split_expression(expression)
    expression, unary_temps = self.collapse_unary_nots(expression)
    print(f"new expression: {expression}")

    if len(expression) == 1:
      data_type, mem, temps = self.solve_expression_node(expression)
      all_temps = []
      all_temps.extend(unary_temps)
      if temps: all_temps.extend(temps)
      return data_type, mem, all_temps

    if len(expression) % 2 == 0:
      raise Exception(
        f"CRISP Error: Expected an odd-lengthed expression. "
        f"Expected pattern: value operation value operation value. "
        f"(Line {self.line_index + 1})"
      )

    temp_names = []
    temp_names.extend(unary_temps)

    node = expression[:3]
    var_type, result_mem, temps = self.solve_expression_node(node)

    if temps: temp_names.extend(temps)

    result_name = temps[-1] if temps else None

    idx = 3

    while idx < len(expression):
      op = expression[idx]
      rhs = expression[idx + 1]

      if result_name is None:
        result_name = self.empty_temp_vars.pop(0)
        self.full_temp_vars.append(result_name)
        self.mem_addr[result_name] = [result_mem, var_type]
        temp_names.append(result_name)

      node = [result_name, op, rhs]

      var_type, result_mem, temps = self.solve_expression_node(node)

      if temps:
        temp_names.extend(temps)
        result_name = temps[-1]

      idx += 2

    return var_type, result_mem, temp_names

  def compile_lines(self):
    for line_index, line in enumerate(self.crisp):
      line = line.strip()

      if line in ("", None, "\n", self.comment_str): continue

      if line == self.crawssembly_str:
        self.craw_mode = not self.craw_mode
        continue

      if self.craw_mode:
        self.crawssembly.append(line.strip())
        continue

      self.line_index = line_index

      self.line_indent = len(line) - len(line.lstrip(" "))

      print(f"\nline: {line}\nmemory: {self.mem_addr}")

      try: tokens = line.split(self.comment_str)[0].split(" ")
      except: continue

      if len(tokens) == 0: continue
      else: command = tokens[0]

      for i, token in enumerate(tokens):
        if i == (len(tokens) - 1) and token.endswith("\n"): tokens[-1] = token[:-1]

      if self.line_indent != 0: tokens = tokens[self.line_indent:]

# -----------------------------------   C O M M A N D S   -----------------------------------

      if command in ("", " ", None, "\n"):
        raise Exception(f"CRISP Error: Malformed line start. (Line {self.line_index + 1})")

      elif command == "let":
        var = tokens[1]
        value = self.solve_expression(tokens[tokens.index("=") + 1:])
        self.assign_var(var, value, recursion=1)

      elif command == "print":
        expression = tokens[1:]

        data_type, mem, temp_names = self.solve_expression(expression)
        self.print_value(data_type, mem)

        if temp_names: self.free_temp(temp_names)

      elif command == "println":
        expression = tokens[1:]

        data_type, mem, temp_names = self.solve_expression(expression)

        self.print_value(data_type, mem)
        self.crawssembly.append("io text newline rff")

        if temp_names: self.free_temp(temp_names)

      elif command == "debug":
        self.debug_memory(tokens[1], tokens[2:])

      elif command == "unbind":
        if len(tokens) == 1: raise Exception(f"CRISP Error: No variable(s) specified for 'unbind' (Line {self.line_index + 1})")

        for var in tokens[1:]: _ = self.mem_addr.pop(var, None)

      elif command == "delete":
        if len(tokens) == 1: raise Exception(f"CRISP Error: No variable(s) specified for 'delete' (Line {self.line_index + 1})")

        for var in tokens[1:]:
          if var in self.mem_addr: self.free_temp([var])

      elif command == "stop": self.crawssembly.append("stp")

      elif command == "if":
        result, mem, temp_names = self.solve_expression(tokens[1:])
        self.make_if_statement(result, mem, temp_names)

      elif command == "endif":
        if not self.branch_labels: raise Exception(f"CRISP Error: 'endif' without matching 'if' (Line {self.line_index + 1})")

        label, temp_names = self.branch_labels.pop(-1)
        self.crawssembly.append(f"rmv {label}")

        if temp_names: self.free_temp(temp_names)







      else: raise Exception(f"CRISP Error: Unknown command '{command}' (Line {self.line_index + 1})")

def main():
  with open("code.crisp", "r") as f: crisp = f.readlines()
  compiler = Compiler(crisp)
  compiler.compile_lines()
  with open("assembly.craw", "w") as f:
    for line in compiler.crawssembly: f.write(line + "\n")

if __name__ == "__main__": main()
