extern crate regex;
use self::regex::Regex;
use self::regex::Captures;
use std::path::Path;
use std::io::Read;
use std::fs::File;

#[cfg(test)]
mod test;

// containing_path is the full path of program that is being processed
// including the file itself, eg "/home/me/Documents/my_program.jasm"
pub fn preprocess(program: &str, containing_path: &Path) -> String {
    info!("Starting preprocess stage");
    let included = process_includes(program, containing_path);
    process_defines(&included)
}

// replaces `#include "path/to/file.j"` with the contents of <project root>/path/to/file.j
// the path is relative to the parent directory of the file being compiled
fn process_includes(program: &str, containing_path: &Path) -> String {
    info!("Preprocessing include statements");

    let include_regex = Regex::new(r#"#include\s+"(?P<relative_path>.+?)""#).unwrap();

    include_regex.replace_all(program, |captures: &Captures| {

        let rel_path = captures.name("relative_path").unwrap();
        trace!("found include, relative path: {:?}", rel_path);
        let full_path = containing_path.join(rel_path);
        trace!("full path: {:?}", full_path);

        // recursivley process includes for the included file
        process_includes(&get_contents(&full_path), full_path.parent().unwrap())
    })
}

// gets the string contents of a file represented by file_path
fn get_contents(file_path: &Path) -> String {
    trace!("reading contents of file: {:?}", file_path);

    let mut file = File::open(file_path)
        .expect(&format!("could not open included file: `{:?}`", file_path));

    let mut include_str = String::new();
    file.read_to_string(&mut include_str)
        .expect(&format!("could not read included file: `{:?}`", file_path));

    include_str
}

// finds all instances of `#define CONST_NAME value` and removes them
// searches through the entire program and replaces CONST_NAME with value,
// even if CONST_NAME occurs before the #define statement
fn process_defines(program: &str) -> String {
    info!("Preprocessing define statementes");

    let define_regex =
        Regex::new(r#"(?m)#define\s+(?P<find>[a-zA-Z0-9_'@`]+)\s+(?P<replace>[a-zA-Z0-9_'@`]+)"#)
            .unwrap();
    let captures_iter = define_regex.captures_iter(program);

    let mut program = program.to_owned();
    for captures in captures_iter {
        trace!("replacing define statement regex capture: {:?}", captures);
        program = program.replace(captures.name("find").unwrap(),
                                  captures.name("replace").unwrap());
    }

    // remove all instances of `#define CONST val` because it breaks the parser
    program = define_regex.replace_all(&program, "");

    program
}
