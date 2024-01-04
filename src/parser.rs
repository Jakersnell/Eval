use std::mem;

use crate::{
    ast::{BinaryNode, EnclosedNode, Node, NumberNode, SyntaxTree, UnaryNode},
    lexer::Lexer,
    token::{Token, TokenKind},
};

macro_rules! boolean_match {
    ($value:expr, $elem:expr) => {
        $value == $elem
    };
    ($value:expr, $elem:expr, $($rest:expr),+) => {
        $value == $elem || boolean_match!($value, $($rest),+)
    };
}

/// This is a macro that generates functions that correspond to operator precedence for binary operators.
/// I added a "mock dsl" syntax to make the usage more readable.
macro_rules! generate_binary_parse_level {
    (
        create --> fn $name:ident(&mut self) -> Node;
        next_level --> self.$next_function:ident();
        matches_on --> ($elem:expr $(, $rest:expr)*);
    )  => {
        fn $name(&mut self) -> Node {
            let mut left = self.$next_function();

            while boolean_match!(self.current().kind, $elem $(, $rest)*) {
                let operator = self.next_token();
                let right = self.$next_function();
                left = Node::Binary(BinaryNode::new(operator, left, right));
            }

            left
        }
    };
}

pub struct Parser {
    diagnostics: Vec<String>,
    tokens: Vec<Token>,
    position: usize,
}

/// made multiple helper functions #[inline] for slight performance increase
impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<SyntaxTree, Vec<String>> {
        let mut parser = Self::new(tokens);
        let root = parser.parse_top_level();
        let eof = parser.get_match(TokenKind::EndOfFile);
        if parser.diagnostics.is_empty() {
            Ok(SyntaxTree::new(root, eof))
        } else {
            Err(mem::take(&mut parser.diagnostics))
        }
    }

    #[inline]
    fn new(tokens: Vec<Token>) -> Self {
        let mut diagnostics = Vec::new();
        let tokens = tokens.into_iter().filter(Token::is_usable_token).collect();

        Parser {
            diagnostics,
            tokens,
            position: 0,
        }
    }

    #[inline]
    fn peek(&self, offset: i32) -> Token {
        self.tokens
            .get(self.position + offset as usize)
            .unwrap_or(self.tokens.last().unwrap())
            .clone()
    }

    #[inline]
    fn current(&self) -> Token {
        self.peek(0)
    }

    #[inline]
    fn next_token(&mut self) -> Token {
        let current = self.current();
        self.position += 1;
        current
    }

    #[inline]
    fn get_match(&mut self, kind: TokenKind) -> Token {
        let current = self.current();
        if current.kind == kind {
            self.next_token()
        } else {
            let error_str = format!(
                "Error at input index {}. Expected {:#?} but found {:#?} '{:#?}'",
                current.index, kind, current.kind, current.value
            );
            self.diagnostics.push(error_str);
            Token::new(kind, self.position, "".to_owned())
        }
    }

    #[inline]
    fn parse_top_level(&mut self) -> Node {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, lp: u8) -> Node {
        
        let mut left = self.parse_unary_expression();

        loop {
            let precedence = self.current().get_binary_precedence();
            if precedence == 0 || precedence <= lp {
                break;
            }
            let operator = self.next_token();
            let right = self.parse_binary_expression(precedence);

            left = Node::Binary(BinaryNode::new(operator, left, right));
        }

        left
    }

    fn parse_unary_expression(&mut self) -> Node {
        // doesnt parse repeating minus signs
        let mut node;

        if self.current().kind == TokenKind::Minus {
            let unary_token = self.get_match(TokenKind::Minus);
            let expression = self.parse_unary_expression();
            node = Node::Unary(UnaryNode::new(unary_token, expression));
        } else {
            node = self.parse_primary_expression();
        }

        node
    }

    fn parse_primary_expression(&mut self) -> Node {
        static ENCLOSING_TOKENS: [(TokenKind, TokenKind); 2] = [
            (TokenKind::OpenParenthesis, TokenKind::CloseParenthesis),
            (TokenKind::Pipe, TokenKind::Pipe),
        ];

        for (opening_type, closing_type) in ENCLOSING_TOKENS {
            if self.current().kind == opening_type {
                let left = self.next_token();
                let expression = self.parse_binary_expression(0);
                let right = self.get_match(closing_type);

                return Node::Enclosed(EnclosedNode::new(left, expression, right));
            }
        }

        self.parse_number_node()
    }

    fn parse_number_node(&mut self) -> Node {
        let token = self.get_match(TokenKind::NumberToken);

        let mut node_number = token.value.parse::<f32>().unwrap_or_else(|_| {
            self.diagnostics.push(format!(
                "Invalid number found {} at input index {}.",
                token.value, token.index
            ));
            0.0
        });

        Node::Number(NumberNode::new(token, node_number))
    }
}
