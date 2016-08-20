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

#[test]
fn math_operators() {
    let prog = &[Add(U32(345), U32(876), U32(0)),
                 Sub(U32(945), U32(876), U32(1)),
                 Mul(U32(345), U32(876), U32(2)),
                 Div(U32(945), U32(121), U32(3))];
    let mut vm = Vm::new(prog, io::sink());
    vm.exec();
    assert_eq!((345 + 876, 945 - 876, 345 * 876, 945 / 121),
               (vm.get_ram(0), vm.get_ram(1), vm.get_ram(2), vm.get_ram(3)));
}

#[test]
fn bit_operations() {
    let prog = &[And(U32(563), U32(395), U32(0)),
                 Or(U32(843), U32(237), U32(1)),
                 Xor(U32(937), U32(183), U32(2)),
                 Invert(U32(333), U32(3)),
                 LeftShift(U32(173), U32(3), U32(4)),
                 RightShift(U32(182), U32(2), U32(5))];
    let mut vm = Vm::new(prog, io::sink());
    vm.exec();
    assert_eq!((563 & 395, 843 | 237, 937 ^ 183, !333, 173 << 3, 182 >> 2),
               (vm.get_ram(0),
                vm.get_ram(1),
                vm.get_ram(2),
                vm.get_ram(3),
                vm.get_ram(4),
                vm.get_ram(5)));
}

#[test]
fn data_command() {
    let prog = &[Data(vec![U32(123), U32(456), U32(789)], U32(0))];
    let mut vm = Vm::new(prog, io::sink());
    vm.exec();
    assert_eq!((123, 456, 789), (vm.get_ram(0), vm.get_ram(1), vm.get_ram(2)));
}