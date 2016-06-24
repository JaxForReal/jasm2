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

    pub fn exec(prog: Vec<Command>) {

    }

    fn get_value(&self, value: &Value) -> u32 {
        match *value {
            Value::U32(n) => n,
            Value::Address(address) => self.ram[get_value(&address)]
        }
    }

    //retirives the <index> value of ram. If it is ouside the vector length, auto grows vector
    ram_addr(&mut self, index: usize) -> u32 {

    }
}
