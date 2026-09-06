# Wodey-8

A simple 8-bit CPU emulator of my own design

## Details

This program is meant to run a simple, 8-bit CPU emulation with a 256B RAM. The instruction set, register codes and flags are defined as below.

## Registers

The emulator provides a set of 8 general purpose registers, as well as a program counter and instruction register.

- `r1` - with code `0b00000000`, `0x00`,
- `r2` - with code `0b00000001`, `0x01`,
- `r3` - with code `0b00000010`, `0x02`,
- `r4` - with code `0b00000011`, `0x03`,
- `r5` - with code `0b00000100`, `0x04`,
- `r6` - with code `0b00000101`, `0x05`,
- `r7` - with code `0b00000110`, `0x06`,
- `r8` - with code `0b00000111`, `0x07`

### Special registers

The emulator also provides two special registers, that are not meant to be used as a general purpose registers.

- `pc` - program counter register (with code `0b10000000`, `0x80`). It's value determines the address of the next fetched instruction.
- `ir` - instruction register (with code `0b10000001`, `0x81`). It's value stores the currently decoded instruction.

## Flags

The emulator currently provides three flags, whose values can be set via specific instructions.

- `z` - zero. Determines whether the current operation's result is zero.
- `n` - negative. Determines whether the current operation's result is negative.
- `of` - overflow. Determines whether the current operation resulted in an overflow.

Those flags can be used to create branching using specific conditional instructions.

## IO ports

The emulator currently supports only one IO device, namely the terminal.

- `0` - the IO device code of the terminal.

## Instructions

The emulator provides a set of 27 instructions that it can execute.

### Arithmetic instructions

Those instructions are meant to be used to perform mathematic operations.

- `add [dest] [source] [other]`
  - performs the addition of values stored in registers specified with codes `[source]` and `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z`, `n` and `of` according to the result of the operation.
- `sub [dest] [source] [other]`
  - performs the subtraction of values stored in registers specified with codes `[source]` and `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z`, `n` and `of` according to the result of the operation.
- `mul [dest] [source] [other]`
  - performs the multiplication of values stored in registers specified with codes `[source]` and `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z`, `n` and `of` according to the result of the operation.
- `div [dest] [source] [other]`
  - performs the division of values stored in registers specified with codes `[source]` and `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z`, `n` and `of` according to the result of the operation.
  - in case of division by zero, the CPU crashes without performing the operation.
- `and [dest] [source] [other]`
  - performs a bitwise AND of values stored in registers specified with codes `[source]` and `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the result of the operation.
- `or [dest] [source] [other]`
  - performs a bitwise OR of values stored in registers specified with codes `[source]` and `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the result of the operation.
- `xor [dest] [source] [other]`
  - performs a bitwise XOR of values stored in registers specified with codes `[source]` and `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the result of the operation.
- `not [dest] [source]`
  - performs a bitwise NOT of a value stored in the register specified with code `[source]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the result of the operation.
- `shl [dest] [source] [other]`
  - performs a bitshift to the left of the value in register specified with code `[source]` by the value in the register specified with code `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z`, `n` and `of` according to the result of the operation.
- `shr [dest] [source] [other]`
  - performs a bitshift to the right of the value in register specified with code `[source]` by the value in the register specified with code `[other]`. Stores the result in the register specified with code `[dest]`.
  - sets the flags `z`, `n` and `of` according to the result of the operation.
- `cmp [value]`
  - sets the flags `z` and `n` according to the value in register specified with code `[value]`.

### Memory instructions

Those instructions are meant to be used to manage memory - both RAM and registers

- `load [dest] [addr]`
  - loads the byte stored at the `[addr]` in RAM into the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the loaded value.
- `loadr [dest] [source]`
  - loads the byte stored at the address in RAM provided by the value in the register specified with code `[source]` into the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the loaded value.
- `store [source] [addr]`
  - stores the byte stored in the register specified with code `[source]` at the `[addr]` location in RAM.
  - sets the flags `z` and `n` according to the loaded value.
- `storer [source] [dest]`
  - stores the byte stored in the register specified with code `[source]` at the address in RAM provided by the value in the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the loaded value.
- `move [dest] [source]`
  - copies the byte from the register specified with code `[source]` into the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the loaded value.
- `moveim [dest] [value]`
  - copies the byte from the `[value]` into the register specified with code `[dest]`.
  - sets the flags `z` and `n` according to the loaded value.
- `cmovz [dest] [source]`
  - if the `z` flag is set, copies the byte from register specified with code `[source]` into the register specified with code `[dest]`, otherwise does nothing.
  - if the `z` flag is set, sets the flags `z` and `n` according to the loaded value.
- `cmovn [dest] [source]`
  - if the `n` flag is set, copies the byte from register specified with code `[source]` into the register specified with code `[dest]`, otherwise does nothing.
  - if the `n` flag is set, sets the flags `z` and `n` according to the loaded value.
- `cmovof [dest] [source]`
  - if the `of` flag is set, copies the byte from register specified with code `[source]` into the register specified with code `[dest]`, otherwise does nothing.
  - if the `of` flag is set, sets the flags `z` and `n` according to the loaded value.

### IO instructions

Those instructions are meant to be used to manage IO operations.

- `in [port]`
  - loads the value read from the IO device specified with code `[port]` into the `r1` register.
  - sets the flags `z` and `n` according to the loaded value.
- `out [port]`
  - loads the value stored in `r1` register into the IO device specified with code `[port]`.
  - sets the flags `z` and `n` according to the loaded value.

### Flow control instructions

Those instructions are meant to be used to manage the flow control of the program.

- `halt`
  - freezes the execution indefinitely.
- `jmp [addr]`
  - sets the `pc` register's value to point to the `[addr]` address.
- `jz [addr]`
  - if the flag `z` is set, sets the `pc` register's value to point to the `[addr]` address, otherwise does nothing.
- `jn [addr]`
  - if the flag `n` is set, sets the `pc` register's value to point to the `[addr]` address, otherwise does nothing.
- `jof [addr]`
  - if the flag `of` is set, sets the `pc` register's value to point to the `[addr]` address, otherwise does nothing.
