extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;

pub fn preprocess(program: &str) -> String {
    process_defines(program)
}

fn process_defines(program: &str) -> String {
    let define_regex =
        Regex::new(r"(?m)#define\s+(?P<find>[a-zA-Z0-9_]+)\s+(?P<replace>[a-zA-Z0-9_]+)").unwrap();
    let captures_iter = define_regex.captures_iter(program);
    
    let mut const_mappings = HashMap::new();

    for captures in captures_iter {
        println!("capture found {:?}", captures);
        const_mappings.insert(captures.name("find").unwrap(),
                              captures.name("replace").unwrap());
    }

    println!("{:?}", const_mappings);

    "".to_owned()
}