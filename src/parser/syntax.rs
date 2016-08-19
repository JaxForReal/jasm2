#[derive(Debug, PartialEq)]
pub enum Value {
    U32(u32),
    Address(Box<Value>),
}

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    // BinaryOp(arg1, arg2, destination)
    Add(Value, Value, Value),
    Sub(Value, Value, Value),
    Div(Value, Value, Value),
    Mul(Value, Value, Value),
    // shift(value_to_shift, amount, destination)
    LeftShift(Value, Value, Value),
    RightShift(Value, Value, Value),
    And(Value, Value, Value),
    Or(Value, Value, Value),
    Xor(Value, Value, Value),

    Compare(Value, Value, Value),

    // UnaryOp(arg, destination)
    Invert(Value, Value),
    ValueOf(Value, Value),

    // Data(values, destination)
    Data(Vec<Value>, Value),

    // execute function
    // Call(name)
    Call(&'a str),

    // defines a label at this point in code
    Label(&'a str),

    SysCall(&'a str),

    // JumpConditional(val_to_test, label)
    JumpZero(Value, &'a str),
    JumpNotZero(Value, &'a str),

    Ret,
}
