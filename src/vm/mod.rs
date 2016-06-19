//
pub struct Vm {
    ram: Vec<u32>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm { ram: vec![0;32] }
    }
}
