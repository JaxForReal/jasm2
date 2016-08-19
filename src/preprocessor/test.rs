use super::process_defines;

#[test]
fn defines() {
    let program = "#define A_CONST_EXPR constant_value_123 add 3 A_CONST_EXPR -> 4";
    assert_eq!(&process_defines(program),
               " add 3 constant_value_123 -> 4");
}

#[test]
fn defines_before_declaration() {
    let program = "@SECRET_NUMBER -> 4 #define SECRET_NUMBER 1435 mul SECRET_NUMBER 3 -> \
                   @@SECRET_NUMBER";
    assert_eq!(&process_defines(program), "@1435 -> 4  mul 1435 3 -> @@1435");
}

#[test]
fn multiple_defines() {
    let program = "#define CONST_1 4563 #define CONST_TWO 2323 @@CONST_1textCONST_TWOasdasd";
    assert_eq!(&process_defines(program), "  @@4563text2323asdasd");
}