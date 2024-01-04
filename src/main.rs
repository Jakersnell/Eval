#![allow(unused)]

use std::{fs::File, io::Write};

use evaluator::evaluate;
use lexer::Lexer;
use parser::Parser;
use regex::Regex;
mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;


//  <clap plan>
// -----------
//  switches:
//      "--h" for help
//  params:
//      <expression> optional string to be parsed, exits after evaluating
//  no params will launch REPL

fn main() {
    let test = String::from("-5*(-2+2)");

    let tokens = Lexer::lex(test);

    println!("{:#?}", tokens);

    let tree = Parser::parse(tokens).unwrap();
    println!("{:#?}", tree);

    let result = evaluate(tree);
    println!("{:#?}", result);
}
