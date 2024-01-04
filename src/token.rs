use std::thread::current;

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
    LeftGuillemet,
    RightGuillemet,
    OpenParenthesis,
    CloseParenthesis,
    Pipe,
    FunctionCall,
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

impl Token {
    #[inline]
    pub fn is_usable_token(token: &Token) -> bool {
        match token.kind {
            TokenKind::WhiteSpace | TokenKind::BadSymbol => false,
            _ => true,
        }
    }

    pub fn get_binary_precedence(&self) -> u8 {
        use TokenKind::*;
        match self.kind {
            DoubleStar | Caret => 3,
            Star | Slash | Percent => 2,
            Plus | Minus => 1,
            _ => 0,
        }
    }

    pub fn get_unary_precedence(&self) -> u8 {
        use TokenKind::*;
        match self.kind {
            Plus | Minus => 1,
            _ => 0,
        }
    }
}

impl Token {
    #[inline]
    pub fn new(kind: TokenKind, index: usize, value: String) -> Self {
        Token { kind, index, value }
    }
}
