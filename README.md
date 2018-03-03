# brainfuck-rust
Brainfuck interpreter in Rust. Rust novice, probably a lot of suboptimal code.
The interpreter is only feeding new tape (think turing machines) on the right side.
Cells are 
unsigned 8-bit integers. Should be able to interpret all brainfuck programs that don't
rely on feeding tape in both directions or rely on looping around the tape.