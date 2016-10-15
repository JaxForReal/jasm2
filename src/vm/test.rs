use super::Vm;
use parser::syntax::Command::*;
use parser::syntax::Value::*;
use std::str;
use std::io;

#[test]
fn print() {
    let prog = &[ValueOf(U32(3146453), U32(0)), SysCall("print"), Ret];
    let mut out: Vec<u8> = Vec::new();
    Vm::new(prog, &mut out).exec();

    assert_eq!(str::from_utf8(&out).unwrap(), "3146453");
}

// invocations come in this form:
// `program_run_test!{ test_name, &[Add(U32(1), U32(2), U32(3)), Ret], vm_ram_index =>
// expected_value, another_ram_index => another_expected_value, should_panic("reason")}`
// the `should_panic bit` is optional (but not the semicolon)
macro_rules! program_run_test {
    {$test_name:ident, $program:expr, $($ram_index:expr=>$ram_value:expr),* } =>
    {
        #[test]
        fn $test_name() {
            let prog = $program;
            let mut vm = Vm::new(prog, io::sink());
            vm.exec();
            $(
                assert_eq!(vm.get_ram($ram_index), $ram_value);
            )*
        }
    };
}

program_run_test! { add, &[Add(U32(7537), U32(1745), U32(0))], 0 => 7537 + 1745 }
program_run_test! { subtract, &[Sub(U32(7537), U32(1745), U32(0))], 0 => 7537 - 1745 }
program_run_test! { multiply, &[Mul(U32(7537), U32(1745), U32(0))], 0 => 7537 * 1745 }
program_run_test! { divide, &[Div(U32(7537), U32(1745), U32(0))], 0 => 7537 / 1745 }

program_run_test! { left_shift, &[LeftShift(U32(7537), U32(5), U32(0))], 0 => 7537 << 5 }
program_run_test! { right_shift, &[RightShift(U32(7537), U32(3), U32(0))], 0 => 7537 >>3 }
program_run_test! { and, &[And(U32(7537), U32(1745), U32(0))], 0 => 7537 & 1745 }
program_run_test! { or, &[Or(U32(7537), U32(1745), U32(0))], 0 => 7537 | 1745 }
program_run_test! { xor, &[Xor(U32(7537), U32(1745), U32(0))], 0 => 7537 ^ 1745 }

program_run_test! { invert, &[Invert(U32(7537), U32(0))], 0 => !7537 }
program_run_test! { valueof, &[ValueOf(U32(7537), U32(0))], 0 => 7537 }

program_run_test! {
    data_command,
    &[Data(vec![U32(2342), U32(2), U32(53421), U32(645645)], U32(5))],
    5 => 2342, 6 => 2, 7 => 53421, 8 => 645645
}

program_run_test! {
    push,
    &[Push(U32(2345)), Push(U32(946))],
    super::STACK_POINTER_ADDRESS => super::INITIAL_STACK_POINTER - 2,
    (super::INITIAL_STACK_POINTER - 1) as usize => 2345,
    (super::INITIAL_STACK_POINTER - 2) as usize => 946
}

#[test]
#[should_panic(expected = "attempted to pop when the stack was empty")]
fn pop_empty() {
    let prog = &[Pop(U32(0))];
    let mut vm = Vm::new(prog, io::sink());
    vm.exec();
}
