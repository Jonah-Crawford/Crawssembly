# Crawssembly Standard Library (CSL)

The **CSL** is a small group of common functions included to
make it easier to write longer programs. Use of `executestd module/function.craw`
is used to access the program. `execute` is used for **user-created** programs,
and is differentiated as `execute` uses the path relative to the program being executed, while `executestd` works *anywhere* on your machine.

You can save the contents of `std/` to your root folder by running `craw install-std`. This makes `executestd` work from anywhere.

Each program takes inputs starting at `r02`, and emits outputs also starting at `r02`. Each program also uses labels starting at 60000.

Example

```
sav 12 r02                      ; 12 is the first argument
sav 5 r03                       ; 5 is the second argument

executestd math/multiply.craw   ; r02 = r02 * r03

; at this point, registers r01, r03, and r04 are now empty (see the base program list)

io text int r02                 ; outputs 12*5 (i.e. 60)
```

## Base Programs

This is the list of programs available, along with Inputs, Outputs, and **Scope**.

> The **scope** of a function is the range of data the program touches. It's important to note what values get mutated so your program works correctly. This includes label values.

- reset_regs: I: r02 O: N/A S: Variable | 60000 (`executestd reset_regs.craw`)
  - Resets all the register values up to the code in `r02`
- compare:
  - equal: I: r02-r03 O: r02 S: r01-r02 | 60000 (`executestd compare/equal.craw`)
    - outputs `1` if `r02` = `r03`, else `0`
  - greater: I: r02-r03 O: r02 S: r01-r02 | 60000 (`executestd compare/greater.craw`)
    - outputs `1` if `r02` > `r03`, else `0`
  - greater_equal: I: r02-r03 O: r02 S: r01-r02 | 60000 (`executestd compare/greater_equal.craw`)
    - outputs `1` if `r02` >= `r03`, else `0`
  - less: I: r02-r03 O: r02 S: r01-r02 | 60000 (`executestd compare/less.craw`)
    - outputs `1` if `r02` < `r03`, else `0`
  - less_equal: I: r02-r03 O: r02 S: r01-r02 | 60000 (`executestd compare/less_equal.craw`)
    - outputs `1` if `r02` <= `r03`, else `0`
  - not_equal: I: r02-r03 O: r02 S: r01-r02 | 60000 (`executestd compare/not_equal.craw`)
    - outputs `1` if `r02` != `r03`, else `0`
- math:
  - abs: I: r02 O: r02 S: r01-r02 | 60000 (`executestd math/abs.craw`)
    - outputs the positive-only number
  - divide: I: r02-r03 O: r02-r03 S: r01-r04 | 60000 (`executestd math/divide.craw`)
    - outputs `r02` / `r03` into `r02`, and the remainder into `r03`
  - modulo: I: r02-r03 O: r02 S: r01-r03 | 60000 (`executestd math/modulo.craw`)
    - the same as divide, but only outputs the remainder
  - multiply: I: r02-r03 O: r02 S: r01-r04 | 60000 (`executestd math/multiply.craw`)
    - outputs `r02` * `r03`
  - negate: I: r02 O: r02 S: r01-r02 (`executestd math/negate.craw`)
    - negates the value inside `r02` (i.e. `r02` = -`r02`)
  - power: I: r02-r03 O: r02 S: r01-r05 | 60000-60001 (`executestd math/power.craw`)
    - outputs `r02` ^ `r03`
  - random: I: N/A O: r02 S: r01-r03 | 60000 (`executestd math/random.craw`)
    - outputs a random number using the timestamp in milliseconds as the seed
  - sign: I: r02 O: r02 S: r01-r02 | 60000 (`execute math/sign.craw`)
    - outputs `-1` if the input is negative, `1` if positive, and `0` if 0
