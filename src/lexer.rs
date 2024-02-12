use std::str::Chars;

use regex::Regex;

use crate::token::{Token, TokenKind, TokenKind::*, PATTERNS};

pub struct Lexer {
    pub errors: Vec<String>,
    source: String,
    index: usize,
}

impl Lexer {
    #[inline]
    pub fn lex(source: String) -> Vec<Token> {
        Self::new(source).collect()
    }

    pub fn new(source: String) -> Self {
        Self {
            errors: Vec::new(),
            source: source + "\0",
            index: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        let index = self.index;
        self.index += 1;
        let mut result_kind = BadSymbol;
        let mut value = self.source[index..index + 1].to_string();
        let mut regex: Regex;

        for (pattern, kind) in &PATTERNS {
            regex = Regex::new(pattern).unwrap();

            if let Some(captures) = regex.captures(&self.source[index..]) {
                result_kind = *kind;
                value = captures.get(1).unwrap().as_str().to_string();
                self.index += value.len() - 1;
                break;
            }
        }

        if result_kind == BadSymbol  {
            self.errors.push(format!("Invalid symbol {} at input index {}.", value, index));
        }

        Token::new(result_kind, index, value)
    }

    fn is_valid_int(string: &str) -> bool {
        string.parse::<f32>().is_ok()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.source.len() == self.index {
            None
        } else {
            Some(self.next_token())
        }
    }
}

// problem with test
#[test]
fn test_next_for_proper_lexing() {
    let test_string = String::from("3.0f/-+*|**()");
    let control_tokens = vec![
        NumberToken,
        BadSymbol,
        Slash,
        Minus,
        Plus,
        Star,
        Pipe,
        DoubleStar,
        OpenParenthesis,
        CloseParenthesis,
    ]
    .into_iter();

    let mut lexer = Lexer::new(test_string);

    for control_token_kind in control_tokens {
        let token_kind = lexer.next().unwrap().kind;
        assert_eq!(control_token_kind, token_kind);
    }
}
