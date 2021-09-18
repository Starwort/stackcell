# In this folder are a selection of code samples

## [cat.cel](./cat.cel)

Cat is a program that outputs its input unchanged.

```
@:[;@:]
```

- `@`: Take a character of input.
- `:`: Duplicate the read character.
- `[`: Consume the duplicated character, and start the main loop if non-EOF.
- `;`: Print the read character.
- `@`: Take another character of input.
- `:`: Duplicate the read character.
- `]`: Consume the duplicated character, and end the main loop if EOF reached.

## [error_quine.cel](./error_quine.cel)

An error quine is not a true quine, but one which uses interpreter/compiler messages to print its own source. error_quine.cel is such a program.

```
Error: Unknown instruction: 'E'

```

- The first character of `Error`, `E`, isn't a valid instruction and so the program crashes.

## [hello.cel](./hello.cel)

Prints "Hello, world!"

```
"
!dlrow ,olleH":[;:]
```

- ```
  "
  !dlrow ,olleH"
  ```
  Push the string `"Hello, world!\n"` onto the stack, in reverse.
- `:` Duplicate the `H` so it isn't lost.
- `[` Consume the duplicated `H` and begin the print loop.
- `;` Print the next character.
- `:` Duplicate the next character so it isn't lost.
- `]` Consume the duplicated character and return to the start of the print loop, or end.

## [tac.cel](./tac.cel)

Tac is a program that outputs its input in reverse.

```
@:[@:]`:[;:]
```

- `@`: Take a character of input.
- `:`: Duplicate the read character.
- `[`: Consume the duplicated character, and start the read loop if non-EOF.
- `@`: Take another character of input.
- `:`: Duplicate the read character.
- `]`: Consume the duplicated character, and end the read loop if EOF reached.
- `` ` ``: Discard the EOF.
- `:`: Duplicate the next character so it isn't lost.
- `[`: Consume the duplicated character, and start the print loop if non-EOF.
- `;`: Print the character.
- `:`: Duplicate the next character so it isn't lost.
- `]`: Consume the duplicated character, and end the print loop if EOF reached.

## [tac_line.cel](./tac_line.cel)

Tac, but executes on one line only

```
(@:#0A-)`:[;:]#0A;.
```

- `(`: Begins the input loop
- `@`: Reads a character from the keyboard and pushes it onto the stack
- `:`: Duplicates the top inputted character to allow for comparison
- `#0A`: Pushes a newline (`0x0A`) to the stack
- `-`: Subtracts the top two values of the stack (it doesn't matter which way round, as the result is either zero or non-zero)
- `)`: Returns to the beginning of the loop, if the last character wasn't a newline.
- `` ` ``: Discard the newline at the end of the user's input
- `:[;:]`: Print the stack, as before. This will print the user's string, but in reverse
- `#0A;.`: Print the trailing newline, then stop

## [truth_machine.cel](./truth_machine.cel)

As per [the esolang page](https://esolangs.org/wiki/Truth-machine), this program accepts an input of 1 or 0, and then exhibits one of two behaviours:

- If the input is 1, it prints 1 repeatedly, forever
- If the input is 0, it prints 0 once, then exits

```
'0@-:[:'0+;:]'0+;.
```

- `'0`: Pushes an ASCII `'0'` to the stack
- `@`: Reads a character and pushes it onto the stack (`'0'` or `'1'`)
- `-`: Subtract the `'0'` we pushed before from the `'0'` or `'1'` we just read (this gives us a numeral result of `0` or `1`, respectively)
- `:`: Duplicate the top value of the stack (`0` or `1`) so that there is still a copy on the stack after the `[`
- `[`: Begin the loop, consuming the top value of the stack (`0` or `1`)
- `:`: Duplicate the top value (`1`) of the stack, so that there is still a copy on the stack after transforming and printing it
- `'0+`: Add an ASCII `'0'` to the top value of the stack (`1`) to get the ASCII `'1'`
- `;:]`: Print the top value of the stack (`'1'`) and return to the beginning of the loop
- `'0+;.`: Transform the `0` back to `'0'` and print it, then stop
