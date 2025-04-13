use std::rc::Rc;

use crate::Token;

use super::Node;

pub struct VarDecl<'de> {
    pub name: Token<'de>,
    pub initializer: Option<Box<Node<'de>>>,
}

pub struct FunctionDecl<'de> {
    pub name: Token<'de>,
    pub parameters: Vec<Token<'de>>,
    pub body: Rc<Node<'de>>,
}

pub struct ClassMethodDecl<'de> {
    pub name: Token<'de>,
    pub parameters: Vec<Token<'de>>,
    pub body: Rc<Node<'de>>,
}

pub struct ClassDecl<'de> {
    pub name: Token<'de>,
    pub super_class: Option<SuperClassDecl<'de>>,
    pub methods: Vec<Box<Node<'de>>>,
}

pub struct SuperClassDecl<'de> {
    pub name: Token<'de>,
}
