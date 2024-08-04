# Electron Emulator/Compiler
This is the repo for the Electron Minecraft Redstone Computer.
Here you'll find the emululator and compiler, as well as some example programs and a vscode extention for syntax highlighting!

To compile and run the example program, clone the repo and run `cargo run -- -f fibbonaci.elt` or `cargo run -- -f heart.elt`.
You may need to install C/C++ depandencies from raylib-rs to run the GUI from here: https://github.com/deltaphc/raylib-rs.

## Programming & VS Code extention

The custom programming languege currently supports these instuctions and their varients:

### Syntax Highlighting example:
![Screenshot (1)](https://github.com/user-attachments/assets/a1841e33-3296-4aee-bc1d-d63cdf80b4d8)

## Emulator
To run the emulator, create a .elt file and write your program there. Then, run electron.exe with the arguments `-f your_program.elt`.

## Compiler
In development.

## Other
This is the spreadsheet which goes more in depth. This is not currently synced with the progess of this repo. 
https://docs.google.com/spreadsheets/d/1BrFaLE5tVunBa1GLoMH4RvVh4GXqqtaN3Qv29scvfyQ/edit?usp=sharing
