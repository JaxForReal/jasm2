peg_file! peg("peg.rustpeg");

pub fn try_parse<'a>(program: &'a str) -> Result<Vec<Command<'a>>, peg::ParseError> {
    peg::program(program)
}

#[derive(Debug)]
pub enum Value {
    U32(u32),
    Address(Box<Value>),
}

#[derive(Debug)]
pub enum Command<'a> {
    // BinaryOp(arg1, arg2, destination)
    Add(Value, Value, Value),
    Sub(Value, Value, Value),
    Div(Value, Value, Value),
    Mul(Value, Value, Value),
    // shift(value_to_shift, amount, destination)
    LeftShift(Value, Value, Value),
    RightShift(Value, Value, Value),

    Compare(Value, Value, Value),

    // UnaryOp(arg, destination)
    Invert(Value, Value),
    ValueOf(Value, Value),

    // declare start of function
    // Func(name)
    Func(&'a str),
    // execute function
    // Call(name)
    Call(&'a str),

    // defines a label at this point in code
    Label(&'a str),

    // JumpConditional(flag_address, label)
    JumpZero(Value, &'a str),
    JumpNotZero(Value, &'a str),

    Ret,
}
