# StackCell

*StackCell* is an esoteric programming language which consists of a stack of integers and some commands to manipulate it. You also get a single cell which can be written to and read from as much as you like.

The interpreter can either take a filename as an argument, or programs can be entered into an interactive interface.

Recommended file extension: `.cel`

## The stack

The stack is a list of unsigned bytes. The stack is initially empty, and attempting to read elements from an empty stack will give a zero.

## The Cell

The cell contains a single unsigned byte. Initially it contains the value 0, but can be written to to overwrite its value. When it is read from, its value is not cleared.

## Commands

| Command                                                                                                                                                                                                               | Effect                                                                                                                                                               | Example                 |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `'`                                                                                                                                                                                                                   | Pushes a character to the stack                                                                                                                                      | `'a`                    |
| `"`                                                                                                                                                                                                                   | Pushes a string to the stack[<sup>[1]</sup>](#footnote-1)                                                                                                            | `"!dlrow olleH"`        |
| `#`                                                                                                                                                                                                                   | Pushes a byte to the stack[<sup>[2]</sup>](#footnote-2)                                                                                                              | `#42`                   |
| Numeral from `1` to `9`                                                                                                                                                                                               | Skips the number of characters specified                                                                                                                             | `'0@-:?5'0+;.:[:'0+;:]` |
| `[`                                                                                                                                                                                                                   | Starts a zero-sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                             | `:[;:]`                 |
| `]`                                                                                                                                                                                                                   | Ends a zero-sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                               | `:[;:]`                 |
| `(`                                                                                                                                                                                                                   | Starts a non-zero sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                         | `:(;:)`                 |
| `)`                                                                                                                                                                                                                   | Ends a non-zero sentinel loop[<sup>[3]</sup>](#footnote-3)                                                                                                           | `:(;:)`                 |
| `.`                                                                                                                                                                                                                   | Terminates the program                                                                                                                                               | `.`                     |
| `:`                                                                                                                                                                                                                   | Duplicates the top value of the stack                                                                                                                                | `#42:`                  |
| `{`                                                                                                                                                                                                                   | Moves the top value of the stack to the cell                                                                                                                         | `#42{`                  |
| `}`                                                                                                                                                                                                                   | Copies the value of the cell on to the stack                                                                                                                         | `}`                     |
| `` ` `` | Discards the top value of the stack with no other side-effects                                                                                                                                  | ``#42` `` |
| `x`                                                                                                                                                                                                                   | Swaps the top two values of the stack                                                                                                                                | `'a'bx`                 |
| `!`                                                                                                                                                                                                                   | Logically negates the top value of the stack[<sup>[4]</sup>](#footnote-4)                                                                                            | `#42!`                  |
| `<`                                                                                                                                                                                                                   | Compares the top two values[<sup>[6]</sup>](#footnote-6) of the stack[<sup>[5]</sup>](#footnote-5) and return 1 if the first is less than the second, otherwise 0    | `#42#42<`               |
| `>`                                                                                                                                                                                                                   | Compares the top two values[<sup>[6]</sup>](#footnote-6) of the stack[<sup>[5]</sup>](#footnote-5) and return 1 if the first is greater than the second, otherwise 0 | `#42#42>`               |
| `=`                                                                                                                                                                                                                   | Compares the top two values of the stack[<sup>[5]</sup>](#footnote-5) and return 1 if they are equal, otherwise 0                                                    | `#42#42=`               |
| `+`                                                                                                                                                                                                                   | Adds the top two values of the stack[<sup>[5]</sup>](#footnote-5)                                                                                                    | `#42#42+`               |
| `-`                                                                                                                                                                                                                   | Subtracts the top two values[<sup>[6]</sup>](#footnote-6) of the stack[<sup>[5]</sup>](#footnote-5)                                                                  | `#42#42-`               |
| `*`                                                                                                                                                                                                                   | Multiplys the top two values of the stack[<sup>[5]</sup>](#footnote-5)                                                                                               | `#42#42*`               |
| `/`                                                                                                                                                                                                                   | Divides, using integer division, the top two values[<sup>[6]</sup>](#footnote-6) of the stack[<sup>[5]</sup>](#footnote-5)                                           | `#0C#3/`                |
| `%`                                                                                                                                                                                                                   | Take the modulus of the top two values of the stack[<sup>[6]</sup>](#footnote-6) of the stack[<sup>[5]</sup>](#footnote-5)                                           | `#42#42%`               |
| `^`                                                                                                                                                                                                                   | Bitwise-XORs the top two values of the stack[<sup>[5]</sup>](#footnote-5)                                                                                            | `#42#42^`               |
| `&`                                                                                                                                                                                                                   | Bitwise-ANDs the top two values of the stack[<sup>[5]</sup>](#footnote-5)                                                                                            | `#42#42&`               |
| `\|`                                                                                                                                                                                                                  | Bitwise-ORs the top two values of the stack[<sup>[5]</sup>](#footnote-5)                                                                                             | `#42#42|`               |
| `~`                                                                                                                                                                                                                   | Bitwise-negates the top value of the stack                                                                                                                           | `#42#42~`               |
| `?`                                                                                                                                                                                                                   | Skips the next instruction if the top value of the stack is zero (consumes the top value of the stack)                                                               | `#42::?+`               |
| `;`                                                                                                                                                                                                                   | Consumes, and outputs as ASCII, the top value of the stack                                                                                                           | `#42;`                  |
| `@`                                                                                                                                                                                                                   | Inputs a character from the keyboard and pushes it to the stack                                                                                                      | `@`                     |

## Examples

### Hello world!

`#0A"!dlrow olleH":[;:].`

This program prints `Hello world!` to the console.

| Instruction      | Description                                                                                                                  | Stack             | Output |
| ---------------- | ---------------------------------------------------------------------------------------------------------------------------- | ----------------- | ------ |
| `#0A`            | Pushes a newline (`0x0A`) to the stack                                                                                       | `\n`              |
| `"!dlrow olleH"` | Pushes the string "Hello world!" to the stack, in reverse order                                                              | `Hello world!\n`  |
| `:`              | Duplicate the top value of the stack (`0x48`) so that there is still a copy on the stack after the `[`                       | `HHello world!\n` |
| `[`              | Begin the print loop, consuming the top value of the stack (`0x48` during the first iteration)                               | `Hello world!\n`  |
| `;`              | Consume the top value of the stack (`0x48` during the first iteration) and output it as ASCII                                | `ello world!\n`   | `H`    |
| `:`              | Duplicate the top value of the stack (`0x65`) so that there is still a copy on the stack after the `[` of the next iteration | `eello world!\n`  | `H`    |
| `]`              | Return to the beginning of the loop and repeat                                                                               | `eello world!\n`  | `H`    |

### Reverse input

``(@:#0A-)`:[;:]#0A;.``

This program reads a line of input from the user, and prints it in reverse.

- `(`: Begins the input loop
- `@`: Reads a character from the keyboard and pushes it onto the stack
- `:`: Duplicates the top inputted character to allow for comparison
- `#0A`: Pushes a newline (`0x0A`) to the stack
- `-`: Subtracts the top two values of the stack (it doesn't matter which way round, as the result is either zero or non-zero)
- `)`: Returns to the beginning of the loop, if the last character wasn't a newline.
- `` ` ``: Discard the newline at the end of the user's input
- `:[;:]`: Print the stack, as before. This will print the user's string, but in reverse
- `#0A;.`: Print the trailing newline, then stop

### Truth machine

`'0@-:[:'0+;:]'0+;.`

As per [the esolang page](https://esolangs.org/wiki/Truth-machine), this program accepts an input of 1 or 0, and then exhibits one of two behaviours:

- If the input is 1, it prints 1 repeatedly, forever
- If the input is 0, it prints 0 once, then exits

<!-- -->

- `'0`: Pushes an ASCII `'0'` to the stack
- `@`: Reads a character and pushes it onto the stack (`'0'` or `'1'`)
- `-`: Subtract the `'0'` we pushed before from the `'0'` or `'1'` we just read (this gives us a numeral result of `0` or `1`, respectively)
- `:`: Duplicate the top value of the stack (`0` or `1`) so that there is still a copy on the stack after the `[`
- `[`: Begin the loop, consuming the top value of the stack (`0` or `1`)
- `:`: Duplicate the top value (`1`) of the stack, so that there is still a copy on the stack after transforming and printing it
- `'0+`: Add an ASCII `'0'` to the top value of the stack (`1`) to get the ASCII `'1'`
- `;:]`: Print the top value of the stack (`'1'`) and return to the beginning of the loop
- `'0+;.`: Transform the `0` back to `'0'` and print it, then stop

### Quine

I was unable to write a quine. Probably just due to me being bad at that. I got close a couple of times and added more language features (such as cell storage and jumps) but I couldn't make it work. If you manage to write a quine, please submit a pull request!

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
