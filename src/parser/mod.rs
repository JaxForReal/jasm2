peg_file! peg("peg.rustpeg");

pub fn try_parse<'a>(program: &'a str) -> Result<Vec<Command<'a>>, peg::ParseError> {
    peg::program(program)
}

#[derive(Debug)]
pub enum Value {
    U32(u32),
    Address(Box<Value>)
}

#[derive(Debug)]
pub enum Command<'a> {
    //BinOp(arg1, arg2, destination)
    Add(Value, Value, Value),
    Sub(Value, Value, Value),
    Div(Value, Value, Value),
    Mul(Value, Value, Value),

    //UnOp(arg, destination)
    Invert(Value, Value),
    ValueOf(Value, Value),

    //declare start of function
    //Func(name)
    Func(&'a str),
    //execute function
    //Call(name)
    Call(&'a str),
    
    Ret,
}
