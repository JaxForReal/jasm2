#![feature(plugin)]
#![plugin(peg_syntax_ext)]

extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;


pub mod parser;
pub mod vm;
#[cfg(feature = "graphics")]
pub mod graphics;
pub mod preprocessor;
#[cfg(test)]
mod test;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;

use clap::{App, Arg};

fn main() {
    let matches = App::new("JASM Interpreter")
        .version("versionNumbersAreAPain.0.1")
        .about("Interprets JASM assembly. See http://github.com/JaxForReal/jasm2/DOCUMENTATION.md")
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
            .possible_values(&["preprocessed", "parsed"])
            .help("Emits preprocessed file (expanded), or the parse result (a list of commands)"))
        .arg(Arg::with_name("log_level")
            .short("l")
            .long("log-level")
            .takes_value(true)
            .possible_values(&["off", "error", "warn", "info", "debug", "trace"])
            .default_value("info"))
        .get_matches();

    setup_logger(match matches.value_of("log_level").unwrap() {
        "off" => log::LogLevelFilter::Off,
        "error" => log::LogLevelFilter::Error,
        "warn" => log::LogLevelFilter::Warn,
        "info" => log::LogLevelFilter::Info,
        "debug" => log::LogLevelFilter::Debug,
        "trace" => log::LogLevelFilter::Trace,
        _ => unreachable!(),
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

    program = preprocessor::preprocess(&program, file_path.parent().unwrap());

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

    let stdout = io::stdout();
    vm::Vm::new(&parsed_program, stdout).exec();
}

#[allow(unused_variables)]
fn setup_logger(log_level: log::LogLevelFilter) {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|log: &str, level: &log::LogLevel, location: &log::LogLocation| {
            let color_ansi = match *level {
                log::LogLevel::Error => "31",// red
                log::LogLevel::Warn => "33",// yellow
                log::LogLevel::Info => "32",// green
                log::LogLevel::Debug => "34",// blue
                log::LogLevel::Trace => "35",// purple
            };
            if cfg!(not(feature = "show-log-locations")) {
                format!("{escape}[{color};1m{log_level}{escape}[0m {log}",
                        escape = 27 as char,
                        color = color_ansi,
                        log = log,
                        log_level = level)
            } else {
                format!("[{location}] {escape}[{color};1m{log_level}{escape}[0m {log}",
                        escape = 27 as char,
                        color = color_ansi,
                        log = log,
                        log_level = level,
                        location = location.module_path())
            }
        }),
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Trace,
    };
    fern::init_global_logger(logger_config, log_level).unwrap();
}
