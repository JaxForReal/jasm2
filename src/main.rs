#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

fn main() {
    let program = parser::try_parse("valueof 98 -> 0; syscall print_ascii;").unwrap();
    let mut machine = vm::Vm::new();
    machine.exec(&program);
}
