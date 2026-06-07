CRAWSEMBLY DOCUMENTATION (FULL TEXT VERSION)

OVERVIEW

Crawssembly is a assembly-like programming language designed around a custom virtual CPU.
It provides direct control over registers, memory, and hardware-like I/O.

Programs consist of 21-bit instructions that operate on registers, perform calculations, control flow, and interact with devices.

CPU ARCHITECTURE

DATA TYPE

All values are stored as signed 32-bit integers using standard signed binary (two’s complement). This allows both positive and negative numbers.

REGISTERS

There are 256 registers, addressed in hexadecimal:

r00 to rff (0x00 to 0xFF)

SPECIAL REGISTERS

r00 = CPU input (read-only)
r01 = accumulator (used for all calculation results and condition checks)
ree = I/O exit code register
ref = ASCII output register
rff = output / discard register (safe to write when output is not needed)

CONSTANT REGISTERS

Registers rf0 to rfe store predefined constants, scaled by 10^8 for decimal precision:

rf0 = pi			314159265
rf1 = e				271828182
rf2 = log10(2)			30102999
rf3 = log10(3)			47712125
rf4 = log10(5)			69897000
rf5 = ln(2)			69314718
rf6 = ln(3)			109861228
rf7 = ln(5)			160943791
rf8 = sqrt(2)			141421356
rf9 = sqrt(3)			173205080
rfa = sqrt(5)			223606797
rfb = cbrt(2)			125992105
rfc = cbrt(3)			144224957
rfd = cbrt(5)			170997594
rfe = 2^31 - 1 (Not scaled)	2147483647

INSTRUCTION FORMAT

Each instruction is 21 bits:

Core (2 bits) | Mode (3 bits) | Value A (8 bits) | Value B (8 bits)

CORE TYPES

00 = register → register
01 = immediate → register
10 = register calculation
11 = immediate calculation

IMMEDIATE VALUES

Immediate values range from -128 to +127. Larger values must be constructed using multiple instructions.

BASIC INSTRUCTIONS

SAVE (MOVE DATA)

sav rA rB ; copies value from rA into rB
sav A rB ; stores immediate value A into rB

Example:

sav 5 r02 ; r02 = 5
sav r02 r03 ; r03 = 5

CALCULATIONS

All calculations store their result in r01.

Operations:

000 = NOT
001 = AND
010 = OR
011 = XOR
100 = SHL (Logical Shift Left)
101 = SHR (Logical Shift Right)
110 = SAR (Arithmetic Shift Right)
111 = ADD (a + b)

Examples:

cal add r02 r03 ; r01 = r02 + r03
cal xor r04 r05 ; r01 = r04 XOR r05

Immediate example:

cal add 5 r02 ; r01 = 5 + r02

immediate values always come first

CONTROL FLOW

LABELS

Labels mark positions in the program:

1
2
100
etc...

These are stored internally during execution.

JUMPS

jmp L ; jump to label
jmz L ; jump if r01 == 0
jmg L ; jump if r01 > 0
jml L ; jump if r01 < 0

Example:

cal add r02 r03
jmz END

INLINE CONDITIONS

ifz L ; continue if r01 == 0
ifg L ; continue if r01 > 0
ifl L ; continue if r01 < 0

These control whether the next instruction executes.

LABEL REMOVAL

rmv L ; removes label from memory

FORCE JUMP (fgo)

fgo N ; jump directly to line number N

This ignores labels and jumps based on raw line position.

Special case:

fgo 0 ; jumps to the line stored in r01

Example:

sav 10 r01
fgo 0 ; jumps to line 10

PROGRAM CONTROL

nop ; does nothing for one cycle
stp ; stops program execution
inp ; forces next input value

Example:

nop
stp

STORAGE SYSTEM

STORE PROGRAM

str <line_start> <block>

Stores instructions into a persistent block.

Example:

str 1 5

Stores lines 1 to current into block 5.

Special:

str 1 0 ; block number comes from r01

RUN PROGRAM

run <block>

Executes a stored block.

Example:

run 5

Special:

run 0 ; block number comes from r01

INPUT / OUTPUT (io)

General format:

io <device> <command> <register>

Result codes are stored in ree

EXIT CODES

0 = OK
1 = invalid device
2 = invalid command
3 = bad value
4 = screen out of bounds
5 = unavailable

DEVICES

TEXT DEVICE

Commands:

char ; print ASCII from register
int ; print integer
newline ; print newline

Example:

sav 65 r02
io text char r02 ; prints 'A'

TIME DEVICE

seconds → register
millis → register (Last 31 bits to avoid 2038-problem issues)

Example:

io time seconds r02

SCREEN DEVICE

Commands:

x ; set X coordinate
y ; set Y coordinate
pixel ; draw pixel
clear ; clear screen
present/render ; render screen
red/r ; set red value
green/g ; set green value
blue/b ; set blue value
erase ; remove pixel, removes bottom if input is negative, else remove top
erasecell ; remove entire screen cell

Example:

sav 10 r02
sav 5 r03

io screen x r02
io screen y r03

sav 127 r04
io screen r r04

io screen pixel rff
io screen present rff

KEYBOARD DEVICE

io keyboard poll REG

Returns:

0 = no key
27 = escape
-1 to -4 = arrow keys (up, down, left, right order)
other = ASCII

MOUSE DEVICE

io mouse x r02
io mouse y r03
io mouse buttons r04

Buttons are bitfields:

bit 0 = left
bit 1 = right
bit 2 = middle
bit 8–11 = scroll

SPEAKER DEVICE

Reserved (not yet implemented)

EXECUTION MODEL

JUMP-THEN-EXECUTE (JTE)

A common pattern:

Jump over code to register labels
Return later to execute it

This allows function-like behaviour.

Example:

sav 5 r01
1
fgo 0
sav 5 r02
stp

jmp 1

OUTPUT SHORTCUT

Writing to ref prints ASCII:

sav 65 ref ; prints 'A'

EXAMPLE PROGRAMS

EXAMPLE 1: ADD TWO NUMBERS

sav 5 r02
sav 10 r03

cal add r02 r03

sav r01 ref ; print result as ASCII (may not be readable)
stp

EXAMPLE 2: LOOP COUNTDOWN

sav 5 r02

1
cal add -1 r02
sav r01 r02

sav r02 ref
io text newline rff

jmg 1

stp

EXAMPLE 3: SIMPLE PIXEL DRAW

sav 10 r02
sav 10 r03

io screen x r02
io screen y r03

sav 127 r04
io screen r r04

io screen pixel rff
io screen present rff

stp

EXAMPLE 4: KEYBOARD ECHO

1
io keyboard poll r02

jmz 1

io text char r02
jmp 1
