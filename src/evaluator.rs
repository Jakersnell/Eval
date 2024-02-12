use core::panic;

use crate::{
    ast::{BinaryNode, EnclosedNode, Node, NumberNode, SyntaxTree, UnaryNode, FunctionKind, FunctionNode},
    token::{Token, TokenKind, TokenKind::*},
};

// should never be called on a tree that has any errors, this must be enforced through the parsers Result<_, _> return
#[inline]
pub fn evaluate(tree: SyntaxTree) -> f32 {
    let result = evaluate_node(&tree.root);
    if result == -0.0 {
        return 0.0;
    }
    result
}

fn evaluate_node(node: &Node) -> f32 {
    use Node::*;
    match node {
        Number(number_node) => number_node.number,
        Function(function_node) => evaulate_function_node(function_node),
        Unary(unary_node) => evaluate_unary_node(unary_node),
        Binary(binary_node) => evaluate_binary_node(binary_node),
        Enclosed(enclosed_node) => evaluate_enclosed_node(enclosed_node),
        _ => panic!("Unsupported node type:\n{:?}\n", node),
    }
}

fn evaluate_binary_node(binary_node: &BinaryNode) -> f32 {
    let left = evaluate_node(&binary_node.left);
    let right = evaluate_node(&binary_node.right);

    match &binary_node.token.kind {
        Plus => left + right,
        Minus => left - right,
        Star => left * right,
        Slash => left / right,
        Percent => left % right,
        DoubleStar | Caret => left.powf(right),
        unsupported_type => panic!("Unsupported type for binary node: {:#?}", unsupported_type),
    }
}

fn evaluate_unary_node(unary_node: &UnaryNode) -> f32 {
    let value = evaluate_node(&unary_node.expression);
    match unary_node.token.kind {
        Plus => value,
        Minus => -value,
        unsupported_type => panic!("Syntax token: {:#?} is unsupported as unary operator!", unsupported_type)
    }
}

fn evaulate_function_node(function_node: &FunctionNode) -> f32 {
    use FunctionKind::*;
    let values: Vec<f32> = function_node.args.iter().map(evaluate_node).collect();
    let first = values.first().unwrap();
    match &function_node.kind {
        Cos => first.cos(),
        Sin => first.sin(),
        Tan => first.tan(),
        Sqrt => first.sqrt(),
        Sinh => first.sinh(),
        Cosh => first.cosh(),
        Tanh => first.cosh(),
        Sqrt => first.sqrt(),
        Ln => first.ln(),
        Pow => first.powf(*values.get(1).unwrap()),
        Root => first.powf(1.0/(*values.get(1).unwrap())),
        Log => first.log(*values.get(1).unwrap()),
        unsupported_value => panic!("Unsupported function type {:#?}.", unsupported_value)
    }
}

fn evaluate_enclosed_node(enclosed_node: &EnclosedNode) -> f32 {
    let expression = &enclosed_node.expression;
    let mut number = evaluate_node(expression);

    if enclosed_node.left.kind == Pipe {
        number = number.abs();
    }

    number
}

