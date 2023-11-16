use std::fs;
use std::process::exit;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use lexer::Lexer;
use stopwatch::StopWatch;

mod error;
mod interpreter;
mod lexer;
mod parser;
mod stopwatch;

fn main() {
    let mut task_stopwatch = StopWatch::new(true);
    let mut overall_stopwatch = StopWatch::new(true);

    overall_stopwatch.start();

    task_stopwatch.start();
    let mut file_content = fs::read_to_string("input.red").unwrap_or_else(|_| {
        println!("Error while reading file 'input.red'");
        exit(-1);
    });
    task_stopwatch.stop("Reading File");

    task_stopwatch.start();
    file_content = file_content.replace("\r", "");
    let mut lexer = Lexer::new(&file_content);
    let lexed = lexer.lex().unwrap_or_else(|err| {
        err.print_error(&file_content);
        exit(-1);
    });
    //println!("{:?}", lexed.clone());
    task_stopwatch.stop("Lexing");

    task_stopwatch.start();
    let mut parser = Parser::new(lexed);
    let parsed = parser.parse().unwrap_or_else(|err| {
        err.print_error(&file_content);
        exit(-1);
    });
    //println!("{:?}", parsed);
    task_stopwatch.stop("Parsing");

    task_stopwatch.start();
    let mut interpreter = Interpreter::new();
    interpreter.run(parsed).unwrap_or_else(|err| {
        err.print_error(&file_content);
        exit(-1);
    });
    task_stopwatch.stop("Interpreting");

    overall_stopwatch.stop("Overall Execution");
}
