#![allow(unused)]

use ast::SyntaxTree;
use clap::Parser as ArgParser;
use colored::Colorize;
use evaluator::evaluate;
use lexer::Lexer;
use parser::Parser as TokenParser;
use regex::Regex;
use std::{fs::File, io::Write, process::exit};
mod ast;
mod evaluator;
mod lexer;
mod parser;
mod token;

static mut SHOW_TOKENS: bool = false;
static mut SHOW_TREE: bool = false;

const REPL_HEADER: &str = r#"
                 _ _
  _____   ____ _| | |
 / _ \ \ / / _` | | |
|  __/\ V / (_| | |_|
 \___| \_/ \__,_|_(_)
 
 welcome to eval repl.
 enter ".help" for help.

"#;

const HELP_MENU: &str = "
    single parameter functions:
        sin(N)                  : Standard sin 
        cos(N)                  : Standard cos
        tan(N)                  : Standard tan
        sinh(N)                 : Standard sinh
        cosh(N)                 : Standard cosh
        tanh(N)                 : Standard tanh
        sqrt(N)                 : Standard square root
        ln(N)                   : Standard natural logarithm

    double parameter functions:
        pow(N, E)               : Raise N to the power of E.
        root(N, R)              : Return the R root of N.
        log(N, B)               : Return N log B.
        
    supported operators:
        *                       : Multiplication
        /                       : Division
        -                       : Subtraction
        +                       : Addition
        %                       : Modulo
        ^ or **                 : Exponentiation

    other:
        (EXPRESSION)            : Parens to denote precedence
        |EXPRESSION|            : Pipes for absolute of expression

    commands:
        .help : Displays this menu
        .show-tree              : Toggles the SHOW_TREE flag, set to false by default.
        .show-tokens            : Toggles the SHOW_TOKENS flag, set to false by default.
";

//  goal usage:     eval {4.3 ** 3 + cos(2.3*(4+2))}
//  <clap plan>
// -----------
//  switches:
//      "--h" for help
//  params:
//      <expression> optional string to be parsed, exits after evaluating
//  no params will launch REPL

// #[derive(ArgParser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Optional expression to evaluate
//     expression: String,
// }

fn main() {
    repl();
}

fn repl() {
    println!("{}", REPL_HEADER);
    loop {
        print!(">>> ");
        std::io::stdout().flush();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if handle_line(input) {
                    break;
                }
            }
            Err(error) => {
                println!("Error while reading input: {}.", error);
            }
        }
    }
    println!("Goodbye!");
}

fn handle_line(input: String) -> bool {
    let help_regex = Regex::new(r"^\s*\.help\s*$").unwrap();
    let show_tree_regex = Regex::new(r"^\s*\.show-tree\s*$").unwrap();
    let show_tokens_regex = Regex::new(r"^\s*\.show-tokens\s*$").unwrap();
    let exit_regex = Regex::new(r"^\s*\.exit\s*$").unwrap();

    if help_regex.is_match(&input) {
        print_help_menu();
    } else if show_tree_regex.is_match(&input) {
        toggle_show_tree();
    } else if exit_regex.is_match(&input) {
        return true;
    } else {
        evaluate_line(input);
    }

    return false;
}

#[inline]
fn toggle_show_tree() {
    unsafe {
        SHOW_TREE = !SHOW_TREE;
    }
}

#[inline]
fn print_help_menu() {
    println!("{}", HELP_MENU.green());
}

fn evaluate_line(input: String) {
    let tokens = Lexer::lex(input);
    unsafe {
        if SHOW_TOKENS {
            println!("Tokens:\n{}\n", format!("{:#?}", tokens).blue());
        }
    }
    match TokenParser::parse(tokens) {
        Ok(tree) => {
            unsafe {
                if SHOW_TREE {
                    println!("Tree:\n{}\n", format!("{:#?}", tree).blue());
                }
            }
            println!("{}", evaluate(tree));
        }
        Err(errors) => {
            let errors = format!("{:#?}", errors).red();
            println!("Errors occured while parsing expression: {}", errors);
        }
    }
}
