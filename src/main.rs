#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

static PROGRAM: &'static str = "
<start>;
data 0 23 453 3 4543 55 -> 0;
";

fn main() {
    let parsed_program = parser::try_parse(PROGRAM).unwrap();
    println!("{:?}", parsed_program);
    let mut machine = vm::Vm::new(&parsed_program);
    machine.exec();
}
