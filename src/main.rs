#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

fn main() {
    println!("{:?}",
             parser::try_parse("ret;")
                 .unwrap());


    let program = parser::try_parse("ret;").unwrap();
    let mut machine = vm::Vm::new();
    machine.exec(program);
}
