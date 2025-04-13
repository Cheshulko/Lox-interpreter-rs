use super::Node;

pub struct EmptyStm {}

pub struct PrintStm<'de> {
    pub expression: Box<Node<'de>>,
}

pub struct BlockStm<'de> {
    pub statements: Vec<Box<Node<'de>>>,
}

pub struct FuncBodyStm<'de> {
    pub statements: Vec<Box<Node<'de>>>,
}

pub struct IfElseStm<'de> {
    pub condition: Box<Node<'de>>,
    pub then_branch: Box<Node<'de>>,
    pub else_branch: Option<Box<Node<'de>>>,
}

pub struct WhileStm<'de> {
    pub condition: Box<Node<'de>>,
    pub body: Box<Node<'de>>,
}

pub struct ExpressionStm<'de> {
    pub expression: Box<Node<'de>>,
}

pub struct ReturnStm<'de> {
    pub expression: Box<Node<'de>>,
}
