use super::Vm;
use std::io::Write;


// implement all syscalls of the Vm
impl<'a, TOut: Write> Vm<'a, TOut> {
    pub fn syscall(&mut self, name: &str) {
        match name {
            "print" => self.print(),
            "print_ascii" => self.print_ascii(),
            _ => panic!("unknown syscall"),
        }
    }


    // print the value in address 0 as a decimal
    fn print(&mut self) {
        //convert u32 to string, to get the digits as individual chars
        //then convert the digit characters to a byte array to pass to output
        let value = self.get_ram(0).to_string();
        self.output.write(value.as_bytes()).expect("couldnt write to output");
    }

    // print the value in address 0 as an ascii char
    fn print_ascii(&mut self) {
        let value = self.get_ram(0);
        //write the value directly to out, because stdout formats as a char
        self.output.write(&[value as u8]).expect("couldnt write to output");
    }
}
