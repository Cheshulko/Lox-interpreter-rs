use crate::Token;

use super::Node;

pub struct NilExp {}

pub struct LiteralExp<'de> {
    pub name: Token<'de>,
}

pub struct GroupingExp<'de> {
    pub inner: Box<Node<'de>>,
}

pub struct UnaryExp<'de> {
    pub operator: Token<'de>,
    pub right: Box<Node<'de>>,
}

pub struct BinaryExp<'de> {
    pub left: Box<Node<'de>>,
    pub operator: Token<'de>,
    pub right: Box<Node<'de>>,
}

pub struct LogicalExp<'de> {
    pub left: Box<Node<'de>>,
    pub operator: Token<'de>,
    pub right: Box<Node<'de>>,
}

pub struct AssignmentExp<'de> {
    pub name: Token<'de>,
    pub value: Box<Node<'de>>,
}

pub struct CallExp<'de> {
    pub callee: Box<Node<'de>>,
    pub args: Vec<Box<Node<'de>>>,
}

pub struct GetExp<'de> {
    pub callee: Box<Node<'de>>,
    pub name: Token<'de>,
}

pub struct SetExp<'de> {
    pub get_exp: Box<Node<'de>>,
    pub value: Box<Node<'de>>,
}

pub struct ThisExp<'de> {
    pub token: Token<'de>,
}

pub struct SuperExp<'de> {
    pub token: Token<'de>,
    pub method: Token<'de>,
}
