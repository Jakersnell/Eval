use core::panic;

use crate::{
    ast::{BinaryNode, EnclosedNode, Node, NumberNode, SyntaxTree, UnaryNode},
    token::{Token, TokenKind, TokenKind::*},
};

// should never be called on a tree that has any errors, this must be enforced through the parsers Result<_, _> return
pub fn evaluate(tree: SyntaxTree) -> f32 {
    evaluate_node(&tree.root)
}

fn evaluate_node(node: &Node) -> f32 {
    use Node::*;
    match node {
        Number(number_node) => number_node.number,
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
        Minus => -value,
        unsupported_type => panic!("Syntax token: {:#?} is unsupported as unary operator!", unsupported_type)
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

#[test]
fn test_evaluate_number() {
    let test_string = "3*3*3/2+1-3";
    todo!()
}
