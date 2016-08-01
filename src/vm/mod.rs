use super::parser::{Command, Value};
use std::collections::HashMap;
use std::io::Write;

mod syscalls;
mod instructions;
#[cfg(test)]
mod test;

// ram size in cells
const RAM_SIZE: usize = 1024;

pub struct Vm<'a, TOut: Write> {
    // vector of commands, executed sequentially
    prog: &'a Vec<Command<'a>>,
    // all print syscalls, etc output is written to this object
    output: TOut,
    ram: Vec<u32>,

    // a stack for func calls. records where the instruction pointer was when a func was called,
    // and returns to that instruction when ret is encountered
    call_stack: Vec<usize>,

    // The index into prog vector where the currently executing command is
    instruction_pointer: usize,

    // translates label names to the instruction pointer where that function begins
    label_table: HashMap<&'a str, usize>,
}

impl<'a, TOut: Write> Vm<'a, TOut> {
    pub fn new<'b>(new_prog: &'b Vec<Command>, out: TOut) -> Vm<'b, TOut> {
        Vm {
            prog: new_prog,
            output: out,
            ram: vec![0; RAM_SIZE],
            call_stack: Vec::new(),
            instruction_pointer: 0,
            label_table: HashMap::new(),
        }
    }

    // executes the program, writing all output to the `out` object
    pub fn exec(&mut self) {
        self.build_label_table();

        while self.instruction_pointer < self.prog.len() {
            let next_command = &self.prog[self.instruction_pointer];
            if !self.exec_single_command(next_command) {
                return;
            }
            self.instruction_pointer += 1;
        }
    }

    // returns u32 of a value field.
    // needs to be &mut self because retrieving values can cause ram to grow
    fn get_value(&mut self, value: &Value) -> u32 {
        match *value {
            Value::U32(n) => n,
            Value::Address(ref address) => {
                let address_val = self.get_value(address);
                self.get_ram(address_val as usize)
            }
        }
    }

    // retirives the <index> value of ram.
    // If it is ouside the vector length, auto grows vector
    fn get_ram(&self, index: usize) -> u32 {
        self.ram[index]
    }

    fn set_ram(&mut self, index: usize, value: u32) {
        self.ram[index] = value;
    }

    fn build_label_table(&mut self) {
        // for some reason we cannot use enumerate() here, so we must use a manual increment iterator
        let mut iter = 0;
        for command in self.prog {
            if let Command::Label(name) = *command {
                self.label_table.insert(name, iter);
            }
            iter += 1;
        }
    }
}
