use std::fs;
use std::process::exit;

use lexer::Lexer;
use stopwatch::StopWatch;

use crate::interpreter::Interpreter;
use crate::parser::Parser;

mod interpreter;
mod lexer;
mod parser;
mod stopwatch;

fn main() {
    let mut task_stopwatch = StopWatch::new();
    let mut overall_stopwatch = StopWatch::new();

    overall_stopwatch.start();

    task_stopwatch.start();
    let file_content = fs::read_to_string("input.red").unwrap_or_else(|_| {
        println!("Error while reading file 'input.red'");
        exit(-1);
    });
    task_stopwatch.stop("Reading File");

    task_stopwatch.start();
    let lexed = Lexer::new(file_content).lex();
    //println!("{:?}", lexed.clone());
    task_stopwatch.stop("Lexing");

    task_stopwatch.start();
    let parsed = Parser::new(lexed).parse().unwrap_or_else(|e| {
        println!("{}", e);
        exit(-1);
    });
    task_stopwatch.stop("Parsing");
    //println!("{:?}", parsed);

    task_stopwatch.start();
    Interpreter::new().run(parsed).unwrap_or_else(|e| {
        println!("{}", e);
        exit(-1);
    });
    task_stopwatch.stop("Interpreting");
    overall_stopwatch.stop("Overall Execution");
}
