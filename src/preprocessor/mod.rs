use std::path::Path;
use std::fs::File;
use std::io::Read;

// this grammar returns a ( rest_of_prog:&str, Vec<(before:&str, include:&str, after:&str)> )
// rest of program holds the program if no includes are found, otherwise an empty string
// before is unaltered program before the include statement
// include is the path to include (whatever is in the quotes)
// after is unaltered program after the include statement
peg_file! preprocess_include_peg("preprocess_include.rustpeg");

peg_file! preprocess_define_peg("preprocess_define.rustpeg");

pub fn preprocess(program: &str, compiled_file_path: &str) -> String {
    let included_program = process_includes(program, compiled_file_path);
    println!("{}", included_program);
    //process_defines(program);
    "".to_owned()
}

fn process_includes(program: &str, compiled_file_path: &str) -> String {
    let mut new_program = String::new();

    let (rest_of_prog, includes_list) = preprocess_include_peg::program(program).unwrap();
    new_program += rest_of_prog;

    for (before, incl, after) in includes_list {
        new_program += before;
        new_program += &get_file_string(incl, compiled_file_path);
        new_program += after;
    }
    new_program
}

// takes the relative path given by #include, and the path of file being compiled
// returns the string inside that specified file
fn get_file_string(rel_path: &str, compiled_file_path: &str) -> String {
    let parent_path = Path::new(compiled_file_path)
        .parent()
        .expect("cant get parent dir of main file")
        .to_str()
        .unwrap();
    // println!("rel path: {}\nparent path: {}", rel_path, parent_path);
    let mut file_string = String::new();
    let full_path = format!("{}{}", parent_path, rel_path);
    File::open(&full_path)
        .expect(&format!("couldnt open included file: {}", &full_path))
        .read_to_string(&mut file_string)
        .expect("couldnt read included file to string");

    file_string
}

fn process_defines(program: &str) -> String {
    let defs = preprocess_define_peg::program(program).unwrap();
    
    let mut out = String::new();

    println!("{:?}", defs);

    "".to_owned()
}