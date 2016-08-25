use super::parser::syntax::{Command, Value};
use std::collections::HashMap;
use std::io::Write;
#[cfg(feature = "graphics")]
use std::ops;
use std::process;

#[cfg(feature = "graphics")]
use graphics;

mod syscalls;
mod instructions;
#[cfg(test)]
mod test;

// ram size in cells
const RAM_SIZE: usize = 3000;
// the the address where the stack poniter is stored in memory
const STACK_POINTER_ADDRESS: usize = 2000;
// the initial stack pointer (points to cell 2999)
const INITIAL_STACK_POINTER: u32 = 2999;

// the range in memory where the graphics output will be mapped
#[cfg(feature = "graphics")]
static GRAPHICS_LOCATION: ops::Range<usize> = 1000..1400;

#[cfg(feature = "graphics")]
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

// this version does not include graphics struct members, for compiling without "graphics" feature
#[cfg(not(feature = "graphics"))]
pub struct Vm<'a, TOut: Write> {
    prog: &'a [Command<'a>],
    output: TOut,
    ram: Vec<u32>,
    call_stack: Vec<usize>,
    instruction_pointer: usize,
    label_table: HashMap<&'a str, usize>,
}

impl<'a, TOut: Write> Vm<'a, TOut> {
    #[cfg(feature = "graphics")]
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

    #[cfg(not(feature = "graphics"))]
    pub fn new(new_prog: &'a [Command], out: TOut) -> Vm<'a, TOut> {
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
        info!("building label table");
        self.build_label_table();

        // make the sp point to initial stack address
        self.set_ram(STACK_POINTER_ADDRESS, INITIAL_STACK_POINTER);

        while self.instruction_pointer < self.prog.len() {
            let next_command = &self.prog[self.instruction_pointer];
            debug!("executing command (index {}): {:?}",
                   self.instruction_pointer,
                   next_command);
            if !self.exec_single_command(next_command) {
                return;
            }
            self.instruction_pointer += 1;
        }
    }

    // returns u32 of a value field.
    // needs to be &mut self because retrieving values can cause ram to grow
    fn get_value(&mut self, value: &Value) -> u32 {
        trace!("resolving value: {:?}", value);
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
        trace!("fetching ram at index: {}", index);
        self.ram[index]
    }

    fn set_ram(&mut self, index: usize, value: u32) {
        trace!("setting ram index {} to {}", index, value);
        self.ram[index] = value;
        self.copy_ram_to_graphics_buffer(index, value);
    }

    // this function extracts the conditionally compiled code to its own function
    // so #cfg[] can be used
    #[cfg(feature = "graphics")]
    fn copy_ram_to_graphics_buffer(&mut self, index: usize, value: u32) {
        // only map memory to screen if in graphics mode
        if self.is_graphics_mode {
            let new_sdl = match self.sdl {
                Some(ref mut new_sdl) => new_sdl,
                None => self.error("tried to map memory to sdl, but sdl isnt initialized"),
            };

            // if within graphics memory mapping, update the screen buffer as well
            if (index >= GRAPHICS_LOCATION.start) && (index < GRAPHICS_LOCATION.end) {
                trace!("copying ram to screen buffer");
                new_sdl.screen_buffer[index - GRAPHICS_LOCATION.start] = value != 0;
            }
        }
    }

    #[cfg(not(feature = "graphics"))]
    #[allow(unused_variables)]
    fn copy_ram_to_graphics_buffer(&mut self, index: usize, value: u32) {}

    fn build_label_table(&mut self) {
        for (index, command) in self.prog.iter().enumerate() {
            if let Command::Label(name) = *command {
                trace!("found label: '{}' at command index: {}", name, index);
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
