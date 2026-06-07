# Crawssembly: A beginner entry-point to Assembly Languages

![Crawssembly Banner](https://www.dropbox.com/scl/fi/e4fhcba8zkgw2youvai8s/Crawssembly.png?rlkey=x1xojmcn29z9joxnmpb09iums&st=lxansr9a&raw=1)

## Why does Crawssembly exist?

Crawssembly is an educational assembly-like language designed to teach how computers work from the ground up.

Traditional assembly languages such as x86 and ARM are powerful, but often overwhelming for beginners.
Crawssembly removes much of the complexity while preserving the core ideas:
- Registers
- Memory
- Arithmetic
- Program Flow
- Input and Output

The goal is to help you understand what high-level languages are really doing behind the scenes.

## Binary

Getting your head around binary (also known as Base-2) is perhaps the most important skill when working with low-level computing. However, the need for binary thinking has diminished with the massive rise of high-level languages like Python, Java, and Swift. A programmer can create amazing tools, games, and projects without ever actually needing to know what binary is.
For those who don't know what binary is, this guide go through the basics, but for more information, Wikipedia is always a great place to start!

### What is Bianry?

Binary, simply, is just another way to use numbers. We humans use the Base-10 system, meaning that we have 10 seperate symbols for counting (i.e. 0, 1, 2, 3, 4, 5, 6, 7, 8, and 9). You can make any number using enough of these digits. Computers don't have the luxury of 10 digits though.

At their core, computers are a bunch of transistors, very small switches that can only be in two states; On or Off. This is because two-state switches are very realiable and can change state quickly.

Think about a car's gearstick, it would be easier to use if there were only 2 gears rather than the 6 or more in reality.

### So how does binary get used?
It's basically the same as normal counting. In Base-10, when you reach '9' and you want to go up again, you have to use the next number in the place value, that being 10. Every time you reach the end, you need to reset the count and use the next place value.

9 + 1 becomes 10

99 + 1 becomes 100

999 + 1 becomes 1000

etc...

It's clear that place value works in 10's, the number '8' could represent 8 1's, 8 10's 8 100's, 8 1000's etc... depending on where it's place in the number.

| Place Value | 10 000 | 1 000 | 100 | 10 | 1 |
| ----------- | ------ | ----- | --- | -- | - |
|   Number    |   8    |   3   |  0  | 7  | 2 |

The example above shows how the number 83072 is equal to

8 * 10 000 +

3 * 1 000 +

0 * 100 + 

7 * 10 +

2 * 1

For binary, only 2 digits are used. These are 0 for 'Off', and 1 for 'On'. Because only 2 symbols are used, place value works based on 2, not 10.

So a '1' could represent 1 1's, 1 2's, 1 4's, 1 8's, 1 16's, etc... depending on where it is in the number.

| Place Value | 16 | 8 | 4 | 2 | 1 |
| ----------- | -- | - | - | - | - |
|   Binary    | 1  | 0 | 1 | 1 | 0 |

We can see that the binary number 10110 is the same as

1 * 16 +

0 * 8 +

1 * 4 +

1 * 2 +

0 * 1

Which, when calcualted, equals 22. So 10110 in Binary is the exact same as 22 in Base-10

A counting example is provided below to show what counting in binary looks like

| Base-10 | Binary |
| ------- | ------ |
|    0    |    0   |
|    1    |    1   |
|    2    |   10   |
|    3    |   11   |
|    4    |   100  |
|    5    |   101  |
|    6    |   110  |
|    7    |   111  |
|    8    |  1000  |
|    9    |  1001  |
|    10   |  1010  |
|    11   |  1011  |
|    12   |  1100  |
|    13   |  1101  |
|    14   |  1110  |
|    15   |  1111  |


High level languages tend to spoil the user with many programs, functions, and data types. The computer doesn't see it that way. From the silicon's perspective, everything is binary numbers. A string such as "Hello World!" is actually a long binary number, the computer doesn't know what 'H' is, only it's binary representation.

Every photo, song, text, and piece of code is actually just a long list of 1's and 0's.

## The Basics

> A small warning before beginning; Because assembly works directly with the computer's processor, you can easily break something if working alongside important data, such as an Operating System like Windows. For this reason, Crawssembly is kept within reasonable hardware limits. Removing these limits, while condoned in the name of learning and curiosity, should only be done if you are confident that your programs won't overwrite data that really shouldn't be overwritten.

If you've used high-level programming languages before, you'll know that telling a computer how to work is quite different than telling another person what to do.

For example, asking your friend to get a drink from the kitchen is a simple matter, but getting a computer to do this requires you to define a drink, where the ktichen is, the precise steps needed to move to the drink's location, how to pick up the drink, the movements needed to take the drink back to you while not dropping or crushing the cup, etc...

The computer will do what it's told exactlly as written to the letter. If you told a computer "Make me a sandwich", you might find the computer think of ways to turn your skin into bread to literally convert your body into that of a sandwich. It is an important skill to learn that, when dealing with computers, you must be purely logical; ambiguity kills the machine.

### Registers

The most basic instruction is storing values. 





















*Crawssembly is a product of CRAW SYSTEMS (C) 2026*
