extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = App::new("brainfuck-rust")
        .version("0.0.1")
        .author("Aksel Slettemark <akselslettemark@gmail.com>")
        .about("Brainfuck interpreter")
        .arg(Arg::with_name("file")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Brainfuck file to run"))
        .arg(Arg::with_name("output")
            .required(false)
            .takes_value(true)
            .index(2)
            .help("Output mode: stdout/tape/tape_as_ascii"))
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let mut file = File::open(filename).expect("Error opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    match matches.value_of("output").unwrap_or("stdout") {
        "tape" => brainfuck(contents, 1),
        "tape_as_ascii" => brainfuck(contents, 2),
        _ => brainfuck(contents, 0) //Default is stdout (as ascii)
    }
}

fn brainfuck(code: String, outputmode: u8) {
    let mut tape: Vec<u8> = Vec::new();
    let mut tape_pointer: usize = 0;
    tape.push(0);

    let instructions: Vec<char> = code.chars().collect();
    let mut stdin = std::io::stdin().bytes();

    let code_length = instructions.len();
    let mut instruction_pointer: usize = 0;
    while instruction_pointer < code_length {
        let instruction = instructions[instruction_pointer];

        match instruction {
            '+' => {
                tape[tape_pointer] += 1;
                instruction_pointer += 1;
            },
            '-' => {
                tape[tape_pointer] -= 1;
                instruction_pointer += 1;
            },
            '>' => {
                tape_pointer += 1;
                let length = tape.len();
                if tape_pointer == length {
                    tape.resize(2 * length, 0); //Create more tape
                }
                instruction_pointer += 1;
            },
            '<' => {
                tape_pointer -= 1;
                instruction_pointer += 1;
            },
            '.' => {
                if outputmode == 0 { //Printing as we go for the time being
                    print!("{}", tape[tape_pointer] as char);
                }
                instruction_pointer += 1;
            },
            ',' => {
                let input: u8 = match stdin.next() {
                    Some(i) => i.unwrap(),
                    _ => break
                };
                tape[tape_pointer] = input;
                instruction_pointer += 1;
            }
            '[' => {
                if tape[tape_pointer] == 0 { //jump to matching ]
                    let mut nest = 1;
                    while nest > 0 {
                        instruction_pointer += 1;
                        match instructions[instruction_pointer] {
                            '[' => nest += 1,
                            ']' => nest -= 1,
                            _ => {}
                        }
                    }
                    instruction_pointer += 1; //Don't need to read the bracket again
                } else {
                    instruction_pointer += 1; //just move on
                }
            },
            ']' => {
                if tape[tape_pointer] != 0 { //jump to matching [
                    let mut nest = 1;
                    while nest > 0 {
                        instruction_pointer -= 1;
                        match instructions[instruction_pointer] {
                            '[' => nest -= 1,
                            ']' => nest += 1,
                            _ => {}
                        }
                    }
                    instruction_pointer += 1; //Don't need to read the bracket again
                } else {
                    instruction_pointer += 1;  //move on
                }
            }
            _ => {
                instruction_pointer += 1;
            }
        }
    }

    if outputmode == 1 {
        println!("{:?}", tape.as_slice());
    }
    else if outputmode == 2 {
        let s = std::str::from_utf8(tape.as_slice()).unwrap();
        println!("{}", s);
    }
}