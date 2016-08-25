use super::Vm;
use parser::syntax::Value;
use parser::syntax::Command;
use parser::syntax::Command::*;
use std::io::Write;

use super::STACK_POINTER_ADDRESS;

// here the actual operations of the vm are implemented
impl<'a, TOut: Write> Vm<'a, TOut> {
    // returns whether or not execution should ocntinue after this command;
    pub fn exec_single_command(&mut self, command: &Command) -> bool {
        match *command {
            Add(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a + b),
            Sub(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a - b),
            Mul(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a * b),
            Div(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a / b),

            LeftShift(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a << b),
            RightShift(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a >> b),

            And(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a & b),
            Or(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a | b),
            Xor(ref l, ref r, ref d) => self.auto_binary_op(l, r, d, |a, b| a ^ b),

            Invert(ref a, ref d) => self.auto_unary_op(a, d, |a| !a),

            ValueOf(ref a, ref d) => self.auto_unary_op(a, d, |a| a),

            Data(ref values, ref d) => {
                let dest = self.get_value(d) as usize;

                for (index, value) in values.iter().enumerate() {
                    let value_as_number = self.get_value(value);
                    self.set_ram(dest + index, value_as_number);
                }
            }

            Call(name) => {
                trace!("calling function: {}", name);
                // save our current place in program so we can return from call
                self.call_stack.push(self.instruction_pointer);
                // jump to the function that we looked up in the table
                self.instruction_pointer = self.label_table[name];
            }

            JumpAlways(label) => {
                self.instruction_pointer = self.label_table[label];
            }

            JumpEqual(ref a, ref b, label) => self.auto_jump_op(a, b, label, |a, b| a == b),
            JumpNotEqual(ref a, ref b, label) => self.auto_jump_op(a, b, label, |a, b| a != b),
            JumpLess(ref a, ref b, label) => self.auto_jump_op(a, b, label, |a, b| a < b),
            JumpGreater(ref a, ref b, label) => self.auto_jump_op(a, b, label, |a, b| a > b),

            Push(ref val) => {
                let stack_pointer = self.get_ram(STACK_POINTER_ADDRESS);
                trace!("pushing value onto stack: {:?}, stack pointer: {}", val, stack_pointer);

                if stack_pointer <= super::STACK_POINTER_ADDRESS as u32 {
                    self.error("attempted to push onto a full stack");
                }

                // add value onto the stack
                let resolved_value = self.get_value(val);
                self.set_ram(stack_pointer as usize, resolved_value);
                // decrement the pointer
                self.set_ram(STACK_POINTER_ADDRESS, (stack_pointer - 1) as u32);
            }

            Pop(ref dest) => {
                let stack_pointer = self.get_ram(STACK_POINTER_ADDRESS);

                if stack_pointer >= (super::RAM_SIZE - 1) as u32 {
                    self.error("attempted to pop when the stack was empty");
                }

                let new_sp = stack_pointer + 1;
                self.set_ram(STACK_POINTER_ADDRESS, new_sp);

                let resolved_dest = self.get_value(dest);
                let top_of_stack_value = self.get_ram(new_sp as usize);
                self.set_ram(resolved_dest as usize, top_of_stack_value);
            }

            // see self::syscalls module
            SysCall(name) => self.syscall(name),

            // ignore labels
            Label(_) => {}

            Ret => {
                let prev_position_maybe = self.call_stack.pop();

                if let Some(prev_position) = prev_position_maybe {
                    trace!("returning to previous instruction index: {}", prev_position);
                    // there is a function to return to, so set IP to last call position
                    self.instruction_pointer = prev_position;
                } else {
                    trace!("returning from top level, exiting");
                    // this means we are returning from top level code, and we should stop execution
                    return false;
                }
            }
        }
        // default return true to continue execution
        true
    }


    // performs a binary operation based on a simple fn(u32, u32) -> u32 closure
    // given left, right, and destination vals from parser
    fn auto_binary_op<TFunc>(&mut self,
                             left: &Value,
                             right: &Value,
                             dest: &Value,
                             operation: TFunc)
        where TFunc: Fn(u32, u32) -> u32
    {
        let dest_val = self.get_value(dest) as usize;

        let result = operation(self.get_value(left), self.get_value(right));
        self.set_ram(dest_val, result);
    }

    // takes a closure and argument/destination values and performs that closure on those values
    fn auto_unary_op<TFunc>(&mut self, arg: &Value, dest: &Value, operation: TFunc)
        where TFunc: Fn(u32) -> u32
    {
        let dest_val = self.get_value(dest) as usize;
        let result = operation(self.get_value(arg));
        self.set_ram(dest_val, result);
    }

    // closure is applied to l and r values, if it evals to true, jump to label
    fn auto_jump_op<TFunc>(&mut self, l: &Value, r: &Value, label: &str, operation: TFunc)
        where TFunc: Fn(u32, u32) -> bool
    {
        if operation(self.get_value(l), self.get_value(r)) {
            self.instruction_pointer = self.label_table[label];
        }
    }
}
