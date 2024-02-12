use crate::token::Token;

#[derive(Debug)]
pub struct SyntaxTree {
    pub root: Node,
    pub eof: Token,
}

impl SyntaxTree {
    #[inline]
    pub fn new(root: Node, eof: Token) -> Self {
        Self { root, eof }
    }
}

#[derive(Debug)]
pub enum Node {
    Function(FunctionNode),
    Unary(UnaryNode),
    Number(NumberNode),
    Binary(BinaryNode),
    Enclosed(EnclosedNode),
}

#[derive(Debug)]
pub struct BinaryNode {
    pub token: Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl BinaryNode {
    #[inline]
    pub fn new(token: Token, left: Node, right: Node) -> Self {
        Self {
            token,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct EnclosedNode {
    pub left: Token,
    pub expression: Box<Node>,
    pub right: Token,
}

impl EnclosedNode {
    #[inline]
    pub fn new(left: Token, expression: Node, right: Token) -> Self {
        Self {
            left,
            expression: Box::new(expression),
            right,
        }
    }
}

#[derive(Debug)]
pub struct NumberNode {
    pub token: Token,
    pub number: f32,
}

impl NumberNode {
    #[inline]
    pub fn new(token: Token, number: f32) -> Self {
        Self { token, number }
    }
}

#[derive(Debug)]
pub struct UnaryNode {
    pub token: Token,
    pub expression: Box<Node>,
}

impl UnaryNode {
    #[inline]
    pub fn new(token: Token, expression: Node) -> Self {
        Self {
            token,
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug)]
pub struct FunctionNode {
    pub kind: FunctionKind,
    pub token: Token,
    pub args: Vec<Node>,
}

impl FunctionNode {
    #[inline]
    pub fn new(kind: FunctionKind, token: Token, args: Vec<Node>) -> Self {
        Self { kind, token, args }
    }
}

#[derive(Debug)]
pub enum FunctionKind {
    // single arg functions
    Cos,
    Sin,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Sqrt,
    Ln,
    
    // two arg functions
    Pow,
    Root,
    Log,
    BadValue,
}

impl FunctionKind {
    pub fn from(value: &str) -> Self {
        use FunctionKind::*;
        match value {
            "sin" => Sin,
            "cos" => Cos,
            "tan" => Tan,
            "sinh" => Sinh,
            "cosh" => Cosh,
            "tanh" => Tanh,
            "sqrt" => Sqrt,
            "ln" => Ln,
            "pow" => Pow,
            "root" => Root,
            "log" => Log,
            _ => BadValue,
        }
    }

    pub fn get_args_count(&self) -> u8 {
        use FunctionKind::*;
        match self {
            Sin | Cos | Tan | Sinh | Cosh | Tanh | Sqrt | Ln => 1,
            Pow | Root | Log => 2,
            BadValue => 0,
        }
    }
}
