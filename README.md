# Brain F\*ck Interpreter

## Requirements

- Rust Development Environment

## Usage

```sh
cargo run {bf code file path}
```

## Showcase

### test.bf

![](screenshot/test.result.png)

### test2.bf

![](screenshot/test2.result.png)

### test5.bf

![](screenshot/test5.result.png)

### test8.bf

![](screenshot/test8.result.png)

## Explanation

- `expression.rs`

  - Defines the `Expression` trait and its implementations for various expression types.
  - Provides methods for converting expressions from strings.

- `interpreter.rs`

  - Implements the `Interpreter` struct and its methods for interpreting Brainfuck code.
  - Handles the parsing of Brainfuck code and the execution of commands.
  - Executes the Brainfuck commands using the `Machine` struct.

- `machine.rs`

  - Implements the `Machine` struct and its methods for executing Brainfuck code.
  - Handles the interaction with the user.
  - Manages the memory for the Brainfuck machine.
  - Provides the operation of Brainfuck commands.

- `main.rs`
  - Entry point of the program.
  - Handles command-line arguments and initializes the interpreter.
