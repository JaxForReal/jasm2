#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

static PROGRAM: &'static str = "
<start>;
valueof 97 -> 0;
syscall print_ascii;
valueof 98 -> 0;
syscall print_ascii;

call other;

valueof 102 -> 0;
syscall print_ascii;
ret;

<other>;
valueof 99 -> 0;
syscall print_ascii;
valueof 100 -> 0;
syscall print_ascii;

call poo;
ret;

<poo>;
valueof 101 -> 0;
syscall print_ascii;
ret;
";

fn main() {
    let program = parser::try_parse(PROGRAM).unwrap();
    let mut machine = vm::Vm::new(&program);
    machine.exec();
}
