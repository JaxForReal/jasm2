use super::Vm;
use std::io::{self, Write};
use std::thread;
use std::time;
#[cfg(feature = "graphics")]
use graphics;

// implement all syscalls of the Vm
impl<'a, TOut: Write> Vm<'a, TOut> {
    pub fn syscall(&mut self, name: &str) {
        match name {
            "print" => self.print(),
            "print_char" => self.print_char(),
            "print_binary" => self.print_binary(),
            "read" => self.read(),
            "read_string" => self.read_string(),
            "read_char" => self.read_char(),
            "render_graphics" => self.render_graphics(),
            "delay" => self.delay(),
            "set_mode" => self.set_mode(),
            _ => self.error(&format!("unknown syscall: {}", name)),
        }
    }


    // print the value in address 0 as a decimal
    fn print(&mut self) {
        // println!("print called");
        // convert u32 to string, to get the digits as individual chars
        // then convert the digit characters to a byte array to pass to output
        let value = self.get_ram(0).to_string();
        if let Err(..) = self.output.write(value.as_bytes()) {
            self.error("couldnt write to stdout")
        }
        // need to flush because stdout is line buffered, and will not update with single char
        self.output.flush().unwrap();
    }

    // print the value in address 0 as an ascii char
    fn print_char(&mut self) {
        let value = self.get_ram(0);
        // write the value directly to out, because stdout formats as a char
        if let Err(..) = self.output.write(&[value as u8]) {
            self.error("couldnt write to output");
        }
        // need to flush because stdout is line buffered, and will not update with single char
        self.output.flush().unwrap();
    }

    fn print_binary(&mut self) {
        let value = self.get_ram(0);
        if let Err(..) = self.output.write(format!("{:b}", value).as_bytes()) {
            self.error("couldnt write to output");
        }
        // need to flush because stdout is line buffered, and will not update with single char
        self.output.flush().unwrap();
    }

    // read a single value into address 0
    // returns 0 if the value coild not be parsed.
    fn read(&mut self) {
        let mut input = String::new();
        if let Err(..) = io::stdin().read_line(&mut input) {
            self.error("couldnt read from stdin");
        }
        let value = input.trim().parse::<u32>().unwrap_or(0);
        self.set_ram(0, value);
    }

    // read a string of utf8 chars starting at the pointer in call #0
    fn read_string(&mut self) {
        let save_pointer = self.get_ram(0);

        let mut input = String::new();
        if let Err(..) = io::stdin().read_line(&mut input) {
            self.error("oculdnt read from stdin");
        }
        for (index, chr) in input.chars().enumerate() {
            self.set_ram(save_pointer as usize + index, chr as u32);
        }
    }

    // reads a single utf-8 char to memory cell 0
    fn read_char(&mut self) {
        let mut input = String::new();
        if let Err(..) = io::stdin().read_line(&mut input) {
            self.error("couldnt read from stdin");
        }
        let chr = input.chars().next().unwrap_or(0 as char) as u32;
        self.set_ram(0, chr);
    }

    // render graphics to screen
    #[cfg(feature = "graphics")]
    fn render_graphics(&mut self) {
        match self.sdl {
            Some(ref mut inner_sdl) => inner_sdl.render(),
            None => self.error("Tried to render when Sdl was not initialized."),
        }
    }
    #[cfg(not(feature = "graphics"))]
    fn render_graphics(&mut self) {
        self.error("Graphics is not enabled in this installation, compile with `--features \
                    \"graphics\"` to enable");
    }

    // delay for @0 milliseconds
    fn delay(&mut self) {
        let time = self.get_ram(0);
        thread::sleep(time::Duration::from_millis(time as u64));
    }

    // sets the mode based on @0
    // 0 = console
    // anything else = graphics
    #[cfg(feature = "graphics")]
    fn set_mode(&mut self) {
        if self.get_ram(0) == 0 {
            trace!("turning off graphics mode");
            self.is_graphics_mode = false;
            self.sdl = None;
        } else {
            trace!("turning on graphcis mode");
            // change vm mode, and initialize sdl graphics
            self.is_graphics_mode = true;
            self.sdl = Some(graphics::MySdl::new());
        }
    }

    #[cfg(not(feature = "graphics"))]
    fn set_mode(&mut self) {
        self.error("Graphics is not enabled in this installation, compile with `--features \
                    \"graphics\"` to enable");
    }
}
