use super::Vm;
use parser::Command::*;
use parser::Value::*;
use std::str;
use std::io;

#[test]
fn test_print() {
    let prog = &[ValueOf(U32(3146453), U32(0)), SysCall("print"), Ret];
    let mut out: Vec<u8> = Vec::new();
    Vm::new(prog, &mut out).exec();

    assert_eq!(str::from_utf8(&out).unwrap(), "3146453");
}

#[test]
fn test_math_operators() {
    let prog = &[Add(U32(345), U32(876), U32(0)),
                 Sub(U32(945), U32(876), U32(1)),
                 Mul(U32(345), U32(876), U32(2)),
                 Div(U32(945), U32(121), U32(3))];
    let mut vm = Vm::new(prog, io::sink());
    vm.exec();
    assert_eq!((345 + 876, 945 - 876, 345 * 876, 945 / 121),
               (vm.get_ram(0), vm.get_ram(1), vm.get_ram(2), vm.get_ram(3)));
}