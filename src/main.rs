#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod parser;
pub mod vm;

fn main() {
    println!("{:?}", parser::try_parse(
        "
        <my_sub>;
        mul @0 3 -> 0;
        valueof 3 -> 1;
        ret;
        "
    ).unwrap());

}
