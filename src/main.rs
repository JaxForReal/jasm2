#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[cfg(test)]
mod tests;

pub mod parser;
pub mod vm;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // skip the 0th arg, because it is inherently the path of the binary (useless here)
    let filename = env::args().nth(1).expect("No input file");
    let mut file = File::open(filename).expect("could not open file");

    let mut program = String::new();
    file.read_to_string(&mut program).expect("could not read file");

    let parsed_program = parser::try_parse(&program).expect("could not parse program");

    vm::Vm::new(&parsed_program).exec();
}
