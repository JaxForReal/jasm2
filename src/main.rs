#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[cfg(test)]
mod test;

pub mod parser;
pub mod vm;
pub mod graphics;
pub mod preprocessor;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    // skip the 0th arg, because it is inherently the path of the binary (useless here)
    let filename = env::args().nth(1).expect("No input file");
    let mut file = File::open(Path::new(&filename)).expect("could not open file");

    let mut program = String::new();
    file.read_to_string(&mut program).expect("could not read file");

    preprocessor::preprocess(&program, &filename);
    /*let parsed_program = parser::try_parse(&program).expect("could not parse program");

    // println!("{:?}", parsed_program);
    let stdout = io::stdout();
    vm::Vm::new(&parsed_program, stdout).exec();*/
}
