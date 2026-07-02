# Crawssembly: The beginner's gateway to low-level thinking.

![Crawssembly Banner](https://www.dropbox.com/scl/fi/e4fhcba8zkgw2youvai8s/Crawssembly.png?rlkey=x1xojmcn29z9joxnmpb09iums&st=lxansr9a&raw=1)

Project Lead: J.D. Crawford. [Help support Crawssembly and open-source learning!](https://buymeacoffee.com/jonah_crawford)

Check out Crawssembly Online with [this link!](https://crawssembly.ultimatecraw.xyz/)  
[Dedicated Documentation Website](http://docs-crawssembly.ultimatecraw.xyz)

![Status](https://img.shields.io/badge/Status-Under%20Development-blue)
![Version](https://img.shields.io/badge/Latest_Release-v1.1.0-blue)
![Rust](https://img.shields.io/badge/Rust-Implementation-orange)
![VM](https://img.shields.io/badge/Virtual%20Machine-Custom-blue)
![Beginner Friendly](https://img.shields.io/badge/Beginner-Friendly-brightgreen)

## Contents

- [Why does Crawssembly exist?](#why-does-crawssembly-exist)
- [Installation](#how-to-install-crawssembly)
- [Your First Program](#your-first-program)
- [Beginner Overview](#what-will-i-learn)
- [Detailed Guide](#detailed-guide)
- [Quick Reference](#quick-reference)
- [Legal](#legal)

## Why does Crawssembly exist?

**Crawssembly** is an educational assembly-like language designed to teach how computers work from the ground up.

Traditional assembly languages such as x86 and ARM are powerful, but often overwhelming for beginners.
Crawssembly removes much of the complexity while preserving the core ideas:
- Registers
- Memory
- Arithmetic
- Program Flow
- Input and Output

Crawssembly is not:

❌ A replacement for x86 or ARM  
❌ A better version of assembly  
❌ A production language

Crawssembly is:

✅ A teaching language  
✅ A CPU simulator  
✅ A bridge between lower-level thinking and higher-level programming

The goal is to help you understand what high-level languages are really doing behind the scenes.

> Don't worry if some of the concepts in this guide seem unfamiliar at first. Computers are surprisingly simple once you break them into smaller pieces, and we'll build up the ideas one step at a time.
>
> Because Crawssembly assumes near-zero knowledge, there might be concepts you already know. Feel free to skip over them if you are familiar with anything shown!

![Registers](https://img.shields.io/badge/Registers-256-blue)
![Instruction%20Width](https://img.shields.io/badge/Instruction-21--bit-orange)
![Learn CPUs](https://img.shields.io/badge/Learn-CPU%20Architecture-blueviolet)

## How to install Crawssembly

![Linux](https://img.shields.io/badge/Linux-Tested-green)
![Windows 11](https://img.shields.io/badge/Windows_11-Tested-green)
![macOS](https://img.shields.io/badge/macOS-Tested-green)

> CRAW SYSTEMS can't guarantee *every version* of your system will react the same way to Crawssembly, please make a [**Github Issue**](https://github.com/Jonah-Crawford/Crawssembly/issues/new) if something doesn't work on your machine.

If you don't have Rust installed, download it by running these commands:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

To install Crawssembly, run these commands:

```
git clone https://github.com/Jonah-Crawford/Crawssembly.git
cd Crawssembly
cargo install --path .
```

To run a Crawssembly file (These are `.craw` files), simply run `craw <file.craw>` and swap `<file.craw>` with the name of the Crawssembly file you wish to run.

Use `craw --help` to display helpful use information about the different ways to run Crawssembly files.

## Your First Program

![First Program](https://img.shields.io/badge/First_Program-Under_60_Seconds-green)

Create a file called `hello.craw` and write the following:

```
sav 72 ref
sav 105 ref
stp
```

Run this file using `craw hello.craw`

Expected Output

```
Hi
```

Congratulations, you are taking the first steps to learning Crawssembly!

## What will I learn?

By the end of this guide you will understand:

✅ Binary and hexadecimal  
✅ Registers  
✅ Memory  
✅ Arithmetic  
✅ Program flow  
✅ Input and output  
✅ How high-level languages work underneath  
✅ Key computing concepts  
✅ The fundamentals of CPU architecture  

The biggest thing you will learn is

**Programming Languages Are A Lie**

Because computers don't actually know what you're typing. At the hardware level, computers ultimately manipulate electrical states that we usually represent as On and Off. Crawssembly will help you learn why this is, and how people have turned a collection of switches into one of the most complex man-made objects in the universe.

## Why not just learn real assembly?

Real assembly is designed to be run, not explained. Crawssembly does not aim to be a replacement for real assembly languages, rather a teaching tool.
Just like riding a bike, you practise using the slower stabiliser wheels before riding off into the sunset by yourself.

| Feature | x86 | ARM | Crawssembly |
| ------- | --- | --- | ----------- |
| Beginner Friendly | ❌ | ❌ | ✅ |
| Small Instruction Set | ❌ | ✅ | ✅ |
| Direct Hardware Concepts | ✅ | ✅ | ✅ |
| Easy Toolchain | ❌ | ✅ | ✅ |
| Educational Focus | ❌ | ❌ | ✅ |

## About This Document

This guide is made to help you learn the basics of Crawssembly, you can read it in any order, skip over any parts you don't like, or read the entire thing before writing a single piece of code.

> Most ideas build upon each other, so it's smart, if you're skipping around, to take a look at some important bits you may have missed that link to a later topic. (e.g. *bitfields* require you knowing about *bit masking*)

## Detailed Guide

<details>
<summary><strong>Assembly; What's the Deal?</strong></summary>

## Assembly; What's the deal?

Computers are an amazing product of the information age. Billions of computers are running around the world all doing important, or not-so-important, tasks. But how do the computers know what to do?
Programming languages are used to tell the computer how to work, but the issue is that there are so many to choose from. There's around **8 000** different languages to speak to the device!
How can computers keep up with the different syntax, styles, and methods? The secret? All languages basically boil down to machine code.

**Machine code** is the raw binary that the computer executes. **Assembly languages** are human-readable representations of this machine code.
In many assembly languages, including crawssembly, a single instruction usually corresponds closely to a machine code instruction, compared to a 'higher' language where a line could equal upwards of thousands of lines.

Learning assembly is a great way to internalise how computers work, as by learning assembly you learn how a computer 'thinks'.

</details>

<details>
<summary><strong>How Crawssembly compares to higher languages</strong></summary>

## How Crawssembly compares to higher languages

Higher languages, like C++, Python, and Java, hide how the computer thinks behind layers of abstraction.

In Python, you might type `print("Hello World!")` but that doesn't show how the computer splits the function and the string, or the buffer memory storing a copy of "Hello World!", or how the text gets to the screen. Python hides the instructions, Crawssembly exposes them.

</details>

<details>
<summary><strong>Binary</strong></summary>

## Binary

Getting your head around binary (also known as Base-2) is perhaps the most important skill when working with low-level computing. However, the need for binary thinking has diminished with the massive rise of high-level languages like Python, Java, and Swift. A programmer can create amazing tools, games, and projects without ever actually needing to know what binary is.
For those who don't know what binary is, this guide goes through the basics, but for more information, Wikipedia is always a great place to start!

### What is Binary?

Binary, simply, is just another way to use numbers. Humans typically use the Base-10 (A.K.A Decimal) system, meaning that we have 10 separate symbols for counting (i.e. 0, 1, 2, 3, 4, 5, 6, 7, 8, and 9). You can make any number using enough of these digits. Computers don't have the luxury of 10 digits though.

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

Which, when calculated, equals 22. So 10110 in binary is the exact same as 22 in Base-10. Every number that can be made in Base-10, can be made in Base-2. There are no gaps.

### Two's complement

Binary numbers like talked about above can only store positive numbers. Inside computers though, negative numbers are used all the time. So how do we get numbers like -1 or -42?

Inside computers, and Crawssembly, the **sign** of the number is represented with the leftmost bit. Where `0` means the number is positive, and `1` means the number is negative. Because negative numbers work in reverse (i.e. `-1` is greater than `-42`, while `1` is smaller than `42`), negative binary numbers also work in reverse. This system is called **Two's Complement**.

Because Crawssembly stores all numbers as **32-bit** values, meaning that each number has 32 bits, `-1` is represented by a string of `1`s with length 32.

| Decimal Value | Binary |
| ------------- | ------ |
| 10 | 00000000 00000000 00000000 00001010 |
| 2 | 00000000 00000000 00000000 00000010 |
| 1 | 00000000 00000000 00000000 00000001 |
| 0 | 00000000 00000000 00000000 00000000 |
| -1 | 11111111 11111111 11111111 11111111 |
| -2 | 11111111 11111111 11111111 11111110 |
| -10 | 11111111 11111111 11111111 11110110 |

A way to convert a positive value is by flipping all the bit values, then adding 1. We will go into more detail about this when we talk about binary operations.

Another way to think about it is setting the last place value as a negative. Below is an example with Two's Complement being used on an *8-bit value*.

| Place Value | -128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
| ----------- | ---- | -- | -- | -- | - | - | - | - |
| Binary Number | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 0 |

The binary number here, `11001110` is equal to

1 * -128 +  
1 * 64 +  
0 * 32 +  
0 * 16 +  
1 * 8 +  
1 * 4 +  
1 * 2 +  
0 * 1 = -50

### Other Number Systems

Another counting system regularly used in computing is **Hexadecimal**, also called  Base-16. Because we only have 10 numbers, the letters A-F are also used. A = 10, B = 11, C = 12, etc...

| Place Value | 256 | 16 | 1 |
| ----------- | --- | -- | - |
| Hexadecimal |  B  |  3 | F |

B3F =  
11 * 256 +  
3 * 16 +  
15 * 1 = 2623

Hexadecimal is used because it neatly condenses binary numbers, every hexadecimal digit represents exactly four binary digits.

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

High level languages tend to spoil the user with many programs, functions, and data types. The computer doesn't see it that way. From the silicon's perspective, everything is binary numbers. A string such as "Hello World!" is actually a long binary number, the computer doesn't know what 'H' is, only its binary representation.

Every photo, song, text, and piece of code is actually just a long list of 1's and 0's!

### How can I tell the difference?

Because the number '100' could be in weird and wonderful base, it's necessary to show what number system is being used if Base-10 has moved aside for a moment.

If the number is in binary, it's usually prefaced with '0b', so if you saw '0b100' then you can be sure that's binary, equal to 4 in Base-10. If you see '0x', that's hexadecimal. So '0x100' would be 256 in Base-10.

</details>

<details>
<summary><strong>The Basics</strong></summary>

## The Basics

> Real assembly languages can interact directly with hardware and operating systems. Mistakes can sometimes crash software or corrupt data. Crawssembly runs inside a virtual machine, allowing you to experiment safely while still learning the same fundamental concepts.

If you've used high-level programming languages before, you'll know that telling a computer how to work is quite different than telling another person what to do.

For example, asking your friend to get a drink from the kitchen is a simple matter, but getting a computer to do this requires you to define a drink, where the kitchen is, the precise steps needed to move to the drink's location, how to pick up the drink, the movements needed to take the drink back to you while not dropping or crushing the cup, etc...

The computer will do what it's told exactly as written to the letter. If you told a computer "Make me a sandwich", you might find the computer think of ways to turn your hair into cheese, to literally convert your body into that of a sandwich. It is an important skill to learn that, when dealing with computers, you must be purely logical. The first truth about computers:

"Ambiguity kills the machine, precision keeps it running."

To use Crawssembly, these practices are no different.

> Crawssembly comes with a folder full of example programs, each documented. Take a look at them for ideas!
>
> Crawssembly runs entierly in the terminal. To quickly edit files, you might use the `nano` command. This opens up a file for quick editing. By running the command `craw install-nano`, you can use `nano` to see your `.craw` files in ful syntax colour! This makes programs easier to write and debug.

### Instructions

Every line of Crawssembly is a dedicated instruction. Every line is executed one after another in the exact order the program is written in. The computer reads each line, converts the instruction into a binary number, and executes the command. This is the **Fetch-Decode-Execute** loop, the fundamental process of the computer's Central Processing Unit (CPU)

The simplest instruction is `nop`. Typing this into a file and running doesn't do anything because this is the **no operation** instruction; it doesn't do anything. Every empty line compiles to a `nop` command when the `.craw` file is compiled.

### Registers

The most basic instruction is storing data, which is stored as numbers, which is stored in cells. Each storage cell, which is called a **register**, holds a value, much like a variable in a higher language.

Crawssembly provides 256 registers for quick storage, most of which can be used to store data. Here is an example of how to save data to one of these registers:

Example

```
sav 10 r01      ; this saves '10' to register 1
stp
```

This program has 5 parts:
- `sav`
- `10`
- `r01`
- `; this saves '10' to register 1`
- `stp`

`sav` is the **save** command. It 'Saves A Value' to a register.

`10` is the value that we are saving. We call this value a **literal**, since we have explicitly told Crawssembly what value to save.

`r01` is a register. Every register has a code like this, with the register number in **Hexadecimal**. Registers are indexed **starting at 0**, so the first register is actually `r00`, because the codes start at 0. This *0 indexing* is commonplace in programming, and a usual stumbling block for beginners.

`;` is a **comment character**, anything after the `;` doesn't get run. It's the best way to talk about and annotate the program to help explain an instruction's function.

`stp` is a new instruction, as it occupies a new line in the file. `stp` forces the end of the program, but if the end of the file is reached it's not necessary.

Register codes include:

- `r01` is register 1, the *2nd* register available
- `r0a` is register 10, the *11th* register available
- `r10` is register 16, the *17th* register available

As there are 256 registers available in Crawssembly, the codes range from `r00` to `rff`, since the register at 0xff is the *256th* register available

So the example instruction `sav 10 r01` saves the literal value `10` into register 1, which is the *2nd* register available.

Most registers are available for general-purpose storage. But some registers do other functions. The 3 most important ones are `r00`, `ref`, and `rff`. These are the *1st*, *240th*, and *256th* registers respectively.

- Register 0x00 (`r00`): The first register is **read-only**. This means that you can look at what's inside the register, but you can't save a value to it. This is because `r00` is used for input data, such as a file or a list of numbers.
- Register 0xEF (`ref`): This register is the **ASCII** register. Any value that is saved to this register is converted to a letter and is shown to the terminal screen. Visit [this page](https://www.asciitable.com/) to see what ASCII numbers relate to what letter.
- Register 0xFF (`rff`): The last register is **write-only**. This means that you can save a value to the register, but you can't look at what's inside. This is because `rff` is used for the computer output, like the result of a maths problem the computer has solved.

In addition, registers from `rf0` to `rfe` already contain values. These represent constants such as pi, Euler's number, some roots, and certain logarithms. There is more on these later on.

You can imagine the registers as boxes inside the computer:

```
┌─────┬────────┐
│ r00 │  Input │
├─────┼────────┤
│ r01 │   10   │
├─────┼────────┤
│ r02 │   20   │
├─────┼────────┤
│ ... │   ...  │
├─────┼────────┤
│ ref │  Text  │
├─────┼────────┤
│ ... │   ...  │
├─────┼────────┤
│ rff │ Output │
└─────┴────────┘
```

### Activity: Outputs

Because `ref` can be used to output characters to the screen, see if you can use this beginner input/output instruction to show any text you desire.

```
sav 72 ref      ; 72 = 'H'
sav 101 ref     ; 101 = 'e'
sav 108 ref     ; 108 = 'l'
sav 108 ref     ; 108 = 'l'
sav 111 ref     ; 111 = 'o'
```

This program shows "Hello". Try to add to this program, using [this page](https://www.asciitable.com/) as a reference, to get "Hello World!" to be shown.

> Possible solutions to activities like this one can be found in the folder titled `answers`. If you get stuck, they are there to help you solve an activity and still continue learning. However, attempting the activity first is advised, failing is the best way to learn!

### More about 'literals'

Literals in Crawssembly can be positive or negative, however they can only be between -128 and 127.

Why these values? Because, of course, binary.

Like other assembly languages, every possible instruction is encoded as a binary number. In Crawssembly's case, each instruction is 21 numbers, or **bits**.
Because each line has this fixed size, literal values can only be so high. The literal in a `sav` command is given 8 bits, also known as a **byte**.

More details about how Crawssembly encodes data will be explained in later sections.

Unlike the raw register values, **Two's Complement** used in the literal values are **8-bit**, not **32-bit**.

| Decimal Value | Binary |
| ------------- | ------ |
| 10 | 00001010 |
| 1 | 00000001 |
| 0 | 00000000 |
| -1 | 11111111 |
| -2 | 11111110 |
| -10 | 11110110 |

### More about `sav`

One of the best things about computers is one that often goes unnoticed; flexability. A vast amount of tasks would be nigh on impossible if the machines of the modern world were to become entirely hard-coded.
If you hard-coded everything, then there would be little point for computing languages, logical abstraction, and computers as a whole. If you know the result, why build a machine to express the program?

Crawssembly is no exception, dynamically storing values is as easy as removing the literal value in-place for another register

Example

```
sav 10 r01              ; saves 10 to register 1
sav r01 r02             ; saves value of register 1 to register 2
```

See how the literal value in the second line was replaced by `r02`? This program saves the literal value `10` into `r01`, and the next instruction saves whatever value is in `r01` into the register `r02`.
The computer has no clue what `r01` actually contains, just that it should copy whatever may be inside `r01` into `r02`. This reveals another key truth about computers:

"The machine is blind."

</details>

<details>
<summary><strong>Maths</strong></summary>

## Maths

Any good programmer must have a good mathematical brain, computers are large heaps of silicon held together by electricity and maths. Just as a hardware engineer needs to know about the electron, the software programmer needs to know about **bitwise operations**

Because of the binary representation of values, they can be manipulated in many ways to achieve a result.

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

You can imagine AND to be like a padlock that needs 2 keys to open.

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

0b1010 shifted 1 unit to the left is 0b10100  
0b1010 shifted 1 unit to the right is 0b101

SHIFT is split into 3 subcategories:
- Logical Shift Left (shl)
- Logical Shift Right (shr)
- Arithmetic Shift Right (sar)

> Because the sign of numbers is stored in the first bit of the binary numbers, shifting to the right can change the sign chaotically. Arithmetic Shift Right preserves the sign of the number.

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

#### How to use

Calculation using these operations uses the `cal` instruction. All `cal` results get saved to register 1 (`r01`).

Example

```
sav 10 r01              ; saves 10 into register 1
cal add 5 r01           ; adds 5 to the value in register 1
```

This example adds `10` to `5`, resulting in `r01` having a value of `15`

Because of the nature of commands, the first value can be a register or literal, but the second value **must** be a register value. `cal add 1 1` would not work. This is why in the example, `10` must be saved to a register.
More information will be given for why this is the case later on.

Like `sav`, the first value can be swapped for a register code.

Example

```
sav 10 r01              ; saves 10 to register 1
sav 20 r02              ; saves 20 to register 2

cal xor r01 r02         ; XORs 10 and 20, result saves to r01
```

```
sav 1 r01               ; set shift amount
cal shl 7 r01           ; 0b111 shifted to the left by 1 unit is 0b1110
```

### Activity: Addition

```
sav 50 r01
```

Add to this program to triple the value, `50`, stored in `r01`.

Feel free to add outputs to your programs to test if your program works.

### Advanced Activity: Doubling

**Without using `cal add`**, write a program to double a number.

> Hint: Multiplying numbers by 10 is easy, just add a 0 to the end. What is the parallel to binary?

### Bit masking

Bit masking is a technique to extract or modify specific bits within a binary number. Masking is very efficient as binary operations are some of the easiest tasks for computers to execute.

#### Extraction

If you had the number `0b01100100` (i.e. 100) stored in `r01` and you only wanted the last 4 bits, you would use the AND operator with `0b01100100` and `0b00001111` (i.e. 15, called the **mask value**)  as inputs.

Example

```
sav 100 r01             ; 0b01100100
cal and 15 r01          ; 0b01100100 AND 0b00001111 = 0b00000100 (i.e. 4)
```

This works because AND needs both values to be `1`. Any value not included in the **mask value** is set to `0`, and any `1` value included in the **mask value** is set to `1`.

#### Turning On

To force a particular bit to be `1`, then use OR with the mask value being the bit position you desire. The following example changes `0b01100100` to `0b01100110` using a bit mask of `0b00000010` (i.e. 2).

Example

```
sav 100 r01             ; 0b01100100
cal or 2 r01            ; 0b01100100 OR 0b00000010 = 0b01100110 (i.e. 102)
```

This works because any `1` value present will remain a `1`, so if the mask value contains a `1`, the result will also contain a `1` regardless of the other value.

> You can set multiple bits in this mask, if you really wanted to you could make a mask of 0b11111111 to set **every** bit to `1`!

#### Turning Off

To force a particular bit to be `0`, you must use AND with a mask value of all `1`s apart from the bit you want to set to `0`. The following example changes `0b01100100` to `0b01000100` using a bit mask of `0b11011111` (i.e. -33).

Example

```
sav 100 r01             ; 0b01100100
cal and -33 r01         ; 0b01100100 AND 0b11011111 = 0b01000100 (i.e. 68)
```

This works because all bits are untouched in AND, apart from the missing bits which are turned off, because AND needs both to be `1`.

> Like using OR, you can set multiple bits off using this method.

#### Toggling

Toggling a bit uses the combined XOR calculation, with a mask value of the same form as AND, where each `1` represents the bit to flip. The following example changes `0b01100100` into `0b01000110` using a mask value of `0b00100010` (i.e. 34).

Example

```
sav 100 r01             ; 0b01100100
cal xor 34 r01          ; 0b01100100 XOR 0b00100010 = 0b01000110 (i.e. 70)
```

This works because XOR outputs `1` when a difference in inputs is found. If an 'active' bit is `0`, there is a difference so `1` is outputted. If the 'active' bit is `1`, there is no difference so `0` is outputted.

> Like OR again, you can toggle multiple bits using this method.

#### Negation

To turn a value into it's negative version, you must **flip the bits and add 1**. You can do this using NOT and then ADD. The following example turns 100 into -100 (`0b01100100` into `0b10011100`)

Example

```
sav 100 r01             ; 0b01100100
cal not r01 rff         ; NOT 0b01100100 = 0b10011011 (i.e. -101)
cal add 1 r01           ; 0b10011011 + 1 = 0b10011100 (i.e. -100)
```

> `rff` was used here to indicate that NOT doesn't take another input, because `rff` can't be read from.

### Symbols

You're likely to see symbols in-place of words like AND or XOR. The most common symbols are shown below.

| Operation Name | Possible Symbols |
| -------------- | ---------------- |
| AND | & or && |
| OR | \| or \|\| |
| NOT | ~ or ¬ or ! |
| XOR | ^ |
| Left Shift | << |
| Right Shift | >> |

</details>

<details>
<summary><strong>Loops</strong></summary>

## Loops

Say you wanted to do a very similar, or exact, set of instructions multiple times.

```
sav 1 r01
cal add 1 r01
cal add 1 r01
cal add 1 r01
cal add 1 r01
cal add 1 r01
cal add 1 r01
cal add 1 r01
```

This gets tiresome quickly! This type of code has a name; **boilerplate code**. It wastes time, and increases the chance of human error.

A much better way is to loop over the code you want to repeat, rather than copying it out over and over again.

### Labels

Loops are defined using a **label**. A label is a pointer to an instruction number. The line number is stored under this label. Simply state the label value as an instruction.
Loop labels

Example

```
1       ; creates the label '1' pointing to the first instruction
10      ; creates the label '10' pointing to the second instruction
```

Remember that annoying immediate limit from `sav` and `cal`? With labels the range is bigger, since negative numbers aren't used and the entire instruction is just the number.

**The range for labels is `0 - 65535`**

### Line Numbers

At this point it's a good idea to write Crawssembly with line numbers turned on. This is because, unlike most higher-level languages, the position of the instruction matters almost as much as what the instruction does.

Example

```
1       10
2       20
3       100
4       200
5       1000
6       2000
7       10000
8       20000
```

It's much easier to see that the label `10000` points to line `7` with line numbers! And for longer programs, a must.

For this reason, code examples in this section will be provided with line numbers to aid understanding.

### Removing Labels

To remove a label pointer, use `rmv`.

Example

```
1       10              ; label 10 points to line 1
2       20              ; label 20 points to line 2
3
4       rmv 10          ; removes label 10
5       10              ; label 10 now points to line 5
```

### Jumping

So you've got a label pointing to a line you want to execute many times. How do you get it to run again?

Simply use the **jump** command `jmp`, followed by the pointer label.

Example

```
1       sav 10 r01      ; saves literal value 10 into register 1
2       1               ; creates a label, pointing to line 2
3       cal add 1 r01   ; adds 1 to value inside register 1, saving it to register 1
4       jmp 1           ; jumps to the line number that label 1 is pointing to (i.e. line 2)
5       stp             ; end the program
```

This program increases the value in `r01`, being `10` in this example, every loop. This program is an example of an **infinite loop**, it will never reach the `stp` command.

Because this program is an infinite loop, you have to **force quit** the program, you can do this by pressing `CTRL` and `c` at the same time.

### Dynamic Jumping

Infinite loops, while nice in languages like Python and Java, are usually very unhelpful in assembly. Since the program is so close to the hardware, an infinite loop can't be stopped, save a full power cycle or in-built reset.

Making a loop run a set amount of times, or some number depending on values, is almost always necessary for loops.

Like other commands, conditional jumps work on the `r01` register.

There are 3 conditional jump commands:
- `jmg` - JuMp if Greater (Jumps to label pointer if `r01` > 0)
- `jmz` - JuMp if Zero (Jumps to label pointer if `r01` = 0)
- `jml` - JuMp if Less (Jumps to label pointer if `r01` < 0)

Example

```
1       sav 5 r01       ; saves the value '5' into register 1
2       1               ; creates a label pointer to line 2
3       jmg 1           ; if register 1 holds a >0 value, jump
4       stp             ; the program stops
```

Above is another example of an **infinite loop**, but the loop is only entered if the starting value in `r01` is greater than zero, thanks to the `jmg` command.

Example

```
1       sav 10 r02
2       sav 0 r03
3       1
4       cal add 1 r03
5       sav r01 r03
6       cal add -1 r02
7       sav r01 r02
8       jmg 1
9       stp
```

Let's read through each line in detail:

1. `sav 10 r02`: This saves the literal value `10` into register 2. This value is used for the number of times the loop will run. In this case, 10 times.
2. `sav 0 r03`: This is the initial value of the 'increment' register. This value is increased by `1` every loop cycle.
3. `1`: This is the loop label, it points to line 3.
4. `cal add 1 r03`: This adds `1` to the value inside `r03`, that being the 'increment' value.
5. `sav r01 r03`: This saves the +1 calculation step back into `r03`.
6. `cal add -1 r02`: This subtracts `1` from the loop count, indicating that a loop has taken place.
7. `sav r01 r02`: This line updates `r02`, where the loop count is stored.
8. `jmg 1`: Because the -1 step happened, the result is still stored in `r01`. If this becomes 0, the loop stops running.
9. `stp`: The program ends, resulting in 10 loops being executed.

### Activity: Letter Loops

Write a program that loops over every uppercase letter and prints it to the screen

> Hint: Uppercase letters start at `65`. Use [this page](https://www.asciitable.com/) for reference!

### Advanced Activity: Double Loops

Update your 'Letter Loops' activity program to contain a **nested loop** that prints
`AA AB AC AD AE AF AG` etc... `AX AY AZ BA BB BC BD` etc... `ZX ZY ZZ`

> Hint: Both loops take similar forms, but take care to not mix register values together, separate the 1st loop from the 2nd loop clearly, and reset the inner-loop values in the outer loop.

</details>

<details>
<summary><strong>Branching</strong></summary>

## Branching

Let's say you wanted to only run a piece of code if a certain condition was met.
You can use the `if` group of commands to execute code depending on the value in `r01`:

- `ifg`: Continues IF Greater (`r01` > 0)
- `ifz`: Continues IF Zero (`r01` = 0)
- `ifl`: Continues IF Less (`r01` < 0)

The `if` group also works on label pointers. To end the `if` command, use `rmv` on the label.

Example

```
1       sav 50 r01              ; sets '50' as the test value
2       ifz 1                   ; if branch using label 1 for test value = 0
3       2                       ; label 2 points to 3
4       jmp 2                   ; jumps back to label 2, an infinite loop
5       rmv 1                   ; ends the if branch using label 1
```

### Activity: Even Looper

Write a program that loops from 0 to 9, and outputs 'e' if the number is even, and 'o' if the number is odd.

### Advanced Activity: Even Checker

Edit your program for 'Even Looper', so that instead of outputting 'e' or 'o', it outputs the number if it's even, and doesn't output anything if the number is odd.

### Indentation

For longer programs, it can be difficult to see what branch label links to what code, you can make it easier for yourself by using **indentation**. This is whitespace before instructions to imply their scope, which is ignored by the Crawssembly compiler.

Example

```
ifg 1
    ifz 2
      cal add r01 r02
    rmv 2
    ifg 2
      ifz 3
        cal not r01 rff
      rmv 3
    rmv 2
rmv 1
```

The above program is easier to read than

```
ifg 1
ifz 2
cal add r01 r02
rmv 2
ifg 2
ifz 3
cal not r01 rff
rmv 3
rmv 2
rmv 1
```

But both do the exact same function. 

> Indentation does not imply scope, that's determined my `rmv`. If there is no `rmv` command, the code will keep running no matter how much indentation you use. It is simply a visual hint.

</details>

<details>
<summary><strong>Turing Complete</strong></summary>

## Turing Complete

If a machine is **Turing Complete**, that means the machine could theoretically solve any computable problem. The instructions shows so far are enough for Crawssembly to be **Turing Complete**.

This means that you could go off now, and make a 3D renderer, a calculator, or calculate pi to a trillion decimal places. But it would be difficult, time-consuming, and not worth the effort. This is a reason high-level languages exist.

Not only are higher-level languages, like Python, easier to read, they condense lots of functionality into a couple of lines.

A program that would be 100 lines in Crawssembly, would only be a single line in Python.

For instance, writing `mylist = ["Hello", 27.6 + 1, True]` in Python means the computer has to allocate memory for the variable, solve the **floating-point** (a name for a non-whole number) calculation, and compress the string into bytes, all while maintaining the OS, data security, encryption, and other background tasks.

It's easy to see why, while powerful, programming in assembly has faded out of fashion among modern-day programmers.

</details>

<details>
<summary><strong>Files</strong></summary>

## Files

As said before, `r00` is the **input register**. You can select what these values are by adding an input file to Crawssembly's execution.

To add a file to the execution, add the `--file` option to `craw`.

Example

`craw program.craw --file input.txt`

This creates a list of **8-bit** (or **1 byte**) values that are stored in `r00`. To access the next byte, use the `inp` instruction.

Example

```
sav r00 ref
inp
sav r00 ref
inp
sav r00 ref
```

This program gets the first three bytes of the input values, and outputs that ASCII code to the screen.

Writing to `rff` appends the **lowest 8 bits** of the value to the output buffer, stored as `output.bin`.

Example

```
sav 72 rff              ; 72 = 'H' -> output register
sav 105 rff             ; 105 = 'i' -> output register
```

This program saves 'Hi' to `output.bin`.

> Because file writing is slow, the output file is only written after the program has ended. Force exiting might not create the entire `output.bin` file.

</details>

<details>
<summary><strong>Devices</strong></summary>

## Devices

`io` is the largest group of commands, used to interact with data outside the CPU, such as speakers, keyboards, and storage. All `io` commands take a register value as input, never an immediate value.

Because `io` commands are dependent on other devices, commands might fail if there is an issue. This is why register `ree` is used for *error codes*

| Code | Name | Description |
| ---- | ---- | ----------- |
| 0 | IO_OK | Operation completed successfully |
| 1 | IO_INVALID_COMMAND | Invalid device command |
| 2 | IO_BAD_VALUE | Value outside valid range |
| 3 | IO_UNAVAILABLE | Device unavailable |

</details>

<details>
<summary><strong>Storage</strong></summary>

## Storage

Computers use two different systems for storing data; **Memory** and **Disk**.

- Memory: A fast way to temporarily store data. It is where active data which is being used is stored. Memory is cleared when the computer resets, called **volatile**.
- Disk: Long-term storage, used for files, apps, operating systems, and more. The disk is usually slower than memory, which is why only long-term data is stored. Because data is kept, it is called **non-volatile**.

Registers are the fastest type of data storage, but the relative data they can hold is tiny. Crawssembly registers can hold a little over a kilobyte in register storage, but can access around 17 million kilobytes of memory, and another 17 million on the disk!

| Storage Medium | Storage Length | Size | Speed |
| -------------- | -------------- | ---- | ----- |
| Registers | Very Short | Very Small | Very Fast |
| Memory | Short | Large | Fast |
| Disk | Indefinite | Very Large | Slow |

### Addresses

Much like registers, every cell in storage has a number based on it's position. This number is called the **address**.

Like registers, the first cell has an address of 0, the second cell has an address of 1, etc...

Storage addresses are commonly given in **hexadecimal** format, such as `0xF58`. This is because hex perfectly matches how computer hardware is laid out, while keeping the address short enough for humans to read.

Because registers can hold values between `- 2^31` and `2^31 - 1` (This is the *signed 32-bit limit*), the largest address is **0xFFFFFFFF**. That's over 4 billion addresses available!

> Addresses can theoretically range up to 0xFFFFFFFF, although the VM provides memory up to 65536 addresses. If your computer has 8GB of RAM, Crawssembly would take all of it, and would crash your machine!

### Memory

You've probably heard of RAM, or Random Access Memory. RAM is a type of memory used by computers for large amounts of data.
It's called 'Random Access' because the computer can read and write to any piece of data in any order. This makes RAM fast and efficient compared to older storage methods.

Like registers, memory is empty on startup. To write to a memory address, you must use a command in the `io` group.

For using memory, we must use `io mem`.

Example

```
sav 10 r01              ; saves 10 to register 1
sav 20 r02              ; saves 20 to register 2
io mem addr r01         ; sets the active memory address to the value in r01 (i.e. 10)
io mem write r02        ; writes the value of r02 (i.e. 20) into the active memory address

sav 20 r01              ; saves 20 to register 1
io mem addr r01         ; sets the active memory address to the value in r01 (i.e. 20)
io mem read ref         ; reads the memory address contents into the text output register
```

- `io mem addr` sets the active memory address that Crawssembly is considering. All `io mem` commands act on this address.
- `io mem write` sets the value of the active memory address to whatever is in the input register.
- `io mem read` saves the value of the active memory address to the given register.

#### Activity: Read/Write Cycle

Write a program to write the codes for 'A', 'B', and 'C' to memory, then read them back from memory in reverse order.

#### Advanced Activity: Save Arrays

Write a program to loop over the numbers 1 to 10, saving each number to it's own memory address.

### Disk

Disk space is used for data you don't want to lose. These can be results of a long program, long files, photos, videos, etc...

Disk commands follow the same form as memory commands.

- `io disk addr` sets the active disk address that Crawssembly is considering. All `io disk` commands act on this address.
- `io disk write` sets the value of the active disk address to whatever is in the input register.
- `io disk read` saves the value of the active disk address to the given register.
- `io disk save` forces the disk file to update, this can take a while so only do this if you suspect the program won't exit cleanly.

Example

```
sav 10 r01              ; saves 10 to register 1
sav 20 r02              ; saves 20 to register 2
io disk addr r01        ; sets the active disk address to the value in r01 (i.e. 10)
io disk write r02       ; writes the value of r02 (i.e. 20) into the active disk address

sav 20 r01              ; saves 20 to register 1
io disk addr r01        ; sets the active disk address to the value in r01 (i.e. 20)
io disk read ref        ; reads the disk address contents into the text output register
```

> Disk data is saved to the file `storage.bin`, deleting this file will cause any saved data to be removed, so take care during file cleanup!

#### Activity: Long Live The Data

Write one program that saves a piece of data to disk. Then write a new program to read that data from the disk, and output it.

#### Advanced Activity: My Long List

Write one program to loop over the numbers from 0 to 9, saving each to a disk address. Then write a new program to read these values in reverse order, and output each one.

### What high-level programs do

High-level programs use memory all the time. When you type something like `5 + 10` the computer has no clue how large that result will be without executing it. So the program allocates memory for `5`, `10`, and the result.

Arrays (or lists) are just a group of memory addresses.
For example, an array like `[1, 2, 3]` is really just a set of memory addresses.

Possible representation:

`[1, 2, 3]`

=

```
Memory addresses

0x00000000: 1
0x00000001: 2
0x00000002: 3
```

=

```
sav 0 r01
sav 1 r02
io mem addr r01
io mem write r02

sav 1 r01
sav 2 r02
io mem addr r01
io mem write r02

sav 2 r01
sav 3 r02
io mem addr r01
io mem write r02
```

All variables are like this behind the fancy symbols and abstraction, just a group of memory addresses.

</details>

<details>
<summary><strong>Input</strong></summary>

## Input

If you've ever learnt a higher programming language, chances are they taught you how to get user input very early on. This is sensible, as the user gets near-instant gratification from adding their own name to a greeting program, or telling the computer how old they are to get a semi-personalised message.
So why is user input so far in the learning path for Crawssembly?

Because input uses the keyboard and mouse, there is a hidden layer between what you type, and that data being processed by the CPU. Much like storage, the keyboard is a separate device to the core computer. Because of this, input handling builds upon what you already know about more basic data manipulation.

### The Keyboard

Crawssembly provides direct keyboard access using `io keyboard`. The main command in this group is `io keyboard poll`, which gets the last key pressed as a key code into the input register.

Key Codes:
- 0: No key pressed
- -1: Up Arrow
- -2: Down Arrow
- -3: Left Arrow
- -4: Right Arrow
- -5: Enter
- 27: Escape
- other: ASCII code ([See this chart for more](https://www.asciitable.com/))

Example

```
1                               ; Label pointing to line 1
  sav 0 r01                     ; Resets last keycode to get most recent
  io keyboard poll r01          ; Gets the last key and stores into register 1
  jmz 1                         ; If no key pressed, don't print
  sav r01 ref                   ; Outputs that key code to the screen
  cal add -27 r01               ; Finds key - 27, the result is saved to register 1
  jmg 1                         ; continues the loop if the key code is greater than 26
  jml 1                         ; continues the loop if the key code is less than 27
  stp                           ; stops the program if the key code is exactly 27
```

This program shows the last key printed by outputting to `ref`, and ends the loop when `Esc` is pressed.

> Remember, `ref` outputs the [ASCII code](https://www.asciitable.com/) of the key pressed, not the real raw value of that key. Pressing Enter won't print 'Enter' to the screen!

### The Mouse

Almost all basic mice output 4 streams of data:
- Position
- Left Button
- Middle Button
- Right Button

The `io mouse` command group captures mouse events using 3 commands:
- `io mouse x`: extracts current mouse X position, relative to the screen's top-left corner
- `io mouse y`: extracts current mouse Y position, relative to the screen's top-left corner
- `io mouse btn`: creates a **bitfield** for the middle, right, and left buttons in that order.

> Normal terminals can't capture mouse position, which is why `--tui` needs to be used for mouse events to work.

#### Position

The mouse's position can be extracted using `io mouse x` and `io mouse y`.

Example

```
io mouse x r01
io mouse y r02
```

This example stores the (x, y) location as (`r01`, `r02`).

#### Button Bit field

A **bit field** is a single binary number which encodes multiple values. Instead of using a single bit for if the left mouse button is being pressed, another for the right button being pressed, and another again for the middle button, we can merge these 3 numbers into a single number.

| Middle | Right | Left | Bit field | Decimal |
| ------ | ----- | ---- | --------- | ------- |
| Off | Off | Off | `0b000` | 0 |
| Off | Off | On | `0b001` | 1 |
| Off | On | Off | `0b010` | 2 |
| Off | On | On | `0b011` | 3 |
| On | Off | Off | `0b100` | 4 |
| On | Off | On | `0b101` | 5 |
| On | On | Off | `0b110` | 6 |
| On | On | On | `0b111` | 7 |

You can extract each button using bit masks.

Example

```
io mouse btn r01
cal and 1 r01
```

This program extracts the *left mouse* button into `r01`.

> We covered bit masks earlier on when talking about binary operations.

Bit masks can be used to expand the mouse bit field into separate registers.

Example

```
io mouse btn r04

cal and 1 r04

cal and 2 r04
sav r01 r02

cal and 4 r04
sav r01 r03
```

This example unpacks the bitfield, stored in `r04` into `r01` (Left Button), `r02` (Right Button), and `r03` (Middle Button)

#### Activity: Clicking Away

Write a program to show "Click" when a mouse button is pressed.

#### Advanced Activity: Mouse Dump

Write a program that shows "L" if the left button is pressed, "R" if the right button is being pressed, and "M" if the middle button is being pressed.

</details>

<details>
<summary><strong>Output</strong></summary>

## Output

Inputting values into your program is all well and good, but it's pretty useless if you can't get anything meaningful back. Crawssembly can control both your screen, and your speakers.

### The Screen

The screen is a great all-purpose viewing device. Not only can it show text, it can show graphics and symbols.

> Crawssembly's screen is contained within the terminal. This is to protect your screen's contents outside the terminal, and to allow outputs to be easily seen. However, this makes graphics quite slow, so fast graphics programs won't work as well in the Crawssembly VM.

#### Text

Before now, program outputs have used the `ref` register to send raw ASCII codes to the screen. While useful for debugging, this isn't the best way to get a program output. This uses the `io text` group of commands.

- `io text char`: Shows the ASCII code stored in the input register to the screen, the more *idiomatic* version of using `ref`.
- `io text int`: Shows the actual value inside the input register.
- `io text newline`: Shorthand for moving the text cursor to a new line.
- `io text hex`: Shows the hex value stored inside the input register.
- `io text clear`: Clears the terminal, is usually quite slow.

Example

```
sav 100 r01

io text char r01
io text newline rff

io text int r01
io text newline rff

io text hex r01
io text newline rff
```

Output

```
d
100
64
```

#### Activity: The New Way To Talk

Write a program that prints the key you press, and make it so when `Esc` is pressed, the program ends.

#### Advanced Activity: Arcade Leaderboard Name

Write a program that stores a 3 letter name from the keyboard, and then output `Hello, ` followed by that name.

### Graphics

Unlike text output, graphics allow programs to control individual pixels. Crawssembly provides a virtual screen using the `io screen` command group.

There are 8 main commands in the `io screen` group:
- `io screen x`: sets the current active X coordinate
- `io screen y`: sets the current active Y coordinate
- `io screen red`: sets the current red colour value
- `io screen green`: sets the current green colour value
- `io screen blue`: sets the current blue colour value
- `io screen pixel`: updates the graphics buffer at (X,Y)
- `io screen present`: sends the graphics buffer to the screen
- `io screen clear`: clears the screen buffer

> To get graphics working properly, you almost always need to use `craw myprogram.craw --tui`, since this sends event data to Crawssembly. When using `--tui`, the terminal screen clears when the program ends. A good way to stop this is by making a loop at the end of the program, that pressing a keyboard key exits from and ends the program once you've finished looking at the output.
>
> RGB values are mapped from -128 to 127, so `-128` is no colour, `0` is some colour, and `127` is all colour. This is so all colours can be made with immediate values for ease of use.
>
> You commonly see RGB values in **Hexadecimal**, so 0xFFFFFF would be white, 0xFF0000 would be red, 0x00FF00 would be green, 0x0000FF would be blue, and 0x000000 would be black.

Example:

```
sav 5 r01               ; saves 5 into register 1, used for the pixel position
sav 127 r02             ; saves 127 into register 2, used for colouring the pixel
io screen x r01         ; sets the X coordinate to 5
io screen y r01         ; sets the Y coordinate to 5

io screen red r02       ; sets the red colour to 127, making the colour 0xFF0000
io screen green r02     ; sets the green colour to 127, making the colour 0xFFFF00
io screen blue r02      ; sets the blue colour to 127, making the colour 0xFFFFFF

io screen pixel         ; updates the graphics buffer, pixel at (5,5) now has a colour of 0xFFFFFF (i.e. pure white)

io screen present       ; sends the graphics buffer to the screen
```

> Because graphics are being used, `craw myfile.craw --tui` should be used for best results. You can also set the size of the graphics screen using `craw myfile.craw --tui --screen WIDTHxHEIGHT`, and replace `WIDTHxHEIGHT` with your screen dimensions.

#### Activity: Smiling Screen

Write a program that outputs a smiley face.

#### Advanced Activity: Colourful Terminal

Add to your program in "Smiling Screen" to make each pixel a different colour.

#### Extra Screen Commands

There are some additional commands that are lesser used, but still useful:
- `io screen dump`: prints a low-quality dump of the current screen output
- `io screen erase`: clears a single pixel from the buffer
- `io screen erasecell`: clears both the top and bottom of the active **terminal cell**

> Crawssembly uses a faux-graphics system, which is actually coloured Unicode blocks. Because of this, each line of the graphics output is actually 1 half of a terminal character. So running `io screen erasecell` clears the entire cell, not just one of the halves.

### Speakers

Making your computer 'speak' is one of the more enjoyable ways to get a program output. Crawssembly provides many commands to take control of your speakers using `io speaker`.

The commands are:
- `io speaker channel`: sets the active speaker channel. There are 8 channels in Crawssembly to play sounds from.
- `io speaker freq`: sets the active channel's frequency, in Hertz
- `io speaker volume`: sets the active channel's volume, from 0 to 100
- `io speaker wave`: sets the wave type of that channel
- `io speaker on`: turns on the active channel
- `io speaker off`: turns off the active channel
- `io speaker toggle`: toggles the on/off state of the active channel

> Crawssembly might not be able to access every channel your machine exposes. Because of this, audio is the most temperamental functions of Crawssembly as it's entirely dependent on the host, not the Crawssembly VM. If you encounter issues, please raise a [**Github Issue**](https://github.com/Jonah-Crawford/Crawssembly/issues/new) with the details!

The *wave type* is a number to tell the channel what type of sound to make.

| Wave Type | Wave Name | Sounds Like? |
| --------- | --------- | ------------ |
| 0 | Square Wave | Retro/Arcade like chords |
| 1 | Sine Wave | Pure sound, no texture |
| 2 | Triangle Wave | Flute-like tones |
| 3 | Sawtooth Wave | Aggressive, harsh beeps |
| 4 | Random Noise | Great for sound effects |

Because sound is very time-dependent, you'll want to play sounds for a set period of time. You can do this by pausing the program using `io time sleep`.

> `io time sleep` pauses Crawssembly's execution for the inputted length in **milliseconds**. There are other `io time` commands which will be covered shortly.

Example

```
sav 100 r02             ; saves 100 to register 2, used as the sleep length in milliseconds

sav 125 r01             ; saves 125 to register 1
cal add 125 r01         ; 125 + 125 = 250
sav r01 r03             ; saves 250 to register 3, used as the frequency value

sav 0 r01               ; saves 0 to register 1, used as the speaker channel

io speaker channel r01  ; sets active speaker channel to '0'
io speaker freq r03     ; sets active speaker to 250Hz

sav 50 r01              ; saves 50 to register 1, used for the speaker volume

io speaker volume r01   ; sets active speaker volume to 50%

io speaker on rff       ; turns on speaker
io time sleep r02       ; waits 100ms

io speaker off rff      ; turns off speaker
io time sleep r02       ; waits 100ms

io speaker on rff       ; turns on speaker
io time sleep r02       ; waits 100ms

io speaker off rff      ; turns off speaker
```

> All speakers stop playing when the program ends or is forcefully quit.

#### Activity: Piano Note

Write a program to play a 440Hz tone for 1 second (1000 milliseconds).

#### Advanced Activity: Scales

Write a program that plays every note in an octave. You can use [this Wikipedia article](https://en.wikipedia.org/wiki/Piano_key_frequencies) to find the frequencies.

> Real musical/piano notes increase by the twelfth-root of 2. Because Crawssembly can't use fractional numbers, the frequency values have to be rounded to the nearest whole number.

</details>

<details>
<summary><strong>Time</strong></summary>

## Time

It's common, in larger programs, that the current time should be known. In computers, this is maintained through a CMOS battery. This keeps the internal clock running on the machine when the computer is turned off.

To use time commands, you would use the `io time` group. We already discussed `io time sleep` in the speakers section.
- `io time sleep`: pauses execution for a set period in milliseconds
- `io time unix`: outputs the current **UNIX timestamp**
- `io time low`: outputs the last 31 bits of the **UNIX timestamp**

### UNIX Timestamp

The **UNIX timestamp**, or **UNIX Epoch** is the starting point for measuring time in computer systems. It's defined exactly as the **1st January 1970, at 00:00:00 UTC**.

This time was selected as a nice round start point for the early development of the UNIX operating system in the 1970's, the direct ancestor of the most dominant modern operating systems in use today.

#### More about `low`

`io time low` is a separate command due to binary limits. If you were alive before 2000, you likely remember the *Y2K* bug, where early programs that used only 2 digits for the year (e.g. '99' for '1999') would fail in the new millennium. A similar, but different issue occurs because of the limit of binary numbers.

Because lots of systems are **32-bit**, meaning that the data length in the CPU is 32-bits of binary, the largest positive number this can take is around 2.15 billion. The UNIX timestamp reaches this number of seconds early in 2038. Once we go over that point, the timestamp will roll over into negative numbers. This is an example of **underflow**. This causes the timestamp to point to December 1901.

This is called the **2038 problem**, or **Y2K38**. And because it affects many more systems than Y2K, it's actually more dangerous, and harder to fix as low-level systems are used everywhere. But Crawssembly is prepared. Using `io time low`, only the value bits are extracted, not the sign. So the time can be used without worrying about negative time.

</details>

<details>
<summary><strong>Functions</strong></summary>

## Functions

Crawssembly doesn't have functions like higher level programs do. But you can still run other programs, similar to library imports.

You can do this using `execute path/to/file.craw`. Unlike the other instructions, this is a **macro**, a non-binary instruction that is interpreted by the compiler.

`execute` tells the compiler to read the file being pointed to, and copy that file in to replace that line.

It's very important to know what your function does when it's copied in, since the same registers and labels are used before and after the program is executed.

Example

```
; add_three.craw

cal add 3 r10           ; adds 3 to value inside r10
sav r01 r10             ; updates r10 value
```

```
; main.craw

sav 100 r10             ; sets input of add_three to 100

execute add_three.craw  ; executes the add_three.craw program

io text int r10         ; shows the result (103)
```

> You must make sure that the path to the function is correct, else `craw main.craw` will fail.

</details>

<details>
<summary><strong>Crawssembly Standard Library</strong></summary>

## Crawssembly Standard Library

The **Crawssembly Standard Library (CSL)** is a group of common functions included to
make it easier to write longer programs. Use of `executestd module/function.craw`
is used to access the program. `execute` is used for **user-created** programs,
and is differentiated as `execute` uses the path relative to the program being executed, while `executestd` works *anywhere* on your machine.

> Like `execute`, `executestd` is a **macro**, not a raw binary instruction.

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

### Base Programs

This is the list of programs available, along with Inputs, Outputs, and **Scope**.

> The **scope** of a function is the range of data the program touches. It's important to note what values get mutated so your program works correctly. This includes label values.

| Module  | Function        | Inputs    | Outputs   | Scope                    | Description                                                               |
| ------- | --------------- | --------- | --------- | ------------------------ | ------------------------------------------------------------------------- |
| Core    | `reset_regs`    | `r02`     | -         | Variable, `60000`        | Resets registers from `r01` up to the register specified in `r02`.        |
| Compare | `equal`         | `r02-r03` | `r02`     | `r01-r02`, `60000`       | Outputs `1` if `r02 == r03`, otherwise `0`.                               |
| Compare | `greater`       | `r02-r03` | `r02`     | `r01-r02`, `60000`       | Outputs `1` if `r02 > r03`, otherwise `0`.                                |
| Compare | `greater_equal` | `r02-r03` | `r02`     | `r01-r02`, `60000`       | Outputs `1` if `r02 >= r03`, otherwise `0`.                               |
| Compare | `less`          | `r02-r03` | `r02`     | `r01-r02`, `60000`       | Outputs `1` if `r02 < r03`, otherwise `0`.                                |
| Compare | `less_equal`    | `r02-r03` | `r02`     | `r01-r02`, `60000`       | Outputs `1` if `r02 <= r03`, otherwise `0`.                               |
| Compare | `not_equal`     | `r02-r03` | `r02`     | `r01-r02`, `60000`       | Outputs `1` if `r02 != r03`, otherwise `0`.                               |
| Math    | `abs`           | `r02`     | `r02`     | `r01-r02`, `60000`       | Returns the absolute value of `r02`.                                      |
| Math    | `divide`        | `r02-r03` | `r02-r03` | `r01-r04`, `60000`       | Quotient in `r02`, remainder in `r03`.                                    |
| Math    | `modulo`        | `r02-r03` | `r02`     | `r01-r03`, `60000`       | Returns only the remainder.                                               |
| Math    | `multiply`      | `r02-r03` | `r02`     | `r01-r04`, `60000`       | Multiplies `r02` by `r03`.                                                |
| Math    | `negate`        | `r02`     | `r02`     | `r01-r02`, `60000`       | Negates `r02`.                                                            |
| Math    | `power`         | `r02-r03` | `r02`     | `r01-r05`, `60000-60001` | Raises `r02` to the power of `r03`.                                       |
| Math    | `random`        | -         | `r02`     | `r01-r03`, `60000`       | Generates a pseudorandom integer using the current millisecond timestamp. |
| Math    | `sign`          | `r02`     | `r02`     | `r01-r02`, `60000`       | Returns `-1`, `0`, or `1` depending on the sign of `r02`.                 |

### Activity: Multiplication

Write a program that multiplies 2 numbers together using `executestd math/multiply.craw`

### Advanced Activity: Calculator

Write a program that outputs `r02` + `r03`, `r02` * `r03`, and `r02` / `r03`.

> Remember, look at the scope of each `executestd` call to make sure your values don't get lost.

### Custom Programs

Adding to this list is quite easy, simply add your programs to the `std/` folder inside the main Crawssembly folder (where you installed Crawssembly in the first place) and update the CSL by running `craw install-std`. This will propogate your new programs and modules you added
so that they too can be used from any location. This is helpful for custom programs you would use often that aren't included.

</details>

<details>
<summary><strong>Register Constants</strong></summary>

## Register Constants

Like mentioned before, registers from `rf0` to `rfe` have preloaded constants.

All constants are scaled by **100 million** to preserve their decimal places. 314159265‎ is much more useful as pi than 3 because the decimal places are preserved.

| Register | Value | Constant |
| -------- | ----- | -------- |
| `rf0` | 314159265 | π |
| `rf1` | 271828182 | e |
| `rf2` | 30102999 | log₁₀(2) |
| `rf3` | 47712125 | log₁₀(3) |
| `rf4` | 69897000 | log₁₀(5) |
| `rf5` | 69314718 | ln(2) |
| `rf6` | 109861228 | ln(3) |
| `rf7` | 160943791 | ln(5) |
| `rf8` | 141421356 | √2 |
| `rf9` | 173205080 | √3 |
| `rfa` | 223606797 | √5 |
| `rfb` | 125992105 | ∛2 |
| `rfc` | 144224957 | ∛3 |
| `rfd` | 170997594 | ∛5 |
| `rfe` | 2147483647 | 2^31 - 1 |

> Remember, 2^31 -1 is the largest positive number that can be stored in **32-bit** values, like what Crawssembly uses.

</details>

<details>
<summary><strong>Congratulations!</strong></summary>

## Congratulations!

You've learned all the Crawssembly instructions! That's no small feat, especially for a beginner. Great job!

The rest of this section is used as quick-reference and help.

### Quick Reference

`sav IMMEDIATE REGISTER` or `sav REGISTER REGISTER`: Saves the value of the first argument to the register in the second argument.  
`cal OPERATION IMMEDIATE REGISTER` or `cal OPERATION REGISTER REGISTER`: Calculates 'VALUE/REGISTER OPERATION REGISTER', and saves the result to `r01`.  
`LABEL`: Creates a label with that value pointing to that line number.  
`jmp LABEL`: Jumps unconditionally to LABEL.  
`jmg LABEL`: Jumps to LABEL if `r01` is greater than 0.  
`jmz LABEL`: Jumps to LABEL if `r01` is equal to 0.  
`jml LABEL`: Jumps to LABEL if `r01` is less than 0.  
`ifg LABEL`: Continues if `r01` is greater than 0.  
`ifz LABEL`: Continues if `r01` is equal to 0.  
`ifl LABEL`: Continues if `r01` is less than 0.  
`rmv LABEL`: Removes the label from memory, and ends any `if` commands.  
`stp`: Stops the program.  
`nop`: Does nothing.  

`io text`
- `io text char`: Shows the character code stored in the input register.
- `io text int`: Shows the integer value stored in the input register.
- `io text newline`: Drops the text cursor to the next terminal line.
- `io text hex`: Shows the value of the input register as a Hexadecimal value.

`io time`
- `io time unix`: Extracts the current UNIX timestamp into the input register.
- `io time low`: Extracts the value-only bits of the UNIX timestamp, avoids potential negative values.
- `io time sleep`: Pauses execution for the inputted number of milliseconds.

`io screen`
- `io screen x`: Sets the current active X coordinate.
- `io screen y`: Sets the current active Y coordinate.
- `io screen pixel`: Updates the active pixel in the graphics buffer.
- `io screen clear`: Wipes the graphics buffer clean.
- `io screen dump`: Print the current screen as a simple character field.
- `io screen present`: Sends the graphics buffer to the screen.
- `io screen red`: Sets the red RGB code of the active pixel.
- `io screen green`: Sets the green RGB code of the active pixel.
- `io screen blue`: Sets the blue RGB code of the active pixel.
- `io screen erase`: Clears the active pixel from the graphics buffer.
- `io screen erasecell`: Clears the terminal cell from the graphics buffer.

`io keyboard`
- `io keyboard poll`: Gets the last key pressed as a code into the input register.

`io mouse`
- `io mouse x`: Gets the mouse X coordinate into the input register.
- `io mouse y`: Gets the mouse Y coordinate into the input register.
- `io mouse btn`: Gets the bitfield of Middle, Right, and Left buttons into the input register.

`io speaker`
- `io speaker channel`: Sets the active channel (0-7) to the value in the input register.
- `io speaker freq`: Sets the active channel's frequency, in Hertz, to the value in the input register.
- `io speaker volume`: Sets the active channel's volume from, 0-100, to the value in the input register.
- `io speaker wave`: Sets the active channel's wave type from Square, Sine, Triangle, Sawtooth, or Noise.
- `io speaker on`: Turns on the active speaker channel.
- `io speaker off`: Turns off the active speaker channel.
- `io speaker toggle`: Toggles the on/off state of the active speaker channel.

`io mem`
- `io mem addr`: Sets the active memory address to the value in the input register.
- `io mem read`: Reads the value of the active memory address into the input register.
- `io mem write`: Writes the value of the input register into the active memory address.

`io disk`
- `io disk addr`: Sets the active disk address to the value in the input register.
- `io disk read`: Reads the value of the active disk address into the input register.
- `io disk write`: Writes the value of the input register into the active disk address.
- `io disk save`: Forces the `storage.bin` file to update its contents.

### Common Mistakes

- Registers start at `r00`, not `r01`
- ref outputs [ASCII](https://asciitable.com/), not numbers
- Screen updates need `io screen present`
- Labels are not variables, they are line pointers
- Literals must be between `-128` and `127`
- Infinite loops require CTRL+C to break
- Memory and disk are different, memory is volatile while disk is permanent

You can stop at this point and you'd be fine. But to *really* understand what computers do, we have to peel back another abstraction layer.

</details>

<details>
<summary><strong>Sorry, Crawssembly is a lie.</strong></summary>

## Sorry, Crawssembly is a lie.

Remember how one of the first things that was said was that programming languages are a lie? That goes for Crawssembly too. The computer has no idea what `sav` or `cal` is, what it means, or how it works.

Ultimately, every programming language must be translated into machine code before the CPU can execute it. This was mentioned before, but now we'll actually get into the nitty gritty.

Crawssembly instructions all translate to a binary number of length **21**, so there are **2^21** possible Crawssembly commands, this accounts for all the various combinations of register codes and immediate values.

### Binary Breakdown

Below are the tables for what Crawssembly instructions look like to the machine.

Most instructions follow the form of `aa bbb cccccccc dddddddd`
- `aa`: Core Group, differentiates between `sav` and `cal` immediate values
- `bbb`: Instruction Modes for `cal`, or used for control instructions
- `cccccccc`: First input value, or in `io` this contains the device and instruction code.
- `dddddddd`: Second input, almost always a register code unless using labels.

#### Core Instruction Patterns

| Instruction | Pattern | Meaning |
| ----------- | ------- | ------- |
| `nop` | `00 000 00000000 00000000` | No operation |
| `inp` | `01 100 00000000 00000000` | Advance input |
| `stp` | `01 111 11111111 11111111` | Stop program |
| `sav reg reg` | `00 000 aaaaaaaa bbbbbbbb` | Save register `a` to register `b` |
| `sav imm reg` | `01 000 iiiiiiii bbbbbbbb` | Save immediate to register `b` |
| `cal op reg reg` | `10 ooo aaaaaaaa bbbbbbbb` | Calculate using register `a` and register `b` |
| `cal op imm reg` | `11 ooo iiiiiiii bbbbbbbb` | Calculate using immediate and register `b` |

#### `cal` Operation Codes

| Operation | Mode | Meaning |
| --------- | ---- | ------- |
| `not` | `000` | Flip each bit |
| `and` | `001` | `1` if both inputs are `1`, else `0` |
| `or`  | `010` | `1` if any input is `1`, else `0` |
| `xor` | `011` | `1` if only 1 input is `1`, else `0` |
| `shl` | `100` | Moves first value to the left |
| `shr` | `101` | Moves first value to the right |
| `sar` | `110` | Moves first value to the right, maintains sign |
| `add` | `111` | Adds two values together |

#### Control Instruction Patterns

| Instruction | Code | Pattern | Meaning |
| ----------- | ---- | ------- | ------- |
| `jmz` | `00 001` | `00 001 llllllll llllllll` | Jump to label if `r01` = 0 |
| `jmg` | `00 010` | `00 010 llllllll llllllll` | Jump to label if `r01` > 0 |
| `ifl` | `00 011` | `00 011 llllllll llllllll` | Continue if `r01` < 0 |
| `jml` | `00 100` | `00 100 llllllll llllllll` | Jump to label if `r01` < 0 |
| `ifg` | `00 101` | `00 101 llllllll llllllll` | Continue if `r01` > 0 |
| `ifz` | `00 110` | `00 110 llllllll llllllll` | Continue if `r01` = 0 |
| `jmp` | `00 111` | `00 111 llllllll llllllll` | Jump to label |
| `rmv` | `01 101` | `01 101 llllllll llllllll` | Removes/ends label scope |
| `io`  | `01 110` | `01 110 ddddcccc rrrrrrrr`| Accesses non-CPU devices |
| label definition | `01 111` | `01 111 llllllll llllllll` | Creates a label |

#### `io` Device Codes

| Instruction | Device | Usage |
| ----------- | ------ | ----- |
| `io text` | `0000` | Accesses text character printing |
| `io time` | `0001` | Accesses time-based data |
| `io screen` | `0010` | Accesses screen graphics |
| `io keyboard` | `0011` | Accesses keyboard events |
| `io mouse` | `0100` | Accesses mouse events |
| `io speaker` | `0101` | Accesses speaker control |
| `io mem` | `0110` | Accesses volatile storage |
| `io disk` | `0111` | Accesses persistent storage |

#### `io` Command Codes

| Instruction | Device | Command | Binary | Meaning |
| ----------- | ------ | ------- | ------ | ------- |
| `io text char` | `0000` | `0000` | `01 110 0000 0000 rrrrrrrr` | Print inputted character code |
| `io text int` | `0000` | `0001` | `01 110 0000 0001 rrrrrrrr` | Print input register's value |
| `io text newline` | `0000` | `0010` | `01 110 0000 0010 rrrrrrrr` | Moves the text cursor to the next line |
| `io text hex` | `0000` | `0011` | `01 110 0000 0011 rrrrrrrr` | Print input register's value in hexadecimal |
| `io text clear` | `0000` | `0100` | `01 110 0000 0100 rrrrrrrr` | Clears the terminal, can be slow |
| `io time unix` | `0001` | `0000` | `01 110 0001 0000 rrrrrrrr` | Stores current UNIX timestamp in input register |
| `io time low` | `0001` | `0001` | `01 110 0001 0001 rrrrrrrr` | Stores magnitude of the UNIX timestamp in input register |
| `io time sleep` | `0001` | `0010` | `01 110 0001 0010 rrrrrrrr` | Pauses execution for inputted number of milliseconds |
| `io time milli` | `0001` | `0011` | `01 110 0001 0011 rrrrrrrr` | Stores the `low` time in milliseconds |
| `io screen x` | `0010` | `0000` | `01 110 0010 0000 rrrrrrrr` | Sets active x coordinate in the graphics buffer |
| `io screen y` | `0010` | `0001` | `01 110 0010 0001 rrrrrrrr` | Sets active Y coordinate in the graphics buffer |
| `io screen pixel` | `0010` | `0010` | `01 110 0010 0010 rrrrrrrr` | Updates pixel in the graphics buffer at active coordinates |
| `io screen clear` | `0010` | `0011` | `01 110 0010 0011 rrrrrrrr` | Clears the graphics buffer |
| `io screen dump` | `0010` | `0100` | `01 110 0010 0100 rrrrrrrr` | Shows a basic readout of the current screen |
| `io screen present` | `0010` | `0101` | `01 110 0010 0101 rrrrrrrr` | Sends the graphics buffer to the screen |
| `io screen red` | `0010` | `0110` | `01 110 0010 0110 rrrrrrrr` | Sets the red colour value of the active coordinates of the graphics buffer |
| `io screen green` | `0010` | `0111` | `01 110 0010 0111 rrrrrrrr` | Sets the green colour value of the active coordinates of the graphics buffer |
| `io screen blue` | `0010` | `1000` | `01 110 0010 1000 rrrrrrrr` | Sets the blue colour value of the active coordinates of the graphics buffer |
| `io screen erase` | `0010` | `1001` | `01 110 0010 1001 rrrrrrrr` | Clears the pixel in the graphics buffer at the active coordinates |
| `io screen erasecell` | `0010` | `1010` | `01 110 0010 1010 rrrrrrrr` | Clears the entire terminal cell of the active coordinates in the graphics buffer |
| `io keyboard poll` | `0011` | `0000` | `01 110 0011 0000 rrrrrrrr` | Extracts the last key code pressed into input register |
| `io mouse x` | `0100` | `0000` | `01 110 0100 0000 rrrrrrrr` | Extracts current mouse X coordinate into input register |
| `io mouse y` | `0100` | `0001` | `01 110 0100 0001 rrrrrrrr` | Extracts current mouse Y coordinate into input register |
| `io mouse btn` | `0100` | `0010` | `01 110 0100 0010 rrrrrrrr` | Extracts button bit field into input register (Middle, Right, Left) |
| `io speaker channel` | `0101` | `0000` | `01 110 0101 0000 rrrrrrrr` | Sets the active speaker channel to the value in input register |
| `io speaker freq` | `0101` | `0001` | `01 110 0101 0001 rrrrrrrr` | Sets active speaker frequency to value in input register |
| `io speaker volume` | `0101` | `0010` | `01 110 0101 0010 rrrrrrrr` | Sets volume (0-100) of active speaker to value in input register |
| `io speaker wave` | `0101` | `0011` | `01 110 0101 0011 rrrrrrrr` | Sets the active speaker's wave type to value in input register |
| `io speaker on` | `0101` | `0100` | `01 110 0101 0100 rrrrrrrr` | Turns on the active speaker |
| `io speaker off` | `0101` | `0101` | `01 110 0101 0101 rrrrrrrr` | Turns off the active speaker |
| `io speaker toggle` | `0101` | `0110` | `01 110 0101 0110 rrrrrrrr` | Toggles the active speaker on/off |
| `io mem addr` | `0110` | `0000` | `01 110 0110 0000 rrrrrrrr` | Sets the active memory address to the value in input register |
| `io mem read` | `0110` | `0001` | `01 110 0110 0001 rrrrrrrr` | Extracts the value of the active memory address into input register |
| `io mem write` | `0110` | `0010` | `01 110 0110 0010 rrrrrrrr` | Sets the active memory address's value to the value in the input register |
| `io disk addr` | `0111` | `0000` | `01 110 0111 0000 rrrrrrrr` | Sets the active disk address to value in input register |
| `io disk read` | `0111` | `0001` | `01 110 0111 0001 rrrrrrrr` | Extracts the active disk address's value into input register |
| `io disk write` | `0111` | `0010` | `01 110 0111 0010 rrrrrrrr` | Sets the active disk address's value to value in input register |
| `io disk save` | `0111` | `0011` | `01 110 0111 0011 rrrrrrrr` | Forces the disk file to reload and update |

Because Crawssembly translates to raw binary, you can use binary (or hexadecimal!) in your programs.

Example

```
010000011010000000001           ; Translates to 'sav 100 r01'
io text int r01
```

Running the above will result in the exact same functionality as if your replaced `010000011010000000001` with `sav 100 r01`

### The VM

Crawssembly isn't real assembly, since your real CPU can't understand the raw binary. So Crawssembly runs in it's own **VM (Virtual Machine)**.

The VM is programmed in **Rust**, a language that focuses on speed and safety. Because of those qualities, it was chosen for Crawssembly to create fast, but safe, programs. The perfect testing/learning environment to explore low-level thinking.

#### The Pipeline

The VM is in two parts, the **Compiler** and the **Executioner**.

The Compiler goes through each line of the program, and converts it from Crawssembly to the binary machine code, stored *backwards*, in `program.bin`.

> The backwards binary called *little-endian*, meaning that the smallest bit comes first in order, this mirrors real CPU encoding as little-endian encoding can reduce clock times and increase efficiency by taking mathematical shortcuts.

The Executioner reads each binary code, decodes it into separate blocks, and executes the instruction, in the **Fetch-Decode-Execute** cycle.

```
                           ┌───────────────── Executioner ─────────────────┐
                           │                 (Ran in a VM)                 │
┌────────────── Compiler ──┼─────────────┐                                 │
│                          │             │                                 │
│ program.craw ----------> │ program.bin │ ----------> Fetched Instruction │
│                          │             │                     │           │
└──────────────────────────┼─────────────┘                     │           │
                           │      ^          FDE Cycle         │           │
                           │      │                            │           │
                           │      │                            V           │
                           │  Execution <------------- Decoded Instruction │
                           │                                               │
                           └───────────────────────────────────────────────┘
```

</details>

<details>
<summary><strong>The CPU Cycle</strong></summary>

## The CPU Cycle

Every program, from a tiny calculator to a modern web browser, ultimately runs by repeating one incredibly simple process over and over again.
This process is called the **Fetch-Decode-Execute** cycle, which we touched upon earlier.
A CPU doesn't understand a program all at once. Instead, it reads one instruction at a time, carries it out, then moves on to the next instruction.

```
Instruction
     │
     V
┌─────────┐
│  Fetch  │
└─────────┘
     │
     V
┌─────────┐
│ Decode  │
└─────────┘
     │
     V
┌─────────┐
│ Execute │
└─────────┘
     │
     V
Next instruction
```

Modern processors perform this cycle billions of times every second.

Even though a program might contain millions of instructions, the CPU is still only doing the same three steps repeatedly

Every operating system, game, browser, programming language, and AI model ultimately runs because a processor never stops repeating this simple cycle.

### Fetch

The CPU first finds the next instruction to execute.
To know where that instruction is stored, it keeps track of a number called the **Program Counter (PC)**.
If the program counter contains the value 15, the CPU fetches instruction number 15.
After fetching the instruction, the program counter usually moves on to the next instruction automatically.

### Decode

The fetched instruction is just a binary number. The CPU now works out what that binary means.

Example

```
sav 10 r01
```

is decoded into
- Instruction`sav`
- Source `10`
- Destination `r01`

The CPU now knows what job it has to perform

### Execute

The CPU carries out the instruction.

For our example:

`sav 10 r01`

means

```
Register 1

Before

r01 = 0

↓

After

r01 = 10
```

The instruction is now finished.

The CPU repeats the cycle for the next instruction.

</details>

<details>
<summary><strong>How you can help Crawssembly</strong></summary>

## How you can help Crawssembly

Eagle-eyed readers will have spotted that not *every* possible binary instruction has a function yet. This is to allow users like yourself to come up with ideas for Crawssembly's next versions!

*CRAW SYSTEMS* is proud to release Crawssembly in open-source format to allow anyone to make their own changes to Crawssembly. If you have any ideas for the Crawssembly team, feel free to contact the project lead **Jonah Crawford** at *jonah@jonahcrawford.com*. You can also make a [pull request](https://github.com/Jonah-Crawford/Crawssembly/compare) on the official GitHub page.

</details>

<details>
<summary><strong>What's next?</strong></summary>

## What's next?

You've done it! Kudos all around, you've done great work! So what's next?

If you want a challenge, here are some longer programming ideas to get stuck in:
- Build a bitmap viewer.
- Build a calculator.
- Build a drawing program.
- Build a music player.
- Build [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

</details>

<details>
<summary><strong>Legal</strong></summary>

## Legal

All code was written by **Jonah 'The Craw' Crawford**, with help of AI (Artificial Intelligence) at certain points; all ideas, techniques, and the vast amount of the program was, and continues to be, written by human hands.

### Thanks

Thank you to **Koy Camerini-Yachdav** who tested Crawssembly on macOS, and their amazing work making detailed error reports.
Thank you to the *CRAW SYSTEMS* team for help with programming, especially *@Xytrophico* with testing on Linux systems and providing invaluable help.

*Crawssembly is a product of CRAW SYSTEMS (2026)*

</details>
