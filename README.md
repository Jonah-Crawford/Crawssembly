# Crawssembly: A beginner entry-point to Assembly Languages

![Crawssembly Banner](https://www.dropbox.com/scl/fi/e4fhcba8zkgw2youvai8s/Crawssembly.png?rlkey=x1xojmcn29z9joxnmpb09iums&st=lxansr9a&raw=1)

## Why does Crawssembly exist?

**Crawssembly** is an educational assembly-like language designed to teach how computers work from the ground up.

Traditional assembly languages such as x86 and ARM are powerful, but often overwhelming for beginners.
Crawssembly removes much of the complexity while preserving the core ideas:
- Registers
- Memory
- Arithmetic
- Program Flow
- Input and Output

The goal is to help you understand what high-level languages are really doing behind the scenes.

> Don't worry if some of the concepts in this guide seem unfamiliar at first. Computers are surprisingly simple once you break them into smaller pieces, and we'll build up the ideas one step at a time.

> Because Crawssembly assumes near-zero knowledge, there might be concepts you already know. Feel free to skip over them if you are familar with anything shown!

## How to install Crawssembly

> Present versions have only been tested on Linux systems. Errors are likely if you are not using a Linux OS.

To install Crawssembly, run these commands

```
git clone https://github.com/Jonah-Crawford/Crawssembly.git
cd Crawssembly
cargo install --path .
```

To run a Crawssembly file (These are `.craw` files), simply run `craw <file.craw>` and swap `<file.craw>` with the name of the Crawssembly file you wish to edit.

### What will I learn?

By the end of this guide you will understand:

✓ Binary and hexadecimal

✓ Registers

✓ Memory

✓ Variables

✓ Arithmetic

✓ Program flow

✓ Input and output

✓ How high-level languages work underneath

✓ The fundamentals of CPU architecture

## Why not just learn real assembly?

Real assembly is designed to be run, not explained. Crawssembly does not aim to be a replacement for real assembly languages, rather a teaching tool.
Just like riding a bike, you practise using the slower stabiliser wheels before riding off into the sunset by yourself.

| Feature | x86 | ARM | Crawssembly |
| ------- | --- | --- | ----------- |
| Beginner Friendly | X | X | ✓ |
| Small Instruction Set | X | ✓ | ✓ |
| Direct Hardware Concepts | ✓ | ✓ | ✓ |
| Easy Toolchain | X | ✓ | ✓ |
| Educational Focus | X | X | ✓ |

## Assembly; What's the deal?

Computers are an amazing product of the information age. Billions of computers are running around the world all doing important, or not-so-important, tasks. But how do the computers know what to do?
Programming languages are used to tell the computer how to work, but the issue is that there are so many to choose from. There's around **8 000** different languages to speak to the device!
How can computers keep up with the different syntax, styles, and methods? The secret? All languages basically boil down to a single language, Machine Code.

**Machine code** is the raw binary that the computer executes. **Assembly languages** are human-readable representations of this machine code.
In many assembly languages, including crawssembly, a single instruction usually corresponds closely to a machine code instruction, compared to a 'higher' language where a line could equal upwards of thousands of lines.

Learning assembly is a great way to internalise how computers work, as by learning assembly you learn how a computer 'thinks'. 

## Binary

Getting your head around binary (also known as Base-2) is perhaps the most important skill when working with low-level computing. However, the need for binary thinking has diminished with the massive rise of high-level languages like Python, Java, and Swift. A programmer can create amazing tools, games, and projects without ever actually needing to know what binary is.
For those who don't know what binary is, this guide go through the basics, but for more information, Wikipedia is always a great place to start!

### What is Binary?

Binary, simply, is just another way to use numbers. We humans use the Base-10 (A.K.A Decimal) system, meaning that we have 10 separate symbols for counting (i.e. 0, 1, 2, 3, 4, 5, 6, 7, 8, and 9). You can make any number using enough of these digits. Computers don't have the luxury of 10 digits though.

At their core, computers are a bunch of transistors, very small switches that can only be in two states; On or Off. This is because two-state switches are very reliable and can change state quickly.

Think about a car's gear stick, it would be easier to use if there were only 2 gears rather than the 6 or more in reality.

### How does binary get used?
It's basically the same as normal counting. In Base-10, when you reach '9' and you want to go up again, you have to use the next number in the place value, that being 10. Every time you reach the end, you need to reset the count and use the next place value.

9 + 1 becomes 10

99 + 1 becomes 100

999 + 1 becomes 1000

etc...

It's clear that place value works in 10's, the number '8' could represent 8 1's, 8 10's 8 100's, 8 1000's etc... depending on where it is in the number.

| Place Value | 10 000 | 1 000 | 100 | 10 | 1 |
| ----------- | ------ | ----- | --- | -- | - |
|   Number    |    8   |   3   |  0  |  7 | 2 |

The example above shows how the number 83072 is equal to

8 * 10 000 +

3 * 1 000 +

0 * 100 + 

7 * 10 +

2 * 1

For binary, only **2 digits** are used. These are 0 for 'Off', and 1 for 'On'. Because only 2 symbols are used, place value works based on 2, not 10.

So a '1' could represent 1 1's, 1 2's, 1 4's, 1 8's, 1 16's, etc... depending on where it is in the number.

| Place Value | 16 | 8 | 4 | 2 | 1 |
| ----------- | -- | - | - | - | - |
|   Binary    |  1 | 0 | 1 | 1 | 0 |

We can see that the binary number 10110 is the same as

1 * 16 +

0 * 8 +

1 * 4 +

1 * 2 +

0 * 1

Which, when calcualted, equals 22. So 10110 in binary is the exact same as 22 in Base-10. Every number that can be made in Base-10, can be made in Base-2. There are no gaps.

### Other Number Systems

Another counting system regually used is **Hexadecimal**, also called  Base-16. Because we only have 10 numbers, the letters A-F are also used. A = 10, B = 11, C = 12, etc...

| Place Value | 256 | 16 | 1 |
| ----------- | --- | -- | - |
| Hexadecimal |  B  |  3 | F |

B3F =

11 * 256 + 

3 * 16 +

15 * 1

= 2623

Hexadecimal is used because it neatly condenses binary numbers, a 4 digit binary number can be compressed into a single hexadecimal digit.

A counting example is provided below to show what counting in different bases looks like:

| Base-10 | Binary | Hexadecimal |
| ------- | ------ | ----------- |
|    0    |    0   |      0      |
|    1    |    1   |      1      |
|    2    |   10   |      2      |
|    3    |   11   |      3      |
|    4    |   100  |      4      |
|    5    |   101  |      5      |
|    6    |   110  |      6      |
|    7    |   111  |      7      |
|    8    |  1000  |      8      |
|    9    |  1001  |      9      |
|    10   |  1010  |      A      |
|    11   |  1011  |      B      |
|    12   |  1100  |      C      |
|    13   |  1101  |      D      |
|    14   |  1110  |      E      |
|    15   |  1111  |      F      |

High level languages tend to spoil the user with many programs, functions, and data types. The computer doesn't see it that way. From the silicon's perspective, everything is binary numbers. A string such as "Hello World!" is actually a long binary number, the computer doesn't know what 'H' is, only it's binary representation.

Every photo, song, text, and piece of code is actually just a long list of 1's and 0's!

### How can I tell the difference?

Because the number '100' could be in weird and wonderful base, it's neccessary to show what number system is being used if Base-10 has moved aside for a moment.

If the number is in binary, it's usually prefaced with '0b', so if you saw '0b100' then you can be sure that's binary, equal to 4 in Base-10. If you see '0x', that's hexadecimal. So '0x100' would be 256 in Base-10.

## The Basics

> Real assembly languages can interact directly with hardware and operating systems. Mistakes can sometimes crash software or corrupt data. Crawssembly runs inside a virtual machine, allowing you to experiment safely while still learning the same fundamental concepts.

If you've used high-level programming languages before, you'll know that telling a computer how to work is quite different than telling another person what to do.

For example, asking your friend to get a drink from the kitchen is a simple matter, but getting a computer to do this requires you to define a drink, where the ktichen is, the precise steps needed to move to the drink's location, how to pick up the drink, the movements needed to take the drink back to you while not dropping or crushing the cup, etc...

The computer will do what it's told exactly as written to the letter. If you told a computer "Make me a sandwich", you might find the computer think of ways to turn your hair into cheese, to literally convert your body into that of a sandwich. It is an important skill to learn that, when dealing with computers, you must be purely logical. The first truth about computers:  "Ambiguity kills the machine, precision keeps it running."

To use Crawssembly, these practises are no different. 

### Instructions

Every line of Crawssembly is a dedicated instruction. Every line is executed one after another in the exact order the program is written in. The computer reads each line, converts the instruction into a binary number, and executes the command. This is the **Fetch-Decode-Execute** loop, the fundemental process of the computer's Central Processing Unit (CPU)

### Registers

The most basic instruction is storing data, which is stored as numbers, which is stored in cells. Each storage cell, which is called a **register**, holds a value, much like a variable in a higher language.

Crawssembly provides 256 registers for quick storage, most of which can be used to store data. Here is an example of how to save data to one of these registers:

```
sav 10 r01 ; this saves '10' to register 1
```

This example has 3 parts:
- `sav`
- `10`
- `r01`
- `;`

`sav` is the **save** command. It 'Saves A Value' to a register.

`10` is the value that we are saving. We call this value a **literal**, since we have explicitly told Crawssembly what value to save.

`r01` is a register. Every register has a code like this, with the register number in **Hexadecimal**. Registers are indexed **starting at 0**, so the first register is actually `r00`, because the codes start at 0. This *0 indexing* is commonplace in programming, and a usual stumbling block for beginners.

`;` is a **comment**, anything after it doesn't get run. It's the best way to talk about and annotate the program to help explain it's function.

Register codes include:

- `r01` is regsiter 1, the *2nd* regsiter available
- `r0a` is register 10, the *11th* register available
- `r10` is register 16, the *17th* register available

As there are 256 registers available in Crawssembly, the codes range from `r00` to `rff`, since the register at 0xff is the *256th* register available

So the example instruction `sav 10 r01` saves the literal value `10` into register 1, which is the *2nd* register available.

Most registers are ready and raring to hold your numbers. But some registers do other functions. The 3 most important ones are `r00`, `ref`, and `rff`. These are the *1st*, *240th*, and *256th* registers respectively.

- Register 0x00: The first register is **read-only**. This means that you can look at what's inside the register, but you can't save a value to it. This is because `r00` is used for input data, such as a file or a list of numbers.
- Register 0xEF: This register is the **ASCII** register. Any value that is saved to this register is converted to a letter and is shown to the terminal screen. Visit [this page](https://www.asciitable.com/) to see what ASCII numbers relate to what letter.
- Register 0xFF: The last register is **write-only**. This means that you can save a value to the register, but you can't look at what's inside. This is because `rff` is used for the computer output, like the result of a maths problem the computer has solved.

In addition, registers from `rf0` to `rfe` already contain values. These represent constants such as pi, Euler's number, some roots, and certain logarithms. There is more on these later on.

### Activity: Outputs

Because `ref` can be used to output characters to the screen, see if you can use this beginner input/output instruction to show any text you desire.

```
sav 72 ref	; 72 = 'H'
sav 101 ref	; 101 = 'e'
sav 108 ref	; 108 = 'l'
sav 108 ref	; 108 = 'l'
sav 111 ref	; 111 = 'o'
```

This program shows "Hello". Try to add to this program, using [this page](https://www.asciitable.com/) as a reference, to get "Hello World!" to be shown.

### More about 'literals'

Literals in Crawssembly can be positive or negative, however they can only be between -128 and 127.

Why these values? Because, of course, binary.

Like other assembly languages, every possible instruction is encoded as a binary number. In Crawssembly's case, each instruction is 21 numbers, or **bits**.
Because each line has this fixed size, literal values can only be so high. The literal in a `sav` command is given 8 bits, also known as a **byte**.

More details about how Crawssembly encodes data will be explained in later sections.

### More about `sav`

One of the best things about computers is one that often goes unnoticed; dynamicity. A vast amount of tasks would be nigh on impossible if the machines of the modern world were to become entirely hard-coded.
If you hardcoded everything, then there would be little point for computing languages, logical abstraction, and computers as a whole. If you know the result, why build a machine to express the program?

Crawssembly is no exception, dynamically storing values is as easy as removing the literal value in-place for another register

Example:

```
sav 10 r01	; saves 10 to register 1
sav r01 r02	; saves value of register 1 to register 2
```

See how the literal value in the second line was replaced by `r02`? This program saves the literal value `10` into `r01`, and the next instruction saves whatever value is in `r01` into the register `r02`.
The computer has no clue what `r01` actually contains, just that it should copy whatever may be inside `r01` into `r02`. This reveals another key truth about computers; "The machine is blind."

## Maths

Any good programmer must have a good mathematical brain, computers are large heaps of silicon held together by electricity and maths. Just as a hardware engineer needs to know about the electron, the software programmer needs to know about **bitwise operations**

Because of the binary representation of values, they can be manipulated in many ways to acheive a result.

### Boolean Logic

A **boolean** is a fancy name for something that is either in one particular state or another particular state, very similar to the binary On or Off.
Because of a boolean's nature, you can use a type of maths called **bitwise operations**.

There are 4 core operations used:

- NOT
- AND
- OR
- SHIFT

From these, all of computing can be derived. Apart from SHIFT, these are all examples of **Logic Gates**

#### NOT

The **NOT** operator takes 1 input, and reverses it. *On* outputs *Off*, and *Off* outputs *On*.

| A | Out |
| - | --- |
| Off | On |
| On | Off |

You can imagine NOT to be like a button toggle, pressing changes the state and pressing again returns to the original state.

#### AND

The **AND** operator takes 2 inputs, and returns a single output.
If both inputs are *On*, then the output is *On*. Else AND outputs *Off*.

| A | B | Out |
| - | - | --- |
| Off | Off | Off |
| Off | On | Off |
| On | Off | Off |
| On | On | On |

You can image AND to be like a padlock that needs 2 keys to open.

#### OR

The **OR** operator is like AND, but it only needs 1 input to be *On* to also return *On*.

| A | B | Out |
| - | - | --- |
| Off | Off | Off |
| Off | On | On |
| On | Off | On |
| On | On | On |

You can imagine OR to be like a bulb connected to two light switches, flipping either one will turn on the light.

#### SHIFT

A **SHIFT** is a special type of operation, where it moves the bits to the left or right.

Example:

0b1010 shifted 1 unit to the left is 0b10100
0b1010 shifted 1 unit to the right is 0b101

SHIFT is split into 3 subcatagories:
- Logical Shift Left (SHL)
- Logical Shift Right (SHR)
- Arithmetic Shift Right (SAR)

Because the sign of numbers is stored in the last bit of the binary numbers, shifting to the right can change the sign chaotically. Arithmetic Shift Right preserves the sign of the number.

#### Additional Operations

Using a combination of ANDs, ORs, and NOTs, another important operation called **XOR** can be used.

XOR is very similar to OR, but it needs **exactly** 1 input to be *On* for the output to be *On*, else the output will be *Off*.

| A | B | Out |
| - | - | --- |
| Off | Off | Off |
| Off | On | On |
| On | Off | On |
| On | On | Off |

You can imagine the XOR gate as an odd-number detector, it only likes an odd number of *On* values.

By wiring up these operations, you can also add numbers together, this is denoted as ADD.

#### Use

Calculation using these operations uses the `cal` instruction. All `cal` results get saved to register 1 (`r01`).

Example:

```
sav 10 r01	; saves 10 into register 1
cal add 5 r01	; adds 5 to value in register 1
```

This example adds `10` to `5`, resulting in `r01` having a value of `15`

Because of the nature of commands, the first value can be a register or literal, but the second vaule **must** be a register value. `cal add 1 1` would not work. This is why in the example, `10` must be saved to a register.
More information will be given for why this is the case later on.

























*Crawssembly is a product of CRAW SYSTEMS (C) 2026*
