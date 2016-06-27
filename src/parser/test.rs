use super::peg;
use super::Value;
use super::Command;

#[test]
fn peg_values_literals() {
    assert_eq!(peg::value("132435").unwrap(), Value::U32(132435));
    assert_eq!(peg::value("b101011100001").unwrap(),
               Value::U32(0b101011100001));
    assert_eq!(peg::value("'a").unwrap(), Value::U32(97));
}

#[test]
fn peg_values_addresses() {
    assert_eq!(peg::value("@420").unwrap(),
               Value::Address(Box::new(Value::U32(420))));
    assert_eq!(peg::value("@b110110").unwrap(),
               Value::Address(Box::new(Value::U32(0b110110))));
    assert_eq!(peg::value("@@5").unwrap(),
               Value::Address(Box::new(Value::Address(Box::new(Value::U32(5))))));
}

#[test]
fn binary_ops() {
    assert_eq!(peg::command("add 3 4 -> 5").unwrap(),
               Command::Add(Value::U32(3), Value::U32(4), Value::U32(5)));
    assert_eq!(peg::command("compare 3 4 -> 5").unwrap(),
               Command::Compare(Value::U32(3), Value::U32(4), Value::U32(5)));
}

#[test]
fn unary_ops() {
    assert_eq!(peg::command("invert 654 -> 23").unwrap(),
               Command::Invert(Value::U32(654), Value::U32(23)));
}

#[test]
fn single_comments() {
    assert_eq!(peg::command("add   \n\n   3  \n\n\n//comment   \r\n\n    4 \n-> \n\n\n\n \
                             \n\n  5")
                   .unwrap(),
               Command::Add(Value::U32(3), Value::U32(4), Value::U32(5)));
}

#[test]
fn multi_comments() {
    assert_eq!(peg::command("add /*multiline comment*/ 3 4 /*\nmulti\nline\n*/-> 5").unwrap(),
               Command::Add(Value::U32(3), Value::U32(4), Value::U32(5)));
}

#[test]
fn labels() {
    assert_eq!(peg::command("<label_name>").unwrap(),
               Command::Label("label_name"));
}

#[test]
fn data_values() {
    assert_eq!(peg::command("data 3 1 4 159 26535 -> 56").unwrap(),
               Command::Data(vec![Value::U32(3),
                                  Value::U32(1),
                                  Value::U32(4),
                                  Value::U32(159),
                                  Value::U32(26535)],
                             Value::U32(56)));
}

#[test]
fn data_string() {
    assert_eq!(peg::command("data `abd` -> 56").unwrap(),
               Command::Data(vec![Value::U32(97), Value::U32(98), Value::U32(100)],
                             Value::U32(56)));
}
