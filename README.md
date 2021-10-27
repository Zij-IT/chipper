# Chipper:
Chipper is a Chip8 Emulator written in Rust. It currently supports a few known quirks of the Chip8 varieties. I intend on making it support the large majority of known quirks, and am on the fence on building a Chip8 debugger.
    
## Coding Philosophy:
I do not want my program to crash if the user provided bad input. If this emulator panics, I consider it a bug. All points of bad input should lead to a nice error message, followed by a smooth exit. To help me out with the errors, I decided to use the anyhow crate. I could have created (and did in the beginning), many different error types, but ended up using `anyhow::Error::msg` as it does exactly what I wanted my errors to do, and I don't have to type the boilerplate (win!).

## Usage:

There are currently no available binaries, so building from source is the only way. After that, just run the third command down below:

```shell
git clone git@github.com:Zij-IT/chipper.git
cd chipper
cargo run --release -- /path/to/game.ch8
```
 
## Supported Chip8 Quirks:
- [x] load store
- [x] offset jump
- [x] index register overflow
- [x] vertical wrap
- [x] shift
   
## Recommended Resources:
* [Tobiasvl's amazing guide for Chip8](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
* [corax89's OpCode Tests rom](https://github.com/corax89/chip8-test-rom)
* [loktar00's IBM Logo rom](https://github.com/loktar00/chip8/blob/master/roms/IBM%20Logo.ch8)

