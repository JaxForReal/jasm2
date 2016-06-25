use super::Vm;
use super::syscalls;
use parser::Value;
use parser::Command;

impl Vm {
    pub fn exec_single_command(&mut self, command: &Command) {
        match *command {
            Command::Add(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a + b),
            Command::Sub(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a - b),
            Command::Mul(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a * b),
            Command::Div(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a / b),
            Command::SysCall(name) => syscalls::syscall(self, name),
            _ => {}
        }
    }


    //performs a binary operation based on a simple fn(u32, u32) -> u32 closure
    // TODO is it better to use dynamic or static dispatch here for closure?
    // currently using static
    fn auto_binary_op<TFunc>(&mut self,
                             left: &Value,
                             right: &Value,
                             dest: &Value,
                             operation: TFunc)
        where TFunc: Fn(u32, u32) -> u32
    {
        let left_val = self.get_value(left);
        let right_val = self.get_value(right);
        let dest_val = self.get_value(dest) as usize;

        // perform closure operation on values
        let result = operation(left_val, right_val);
        self.set_ram(dest_val, result);
    }

    fn auto_unary_op<TFunc> (&mut self, arg: &Value, dest: &Value, operation: TFunc) where TFunc: Fn(u32) -> u32 {
        let arg_val = self.get_value(arg);
        let dest_val = self.get_value(dest) as usize;

        let result = operation(arg_val)
    }
}
