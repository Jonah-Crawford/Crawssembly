# Crawssembly

A custom 21-bit instruction set architecture, assembler, and virtual machine written primarily in Rust.

## Features
- 256 registers
- 21-bit instruction format
- 32-bit system structure
- DWORD-sized memory
- Custom ALU
- Disk IO
- Memory operations
- Graphics experiments
- Audio experiments
- Branching and labels
- Signed integer support

## Components
- Assembler
- VM
- Experimental tooling

## Example
```
sav 14 r02              ; jte line number
1                       ; draw logic
sav r02 r01             ; ready jte line number
fgo 0                   ; jte fgo
io screen x r10         ; set x
io screen y r11         ; set y
io screen r r12         ; set r
io screen g r13         ; set g
io screen b r14         ; set b
io screen pixel rff     ; set pixel
io screen present rff   ; show screen
fgo 121                 ; return to main code (hardcoded)

sav 21 r02              ; jte line number
2                       ; image error code
sav r02 r01             ; ready jte line number
fgo 0                   ; jte fgo
sav 58 ref              ; stdout :
sav 40 ref              ; stdout (
stp                     ; end program

sav 30 r02              ; jte line number
3                       ; y jump code
sav r02 r01             ; ready jte line number
fgo 0                   ; jte fgo
sav 0 r10               ; reset x coordinate
cal add -1 r11          ; deccrement y coordinate
sav r01 r11             ; update y coordinate
fgo 127                 ; return to main code (hardcoded)

io screen clear rff     ; clears the screen
sav 19 r02              ; label 2 jte line number
cal add -66 r00         ; test for 66 for B
jmg 2                   ; code > 66, error
jml 2                   ; code < 66, error
inp                     ; get next byte
cal add -77 r00         ; test for 77 for M
jmg 2                   ; code > 77, error
jml 2                   ; code < 77, error
inp                     ; byte 0x02
inp                     ; byte 0x03
inp                     ; byte 0x04
inp                     ; byte 0x05
inp                     ; byte 0x06
inp                     ; byte 0x07
inp                     ; byte 0x08
inp                     ; byte 0x09
inp                     ; byte 0x0A
sav 8 r02               ; load 8 shifts
sav r00 r06             ; save 0x0A
inp                     ; byte 0x0B
sav r00 r05             ; save 0x0B
inp                     ; byte 0x0C
sav r00 r04             ; save 0x0C
inp                     ; byte 0x0D
sav r00 r03             ; save 0x0D
cal shl r03 r02         ; 0x0D << 8
cal add r04 r01         ; 0x0D | 0x0C
cal shl r01 r02         ; (0x0D | 0x0C) << 8
cal add r05 r01         ; 0x0D | 0x0C | 0x0B
cal shl r01 r02         ; (0x0D | 0x0C | 0x0B) << 8
cal add r06 r01         ; 0x0D | 0x0C | 0x0B | 0x0A (offset)
sav r01 r07             ; save offset
inp                     ; byte 0x0E
inp                     ; byte 0x0F
inp                     ; byte 0x10
inp                     ; byte 0x11
inp                     ; byte 0x12
sav r00 r06             ; save 0x12
inp                     ; byte 0x13
sav r00 r05             ; save 0x13
inp                     ; byte 0x14
sav r00 r04             ; save 0x14
inp                     ; byte 0x15
sav r00 r03             ; save 0x15
cal shl r03 r02         ; 0x15 << 8
cal add r04 r01         ; 0x15 | 0x14
cal shl r01 r02         ; (0x15 | 0x14) << 8
cal add r05 r01         ; 0x15 | 0x14 | 0x13
cal shl r01 r02         ; (0x15 | 0x14 | 0x13) << 8
cal add r06 r01         ; 0x15 | 0x14 | 0x13 | 0x12 (width)
cal not r01 r01         ; NOT width
cal add 1 r01           ; negate width
sav r01 r08             ; save width
inp                     ; byte 0x16
sav r00 r06             ; save 0x16
inp                     ; byte 0x17
sav r00 r05             ; save 0x17
inp                     ; byte 0x18
sav r00 r04             ; save 0x18
inp                     ; byte 0x19
sav r00 r03             ; save 0x19
cal shl r03 r02         ; 0x19 << 8
cal add r04 r01         ; 0x19 | 0x18
cal shl r01 r02         ; (0x19 | 0x18) << 8
cal add r05 r01         ; 0x19 | 0x18 | 0x17
cal shl r01 r02         ; (0x19 | 0x18 | 0x17) << 8
cal add r06 r01         ; 0x19 | 0x18 | 0x17 | 0x16 (height)
sav r01 r09             ; save height
sav r09 r11             ; start y at bottom (BMP are backwards)
cal add -25 r07         ; remove passed bytes
sav r01 r07             ; update byte offset
5                       ; byte offset loop
inp                     ; move to next byte
cal add -1 r07          ; decrement offset
sav r01 r07             ; update offset
jmg 5                   ; move again if not done
6                       ; main data extract loop
cal add -128 r00        ; normalise blue byte
sav r01 r14             ; save byte into blue reg
inp                     ; move to next byte
cal add -128 r00        ; normalise green byte
sav r01 r13             ; save byte into green reg
inp                     ; move to next byte
cal add -128 r00        ; normalise red byte
sav r01 r12             ; save byte into red reg
inp                     ; move to next byte
inp                     ; skip alpha
sav 5 r02               ; label 1 jte line number
jmp 1                   ; draw pixel
cal add 1 r10           ; increment x
sav r01 r10             ; update x coordinate
cal add r08 r10         ; test for x end
jml 6                   ; x < end, do next pixel
sav 26 r02              ; label 3 jte line number
jmz 3                   ; decrement y
sav r11 r01             ; ready y coordinate for end test
jmg 6                   ; y > 0, continue
7                       ; exit loop
io keyboard poll r01    ; get last key
cal add -27 r01         ; test for code 27 for Esc
jml 7                   ; code < 27, continue
jmg 7                   ; code > 27, continue
stp                     ; code = 27, end
```

## Philosophy
Built as an exploration of low-level computing systems, ISA design, and virtual machine architecture.
