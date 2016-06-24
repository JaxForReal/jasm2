use super::parser::Command;
use super::parser::Value;


/*
ram value
0: instruction pointer


*/
pub struct Vm {
    ram: Vec<u32>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { ram: vec![0;32] }
    }

    pub fn exec(&mut self, prog: Vec<Command>) {

    }

    fn exec_command(&mut self, command: &Command) {

    }

    //returns u32 of a value field.
    //needs to be &mut self because retrieving values can cause ram to grow
    fn get_value(&mut self, value: &Value) -> u32 {
        match *value {
            Value::U32(n) => n,
            Value::Address(ref address) => {
                let address_val = self.get_value(address);
                self.get_ram(address_val as usize)
            },
        }
    }

    //retirives the <index> value of ram.
    //If it is ouside the vector length, auto grows vector
    fn get_ram(&mut self, index: usize) -> u32 {
        while index > self.ram.len() {
            self.ram.push(0);
        }
        self.ram[index]
    }
}
