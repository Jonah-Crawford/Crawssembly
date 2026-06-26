// c_backend.rs

fn reg_to_index(reg: &str) -> u8 {
    match reg {
        "ref" => 239,
        _ if reg.starts_with('r') => reg[1..].parse::<u8>().unwrap(),
        _ => panic!("Unknown register: {}", reg),
    }
}

fn safe_label(label: &str) -> String {
    format!("craw_label_{}", label.replace("-", "_"))
}

fn find_matching_rmv(lines: &[String], start: usize, id: &str) -> Option<usize> {
    for (i, raw_line) in lines.iter().enumerate().skip(start + 1) {
        let line = raw_line.split(";").next().unwrap_or("").trim();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 && parts[0] == "rmv" && parts[1] == id {
            return Some(i);
        }
    }

    None
}

pub fn emit_c(lines: Vec<String>) -> String {
    let mut out = String::new();
    let line_count = lines.len();

    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stdio.h>\n\n");
    out.push_str("int32_t r[256];\n");
    out.push_str("int32_t craw_fgo_target = -1;\n\n");
    out.push_str("int main(void) {\n");

    for (i, raw_line) in lines.iter().enumerate() {
        out.push_str(&format!("craw_line_{i}:\n"));

        let line = raw_line.split(';').next().unwrap_or("").trim();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "sav" => {
                let value = parts[1];
                let reg = reg_to_index(parts[2]);

                out.push_str(&format!("  r[{reg}] = {value};\n"));
            }

            "cal" => {
                if parts[1] == "add" {
                    let value = parts[2];
                    let reg = reg_to_index(parts[3]);

                    out.push_str(&format!("  r[{reg}] += {value};\n"));
                }
            }

            "io" => {
                if parts[1] == "text" && parts[2] == "char" {
                    let reg = reg_to_index(parts[3]);
                    out.push_str(&format!("  putchar(r[{reg}]);\n"));
                }

                if parts[1] == "text" && parts[2] == "int" {
                    let reg = reg_to_index(parts[3]);
                    out.push_str(&format!("  printf(\"%d\", r[{reg}]);\n"));
                }

                if parts[1] == "text" && parts[2] == "hex" {
                    let reg = reg_to_index(parts[3]);
                    out.push_str(&format!("  printf(\"%08X\", (uint32_t)r[{reg}]);\n"));
                }

                if parts[1] == "text" && parts[2] == "newline" {
                    out.push_str("  putchar('\\n');\n");
                }
            }

            "jmp" => {
                out.push_str(&format!("  goto {};\n", safe_label(parts[1])));
            }

            "jmz" => {
                out.push_str(&format!(
                    "  if (r[1] == 0) goto {};\n",
                    safe_label(parts[1])
                ));
            }

            "jmg" => {
                out.push_str(&format!("  if (r[1] > 0) goto {};\n", safe_label(parts[1])));
            }

            "jml" => {
                out.push_str(&format!("  if (r[1] < 0) goto {};\n", safe_label(parts[1])));
            }

            "ifz" => {
                let Some(target) = find_matching_rmv(&lines, i, parts[1]) else {
                    panic!("No matching rmv {} found for ifz on line {}", parts[1], i);
                };

                out.push_str(&format!("  if (r[1] != 0) goto craw_line_{target};\n"));
            }

            "ifg" => {
                let Some(target) = find_matching_rmv(&lines, i, parts[1]) else {
                    panic!("No matching rmv {} found for ifg on line {}", parts[1], i);
                };

                out.push_str(&format!("  if (r[1] <= 0) goto craw_line_{target};\n"));
            }

            "ifl" => {
                let Some(target) = find_matching_rmv(&lines, i, parts[1]) else {
                    panic!("No matching rmv {} found for ifl on line {}", parts[1], i);
                };

                out.push_str(&format!("  if (r[1] >= 0) goto craw_line_{target};\n"));
            }

            "rmv" => {}

            "fgo" => {
                let target = parts[1];

                if target == "0" {
                    out.push_str("  craw_fgo_target = r[1];\n");
                } else {
                    out.push_str(&format!("  craw_fgo_target = {};\n", target));
                }

                out.push_str("  goto craw_fgo_dispatch;\n");
            }

            "stp" => {
                out.push_str("  return 0;\n");
            }

            other => {
                if parts.len() == 1 {
                    out.push_str(&format!("{}:\n", safe_label(other)));
                }
            }
        }
    }

    out.push_str("  return 0;\n\n");
    out.push_str("craw_fgo_dispatch:\n");
    out.push_str("  switch (craw_fgo_target) {\n");

    for i in 0..line_count {
        out.push_str(&format!("    case {i}: goto craw_line_{i};\n"));
    }

    out.push_str("    default: return 1;\n");
    out.push_str("  }\n");
    out.push_str("}\n");

    out
}
