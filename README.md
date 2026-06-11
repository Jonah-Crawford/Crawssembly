# Crawssembly: The beginner's gateway to low-level thinking.

![Documentation In Construction](https://img.shields.io/badge/Documentation-In_Construction-blue)

![Crawssembly Banner](https://www.dropbox.com/scl/fi/e4fhcba8zkgw2youvai8s/Crawssembly.png?rlkey=x1xojmcn29z9joxnmpb09iums&st=lxansr9a&raw=1)

Developed By J.D. Crawford. [Help support Crawssembly and open-source learning!](https://buymeacoffee.com/jonah_crawford)

![Status](https://img.shields.io/badge/Status-Under%20Development-blue)
![Rust](https://img.shields.io/badge/Rust-Implementation-orange)
![VM](https://img.shields.io/badge/Virtual%20Machine-Custom-blue)
![Beginner Friendly](https://img.shields.io/badge/Beginner-Friendly-brightgreen)

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

> Because Crawssembly assumes near-zero knowledge, there might be concepts you already know. Feel free to skip over them if you are familiar with anything shown!

![Registers](https://img.shields.io/badge/Registers-256-blue)
![Instruction%20Width](https://img.shields.io/badge/Instruction-21--bit-orange)
![Learn CPUs](https://img.shields.io/badge/Learn-CPU%20Architecture-blueviolet)

## How to install Crawssembly

![Linux](https://img.shields.io/badge/Linux-Tested-green)
![Windows](https://img.shields.io/badge/Windows-Tested-green)
![macOS](https://img.shields.io/badge/macOS-Untested-lightgrey)

> Present versions have been confirmed to work on Linux and Windows. Mac OS has not been tested, errors may occur.

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

To run a Crawssembly file (These are `.craw` files), simply run `craw <file.craw>` and swap `<file.craw>` with the name of the Crawssembly file you wish to edit.

Use `craw --help` to display helpful use information.

## Your First Program

Create a file called `hello.craw` and write the following:

```
sav 72 ref
sav 105 ref
stp
```

Run this file using `craw hello.craw`

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

## Assembly; What's the deal?

Computers are an amazing product of the information age. Billions of computers are running around the world all doing important, or not-so-important, tasks. But how do the computers know what to do?
Programming languages are used to tell the computer how to work, but the issue is that there are so many to choose from. There's around **8 000** different languages to speak to the device!
How can computers keep up with the different syntax, styles, and methods? The secret? All languages basically boil down to a single language, Machine Code.

**Machine code** is the raw binary that the computer executes. **Assembly languages** are human-readable representations of this machine code.
In many assembly languages, including crawssembly, a single instruction usually corresponds closely to a machine code instruction, compared to a 'higher' language where a line could equal upwards of thousands of lines.

Learning assembly is a great way to internalise how computers work, as by learning assembly you learn how a computer 'thinks'. 

## How Crawssembly compares to higher languages

Higher languages, like C++, Python, and Java, hide how the computer thinks behind layers of abstraction.

In Python you might type `print("Hello World!")` but that doesn't show how the computer splits the function and the string, or the buffer memory storing a copy of "Hello World!", or how the text gets to the screen. Python hides the instructions, Crawssembly exposes them.

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

Which, when calculated, equals 22. So 10110 in binary is the exact same as 22 in Base-10. Every number that can be made in Base-10, can be made in Base-2. There are no gaps.

### Two's complement

Binary numbers like talked about above can only store postitive numbers. Inside computers though, negative numbers are used all the time. So how do we get numbers like -1 or -42?

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

A way to convert a positive value is by inversing all the bit values, then adding 1. We will go into more detail about this when we talk about binary operations.

Another way to think about it is setting the last place value as a negative. Below is an example with Two's Complement being used on an *8-bit value*.

| Place Value | -128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
| ----------- | ---- | -- | -- | -- | - | - | - | - |
| Binary Number | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 0 |

The binary number here, `11001110` is equal to

1 * -127 +  
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

Because the number '100' could be in weird and wonderful base, it's necessary to show what number system is being used if Base-10 has moved aside for a moment.

If the number is in binary, it's usually prefaced with '0b', so if you saw '0b100' then you can be sure that's binary, equal to 4 in Base-10. If you see '0x', that's hexadecimal. So '0x100' would be 256 in Base-10.

## The Basics

> Real assembly languages can interact directly with hardware and operating systems. Mistakes can sometimes crash software or corrupt data. Crawssembly runs inside a virtual machine, allowing you to experiment safely while still learning the same fundamental concepts.

If you've used high-level programming languages before, you'll know that telling a computer how to work is quite different than telling another person what to do.

For example, asking your friend to get a drink from the kitchen is a simple matter, but getting a computer to do this requires you to define a drink, where the kitchen is, the precise steps needed to move to the drink's location, how to pick up the drink, the movements needed to take the drink back to you while not dropping or crushing the cup, etc...

The computer will do what it's told exactly as written to the letter. If you told a computer "Make me a sandwich", you might find the computer think of ways to turn your hair into cheese, to literally convert your body into that of a sandwich. It is an important skill to learn that, when dealing with computers, you must be purely logical. The first truth about computers:  "Ambiguity kills the machine, precision keeps it running."

To use Crawssembly, these practises are no different. 

### Instructions

Every line of Crawssembly is a dedicated instruction. Every line is executed one after another in the exact order the program is written in. The computer reads each line, converts the instruction into a binary number, and executes the command. This is the **Fetch-Decode-Execute** loop, the fundamental process of the computer's Central Processing Unit (CPU)

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

`stp` is a new instuction, as it occupies a new line in the file. `stp` forces the end of the program, but if the end of the file is reached it's not necessary.

Register codes include:

- `r01` is regsiter 1, the *2nd* regsiter available
- `r0a` is register 10, the *11th* register available
- `r10` is register 16, the *17th* register available

As there are 256 registers available in Crawssembly, the codes range from `r00` to `rff`, since the register at 0xff is the *256th* register available

So the example instruction `sav 10 r01` saves the literal value `10` into register 1, which is the *2nd* register available.

Most registers are ready and raring to hold your numbers. But some registers do other functions. The 3 most important ones are `r00`, `ref`, and `rff`. These are the *1st*, *240th*, and *256th* registers respectively.

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

One of the best things about computers is one that often goes unnoticed; dynamicity. A vast amount of tasks would be nigh on impossible if the machines of the modern world were to become entirely hard-coded.
If you hardcoded everything, then there would be little point for computing languages, logical abstraction, and computers as a whole. If you know the result, why build a machine to express the program?

Crawssembly is no exception, dynamically storing values is as easy as removing the literal value in-place for another register

Example

```
sav 10 r01              ; saves 10 to register 1
sav r01 r02             ; saves value of register 1 to register 2
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

Example

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

#### How to use

Calculation using these operations uses the `cal` instruction. All `cal` results get saved to register 1 (`r01`).

Example

```
sav 10 r01              ; saves 10 into register 1
cal add 5 r01           ; adds 5 to the value in register 1
```

This example adds `10` to `5`, resulting in `r01` having a value of `15`

Because of the nature of commands, the first value can be a register or literal, but the second vaule **must** be a register value. `cal add 1 1` would not work. This is why in the example, `10` must be saved to a register.
More information will be given for why this is the case later on.

Like `sav`, the first value can be swapped for a register code.

Example

```
sav 10 r01              ; saves 10 to register 1
sav 20 r02              ; saves 20 to register 2

cal xor r01 r02         ; XORs 10 and 20, result saves to r01
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

This works because AND needs both values to be `1`. Any value not included in the **mask value** is set to `0`, and any `1` value included in the **mas value** is set to `1`.

#### Turning On

To force a particular bit to be `1`, then use OR with the mask value being the bit position you desire. The following example changes `0b01100100` to `0b01100110` using a bit mask of `0b00000010` (i.e. 2).

Example

```
sav 100 r01             ; 0b01100100
cal or 2 r01            ; 0b01100100 OR 0b00000010 = 0b01100110 (i.e. 102)
```

This works because any `1` value present will remain a `1`, so if the mask value contians a `1`, the result will also contain a `1` regardless of the other value.

> You can set multiple bits in this mask, if you really wanted to you could make a mask of 0b11111111 to set **every** bit to `1`!

#### Turning Off

To force a particular bit to be `0`, you must use AND with a mask value of all `1`s apart from the bit you want to set to `0`. The following example changes `0b01100100` to `0b01000100` using a bit mask of `0b11011111` (i.e. -33).

Example

```
sav 100 r01             ; 0b01100100
cal and -33 r01         ; 0b01100100 AND 0b011011111 = 0b01000100 (i.e. 68)
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
| OR | | or || |
| NOT | ~ or ¬ or ! |
| XOR | ^ |
| Left Shift | << |
| Right Shift | >> |

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

Loops are defined using a **label**. A label is a pointer to a instruction number. The line number is stored under this label. Simply state the label value as an instruction.
Loop labels 

Example

```
1       ; creates the label '1' pointing to the first instruction
10      ; creates the label '10' pointing to the second instruction
```

Remember that annoying immediate limit from `sav` and `cal`? With labels the range is bigger, since negative numbers arn't used and the entire instuction is just the number.

**The range for labels is `0 - 65535`**

### Line Numbers

At this point it's a good idea to write Crawssembly with line numbers turned on. This is because, unlike most higher-level languages, the position of the instruction matters almost as much as what the instruction does.

Example

```
1	10
2	20
3	100
4	200
5	1000
6	2000
7	10000
8	20000
```

It's much easier to see that the label `10000` points to line `7` with line numbers! And for longer programs, a must.

For this reason, code examples in this section will be provided with line numbers to aid understanding.

### Removing Labels

To remove a label pointer, use `rmv`.

Example

```
1	10              ; label 10 points to line 1
2	20              ; label 20 points to line 2
3
4	rmv 10          ; removes label 10
5	10              ; label 10 now points to line 5
```

### Jumping

So you've got a label pointing to a line you want to execute many times. How do you get it to run again?

Simply use the **jump** command `jmp`, followed by the pointer label.

Example

```
1	sav 10 r01      ; saves literal value 10 into register 1
2	1               ; creates a label, pointing to line 2
3	cal add 1 r01   ; adds 1 to value inside register 1, saving it to register 1
4	jmp 1           ; jumps to the line number that label 1 is pointing to (i.e. line 2)
5	stp             ; end the program
```

This program adds increases the value in `r01`, being `10` in this example, every loop. This program is an example of an **infinite loop**, it will never reach the `stp` command.

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
1	sav 5 r01       ; saves the value '5' into register 1
2	1               ; creates a label pointer to line 2
3	jmg 1           ; if register 1 holds a >0 value, jump
4	stp             ; the program stops
```

Above is another example of an **infinite loop**, but the loop is only entered if the starting value in `r01` is greater than zero, thanks to the `jmg` command.

Example

```
1	sav 10 r02
2	sav 0 r03
3	1
4	cal add 1 r03
5	sav r01 r03
6	cal add -1 r02
7	sav r01 r02
8	jmg 1
9	stp
```

Let's read through each line in detail:

1. `sav 10 r02`: This saves the literal value `10` into register 2. This value is used for the number of times the loop will run. In this case, 10 times.
2. `sav 0 r03`: This is the initial value of the 'increment' register. This value is increased by `1` every loop cycle.
3. `1`: This is the loop label, it points to line 3.
4. `cal add 1 r03`: This adds `1` to the value inside `r03`, that being the 'increment' value.
5. `sav r01 r03`: This saves the +1 calculation step back into `r03`.
6. `cal add -1 r02`: This minuses `1` from the loop count, indicating that a loop has taken place.
7. `sav r01 r02`: This line updates `r02`, where the loop count is stored.
8. `jmp 1`: Because the -1 step happened, the result is still stored in `r01`. If this >=0, the loops doesn't run.
9. `stp`: The program ends, resulting in 10 loops being executed.

### Activity: Letter Loops

Write a program that loops over every uppercase letter and prints it to the screen

> Hint: Uppercase letters start at `65`. Use [this page](https://www.asciitable.com/) for reference!

### Advanced Activity: Double Loops

Update your 'Letter Loops' activity program to contain a **nested loop** that prints
`AA AB AC AD AE AF AG` etc... `AX AY AZ BA BB BC BD` etc... `ZX ZY ZZ`

> Hint: Both loops take similar forms, but take care to not mix register values together, seperate the 1st loop from the 2nd loop clearly, and reset the inner-loop values in the outer loop.

### More about jumping

Sometimes you don't want a label to be used. If you want to jump to a line once and you know where it is, don't bother with making a label pointer!

If you know where you want the Crawssembly executioner to jump to, use `fgo` (Force GO).

Example

```
1	sav 10 r01      ; saves value 10 to register 1
2	cal add 1 r01   ; adds 1 to value inside register 1
3	fgo 2           ; jumps directly to line 2
```

This is an example of using `fgo` for an infinite loop.

#### Dynamic `fgo`

Keen-eyed readers will have noticed that line numbers *dont't start at 0* like registers do. This is because `fgo 0` doesn't go to the first line number, rather `fgo` jumps to whatever line number is stored in `r01`.

Example

```
1	sav 5 r01
2	fgo 0
3	1 
4	jmp 1
5
6	stp
```

`fgo 0` above skips over the evil infinite loop, since `r01` contains `5`.

## Branching

Let's say you wanted to only run a piece of code if a certain condition was met. You could use labels and jumps in what is called the **Jump-Then-Execute** format.

```
1	sav 7 r02               ; saves 7, which is where execution jumps to after label 1 is defined
2	1                       ; label 1 points to line 2
3	sav r02 r01             ; readies the line number into register 1, so 'fgo' can jump past the code below
4	fgo 0                   ; the first execution jumps to line 7, the second execution jumps to line 5
5	2                       ; label 2 points to line 5
6	jmp 2                   ; jumps back to the label 2 pointer, an infinite loop
7
8	sav 50 r01              ; sets '50' as the test value
9	sav 5 r02               ; sets the next fgo line number to '5', if the check succeeds this executes the instructions under label 1
10	jmz 1                   ; if the test value is zero, activate the infinite loop
```

JTE is complex, and luckily, there is a better way. You can use the `if` group of commands to execute code depending on the value in `r01`:

- `ifg`: Continues IF Greater (`r01` > 0)
- `ifz`: Continues IF Zero (`r01` = 0)
- `ifl`: Continues IF Less (`r01` < 0)

The `if` group also works on label pointers. To end the `if` command, use `rmv` on the label.

Example

```
1	sav 50 r01              ; sets '50' as the test value
2	ifz 1                   ; if branch using label 1 for test value = 0
3	2                       ; label 2 points to 3
4	jmp 2                   ; jumps back to label 2, an infiniate loop
5	rmv 1                   ; ends the if branch using label 1
```
This format is much cleaner than JTE.

### Activity: Even Looper

Write a program that loops from 0 to 10, and outputs 'e' if the number is even, and 'o' if the number is odd.

### Advanced Activity: Even Checker

Edit your program for 'Even Looper', so that instead of outputting 'e' or 'o', it outputs the number if it's even, and doesn't output anything if the number is odd.

## Devices

`io` is the largest group of commands, used to interact with data outside of the CPU, such as speakers, keyboards, and storage. All `io` commands take a register value as input, never an immediate value.

Because `io` comamnds are dependant on other devices, commands might fail if there is an issue. This is why register `ree` is used for *error codes*

| Code | Name | Description |
| ---- | ---- | ----------- |
| 0 | IO_OK | Operation completed successfully |
| 1 | IO_INVALID_COMMAND | Invalid device command |
| 2 | IO_BAD_VALUE | Value outside valid range |
| 3 | IO_UNAVAILABLE | Device unavailable |

## Storage

Computers use two different systems for storing data. **Memory** and **Disk**.

- Memory: A fast way to temporarily store data. It is where active data which is being used is stored. Memory is cleared when the computer resets, called **volitile**.
- Disk: Long-term storage, used for files, apps, operating systems, and more. The disk is usually slower than memory, which is why only long-term data is stored. Because data is kept, it is called **non-volitile**.

Registers are the fastest type of data storage, but the relative data they can hold is tiny. Crawssembly registers can hold a little over a kilobyte in register storage, but can access around 17 million kilobytes in memory!

| Storage Medium | Storage Length | Size | Speed |
| -------------- | -------------- | ---- | ----- |
| Registers | Very Short | Very Small | Very Fast |
| Memory | Short | Large | Fast |
| Disk | Indefinite | Very Large | Slow |

#### Addresses

Much like registers, every cell in storage has a number based on it's position. This number is called the **address**.

Like registers, the first cell has an address of 0, the second cell has an address of 1, etc...

Storage addresses are commonly given in **hexadecimal** format, such as `0xF58`. This is because hex perfectly matches how computer hardware is layed out, while keeping the address short enough for humans to read.

Because registers can hold values between `- 2^31` and `2^31 - 1` (This is the *signed 32-bit limit*), the largest address is **0xFFFFFFFF**. That's over 4 billion addresses available!

### Memory

You've probably heard of RAM, or Random Access Memory. RAM is a type of memory used by computers for large amounts of data.
It's called 'Random Access' because the computer can read and write to any piece of data in any order. This makes RAM fast and efficient compared to older storage methods.

Like registers, memory is empty on startup. To write to a memory address, you must use a command in the `io` group.

For using mamory, we must use `io mem`.

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

`io mem addr` sets the active memory address that Crawssembly is considering. All `io mem` commands act on this address.

`io mem write` sets the value of the active memory address to whatever is in the input register.

`io mem read` saves the value of the active memory address to the given register.

#### Activity: Read/Write Cycle

Write a program to write the codes for 'A', 'B', and 'C' to memory, then read them back from memory in reverse order.

#### Advanced Activity: Save Arrays

Write a program to loop over the numbers 1 to 10, saving each number to it's own memory address.

### Disk

Disk space is used for data you don't want to lose. These can be results of a long program, long files, photos, videos, soundbytes, etc...

Disk commands follow the same form as memory commands.

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

`io disk addr` sets the active disk address that Crawssembly is considering. All `io disk` commands act on this address.

`io disk write` sets the value of the active disk address to whatever is in the input register.

`io disk read` saves the value of the active disk address to the given register.

#### Activity: Long Live The Data

Write one program that saves a piece of data to disk. Then write a new program to read that data from the disk, and output it.

#### Advanced Activity: My Long List

Write one program to loop over the numbers from 0 to 10, saving each to a disk address. Then write a new program to read these vaules in reverse order, and output each one.

### What high-level programs do

High-level programs use memory all the time. When you type somthing like `5 + 10` the computer has no clue how large that result will be without executing it. So the program allocates memory for `5`, `10`, and the result.

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

## Input

If you've ever learnt a higher programming langauge, chances are they taught you how to get user input very early on. This is sensible, as the user gets near-instant gratification from adding thier own name to a greeting program, or telling the computer how old they are to get a semi-personalised message.
So why is user input so far in the learning path for Crawssembly?

Because input uses the keyboard and mouse, there is a hidden layer between what you type, and that data being processed by the CPU. Much like storage, the keyboard is a seperate device to the core computer. Because of this, input handling builds upon what you already know about more basic data manipulation.

### The Keyboard

Crawssembly provides direct keyboard access using `io keyboard`. The main command in this group is `io keyboard poll`, which gets the last key pressed as a key code into the input register.

Key Codes:
- 0: No key pressed
- -1: up arrow
- -2: down arrow
- -3: left arrow
- -4: right arrow
- 27: Escape
- other: ASCII code ([See this chart for more](https://www.asciitable.com/))

Example

```
1                               ; Label pointing to line 1
io keyboard poll ref            ; Gets the last key and stores into the output register
cal add -27 ref                 ; Finds key - 27, the result is saved to register 1
jmg 1                           ; continues the loop if the key code is greater than 26
jml 1                           ; continues the loop if the key code is less than 27
stp                             ; stops the program if the key code is exactlly 27
```

This program shows the last key printed by outputting to `ref`, and ends the loop when `Esc` is pressed.

> Remember, `ref` outpus the [ASCII code](https://www.asciitable.com/) of the key pressed, not the real raw value of that key. Pressing Enter won't print 'Enter' to the screen!

### The Mouse

Almost all basic mice output 4 streams of data:
- Position
- Left Button
- Middle Button
- Right Button

The `io mouse` command group captures mouse events using 3 commands:
- `io mouse x`: extracts current mouse X positon, relative to the screen's top-left corner
- `io mouse y`: extracts current mouse Y position, relative to the screen's top-left corner
- `io mouse btn`: creates a **bitfield** for the left, right, and middle buttons in the order.

#### Position

The mouse's position can be extracted using `io mouse x` and `io mouse y`.

Example

```
io mouse x r01
io mouse y r02
```

This example stores the (x, y) location as (`r01`, `r02`).

#### Button Bitfield

A **bitfield** is a single binary number which encodes multiple values. Instead of using a single bit for if the left mouse button is being pressed, another for the right button being pressed, and another again for the middle button, we can merge these 3 numbers into a single number.

| Left Button | Right Button | Middle Button | Bitfield | Base-10 Number |
| ----------- | ------------ | ------------- | -------- | -------------- |
| Not Pressed | Not Pressed | Not Pressed | 0b000 | 0 |
| Not Pressed | Not Pressed | Pressed | 0b001 | 1 |
| Not Pressed | Pressed | Not Pressed | 0b010 | 2 |
| Not Pressed | Pressed | Pressed | 0b011 | 3 |
| Pressed | Not Pressed | Not Pressed | 0b100 | 4 |
| Pressed | Not Pressed | Pressed | 0b101 | 5 |
| Pressed | Pressed | Not Pressed | 0b110 | 6 |
| Pressed | Pressed | Pressed | 0b111 | 7 |

You can extract each button using bit masks.

Example

```
io mouse btn r01
cal and 1 r01
```

This program extracts the *left mouse* button into `r01`.

> We covered bit masks earlier on when talking about binary operations.

Bit masks can be used to expand the mouse bitfield into seperate registers.

Example

```
io mouse btn r04

io cal and 1 r04

io cal and 2 r04
sav r01 r02

io cal and 4 r04
sav r01 r03
```

This example unpacks the bitfield, stored in `r04` into `r01` (Left Button), `r02` (Right Button), and `r03` (Middle Button)

#### Activity: Clicking Away

Write a program to show "Click" when a mouse button is pressed.

#### Advanced Activity: Mouse Dump

Write a program that shows "L" if the left button is pressed, "R" if the right button is being pressed, and "M" if the middle button is being pressed.

## Output

Inputting values into your program is all well and good, but it's pretty useless if you can't get anythin meaningful back. Crawssembly can control both your screen, and your speakers.

### The Screen

The screen is a great all-purpose viewing device. Not only can it show text, it can show graphics and symbols.

> Crawssembly's screen is contained within the terminal. This is to protect your screen's contents outside of the terminal, and to allow outputs to be easily seen.

#### Text

Before now, program outputs have used the `ref` register to send raw ASCII codes to the screen. While useful for debugging, this isn't the best way to get a program output. This uses the `io text` group of commands.

- `io text char`: Shows the ASCII code stored in the input register to the screen, the more *idiomatic* version of using `ref`.
- `io text int`: Shows the actual value inside the input register.
- `io text newline`: Shorthand for moving the text cursor to a new line.
- `io text hex`: Shows the hex value stored inside the input register.

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

















*Crawssembly is a product of CRAW SYSTEMS (C) 2026*

