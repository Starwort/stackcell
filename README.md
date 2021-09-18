# StackCell

*StackCell* is an esoteric programming language which consists of two stacks of integers and some commands to manipulate them. You also get a single cell which can be written to and read from as much as you like.

*StackCell* is turing complete: `bf_to_cel.py` can compile any program written in [the turing tarpit BF](https://esolangs.org/wiki/Brainfuck) into *StackCell*.

The interpreter can either take a filename as an argument, or programs can be entered into an interactive interface.

Recommended file extension: `.cel`

## The stack

The stack is a list of (wrapping) unsigned bytes. The stack is initially empty, and attempting to read elements from an empty stack will give a zero.

## The Cell

The cell contains a single (wrapping) unsigned byte. Initially it contains the value 0, but can be written to to overwrite its value. When it is read from, its value is not cleared.

## Commands

| Command                 | Effect                                                                                                                                                                       | Example                 |
| ------------------------| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `'`                     | Pushes a character to the primary stack                                                                                                                                      | `'a`                    |
| `"`                     | Pushes a string to the primary stack[<sup>[1]</sup>](#footnote-1)                                                                                                            | `"!dlrow olleH"`        |
| `#`                     | Pushes a byte to the primary stack[<sup>[2]</sup>](#footnote-2)                                                                                                              | `#42`                   |
| Numeral from `1` to `9` | Skips the number of characters specified                                                                                                                                     | `'0@-:?5'0+;.:[:'0+;:]` |
| `[`                     | Starts a zero-sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                                     | `:[;:]`                 |
| `]`                     | Ends a zero-sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                                       | `:[;:]`                 |
| `(`                     | Starts a non-zero sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                                 | `:(;:)`                 |
| `)`                     | Ends a non-zero sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                                   | `:(;:)`                 |
| `.`                     | Terminates the program                                                                                                                                                       | `.`                     |
| `:`                     | Duplicates the top value of the primary stack                                                                                                                                | `#42:`                  |
| `{`                     | Moves the top value of the primary stack to the cell                                                                                                                         | `#42{`                  |
| `}`                     | Copies the value of the cell on to the primary stack                                                                                                                         | `}`                     |
| `` ` ``                 | Discards the top value of the primary stack with no other side-effects                                                                                                       | ``#42` ``               |
| `x`                     | Swaps the top two values of the primary stack                                                                                                                                | `'a'bx`                 |
| `X`                     | Swaps the primary and secondary stacks                                                                                                                                       | `X`                     |
| `!`                     | Logically negates the top value of the primary stack[<sup>[4]</sup>](#footnote-4)                                                                                            | `#42!`                  |
| `<`                     | Compares the top two values[<sup>[6]</sup>](#footnote-6) of the primary stack[<sup>[5]</sup>](#footnote-5) and return 1 if the first is less than the second, otherwise 0    | `#42#42<`               |
| `>`                     | Compares the top two values[<sup>[6]</sup>](#footnote-6) of the primary stack[<sup>[5]</sup>](#footnote-5) and return 1 if the first is greater than the second, otherwise 0 | `#42#42>`               |
| `=`                     | Compares the top two values of the primary stack[<sup>[5]</sup>](#footnote-5) and return 1 if they are equal, otherwise 0                                                    | `#42#42=`               |
| `+`                     | Adds the top two values of the primary stack[<sup>[5]</sup>](#footnote-5)                                                                                                    | `#42#42+`               |
| `-`                     | Subtracts the top two values[<sup>[6]</sup>](#footnote-6) of the primary stack[<sup>[5]</sup>](#footnote-5)                                                                  | `#42#42-`               |
| `*`                     | Multiplys the top two values of the primary stack[<sup>[5]</sup>](#footnote-5)                                                                                               | `#42#42*`               |
| `/`                     | Divides, using integer division, the top two values[<sup>[6]</sup>](#footnote-6) of the primary stack[<sup>[5]</sup>](#footnote-5)                                           | `#0C#3/`                |
| `%`                     | Take the modulus of the top two values of the primary stack[<sup>[6]</sup>](#footnote-6) of the primary stack[<sup>[5]</sup>](#footnote-5)                                   | `#42#42%`               |
| `^`                     | Bitwise-XORs the top two values of the primary stack[<sup>[5]</sup>](#footnote-5)                                                                                            | `#42#42^`               |
| `&`                     | Bitwise-ANDs the top two values of the primary stack[<sup>[5]</sup>](#footnote-5)                                                                                            | `#42#42&`               |
| `\|`                    | Bitwise-ORs the top two values of the primary stack[<sup>[5]</sup>](#footnote-5)                                                                                             | `#42#42\|`              |
| `~`                     | Bitwise-negates the top value of the primary stack                                                                                                                           | `#42#42~`               |
| `?`                     | Skips the next instruction if the top value of the primary stack is zero (consumes the top value of the primary stack)                                                       | `#42::?+`               |
| `;`                     | Consumes, and outputs as ASCII, the top value of the primary stack                                                                                                           | `#42;`                  |
| `@`                     | Inputs a character from the keyboard and pushes it to the primary stack                                                                                                      | `@`                     |

## Examples

See [the samples](./samples).

## Footnotes

<dl>
    <dt id="footnote-1">1</dt>
    <dd>Strings are pushed to the stack as a sequence of bytes, in the order the characters are written. This means that to print them correctly, they should be entered in reverse.</dd>
    <dt id="footnote-2">2</dt>
    <dd>Values pushed to the stack with <code>#</code> must be in two-digit hexadecimal form; however it is case-insensitive.</dd>
    <dt id="footnote-3">3</dt>
    <dd>Loops started with <code>[</code> are executed until the stack is empty or the byte at the top of the stack is zero. Loops started with <code>(</code> are executed until the stack is empty or the byte at the top of the stack is non-zero. Loops may be nested. Loops may be skipped entirely. Both forms of loop consume the top value of the stack when determining whether to run.</dd>
    <dt id="footnote-4">4</dt>
    <dd>That is to say, transform the top value of the stack such that any non-zero value becomes zero, and zero becomes one</dd>
    <dt id="footnote-5">5</dt>
    <dd>This operation consumes the top two values of the stack, and places a new one containing the result</dd>
    <dt id="footnote-6">6</dt>
    <dd>The top value of the stack (i.e. the one pushed later) is considered the left hand side of the operation</dd>
</dl>
