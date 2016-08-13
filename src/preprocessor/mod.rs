extern crate regex;
use self::regex::Regex;
use std::path::Path;
use std::io::Read;
use std::fs::File;

// program_path is the full path of program that is being processed
// including the file itself, eg "/home/me/Documents/my_program.jasm"
pub fn preprocess(program: &str, program_path: &Path) -> String {
    let included = process_includes(program, program_path);
    process_defines(&included)
}

// replaces `#include "/path/to/file.j"` with the contents of /path/to/file.j
// the path is relative to the parent directory of the file being compiled
fn process_includes(program: &str, program_path: &Path) -> String {
    let include_regex = Regex::new(r#"#include\s+"(?P<filename>.+?)""#).unwrap();
    let captures_iter = include_regex.captures_iter(program);

    // accumulator for all changes to the program
    let mut program = program.to_owned();

    for captures in captures_iter {
        let (incl_start_pos, incl_end_pos) = captures.pos(0).unwrap();

        // remove the `#include "path"` from the program because it breaks the parser
        program.drain(incl_start_pos..incl_end_pos);

        let file_path = format!("{}{}",
                                program_path.parent().unwrap().to_str().unwrap(),
                                captures.name("filename").unwrap());
        let file_string = get_file_string(&file_path);

        // insert the included file to the middle of program
        program = format!("{}{}{}",
                          &program[..incl_start_pos],
                          file_string,
                          &program[incl_start_pos..]);
    }

    program
}

// gets the string contents of a file represented by file_path
fn get_file_string(file_path: &str) -> String {
    let mut file = File::open(file_path)
        .expect(&format!("could not open included file: `{}`", file_path));

    let mut include_str = String::new();
    file.read_to_string(&mut include_str)
        .expect(&format!("could not read included file: `{}`", file_path));

    include_str
}

// finds all instances of `#define CONST_NAME value` and removes them
// searches through the entire program and replaces CONST_NAME with value,
// even if CONST_NAME occurs before the #define statement
fn process_defines(program: &str) -> String {
    let define_regex =
        Regex::new(r#"(?m)#define\s+(?P<find>[a-zA-Z0-9_'@`]+)\s+(?P<replace>[a-zA-Z0-9_'@`]+)"#)
            .unwrap();
    let captures_iter = define_regex.captures_iter(program);

    let mut program = program.to_owned();
    for captures in captures_iter {
        program = program.replace(captures.name("find").unwrap(),
                                  captures.name("replace").unwrap());
    }

    // remove all instances of `#define CONST val` because it breaks the parser
    program = define_regex.replace_all(&program, "");

    program
}