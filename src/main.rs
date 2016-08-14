#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;


pub mod parser;
pub mod vm;
pub mod graphics;
pub mod preprocessor;
#[cfg(test)]mod test;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;

use clap::{App, Arg};

fn main() {
    let matches = App::new("JASM interpreter")
        .author("Liam Pribis <liampribis@gmail.com>")
        .arg(Arg::with_name("filename")
            .index(1)
            .help("The file to run.")
            .required_unless("string"))
        .arg(Arg::with_name("string")
            .short("s")
            .long("string")
            .takes_value(true)
            .conflicts_with("filename")
            .help("Runs the string as a JASM file"))
        .arg(Arg::with_name("emit_type")
            .short("e")
            .long("emit")
            .takes_value(true)
            .possible_values(&["preprocessed", "parsed"]))
        .arg(Arg::with_name("log_level")
            .short("l")
            .long("log-level")
            .takes_value(true)
            .possible_values(&["off", "error", "warn", "info", "debug", "trace"])
            .default_value("off"))
        .get_matches();

    setup_logger(match matches.value_of("log_level").unwrap() {
        "off" => log::LogLevelFilter::Off,
        "error" => log::LogLevelFilter::Error,
        "warn" => log::LogLevelFilter::Warn,
        "info" => log::LogLevelFilter::Info,
        "debug" => log::LogLevelFilter::Debug,
        "trace" => log::LogLevelFilter::Trace,
        _ => panic!("unknown log level"),
    });

    let mut program = String::new();
    let current_pathbuf = env::current_dir().unwrap();
    let file_path = if matches.is_present("string") {
        program += matches.value_of("string").unwrap();
        current_pathbuf.as_path()
    } else {
        // if a file was used instead of a string
        let file_path = Path::new(matches.value_of("filename").unwrap());
        let mut file = File::open(file_path).expect("could not open file");
        file.read_to_string(&mut program).expect("could not read file");
        file_path
    };

    program = preprocessor::preprocess(&program, file_path);

    if matches.value_of("emit_type") == Some("preprocessed") {
        println!("{}", program);
        process::exit(0);
    }

    let parsed_program = match parser::try_parse(&program) {
        Ok(prog) => prog,
        Err(err) => panic!("Error parsing: {}", err),
    };

    if matches.value_of("emit_type") == Some("parsed") {
        for command in &parsed_program {
            println!("{:?}", command);
        }
    }

    // println!("{:?}", parsed_program);
    let stdout = io::stdout();
    vm::Vm::new(&parsed_program, stdout).exec();
}


fn setup_logger(log_level: log::LogLevelFilter) {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|log: &str, level: &log::LogLevel, location: &log::LogLocation| {
            let color_ansi = match *level {
                log::LogLevel::Error => "[31m",// red
                log::LogLevel::Warn => "[33m",// yellow
                log::LogLevel::Info => "[32m",// green
                log::LogLevel::Debug => "[35m",// purple
                log::LogLevel::Trace => "[34m",// blue
            };

            format!("[{module}:L{line}] {escape}{color}{log}{escape}[0m",
                    module = location.module_path(),
                    line = location.line(),
                    escape = 27 as char,
                    color = color_ansi,
                    log = log)
        }),
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Trace,
    };
    fern::init_global_logger(logger_config, log_level).unwrap();
}