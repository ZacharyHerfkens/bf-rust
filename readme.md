# bf-rust
An implementation of the brainfuck esolang in rust

## Usage
```
bf-rust <FILE>
```

## Behavior
Implements the standard 8-operations of brainfuck: + - < > , . [ ]

Memory is an unsigned byte array of 65536 bytes, decrementing
the cell pointer past 0 or incrementing past 65535 results in a cell index out of bounds error and exits the program.

IO is done using `,` and `.`, which reads a byte from stdin and writes a byte to stdout respectively. End of Input (EOF) is represented by a 0 writen to the cell after `,`. Output is generally line-buffered. 