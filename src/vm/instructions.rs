use super::Vm;
use parser::Value;
use parser::Command;
use std::io::Write;

// here the actual operations of the vm are implemented
impl<'a, TOut: Write> Vm<'a, TOut> {
    // returns whether or not execution should ocntinue after this command;
    pub fn exec_single_command(&mut self, command: &Command) -> bool {
        match *command {
            // TODO macro for this pattern?
            Command::Add(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a + b),
            Command::Sub(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a - b),
            Command::Mul(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a * b),
            Command::Div(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a / b),

            Command::LeftShift(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a << b),
            Command::RightShift(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a >> b),

            Command::And(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a & b),
            Command::Or(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a | b),
            Command::Xor(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a ^ b),

            // See README.md for documentation on the compare command
            Command::Compare(ref l, ref r, ref d) => {
                self.auto_binary_op(l, r, d, |a, b| {
                    let mut ret = 0;
                    if a == b {
                        ret |= 0b1;
                    };
                    if a != b {
                        ret |= 0b10;
                    };
                    if a < b {
                        ret |= 0b100;
                    };
                    if a > b {
                        ret |= 0b1000;
                    };
                    if a <= b {
                        ret |= 0b10000;
                    };
                    if a >= b {
                        ret |= 0b100000;
                    };
                    ret
                })
            }

            Command::Invert(ref a, ref d) => self.auto_unary_op(a, d, |a| !a),
            Command::ValueOf(ref a, ref d) => self.auto_unary_op(a, d, |a| a),

            Command::Data(ref values, ref d) => {
                let dest = self.get_value(d) as usize;

                for (index, value) in values.iter().enumerate() {
                    let value_as_number = self.get_value(value);
                    self.set_ram(dest + index, value_as_number);
                }
            }

            Command::Call(name) => {
                // save our current place in program so we can return from call
                self.call_stack.push(self.instruction_pointer);
                // jump to the function that we looked up in the table
                self.instruction_pointer = self.label_table[name];
            }

            Command::Ret => {
                let prev_position_maybe = self.call_stack.pop();

                if let Some(prev_position) = prev_position_maybe {
                    // there is a function to return to, so set IP to last call position
                    self.instruction_pointer = prev_position;
                } else {
                    // this means we are returning from top level code, and we should stop execution
                    return false;
                }
            }

            Command::JumpZero(ref a, name) => {
                if self.get_value(a) == 0 {
                    // jump to label defined by name
                    self.instruction_pointer = self.label_table[name]
                }
            }
            Command::JumpNotZero(ref a, name) => {
                if self.get_value(a) != 0 {
                    // jump to label defined by name
                    self.instruction_pointer = self.label_table[name]
                }
            }

            // see self::syscalls module
            Command::SysCall(name) => self.syscall(name),

            // ignore labels
            Command::Label(_) => {}
        }
        // default return true to continue execution
        true
    }


    // performs a binary operation based on a simple fn(u32, u32) -> u32 closure
    // given left, right, and destination vals from parser
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

    // takes a closure and argument/destination values and performs that closure on those values
    fn auto_unary_op<TFunc>(&mut self, arg: &Value, dest: &Value, operation: TFunc)
        where TFunc: Fn(u32) -> u32
    {
        let arg_val = self.get_value(arg);
        let dest_val = self.get_value(dest) as usize;

        let result = operation(arg_val);
        self.set_ram(dest_val, result);
    }
}
