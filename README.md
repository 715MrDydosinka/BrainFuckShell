# BrainFuck Shell 

Is a command-line interactive shell where all commands must be written in the Brainfuck language.

## Run

```
git clone https://gitlab.com/Hluppppa/brainfuckshell
cd brainfuckshell
cargo run
```

## Supported symbols

    > - Increment the data pointer by one (to point to the next cell to the right).

    < - Decrement the data pointer by one (to point to the next cell to the left).

    + - Increment the byte at the data pointer by one.

    - - Decrement the byte at the data pointer by one.

    . - Output the byte at the data pointer.

    [ - If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. 

    ] - If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

There's no ',' cause it will be too easy.

## Example
Command "fastfetch":

    ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.>
P.S. I've been to lazy to use loops in this example -_-

## TODO
    - lexer
    - И так далее

## License

[Fuck your license](./LICENSE)