use super::parser::{Command, Value};
use std::collections::HashMap;
use std::io::Write;
use std::ops;
use std::process;

use graphics;

mod syscalls;
mod instructions;
#[cfg(test)]
mod test;

// ram size in cells
const RAM_SIZE: usize = 2048;

// the range in memory where the graphics output will be mapped
static GRAPHICS_LOCATION: ops::Range<usize> = 1000..1400;

pub struct Vm<'a, TOut: Write> {
    // vector of commands, executed sequentially
    prog: &'a [Command<'a>],
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

    // used to draw graphics to screen
    // Some(sdl) if in graphics mode
    // None if in console mode
    sdl: Option<graphics::MySdl<'a>>,

    // if true, graphics mode is enabled in the program.
    is_graphics_mode: bool,
}

impl<'a, TOut: Write> Vm<'a, TOut> {
    pub fn new(new_prog: &'a [Command], out: TOut) -> Vm<'a, TOut> {
        Vm {
            prog: new_prog,
            output: out,
            ram: vec![0; RAM_SIZE],
            call_stack: Vec::new(),
            instruction_pointer: 0,
            label_table: HashMap::new(),
            // start in console mode
            is_graphics_mode: false,
            sdl: None,
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

        // only map memory to screen if in graphics mode
        if self.is_graphics_mode {
            let new_sdl = match self.sdl {
                Some(ref mut new_sdl) => new_sdl,
                None => self.error("tried to map memory to sdl, but sdl isnt initialized"),
            };

            // if within graphics memory mapping, update the screen buffer as well
            if (index >= GRAPHICS_LOCATION.start) && (index < GRAPHICS_LOCATION.end) {
                new_sdl.screen_buffer[index - GRAPHICS_LOCATION.start] = value != 0;
            }
        }

    }

    fn build_label_table(&mut self) {

        for (index, command) in self.prog.iter().enumerate() {
            if let Command::Label(name) = *command {
                self.label_table.insert(name, index);
            }
        }
    }

    fn error(&self, message: &str) -> ! {
        let ip = self.instruction_pointer;
        println!("Error in instruction: `{:?}`", self.prog[ip]);
        println!("{}", message);
        process::exit(1);
    }
}
