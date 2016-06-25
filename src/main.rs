#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

static PROGRAM: &'static str = "
<start>;
valueof 97 -> 0;
syscall print_ascii;
jumpzero 0 start;

ret;

";

fn main() {
    let program = parser::try_parse(PROGRAM).unwrap();
    let mut machine = vm::Vm::new(&program);
    machine.exec();
}
