#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

static PROGRAM: &'static str = "
12 -> 1;
data 5 8 99 456 2 -> 10;
@@1 -> 0;
syscall print;
";

fn main() {
    let parsed_program = parser::try_parse(PROGRAM).unwrap();
    println!("{:?}", parsed_program);
    let mut machine = vm::Vm::new(&parsed_program);
    machine.exec();
}
