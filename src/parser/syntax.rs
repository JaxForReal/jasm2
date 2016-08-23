#[derive(Debug, PartialEq)]
pub enum Value {
    U32(u32),
    Address(Box<Value>),
}

type Label<'a> = &'a str;

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
    Call(Label<'a>),

    // defines a label at this point in code
    Label(Label<'a>),

    SysCall(Label<'a>),

    JumpEqual(Value, Value, Label<'a>),
    JumpNotEqual(Value, Value, Label<'a>),
    JumpGreater(Value, Value, Label<'a>),
    JumpLess(Value, Value, Label<'a>),

    Ret,
}
