use super::Vm;
use std::char;

pub fn syscall(machine: &mut Vm, name: &str) {
    match name {
        "print" => print(machine),
        "print_ascii" => print_ascii(machine),
        _ => panic!("unknown syscall"),
    }
}

// print the value in address 0 as a decimal
fn print(machine: &mut Vm) {
    println!("{}", machine.get_ram(0));
}

// print the value in address 0 as an ascii char
fn print_ascii(machine: &mut Vm) {
    println!("{}",
             char::from_u32(machine.get_ram(0)).expect("char is not a valid character"));
}
