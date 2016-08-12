extern crate regex;
use self::regex::Regex;

pub fn preprocess(program: &str) -> String {
    process_defines(program)
}

fn process_defines(program: &str) -> String {
    let define_regex =
        Regex::new(r"(?m)#define\s+(?P<find>[a-zA-Z0-9_'@]+)\s+(?P<replace>[a-zA-Z0-9_'@]+)").unwrap();
    let captures_iter = define_regex.captures_iter(program);

    let mut program = program.to_owned();
    for captures in captures_iter {
        program = program.replace(captures.name("find").unwrap(), captures.name("replace").unwrap());
    }

    program = define_regex.replace_all(&program, "");

    program
}