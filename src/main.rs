extern crate regex;
extern crate clap;
#[macro_use]
extern crate lazy_static;

mod ast;
mod dtm;
mod parser;
pub mod utility;

use std::error::Error;
use std::process;
use std::fs::File;
use std::io::{self, Read};
use ast::construct::construct_dtm;
use parser::Parser;
use clap::{App, Arg, ArgGroup, SubCommand, ArgMatches};
use dtm::interpret::{Interpreter, Tape};

fn run<'a>(matches: &ArgMatches<'a>) {
    let source = read(matches.value_of("source").unwrap());
    let program = filter_err(Parser::new().parse(source.as_str()));
    let dtm = filter_err(construct_dtm(program));

    let input = if let Some(input) = matches.value_of("input") {
        input.to_owned()
    } else if let Some(file) = matches.value_of("input-file") {
        read(file)
    } else {
        let mut input = String::new();
        filter_err(io::stdin().read_to_string(&mut input));
        input
    };

    let tape = filter_err(input.parse::<Tape>());
    let i = Interpreter::new(&dtm, tape);
    let outcome = filter_err(i.execute(matches.is_present("debug")));
    println!("{}", outcome);
}

fn latex<'a>(matches: &ArgMatches<'a>) {
    let source = read(matches.value_of("source").unwrap());
    let program = filter_err(Parser::new().parse(source.as_str()));
    let dtm = filter_err(construct_dtm(program));

    println!("{}", dtm.latex());
}

fn read(path: &str) -> String {
    let mut f = filter_err(File::open(path));

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("error reading file");

    contents
}

fn filter_err<T, E>(result: Result<T, E>) -> T
where E: Error {
    match result {
        Ok(x) => x,
        Err(err) => exit(err)
    }
}

fn exit<E>(err: E) -> !
where E: Error {
    eprintln!("Error: {}", err);
    process::exit(1);
}

fn main() {
    let matches = 
        App::new("Mamba")
            .version("0.1.0")
            .subcommand(
                SubCommand::with_name("run")
                    .about("Executes turing machines")
                    .arg(
                        Arg::with_name("source")
                            .help("Turing machine source")
                            .index(1)
                            .required(true)
                    )
                    .arg(
                        Arg::with_name("input")
                            .help("Turing machine input")
                            .short("i")
                            .long("input")
                            .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("input-file")
                            .help("Turing machine input file")
                            .short("f")
                            .long("input-file")
                            .takes_value(true)
                    )
                    .arg(
                        Arg::with_name("debug")
                            .help("Prints tape after each step")
                            .short("d")
                            .long("debug")
                    )
                    .group(
                        ArgGroup::with_name("input-group")
                            .args(&["input", "input-file"])
                    )
            )
            .subcommand(
                SubCommand::with_name("latex")
                    .about("Generates latex tables from a turing machine")
                    .arg(
                        Arg::with_name("source")
                            .help("Turing machine source")
                            .index(1)
                            .required(true)
                    )
            )
            .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        run(matches)
    } else if let Some(matches) = matches.subcommand_matches("latex") {
        latex(matches);
    } else {
        eprintln!("Error: Subcommand required, see mamba --help")
    }
}
