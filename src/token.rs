use std::thread::current;

pub const PATTERNS: [(&str, TokenKind); 15] = [
    (r"^(\s)", WhiteSpace),
    (r"^(\+)", Plus),
    (r"^(-)", Minus),
    (r"^(\*\*)", DoubleStar),
    (r"^(%)", Percent),
    (r"^(\^)", Caret),
    (r"^(\*)", Star),
    (r"^(\/)", Slash),
    (r"^(\|)", Pipe),
    (r"^(\()", OpenParenthesis),
    (r"^(\))", CloseParenthesis),
    (r"^(\d+\.?\d*)", NumberToken),
    (r"^(,)", Comma),
    (r"^([a-zA-Z]\w*)\(", FunctionCall),
    ("^(\0)$", EndOfFile),
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    NumberToken,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DoubleStar,
    Caret,
    OpenParenthesis,
    CloseParenthesis,
    Pipe,
    FunctionCall,
    Comma,
    EndOfFile,
    WhiteSpace,
    BadSymbol,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub index: usize,
    pub value: String,
}
use TokenKind::*;
impl Token {
    #[inline]
    pub fn new(kind: TokenKind, index: usize, value: String) -> Self {
        Token { kind, index, value }
    }

    pub fn is_usable_token(token: &Token) -> bool {
        match token.kind {
            WhiteSpace | BadSymbol | Comma => false,
            _ => true,
        }
    }

    pub fn get_binary_precedence(&self) -> u8 {
        match self.kind {
            DoubleStar | Caret => 3,
            Star | Slash | Percent => 2,
            Plus | Minus => 1,
            _ => 0,
        }
    }

    pub fn get_unary_precedence(&self) -> u8 {
        match self.kind {
            Plus | Minus => 1,
            _ => 0,
        }
    }
}
