#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

static PROGRAM: &'static str = "



ret;

";

fn main() {
    let program = parser::try_parse(PROGRAM).unwrap();
    let mut machine = vm::Vm::new(&program);
    machine.exec();
}
