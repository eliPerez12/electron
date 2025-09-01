# Electron Emulator/Compiler

Welcome to the repository for the Electron Minecraft Redstone Computer! This repo contains the emulator, compiler, example programs, and a VS Code extension for syntax highlighting.

## Getting Started

To compile and run the example programs, clone this repo and execute one of the following commands:

```sh
cargo run -- -f fibonacci.elt
cargo run -- -f heart.elt
```

### Prerequisites

To run the GUI, you may need to install C/C++ dependencies from [raylib-rs](https://github.com/deltaphc/raylib-rs).

### Running Without Compiling

You can also run the program without compiling. For instructions, refer to the [Emulator](#emulator) section.

## Programming & VS Code Extension

The custom programming language currently supports the following instructions and their variants:

- `IMM`
- `MOV`
- `ADD`
- `ADDC`
- `SHR`
- `NOOP`
- `OUT`
- `JMP`
- `BIE`
- `NOT`

### Syntax Highlighting Example:

Install the extension in VS Code using the VSIX file located at `electron-lang/electron-language-0.0.1.vsix`.

![Syntax Highlighting Example](https://github.com/user-attachments/assets/a1841e33-3296-4aee-bc1d-d63cdf80b4d8)

## Emulator

To run the emulator, create a `.elt` file and write your program in it. Then, execute `electron.exe` with the argument `-f your_program.elt`. You may also use one of the example programs.

## Compiler

The compiler is currently in development. Stay tuned for updates!

## Computer Specs

The version of the computer built in minecraft using redstone components has these specifications:

Electron Redstone Computer V1:

ROM: 96 bytes (32 lines)
Instruction size: 3 bytes
RAM: 32 bytes
ALUP: 8 ticks
Registers: 7 bytes ( +1 zero register )
Cores: 1
Pipeline 	4 stage waterfall
Bus width	8 bit
Display: 	16x16 screen 
I/O: 8 Bytes out, 1 byte in
Speed (Real time): 1hz


## Additional Resources

For more detailed information, refer to the [spreadsheet](https://docs.google.com/spreadsheets/d/1BrFaLE5tVunBa1GLoMH4RvVh4GXqqtaN3Qv29scvfyQ/edit?usp=sharing). Note that this document may not be fully up-to-date with the current progress of the repo.
